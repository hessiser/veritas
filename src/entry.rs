use crate::{get_module_handle, kreide, logging, overlay, server, subscribers};
use ctor::ctor;
use egui_notify::Toast;
use il2cpp_runtime::api::ApiIndexTable;
use windows::Win32::Foundation::{GetLastError, HMODULE, MAX_PATH};
use windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;
use windows::Win32::System::ProcessStatus::{GetModuleInformation, MODULEINFO};
use windows::Win32::System::Threading::GetCurrentProcess;
use windows::core::w;
use std::ffi::{OsString, c_void};
use std::io::{Cursor, Write};
use std::os::windows::ffi::OsStringExt;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::PathBuf;
use std::{
    thread::{self},
    time::Duration,
};
use windows::Win32::System::LibraryLoader::{GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, GetModuleFileNameW, GetModuleHandleExA, GetModuleHandleW, GetProcAddress};
use anyhow::{Context, Result, anyhow};

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
    log::info!("Build ID: {}", env!("VERITAS_BUILD_ID"));
    match setup_subscribers_protected() {
        Ok(_) => {
            let msg = format!("Core initialized successfully");
            log::info!("{}", msg);
            toasts.push(Toast::success(msg));
        }
        Err(e) => {
            let err = format!("Core failed to initialize and has been disabled: {e}");
            log::error!("Core failed to initialize and has been disabled: {:#}", e);
            let mut toast = Toast::error(err);
            toast.duration(None);
            toasts.push(toast);
        }
    };

    fn setup_subscribers_protected() -> anyhow::Result<()> {
        match microseh::try_seh(|| catch_unwind(AssertUnwindSafe(setup_subscribers))) {
            Ok(Ok(result)) => result,
            Ok(Err(panic)) => {
                let message = panic
                    .downcast_ref::<&str>()
                    .map(|value| (*value).to_string())
                    .or_else(|| panic.downcast_ref::<String>().cloned())
                    .unwrap_or_else(|| "unknown panic payload".to_string());
                Err(anyhow!("Startup panic in setup_subscribers: {message}"))
            }
            Err(seh) => Err(anyhow!("Startup SEH in setup_subscribers: {seh:?}")),
        }
    }

    thread::spawn(|| server::start_server());

    match overlay::initialize(toasts) {
        Ok(_) => log::info!("Overlay initialized successfully"),
        Err(e) => log::error!("Overlay failed to initialize: {}", e),
    }
}


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

        let mut buffer = vec![0u8; lp_mod_info.SizeOfImage as usize];
        let mut bytes_read = 0usize;

        ReadProcessMemory(
            process_handle,
            module.0,
            buffer.as_mut_ptr() as _,
            lp_mod_info.SizeOfImage as usize,
            Some(&mut bytes_read),
        )
        .context("Failed to read module memory")?;

        static PATTERN: &str = "48 8B 05 ? ? ? ? 48 8D 0D ? ? ? ? FF D0";
        let locs = patternscan::scan(Cursor::new(buffer.as_slice()), &PATTERN)
            .context("Failed to scan for il2cpp pattern")?;
        let instruction_offset = *locs
            .get(0)
            .context("Pattern not found in UnityPlayer module")?;
        let addr = instruction_offset + module.0 as usize;

        let displacement_bytes: [u8; 4] = buffer
            .get(instruction_offset + 3..instruction_offset + 7)
            .context("Pattern displacement bytes were out of bounds")?
            .try_into()
            .map_err(|_| anyhow!("Pattern displacement did not contain 4 bytes"))?;
        let displacement = i32::from_le_bytes(displacement_bytes) as isize;
        let qword_addr = ((addr + 7) as isize + displacement) as usize;
        Ok(qword_addr)
    }
}

unsafe fn try_get_il2cpp_table_from_export() -> Result<Option<usize>> {
    const IL2CPP_GET_API_TABLE_EXPORT: &[u8] = b"il2cpp_get_api_table\0";

    let gameassembly_offset = get_module_handle(w!("GameAssembly"))
        .map_err(|e| anyhow!(e.to_string()))
        .context("Failed to resolve GameAssembly module")?;
    let module = HMODULE(gameassembly_offset as *mut c_void);

    let Some(proc) = (unsafe {
        GetProcAddress(
            module,
            windows::core::PCSTR::from_raw(IL2CPP_GET_API_TABLE_EXPORT.as_ptr()),
        )
    }) else {
        return Ok(None);
    };

    let export_addr = proc as usize;

    let get_api_table = unsafe {
        std::mem::transmute::<
            unsafe extern "system" fn() -> isize,
            unsafe extern "system" fn() -> *const usize,
        >(proc)
    };

    let api_table_offset = match microseh::try_seh(|| {
        catch_unwind(AssertUnwindSafe(|| unsafe { get_api_table() as usize }))
    }) {
        Ok(Ok(ptr)) => ptr,
        Ok(Err(panic)) => {
            let message = panic
                .downcast_ref::<&str>()
                .map(|value| (*value).to_string())
                .or_else(|| panic.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "unknown panic payload".to_string());
            let _ = message;
            return Ok(None);
        }
        Err(seh) => {
            let _ = seh;
            return Ok(None);
        }
    };

    if api_table_offset == 0 {
        return Ok(None);
    }
    let _ = gameassembly_offset;
    let _ = export_addr;
    Ok(Some(api_table_offset))
}

fn setup_subscribers() -> anyhow::Result<()> {
    unsafe {
        while GetModuleHandleW(windows::core::w!("GameAssembly")).is_err()
            || GetModuleHandleW(windows::core::w!("UnityPlayer")).is_err()
        {
            thread::sleep(Duration::from_secs(3));
        }

        let gameassembly = get_module_handle(w!("GameAssembly"))
            .map_err(|e| anyhow!(e.to_string()))
            .context("Failed to confirm GameAssembly module after wait")?;
        let unityplayer = get_module_handle(w!("UnityPlayer"))
            .map_err(|e| anyhow!(e.to_string()))
            .context("Failed to confirm UnityPlayer module after wait")?;
        let _ = gameassembly;
        let _ = unityplayer;

        let table = ApiIndexTable {
            il2cpp_assembly_get_image: 22,
            il2cpp_class_get_fields: 31,
            il2cpp_class_get_methods: 35,
            il2cpp_class_get_name: 37,
            il2cpp_class_get_namespace: 39,
            il2cpp_class_get_parent: 40,
            il2cpp_class_from_type: 49,
            il2cpp_domain_get: 63,
            il2cpp_domain_get_assemblies: 65,
            il2cpp_field_get_name: 73,
            il2cpp_field_get_offset: 75,
            il2cpp_field_get_type: 76,
            il2cpp_field_get_value_object: 77,
            il2cpp_method_get_return_type: 116,
            il2cpp_method_get_name: 117,
            il2cpp_method_get_param_count: 123,
            il2cpp_method_get_param: 124,
            il2cpp_object_new: 130,
            il2cpp_thread_attach: 154,
            il2cpp_type_get_name: 161,
            il2cpp_image_get_class_count: 169,
            il2cpp_image_get_class: 170,
        };
        let api_table_offset = get_il2cpp_table_offset().context("Failed to resolve IL2CPP API table")?;
        il2cpp_runtime::init(api_table_offset, table).context("Failed to initialize IL2CPP runtime")?;
        subscribers::battle::subscribe().context("Failed to register battle subscriber hooks")?;
        subscribers::enable_subscribers!().context("Failed to register subscriber hooks")?;
        Ok(())
    }
}

