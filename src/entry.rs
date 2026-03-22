use crate::{get_module_handle, kreide, logging, overlay, server, subscribers};
use ctor::ctor;
use egui_notify::Toast;
use il2cpp_runtime::api::ApiIndexTable;
use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;
use windows::Win32::System::ProcessStatus::{GetModuleInformation, MODULEINFO};
use windows::Win32::System::Threading::GetCurrentProcess;
use windows::core::w;
use std::ffi::c_void;
use std::io::Cursor;
use std::sync::{LazyLock, Mutex};
use std::{
    thread::{self},
    time::Duration,
};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;

#[derive(Clone, Debug)]
pub enum InitErrorInfo {
    ObfuscationMismatch {
        class_name: Option<String>,
        message: String,
    },
    Other {
        message: String,
    },
}

pub static INIT_ERROR: LazyLock<Mutex<Option<InitErrorInfo>>> = LazyLock::new(|| Mutex::new(None));

pub fn take_init_error() -> Option<InitErrorInfo> {
    INIT_ERROR.lock().unwrap().take()
}

fn store_init_error(info: InitErrorInfo) {
    *INIT_ERROR.lock().unwrap() = Some(info);
}

#[ctor]
#[cfg(not(test))]
fn entry() {
    thread::spawn(|| init());
}

fn init() {
    logging::MultiLogger::init().unwrap();
    #[cfg(debug_assertions)]
    unsafe {
        windows::Win32::System::Console::AllocConsole().unwrap();
    }

    let mut toasts = Vec::<Toast>::new();
    let plugin_name = format!("{} ({})", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    log::info!("{}", plugin_name);
    match setup_subscribers() {
        Ok(_) => {
            let msg = format!("Core initialized successfully");
            log::info!("{}", msg);
            toasts.push(Toast::success(msg));
        }
        Err(e) => {
            let err = format!("Core failed to initialize and has been disabled: {e}");
            log::error!("{}", err);
            if let Some(info) = classify_init_error(&e) {
                store_init_error(info);
            }
            let mut toast = Toast::error(err);
            toast.duration(None);
            toasts.push(toast);
        }
    };

    thread::spawn(|| server::start_server());

    match overlay::initialize(toasts) {
        Ok(_) => log::info!("Overlay initialized successfully"),
        Err(e) => log::error!("Overlay failed to initialize: {}", e),
    }
}
use anyhow::{Context, Result, anyhow};


fn get_il2cpp_table_offset() -> Result<usize> {
    unsafe {
        let unityplayer_offset = get_module_handle(w!("UnityPlayer"))
            .map_err(|e| anyhow!(e.to_string()))
            .context("Failed to resolve UnityPlayer module")?;
        let module = windows::Win32::Foundation::HMODULE(unityplayer_offset as *mut c_void);

        let process_handle = GetCurrentProcess();
        let mut lp_mod_info = MODULEINFO::default();

        GetModuleInformation(
            process_handle,
            module,
            &mut lp_mod_info,
            size_of::<MODULEINFO>() as u32,
        )
        .context("Failed to read module information")?;

        let buffer = vec![0u8; lp_mod_info.SizeOfImage as usize];
        let mut bytes_read = 0usize;

        ReadProcessMemory(
            process_handle,
            module.0,
            buffer.as_ptr() as _,
            lp_mod_info.SizeOfImage as usize,
            Some(&mut bytes_read),
        )
        .context("Failed to read module memory")?;

        static PATTERN: &str = "48 8B 05 ? ? ? ? 48 8D 0D ? ? ? ? FF D0";
        let locs = patternscan::scan(Cursor::new(buffer), &PATTERN)
            .context("Failed to scan for il2cpp pattern")?;
        let addr = locs
            .get(0)
            .context("Pattern not found in UnityPlayer module")?
            + module.0 as usize;

        let qword_addr = addr + 7 + *((addr + 3) as *const i32) as usize;
        Ok(qword_addr)
    }
}

fn setup_subscribers() -> anyhow::Result<()> {
    unsafe {
        log::info!("Setting up...");

        while GetModuleHandleW(windows::core::w!("GameAssembly")).is_err()
            || GetModuleHandleW(windows::core::w!("UnityPlayer")).is_err()
        {
            thread::sleep(Duration::from_secs(3));
        }

        let table = ApiIndexTable {
            il2cpp_assembly_get_image: 22,
            il2cpp_class_get_methods: 35,
            il2cpp_class_get_name: 37,
            il2cpp_class_from_type: 49,
            il2cpp_domain_get: 63,
            il2cpp_domain_get_assemblies: 65,
            il2cpp_field_get_name: 73,
            il2cpp_field_get_value_object: 77,
            il2cpp_method_get_return_type: 116,
            il2cpp_method_get_name: 117,
            il2cpp_method_get_param_count: 123,
            il2cpp_method_get_param: 124,
            il2cpp_thread_attach: 154,
            il2cpp_type_get_name: 161,
            il2cpp_image_get_class_count: 169,
            il2cpp_image_get_class: 170,
        };
        il2cpp_runtime::init(get_il2cpp_table_offset()?, table)?;
        subscribers::battle::subscribe()?;
        subscribers::enable_subscribers!()?;
        Ok(())
    }
}

fn classify_init_error(error: &anyhow::Error) -> Option<InitErrorInfo> {
    let message = error.to_string();
    if let Some(class_name) = extract_missing_class(&message) {
        Some(InitErrorInfo::ObfuscationMismatch {
            class_name: Some(class_name),
            message,
        })
    } else {
        Some(InitErrorInfo::Other { message })
    }
}

fn extract_missing_class(message: &str) -> Option<String> {
    let needle = "no such class";
    if !message.contains(needle) {
        return None;
    }

    let after = message.split(needle).nth(1)?;
    let class_name = after
        .trim()
        .split(|c: char| c.is_whitespace() || c == ':' || c == ')')
        .next()
        .filter(|s| !s.is_empty())
        .map(|s| s.trim_matches('"').to_string())?;
    Some(class_name)
}
