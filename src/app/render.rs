use crate::expressions::*;
use ahash::AHashMap;
use chumsky::Parser;
use eframe::egui;
use eframe::egui::plot;
use egui_extras::{Size, TableBuilder};
use rayon::prelude::*;
use chrono::{Local, DateTime};

#[derive(PartialEq)]
pub enum ToolsTab {
    Hidden,
    Errors,
    Inspector,
}

pub struct RenderWorkspace {
    parser_result: Vec<parser::Assignment>,
    parser_errors: Vec<errors::Error>,
    eval_errors: Vec<Vec<errors::Error>>,
    eval_variables: Vec<AHashMap<String, f64>>,
    eval_result: Vec<RenderedPoint>,
    projection_start_time: DateTime<Local>,
    tools_tab: ToolsTab,
    tools_index_tb: u16,
    pub eval_frozen: bool,
    encountered_eval_error: bool,
    encountered_parser_error: bool,
    eval_error_indexes: Vec<u16>,
}

impl Default for RenderWorkspace {
    fn default() -> Self {
        Self {
            parser_result: vec![],
            parser_errors: vec![],
            eval_errors: vec![],
            eval_variables: vec![],
            eval_result: vec![],
            projection_start_time: Local::now(),
            tools_tab: ToolsTab::Hidden,
            tools_index_tb: 0,
            eval_frozen: false,
            encountered_eval_error: false,
            encountered_parser_error: false,
            eval_error_indexes: vec![],
        }
    }
}

#[derive(Debug)]
struct RenderedPoint {
    x: f64,
    y: f64,
    h: f64,
    s: f64,
    v: f64,
    index: u16,
}

impl RenderWorkspace {
    pub fn on_switch_render(&mut self, project: &crate::project::Project) {
        self.eval_frozen = false;
        self.encountered_eval_error = false;
        self.encountered_parser_error = false;
        self.eval_error_indexes = vec![];
        self.eval_errors = vec![];
        self.parser_errors = vec![];

        match parser::parser().parse(project.text_data.content.clone()) {
            Ok(value) => {
                self.parser_errors = vec![];
                self.parser_result = value;
                self.calculate_points(project.text_data.content.clone(), 20, 20);
            }
            Err(error) => {
                self.parser_errors = error
                .iter()
                .map(|err| {
                    parser::process_parser_error(err.clone(), project.text_data.content.clone())
                })
                .collect();

                // calculate a set of points so that we don't panic
                self.calculate_points("".to_string(), 20, 20);
                self.encountered_parser_error = true;
                self.tools_tab = ToolsTab::Errors;
            }
        };

        self.projection_start_time = Local::now();
    }

    fn calculate_points(&mut self, text: String, x_size: u16, y_size: u16) -> Vec<RenderedPoint> {
        self.encountered_eval_error = false;
        self.eval_error_indexes = vec![]; 

        let time = Local::now().naive_local().timestamp_millis() as f64 / 1000.0;

        let projection_start_time = self.projection_start_time.naive_local().timestamp_millis() as f64 / 1000.0;

        let base_ctx = eval::EvalContext {
            x: 0.0,
            y: 0.0,
            index: 0.0,
            count: (x_size * y_size) as f64,
            fraction: 0.0,
            pi: std::f64::consts::PI,
            tau: std::f64::consts::TAU,
            time,
            projection_time: time - projection_start_time,
            projection_start_time,
        };

        self.eval_errors = vec![];
        self.eval_variables = vec![];

        let points: Vec<(AHashMap<String, f64>, Vec<errors::Error>, eval::EvalContext, u16)> = (0..(x_size * y_size))
        .into_par_iter()
        .map(|index| {
            let mut ctx = base_ctx.clone();

            let f_index = index as f64;
            let x_size = x_size as f64;
            let y_size = y_size as f64;

            ctx.index = f_index;
            ctx.x = -100.0 + (f_index % x_size)*(200.0/(x_size - 1.0));
            ctx.y = 100.0 - (200.0/(y_size - 1.0)) * f64::floor(f_index / x_size);
            ctx.fraction = f_index / (ctx.count - 1.0);

            let mut hash_map = AHashMap::new();

            let result = eval::run(
                    self.parser_result.clone(),
            text.clone(),
            &mut hash_map,
            ctx,
            );
            let error = result.1.clone();
            (hash_map, error, ctx, index)
        })
        .collect();

        for tuple in points.clone() {
            if tuple.1.len() > 0 {
                self.encountered_eval_error = true;
                self.eval_error_indexes.push(tuple.3);
                self.eval_frozen = true;
                self.tools_index_tb = tuple.3;
                self.tools_tab = ToolsTab::Errors;
            }

            self.eval_errors.push(tuple.1.clone());
            self.eval_variables.push(tuple.0.clone());
        }

    let calculated_points = points
    .par_iter()
    .map(|(variables, _errors, _ctx, index)| RenderedPoint {
        x: *variables.get("x'").unwrap_or(&0.0),
        y: *variables.get("y'").unwrap_or(&0.0),
        h: *variables.get("h").unwrap_or(&0.0),
        s: *variables.get("s").unwrap_or(&0.0),
        v: *variables.get("v").unwrap_or(&1.0),
        index: *index,
    })
    .collect();

        calculated_points
    }

    pub fn update_render_workspace(&mut self, ctx: &egui::Context, project: &mut crate::project::Project) {
        let mut tools_frame = egui::Frame::default();

        tools_frame.fill = ctx.style().visuals.window_fill();
        tools_frame.stroke = ctx.style().visuals.window_stroke();

        let mut frame = egui::Frame::default();

        frame.fill = egui::Color32::DARK_RED;
        frame.stroke = egui::Stroke::new(1.0, egui::Color32::RED);
        frame.inner_margin = egui::style::Margin {
            left: 5.0,
            right: 5.0,
            top: 2.0,
            bottom: 5.0
        };
        frame.outer_margin = egui::style::Margin::same(10.0);
        frame.rounding = frame.rounding.at_least(1.0);

        if self.encountered_parser_error {
            egui::containers::Area::new("Parser Error")
                .fixed_pos(egui::pos2(0.0, 30.0))
                .show(ctx, |ui| {
                    frame.show(ui, |ui| {
                        ui.label(egui::RichText::new("A parser error occured; see the errors displayed below.").color(egui::Color32::WHITE))
                    })
                });
        } else if self.encountered_eval_error {
            egui::containers::Area::new("Eval Error")
                .fixed_pos(egui::pos2(0.0, 30.0))
                .show(ctx, |ui| {
                    frame.show(ui, |ui| {
                        let index_text = self.eval_error_indexes.iter()
                            .map(|index| index.to_string())
                            .collect::<Vec<String>>()
                            .join(", ");

                        ui.label(egui::RichText::new("Execution stopped - an evaluation error occured in the following index(es): ".to_string() + &index_text).color(egui::Color32::WHITE))
                    });
                    ui.add_space(10.0);
                });
        }

        egui::TopBottomPanel::bottom("info_render_toolbar")
        .frame(tools_frame)
        .resizable(true)
        .show(ctx, |ui| {
            ui.spacing_mut().button_padding.x = 5.0;
            ui.spacing_mut().button_padding.y = 3.0;
            ui.spacing_mut().item_spacing.x = 3.0;
            ui.spacing_mut().item_spacing.y = 3.0;

            let tab_radius = egui::Rounding {
                ne: 2.0,
                se: 2.0,
                nw: 2.0,
                sw: 2.0,
            };

            ui.visuals_mut().widgets.active.rounding = tab_radius;
            ui.visuals_mut().widgets.hovered.rounding = tab_radius;
            ui.visuals_mut().widgets.inactive.rounding = tab_radius;

            let mut control_frame = egui::Frame::default();

            control_frame.inner_margin = egui::style::Margin {
                left: 5.0,
                right: 5.0,
                top: 5.0,
                bottom: if self.tools_tab != ToolsTab::Hidden {
                    0.0
                } else {
                    5.0
                },
            };

            control_frame.show(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui.selectable_label(!self.eval_frozen, "▶").clicked() {
                        self.eval_frozen = false;
                    }

                    if ui.selectable_label(self.eval_frozen, "⬛").clicked() {
                        self.eval_frozen = true;
                    }

                    ui.separator();

                    if ui
                    .selectable_label(self.tools_tab == ToolsTab::Hidden, "Hide")
                    .clicked()
                    {
                        self.tools_tab = ToolsTab::Hidden;
                    };

                    let text = if self.encountered_parser_error {
                        egui::RichText::new("！ Errors").color(egui::Color32::RED)
                    } else if self.encountered_eval_error {
                        egui::RichText::new("⚠ Errors").color(egui::Color32::YELLOW)
                    } else {
                        egui::RichText::new("Errors")
                    };

                    if ui
                    .selectable_label(self.tools_tab == ToolsTab::Errors, text)
                    .clicked()
                    {
                        self.tools_tab = ToolsTab::Errors;
                    };

                    if ui
                    .selectable_label(self.tools_tab == ToolsTab::Inspector, "Inspector")
                    .clicked()
                    {
                        self.tools_tab = ToolsTab::Inspector;
                    };

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::BOTTOM), |ui| {
                        if self.tools_tab != ToolsTab::Hidden {
                            let index_value = egui::DragValue::new(&mut self.tools_index_tb)
                            .clamp_range(0..=399)
                            .prefix("inspecting index: ");

                            ui.add(index_value);

                            ui.separator();
                        }

                        let y_value = egui::DragValue::new(&mut project.text_data.size_y)
                        .clamp_range(2..=20);

                        ui.add(y_value);

                        let mut frame = egui::Frame::default();
                        frame.inner_margin = egui::style::Margin {bottom: 3.0, left: 0.0, right: 0.0, top: 0.0};

                        frame.clone().show(ui, |ui| {
                            ui.monospace("by ");
                        });

                        let x_value = egui::DragValue::new(&mut project.text_data.size_x)
                        .clamp_range(2..=20);

                        ui.add(x_value);

                        frame.show(ui, |ui| {
                            ui.monospace("rectangular grid, ")
                        });
                    });
                });
            });

            if self.tools_tab == ToolsTab::Errors {
                ui.separator();

                let index = self.tools_index_tb as usize;

                let eval_errors = self.eval_errors[index].clone();
                let parser_errors = self.parser_errors.clone();

                ui.visuals_mut().widgets.active.rounding = egui::Rounding::none();
                ui.visuals_mut().widgets.hovered.rounding = egui::Rounding::none();
                ui.visuals_mut().widgets.inactive.rounding = egui::Rounding::none();

                TableBuilder::new(ui)
                .column(Size::exact(110.0))
                .column(Size::exact(100.0))
                .column(Size::remainder())
                .striped(true)
                .header(22.0, |mut header| {
                    header.col(|ui| {
                        ui.horizontal(|ui| {
                            ui.add_space(10.0);
                            ui.label(egui::RichText::new("Type").strong());
                        });
                    });
                    header.col(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Location").strong());
                        });
                    });
                    header.col(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Error").strong());
                        });
                    });
                })
                .body(|mut body| {
                    for error in eval_errors {
                        body.row(18.0, |mut row| {
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    ui.add_space(10.0);
                                    ui.label("Runtime");
                                });
                            });
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(
                                            error.line_number.to_string()
                                            + &String::from(":")
                                            + &error.col_number.to_string(),
                                    );
                                });
                            });
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(error.reason.to_string());
                                });
                            });
                        });
                    }
                    for error in parser_errors {
                        body.row(18.0, |mut row| {
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    ui.add_space(10.0);
                                    ui.label("Parse");
                                });
                            });
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(
                                            error.line_number.to_string()
                                            + &String::from(":")
                                            + &error.col_number.to_string(),
                                    );
                                });
                            });
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(error.reason.to_string());
                                });
                            });
                        });
                    }
                });
                //ui.allocate_space(ui.available_size());
            }

            if self.tools_tab == ToolsTab::Inspector {
                ui.separator();

                let index = self.tools_index_tb as usize;

                let eval_variables: AHashMap<String, f64> =
                self.eval_variables[index].clone();

                ui.visuals_mut().widgets.active.rounding = egui::Rounding::none();
                ui.visuals_mut().widgets.hovered.rounding = egui::Rounding::none();
                ui.visuals_mut().widgets.inactive.rounding = egui::Rounding::none();

                TableBuilder::new(ui)
                .column(Size::exact(160.0))
                .column(Size::remainder().at_least(150.0))
                .striped(true)
                .header(22.0, |mut header| {
                    header.col(|ui| {
                        ui.horizontal(|ui| {
                            ui.add_space(10.0);
                            ui.label(egui::RichText::new("Name").strong());
                        });
                    });
                    header.col(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Value").strong());
                        });
                    });
                })
                .body(|mut body| {
                    let mut sorted: Vec<_> = eval_variables.iter().collect();
                    sorted.sort_by_key(|a| a.0);

                    for variable in sorted {
                        body.row(18.0, |mut row| {
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    ui.add_space(10.0);
                                    ui.monospace(variable.0);
                                });
                            });
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    ui.monospace(variable.1.to_string());
                                });
                            });
                        })
                    }
                }); 
                //ui.allocate_space(ui.available_size());
            }
        });

        if !self.eval_frozen && !self.encountered_parser_error {
            self.eval_result =
            self.calculate_points(project.text_data.content.clone(), project.text_data.size_x as u16, project.text_data.size_y as u16);
        }

        let mut frame = egui::Frame::default();

        frame.inner_margin = egui::style::Margin {
            left: 0.0,
            right: 0.0,
            top: 0.0,
            bottom: 0.0,
        };
        frame.fill = ctx.style().visuals.window_fill();

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            let plot = plot::Plot::new("render_plot")
            .show_axes([false, false])
            .data_aspect(1.0)
            .include_x(150.0)
            .include_x(-150.0)
            .include_y(150.0)
            .include_y(-150.0);

            if !self.encountered_parser_error {
                plot.show(ui, |plot_ui| {
                    for point in self.eval_result.iter() {
                        if point.v != 0.0 {
                            let plot_point = plot::Points::new(vec![[point.x, point.y]])
                            .filled(true)
                            .radius(3.0)
                            .color(egui::color::Hsva::new(
                                    (point.h % 360.0 / 360.0) as f32,
                            point.s as f32,
                            point.v as f32,
                            1.0,
                            ))
                            .name(format!("index: {}", point.index));

                            plot_ui.points(plot_point);
                        }
                    }
                });

                if !self.eval_frozen {
                    ctx.request_repaint();
                }
            }
        });
    }
}
