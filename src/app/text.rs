use eframe::egui;
use eframe::egui::text::{CCursor, CCursorRange};

pub struct TextWorkspace {
    cursor: egui::widgets::text_edit::CCursorRange,
    rows: Option<Vec<eframe::epaint::text::Row>>,
}

impl Default for TextWorkspace {
    fn default() -> Self {
        Self {
            cursor: CCursorRange {
                primary: CCursor {
                    index: 0,
                    prefer_next_row: true,
                },
                secondary: CCursor {
                    index: 0,
                    prefer_next_row: true,
                },
            },
            rows: None,
        }
    }
}

impl TextWorkspace {
    pub fn get_position_from_range(range: CCursorRange, string: String) -> (u64, u64) {
        // we don't need the whole span info for the error message,
        // but we keep it the whole time for things like error highlighting
        let mut remaining_chars: u64 = range
        .primary
        .index
        .try_into()
        .expect("text file is longer than 18446744073709551615 characters");
        let mut line = 0;

        for cur_line in string.lines() {
            line += 1;

            let len: u64 = cur_line.len().try_into().expect(
                    "if you encounter this error then something is *extremely* wrong with your computer",
            );

            if remaining_chars > len {
                remaining_chars -= len + 1;
            } else {
                break;
            }
        }

        if remaining_chars == 0 {
            line += 1;
        }

        return (line, remaining_chars);
    }

    pub fn update_text_workspace(&mut self, ctx: &egui::Context, project: &mut crate::project::Project) {
        let mut frame = egui::Frame::default();

        frame.inner_margin = egui::style::Margin {
            left: 0.0,
            right: 0.0,
            top: 0.0,
            bottom: 0.0,
        };
        frame.fill = egui::Color32::from_gray(10);

        let mut status_frame = egui::Frame::default();

        status_frame.inner_margin = egui::style::Margin {
            left: 7.0,
            right: 7.0,
            top: 5.0,
            bottom: 0.0,
        };
        status_frame.fill = ctx.style().visuals.window_fill();
        status_frame.stroke = ctx.style().visuals.window_stroke();

        egui::TopBottomPanel::bottom("text_status")
            .frame(status_frame)
            .show(ctx, |ui| {
                let pos =
                    TextWorkspace::get_position_from_range(self.cursor, project.text_data.content.clone());

                ui.vertical(|ui| {
                    ui.label(format!("Line {}, col {}", pos.0, pos.1));
                })
            });

        egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
            let size_y = ui.available_size().y;

            egui::ScrollArea::vertical()
                .max_height(size_y)
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
                    ui.horizontal(|ui| {
                        let mut num_frame = egui::Frame::default();
                        num_frame.inner_margin = egui::style::Margin {
                            left: 2.0,
                            right: 0.0,
                            top: 2.0,
                            bottom: 0.0,
                        };
                        num_frame.fill = ctx.style().visuals.window_fill();

                        num_frame.show(ui, |ui| {
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
                                ui.set_min_width(40.0);
                                ui.set_min_height(size_y);

                                let rows = self.rows.as_ref();

                                match rows {
                                    Some(rows) => {
                                        let mut cur_num = 1;
                                        let mut last_had_new_line = true;

                                        for row in rows {
                                            if last_had_new_line {
                                                ui.add(egui::Label::new(
                                                    egui::RichText::new(cur_num.to_string())
                                                        .size(14.0)
                                                        .text_style(egui::TextStyle::Monospace),
                                                ));
                                            } else {
                                                ui.add(egui::Label::new(
                                                    egui::RichText::new(" ")
                                                        .size(14.0)
                                                        .text_style(egui::TextStyle::Monospace),
                                                ));
                                            }

                                            if row.ends_with_newline {
                                                cur_num += 1;
                                                last_had_new_line = true;
                                            } else {
                                                last_had_new_line = false;
                                            }
                                        }
                                    }
                                    None => (),
                                };
                            })
                        });

                        ui.vertical(|ui| {
                            ui.style_mut().wrap = Some(false);

                            let response =
                                egui::TextEdit::multiline(&mut project.text_data.content)
                                    .code_editor()
                                    .frame(false)
                                    .desired_width(f32::INFINITY)
                                    .show(ui);
                            match response.state.ccursor_range() {
                                Some(value) => {
                                    self.cursor = value;
                                }
                                None => (),
                            }
                            self.rows = Some(response.galley.rows.clone());
                        })
                    })
                })
        });
    }
}