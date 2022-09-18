mod text;
mod graphical;

use eframe::egui;
use egui::menu;

use crate::project;

#[derive(PartialEq)]
enum Workspace {
    Home,
    Graphical,
    Text,
    Render
}

pub struct LaserStudioApp {
    tab: Workspace,
    project: project::Project

}

impl Default for LaserStudioApp {
    fn default() -> Self {
        Self { 
            tab: Workspace::Home,
            project: project::Project::default()
        }
    }
}

impl eframe::App for LaserStudioApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        self.tab = Workspace::Graphical;
                    }
                    if ui.button("Open").clicked() {
                        self.tab = Workspace::Graphical;
                    }
                });

                ui.menu_button("Edit", |_ui| {

                });

                ui.menu_button("Run", |_ui| {
                    
                });
                ui.separator();
                if self.tab == Workspace::Home {
                    ui.label("Home");
                } else {
                    if ui.selectable_label(self.tab == Workspace::Graphical, "Graphical").clicked() {
                       self.tab = Workspace::Graphical;
                    }
                    if ui.selectable_label(self.tab == Workspace::Text, "Text").clicked() {
                       self.tab = Workspace::Text;
                    } 
                    if ui.selectable_label(self.tab == Workspace::Render, "Render").clicked() {
                       self.tab = Workspace::Render; 
                    }
                } 
            }); 
        });

        match self.tab {
            Workspace::Text => text::update_text_workspace(ctx, self),
            Workspace::Graphical => graphical::update_graphical_workspace(ctx, self),
            _ => {
                egui::SidePanel::left("about").resizable(false).min_width(200.0).show(ctx, |ui| {
                    ui.heading("Laser Studio");
                    ui.label("3.0");
                    ui.separator();
                    ui.label("Licensed under the Apache License, version 2.0.");
                    ui.label("© 2020-2022 william341.");
                });
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("New").clicked() {
                            self.tab = Workspace::Graphical;
                        }
                        if ui.button("Open").clicked() {
                            self.tab = Workspace::Graphical;
                        }
                    });
                    ui.heading("Recent Projects");
                });
            }
        }

    }
}
