use eframe::egui;
use egui_commonmark::*;

#[derive(PartialEq, Clone)]
enum DocumentationEntry {
    Group(String, Vec<DocumentationEntry>),
    Page(String, String),
    None,
}

pub struct DocumentationWindow {
    page: DocumentationEntry,
    available_pages: Vec<DocumentationEntry>,
}

impl Default for DocumentationWindow {
    fn default() -> Self {
        let available_pages = vec![
            DocumentationEntry::Page(
                "Introduction".into(),
                include_str!("../../docs/introduction.md").into(),
            ),
            DocumentationEntry::Group(
                "Expressions".into(),
                vec![
                    DocumentationEntry::Page(
                        "Inputs".into(),
                        include_str!("../../docs/expressions/inputs.md").into(),
                    ),
                    DocumentationEntry::Page(
                        "Outputs".into(),
                        include_str!("../../docs/expressions/outputs.md").into(),
                    ),
                    DocumentationEntry::Page(
                        "Functions".into(),
                        include_str!("../../docs/expressions/functions.md").into(),
                    ),
                    DocumentationEntry::Page("Operators".into(), "# Unfinished".into()),
                ],
            ),
            DocumentationEntry::Group(
                "Laser Studio".into(),
                vec![
                    DocumentationEntry::Page("Editor".into(), "# Unfinished".into()),
                    DocumentationEntry::Page("Render".into(), "# Unfinished".into()),
                    DocumentationEntry::Page("Errors".into(), "# Unfinished".into()),
                ],
            ),
        ];

        Self {
            page: DocumentationEntry::None,
            available_pages,
        }
    }
}

impl DocumentationWindow {
    pub fn update(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new("Documentation")
            .open(open)
            .resizable(true)
            .default_width(600.0)
            .default_height(400.0)
            .show(ctx, |ui| {
                let mut side_frame = egui::Frame::default();

                side_frame.inner_margin.right = 50.0;

                egui::SidePanel::left("documentation_left")
                    .frame(side_frame)
                    .show_inside(ui, |ui| {
                        self.build_tree(ui, self.available_pages.clone());
                    });

                egui::CentralPanel::default()
                    .frame(egui::Frame::default())
                    .show_inside(ui, |ui| {
                        match &self.page {
                            DocumentationEntry::Page(_, content) => {
                                egui::ScrollArea::vertical()
                                    .max_height(f32::INFINITY)
                                    .max_width(f32::INFINITY)
                                    .show(ui, |ui| {
                                        let mut cache = CommonMarkCache::default();
                                        CommonMarkViewer::new("viewer")
                                            .show(ui, &mut cache, &content);
                                    });
                            }
                            _ => {
                                ui.heading("Please select a page.");
                            }
                        };
                    });
                ui.allocate_space(ui.available_size());
            });
    }

    fn build_tree(&mut self, ui: &mut egui::Ui, entries: Vec<DocumentationEntry>) {
        for entry in entries {
            match entry {
                DocumentationEntry::None => (),
                DocumentationEntry::Page(ref name, _) => {
                    if ui
                        .selectable_label(self.page == entry.clone(), name)
                        .clicked()
                    {
                        self.page = entry;
                    }
                }
                DocumentationEntry::Group(name, entries) => {
                    ui.collapsing(name, |ui| self.build_tree(ui, entries));
                }
            }
        }
    }
}
