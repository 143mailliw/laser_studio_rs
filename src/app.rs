use eframe::egui;
use egui::menu;

pub struct LaserStudioApp {
   tab: u32 
}

impl Default for LaserStudioApp {
    fn default() -> Self {
        Self { 
            tab: 0
        }
    }
}

impl eframe::App for LaserStudioApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |_ui| {
                    
                });

                ui.menu_button("Edit", |_ui| {

                });

                ui.menu_button("Run", |_ui| {
                    
                });
                ui.separator();
                if self.tab == 0 {
                    ui.label("Home");
                } else {
                    if ui.selectable_label(self.tab == 1, "Graphical").clicked() {
                       self.tab = 1;
                    }
                    if ui.selectable_label(self.tab == 2, "Text").clicked() {
                       self.tab = 2;
                    } 
                    if ui.selectable_label(self.tab == 3, "Render").clicked() {
                       self.tab = 3; 
                    }
                } 
            }); 
        });

        match self.tab {
            _ => {
                egui::SidePanel::left("about").resizable(false).min_width(200.0).show(ctx, |ui| {
                    ui.heading("Laser Studio");
                    ui.label("3.0");
                    ui.separator();
                    ui.label("Licensed under the Apache License, version 2.0.");
                    ui.label("© 2020-2022 william341.");
                });
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Recent Projects");
                });
            }
        }

    }
}
