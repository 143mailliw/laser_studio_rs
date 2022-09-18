use eframe::egui;

pub fn update_graphical_workspace(ctx: &egui::Context, app: &mut super::LaserStudioApp) {
    let mut frame = egui::Frame::default();
    frame.inner_margin.top = 4.0;
    frame.inner_margin.left = 4.0;
    frame.inner_margin.right = 4.0;
    frame.inner_margin.bottom = 4.0;
    frame.fill = ctx.style().visuals.window_fill();
    frame.stroke.width = 2.0;
    frame.stroke.color = ctx.style().visuals.window_stroke().color;

    egui::SidePanel::left("tools").resizable(false).min_width(19.0).frame(frame).show(ctx, |ui| {
        if ui.selectable_label(false, "🖌").clicked() {

        }
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.text_edit_multiline(&mut app.project.text_data.content);
    }); 
}
