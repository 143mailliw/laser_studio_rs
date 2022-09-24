use std::collections::HashMap;
use chumsky::Parser;
use eframe::egui;
use egui_extras::{TableBuilder, Size};
use crate::expressions::parser;
use crate::expressions::eval;
use crate::expressions::errors;

pub struct TextWorkspace {
    parser_result: Vec<parser::Assignment>,
    interpreter_variables: HashMap<String, f64>,
    errors: Vec<errors::Error>
    
}

impl Default for TextWorkspace {
    fn default() -> Self {
        Self {
            parser_result: vec!(),
            interpreter_variables: HashMap::new(),
            errors: vec!()
        }
    }
}

pub fn update_text_workspace(ctx: &egui::Context, app: &mut super::LaserStudioApp) {
    egui::CentralPanel::default().show(ctx, |ui| {

        ui.text_edit_multiline(&mut app.project.text_data.content);

        if ui.button("Run").clicked() {
            app.text.parser_result = match parser::parser().parse(app.project.text_data.content.clone()) {
                Ok(value) => value,
                Err(error) => panic!("Error parsing: {:?}", error)
            };

            let evaluation_result = eval::run(app.text.parser_result.to_vec());

            app.text.interpreter_variables = evaluation_result.0;
            app.text.errors = evaluation_result.1;
        }

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
                                ui.label(error.line_number.to_string() + &String::from(":") + &error.line_number.to_string());
                            });
                            row.col(|ui| {
                                ui.label(error.reason.to_string());
                            });
                        });
                    };
                });
        }) 
    }); 
}
