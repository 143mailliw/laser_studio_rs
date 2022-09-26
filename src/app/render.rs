use crate::expressions::*;
use chumsky::Parser;
use eframe::egui;
use eframe::egui::plot;
use rayon::prelude::*;
use std::time;
use ahash::AHashMap;
use egui_extras::{TableBuilder, Size};

#[derive(PartialEq)]
pub enum ToolsTab {
    Hidden,
    Errors,
    Inspector
}


pub struct RenderWorkspace {
    parser_result: Vec<parser::Assignment>,
    parser_errors: Vec<errors::Error>,
    eval_errors: Vec<Vec<errors::Error>>,
    eval_variables: Vec<AHashMap<String, f64>>,
    eval_result: Vec<RenderedPoint>,
    projection_start_time: time::Duration,
    tools_tab: ToolsTab,
    tools_index_tb: String,
    tools_index_invalid: bool,
    eval_frozen: bool
}

impl Default for RenderWorkspace {
    fn default() -> Self {
        Self {
            parser_result: vec![],
            parser_errors: vec![],
            eval_errors: vec![],
            eval_variables: vec![],
            eval_result: vec![],
            projection_start_time: time::SystemTime::now()
                .duration_since(time::UNIX_EPOCH)
                .expect("time went backwards"),
            tools_tab: ToolsTab::Hidden,
            tools_index_tb: "0".into(),
            tools_index_invalid: false,
            eval_frozen: false
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
    index: u16
}

pub fn on_switch_render(app: &mut super::LaserStudioApp) {
    app.render.parser_result = match parser::parser().parse(app.project.text_data.content.clone()) {
        Ok(value) => value,
        Err(error) => panic!("Error parsing: {:?}", error)
    };

    app.render.projection_start_time = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .expect("time went backwards");
}


fn calculate_points(workspace: &mut RenderWorkspace, text: String) -> Vec<RenderedPoint> {
    let time = time::SystemTime::now()
        .duration_since(time::SystemTime::UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs_f64();

    let projection_start_time = workspace.projection_start_time.as_secs_f64();

    let base_ctx = eval::EvalContext {
        x: 0.0,
        y: 0.0,
        index: 0.0,
        count: 400.0,
        fraction: 0.0,
        pi: std::f64::consts::PI,
        tau: std::f64::consts::TAU,
        time,
        projection_time: time - projection_start_time,
        projection_start_time
    };

    workspace.eval_errors = vec![];
    workspace.eval_variables = vec![];

    let points: Vec<(AHashMap<String, f64>, Vec<errors::Error>)> = (0..400)
        .into_par_iter()
        .map(|index| {
            let mut ctx = base_ctx.clone();

            ctx.index = index as f64;
            ctx.x = ((index % 20 - 10) * 10) as f64;
            ctx.y = ((index / 20 - 9) * 10) as f64;
            ctx.fraction = index as f64 / 400.0;

            let mut hash_map = AHashMap::new();
        
            let result = eval::run(workspace.parser_result.clone(), text.clone(), &mut hash_map, ctx);
            let error = result.1.clone();

            hash_map.insert("index".into(), index as f64);

            (hash_map, error)
        })
        .collect();

    for tuple in points.clone() {
        workspace.eval_errors.push(tuple.1.clone());
        workspace.eval_variables.push(tuple.0.clone());
    }

    let calculated_points = points
        .par_iter()
        .map(|(variables, _errors)| {
            RenderedPoint {
                x: *variables.get("x'").unwrap_or(&0.0),
                y: *variables.get("y'").unwrap_or(&0.0),
                h: *variables.get("h").unwrap_or(&0.0),
                s: *variables.get("s").unwrap_or(&0.0),
                v: *variables.get("v").unwrap_or(&0.0),
                index: *variables.get("index").unwrap_or(&0.0) as u16
            }
        })
        .collect();

    calculated_points
}

pub fn update_render_workspace(ctx: &egui::Context, app: &mut super::LaserStudioApp) {
    if !app.render.eval_frozen {
        app.render.eval_result = calculate_points(&mut app.render, app.project.text_data.content.clone());
    }

    let mut frame = egui::Frame::default();

    frame.inner_margin = egui::style::Margin {left: 0.0, right: 0.0, top: 0.0, bottom: 0.0};
    frame.fill = ctx.style().visuals.window_fill();

    egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
        let plot = plot::Plot::new("render_plot")
            .show_axes([false, false])
            .data_aspect(1.0)
            .include_x(150.0)
            .include_x(-150.0)
            .include_y(150.0)
            .include_y(-150.0);

        plot.show(ui, |plot_ui| {
            for point in app.render.eval_result.iter() {
                if point.v != 0.0 {
                    let plot_point = plot::Points::new(vec!([point.x, point.y]))
                        .filled(true)
                        .radius(3.0)
                        .color(egui::color::Hsva::new((point.h % 360.0 / 360.0) as f32, point.s as f32, point.v as f32, 1.0))
                        .name(format!("index: {}", point.index));

                    plot_ui.points(plot_point);
                }
            }
        });

        if !app.render.eval_frozen {
            ctx.request_repaint();
        }
    });

    let mut tools_frame = egui::Frame::default();

    tools_frame.inner_margin = egui::style::Margin {left: 5.0, right: 5.0, top: 5.0, bottom: 5.0};
    tools_frame.fill = ctx.style().visuals.window_fill();
    tools_frame.stroke = ctx.style().visuals.window_stroke();

    egui::TopBottomPanel::bottom("info_render_toolbar").frame(tools_frame).max_height(400.0).resizable(true).show(ctx, |ui| {
        ui.spacing_mut().button_padding.x = 5.0;
        ui.spacing_mut().button_padding.y = 3.0;
        ui.spacing_mut().item_spacing.x = 3.0;
        ui.spacing_mut().item_spacing.y = 3.0;

        let tab_radius = egui::Rounding {ne: 2.0, se: 2.0, nw: 2.0, sw: 2.0};

        ui.visuals_mut().widgets.active.rounding = tab_radius;
        ui.visuals_mut().widgets.hovered.rounding = tab_radius;
        ui.visuals_mut().widgets.inactive.rounding = tab_radius;

        ui.horizontal(|ui| {
            if ui.selectable_label(!app.render.eval_frozen, "▶").clicked() {
                app.render.eval_frozen = false;
            }

            if ui.selectable_label(app.render.eval_frozen, "⬛").clicked() {
                app.render.eval_frozen = true;
            }

            ui.separator();

            if ui.selectable_label(app.render.tools_tab == ToolsTab::Hidden, "Hide").clicked() {
                app.render.tools_tab = ToolsTab::Hidden;
            };

            if ui.selectable_label(app.render.tools_tab == ToolsTab::Errors, "Errors").clicked() {
                app.render.tools_tab = ToolsTab::Errors;
            };

            if ui.selectable_label(app.render.tools_tab == ToolsTab::Inspector, "Inspector").clicked() {
                app.render.tools_tab = ToolsTab::Inspector;
            }; 
        });

        if app.render.tools_tab == ToolsTab::Errors {
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Index to view: ");
                ui.text_edit_singleline(&mut app.render.tools_index_tb);

                if app.render.tools_index_invalid {
                    ui.label(" (invalid index)");
                }
            });

            ui.separator();

            let index = match app.render.tools_index_tb.parse::<usize>() {
                Ok(value) => {
                    app.render.tools_index_invalid = if value >= usize::from(400 as u16) { true } else { false };
                    if value >= usize::from(400 as u16) { usize::from(0 as u16) } else { value } 
                },
                Err(_err) => {
                    app.render.tools_index_invalid = true;
                    usize::from(0 as u16)
                        
                }
            };

            let eval_errors: Vec<errors::Error> = app.render.eval_errors[index].clone();

            TableBuilder::new(ui)
                .column(Size::exact(100.0))
                .column(Size::exact(100.0))
                .column(Size::remainder())
                .striped(true)
                .header(16.0, |mut header| {
                    header.col(|ui| {
                        ui.label("Type");
                    });
                    header.col(|ui| {
                        ui.label("Location");
                    });
                    header.col(|ui| {
                        ui.label("Error");
                    });
                })
                .body(|mut body| {
                    for error in eval_errors {
                        body.row(16.0, |mut row| {
                            row.col(|ui| {
                                ui.label("Runtime");
                            });
                            row.col(|ui| {
                                ui.label(error.line_number.to_string() + &String::from(":") + &error.col_number.to_string());
                            });
                            row.col(|ui| {
                                ui.label(error.reason.to_string());
                            });
                        });
                    };
                });
        }

        if app.render.tools_tab == ToolsTab::Inspector {
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Index to view: ");
                ui.text_edit_singleline(&mut app.render.tools_index_tb);

                if app.render.tools_index_invalid {
                    ui.label(" (invalid index)");
                }
            });

            ui.separator();

            let index = match app.render.tools_index_tb.parse::<usize>() {
                Ok(value) => {
                    app.render.tools_index_invalid = if value >= usize::from(400 as u16) { true } else { false };
                    if value >= usize::from(400 as u16) { usize::from(0 as u16) } else { value } 
                },
                Err(_err) => {
                    app.render.tools_index_invalid = true;
                    usize::from(0 as u16)
                        
                }
            };

            let eval_variables: AHashMap<String, f64> = app.render.eval_variables[index].clone();

            TableBuilder::new(ui)
                .column(Size::exact(150.0))
                .column(Size::remainder().at_least(150.0))
                .striped(true)
                .header(16.0, |mut header| {
                    header.col(|ui| {
                        ui.label("Name");
                    });
                    header.col(|ui| {
                        ui.label("Value");
                    });
                })
                .body(|mut body| {
                    let mut sorted: Vec<_> = eval_variables.iter().collect();
                    sorted.sort_by_key(|a| a.0);

                    for variable in sorted {
                        body.row(16.0, |mut row| {
                            row.col(|ui| {
                                ui.label(variable.0);
                            });
                            row.col(|ui| {
                                ui.label(variable.1.to_string());
                            });
                        })

                    }
                });
        }
    });
}
