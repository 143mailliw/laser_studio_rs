use eframe::egui;

use eframe::egui::Widget;

pub struct TextWorkspace {
}

impl Default for TextWorkspace {
    fn default() -> Self {
        Self {
        }
    }
}

pub fn update_text_workspace(ctx: &egui::Context, app: &mut super::LaserStudioApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::TextEdit::multiline(&mut app.project.text_data.content).desired_width(f32::INFINITY).ui(ui);

        })
    });
}
