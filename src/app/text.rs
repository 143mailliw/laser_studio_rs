use eframe::egui;

pub fn update_text_workspace(ctx: &egui::Context, app: &mut super::LaserStudioApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.text_edit_multiline(&mut app.project.text_data.content);
    }); 
}
