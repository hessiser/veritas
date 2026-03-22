#![recursion_limit = "256"]
#![allow(static_mut_refs)]
#![feature(windows_process_extensions_show_window)]
#[macro_use]
extern crate rust_i18n;

mod battle;
mod entry;
mod export;
mod kreide;
mod logging;
mod models;
mod overlay;
mod prelude;
mod server;
mod subscribers;
mod ui;
mod updater;

use phf::phf_map;
use std::sync::LazyLock;
use tokio::runtime::Runtime;
use widestring::u16str;
use windows::{Win32::System::LibraryLoader::GetModuleHandleW, core::PCWSTR};
use anyhow::{Context, Result, anyhow};

fn get_module_handle(name: PCWSTR) -> Result<usize> {
    unsafe {
        GetModuleHandleW(name)
            .map(|v| v.0 as usize)
            .context("Failed to get module handle")
    }
}

pub static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    Runtime::new().unwrap_or_else(|e| {
        log::error!("{e}");
        panic!("{e}");
    })
});

pub const CHANGELOG: &str = include_str!("../CHANGELOG.MD");

static LOCALES: phf::Map<&'static str, &'static str> = phf_map! {
    "en" => "English",
    "fr" => "Français",
    "es" => "Español",
    "de" => "Deutsch",
    "it" => "Italiano",
    "ja" => "日本語",
    "nl" => "Nederlands",
    "pl" => "Polski",
    "pt" => "Português",
    "ru" => "Русский",
    "vi" => "Tiếng Việt",
    "zh" => "中文",
    "ar" => "العربية",
};

rust_i18n::i18n!();

#[cfg(test)]
mod tests {
    use std::thread;

    use edio11::Overlay;
    use eframe::EventLoopBuilderHook;

    use crate::ui::{self, app::SHOW_MENU_SHORTCUT};

    #[test]
    fn egui_main() {
        use winit::platform::windows::EventLoopBuilderExtWindows;
        let handle = thread::spawn(|| {
            let event_loop_builder: Option<EventLoopBuilderHook> =
                Some(Box::new(|event_loop_builder| {
                    event_loop_builder.with_any_thread(true);
                }));
            let native_options = eframe::NativeOptions {
                event_loop_builder,
                ..Default::default()
            };

            let mut app = ui::app::App::new(egui::Context::default());
            eframe::run_simple_native(env!("CARGO_PKG_NAME"), native_options, move |ctx, _| {
                if ctx.input_mut(|i| i.consume_shortcut(&SHOW_MENU_SHORTCUT)) {
                    app.state.show_menu = !app.state.show_menu;
                }

                app.update(ctx);
            }).expect("failed to run app");
        });
        handle.join().unwrap();
    }
}
