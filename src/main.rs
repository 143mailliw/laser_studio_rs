#![forbid(unsafe_code)]

mod app;
pub mod project;
pub mod expressions;

use app::LaserStudioApp;

use tracing::info;
use tracing_subscriber;

fn main() {
    tracing_subscriber::fmt::init(); 
    info!("Starting LS");

    // set up EGUI
    let options = eframe::NativeOptions::default();

    info!("Creating eframe");

    eframe::run_native(
        "Laser Studio",
        options,
        Box::new(|_cc| Box::new(LaserStudioApp::default()))
    );

    info!("Frame closed, exiting...");
}
