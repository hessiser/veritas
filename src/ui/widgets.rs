use crate::ui::app::{DamageBreakdownChart, DamageBreakdownScope, GraphUnit};
use egui::{Color32, Sense, Stroke, Ui, Vec2};
use egui_plot::{Bar, BarChart, Legend, Line, Plot, PlotPoints, Polygon};

use crate::{
    battle::{BattleContext, DamageCategory, DamageCategoryBreakdown},
    models::misc::Avatar,
};

use super::{app::App, helpers};

pub struct PieSegment {
    pub points: Vec<[f64; 2]>,
    pub value: f64,
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

        // I need to make separate DPAV calcs in the battle context
        for (i, avatar) in battle_context.avatar_lineup.iter().enumerate() {
            ui.horizontal(|ui| {
                let style = ui.style_mut();
                style.override_text_style = Some(self.config.legend_text_style.clone());
                let (res, painter) = ui.allocate_painter(Vec2::splat(12.), Sense::empty());
                let rect = res.rect;
                let radius = rect.width() / 2.0 - 1.0;
                painter.circle_filled(rect.center(), radius, helpers::get_character_color(i));

                let dmg = battle_context.real_time_damages[i];

                let percentage = dmg / battle_context.total_damage * 100.0;

                
                let dpav = if battle_context.action_value > 0.0 {
                    dmg / battle_context.action_value
                } else {
                    dmg
                };

                ui.label(format!(
                    "{}: {:.1}% | {} DMG | {} DPAV",
                    avatar.name,
                    percentage,
                    helpers::format_damage(dmg),
                    helpers::format_damage(dpav)
                ));
            });
        }

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
            .legend(
                Legend::default()
                    .text_style(self.config.legend_text_style.clone()),
            )
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

                let overkill_bars_data = create_bar_data(&real_time_overkill_damages, &avatar_lineup);
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
                let overkill_bar_chart = BarChart::new("Overkill", overkill_bars).color(overkill_color)
                        .id("overkill_dmg_bar_chart")
                        .stack_on(&[&dmg_bar_chart]);
                plot_ui.bar_chart(dmg_bar_chart);
                plot_ui.bar_chart(
                    overkill_bar_chart
                );
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
                                battle_context.battle_avatars[i].battle_stats.av
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

        ui.vertical(|ui| {
            for enemy in &enemy_lineup {
                if let Some(i) = battle_context
                    .battle_enemies
                    .iter()
                    .enumerate()
                    .find(|(_, x)| x.entity == *enemy)
                    .map(|(i, _)| i)
                {
                    ui.horizontal(|ui| {
                        ui.label(format!("{}: ", &battle_context.enemies[i].name));
                        ui.label(format!(
                            "{:.2} {}",
                            battle_context.battle_enemies[i].battle_stats.hp,
                            t!("HP")
                        ));
                    });
                }
            }
        });
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

fn aggregate_damage_by_category(
    breakdowns: &[DamageCategoryBreakdown],
) -> DamageCategoryBreakdown {
    let mut aggregate = DamageCategoryBreakdown::default();

    for breakdown in breakdowns {
        for category in DamageCategory::ALL {
            aggregate.add(category, breakdown.amount(category));
        }
    }

    aggregate
}

fn draw_damage_category_grid(
    ui: &mut Ui,
    id: &str,
    breakdown: &DamageCategoryBreakdown,
    selected_damage: f64,
    total_damage: f64,
) {
    egui::Grid::new(id).striped(true).num_columns(4).show(ui, |ui| {
        ui.strong(t!("Category"));
        ui.strong(t!("Damage"));
        ui.strong(t!("Selection Share"));
        ui.strong(t!("Party Share"));
        ui.end_row();

        for category in DamageCategory::ALL {
            let damage = breakdown.amount(category);
            if damage <= 0.0 {
                continue;
            }

            ui.label(category.label());
            ui.label(helpers::format_damage(damage));
            ui.label(format!("{:.1}%", (damage / selected_damage) * 100.0));
            ui.label(format!("{:.1}%", (damage / total_damage) * 100.0));
            ui.end_row();
        }
    });
}

fn show_damage_category_pie_chart(ui: &mut Ui, breakdown: &DamageCategoryBreakdown) {
    let chart_data = collect_damage_category_points(breakdown);
    let available = ui.available_size();
    let chart_height = 220.0;
    let total_damage: f64 = chart_data.iter().map(|(_, damage)| *damage).sum();

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
            let values: Vec<f64> = chart_data.iter().map(|(_, damage)| *damage).collect();
            let segments = create_pie_segments_from_values(&values);

            for ((category, damage), segment) in chart_data.iter().zip(segments.into_iter()) {
                let color = get_damage_category_color(*category);
                let polygon = Polygon::new(category.label(), PlotPoints::new(segment.points))
                    .stroke(Stroke::new(1.5, color))
                    .fill_color(color.linear_multiply(0.35))
                    .name(format!(
                        "{}: {} ({:.1}%)",
                        category.label(),
                        helpers::format_damage(*damage),
                        (damage / total_damage) * 100.0,
                    ));

                plot_ui.polygon(polygon);
            }
        });
}

fn show_damage_category_bar_chart(ui: &mut Ui, breakdown: &DamageCategoryBreakdown) {
    let chart_data = collect_damage_category_points(breakdown);
    let available = ui.available_size();
    let chart_height = 220.0;
    let max_damage = chart_data
        .iter()
        .map(|(_, damage)| *damage)
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
            for (index, (category, damage)) in chart_data.iter().enumerate() {
                let bar = Bar::new(index as f64, *damage)
                    .name(category.label())
                    .fill(get_damage_category_color(*category))
                    .width(0.7);

                plot_ui.bar_chart(
                    BarChart::new(category.label(), vec![bar])
                        .id(format!("damage_type_breakdown_bar_{index}")),
                );
            }
        });
}

fn draw_damage_category_legend(ui: &mut Ui, breakdown: &DamageCategoryBreakdown) {
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
                    for (index, (category, damage)) in chart_data.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.colored_label(get_damage_category_color(*category), "■");
                            ui.label(category.label())
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

fn collect_damage_category_points(breakdown: &DamageCategoryBreakdown) -> Vec<(DamageCategory, f64)> {
    DamageCategory::ALL
        .into_iter()
        .filter_map(|category| {
            let damage = breakdown.amount(category);
            (damage > 0.0).then_some((category, damage))
        })
        .collect()
}

fn get_damage_category_color(category: DamageCategory) -> Color32 {
    match category {
        DamageCategory::Basic => Color32::from_rgb(93, 156, 236),
        DamageCategory::Skill => Color32::from_rgb(72, 201, 176),
        DamageCategory::Ultimate => Color32::from_rgb(241, 196, 15),
        DamageCategory::Qte => Color32::from_rgb(155, 89, 182),
        DamageCategory::Dot => Color32::from_rgb(231, 76, 60),
        DamageCategory::FollowUp => Color32::from_rgb(230, 126, 34),
        DamageCategory::Technique => Color32::from_rgb(52, 152, 219),
        DamageCategory::Additional => Color32::from_rgb(46, 204, 113),
        DamageCategory::Break => Color32::from_rgb(26, 188, 156),
        DamageCategory::Servant => Color32::from_rgb(149, 165, 166),
        DamageCategory::TrueDamage => Color32::from_rgb(127, 140, 141),
        DamageCategory::Elation => Color32::from_rgb(255, 105, 180),
        DamageCategory::Other => Color32::from_rgb(189, 195, 199),
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
