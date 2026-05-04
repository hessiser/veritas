use crate::{kreide::types::RPG_GameCore_AvatarPropertyType, ui::app::{DamageBarValue, DamageBreakdownChart, DamageBreakdownScope, GraphUnit}};
use egui::{Align, Align2, Color32, FontId, Frame, Layout, RichText, ScrollArea, Sense, Stroke, StrokeKind, TextStyle, Ui, Vec2};
use egui_extras::Column;
use egui_plot::{Bar, BarChart, Line, Plot, PlotPoints, Polygon};

use crate::{
    battle::{display_damage_type, BattleContext, DamageTypeBreakdown},
    kreide::types::RPG_GameCore_AttackType,
    models::types::Avatar,
};

use super::{app::App, helpers};

pub struct PieSegment {
    pub points: Vec<[f64; 2]>,
    pub value: f64,
}

struct DamagePortraitRow {
    avatar: Avatar,
    damage: f64,
    dpav: f64,
    effective_damage: f64,
    overkill_damage: f64,
    percentage: f64,
}

impl App {
    pub fn show_damage_distribution_widget(&mut self, ui: &mut Ui) {
        let available = ui.available_size();

        Plot::new("damage_pie")
            // .legend(
            //     Legend::default()
            //         .position(self.config.legend_position)
            //         .text_style(self.config.legend_text_style.clone()),
            // )
            .height(available.y)
            .width(available.x)
            .data_aspect(1.0)
            .clamp_grid(true)
            .show_grid(false)
            .show_background(false)
            .show_axes([false; 2])
            // .allow_drag(false)
            // .allow_zoom(false)
            .allow_scroll(false)
            .show(ui, |plot_ui: &mut egui_plot::PlotUi<'_>| {
                let battle_context = BattleContext::get_instance();

                let total_damage = battle_context.total_damage as f64;
                if total_damage > 0.0 {
                    let segments = create_pie_segments(
                        &battle_context.real_time_damages,
                        &battle_context.avatar_lineup,
                    );
                    for (avatar, segment, i) in segments {
                        let color = helpers::get_character_color(i);
                        // let percentage = segment.value / total_damage * 100.0;

                        let plot_points = PlotPoints::new(segment.points);
                        let polygon = Polygon::new("Damage Pie", plot_points)
                            .stroke(Stroke::new(1.5, color))
                            .fill_color(color.linear_multiply(self.config.pie_chart_opacity))
                            .id(avatar.name.clone());
                        // .name(format!(
                        //     "{}: {:.0}% | {} DMG | {:.0} DPAV",
                        //     avatar.name,
                        //     percentage,
                        //     helpers::format_damage(segment.value),
                        //     segment.value / battle_context.action_value
                        // ));

                        plot_ui.polygon(polygon);
                    }
                }
            });
    }

    pub fn show_character_legend(&mut self, ui: &mut Ui) {
        let battle_context = &BattleContext::get_instance();

        // // I need to make separate DPAV calcs in the battle context
        // for (i, avatar) in battle_context.avatar_lineup.iter().enumerate() {
        //     ui.horizontal(|ui| {
        //         let style = ui.style_mut();
        //         style.override_text_style = Some(self.config.legend_text_style.clone());
        //         let (res, painter) = ui.allocate_painter(Vec2::splat(12.), Sense::empty());
        //         let rect = res.rect;
        //         let radius = rect.width() / 2.0 - 1.0;
        //         painter.circle_filled(rect.center(), radius, helpers::get_character_color(i));

        //         let dmg = battle_context.real_time_damages[i];

        //         let percentage = dmg / battle_context.total_damage * 100.0;

        //         let dpav = if battle_context.action_value > 0.0 {
        //             dmg / battle_context.action_value
        //         } else {
        //             dmg
        //         };

        //         ui.label(format!(
        //             "{}: {:.1}% | {} DMG | {} DPAV",
        //             avatar.name,
        //             percentage,
        //             helpers::format_damage(dmg),
        //             helpers::format_damage(dpav)
        //         ));
        //     });
        // }

        let mut table_builder = egui_extras::TableBuilder::new(ui)
            .cell_layout(Layout::centered_and_justified(egui::Direction::LeftToRight));

        let headers = [t!("Avatar"), t!("DMG"), t!("DPAV")];

        for _ in &headers {
            table_builder = table_builder.column(Column::auto_with_initial_suggestion(20.));
        }

        table_builder
            .header(20.0, |mut header| {
                for label in &headers {
                    header.col(|ui| {
                        ui.heading(label.as_ref());
                    });
                }
            })
            .body(|body| {
                body.rows(52., battle_context.avatar_lineup.len() + 1, |mut row| {
                    if battle_context.avatar_lineup.len() == 0 {
                        return;
                    }
                    
                    let i = row.index();
                    let dmg = if i >= battle_context.avatar_lineup.len() {
                        battle_context.total_damage
                    } else {
                        battle_context.real_time_damages[i]
                    };

                    row.col(|ui| {
                        ui.with_layout(
                            Layout::centered_and_justified(egui::Direction::LeftToRight),
                            |ui| {
                                if i == battle_context.avatar_lineup.len() {
                                    ui.label(t!("Total"));
                                } else {
                                    // Load avatar image with caching, display name if loading fails
                                    if let Some(handle) = helpers::load_avatar_image(
                                        ui.ctx(),
                                        battle_context.avatar_lineup[i].id,
                                        egui::TextureOptions::default(),
                                    ) {
                                        let percentage = if battle_context.total_damage > 0.0 {
                                            dmg / battle_context.total_damage * 100.0
                                        } else {
                                            0.0
                                        };

                                        let dim = 48.0;
                                        let sized_image = egui::load::SizedTexture::new(
                                            handle.id(),
                                            egui::vec2(dim, dim),
                                        );
                                        let image_response =
                                            ui.add(egui::Image::from_texture(sized_image));

                                        let text_pos = image_response.rect.right_bottom()
                                            - egui::vec2(0.0, 0.0);
                                        let percentage_text = format!("{percentage:.0}%");

                                        // Text Shadow
                                        ui.painter().text(
                                            text_pos + egui::vec2(-1., 1.),
                                            Align2::RIGHT_BOTTOM,
                                            &percentage_text,
                                            FontId::proportional(dim / 4.0),
                                            Color32::BLACK,
                                        );

                                        ui.painter().text(
                                            text_pos,
                                            Align2::RIGHT_BOTTOM,
                                            &percentage_text,
                                            FontId::proportional(dim / 4.0),
                                            Color32::WHITE,
                                        );
                                    } else {
                                        ui.label(format!("{}", battle_context.avatar_lineup[i].name));
                                    }
                                }
                            },
                        );
                    });

                    row.col(|ui: &mut Ui| {
                        ui.with_layout(
                            Layout::centered_and_justified(egui::Direction::LeftToRight),
                            |ui| {
                                ui.label(format! {"{}", helpers::format_damage(dmg)});
                            },
                        );
                    });

                    row.col(|ui: &mut Ui| {
                        ui.with_layout(
                            Layout::centered_and_justified(egui::Direction::LeftToRight),
                            |ui| {
                                let dpav = if battle_context.action_value > 0.0 {
                                    dmg / battle_context.action_value
                                } else {
                                    dmg
                                };

                                ui.label(format! {"{}", helpers::format_damage(dpav)});
                            },
                        );
                    });
                });
            });

        // .body(|mut body| {
        //     body.row(30.0, |mut row| {
        //         row.col(|ui| {
        //             let avatars = vec!["1218.png", "1304.png", "1308.png", "1406.png"];
        //                 for avatar in avatars {
        //                     let handle = helpers::load_image(
        //                         ui.ctx(),
        //                         avatar,
        //                         egui::TextureOptions::default(),
        //                     );
        //                     let sized_image = egui::load::SizedTexture::new(
        //                         handle.id(),
        //                         // egui::vec2(handle.size()[0] as f32, handle.size()[1] as f32),
        //                         egui::vec2(64.0, 64.0),
        //                     );
        //                     ui.add(egui::Image::from_texture(sized_image));
        //                 }
        //         });

        //         row.col(|ui: &mut Ui| {
        //             let avatars = vec!["1218.png", "1304.png", "1308.png", "1406.png"];
        //                 for avatar in avatars {
        //                     ui.label("700");
        //                 }
        //         });
        //     });
        // });
    }

    pub fn show_damage_bar_widget(&mut self, ui: &mut Ui) {
        let available = ui.available_size();

        let (num_characters, avatar_lineup, real_time_damages, real_time_overkill_damages) = {
            let battle_context = BattleContext::get_instance();
            (
                battle_context.avatar_lineup.len().max(1) as f32,
                battle_context.avatar_lineup.clone(),
                battle_context.real_time_damages.clone(),
                battle_context.real_time_overkill_damages.clone(),
            )
        };

        let char_width_per_bar = available.x / num_characters;
        let max_chars_per_line = ((char_width_per_bar / 8.0).max(8.0).min(15.0)) as usize;

        let avatar_lineup_for_formatter = avatar_lineup.clone();

        Plot::new("damage_bars")
            .height(available.y)
            .width(available.x)
            .allow_drag(false)
            .allow_zoom(false)
            .allow_scroll(false)
            .show_background(false)
            .y_axis_formatter(|y, _| helpers::format_damage(y.value))
            .x_axis_formatter(move |x, _| {
                let index = x.value.floor() as usize;
                avatar_lineup_for_formatter
                    .get(index)
                    .map(|avatar| helpers::wrap_character_name(&avatar.name, max_chars_per_line))
                    .unwrap_or_default()
            })
            .show(ui, |plot_ui| {
                let adjusted_damages: Vec<f64> = real_time_damages
                    .iter()
                    .zip(real_time_overkill_damages.iter())
                    .map(|(damage, overkill)| (damage - overkill).max(0.0))
                    .collect();
                let bars_data = create_bar_data(&adjusted_damages, &avatar_lineup);
                let bars: Vec<Bar> = bars_data
                    .iter()
                    .enumerate()
                    .map(|(pos, (avatar, value, color_idx))| {
                        Bar::new(pos as f64, *value)
                            .name(&avatar.name)
                            .fill(helpers::get_character_color(*color_idx))
                            .width(0.7)
                    })
                    .collect();

                let overkill_bars_data =
                    create_bar_data(&real_time_overkill_damages, &avatar_lineup);
                let overkill_color = Color32::from_gray(140);
                let overkill_bars: Vec<Bar> = overkill_bars_data
                    .iter()
                    .enumerate()
                    .map(|(pos, (avatar, value, _color_idx))| {
                        Bar::new(pos as f64, *value)
                            .name(&avatar.name)
                            .fill(overkill_color)
                            .width(0.7)
                    })
                    .collect();
                let dmg_bar_chart = BarChart::new("", bars).id("dmg_bar_chart");
                let overkill_bar_chart = BarChart::new("Overkill", overkill_bars)
                    .color(overkill_color)
                    .id("overkill_dmg_bar_chart")
                    .stack_on(&[&dmg_bar_chart]);
                plot_ui.bar_chart(dmg_bar_chart);
                plot_ui.bar_chart(overkill_bar_chart);
            });
    }

    pub fn show_character_damage_widget(&mut self, ui: &mut Ui) {
        ui.spacing_mut().item_spacing = Vec2::new(8.0, 6.0);

        let mut rows = {
            let battle_context = BattleContext::get_instance();
            create_damage_portrait_rows(
                &battle_context.avatar_lineup,
                &battle_context.real_time_damages,
                &battle_context.real_time_overkill_damages,
                battle_context.action_value,
            )
        };

        rows.sort_by(|left, right| {
            let left_value = match self.state.damage_bar_value {
                DamageBarValue::Damage => left.damage,
                DamageBarValue::Dpav => left.dpav,
            };
            let right_value = match self.state.damage_bar_value {
                DamageBarValue::Damage => right.damage,
                DamageBarValue::Dpav => right.dpav,
            };
            right_value
                .partial_cmp(&left_value)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let visuals = ui.visuals().clone();
        let dark_mode = visuals.dark_mode;
        let max_value = rows
            .iter()
            .map(|row| match self.state.damage_bar_value {
                DamageBarValue::Damage => row.damage,
                DamageBarValue::Dpav => row.dpav,
            })
            .fold(0.0, f64::max);

        let primary_text = if dark_mode {
            Color32::from_gray(235)
        } else {
            visuals.widgets.noninteractive.fg_stroke.color
        };
        let secondary_text = if dark_mode {
            Color32::from_gray(138)
        } else {
            visuals.widgets.inactive.fg_stroke.color.gamma_multiply(0.9)
        };
        let accent = visuals.selection.bg_fill;

        let tab_height = 24.0;
        let tab_width = (ui.available_width() / 2.0).max(80.0);
        let tabs = [
            (DamageBarValue::Damage, t!("DMG")),
            (DamageBarValue::Dpav, t!("DPAV")),
        ];

        let (tabs_rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), tab_height), Sense::hover());
        let tab_painter = ui.painter_at(tabs_rect);

        ui.allocate_ui_at_rect(tabs_rect, |ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;

            for (value, label) in tabs {
                let (rect, response) = ui.allocate_exact_size(Vec2::new(tab_width, tab_height), Sense::click());
                let selected = self.state.damage_bar_value == value;

                if response.clicked() {
                    self.state.damage_bar_value = value;
                }

                tab_painter.text(
                    rect.center_top() + egui::vec2(0.0, 2.0),
                    Align2::CENTER_TOP,
                    label.as_ref(),
                    FontId::proportional(16.0),
                    if selected { primary_text } else { secondary_text },
                );

            }
        });
        });

        let separator_color = if dark_mode {
            Color32::from_rgba_premultiplied(255, 255, 255, 20)
        } else {
            visuals.widgets.noninteractive.bg_stroke.color.gamma_multiply(0.6)
        };
        ui.painter().line_segment(
            [
                egui::pos2(tabs_rect.left(), tabs_rect.bottom()),
                egui::pos2(tabs_rect.right(), tabs_rect.bottom()),
            ],
            Stroke::new(1.0, separator_color),
        );

        let selected_index = if self.state.damage_bar_value == DamageBarValue::Damage {
            0.0
        } else {
            1.0
        };
        let underline_left = tabs_rect.left() + selected_index * tab_width + 10.0;
        let underline_right = tabs_rect.left() + (selected_index + 1.0) * tab_width - 10.0;
        ui.painter().line_segment(
            [
                egui::pos2(underline_left, tabs_rect.bottom() - 1.0),
                egui::pos2(underline_right, tabs_rect.bottom() - 1.0),
            ],
            Stroke::new(2.0, accent),
        );

        ui.add_space(8.0);

        if rows.is_empty() {
            return;
        }

        ScrollArea::vertical()
            .id_salt("character_damage_rows_v3")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for row in rows {
                    let display_value = match self.state.damage_bar_value {
                        DamageBarValue::Damage => row.damage,
                        DamageBarValue::Dpav => row.dpav,
                    };
                    let response = ui
                        .horizontal(|ui| {
                            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                                self.show_avatar_portrait(ui, &row.avatar);
                                ui.add_space(12.0);
                                ui.vertical(|ui| {
                                    ui.set_min_width(185.0);
                                    ui.label(
                                        RichText::new(helpers::format_damage(display_value))
                                            .size(if dark_mode { 20.0 } else { 18.0 })
                                            .strong()
                                            .color(primary_text),
                                    );
                                    ui.add_space(2.0);
                                    draw_damage_meter(
                                        ui,
                                        if max_value > 0.0 {
                                            display_value / max_value
                                        } else {
                                            0.0
                                        },
                                        &visuals,
                                    );
                                });
                            });
                        })
                        .response;

                    response.on_hover_text(format!(
                        "{}\n{} dealt\n{} DPAV\n{} effective\n{} overkill\n{:.1}% of team damage",
                        row.avatar.name,
                        helpers::format_damage(row.damage),
                        helpers::format_damage(row.dpav),
                        helpers::format_damage(row.effective_damage),
                        helpers::format_damage(row.overkill_damage),
                        row.percentage,
                    ));

                    ui.add_space(2.0);
                }
            });
    }

    pub fn show_damage_type_breakdown_widget(&mut self, ui: &mut Ui) {
        let battle_context = BattleContext::get_instance();
        let total_damage = battle_context.total_damage;

        if battle_context.avatar_lineup.is_empty() || total_damage <= 0.0 {
            return;
        }

        let overall_breakdown = aggregate_damage_by_category(&battle_context.damage_by_category);
        let max_index = battle_context.avatar_lineup.len().saturating_sub(1);
        self.state.damage_breakdown_character_index = self
            .state
            .damage_breakdown_character_index
            .min(max_index);

        ui.horizontal(|ui| {
            ui.radio_value(
                &mut self.state.damage_breakdown_scope,
                DamageBreakdownScope::Team,
                t!("Teamwide"),
            );
            ui.radio_value(
                &mut self.state.damage_breakdown_scope,
                DamageBreakdownScope::Character,
                t!("Character"),
            );
        });

        ui.horizontal(|ui| {
            ui.radio_value(
                &mut self.state.damage_breakdown_chart,
                DamageBreakdownChart::Pie,
                t!("Pie Chart"),
            );
            ui.radio_value(
                &mut self.state.damage_breakdown_chart,
                DamageBreakdownChart::Bar,
                t!("Bar Chart"),
            );
        });

        if self.state.damage_breakdown_scope == DamageBreakdownScope::Character {
            egui::ComboBox::new("damage_breakdown_character_picker", "")
                .selected_text(
                    battle_context.avatar_lineup[self.state.damage_breakdown_character_index]
                        .name
                        .clone(),
                )
                .show_ui(ui, |ui| {
                    for (i, avatar) in battle_context.avatar_lineup.iter().enumerate() {
                        ui.selectable_value(
                            &mut self.state.damage_breakdown_character_index,
                            i,
                            &avatar.name,
                        );
                    }
                });
        }

        ui.add_space(8.0);

        let (title, breakdown, selected_damage) = match self.state.damage_breakdown_scope {
            DamageBreakdownScope::Team => (
                t!("Teamwide").into_owned(),
                overall_breakdown.clone(),
                total_damage,
            ),
            DamageBreakdownScope::Character => {
                let index = self.state.damage_breakdown_character_index;
                (
                    battle_context.avatar_lineup[index].name.clone(),
                    battle_context.damage_by_category[index].clone(),
                    battle_context.real_time_damages[index],
                )
            }
        };

        ui.label(
            egui::RichText::new(format!(
                "{} | {} | {:.1}%",
                title,
                helpers::format_damage(selected_damage),
                if total_damage > 0.0 {
                    (selected_damage / total_damage) * 100.0
                } else {
                    0.0
                }
            ))
            .strong(),
        );

        match self.state.damage_breakdown_chart {
            DamageBreakdownChart::Pie => {
                show_damage_category_pie_chart(ui, &breakdown);
            }
            DamageBreakdownChart::Bar => {
                show_damage_category_bar_chart(ui, &breakdown);
            }
        }

        ui.add_space(12.0);
        egui::CollapsingHeader::new(t!("Legend"))
            .id_salt("damage_type_breakdown_legend_header")
            .default_open(false)
            .show(ui, |ui| {
                draw_damage_category_legend(ui, &breakdown);
            });

        egui::CollapsingHeader::new(t!("Detailed Stats"))
            .id_salt("damage_type_breakdown_details_header")
            .default_open(false)
            .show(ui, |ui| {
                draw_damage_category_grid(
                    ui,
                    "damage_type_breakdown_selected_grid",
                    &breakdown,
                    selected_damage,
                    total_damage,
                );

                if self.state.damage_breakdown_scope == DamageBreakdownScope::Team {
                    ui.add_space(12.0);
                    ui.label(egui::RichText::new(t!("Character Summary")).strong());
                    egui::Grid::new("damage_type_character_summary_grid")
                        .striped(true)
                        .num_columns(3)
                        .show(ui, |ui| {
                            ui.strong(t!("Character"));
                            ui.strong(t!("Damage"));
                            ui.strong(t!("Party Share"));
                            ui.end_row();

                            for (i, avatar) in battle_context.avatar_lineup.iter().enumerate() {
                                let character_damage = battle_context.real_time_damages[i];
                                let percentage = (character_damage / total_damage) * 100.0;

                                ui.label(&avatar.name);
                                ui.label(helpers::format_damage(character_damage));
                                ui.label(format!("{percentage:.1}%"));
                                ui.end_row();
                            }
                        });
                }
            });
    }

    pub fn show_turn_damage_plot(&mut self, ui: &mut Ui) {
        let battle_context = BattleContext::get_instance();
        let available = ui.available_size();
        Plot::new("damage_plot")
            // .legend(
            //     Legend::default()
            //         .position(self.config.legend_position)
            //         .text_style(self.config.legend_text_style.clone()),
            // )
            .height(available.y)
            .width(available.x)
            .include_y(0.0)
            .x_axis_label(t!("Turn"))
            .y_axis_label(t!("Damage"))
            .y_axis_formatter(|y, _| helpers::format_damage(y.value))
            .show(ui, |plot_ui| {
                for (i, avatar) in battle_context.avatar_lineup.iter().enumerate() {
                    let color = helpers::get_character_color(i);
                    let points = battle_context
                        .turn_history
                        .iter()
                        .enumerate()
                        .map(|(turn_idx, turn)| {
                            [turn_idx as f64 + 1.0, turn.avatars_turn_damage[i]]
                        })
                        .collect::<Vec<[f64; 2]>>();

                    if !points.is_empty() {
                        plot_ui.line(
                            Line::new(&avatar.name, PlotPoints::from(points))
                                .color(color)
                                .width(2.0),
                        );
                    }
                }
            });
    }

    pub fn show_av_damage_plot(&mut self, ui: &mut Ui) {
        let battle_context = BattleContext::get_instance();
        let available = ui.available_size();
        Plot::new("damage_plot")
            // .legend(
            //     Legend::default()
            //         .position(self.config.legend_position)
            //         .text_style(self.config.legend_text_style.clone()),
            // )
            .height(available.y)
            .width(available.x)
            .include_y(0.0)
            .x_axis_label(t!("Action Value"))
            .y_axis_label(t!("Damage"))
            .y_axis_formatter(|y, _| helpers::format_damage(y.value))
            .show(ui, |plot_ui| {
                for (i, avatar) in battle_context.avatar_lineup.iter().enumerate() {
                    let color = helpers::get_character_color(i);
                    let points = battle_context
                        .av_history
                        .iter()
                        .map(|turn| [turn.action_value, turn.avatars_turn_damage[i]])
                        .collect::<Vec<[f64; 2]>>();

                    if !points.is_empty() {
                        plot_ui.line(
                            Line::new(&avatar.name, PlotPoints::from(points))
                                .color(color)
                                .width(2.0),
                        );
                    }
                }
            });
    }

    pub fn show_real_time_damage_graph_widget(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.state.graph_x_unit, GraphUnit::Turn, t!("Turn"));
                ui.radio_value(
                    &mut self.state.graph_x_unit,
                    GraphUnit::ActionValue,
                    t!("Action Value"),
                );
            });
            ui.add_space(8.0);

            match self.state.graph_x_unit {
                GraphUnit::Turn => self.show_turn_damage_plot(ui),
                GraphUnit::ActionValue => self.show_av_damage_plot(ui),
            }
        });
    }

    pub fn show_battle_metrics_widget(&mut self, ui: &mut Ui) {
        let battle_context = BattleContext::get_instance();

        egui::CollapsingHeader::new(format!(
            "{}: {:.2}",
            t!("Total Damage"),
            battle_context.total_damage
        ))
        .id_salt("total_damage_header")
        .show(ui, |ui| {
            ui.vertical(|ui| {
                for (i, avatar) in battle_context.avatar_lineup.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(format!("{}", avatar.name));

                        ui.label(format!("{:.2}", battle_context.real_time_damages[i],));
                    });
                }
            });
        });

        let current_action_value =
            battle_context.action_value - battle_context.last_wave_action_value;

        egui::CollapsingHeader::new(format!("{}: {:.2}", t!("AV"), current_action_value))
            .id_salt("action_value_header")
            .show(ui, |ui| {
                ui.label(format!(
                    "{}: {:.2}",
                    t!("Total Elapsed AV"),
                    battle_context.action_value
                ));
                ui.vertical(|ui| {
                    for (i, avatar) in battle_context.avatar_lineup.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(format!("{}", avatar.name,));

                            ui.label(format!(
                                "{:.2}",
                                battle_context.battle_avatars[i].properties.av()
                                    + current_action_value,
                            ));
                        });
                    }
                });
            });

        let dpav = if battle_context.action_value > 0.0 {
            battle_context.total_damage / battle_context.action_value
        } else {
            battle_context.total_damage
        };
        egui::CollapsingHeader::new(format!("{}: {:.2}", t!("DPAV"), dpav))
            .id_salt("dpav_header")
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    for (i, avatar) in battle_context.avatar_lineup.iter().enumerate() {
                        let dpav = if battle_context.action_value > 0.0 {
                            battle_context.real_time_damages[i] / battle_context.action_value
                        } else {
                            battle_context.real_time_damages[i]
                        };
                        ui.horizontal(|ui| {
                            ui.label(format!("{}", avatar.name));
                            ui.label(format!("{:.2}", dpav));
                        });
                    }
                });
            });
    }

    pub fn show_enemy_stats_widget(&mut self, ui: &mut Ui) {
        let battle_context = BattleContext::get_instance();
        let enemy_lineup = battle_context.enemy_lineup.clone();

        let mut table_builder = egui_extras::TableBuilder::new(ui)
            .cell_layout(Layout::centered_and_justified(egui::Direction::LeftToRight));

        let headers = ["Enemy", "HP"];
        for _ in &headers {
            table_builder = table_builder.column(Column::auto_with_initial_suggestion(20.));
        }

        table_builder
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.heading(headers[0]);
                });

                header.col(|ui| {
                    if let Some(handle) = helpers::load_property_icon_image(
                        ui.ctx(),
                        &RPG_GameCore_AvatarPropertyType::BaseHP.to_string(),
                        egui::TextureOptions::default(),
                    ) {
                        let dim = ui
                            .style()
                            .text_styles
                            .get(&TextStyle::Heading)
                            .map(|font_id| font_id.size)
                            .unwrap_or(14.0);
                        let sized_image = egui::load::SizedTexture::new(handle.id(), egui::vec2(dim, dim));
                        ui.add(egui::Image::from_texture(sized_image));
                    }
                });
            })
            .body(|body| {
                body.rows(52.0, enemy_lineup.len(), |mut row| {
                    let row_idx = row.index();
                    let enemy = &enemy_lineup[row_idx];

                    if let Some(i) = battle_context
                        .battle_enemies
                        .iter()
                        .enumerate()
                        .find(|(_, x)| x.entity == *enemy)
                        .map(|(i, _)| i)
                    {
                        row.col(|ui| {
                            ui.with_layout(
                                Layout::centered_and_justified(egui::Direction::LeftToRight),
                                |ui| {
                                    if let Some(handle) = helpers::load_monster_image(
                                        ui.ctx(),
                                        battle_context.enemies[i].id,
                                        egui::TextureOptions::default(),
                                    ) {
                                        let dim = 48.0;
                                        let sized_image = egui::load::SizedTexture::new(
                                            handle.id(),
                                            egui::vec2(dim, dim),
                                        );
                                        let image_response =
                                            ui.add(egui::Image::from_texture(sized_image));

                                        let text_pos = image_response.rect.right_bottom()
                                            - egui::vec2(0.0, 0.0);
                                        let percentage = if battle_context.battle_enemies[i].properties.max_hp() > 0.0 {
                                            (battle_context.battle_enemies[i].properties.current_hp()
                                                / battle_context.battle_enemies[i].properties.max_hp())
                                                * 100.0
                                        } else {
                                            0.0
                                        };
                                        let percentage_text = format!("{percentage:.0}%");

                                        // Text Shadow
                                        ui.painter().text(
                                            text_pos + egui::vec2(-1., 1.),
                                            Align2::RIGHT_BOTTOM,
                                            &percentage_text,
                                            FontId::proportional(dim / 4.0),
                                            Color32::BLACK,
                                        );

                                        ui.painter().text(
                                            text_pos,
                                            Align2::RIGHT_BOTTOM,
                                            &percentage_text,
                                            FontId::proportional(dim / 4.0),
                                            Color32::WHITE,
                                        );

                                    }
                                },
                            );
                        });

                        row.col(|ui| {
                            ui.with_layout(
                                Layout::centered_and_justified(egui::Direction::LeftToRight),
                                |ui| {
                                    ui.label(format!(
                                        "{:.0}",
                                        battle_context.battle_enemies[i].properties.current_hp(),
                                    ));
                                },
                            );
                        });
                    }
                });
            });
    }

    fn show_avatar_portrait(&mut self, ui: &mut Ui, avatar: &Avatar) {
        let avatar_size = Vec2::splat(46.0);
        if let Some(texture) = self.avatar_texture(ui.ctx(), avatar.id) {
            ui.add(
                egui::Image::new((texture.id(), avatar_size))
                    .corner_radius(999.0)
                    .sense(Sense::hover()),
            );
        } else {
            let (rect, _) = ui.allocate_exact_size(avatar_size, Sense::hover());
            let painter = ui.painter_at(rect);
            let fill = helpers::get_character_color(avatar.id as usize).linear_multiply(0.45);
            painter.circle_filled(rect.center(), rect.width() * 0.5, fill);
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                avatar.name.chars().next().unwrap_or('?'),
                egui::FontId::proportional(20.0),
                Color32::WHITE,
            );
        }
    }

    fn avatar_texture(&mut self, ctx: &egui::Context, avatar_id: u32) -> Option<egui::TextureHandle> {
        helpers::load_avatar_image(ctx, avatar_id, egui::TextureOptions::LINEAR)
    }
}

fn create_bar_data(
    real_time_damages: &Vec<f64>,
    avatars: &Vec<Avatar>,
) -> Vec<(Avatar, f64, usize)> {
    let mut bar_data = Vec::new();
    for (i, avatar) in avatars.iter().enumerate() {
        bar_data.push((avatar.clone(), real_time_damages[i], i));
    }
    bar_data
}

fn draw_damage_meter(ui: &mut Ui, fill_ratio: f64, visuals: &egui::Visuals) {
    let desired_size = Vec2::new(ui.available_width().clamp(110.0, 170.0), 12.0);
    let (rect, _) = ui.allocate_exact_size(desired_size, Sense::hover());
    let painter = ui.painter_at(rect);
    let track_fill = if visuals.dark_mode {
        Color32::from_rgba_premultiplied(8, 10, 16, 220)
    } else {
        Color32::from_rgba_premultiplied(40, 42, 48, 80)
    };
    let track_stroke = if visuals.dark_mode {
        Color32::from_rgba_premultiplied(255, 255, 255, 28)
    } else {
        Color32::from_rgba_premultiplied(0, 0, 0, 18)
    };
    let fill = visuals.selection.bg_fill;
    let highlight = fill.gamma_multiply(if visuals.dark_mode { 1.15 } else { 0.92 });

    painter.rect_filled(rect, 5.0, track_fill);
    painter.rect_stroke(rect, 5.0, Stroke::new(1.0, track_stroke), StrokeKind::Inside);

    let clamped_ratio = fill_ratio.clamp(0.0, 1.0) as f32;
    if clamped_ratio <= 0.0 {
        return;
    }

    let fill_rect = egui::Rect::from_min_max(
        rect.min,
        egui::pos2(rect.left() + rect.width() * clamped_ratio, rect.bottom()),
    );
    painter.rect_filled(fill_rect, 5.0, fill);

    let highlight_y = fill_rect.top() + 1.5;
    painter.line_segment(
        [
            egui::pos2(fill_rect.left() + 2.0, highlight_y),
            egui::pos2((fill_rect.right() - 2.0).max(fill_rect.left() + 2.0), highlight_y),
        ],
        Stroke::new(1.0, highlight),
    );
}

fn create_damage_portrait_rows(
    avatars: &[Avatar],
    real_time_damages: &[f64],
    real_time_overkill_damages: &[f64],
    action_value: f64,
) -> Vec<DamagePortraitRow> {
    let total_damage = real_time_damages.iter().sum::<f64>();
    avatars
        .iter()
        .enumerate()
        .map(|(index, avatar)| {
            let damage = *real_time_damages.get(index).unwrap_or(&0.0);
            let overkill_damage = *real_time_overkill_damages.get(index).unwrap_or(&0.0);
            DamagePortraitRow {
                avatar: avatar.clone(),
                damage,
                dpav: if action_value > 0.0 {
                    damage / action_value
                } else {
                    damage
                },
                effective_damage: (damage - overkill_damage).max(0.0),
                overkill_damage,
                percentage: if total_damage > 0.0 {
                    damage / total_damage * 100.0
                } else {
                    0.0
                },
            }
        })
        .collect::<Vec<_>>()
}

const ORDERED_DAMAGE_TYPES: [RPG_GameCore_AttackType; 12] = [
    RPG_GameCore_AttackType::Normal,
    RPG_GameCore_AttackType::BPSkill,
    RPG_GameCore_AttackType::Ultra,
    RPG_GameCore_AttackType::QTE,
    RPG_GameCore_AttackType::DOT,
    RPG_GameCore_AttackType::Pursued,
    RPG_GameCore_AttackType::Maze,
    RPG_GameCore_AttackType::Insert,
    RPG_GameCore_AttackType::ElementDamage,
    RPG_GameCore_AttackType::Servant,
    RPG_GameCore_AttackType::TrueDamage,
    RPG_GameCore_AttackType::ElationDamage,
];

fn aggregate_damage_by_category(breakdowns: &[DamageTypeBreakdown]) -> DamageTypeBreakdown {
    let mut aggregate = DamageTypeBreakdown::default();

    for breakdown in breakdowns {
        for (damage_type, damage) in breakdown {
            *aggregate.entry(damage_type.clone()).or_insert(0.0) += *damage;
        }
    }

    aggregate
}

fn draw_damage_category_grid(
    ui: &mut Ui,
    id: &str,
    breakdown: &DamageTypeBreakdown,
    selected_damage: f64,
    total_damage: f64,
) {
    egui::Grid::new(id).striped(true).num_columns(4).show(ui, |ui| {
        ui.strong(t!("Category"));
        ui.strong(t!("Damage"));
        ui.strong(t!("Selection Share"));
        ui.strong(t!("Party Share"));
        ui.end_row();

        for (_, category, damage) in collect_damage_category_points(breakdown) {
            if damage <= 0.0 {
                continue;
            }

            ui.label(&category);
            ui.label(helpers::format_damage(damage));
            ui.label(format!("{:.1}%", (damage / selected_damage) * 100.0));
            ui.label(format!("{:.1}%", (damage / total_damage) * 100.0));
            ui.end_row();
        }
    });
}

fn show_damage_category_pie_chart(ui: &mut Ui, breakdown: &DamageTypeBreakdown) {
    let chart_data = collect_damage_category_points(breakdown);
    let available = ui.available_size();
    let chart_height = 220.0;
    let total_damage: f64 = chart_data.iter().map(|(_, _, damage)| *damage).sum();

    Plot::new("damage_type_breakdown_pie")
        .height(chart_height)
        .width(available.x)
        .data_aspect(1.0)
        .clamp_grid(true)
        .show_grid(false)
        .show_background(false)
        .show_axes([false; 2])
        .allow_scroll(false)
        .allow_drag(false)
        .allow_zoom(false)
        .show(ui, |plot_ui| {
            let values: Vec<f64> = chart_data.iter().map(|(_, _, damage)| *damage).collect();
            let segments = create_pie_segments_from_values(&values);

            for ((attack_type, category, damage), segment) in chart_data.iter().zip(segments.into_iter()) {
                let color = get_damage_category_color(attack_type);
                let polygon = Polygon::new(category, PlotPoints::new(segment.points))
                    .stroke(Stroke::new(1.5, color))
                    .fill_color(color.linear_multiply(0.35))
                    .name(format!(
                        "{}: {} ({:.1}%)",
                        category,
                        helpers::format_damage(*damage),
                        (damage / total_damage) * 100.0,
                    ));

                plot_ui.polygon(polygon);
            }
        });
}

fn show_damage_category_bar_chart(ui: &mut Ui, breakdown: &DamageTypeBreakdown) {
    let chart_data = collect_damage_category_points(breakdown);
    let available = ui.available_size();
    let chart_height = 220.0;
    let max_damage = chart_data
        .iter()
        .map(|(_, _, damage)| *damage)
        .fold(0.0, f64::max);
    let y_headroom = if max_damage > 0.0 {
        max_damage * 1.15
    } else {
        1.0
    };

    Plot::new("damage_type_breakdown_bar")
        .height(chart_height)
        .width(available.x)
        .include_y(0.0)
        .include_y(y_headroom)
        .allow_drag(false)
        .allow_zoom(false)
        .allow_scroll(false)
        .show_background(false)
        .show_axes([false, true])
        .y_axis_formatter(|y, _| helpers::format_damage(y.value))
        .show(ui, |plot_ui| {
            for (index, (attack_type, category, damage)) in chart_data.iter().enumerate() {
                let bar = Bar::new(index as f64, *damage)
                    .name(category)
                    .fill(get_damage_category_color(attack_type))
                    .width(0.7);

                plot_ui.bar_chart(
                    BarChart::new(category, vec![bar])
                        .id(format!("damage_type_breakdown_bar_{index}")),
                );
            }
        });
}

fn draw_damage_category_legend(ui: &mut Ui, breakdown: &DamageTypeBreakdown) {
    let chart_data = collect_damage_category_points(breakdown);
    if chart_data.is_empty() {
        return;
    }

    let column_count = chart_data.len().min(3);
    let row_count = chart_data.len().div_ceil(column_count);
    let desired_height = row_count as f32 * 24.0 + 8.0;
    let max_height = desired_height.min(ui.available_height());

    egui::ScrollArea::vertical()
        .id_salt("damage_type_breakdown_legend_scroll")
        .max_height(max_height)
        .auto_shrink([false, true])
        .show(ui, |ui| {
            egui::Grid::new("damage_type_breakdown_legend_grid")
                .num_columns(column_count)
                .spacing([12.0, 6.0])
                .show(ui, |ui| {
                    for (index, (attack_type, category, damage)) in chart_data.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.colored_label(get_damage_category_color(attack_type), "■");
                            ui.label(category)
                                .on_hover_text(helpers::format_damage(*damage));
                        });

                        if (index + 1) % column_count == 0 {
                            ui.end_row();
                        }
                    }

                    if chart_data.len() % column_count != 0 {
                        ui.end_row();
                    }
                });
        });
}

fn collect_damage_category_points(
    breakdown: &DamageTypeBreakdown,
) -> Vec<(RPG_GameCore_AttackType, String, f64)> {
    let mut points = Vec::new();

    for known in ORDERED_DAMAGE_TYPES {
        if let Some(damage) = breakdown.get(&known) {
            if *damage > 0.0 {
                points.push((known, display_damage_type(&known), *damage));
            }
        }
    }

    for (damage_type, damage) in breakdown {
        if ORDERED_DAMAGE_TYPES.contains(damage_type) {
            continue;
        }
        if *damage > 0.0 {
            points.push((*damage_type, display_damage_type(damage_type), *damage));
        }
    }

    points
}

fn get_damage_category_color(category: &RPG_GameCore_AttackType) -> Color32 {
    match category {
        RPG_GameCore_AttackType::Normal => Color32::from_rgb(93, 156, 236),
        RPG_GameCore_AttackType::BPSkill => Color32::from_rgb(72, 201, 176),
        RPG_GameCore_AttackType::Ultra => Color32::from_rgb(241, 196, 15),
        RPG_GameCore_AttackType::QTE => Color32::from_rgb(155, 89, 182),
        RPG_GameCore_AttackType::DOT => Color32::from_rgb(231, 76, 60),
        RPG_GameCore_AttackType::Pursued => Color32::from_rgb(230, 126, 34),
        RPG_GameCore_AttackType::Maze | RPG_GameCore_AttackType::MazeNormal => {
            Color32::from_rgb(52, 152, 219)
        }
        RPG_GameCore_AttackType::Insert => Color32::from_rgb(46, 204, 113),
        RPG_GameCore_AttackType::ElementDamage => Color32::from_rgb(26, 188, 156),
        RPG_GameCore_AttackType::Servant => Color32::from_rgb(149, 165, 166),
        RPG_GameCore_AttackType::TrueDamage => Color32::from_rgb(127, 140, 141),
        RPG_GameCore_AttackType::ElationDamage => Color32::from_rgb(255, 105, 180),
        _ => Color32::from_rgb(189, 195, 199),
    }
}

fn create_pie_segments(
    real_time_damages: &Vec<f64>,
    avatars: &Vec<Avatar>,
) -> Vec<(Avatar, PieSegment, usize)> {
    let generic_segments = create_pie_segments_from_values(real_time_damages);
    let mut segments = Vec::new();

    for (i, (avatar, segment)) in avatars.iter().zip(generic_segments.into_iter()).enumerate() {
        segments.push((
            avatar.clone(),
            segment,
            i,
        ));
    }

    segments
}

fn create_pie_segments_from_values(values: &[f64]) -> Vec<PieSegment> {
    let total_damage = values.iter().sum::<f64>();
    if total_damage <= 0.0 {
        return Vec::new();
    }

    let mut segments = Vec::new();
    let mut start_angle = -std::f64::consts::FRAC_PI_2;

    for damage in values {
        let fraction = *damage / total_damage;
        let angle = fraction * std::f64::consts::TAU;
        let end_angle = start_angle + angle;

        segments.push(PieSegment {
            points: create_pie_slice(start_angle, end_angle),
            value: *damage,
        });

        start_angle = end_angle;
    }

    segments
}

fn create_pie_slice(start_angle: f64, end_angle: f64) -> Vec<[f64; 2]> {
    let center = [0.0, 0.0];
    let radius = 0.8;
    let mut points = vec![center];

    let steps = 50;
    let p = (end_angle - start_angle) / (steps as f64);
    for i in 0..=steps {
        let angle = start_angle + p * i as f64;
        let (sin, cos) = angle.sin_cos();
        points.push([cos * radius, sin * radius]);
    }
    points.push(center);

    points
}
