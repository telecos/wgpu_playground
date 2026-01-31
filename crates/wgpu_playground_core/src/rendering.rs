use crate::examples::{get_all_examples, Example, ExampleCategory};

pub struct RenderingPanel {
    examples: Vec<Example>,
    selected_example: Option<usize>,
    show_source_code: bool,
    category_filter: Option<ExampleCategory>,
}

impl Default for RenderingPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderingPanel {
    pub fn new() -> Self {
        Self {
            examples: get_all_examples(),
            selected_example: None,
            show_source_code: false,
            category_filter: None,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("🎨 Example Gallery");
            ui.separator();
            ui.label("Browse and explore WebGPU examples with descriptions and source code.");
            ui.add_space(10.0);

            // Category filter
            ui.horizontal(|ui| {
                ui.label("Filter by category:");
                if ui
                    .selectable_label(self.category_filter.is_none(), "All")
                    .clicked()
                {
                    self.category_filter = None;
                }
                if ui
                    .selectable_label(
                        self.category_filter == Some(ExampleCategory::Rendering),
                        "Rendering",
                    )
                    .clicked()
                {
                    self.category_filter = Some(ExampleCategory::Rendering);
                }
                if ui
                    .selectable_label(
                        self.category_filter == Some(ExampleCategory::Compute),
                        "Compute",
                    )
                    .clicked()
                {
                    self.category_filter = Some(ExampleCategory::Compute);
                }
            });

            ui.add_space(10.0);
            ui.separator();

            // Example list
            let filtered_examples: Vec<(usize, &Example)> = self
                .examples
                .iter()
                .enumerate()
                .filter(|(_, ex)| {
                    self.category_filter.is_none()
                        || self.category_filter.as_ref() == Some(&ex.category)
                })
                .collect();

            if filtered_examples.is_empty() {
                ui.label("No examples found for this category.");
            } else {
                ui.label(format!("Found {} example(s):", filtered_examples.len()));
                ui.add_space(10.0);

                for (idx, example) in filtered_examples {
                    ui.group(|ui| {
                        let is_selected = self.selected_example == Some(idx);

                        // Example header
                        ui.horizontal(|ui| {
                            let category_icon = match example.category {
                                ExampleCategory::Rendering => "🎨",
                                ExampleCategory::Compute => "🧮",
                            };
                            let category_text = match example.category {
                                ExampleCategory::Rendering => "Rendering",
                                ExampleCategory::Compute => "Compute",
                            };

                            if ui
                                .selectable_label(is_selected, format!("{} {}", category_icon, example.name))
                                .clicked()
                            {
                                self.selected_example = Some(idx);
                                self.show_source_code = false;
                            }

                            ui.label(format!("({})", category_text));
                        });

                        // Show details if selected
                        if is_selected {
                            ui.add_space(5.0);
                            ui.separator();

                            // Description
                            ui.label(egui::RichText::new("Description:").strong());
                            ui.label(example.description);

                            ui.add_space(10.0);

                            // Toggle source code button
                            if ui
                                .button(if self.show_source_code {
                                    "Hide Source Code"
                                } else {
                                    "Show Source Code"
                                })
                                .clicked()
                            {
                                self.show_source_code = !self.show_source_code;
                            }

                            // Source code display
                            if self.show_source_code {
                                ui.add_space(5.0);
                                ui.separator();
                                ui.label(egui::RichText::new("Source Code:").strong());

                                // Display source code in a monospace, scrollable area
                                egui::ScrollArea::vertical()
                                    .max_height(400.0)
                                    .show(ui, |ui| {
                                        let mut source_code = example.source_code.to_string();
                                        ui.add(
                                            egui::TextEdit::multiline(&mut source_code)
                                                .code_editor()
                                                .desired_width(f32::INFINITY)
                                                .interactive(false),
                                        );
                                    });
                            }

                            ui.add_space(5.0);

                            // Action buttons
                            ui.horizontal(|ui| {
                                if ui.button("📋 Copy Source Code").clicked() {
                                    ui.output_mut(|o| o.copied_text = example.source_code.to_string());
                                }

                                ui.label(egui::RichText::new("ℹ️ Click 'Copy Source Code' to copy the shader code to clipboard").weak());
                            });
                        }
                    });

                    ui.add_space(10.0);
                }
            }

            ui.add_space(20.0);
            ui.separator();
            ui.colored_label(
                egui::Color32::from_rgb(100, 150, 255),
                "💡 Tip: Select an example to view its description and source code",
            );
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rendering_panel_new() {
        let panel = RenderingPanel::new();
        assert_eq!(panel.examples.len(), 4);
        assert_eq!(panel.selected_example, None);
        assert!(!panel.show_source_code);
        assert_eq!(panel.category_filter, None);
    }

    #[test]
    fn test_rendering_panel_default() {
        let panel = RenderingPanel::default();
        assert_eq!(panel.examples.len(), 4);
    }

    #[test]
    fn test_rendering_panel_has_all_examples() {
        let panel = RenderingPanel::new();
        let example_names: Vec<&str> = panel.examples.iter().map(|e| e.name).collect();

        assert!(example_names.contains(&"Basic Triangle"));
        assert!(example_names.contains(&"Rotating Cube"));
        assert!(example_names.contains(&"Texture Mapping"));
        assert!(example_names.contains(&"Compute Shader"));
    }
}
