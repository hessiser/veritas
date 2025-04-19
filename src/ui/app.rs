use crate::ui::widgets;
use edio11::{input::InputResult, Overlay, WindowMessage, WindowProcessOptions};
use egui::FontFamily::Proportional;
use egui::Key;
use egui::Modifiers;
use egui::Stroke;
use egui::TextStyle::Body;
use egui::TextStyle::Button;
use egui::TextStyle::Heading;
use egui::TextStyle::Monospace;
use egui::TextStyle::Name;
use egui::TextStyle::Small;
use egui::{
    epaint::text::{FontInsert, InsertFontFamily},
    CentralPanel, Color32, Context, FontId, Frame, Slider, Window,
};
use windows::Win32::{
    Foundation::{LPARAM, WPARAM},
    UI::{Input::KeyboardAndMouse::VK_MENU, WindowsAndMessaging::WM_KEYDOWN},
};


#[derive(Default, PartialEq)]
pub enum Unit {
    #[default]
    Turn,
    ActionValue,
}

pub struct Keybind {
    pub key: egui::Key,
    pub modifiers: Option<egui::Modifiers>,
}

#[derive(Default)]
pub struct App {
    pub menu_keybind: Option<Keybind>,
    pub show_menu: bool,
    pub show_console: bool,

    show_windows: bool,
    show_damage_distribution: bool,
    show_damage_bars: bool,
    show_real_time_damage: bool,
    show_av_metrics: bool,
    widget_opacity: f32,
    pub graph_x_unit: Unit,
    pub text_scale: f32,
    pub should_hide: bool
}

impl Overlay for App {
    fn update(&mut self, ctx: &egui::Context) {
        if !self.should_hide {
            if self.show_menu {
                CentralPanel::default()
                    .frame(Frame {
                        fill: Color32::GRAY.gamma_multiply(0.25),
                        ..Default::default()
                    })
                    .show(ctx, |ui: &mut egui::Ui| {
                        Window::new("Overlay Menu")
                            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                            .resizable(false)
                            .show(ctx, |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.heading("Widget Controls");
    
                                    ui.checkbox(&mut self.show_console, "Show Logs");
                                    ui.checkbox(
                                        &mut self.show_damage_distribution,
                                        "Show Damage Distribution",
                                    );
                                    ui.checkbox(&mut self.show_damage_bars, "Show Damage Bars");
                                    ui.checkbox(
                                        &mut self.show_real_time_damage,
                                        "Show Real-Time Damage",
                                    );
                                    ui.checkbox(&mut self.show_av_metrics, "Show AV Metrics");
    
                                    ui.separator();
                                    ui.label("Window Opacity");
                                    ui.add(Slider::new(&mut self.widget_opacity, 0.0..=1.0).text(""));
    
                                    ui.separator();
                                    ui.label("Text Size");
                                    if ui
                                        .add(Slider::new(&mut self.text_scale, 0.5..=3.0).text(""))
                                        .changed()
                                    {
                                        ctx.style_mut(|style| {
                                            let factor = self.text_scale;
                                            style.text_styles = [
                                                (Heading, FontId::new(factor * 30.0, Proportional)),
                                                (
                                                    Name("Heading2".into()),
                                                    FontId::new(factor * 25.0, Proportional),
                                                ),
                                                (
                                                    Name("Context".into()),
                                                    FontId::new(factor * 23.0, Proportional),
                                                ),
                                                (Body, FontId::new(factor * 18.0, Proportional)),
                                                (Monospace, FontId::new(factor * 14.0, Proportional)),
                                                (Button, FontId::new(factor * 14.0, Proportional)),
                                                (Small, FontId::new(factor * 10.0, Proportional)),
                                            ]
                                            .into();
                                        });
                                    }
    
                                    ui.separator();
                                    if ui.button("Close Menu").clicked() {
                                        self.show_menu = false;
                                    }
                                });
                            });
                    });
            }
    
            if self.show_console {
                egui::Window::new("Log")
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
    
            let opacity = self.widget_opacity.clamp(0.0, 1.0);
            let window_frame = egui::Frame::new()
                .fill(Color32::from_black_alpha((255.0 * opacity) as u8))
                .stroke(Stroke::new(0.5, Color32::WHITE))
                .inner_margin(8.0)
                .corner_radius(10.0);
    
            let transparent_frame = egui::Frame::new().inner_margin(8.0);
    
            if self.show_damage_distribution {
                egui::containers::Window::new("Damage Distribution")
                    .frame(transparent_frame)
                    .resizable(true)
                    .min_width(200.0)
                    .min_height(200.0)
                    .show(ctx, |ui| {
                        widgets::show_damage_distribution_widget(self, ui);
                    });
            }
    
            if self.show_damage_bars {
                egui::containers::Window::new("Damage by Character")
                    .frame(window_frame)
                    .resizable(true)
                    .min_width(200.0)
                    .min_height(200.0)
                    .show(ctx, |ui| {
                        widgets::show_damage_bar_widget(self, ui);
                    });
            }
    
            if self.show_real_time_damage {
                egui::containers::Window::new("Real-Time Damage")
                    .frame(window_frame)
                    .resizable(true)
                    .min_width(200.0)
                    .min_height(200.0)
                    .show(ctx, |ui| {
                        widgets::show_real_time_damage_graph(self, ui);
                    });
            }
    
            if self.show_av_metrics {
                egui::containers::Window::new("Action Value Metrics")
                    .frame(window_frame)
                    .resizable(true)
                    .min_width(200.0)
                    .min_height(150.0)
                    .show(ctx, |ui| {
                        widgets::show_av_metrics(self, ui);
                    });
            }    
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
                            if let Some(menu_keybind) = &self.menu_keybind {
                                if *key == menu_keybind.key && *pressed {
                                    if let Some(keybind_modifiers) = menu_keybind.modifiers {
                                        if modifiers.matches_exact(keybind_modifiers) {
                                            self.show_menu = !self.show_menu;
    
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
                                }    
                            }

                            if *key == Key::H && *pressed {
                                if modifiers.matches_exact(Modifiers::CTRL) {
                                    self.should_hide = !self.should_hide;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        };

        return Some(WindowProcessOptions::default());
    }
}

impl App {
    pub fn new(ctx: Context) -> Self {
        let text_scale = 1.25;
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

        ctx.style_mut(|style| {
            style.visuals.widgets.noninteractive.fg_stroke.color = Color32::WHITE;
            style.text_styles = [
                (Heading, FontId::new(text_scale * 30.0, Proportional)),
                (
                    Name("Heading2".into()),
                    FontId::new(text_scale * 25.0, Proportional),
                ),
                (
                    Name("Context".into()),
                    FontId::new(text_scale * 23.0, Proportional),
                ),
                (Body, FontId::new(text_scale * 18.0, Proportional)),
                (Monospace, FontId::new(text_scale * 14.0, Proportional)),
                (Button, FontId::new(text_scale * 14.0, Proportional)),
                (Small, FontId::new(text_scale * 10.0, Proportional)),
            ]
            .into();
        });

        Self {
            widget_opacity: 0.15,
            text_scale,
            ..Default::default()
        }
    }

    pub fn set_menu_keybind(&mut self, key: egui::Key, modifiers: Option<egui::Modifiers>) {
        self.menu_keybind = Some(Keybind { key, modifiers });
    }
}