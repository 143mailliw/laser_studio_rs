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
    project: project::Project,
    graphical: graphical::GraphicalWorkspaceState
}

impl Default for LaserStudioApp {
    fn default() -> Self {
        Self { 
            tab: Workspace::Home,
            project: project::Project::default(),
            graphical: graphical::GraphicalWorkspaceState::default()
        }
    }
}

impl eframe::App for LaserStudioApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut frame = egui::Frame::default();

        frame.inner_margin.top = 2.0;
        frame.inner_margin.left = 2.0;
        frame.inner_margin.right = 2.0;
        frame.inner_margin.bottom = 2.0;
        frame.fill = ctx.style().visuals.window_fill();
        frame.stroke = ctx.style().visuals.window_stroke();

        egui::TopBottomPanel::top("menu").frame(frame).show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.style_mut().spacing.button_padding.x = 8.0;
                ui.style_mut().spacing.button_padding.y = 6.0;
                ui.style_mut().spacing.item_spacing.x = 0.0;
                ui.style_mut().spacing.item_spacing.y = 0.0;
                ui.style_mut().visuals.widgets.active.rounding = egui::Rounding::none();
                ui.style_mut().visuals.widgets.hovered.rounding = egui::Rounding::none();
                ui.style_mut().visuals.widgets.inactive.rounding = egui::Rounding::none();

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

                ui.style_mut().spacing.item_spacing.x = 3.0;

                ui.separator();
    
                ui.style_mut().spacing.button_padding.x = 5.0;
                ui.style_mut().spacing.button_padding.y = 3.0;
                ui.style_mut().spacing.item_spacing.x = 3.0;
                ui.style_mut().spacing.item_spacing.y = 3.0;

                let tab_radius = egui::Rounding {ne: 2.0, se: 2.0, nw: 2.0, sw: 2.0};

                ui.style_mut().visuals.widgets.active.rounding = tab_radius;
                ui.style_mut().visuals.widgets.hovered.rounding = tab_radius;
                ui.style_mut().visuals.widgets.inactive.rounding = tab_radius;

                if self.tab == Workspace::Home {
                    ui.style_mut().spacing.item_spacing.y = 6.0;
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
