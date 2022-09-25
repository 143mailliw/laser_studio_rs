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
    let mut frame = egui::Frame::default();

    frame.inner_margin = egui::style::Margin {left: 0.0, right: 0.0, top: 0.0, bottom: 0.0};
    frame.fill = egui::Color32::from_gray(10);

    egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::TextEdit::multiline(&mut app.project.text_data.content).code_editor().desired_width(f32::INFINITY).frame(false).ui(ui);
        })
    });
}
