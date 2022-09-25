use crate::expressions::*;
use chumsky::Parser;
use eframe::egui;
use eframe::egui::plot;
use rayon::prelude::*;
use std::time;
use std::collections::HashMap;

pub struct RenderWorkspace {
    parser_result: Vec<parser::Assignment>,
    parser_errors: Vec<errors::Error>,
    eval_errors: Vec<Vec<errors::Error>>,
    projection_start_time: time::Duration
}

impl Default for RenderWorkspace {
    fn default() -> Self {
        Self {
            parser_result: vec![],
            parser_errors: vec![],
            eval_errors: vec![],
            projection_start_time: time::SystemTime::now()
                .duration_since(time::UNIX_EPOCH)
                .expect("time went backwards")
        }
    }
}

#[derive(Debug)]
struct RenderedPoint {
    x: f64,
    y: f64,
    h: f64,
    s: f64,
    v: f64
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
    let mut base_hash_map: HashMap<String, f64> = HashMap::new(); 
    let time = time::SystemTime::now()
        .duration_since(time::SystemTime::UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs_f64();
    let projection_start_time = workspace.projection_start_time.as_secs_f64();

    base_hash_map.insert("count".into(), 400.0);
    base_hash_map.insert("pi".into(), std::f64::consts::PI);
    base_hash_map.insert("tau".into(), std::f64::consts::TAU);
    base_hash_map.insert("time".into(), time as f64);
    base_hash_map.insert("projectionTime".into(), time - projection_start_time);
    base_hash_map.insert("projectionStartTime".into(), projection_start_time);

    workspace.eval_errors = vec![];

    let points: Vec<(HashMap<String, f64>, Vec<errors::Error>)> = (0..400)
        .into_par_iter()
        .map(|index| {
            let mut hash_map = base_hash_map.clone();

            hash_map.insert("index".into(), index as f64);
            hash_map.insert("x".into(), ((index % 20 - 10) * 10) as f64);
            hash_map.insert("y".into(), ((index / 20 - 9) * 10) as f64);
            hash_map.insert("fraction".into(), index as f64 / 400.0);
        
            let result = eval::run(workspace.parser_result.clone(), text.clone(), &mut hash_map);
            let error = result.1.clone();

            (hash_map.to_owned(), error)
        })
        .collect();

    for tuple in points.clone() {
        if tuple.1.len() > 0 {
            println!("{:?}", tuple.1.clone());
        }
        workspace.eval_errors.push(tuple.1.clone());
    }

    let calculated_points = points
        .par_iter()
        .map(|(variables, _errors)| {
            RenderedPoint {
                x: *variables.get("x'").unwrap_or(&0.0),
                y: *variables.get("y'").unwrap_or(&0.0),
                h: *variables.get("h").unwrap_or(&130.0),
                s: *variables.get("s").unwrap_or(&1.0),
                v: *variables.get("v").unwrap_or(&1.0)
            }
        })
        .collect();

    calculated_points
}

pub fn update_render_workspace(ctx: &egui::Context, app: &mut super::LaserStudioApp) {
    let calculated_points = calculate_points(&mut app.render, app.project.text_data.content.clone());

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
            for point in calculated_points {
                let plot_point =plot::Points::new(vec!([point.x, point.y]))
                    .filled(true)
                    .radius(3.0)
                    .color(egui::color::Hsva::new((point.h % 360.0 / 360.0) as f32, point.s as f32, point.v as f32, 1.0));

                plot_ui.points(plot_point);
            }
        });

        ctx.request_repaint();
    });
}
