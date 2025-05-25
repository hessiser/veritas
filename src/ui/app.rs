use edio11::{Overlay, WindowMessage, WindowProcessOptions, input::InputResult};
use egui::Button;
use egui::CollapsingHeader;
use egui::Key;
use egui::KeyboardShortcut;
use egui::Label;
use egui::Modifiers;
use egui::RichText;
use egui::Stroke;
use egui::TextEdit;
use egui::Ui;
use egui::UiBuilder;
use egui::{
    CentralPanel, Color32, Context, Frame, Slider, Window,
    epaint::text::{FontInsert, InsertFontFamily},
};
use egui_colors::Colorix;
use windows::Win32::{
    Foundation::{LPARAM, WPARAM},
    UI::{Input::KeyboardAndMouse::VK_MENU, WindowsAndMessaging::WM_KEYDOWN},
};

use crate::LOCALES;

use super::config::Config;
use super::themes;

#[derive(Default, PartialEq)]
pub enum GraphUnit {
    #[default]
    Turn,
    ActionValue,
}

#[derive(Default)]
pub struct AppState {
    pub show_menu: bool,
    pub show_settings: bool,
    pub show_console: bool,
    pub show_damage_distribution: bool,
    pub show_damage_bars: bool,
    pub show_real_time_damage: bool,
    // show_enemy_stats: bool,
    pub show_av_metrics: bool,
    pub should_hide: bool,
    pub graph_x_unit: GraphUnit,
    pub use_custom_color: bool,
}

#[derive(Default)]
pub struct Settings {
    pub widget_opacity: f32,
    pub streamer_mode: bool,
    pub streamer_msg_size_pt: f32,
    pub streamer_msg: String,
    // pub fps: i32,
    pub colorix: Colorix,
}

pub struct App {
    pub state: AppState,
    settings: Settings,
    config: Config,
}

pub const HIDE_UI: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::H);
pub const SHOW_MENU_SHORTCUT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::M);

impl Overlay for App {
    fn update(&mut self, ctx: &egui::Context) {
        if ctx.input_mut(|i| i.consume_shortcut(&HIDE_UI)) {
            self.state.should_hide = !self.state.should_hide;
        }

        if self.settings.streamer_mode {
            egui::TopBottomPanel::bottom("statusbar")
                .resizable(true)
                .show(ctx, |ui| {
                    for (_text_style, font_id) in ui.style_mut().text_styles.iter_mut() {
                        font_id.size *= self.settings.streamer_msg_size_pt;
                    }
                    let label = Label::new(RichText::new(&self.settings.streamer_msg).strong())
                        .selectable(false);
                    ui.add(label);
                    ui.allocate_space(ui.available_size())
                });
        }

        if !self.state.should_hide {
            if self.state.show_menu {
                CentralPanel::default()
                    .frame(Frame {
                        fill: Color32::GRAY.gamma_multiply(0.25),
                        ..Default::default()
                    })
                    .show(ctx, |_ui: &mut egui::Ui| {
                        Window::new(t!("Menu"))
                            .id("menu_window".into())
                            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                            .collapsible(false)
                            .resizable(false)
                            .show(ctx, |ui| {
                                // Settings
                                egui::Frame::default().inner_margin(5.0).show(ui, |ui| {
                                    egui::menu::bar(ui, |ui| {
                                        ui.toggle_value(
                                            &mut self.state.show_settings,
                                            RichText::new(format!(
                                                "{} {}",
                                                egui_phosphor::bold::GEAR,
                                                t!("Settings")
                                            )));

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

                                egui::Window::new(t!("Settings"))
                                    .id("settings_window".into())
                                    .open(&mut self.state.show_settings)
                                    .show(ctx, |ui| {
                                        egui::menu::bar(ui, |ui| {
                                            let style = ctx.style(); // `ctx` is of type `&egui::Context`
                                            let font_id = &style.text_styles[&egui::TextStyle::Button];
                                            let font_size = font_id.size;
                                            self.settings
                                                .colorix
                                                .light_dark_toggle_button(ui, font_size);

                                            ui.separator();

                                            ui.menu_button(
                                                RichText::new(format!(
                                                    "{} {}",
                                                    egui_phosphor::bold::FILE,
                                                    t!("File")
                                                )),
                                                |ui| {
                                                    if ui.button(t!("Save theme")).clicked() {
                                                        self.config.set_theme(*self.settings.colorix.theme());
                                                        if self.settings.colorix.dark_mode() {
                                                            self.config.set_theme_mode(egui::Theme::Dark);
                                                        }
                                                        else {
                                                            self.config.set_theme_mode(egui::Theme::Light);
                                                        }
                                                        ui.close_menu();
                                                    }

                                                    if ui.button(t!("Revert theme")).clicked() {
                                                        match self.config.get_theme_mode() {
                                                            egui::Theme::Dark => self.settings.colorix.set_dark(ui),
                                                            egui::Theme::Light => self.settings.colorix.set_light(ui),
                                                        }
                                                        self.settings.colorix.update_theme(ctx, *self.config.get_theme());
                                                        ui.close_menu();
                                                    }
                                                },
                                            );

                                            ui.menu_button(
                                                RichText::new(format!(
                                                    "{} {}",
                                                    egui_phosphor::bold::GLOBE,
                                                    t!("Language")
                                                )),
                                                |ui| {
                                                    for locale_code in
                                                        rust_i18n::available_locales!()
                                                    {
                                                        if let Some(locale) =
                                                            LOCALES.get(locale_code)
                                                        {
                                                            if ui.button(*locale).clicked() {
                                                                self.config.set_locale(
                                                                    locale_code.to_string(),
                                                                );
                                                                rust_i18n::set_locale(locale_code);
                                                                ui.close_menu();
                                                            }
                                                        }
                                                    }
                                                },
                                            );

                                            if ui
                                                .toggle_value(
                                                    &mut self.settings.streamer_mode,
                                                    RichText::new(format!(
                                                        "{} {}",
                                                        egui_phosphor::bold::VIDEO_CAMERA,
                                                        t!("Streamer Mode")
                                                    )),
                                                )
                                                .changed()
                                            {
                                                self.config
                                                    .set_streamer_mode(self.settings.streamer_mode);
                                            }

                                        });

                                        ui.separator();

                                        ui.with_layout(
                                            egui::Layout::top_down_justified(egui::Align::LEFT),
                                            |ui| {
                                                ui.add_space(5.);

                                                CollapsingHeader::new(t!("Theme"))
                                                    .id_salt("theme_header")
                                                    .show(ui, |ui| {
                                                        if self.state.use_custom_color {
                                                            self.settings.colorix.twelve_from_custom(ui);
                                                        };

                                                        ui.horizontal(|ui| {
                                                            self.settings.colorix.custom_picker(ui);
                                                            ui.toggle_value(&mut self.state.use_custom_color, t!("Custom color"));
                                                        });
                                                        self.settings
                                                            .colorix
                                                            .themes_dropdown(ui, Some((themes::THEME_NAMES.to_vec(), themes::THEMES.to_vec())), true);

                                                        self.settings.colorix.ui_combo_12(ui, false);
                                                    });

                                                if ui
                                                    .add(
                                                        Slider::new(
                                                            &mut self.settings.widget_opacity,
                                                            0.0..=1.0,
                                                        )
                                                        .text(t!("Window Opacity")),
                                                    )
                                                    .changed()
                                                {
                                                    self.config.set_widget_opacity(
                                                        self.settings.widget_opacity,
                                                    );
                                                };

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

                                                if ui
                                                    .add(
                                                        Slider::new(
                                                            &mut self.settings.streamer_msg_size_pt,
                                                            0.5..=2.0,
                                                        )
                                                        .text(t!("Streamer Message Font Size%")),
                                                    )
                                                    .changed()
                                                {
                                                    self.config.set_streamer_msg_size_pt(self.settings.streamer_msg_size_pt);
                                                }



                                                if ui.add(
                                                    TextEdit::singleline(
                                                        &mut self.settings.streamer_msg,
                                                    )
                                                    .hint_text(RichText::new(format!("{} {}", t!("Streamer Message. Can also use Phosphor Icons!"), egui_phosphor::bold::RAINBOW))),
                                                ).changed() {
                                                    self.config.set_streamer_msg(self.settings.streamer_msg.clone());
                                                };
                                            },
                                        );
                                    });

                                ui.vertical_centered(|ui| {
                                    ui.add_space(5.);
                                    ui.checkbox(&mut self.state.show_console, t!("Show Logs"));
                                    ui.checkbox(
                                        &mut self.state.show_damage_distribution,
                                        t!("Show Damage Distribution"),
                                    );
                                    ui.checkbox(
                                        &mut self.state.show_damage_bars,
                                        t!("Show Damage Bars"),
                                    );
                                    ui.checkbox(
                                        &mut self.state.show_real_time_damage,
                                        t!("Show Real-Time Damage"),
                                    );
                                    // ui.checkbox(
                                    //     &mut self.show_enemy_stats,
                                    //     t!("Show Enemy Stats"),
                                    // );

                                    ui.checkbox(
                                        &mut self.state.show_av_metrics,
                                        t!("Show AV Metrics"),
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

            if self.state.show_console {
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

            let opacity = self.settings.widget_opacity.clamp(0.0, 1.0);
            let color = ctx.style().visuals.extreme_bg_color.gamma_multiply(opacity);
            let window_frame = egui::Frame::new()
                .fill(color)
                .stroke(Stroke::new(0.5, Color32::WHITE))
                .inner_margin(8.0)
                .corner_radius(10.0);

            let transparent_frame = egui::Frame::new().inner_margin(8.0).corner_radius(10.0);

            let damage_distribution_window_title = if self.state.show_menu {
                t!("Damage Distribution").into_owned()
            } else {
                String::new()
            };
            if self.state.show_damage_distribution {
                egui::containers::Window::new(damage_distribution_window_title)
                    .id("damage_distribution_window".into())
                    .frame(if self.state.show_menu {
                        window_frame
                    } else {
                        transparent_frame
                    })
                    .collapsible(false)
                    .resizable(true)
                    .min_width(200.0)
                    .min_height(200.0)
                    .show(ctx, |ui| {
                        self.show_damage_distribution_widget(ui);
                    });
            }

            if self.state.show_damage_bars {
                egui::containers::Window::new(t!("Damage by Character"))
                    .id("damage_by_character_window".into())
                    .frame(window_frame)
                    .resizable(true)
                    .min_width(200.0)
                    .min_height(200.0)
                    .show(ctx, |ui| {
                        self.show_damage_bar_widget(ui);
                    });
            }

            if self.state.show_real_time_damage {
                egui::containers::Window::new(t!("Real-Time Damage"))
                    .id("realt_time_damage_window".into())
                    .frame(window_frame)
                    .resizable(true)
                    .min_width(200.0)
                    .min_height(200.0)
                    .show(ctx, |ui| {
                        self.show_real_time_damage_graph_widget(ui);
                    });
            }

            if self.state.show_av_metrics {
                egui::containers::Window::new(t!("Action Value Metrics"))
                    .id("action_value_metrics_window".into())
                    .frame(window_frame)
                    .resizable(true)
                    .min_width(200.0)
                    .min_height(150.0)
                    .show(ctx, |ui| {
                        self.show_av_metrics_widget(ui);
                    });
            }

            // if self.show_enemy_stats {
            //     egui::containers::Window::new(t!("Enemy Stats"))
            //         .id("enemy_stats_window")
            //         .frame(window_frame)
            //         .resizable(true)
            //         .min_width(200.0)
            //         .min_height(150.0)
            //         .show(ctx, |ui| {
            //             self.show_enemy_stats(ui);
            //         });
            // }
        }
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

        if self.state.show_menu {
            Some(WindowProcessOptions {
                should_capture_all_input: true,
                ..Default::default()
            })
        } else {
            Some(WindowProcessOptions::default())
        }
    }
}

impl App {
    pub fn new(ctx: Context) -> Self {
        let path = r"StarRail_Data\StreamingAssets\MiHoYoSDKRes\HttpServerResources\font\zh-cn.ttf";
        match std::fs::read(path) {
            Ok(font) => {
                // Start with the default fonts (we will be adding to them rather than replacing them).
                ctx.add_font(FontInsert::new(
                    "game_font",
                    egui::FontData::from_owned(font),
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

        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Bold);

        ctx.set_fonts(fonts);

        let config = Config::new(&ctx).unwrap();

        let mut app = Self {
            settings: Settings {
                widget_opacity: *config.get_widget_opacity(),
                streamer_mode: *config.get_streamer_mode(),
                streamer_msg_size_pt: *config.get_streamer_msg_size_pt(),
                streamer_msg: config.get_streamer_msg().to_owned(),
                // fps: *config.get_fps(),
                colorix: Colorix::global(&ctx, *config.get_theme()),
            },
            config,
            state: AppState::default(),
        };

        app.initialize_settings(&ctx);
        app
    }

    fn initialize_settings(&mut self, ctx: &Context) {
        rust_i18n::set_locale(&self.config.get_locale());
        match self.config.get_theme_mode() {
            egui::Theme::Dark => self.settings.colorix.set_dark(&mut Ui::new(
                ctx.clone(),
                "".into(),
                UiBuilder::new(),
            )),
            egui::Theme::Light => self.settings.colorix.set_light(&mut Ui::new(
                ctx.clone(),
                "".into(),
                UiBuilder::new(),
            )),
        }
    }
}
