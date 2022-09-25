use std::collections::HashMap;
use std::time::Instant;
use chumsky::Parser;
use rayon::prelude::*;
use eframe::egui;
use eframe::egui::Widget;
use egui_extras::{TableBuilder, Size};
use crate::expressions::parser;
use crate::expressions::eval;
use crate::expressions::errors;

pub struct TextWorkspace {
    parser_result: Vec<parser::Assignment>,
    interpreter_variables: HashMap<String, f64>,
    errors: Vec<errors::Error>,
    time: f64,
    large_run_time: f64
}

impl Default for TextWorkspace {
    fn default() -> Self {
        Self {
            parser_result: vec!(),
            interpreter_variables: HashMap::new(),
            errors: vec!(),
            time: 0.0,
            large_run_time: 0.0
        }
    }
}

pub fn update_text_workspace(ctx: &egui::Context, app: &mut super::LaserStudioApp) {
    egui::CentralPanel::default().show(ctx, |ui| {

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::TextEdit::multiline(&mut app.project.text_data.content).desired_width(f32::INFINITY).ui(ui);

            if ui.button("Run").clicked() {
                app.text.parser_result = match parser::parser().parse(app.project.text_data.content.clone()) {
                    Ok(value) => value,
                    Err(error) => panic!("Error parsing: {:?}", error)
                };

                println!("{:?}", app.text.parser_result);

                let vec = app.text.parser_result.to_vec();

                let now = Instant::now();

                app.text.interpreter_variables = HashMap::new();
                let evaluation_result = eval::run(vec, app.project.text_data.content.clone(), &mut app.text.interpreter_variables);

                let elapsed = now.elapsed();

                app.text.time = elapsed.as_secs_f64();
                app.text.errors = evaluation_result.1;
            }

            if ui.button("Run 400 times").clicked() {
                app.text.parser_result = match parser::parser().parse(app.project.text_data.content.clone()) {
                    Ok(value) => value,
                    Err(error) => panic!("Error parsing: {:?}", error)
                };

                let vec = app.text.parser_result.to_vec();

                let now = Instant::now();

                app.project.graphical_data.points.par_iter().for_each(|_x| {eval::run(vec.clone(), app.project.text_data.content.clone(), &mut HashMap::new());});

                let elapsed = now.elapsed();

                app.text.large_run_time = elapsed.as_secs_f64();
            }

            // perf info
            ui.label("Performance:");
            ui.label(format!("Single Run Time: {}", app.text.time));
            ui.label(format!("400 * Run Time: {}", app.text.large_run_time));
            ui.label(format!("Estimated FPS: {}", 1.0 / app.text.large_run_time));

            // variable table
            ui.label("Variables:");
            TableBuilder::new(ui)
                .column(Size::exact(150.0))
                .column(Size::remainder().at_least(150.0))
                .striped(true)
                .header(15.0, |mut header| {
                    header.col(|ui| {
                        ui.label("Name");
                    });
                    header.col(|ui| {
                        ui.label("Value");
                    });
                })
                .body(|mut body| {
                    for variable in app.text.interpreter_variables.iter() {
                        body.row(15.0, |mut row| {
                            row.col(|ui| {
                                ui.label(variable.0);
                            });
                            row.col(|ui| {
                                ui.label(variable.1.to_string());
                            });
                        })

                    }
                });

            // error table
            ui.label("Errors:");
            ui.collapsing("Errors", |ui| {
                TableBuilder::new(ui)
                    .column(Size::exact(100.0))
                    .column(Size::exact(100.0))
                    .column(Size::remainder().at_least(150.0))
                    .striped(true)
                    .header(15.0, |mut header| {
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
                        for error in app.text.errors.iter() {
                            body.row(15.0, |mut row| {
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
            }) 
        })
    });
}
