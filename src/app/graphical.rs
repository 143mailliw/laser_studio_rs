use eframe::egui;

#[derive(Clone, Copy, PartialEq)]
enum Tools {
    Pencil,
    Eraser,
    PaintBucket,
    ColorPicker,
}

pub struct GraphicalWorkspaceState {
    selected_color: [f32; 3],
    selected_tool: Tools,
}

impl Default for GraphicalWorkspaceState {
    fn default() -> Self {
        Self {
            selected_color: [255.0, 255.0, 255.0],
            selected_tool: Tools::Pencil,
        }
    }
}

pub fn update_graphical_workspace(ctx: &egui::Context, app: &mut super::LaserStudioApp) {
    let mut frame = egui::Frame::default();

    frame.inner_margin.top = 8.0;
    frame.inner_margin.left = 8.0;
    frame.inner_margin.right = 8.0;
    frame.inner_margin.bottom = 6.0;
    frame.fill = ctx.style().visuals.window_fill();

    egui::CentralPanel::default().show(ctx, |ui| {
        egui::containers::Area::new("Tools")
            .fixed_pos(egui::Pos2::new(4.0, 35.0))
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing.y = 6.0;

                ui.group(|ui| {
                    let curr_tool = app.graphical.selected_tool;

                    ui.spacing_mut().button_padding.x = 3.0;
                    ui.spacing_mut().button_padding.y = 3.0;

                    let mut pencil = ui.selectable_label(
                        curr_tool == Tools::Pencil,
                        egui::RichText::new("✏")
                            .text_style(egui::TextStyle::Monospace)
                            .size(15.0),
                    );
                    let mut eraser = ui.selectable_label(
                        curr_tool == Tools::Eraser,
                        egui::RichText::new("❌")
                            .text_style(egui::TextStyle::Monospace)
                            .size(15.0),
                    );
                    let mut filler = ui.selectable_label(
                        curr_tool == Tools::PaintBucket,
                        egui::RichText::new("🌋")
                            .text_style(egui::TextStyle::Monospace)
                            .size(15.0),
                    );
                    let mut picker = ui.selectable_label(
                        curr_tool == Tools::ColorPicker,
                        egui::RichText::new("🌋")
                            .text_style(egui::TextStyle::Monospace)
                            .size(15.0),
                    );

                    pencil = pencil.on_hover_text("Pencil");
                    eraser = eraser.on_hover_text("Eraser");
                    filler = filler.on_hover_text("Paint Bucket");
                    picker = picker.on_hover_text("Color Picker");

                    if pencil.clicked() {
                        app.graphical.selected_tool = Tools::Pencil;
                    }

                    if eraser.clicked() {
                        app.graphical.selected_tool = Tools::Eraser;
                    }

                    if filler.clicked() {
                        app.graphical.selected_tool = Tools::PaintBucket;
                    }

                    if picker.clicked() {
                        app.graphical.selected_tool = Tools::ColorPicker;
                    }
                });

                ui.group(|ui| {
                    ui.spacing_mut().interact_size = egui::vec2(20.0, 20.0);
                    ui.color_edit_button_rgb(&mut app.graphical.selected_color);
                })
            });
    });
}
