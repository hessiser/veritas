use std::{fs::File, io::Write, path::PathBuf};

use anyhow::{Result, anyhow};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use super::themes::EGUI_THEME;

const CONFIG_FILENAME: &'static str = "config.json";

macro_rules! config {
    ( $ ( $field:ident : $field_type:ty ) ,*) => {
        #[derive(Clone, Deserialize, Serialize)]
        pub struct Config {
            $(
                $field : $field_type,
            )*
        }

        impl Config {
            paste::item! {
                $(
                    pub fn [<set_ $field>] (&mut self, value: $field_type) {
                        if self. $field != value {
                            self. $field = value;
                            self.write().unwrap();
                        }
                    }

                    pub fn [<get_ $field>] (&self) -> & $field_type {
                        & self. $field
                    }

                )*
            }
        }
    };
}

config!(
    locale: String,
    // fps: i32,
    widget_opacity: f32,
    streamer_mode: bool,
    streamer_msg: String,
    streamer_msg_size_pt: f32,
    theme: egui_colors::Theme,
    theme_mode: egui::Theme
);

impl Default for Config {
    fn default() -> Self {
        Self {
            locale: rust_i18n::locale().to_string(),
            // fps: 60,
            widget_opacity: 0.15,
            streamer_mode: true,
            streamer_msg: String::new(),
            theme: EGUI_THEME,
            theme_mode: egui::Theme::Dark,
            streamer_msg_size_pt: 1.0
        }
    }
}

impl Config {
    pub fn new(ctx: &egui::Context) -> Result<Self> {
        match ProjectDirs::from("", "", "veritas") {
            Some(proj_dirs) => {
                let config_local_dir = proj_dirs.config_local_dir();
                let config_path = config_local_dir.join(CONFIG_FILENAME);

                if !config_local_dir.exists() {
                    std::fs::create_dir_all(config_local_dir)?;
                }

                if !config_path.exists() {
                    Self::initialize(&config_path, ctx)
                } else {
                    let mut file = File::open(&config_path)?;
                    match serde_json::from_reader(&file) {
                        Ok(v) => Ok(v),
                        Err(_) => {
                            file.flush()?;
                            Self::initialize(&config_path, ctx)
                        }
                    }
                }
            }
            None => Err(anyhow!("Failed to load/create config.")),
        }
    }

    fn initialize(config_path: &PathBuf, ctx: &egui::Context) -> Result<Self> {
        let config: Config = Config {
            theme_mode: ctx.theme(),
            ..Default::default()
        };
        let mut file = File::create(config_path)?;
        serde_json::to_writer(&mut file, &config)?;
        file.flush()?;
        Ok(config)
    }

    fn write(&self) -> Result<()> {
        match ProjectDirs::from("", "", "veritas") {
            Some(proj_dirs) => {
                let config_local_dir = proj_dirs.config_local_dir();
                let config_path = config_local_dir.join(CONFIG_FILENAME);

                if !config_path.exists() {
                    std::fs::create_dir_all(config_local_dir)?;
                }

                let mut file = File::create(config_path)?;
                serde_json::to_writer(&mut file, self)?;
                file.flush()?;
                Ok(())
            }
            None => Err(anyhow!("Failed to write config.")),
        }
    }
}
