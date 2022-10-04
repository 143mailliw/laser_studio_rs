mod text;
mod graphical;
mod render;

use eframe::egui;
use egui::menu;
use tracing::info;
use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;
use std::io::BufReader;
use rfd::FileDialog;
use std::thread;
use std::sync::mpsc;
use crate::project;

#[derive(PartialEq)]
enum Workspace {
    Home,
    Graphical,
    Text,
    Render
}

enum FileDialogSelection {
    Open(PathBuf),
    Save(PathBuf)
}

pub struct LaserStudioApp {
    tab: Workspace,
    project: project::Project,
    graphical: graphical::GraphicalWorkspaceState,
    pub text: text::TextWorkspace,
    render: render::RenderWorkspace,
    project_rx: mpsc::Receiver<FileDialogSelection>,
    project_tx: mpsc::Sender<FileDialogSelection>
}

impl Default for LaserStudioApp {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();

        Self { 
            tab: Workspace::Home,
            project: project::Project::default(),
            graphical: graphical::GraphicalWorkspaceState::default(),
            text: text::TextWorkspace::default(),
            render: render::RenderWorkspace::default(),
            project_rx: rx,
            project_tx: tx
        }
    }
}

impl eframe::App for LaserStudioApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // check for any new project updates before rendering anything
        self.check_for_selection();

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
                        self.open_dialog();
                        ui.close_menu();
                    }

                    if self.tab != Workspace::Home {
                        ui.separator();
                        if ui.button("Save").clicked() {
                            self.save_dialog();
                            ui.close_menu();
                        }
                        if ui.button("Save As").clicked() {
                            self.save_dialog();
                            ui.close_menu();
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

                        if ui.button("Run").clicked() {
                            self.render.eval_frozen = false;
                            self.tab = Workspace::Render;
                            ui.close_menu();
                        }
                        if ui.button("Stop").clicked() {
                            self.render.eval_frozen = true;
                            ui.close_menu();
                        }
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
                            self.open_dialog();
                        }
                    });
                    ui.heading("Recent Projects");
                });
            }
        }

    }
}

impl LaserStudioApp {
    // a bunch of stuff to handle opening & saving Projects
    fn check_for_selection(&mut self) {
        match self.project_rx.try_recv() {
            Ok(value) => {
                match value {
                    FileDialogSelection::Open(value) => {
                        match LaserStudioApp::open_project(value) {
                            Ok(value) => { 
                                self.project = value;
                                self.tab = Workspace::Graphical;
                            },
                            Err(_err) => { } // TODO: show error dialog
                        };
                    },
                    FileDialogSelection::Save(value) => {
                        match LaserStudioApp::save_project(value, self.project.clone()) {
                            Ok(_) => (),
                            Err(_err) => { } // TODO: show error dialog
                        };
                    }
                };
            },
            Err(_) => () // don't do anything, there's no data
        }
    }

    // dialogs must be done in another thread because otherwise the main thread gets blocked and
    // the aplication hangs
    fn open_dialog(&mut self) {
        let tx = self.project_tx.clone();

        thread::spawn(move || {
            let file_result = FileDialog::new()
                .add_filter("Laser Studio Project",  &["lsp"])
                .set_title("Open File")
                .pick_file();

            match file_result {
                Some(path) => {
                    match tx.send(FileDialogSelection::Open(path)) {
                        Ok(_) => (),
                        Err(_) => panic!("file thread died after application exited")
                    }
                },
                None => ()
            }
        });
    }

    fn save_dialog(&mut self) {
        let tx = self.project_tx.clone();

        thread::spawn(move || {
            let file_result = FileDialog::new()
                .add_filter("Laser Studio Project",  &["lsp"])
                .set_title("Open File")
                .set_file_name("Untitled.lsp")
                .save_file();

            match file_result {
                Some(path) => {
                    match tx.send(FileDialogSelection::Save(path)) {
                        Ok(_) => (),
                        Err(_) => panic!("file thread died after application exited")
                    }
                },
                None => ()
            }
        });
    }

    fn open_project(path: PathBuf) -> std::result::Result<project::Project, String> {
        let file = match File::open(path) {
            Ok(value) => value,
            Err(_err) => return Err("An error occured while opening the file for reading.".into())
        };

        let reader = BufReader::new(file);

        match serde_json::from_reader(reader) {
            Ok(value) => Ok(value),
            Err(_err) => return Err("An deserialization error occured.".into())
        }
    }

    fn save_project(path: PathBuf, project: project::Project) -> std::result::Result<(), String> {
        let serialized = match serde_json::to_string(&project) {
            Ok(value) => value,
            Err(_error) => return Err("A serialization error occured.".into())
        };

        let mut file = match File::create(path) {
            Ok(value) => value,
            Err(_error) => return Err("Failed to create a new file.".into())
        };

        match file.write_all(serialized.as_bytes()) {
            Ok(_value) => Ok(()),
            Err(_error) => return Err("Failed to write to the file.".into())
        }
    }
}
