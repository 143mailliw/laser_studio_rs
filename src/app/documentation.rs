use eframe::egui;

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
            DocumentationEntry::Page("Home".into(), "# Unfinished".into()),
            DocumentationEntry::Group(
                "Expressions".into(),
                vec![
                    DocumentationEntry::Page(
                        "Inputs".into(),
                        include_str!("../../docs/expressions/inputs.md").into(),
                    ),
                    DocumentationEntry::Page("Outputs".into(), "# Unfinished".into()),
                    DocumentationEntry::Page("Functions".into(), "# Unfinished".into()),
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
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        self.build_tree(ui, self.available_pages.clone());
                    })
                })
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
