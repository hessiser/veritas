use anyhow::{Context, Result, anyhow};
use egui_logger::EguiLogger;
use log::{Level, LevelFilter, Metadata, Record};
use slog::{Drain, Logger, o};
use std::{
    fs::OpenOptions,
    io::{self, Read, Seek, SeekFrom, Write},
    sync::{Mutex, OnceLock},
};

static LOGGER: OnceLock<MultiLogger> = OnceLock::new();
const DEBUG_LOG_MAX_SIZE: u64 = 10 * 1024 * 1024;
const DEBUG_LOG_RETAINED_SIZE: u64 = DEBUG_LOG_MAX_SIZE / 4;

struct SizeLimitedWriter {
    file: std::fs::File,
    size: u64,
}

impl SizeLimitedWriter {
    fn new(file: std::fs::File) -> io::Result<Self> {
        let size = file.metadata()?.len();
        Ok(Self { file, size })
    }

    fn reset(&mut self) -> io::Result<()> {
        let tail_start = DEBUG_LOG_MAX_SIZE - DEBUG_LOG_RETAINED_SIZE;
        self.file.seek(SeekFrom::Start(tail_start))?;
        let mut tail = Vec::with_capacity(DEBUG_LOG_RETAINED_SIZE as usize);
        self.file.read_to_end(&mut tail)?;

        self.file.set_len(0)?;
        self.file.seek(SeekFrom::Start(0))?;
        self.file.write_all(&tail)?;
        self.size = tail.len() as u64;
        Ok(())
    }
}

impl Write for SizeLimitedWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.size == DEBUG_LOG_MAX_SIZE {
            self.reset()?;
        }

        let remaining = (DEBUG_LOG_MAX_SIZE - self.size) as usize;
        let bytes_to_write = buf.len().min(remaining);
        let written = self.file.write(&buf[..bytes_to_write])?;
        self.size += written as u64;
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}

pub struct MultiLogger {
    sloggers: Vec<Logger>,
    egui_logger: EguiLogger,
}

impl log::Log for MultiLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        self.egui_logger.log(record);

        let fmt_log = if let Some(mod_path) = record.module_path() {
            format!("{} {}", mod_path, record.args())
        } else {
            format!("{}", record.args())
        };
        for slogger in &self.sloggers {
            match record.level() {
                Level::Error => slog::error!(slogger, "{}", fmt_log),
                Level::Warn => slog::warn!(slogger, "{}", fmt_log),
                Level::Info => slog::info!(slogger, "{}", fmt_log),
                Level::Debug => slog::debug!(slogger, "{}", fmt_log),
                Level::Trace => slog::trace!(slogger, "{}", fmt_log),
            }
        }
    }

    fn flush(&self) {}
}

impl MultiLogger {
    pub fn init() -> Result<()> {
        let mut sloggers = Vec::new();

        // Info+ log file
        {
            let log_path = format!("{}.log", env!("CARGO_PKG_NAME"));
            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(log_path)?;

            let decorator = slog_term::PlainSyncDecorator::new(file);
            let drain = slog_term::FullFormat::new(decorator).build().fuse();
            let drain = Mutex::new(drain).fuse();
            let filtered_drain = slog::LevelFilter::new(drain, slog::Level::Info).fuse();

            sloggers.push(Logger::root(filtered_drain, o!()));
        }

        // Debug+ log file
        {
            let log_path = format!("{}.debug.log", env!("CARGO_PKG_NAME"));
            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .truncate(true)
                .open(log_path)?;
            let file = SizeLimitedWriter::new(file)?;

            let decorator = slog_term::PlainSyncDecorator::new(file);
            let drain = slog_term::FullFormat::new(decorator).build().fuse();
            let drain = Mutex::new(drain).fuse();
            let filtered_drain = slog::LevelFilter::new(drain, slog::Level::Debug).fuse();

            sloggers.push(Logger::root(filtered_drain, o!()));
        }

        // Terminal
        {
            let decorator = slog_term::TermDecorator::new().build();
            let drain = slog_term::FullFormat::new(decorator).build().fuse();
            let drain = Mutex::new(drain).fuse();

            let level = if cfg!(debug_assertions) {
                slog::Level::Debug
            } else {
                slog::Level::Info
            };

            let filtered_drain = slog::LevelFilter::new(drain, level).fuse();

            sloggers.push(Logger::root(filtered_drain, o!()));
        }

        let egui_logger = egui_logger::builder().build();
        let multi_logger = MultiLogger {
            sloggers,
            egui_logger,
        };

        if LOGGER.set(multi_logger).is_err() {
            return Err(anyhow!("Failed to initialize MultiLogger"));
        }

        if log::set_logger(LOGGER.get().context("Failed to get MultiLogger")?).is_err() {
            return Err(anyhow!("Failed to set MultiLogger"));
        }
        log::set_max_level(LevelFilter::Trace);
        Ok(())
    }
}
