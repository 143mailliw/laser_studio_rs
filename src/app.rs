mod text;
mod graphical;
mod render;

use eframe::egui;
use egui::menu;
use tracing::info;
use rfd::{FileDialog, AsyncFileDialog};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

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
    graphical: graphical::GraphicalWorkspaceState,
    pub text: text::TextWorkspace,
    render: render::RenderWorkspace,
}

impl Default for LaserStudioApp {
    fn default() -> Self {
        Self { 
            tab: Workspace::Home,
            project: project::Project::default(),
            graphical: graphical::GraphicalWorkspaceState::default(),
            text: text::TextWorkspace::default(),
            render: render::RenderWorkspace::default()
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
                ui.spacing_mut().button_padding = egui::vec2(8.0, 6.0);
                ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
                ui.visuals_mut().widgets.active.rounding = egui::Rounding::none();
                ui.visuals_mut().widgets.hovered.rounding = egui::Rounding::none();
                ui.visuals_mut().widgets.inactive.rounding = egui::Rounding::none();

                ui.menu_button("File", |ui| {
                    ui.spacing_mut().button_padding = egui::vec2(8.0, 4.0);
                    ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
                    ui.visuals_mut().widgets.active.rounding = egui::Rounding::none();
                    ui.visuals_mut().widgets.hovered.rounding = egui::Rounding::none();
                    ui.visuals_mut().widgets.inactive.rounding = egui::Rounding::none();

                    if ui.button("New").clicked() {
                        self.project = project::Project::default();
                        self.tab = Workspace::Graphical;
                        ui.close_menu();
                    }
                    if ui.button("Open").clicked() {
                        
                        // match open_project() {
                        //     Ok(project) => { 
                        //         self.project = project;
                        //         ui.close_menu();
                        //     },
                        //     Err(_err) => {
                        //         // TODO: show error dialog
                        //     }
                        // }
                        self.tab = Workspace::Graphical;
                    }

                    if self.tab != Workspace::Home {
                        ui.separator();
                        if ui.button("Save").clicked() {
                            // match save_project(self.project.clone()) {
                            //     Ok(_value) => { ui.close_menu(); },
                            //     Err(_err) => {
                            //         // TODO: show error dialog
                            //     }
                            // }                       
                        }
                        if ui.button("Save As").clicked() {
                            // match save_project(self.project.clone()) {
                            //     Ok(_value) => { ui.close_menu(); },
                            //     Err(_err) => {
                            //         // TODO: show error dialog
                            //     }
                            // }
                        }
                        if ui.button("Export").clicked() {}
                    }

                    ui.separator();
                    if ui.button("Exit").clicked() {
                        info!("File -> Exit pressed, exiting...");
                        std::process::exit(0);
                    }
                });

                if self.tab != Workspace::Home {
                    ui.menu_button("Edit", |ui| {
                        ui.spacing_mut().button_padding = egui::vec2(8.0, 4.0);
                        ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
                        ui.visuals_mut().widgets.active.rounding = egui::Rounding::none();
                        ui.visuals_mut().widgets.hovered.rounding = egui::Rounding::none();
                        ui.visuals_mut().widgets.inactive.rounding = egui::Rounding::none();

                        if ui.button("Undo").clicked() {}
                        if ui.button("Redo").clicked() {}
                        if ui.button("History").clicked() {}
                        ui.separator();
                        if ui.button("Find/Replace").clicked() {}
                        ui.separator();
                        if ui.button("Preferences").clicked() {}
                    });
                }

                if self.tab != Workspace::Home {
                    ui.menu_button("Run", |ui| {
                        ui.spacing_mut().button_padding = egui::vec2(8.0, 4.0);
                        ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
                        ui.visuals_mut().widgets.active.rounding = egui::Rounding::none();
                        ui.visuals_mut().widgets.hovered.rounding = egui::Rounding::none();
                        ui.visuals_mut().widgets.inactive.rounding = egui::Rounding::none();

                        if ui.button("Run").clicked() {}
                        if ui.button("Debug").clicked() {}
                        ui.separator();
                        if ui.button("Pause").clicked() {}
                        if ui.button("Step Forwards").clicked() {}
                        if ui.button("Step Backwards").clicked() {}
                        ui.separator();
                        if ui.button("Stop").clicked() {}
                    });
                }

                ui.spacing_mut().item_spacing.x = 3.0;

                ui.separator();
    
                ui.spacing_mut().button_padding.x = 5.0;
                ui.spacing_mut().button_padding.y = 3.0;
                ui.spacing_mut().item_spacing.x = 3.0;
                ui.spacing_mut().item_spacing.y = 3.0;

                let tab_radius = egui::Rounding {ne: 2.0, se: 2.0, nw: 2.0, sw: 2.0};

                ui.visuals_mut().widgets.active.rounding = tab_radius;
                ui.visuals_mut().widgets.hovered.rounding = tab_radius;
                ui.visuals_mut().widgets.inactive.rounding = tab_radius;

                if self.tab == Workspace::Home {
                    ui.spacing_mut().item_spacing.y = 6.0;
                    ui.label("Home");
                } else {
                    if ui.selectable_label(self.tab == Workspace::Graphical, "Graphical").clicked() {
                        self.tab = Workspace::Graphical;
                    }
                    if ui.selectable_label(self.tab == Workspace::Text, "Text").clicked() {
                        self.tab = Workspace::Text;
                    } 
                    if ui.selectable_label(self.tab == Workspace::Render, "Render").clicked() {
                        render::on_switch_render(self);
                        self.tab = Workspace::Render; 
                    }
                } 
            }); 
        });

        match self.tab {
            Workspace::Text => text::update_text_workspace(ctx, self),
            Workspace::Graphical => graphical::update_graphical_workspace(ctx, self),
            Workspace::Render => render::update_render_workspace(ctx, self),
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
                            self.project = project::Project::default();
                            self.tab = Workspace::Graphical;
                        }
                        if ui.button("Open").clicked() {
                            // match open_project() {
                            //     Ok(project) => { self.project = project; },
                            //     Err(_err) => {
                            //         // TODO: show error dialog
                            //     }
                            // }
                            self.tab = Workspace::Graphical;
                        }
                    });
                    ui.heading("Recent Projects");
                });
            }
        }

    }
}

async fn open_project() -> std::result::Result<project::Project, String> {
    let path_option = AsyncFileDialog::new()
        .set_title("Open Project")
        .add_filter("Laser Studio Project", &[".lsp"])
        .pick_file()
        .await;

    let path = match path_option {
        Some(value) => value,
        None => return Err("No file was selected.".into())
    };

    let file = match File::open(path.path()) {
        Ok(value) => value,
        Err(_err) => return Err("An error occured while opening the file for reading.".into())
    };

    let reader = BufReader::new(file);

    match serde_json::from_reader(reader) {
        Ok(value) => Ok(value),
        Err(_err) => return Err("An deserialization error occured.".into())
    }
}

async fn save_project(project: project::Project) -> std::result::Result<(), String> {
    let path_option = AsyncFileDialog::new()
        .set_title("Save Project")
        .add_filter("Laser Studio Project", &[".lsp"])
        .save_file()
        .await;

    let path = match path_option {
        Some(value) => value,
        None => return Err("No file was selected.".into())
    };

    let serialized = match serde_json::to_string(&project) {
        Ok(value) => value,
        Err(_error) => return Err("A serialization error occured.".into())
    };

    let mut file = match File::create(path.path()) {
        Ok(value) => value,
        Err(_error) => return Err("Failed to create a new file.".into())
    };

    match file.write_all(serialized.as_bytes()) {
        Ok(_value) => Ok(()),
        Err(_error) => return Err("Failed to write to the file.".into())
    }
}
