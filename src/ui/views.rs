use egui::{Slider, TextEdit, Ui};
use egui::{CentralPanel, CollapsingHeader, Color32, Frame, Label, Memory, RichText, ScrollArea, Stroke, Window};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use anyhow::anyhow;

use crate::LOCALES;
use crate::export::BattleDataExporter;
use crate::ui::themes;
use crate::{CHANGELOG, RUNTIME, entry::InitErrorInfo, ui::{app::App, helpers::{get_transparent_window_frame, get_window_frame}}, updater::{Status, Update, Updater}};

impl App {
    pub fn show_changelog_window(&mut self, ctx: &egui::Context) {
        let changelog = parse_changelog::parse(CHANGELOG).unwrap();

        if let Some(release) = changelog.get(env!("CARGO_PKG_VERSION")) {
            Window::new(t!("Changelog"))
                .id("changelog_window".into())
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .collapsible(false)
                .resizable(true)
                .frame(Frame::window(&ctx.style()).inner_margin(5.))
                .show(ctx, |ui| {
                    ScrollArea::new([false, true]).show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading(release.title);

                            let mut cache = CommonMarkCache::default();
                            CommonMarkViewer::new().show(ui, &mut cache, release.notes);

                            ui.add_space(5.);

                            if ui.button(t!("Close")).clicked() {
                                self.state.show_changelog = false;
                                self.config.version = env!("CARGO_PKG_VERSION").to_string();
                            }
                        });
                    });
                });
        }
    }

    pub fn show_help_window(&mut self, ctx: &egui::Context) {
        Window::new(format!("{} {}", egui_phosphor::bold::QUESTION, t!("Help")))
            .id("help_window".into())
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .collapsible(false)
            .resizable(true)
            .frame(Frame::window(&ctx.style()).inner_margin(5.))
            .show(ctx, |ui| {
                ScrollArea::new([false, true]).show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        let markup = indoc::indoc!("
                            # [Shortcuts](https://github.com/hessiser/veritas/wiki/Home/#shortcuts)
                            - `Ctrl` + `M` to toggle menu
                            - `Ctrl` + `H` to hide the UI
                            - `Ctrl` + `+` to zoom in
                            - `Ctrl` + `-` to zoom out
                            - `Ctrl` + `0` to reset zoom

                            # [FAQ](https://github.com/hessiser/veritas/wiki/Home/#troubleshooting)
                            - **How do I reset my graphs?**

                            Double-click the graph to reset. Alternatively, can delete `persistence` in `appdata/local/veritas/data` and restart.

                            - **The game is not processing keyboard/mouse inputs.**

                            If your mouse is hovering over the overlay, it will consume all mouse inputs. If the overlay is taking keyboard inputs, it will consume all keyboard inputs as well. Either move your mouse away or click around the overlay, or use the hide UI shortcut.

                            - **`[Error] Client is damaged, please reinstall the client.` on official servers.**

                            Follow instructions [here](https://github.com/hessiser/veritas/wiki/Home/#method-1-recommended-for-official-servers).
                        ");
                        let mut cache = CommonMarkCache::default();
                        CommonMarkViewer::new().show(ui, &mut cache, markup);

                        ui.add_space(5.);

                        if ui.button(t!("Close")).clicked() {
                            self.state.show_help = false;
                            self.config.version = env!("CARGO_PKG_VERSION").to_string();
                        }
                    });
                });
            });
    }

    pub fn show_version_mismatch_popup(&mut self, ctx: &egui::Context) {
        // hacky fix
        if self.state.show_changelog {
            self.state.show_changelog = false;
            self.reopen_changelog = true;
        }
        Window::new(
            RichText::new(format!(
                "{} Version mismatch detected",
                egui_phosphor::bold::WARNING
            ))
            .size(24.0),
        )
        .id("version_mismatch_popup".into())
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .default_width(440.0)
        .min_width(320.0)
        .frame(Frame::window(&ctx.style()).inner_margin(14.0))
        .collapsible(false)
        .resizable(true)
        .show(ctx, |ui| {
        const POPUP_MIN_WIDTH: f32 = 320.0;
        const POPUP_PREFERRED_WIDTH: f32 = 460.0;

        ui.scope(|ui| {
            {
                let spacing = ui.spacing_mut();
                spacing.item_spacing = egui::vec2(0.0, 12.0);
                spacing.button_padding = egui::vec2(16.0, 8.0);
            }

            let available_width = ui.available_width();
            let min_width = available_width.min(POPUP_MIN_WIDTH);
            ui.set_min_width(min_width);
            ui.set_max_width(POPUP_PREFERRED_WIDTH);

            ui.add(
                Label::new(
                    RichText::new(format!(
                        "{} Veritas couldn't start because the version you have was built for a different version of the game",
                        egui_phosphor::regular::SMILEY_SAD
                    ))
                    .size(16.0)
                    .strong(),
                )
                .wrap(),
            );

            if let Some(info) = &self.init_err {
                let message = match info {
                    InitErrorInfo::Other { message } => message,
                    InitErrorInfo::ObfuscationMismatch { message, .. } => message,
                };

                if !message.is_empty() {
                    CollapsingHeader::new("Error details")
                        .show_unindented(ui, |ui| {
                            ui.add(Label::new(message).wrap());
                        });
                }
            }

            ui.add(
                Label::new(
                    RichText::new(format!(
                        "{} But don't worry, we can fix it!",
                        egui_phosphor::regular::SMILEY
                    ))
                    .size(16.0)
                    .strong(),
                )
                .wrap(),
            );

            ui.separator();
            
            ui.add(
                Label::new(
                    RichText::new("Pick the client you are currently playing on")
                        .size(15.0)
                        .strong(),
                )
                .wrap(),
            );

            ui.horizontal(|ui| {
                {
                    let spacing = ui.spacing_mut();
                    spacing.item_spacing.x = 16.0;
                }
                let button_width = ((ui.available_width() - 16.0).max(0.0)) / 2.0;

                if ui
                    .add_sized([button_width, 36.0], egui::Button::new(RichText::new("I'm on live client").strong()))
                    .clicked()
                {
                    self.pick_build(false);
                }

                if ui
                    .add_sized([button_width, 36.0], egui::Button::new(RichText::new("I'm on beta client").strong()))
                    .clicked()
                {
                    self.pick_build(true);
                }
            });

            ui.add_space(6.0);

            Frame::group(ui.style()).inner_margin(8.0).show(ui, |ui| {
                CollapsingHeader::new(
                    RichText::new(format!(
                        "{} {}",
                        egui_phosphor::regular::QUESTION,
                        "How do I check?"
                    ))
                    .strong(),
                )
                .show_unindented(ui, |ui| {
                    ui.spacing_mut().item_spacing.y = 6.0;
                    ui.add(Label::new(RichText::new("Look at the bottom-left corner of the screen when you have the game open").size(14.0).strong()).wrap());
                    ui.add(Label::new("If the text has OSBETA or CNBETA, you are on the beta client and need the beta version").wrap());
                    ui.add(Label::new("If the text has OSPROD or CNPROD, you are on the live client and need the live version").wrap());
                });
            });
            ui.add(Label::new("We'll point the updater at the right download so you can install a version that works with your game version").wrap());

            ui.checkbox(&mut self.skip_version_mismatch_popup, "Don't show this again");

            if ui.button("I'll handle it later").clicked() {
                self.close_version_mismatch_popup();
            }
        });
        });
    }

    pub fn show_menu(&mut self, ctx: &egui::Context) {
        CentralPanel::default()
            .frame(Frame {
                fill: Color32::BLACK.gamma_multiply(0.25),
                ..Default::default()
            })
            .show(ctx, |_ui: &mut egui::Ui| {
                Window::new(t!("Menu"))
                    .id("menu_window".into())
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .collapsible(false)
                    .show(ctx, |ui| {
                        // Settings
                        egui::Frame::default().inner_margin(5.0).show(ui, |ui| {
                            egui::MenuBar::new().ui(ui, |ui| {
                                ui.toggle_value(
                                    &mut self.state.show_settings,
                                    RichText::new(format!(
                                        "{} {}",
                                        egui_phosphor::bold::GEAR,
                                        t!("Settings")
                                    )),
                                );

                                ui.toggle_value(
                                    &mut self.state.show_export_window,
                                    RichText::new(format!(
                                        "{} Export",
                                        egui_phosphor::bold::DOWNLOAD_SIMPLE,
                                    )),
                                );

                                ui.toggle_value(
                                    &mut self.state.show_updater_window,
                                    RichText::new(format!(
                                        "{} Updates",
                                        egui_phosphor::bold::DOWNLOAD,
                                    )),
                                );

                                if ui
                                    .button(RichText::new(format!(
                                        "{} {}",
                                        egui_phosphor::bold::ARROW_COUNTER_CLOCKWISE,
                                        t!("Reset")
                                    )))
                                    .clicked()
                                {
                                    ctx.memory_mut(|writer| *writer = Memory::default());
                                }

                                if ui
                                    .button(RichText::new(format!(
                                        "{} {}",
                                        egui_phosphor::bold::QUESTION,
                                        t!("Help")
                                    )))
                                    .clicked()
                                {
                                    self.state.show_help = !self.state.show_help;
                                }

                                // ui.menu_button(RichText::new(format!(
                                //         "{} {}",
                                //         egui_phosphor::bold::COMMAND,
                                //         t!("Shortcuts")
                                //     )).strong(), |ui| {
                                //         let button = Button::new(RichText::new(t!("Show menu"))).shortcut_text(ctx.format_shortcut(&SHOW_MENU_SHORTCUT));
                                //         if ui.add(button).changed() {

                                //         };
                                //     });
                            });
                        });

                        ui.separator();

                        let mut show_settings = self.state.show_settings;
                        if show_settings {
                            egui::Window::new(format!("{} {}", egui_phosphor::bold::GEAR, t!("Settings")))
                                .id("settings_window".into())
                                .open(&mut show_settings)
                                .show(ctx, |ui| {
                                    self.show_settings(ui);
                                });
                            self.state.show_settings = show_settings;
                        }

                        let mut show_export_window = self.state.show_export_window;
                        if show_export_window {
                            Window::new(format!("{} Export Battle Data", egui_phosphor::bold::DOWNLOAD_SIMPLE))
                                .id("export_window".into())
                                .open(&mut show_export_window)
                                .show(ctx, |ui| {
                                    self.show_export_window(ui);
                                });
                            self.state.show_export_window = show_export_window;
                        }

                        let mut show_updater_window = self.state.show_updater_window;
                        if show_updater_window {
                            let mut updater_window = Window::new(format!(
                                "{} Updates",
                                egui_phosphor::bold::DOWNLOAD
                            ))
                            .id("updater_window".into())
                            .open(&mut show_updater_window);

                            if self.state.center_updater_window {
                                let center = ctx.input(|input| input.content_rect().center());
                                if let Some(size) = self.updater_window_last_size {
                                    let top_left = center - size * 0.5;
                                    updater_window = updater_window.current_pos(top_left);
                                    self.state.center_updater_window = false;
                                } else {
                                    updater_window = updater_window
                                        .pivot(egui::Align2::CENTER_CENTER)
                                        .current_pos(center);
                                }
                            }

                            if let Some(response) = updater_window.show(ctx, |ui| {
                                self.show_updater_window(ui);
                            }) {
                                self.updater_window_last_size = Some(response.response.rect.size());
                            }
                            self.state.show_updater_window = show_updater_window;
                            if !self.state.show_updater_window {
                                self.updater_hint = None;
                            }
                        }

                        ui.vertical_centered(|ui| {
                            ui.add_space(5.);
                            ui.checkbox(&mut self.state.show_console, t!("Show Logs"));
                            ui.checkbox(
                                &mut self.state.show_character_legend,
                                t!("Show Character Legend"),
                            );
                            ui.checkbox(
                                &mut self.state.show_damage_distribution,
                                t!("Show Damage Distribution"),
                            );
                            ui.checkbox(
                                &mut self.state.show_damage_type_breakdown,
                                t!("Show Damage Type Breakdown"),
                            );
                            ui.checkbox(
                                &mut self.state.show_damage_bars,
                                t!("Show Damage Bars"),
                            );
                            ui.checkbox(
                                &mut self.state.show_real_time_damage,
                                t!("Show Real-Time Damage"),
                            );
                            ui.checkbox(
                                &mut self.state.show_enemy_stats,
                                t!("Show Enemy Stats"),
                            );

                            ui.checkbox(
                                &mut self.state.show_battle_metrics,
                                t!("Show Battle Metrics"),
                            );

                            ui.add_space(5.);

                            ui.separator();
                            if ui.button(t!("Close")).clicked() {
                                self.state.show_menu = false;
                            }
                        });
                    });
            });
    }

    fn show_updater_window(&mut self, ui: &mut Ui) {
        if let Some(hint) = self.updater_hint.as_deref() {
            Frame::group(ui.style())
                .inner_margin(10.0)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!("{} Listen...", egui_phosphor::regular::LIGHTBULB))
                                .strong(),
                        );
                    });
                    ui.add(Label::new(hint).wrap());
                });

            ui.add_space(12.0);
        }

        ui.group(|ui| {
            ui.label(RichText::new(format!("{} Version Information", egui_phosphor::regular::INFO)).strong());
            
            let current_version = env!("CARGO_PKG_VERSION");
            if let Some(new_update) = &self.update {
                if let Some(new_version) = &new_update.new_version {
                    ui.colored_label(Color32::GREEN, t!("Version %{version} is available!", version = new_version));
                    ui.horizontal(|ui| {
                        ui.label(format!(
                            "{} ➡ {}",
                            current_version,
                            new_version
                        ));
                    });
                    
                    ui.add_space(8.0);
                    
                    if ui
                        .add_enabled(self.state.update_bttn_enabled, egui::Button::new(format!("{} Update Now", egui_phosphor::bold::DOWNLOAD)))
                        .clicked()
                    {
                        self.updater_hint = None;
                        let defender_exclusion = self.config.defender_exclusion;
                        let new_version = new_version.clone();
                        let sender = self.update_inbox.sender();
                        self.state.update_bttn_enabled = false;
                        self.notifs.success(t!("Update in progress"));
                        RUNTIME.spawn(async move {
                            let status = if let Err(e) = Updater::new(env!("CARGO_PKG_VERSION"))
                                .download_update(defender_exclusion)
                                .await
                            {
                                Some(Status::Failed(e))
                            }
                            else {
                                Some(Status::Succeeded)
                            };

                            if sender.send(Some(Update { new_version: Some(new_version.to_string()), status})).is_err() {
                                let e = anyhow!("Failed to send update to inbox");
                                log::error!("{e}");
                            }

                        });
                    }
                } else {
                    ui.horizontal(|ui| {
                        ui.label("Current version:");
                        ui.monospace(current_version);
                    });
                    ui.colored_label(Color32::LIGHT_GREEN, "You have the latest version!");
                }
            } else {
                ui.horizontal(|ui| {
                    ui.label("Current version:");
                    ui.monospace(current_version);
                });
                ui.label("Checking for updates...");
            }
        });
        
        ui.add_space(12.0);
        
        ui.group(|ui| {
            ui.label(RichText::new(format!("{} Settings", egui_phosphor::regular::GEAR)).strong());
            let prev_beta = self.beta_channel;
            ui.horizontal(|ui| {
                let changed = ui
                    .checkbox(&mut self.beta_channel, "Check beta updates (pre-release)")
                    .changed();

                ui.add(
                    egui::widgets::Label::new(
                        egui::RichText::new(egui_phosphor::regular::INFO).size(16.0),
                    )
                    .sense(egui::Sense::hover()),
                )
                .on_hover_text(
                    "Only enable this if you're running on a beta client, installing a DLL meant for the newest beta client on release client (current official version of the game) might break things",
                );

                if changed && !self.set_beta_flag(self.beta_channel) {
                    self.beta_channel = prev_beta;
                }
            });
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.config.defender_exclusion, t!("Add Defender Exclusion during update"));
                ui.add(egui::widgets::Label::new(egui::RichText::new(egui_phosphor::regular::INFO).size(16.0))
                    .sense(egui::Sense::hover()))
                    .on_hover_text(t!(indoc::indoc!("
                        If enabled, the updater will temporarily add the new DLL file to Windows Defender exclusions during update to avoid false positives.
                        This is recommended to be enabled (if disabled, Windows Defender may cause the update to fail) however you can disable it if you prefer. The exclusion is removed after the update is finished.
                    ")));
            });
        });
    }

    fn show_export_window(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.label(RichText::new(format!("{} Export Current/Last Played Battle", egui_phosphor::regular::UPLOAD)).strong());

            ui.add_space(4.0);

            ui.horizontal(|ui| {
                if ui.button(format!("{} Export JSON", egui_phosphor::bold::FILE_TEXT))
                    .clicked() 
                {
                    match self.export_battle_data("json") {
                        Ok(filepath) => {
                            self.notifs.success("JSON exported successfully!");
                            log::info!("JSON file exported to: {}", filepath);
                        }
                        Err(e) => {
                            self.notifs.error(format!("Failed to export JSON: {}", e));
                            log::error!("Failed to export JSON: {}", e);
                        }
                    }
                }

                if ui.button(format!("{} Export CSV", egui_phosphor::bold::FILE_CSV))
                    .clicked() 
                {
                    match self.export_battle_data("csv") {
                        Ok(filepath) => {
                            self.notifs.success("CSV exported successfully!");
                            log::info!("CSV file exported to: {}", filepath);
                        }
                        Err(e) => {
                            self.notifs.error(format!("Failed to export CSV: {}", e));
                            log::error!("Failed to export CSV: {}", e);
                        }
                    }
                }
            });
            
            ui.add_space(8.0);

            CollapsingHeader::new(format!("{} Format Information", egui_phosphor::regular::INFO))
                .id_salt("format_info_header")
                .default_open(false)
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(format!("{}", egui_phosphor::regular::FILE_TEXT));
                        ui.label("JSON format: Compatible with");
                        ui.hyperlink_to("Firefly Analysis", "https://sranalysis.kain.id.vn/");
                        ui.label("for detailed battle analysis");
                    });
                    
                    ui.horizontal_wrapped(|ui| {
                        ui.label(format!("{}", egui_phosphor::regular::FILE_CSV));
                        ui.label("CSV format: Spreadsheet-friendly data for creating custom charts and graphs");
                    });
                });

            ui.add_space(8.0);
            
            if ui.button(format!("{} Open Export Folder", egui_phosphor::bold::FOLDER_OPEN))
                .clicked() 
            {
                match BattleDataExporter::get_export_directory_path() {
                    Ok(dir_path) => {
                        self.open_folder(&dir_path);
                    }
                    Err(e) => {
                        self.notifs.error(format!("Failed to get export directory: {}", e));
                        log::error!("Failed to get export directory: {}", e);
                    }
                }
            }
            
            ui.add_space(8.0);
            
            ui.label("Export Folder Location:");
            if let Some(custom_path) = self.state.custom_export_path.clone() {
                ui.horizontal(|ui| {
                    ui.monospace(&custom_path);
                    if ui.button(format!("{} Change", egui_phosphor::regular::FOLDER_OPEN)).clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            let path_str = path.to_string_lossy().to_string();
                            self.state.custom_export_path = Some(path_str);
                            self.state.auto_create_date_folders = false;
                        }
                    }
                    if ui.button(format!("{} Reset to Default", egui_phosphor::regular::ARROW_COUNTER_CLOCKWISE)).clicked() {
                        self.state.custom_export_path = None;
                        self.state.auto_create_date_folders = true;
                    }
                });
            } else {
                if let Ok(dir_path) = BattleDataExporter::get_export_directory_path() {
                    ui.horizontal(|ui| {
                        ui.monospace(&dir_path);
                        if ui.button(format!("{} Change", egui_phosphor::regular::FOLDER_OPEN)).clicked() {
                            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                let path_str = path.to_string_lossy().to_string();
                                self.state.custom_export_path = Some(path_str);
                                self.state.auto_create_date_folders = false;
                            }
                        }
                    });
                }
            }
        });
        
        ui.add_space(12.0);
        
        ui.group(|ui| {
            ui.label(RichText::new(format!("{} Settings", egui_phosphor::regular::GEAR)).strong());
            
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.state.auto_save_battle_data, "Auto-export data after battle ends");
                ui.add(egui::widgets::Label::new(egui::RichText::new(egui_phosphor::regular::INFO).size(16.0))
                    .sense(egui::Sense::hover()))
                    .on_hover_text("Automatically exports the most recent battle's data in both JSON and CSV formats immediately after the battle ends");
            });
            
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.state.auto_create_date_folders, "Auto-create date folders");
                ui.add(egui::widgets::Label::new(egui::RichText::new(egui_phosphor::regular::INFO).size(16.0))
                    .sense(egui::Sense::hover()))
                    .on_hover_text("Automatically organize exported data files into date-based folders (YYYY-MM-DD)");
            });
        });
    }


    // rename
    fn show_settings(&mut self, ui: &mut Ui) {
        egui::MenuBar::new().ui(ui, |ui| {
            let style = ui.ctx().style();
            let font_id = &style.text_styles[&egui::TextStyle::Button];
            let font_size = font_id.size;
            self.colorix.light_dark_toggle_button(ui, font_size);

            ui.separator();

            ui.menu_button(
                RichText::new(format!("{} {}", egui_phosphor::bold::GLOBE, t!("Language"))),
                |ui| {
                    for locale_code in rust_i18n::available_locales!() {
                        if let Some(locale) = LOCALES.get(locale_code) {
                            if ui.button(*locale).clicked() {
                                self.config.locale = locale_code.to_owned();
                                rust_i18n::set_locale(locale_code);
                                ui.close();
                            }
                        }
                    }
                },
            );

            ui.toggle_value(
                &mut self.config.streamer_mode,
                RichText::new(format!(
                    "{} {}",
                    egui_phosphor::bold::VIDEO_CAMERA,
                    t!("Streamer Mode")
                )),
            );
        });

        ui.separator();

        ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
            ui.add_space(5.);

            CollapsingHeader::new(t!("Theme"))
                .id_salt("theme_header")
                .show(ui, |ui| {
                    if self.state.use_custom_color {
                        self.colorix.twelve_from_custom(ui);
                    };

                    ui.horizontal(|ui| {
                        self.colorix.custom_picker(ui);
                        ui.toggle_value(
                            &mut self.state.use_custom_color,
                            t!("Custom color. Click here to enable."),
                        );
                    });

                    if self.colorix.dark_mode() {
                        self.colorix.themes_dropdown(
                            ui,
                            Some((themes::THEME_NAMES.to_vec(), themes::THEMES.to_vec())),
                            true,
                        );
                    } else {
                        self.colorix.themes_dropdown(ui, None, false);
                    }

                    self.colorix.ui_combo_12(ui, false);
                });

            ui.horizontal(|ui| {
                ui.horizontal(|ui| {
                    let all_text_styles = ui.style().text_styles();
                    for style in all_text_styles {
                        ui.selectable_value(
                            &mut self.config.legend_text_style,
                            style.clone(),
                            style.to_string(),
                        );
                    }
                });
                ui.label(t!("Legend Text Style"));
            });

            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(
                    &mut self.config.pie_chart_opacity,
                    0.0..=1.0,
                ));
                ui.label(t!("Pie Chart Opacity"));
            });

            ui.add(
                Slider::new(&mut self.config.widget_opacity, 0.0..=1.0).text(t!("Window Opacity")),
            );

            CollapsingHeader::new(t!("Fonts"))
                .id_salt("fonts_header")
                .show(ui, |ui| {
                    for (style, id) in &mut self.config.font_sizes {
                        let label = format!("{:?}", style);
                        ui.add(Slider::new(&mut id.size, 8.0..=48.0).text(label));
                    }

                    let font_sizes = self.config.font_sizes.clone();
                    ui.ctx().all_styles_mut(move |style| {
                        style.text_styles = font_sizes.clone();
                    });
                });

            ui.checkbox(
                &mut self.config.auto_showhide_ui,
                t!("Auto(show/hide) UI on battle (start/end)."),
            );

            if ui
                .checkbox(
                    &mut self.config.nag_versions,
                    "Show version mismatch help when startup fails",
                )
                .changed()
            {
                if let Err(e) = self.config.save() {
                    log::error!("{e}");
                }
            }

            // TODO:
            // Change using a grid like so:

            // ui.label("Text style:");
            // ui.horizontal(|ui| {
            //     let all_text_styles = ui.style().text_styles();
            //     for style in all_text_styles {
            //         ui.selectable_value(&mut config.text_style, style.clone(), style.to_string());
            //     }
            // });
            // ui.end_row();

            // if ui
            //     .add(
            //         Slider::new(
            //             &mut self.settings.fps,
            //             10..=120,
            //         )
            //         .text(t!("FPS")),
            //     )
            //     .changed()
            // {
            //     self.config.set_fps(self.settings.fps);
            //     unsafe {
            //         Application_set_targetFrameRate(
            //             self.settings.fps,
            //         )
            //     };
            // }

            ui.add(
                TextEdit::singleline(&mut self.config.streamer_msg).hint_text(RichText::new(
                    format!(
                        "{} {}",
                        t!("Streamer Message. Can also use Phosphor Icons!"),
                        egui_phosphor::bold::RAINBOW
                    ),
                )),
            );
        });
    }


    // Should I just create a macro?
    pub fn show_console_window(&mut self, ctx: &egui::Context) {
        egui::Window::new(t!("Log"))
            .id("log_window".into())
            .resizable(true)
            .default_height(300.0)
            .default_width(400.0)
            .min_width(200.0)
            .min_height(100.0)
            .show(ctx, |ui| {
                let available = ui.available_size();
                ui.set_min_size(available);
                ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                    egui_logger::logger_ui().show(ui);
                });
            });
    }

    pub fn show_damage_distribution_window(&mut self, ctx: &egui::Context) {
        let damage_distribution_window_title = if self.state.show_menu {
            t!("Damage Distribution").into_owned()
        } else {
            String::new()
        };

        egui::containers::Window::new(damage_distribution_window_title)
            .id("damage_distribution_window".into())
            .frame(if self.state.show_menu {
                get_window_frame(ctx, self.config.widget_opacity)
            } else {
                get_transparent_window_frame(ctx, self.config.widget_opacity)
            })
            .collapsible(false)
            .resizable(true)
            .min_width(200.0)
            .min_height(200.0)
            .show(ctx, |ui| {
                self.show_damage_distribution_widget(ui);
            });
    }

    pub fn show_damage_type_breakdown_window(&mut self, ctx: &egui::Context) {
        egui::containers::Window::new(t!("Damage Type Breakdown"))
            .id("damage_type_breakdown_window".into())
            .frame(get_window_frame(ctx, self.config.widget_opacity))
            .resizable(true)
            .default_width(420.0)
            .default_height(360.0)
            .min_width(280.0)
            .min_height(180.0)
            .show(ctx, |ui| {
                self.show_damage_type_breakdown_widget(ui);
            });
    }

    pub fn show_character_legend_window(&mut self, ctx: &egui::Context) {
        egui::containers::Window::new(t!("Character Legend"))
            .id("character_legend_window".into())
            .frame(get_window_frame(ctx, self.config.widget_opacity))
            .resizable(true)
            .min_width(200.0)
            .min_height(200.0)
            .show(ctx, |ui| {
                self.show_character_legend(ui);
            });
    }

    pub fn show_damage_bar_window(&mut self, ctx: &egui::Context) {
        egui::containers::Window::new(t!("Character Damage"))
            .id("damage_by_character_window".into())
            .frame(get_window_frame(ctx, self.config.widget_opacity))
            .resizable(true)
            .min_width(200.0)
            .min_height(200.0)
            .show(ctx, |ui| {
                self.show_damage_bar_widget(ui);
            });
    }

    pub fn show_real_time_damage_window(&mut self, ctx: &egui::Context) {
        egui::containers::Window::new(t!("Real-Time Damage"))
            .id("real_time_damage_window".into())
            .frame(get_window_frame(ctx, self.config.widget_opacity))
            .resizable(true)
            .min_width(200.0)
            .min_height(200.0)
            .show(ctx, |ui| {
                self.show_real_time_damage_graph_widget(ui);
            });
    }

    pub fn show_battle_metrics_window(&mut self, ctx: &egui::Context) {
        egui::containers::Window::new(t!("Battle Metrics"))
            .id("battle_metrics_window".into())
            .frame(get_window_frame(ctx, self.config.widget_opacity))
            .resizable(true)
            .min_width(200.0)
            .min_height(200.0)
            .show(ctx, |ui| {
                self.show_battle_metrics_widget(ui);
            });
    }

    pub fn show_enemy_stats_window(&mut self, ctx: &egui::Context) {
        egui::containers::Window::new(t!("Enemy Stats"))
            .id("enemy_stats_window".into())
            .frame(get_window_frame(ctx, self.config.widget_opacity))
            .resizable(true)
            .min_width(200.0)
            .min_height(150.0)
            .show(ctx, |ui| {
                self.show_enemy_stats_widget(ui);
            });
    }
}
