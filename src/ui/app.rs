use std::fs::File;
use std::io::Read;
use std::io::Write;
use anyhow::Result;
use anyhow::anyhow;
use directories::ProjectDirs;
use edio11::{Overlay, WindowMessage, WindowProcessOptions, input::InputResult};
use egui::Key;
use egui::KeyboardShortcut;
use egui::Label;
use egui::Memory;
use egui::Modifiers;
use egui::RichText;
use egui::Ui;
use egui::UiBuilder;
use egui::{
    Context,
    epaint::text::{FontInsert, InsertFontFamily},
};
use egui_colors::Colorix;
use egui_inbox::UiInbox;
use egui_notify::Toasts;
use serde::Deserialize;
use serde::Serialize;
use windows::Win32::{
    Foundation::{LPARAM, WPARAM},
    UI::{Input::KeyboardAndMouse::VK_MENU, WindowsAndMessaging::WM_KEYDOWN},
};

use crate::RUNTIME;
use crate::battle::BattleContext;
use crate::export::BattleDataExporter;
use crate::updater::Status;
use crate::updater::Update;
use crate::updater::Updater;

use super::config::Config;

#[derive(Default, PartialEq, Serialize, Deserialize)]
pub enum GraphUnit {
    #[default]
    Turn,
    ActionValue,
}

#[derive(Default, PartialEq, Serialize, Deserialize)]
pub enum DamageBreakdownScope {
    #[default]
    Team,
    Character,
}

#[derive(Default, PartialEq, Serialize, Deserialize)]
pub enum DamageBreakdownChart {
    #[default]
    Pie,
    Bar,
}

#[derive(Default, PartialEq, Serialize, Deserialize)]
pub enum DamageBarValue {
    #[default]
    Damage,
    Dpav,
}

#[derive(Clone)]
pub enum ExportNotification {
    Success,
    Error { message: String },
}

#[derive(Serialize, Deserialize)]
pub struct AppState {
    pub show_menu: bool,
    pub show_changelog: bool,
    pub show_help: bool,
    pub show_settings: bool,
    pub show_console: bool,
    pub show_damage_distribution: bool,
    #[serde(default)]
    pub show_damage_type_breakdown: bool,
    pub show_damage_bars: bool,
    #[serde(default)]
    pub show_character_damage: bool,
    pub show_real_time_damage: bool,
    pub show_enemy_stats: bool,
    pub show_battle_metrics: bool,
    pub hide_ui: bool,
    pub graph_x_unit: GraphUnit,
    #[serde(default)]
    pub damage_bar_value: DamageBarValue,
    #[serde(default)]
    pub damage_breakdown_scope: DamageBreakdownScope,
    #[serde(default)]
    pub damage_breakdown_chart: DamageBreakdownChart,
    #[serde(default)]
    pub damage_breakdown_character_index: usize,
    #[serde(default = "default_true")]
    pub show_damage_breakdown_table: bool,
    #[serde(default = "default_true")]
    pub show_damage_breakdown_legend: bool,
    #[serde(skip)]
    pub use_custom_color: bool,
    #[serde(skip)]
    pub update_bttn_enabled: bool,
    #[serde(skip)]
    pub center_updater_window: bool,
    pub show_character_legend: bool,
    pub auto_save_battle_data: bool,
    pub show_export_window: bool,
    pub show_updater_window: bool,
    pub custom_export_path: Option<String>,
    pub auto_create_date_folders: bool,
}

pub struct App {
    pub state: AppState,
    pub config: Config,
    pub update_config: super::config::UpdateConfig,
    pub notifs: Toasts,
    pub colorix: Colorix,
    pub update_inbox: UiInbox<Option<Update>>,
    pub export_inbox: UiInbox<ExportNotification>,
    pub update: Option<Update>,
    pub updater_hint: Option<String>,
    pub updater_window_last_size: Option<egui::Vec2>,
}

fn default_true() -> bool {
    true
}

pub const HIDE_UI_SHORTCUT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::H);
pub const SHOW_MENU_SHORTCUT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::M);

impl Overlay for App {
    // This is where the main logic of the app lives. This is called every frame and is responsible for rendering the UI and handling input.
    fn update(&mut self, ctx: &egui::Context) {
        // Get rid of this and just switch to egui-toast
        if self.state.show_changelog {
            self.show_changelog_window(ctx);
        }

        if self.state.show_help {
            self.show_help_window(ctx);
        }

        if self.config.streamer_mode {
            egui::TopBottomPanel::bottom("statusbar")
                .resizable(true)
                .show(ctx, |ui| {
                    ui.style_mut().override_text_style = Some(egui::TextStyle::Body);
                    let label = Label::new(RichText::new(&self.config.streamer_msg).strong())
                        .selectable(false);
                    ui.add(label);
                    ui.allocate_space(ui.available_size())
                });
        }

        if !self.state.hide_ui {
            if self.state.show_menu {
                self.show_menu(ctx);
            }

            if self.state.show_console {
                self.show_console_window(ctx);
            }

            if self.state.show_damage_type_breakdown {
                self.show_damage_type_breakdown_window(ctx);
            }

            if self.state.show_character_legend {
                self.show_character_legend_window(ctx);
            }

            if self.state.show_damage_bars {
                self.show_damage_bar_window(ctx);
            }

            if self.state.show_character_damage {
                self.show_character_damage_window(ctx);
            }

            if self.state.show_real_time_damage {
                self.show_real_time_damage_window(ctx);
            }

            if self.state.show_battle_metrics {
                self.show_battle_metrics_window(ctx);
            }

            if self.state.show_enemy_stats {
                self.show_enemy_stats_window(ctx);
            }
        }

        if ctx.input_mut(|i| i.consume_shortcut(&HIDE_UI_SHORTCUT)) {
            self.state.hide_ui = !self.state.hide_ui;
        }

        if ctx.input_mut(|i| i.consume_shortcut(&SHOW_MENU_SHORTCUT)) {
            if self.state.hide_ui {
                self.notifs.info(t!(
                    "`Hide UI` is still active. Use the `Hide UI` shortcut to unhide the UI."
                ));
            }
        }

        if let Some(Some(update)) = self.update_inbox.read(ctx).last() {
            if let Some(new_version) = &update.new_version {
                match &update.status {
                    Some(status) => {
                        match status {
                            Status::Failed(e) => {
                                self.notifs.error(t!("Update failed: %{error}", error = e))
                            }
                            Status::Succeeded => self.notifs.success(t!("Update succeeded")),
                        };
                    }
                    None => {
                        self.notifs
                            .info(t!(
                                "Version %{version} is available!",
                                version = new_version
                            ))
                            .closable(true)
                            .show_progress_bar(true)
                            .duration(Some(std::time::Duration::from_secs_f32(10.0)));
                    }
                }
            }
            self.state.update_bttn_enabled = true;
            self.update = Some(update);
        }

        if let Some(export_notification) = self.export_inbox.read(ctx).last() {
            match export_notification {
                ExportNotification::Success => {
                    self.notifs
                        .success("Battle data auto-exported successfully!");
                }
                ExportNotification::Error { message } => {
                    self.notifs
                        .error(format!("Auto-export failed: {}", message));
                }
            }
        }

        if env!("CARGO_PKG_VERSION") != self.update_config.version {
            self.state.show_changelog = true;
        }

        if let Some(state) = BattleContext::get_instance().state.take() {
            match state {
                crate::battle::BattleState::Started => {
                    if self.config.auto_showhide_ui {
                        self.state.hide_ui = false;
                    }
                }
                crate::battle::BattleState::Ended => {
                    if self.config.auto_showhide_ui {
                        self.state.hide_ui = true;
                    }

                    if self.state.auto_save_battle_data {
                        let export_data = BattleContext::take_prepared_export_data();
                        let csv_data = BattleContext::take_prepared_csv_data();

                        match (export_data, csv_data) {
                            (Some(export_data), Some(csv_data)) => {
                                let custom_path = self.state.custom_export_path.clone();
                                let auto_create_date_folders = self.state.auto_create_date_folders;
                                let export_sender = self.export_inbox.sender();

                                RUNTIME.spawn(async move {
                                    use std::time::{SystemTime, UNIX_EPOCH};

                                    let timestamp = SystemTime::now()
                                        .duration_since(UNIX_EPOCH)
                                        .unwrap_or_default()
                                        .as_secs();

                                    let json_filename = format!(
                                        "{}_{}/{}.json",
                                        env!("CARGO_PKG_NAME"),
                                        timestamp,
                                        env!("CARGO_PKG_NAME")
                                    );
                                    let json_result = export_json_data(
                                        &export_data,
                                        &json_filename,
                                        custom_path.as_deref(),
                                        auto_create_date_folders,
                                    );

                                    let csv_filename = format!(
                                        "{}_{}/{}.csv",
                                        env!("CARGO_PKG_NAME"),
                                        timestamp,
                                        env!("CARGO_PKG_NAME")
                                    );
                                    let csv_result = export_csv_data(
                                        &csv_data,
                                        &csv_filename,
                                        custom_path.as_deref(),
                                        auto_create_date_folders,
                                    );

                                    match (json_result, csv_result) {
                                        (Ok(json_path), Ok(csv_path)) => {
                                            log::info!("Auto-exported JSON to: {}", json_path);
                                            log::info!("Auto-exported CSV to: {}", csv_path);
                                            let _ = export_sender.send(ExportNotification::Success);
                                        }
                                        (Err(e), _) | (_, Err(e)) => {
                                            log::error!("Failed to auto-export: {}", e);
                                            let _ = export_sender.send(ExportNotification::Error {
                                                message: e.to_string(),
                                            });
                                        }
                                    }
                                });
                            }
                            _ => {
                                log::warn!("No prepared export data found");
                                self.notifs
                                    .error("Auto-export failed: No battle data available");
                            }
                        }
                    }
                }
            }
        }

        self.notifs.show(ctx);
    }

    fn window_process(
        &mut self,
        input: &InputResult,
        input_events: &Vec<egui::Event>,
    ) -> Option<WindowProcessOptions> {
        // Refactor later
        match input {
            InputResult::Key => {
                for e in input_events {
                    match e {
                        egui::Event::Key {
                            key,
                            physical_key: _,
                            pressed,
                            repeat: _,
                            modifiers,
                        } => {
                            if modifiers.matches_exact(SHOW_MENU_SHORTCUT.modifiers)
                                && *key == SHOW_MENU_SHORTCUT.logical_key
                                && *pressed
                            {
                                self.state.show_menu = !self.state.show_menu;

                                return Some(WindowProcessOptions {
                                    // Simulate alt to get cursor
                                    window_message: Some(WindowMessage {
                                        msg: WM_KEYDOWN,
                                        wparam: WPARAM(VK_MENU.0 as _),
                                        lparam: LPARAM(0),
                                    }),
                                    ..Default::default()
                                });
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        };

        if !self.state.hide_ui && self.state.show_menu {
            Some(WindowProcessOptions {
                should_capture_all_input: true,
                ..Default::default()
            })
        } else {
            Some(WindowProcessOptions::default())
        }
    }

    fn save(&mut self, _storage: &mut egui::Memory) {
        self.save_persist(_storage)
            .unwrap_or_else(|e| log::error!("{e}"));
        self.state.save().unwrap_or_else(|e| log::error!("{e}"));

        self.config.theme = *self.colorix.theme();
        if self.colorix.dark_mode() {
            self.config.theme_mode = egui::Theme::Dark;
        } else {
            self.config.theme_mode = egui::Theme::Light;
        }

        self.config.save().unwrap_or_else(|e| log::error!("{e}"));
        self.update_config
            .save()
            .unwrap_or_else(|e| log::error!("{e}"));
    }
}

const PERSISTENCE_FILENAME: &'static str = "persistence";
const STATE_FILENAME: &'static str = "state";

impl Default for AppState {
    fn default() -> Self {
        Self {
            show_menu: false,
            show_changelog: false,
            show_help: false,
            show_settings: false,
            show_console: false,
            show_damage_distribution: false,
            show_damage_type_breakdown: false,
            show_damage_bars: false,
            show_character_damage: false,
            show_real_time_damage: false,
            show_enemy_stats: false,
            show_battle_metrics: false,
            hide_ui: false,
            graph_x_unit: GraphUnit::default(),
            damage_bar_value: DamageBarValue::default(),
            damage_breakdown_scope: DamageBreakdownScope::default(),
            damage_breakdown_chart: DamageBreakdownChart::default(),
            damage_breakdown_character_index: 0,
            show_damage_breakdown_table: false,
            show_damage_breakdown_legend: false,
            use_custom_color: false,
            update_bttn_enabled: false,
            center_updater_window: false,
            show_character_legend: false,
            auto_save_battle_data: false,
            show_export_window: false,
            show_updater_window: false,
            custom_export_path: None,
            auto_create_date_folders: true,
        }
    }
}

impl AppState {
    fn load() -> Result<Self> {
        match ProjectDirs::from("", "", env!("CARGO_PKG_NAME")) {
            Some(proj_dirs) => {
                let data_local_dir = proj_dirs.data_local_dir();
                let state_path = data_local_dir.join(STATE_FILENAME);

                if state_path.exists() {
                    let mut file = File::open(&state_path)?;
                    let mut buffer = String::new();
                    file.read_to_string(&mut buffer)?;
                    Ok(ron::from_str(&buffer)?)
                } else {
                    Ok(Self::default())
                }
            }
            None => Err(anyhow!("Failed to load/create data project dirs.")),
        }
    }

    fn save(&mut self) -> Result<()> {
        match ProjectDirs::from("", "", env!("CARGO_PKG_NAME")) {
            Some(proj_dirs) => {
                let data_local_dir = proj_dirs.data_local_dir();
                let state_path = data_local_dir.join(STATE_FILENAME);

                if !state_path.exists() {
                    std::fs::create_dir_all(data_local_dir)?;
                }
                let mut file = File::create(state_path)?;
                file.write(ron::to_string(&self)?.as_bytes())?;
                file.flush()?;
                Ok(())
            }
            None => Err(anyhow!("Failed to load/create data project dirs.")),
        }
    }
}

impl App {
    fn load_persist(ctx: &Context) -> Result<()> {
        match ProjectDirs::from("", "", env!("CARGO_PKG_NAME")) {
            Some(proj_dirs) => {
                let data_local_dir = proj_dirs.data_local_dir();
                let persist_path = data_local_dir.join(PERSISTENCE_FILENAME);

                if persist_path.exists() {
                    let mut file = File::open(&persist_path)?;
                    let mut buffer = String::new();
                    file.read_to_string(&mut buffer)?;
                    let memory: Memory = ron::from_str(&buffer)?;
                    ctx.memory_mut(|writer| {
                        *writer = memory;
                    });
                }

                Ok(())
            }
            None => Err(anyhow!("Failed to load/create data project dirs.")),
        }
    }

    fn save_persist(&mut self, _storage: &mut egui::Memory) -> Result<()> {
        match ProjectDirs::from("", "", env!("CARGO_PKG_NAME")) {
            Some(proj_dirs) => {
                let data_local_dir = proj_dirs.data_local_dir();
                let persist_path = data_local_dir.join(PERSISTENCE_FILENAME);

                if !persist_path.exists() {
                    std::fs::create_dir_all(data_local_dir)?;
                }
                let mut file = File::create(persist_path)?;
                file.write(ron::to_string(_storage)?.as_bytes())?;
                file.flush()?;
                Ok(())
            }
            None => Err(anyhow!("Failed to load/create data project dirs.")),
        }
    }

    pub fn new(ctx: Context) -> Self {
        // This should be called first
        if App::load_persist(&ctx).is_err() {
            log::error!("Failed to load persistence.");
        }

        let fonts = vec![("zh-cn.ttf", "game_font"), ("ja-jp.ttf", "game_font_jp")];

        for font in fonts {
            let path = format!(
                r"StarRail_Data\StreamingAssets\MiHoYoSDKRes\HttpServerResources\font\{}",
                font.0
            );
            match std::fs::read(&path) {
                Ok(font_data) => {
                    ctx.add_font(FontInsert::new(
                        font.1,
                        egui::FontData::from_owned(font_data),
                        vec![
                            InsertFontFamily {
                                family: egui::FontFamily::Proportional,
                                priority: egui::epaint::text::FontPriority::Highest,
                            },
                            InsertFontFamily {
                                family: egui::FontFamily::Monospace,
                                priority: egui::epaint::text::FontPriority::Lowest,
                            },
                        ],
                    ));
                }
                Err(e) => log::warn!(
                    "{} : Could not locate {}. Defaulting to default font.",
                    e,
                    path
                ),
            }
        }

        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Bold);

        ctx.set_fonts(fonts);

        let config = Config::new(&ctx).unwrap_or_else(|e| {
            log::error!("{e}");
            Config::default()
        });

        let update_config = super::config::UpdateConfig::new().unwrap_or_else(|e| {
            log::error!("{e}");
            super::config::UpdateConfig::default()
        });

        let mut app = Self {
            colorix: Colorix::global(&ctx, config.theme),
            config,
            update_config,
            notifs: Toasts::default(),
            state: AppState::load().unwrap_or_else(|e| {
                log::error!("{e}");
                AppState::default()
            }),
            update_inbox: UiInbox::new(),
            export_inbox: UiInbox::new(),
            update: None,
            updater_hint: None,
            updater_window_last_size: None,
        };

        rust_i18n::set_locale(&app.config.locale);
        match app.config.theme_mode {
            egui::Theme::Dark => {
                app.colorix
                    .set_dark(&mut Ui::new(ctx.clone(), "".into(), UiBuilder::new()))
            }
            egui::Theme::Light => {
                app.colorix
                    .set_light(&mut Ui::new(ctx.clone(), "".into(), UiBuilder::new()))
            }
        }

        app.queue_update_check();

        app
    }

    pub fn set_beta_flag(&mut self, enabled: bool) -> bool {
        self.update_config.beta = enabled;
        self.update = None;
        self.state.update_bttn_enabled = true;
        self.queue_update_check();
        true
    }

    fn queue_update_check(&self) {
        let sender = self.update_inbox.sender();
        let allow_prerelease = self.update_config.beta;
        RUNTIME.spawn(async move {
            match Updater::new(env!("CARGO_PKG_VERSION"), allow_prerelease)
                .check_update()
                .await
            {
                Ok(new_ver) => {
                    if sender
                        .send(Some(Update {
                            new_version: new_ver,
                            status: None,
                        }))
                        .is_err()
                    {
                        log::error!("Failed to send update to inbox");
                    }
                }
                Err(e) => {
                    log::error!("Update check failed: {e}");
                    if sender
                        .send(Some(Update {
                            new_version: None,
                            status: Some(Status::Failed(e)),
                        }))
                        .is_err()
                    {
                        log::error!("Failed to send update-failure to inbox");
                    }
                }
            }
        });
    }

    pub fn export_battle_data(&self, format: &str) -> Result<String, Box<dyn std::error::Error>> {
        let battle_context = BattleContext::get_instance();
        let exporter = BattleDataExporter::new();
        let custom_path = self.state.custom_export_path.as_deref();

        match format {
            "json" => exporter.export_to_file_with_custom_path(
                &battle_context,
                None,
                custom_path,
                self.state.auto_create_date_folders,
            ),
            "csv" => exporter.export_to_csv_with_custom_path(
                &battle_context,
                None,
                custom_path,
                self.state.auto_create_date_folders,
            ),
            _ => Err("Unsupported format".into()),
        }
    }

    pub fn open_folder(&mut self, path: &str) {
        #[cfg(target_os = "windows")]
        {
            if let Err(e) = std::process::Command::new("explorer").arg(path).spawn() {
                self.notifs.error(format!("Failed to open folder: {}", e));
                log::error!("Failed to open folder: {}", e);
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            if let Err(e) = std::process::Command::new("xdg-open").arg(path).spawn() {
                self.notifs.error(format!("Failed to open folder: {}", e));
                log::error!("Failed to open folder: {}", e);
            }
        }
    }
}

fn export_json_data(
    export_data: &crate::export::ExportBattleData,
    filename: &str,
    custom_path: Option<&str>,
    auto_create_date_folders: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    use crate::export::BattleDataExporter;

    let json = serde_json::to_string_pretty(export_data)?;

    let export_dir = BattleDataExporter::get_export_directory_with_custom_path(
        custom_path,
        auto_create_date_folders,
    )?;
    let full_path = export_dir.join(filename);

    if let Some(parent) = full_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(&full_path, &json)?;
    Ok(full_path.to_string_lossy().to_string())
}

fn export_csv_data(
    csv_data: &[crate::export::ComprehensiveData],
    filename: &str,
    custom_path: Option<&str>,
    auto_create_date_folders: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    use crate::export::BattleDataExporter;

    let export_dir = BattleDataExporter::get_export_directory_with_custom_path(
        custom_path,
        auto_create_date_folders,
    )?;
    let full_path = export_dir.join(filename);

    if let Some(parent) = full_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut wtr = csv::Writer::from_path(&full_path)?;
    for record in csv_data {
        wtr.serialize(record)?;
    }
    wtr.flush()?;

    Ok(full_path.to_string_lossy().to_string())
}
