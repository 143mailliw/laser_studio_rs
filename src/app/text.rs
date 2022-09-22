use std::collections::HashMap;
use chumsky::Parser;
use eframe::egui;
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
    }); 
}
