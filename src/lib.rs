#![recursion_limit = "256"]
#[macro_use]
extern crate rust_i18n;

macro_rules! lazy_initialize_address {
    ($addr:expr) => {
        LazyLock::new(|| unsafe { std::mem::transmute($addr + *$crate::GAMEASSEMBLY_HANDLE) })
    };
}
mod battle;
mod entry;
mod subscribers;
mod models;
mod server;
mod kreide;
mod ui;
mod logging;
mod overlay;

use std::sync::LazyLock;
use phf::phf_map;
use windows::{
    core::w,
    Win32::System::LibraryLoader::GetModuleHandleW,
};

pub static GAMEASSEMBLY_HANDLE: LazyLock<usize> =
    LazyLock::new(|| unsafe { GetModuleHandleW(w!("GameAssembly")).expect("GameAssembly was not found in the game process").0 as usize });

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