use crate::tooltip::TooltipExt;

/// UI panel for configuring and executing draw commands
pub struct DrawCommandPanel {
    // Draw type selection
    draw_type: DrawType,

    // Non-indexed drawing parameters
    vertex_count_input: String,
    first_vertex_input: String,
    instance_count_input: String,
    first_instance_input: String,

    // Indexed drawing parameters
    index_count_input: String,
    base_vertex_input: String,
    first_index_input: String,

    // Indirect drawing parameters
    indirect_offset_input: String,

    // Validation and messaging
    validation_error: Option<String>,
    success_message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DrawType {
    NonIndexed,
    Indexed,
    Indirect,
    IndexedIndirect,
}

impl Default for DrawCommandPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl DrawCommandPanel {
    /// Create a new draw command panel with default values
    pub fn new() -> Self {
        Self {
            draw_type: DrawType::NonIndexed,
            vertex_count_input: "3".to_string(),
            first_vertex_input: "0".to_string(),
            instance_count_input: "1".to_string(),
            first_instance_input: "0".to_string(),
            index_count_input: "6".to_string(),
            base_vertex_input: "0".to_string(),
            first_index_input: "0".to_string(),
            indirect_offset_input: "0".to_string(),
            validation_error: None,
            success_message: None,
        }
    }

    /// Validate the current parameters
    fn validate(&mut self) -> bool {
        self.validation_error = None;
        self.success_message = None;

        match self.draw_type {
            DrawType::NonIndexed => {
                // Validate vertex count
                if self.vertex_count_input.parse::<u32>().is_err() {
                    self.validation_error = Some("Vertex count must be a valid number".to_string());
                    return false;
                }

                // Validate first vertex
                if self.first_vertex_input.parse::<u32>().is_err() {
                    self.validation_error = Some("First vertex must be a valid number".to_string());
                    return false;
                }

                // Validate instance count
                if self.instance_count_input.parse::<u32>().is_err() {
                    self.validation_error =
                        Some("Instance count must be a valid number".to_string());
                    return false;
                }

                // Validate first instance
                if self.first_instance_input.parse::<u32>().is_err() {
                    self.validation_error =
                        Some("First instance must be a valid number".to_string());
                    return false;
                }

                let vertex_count = self.vertex_count_input.parse::<u32>().unwrap();
                if vertex_count == 0 {
                    self.validation_error = Some("Vertex count must be greater than 0".to_string());
                    return false;
                }

                let instance_count = self.instance_count_input.parse::<u32>().unwrap();
                if instance_count == 0 {
                    self.validation_error =
                        Some("Instance count must be greater than 0".to_string());
                    return false;
                }
            }
            DrawType::Indexed => {
                // Validate index count
                if self.index_count_input.parse::<u32>().is_err() {
                    self.validation_error = Some("Index count must be a valid number".to_string());
                    return false;
                }

                // Validate base vertex
                if self.base_vertex_input.parse::<i32>().is_err() {
                    self.validation_error = Some("Base vertex must be a valid integer".to_string());
                    return false;
                }

                // Validate first index
                if self.first_index_input.parse::<u32>().is_err() {
                    self.validation_error = Some("First index must be a valid number".to_string());
                    return false;
                }

                // Validate instance count
                if self.instance_count_input.parse::<u32>().is_err() {
                    self.validation_error =
                        Some("Instance count must be a valid number".to_string());
                    return false;
                }

                // Validate first instance
                if self.first_instance_input.parse::<u32>().is_err() {
                    self.validation_error =
                        Some("First instance must be a valid number".to_string());
                    return false;
                }

                let index_count = self.index_count_input.parse::<u32>().unwrap();
                if index_count == 0 {
                    self.validation_error = Some("Index count must be greater than 0".to_string());
                    return false;
                }

                let instance_count = self.instance_count_input.parse::<u32>().unwrap();
                if instance_count == 0 {
                    self.validation_error =
                        Some("Instance count must be greater than 0".to_string());
                    return false;
                }
            }
            DrawType::Indirect | DrawType::IndexedIndirect => {
                // Validate indirect offset
                if self.indirect_offset_input.parse::<u64>().is_err() {
                    self.validation_error =
                        Some("Indirect offset must be a valid number".to_string());
                    return false;
                }
            }
        }

        true
    }

    /// Reset all parameters to default values
    fn reset(&mut self) {
        *self = Self::new();
    }

    /// Get a summary of the current draw command
    fn get_summary(&self) -> String {
        match self.draw_type {
            DrawType::NonIndexed => {
                let vertex_count = self.vertex_count_input.parse::<u32>().unwrap_or(0);
                let first_vertex = self.first_vertex_input.parse::<u32>().unwrap_or(0);
                let instance_count = self.instance_count_input.parse::<u32>().unwrap_or(0);
                let first_instance = self.first_instance_input.parse::<u32>().unwrap_or(0);

                format!(
                    "draw(vertices: {}..{}, instances: {}..{})",
                    first_vertex,
                    first_vertex + vertex_count,
                    first_instance,
                    first_instance + instance_count
                )
            }
            DrawType::Indexed => {
                let index_count = self.index_count_input.parse::<u32>().unwrap_or(0);
                let first_index = self.first_index_input.parse::<u32>().unwrap_or(0);
                let base_vertex = self.base_vertex_input.parse::<i32>().unwrap_or(0);
                let instance_count = self.instance_count_input.parse::<u32>().unwrap_or(0);
                let first_instance = self.first_instance_input.parse::<u32>().unwrap_or(0);

                format!(
                    "draw_indexed(indices: {}..{}, base_vertex: {}, instances: {}..{})",
                    first_index,
                    first_index + index_count,
                    base_vertex,
                    first_instance,
                    first_instance + instance_count
                )
            }
            DrawType::Indirect => {
                let offset = self.indirect_offset_input.parse::<u64>().unwrap_or(0);
                format!("draw_indirect(offset: {})", offset)
            }
            DrawType::IndexedIndirect => {
                let offset = self.indirect_offset_input.parse::<u64>().unwrap_or(0);
                format!("draw_indexed_indirect(offset: {})", offset)
            }
        }
    }

    /// Render the UI
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("📊 Draw Command Configuration");
            ui.separator();
            ui.label("Configure and preview draw command parameters for rendering operations.");
            ui.add_space(10.0);

            // Draw type selection
            ui.group(|ui| {
                ui.label(egui::RichText::new("Draw Type Selection").strong());
                ui.separator();
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.draw_type, DrawType::NonIndexed, "Non-indexed");
                    ui.selectable_value(&mut self.draw_type, DrawType::Indexed, "Indexed");
                    ui.selectable_value(&mut self.draw_type, DrawType::Indirect, "Indirect");
                    ui.selectable_value(
                        &mut self.draw_type,
                        DrawType::IndexedIndirect,
                        "Indexed Indirect",
                    );
                });

                ui.add_space(5.0);

                // Description based on selected type
                let description = match self.draw_type {
                    DrawType::NonIndexed => "Draw vertices directly from vertex buffer(s)",
                    DrawType::Indexed => "Draw using an index buffer to reference vertices",
                    DrawType::Indirect => "Draw with parameters stored in a GPU buffer",
                    DrawType::IndexedIndirect => {
                        "Indexed draw with parameters stored in a GPU buffer"
                    }
                };
                ui.label(egui::RichText::new(description).weak().italics());
            });

            ui.add_space(10.0);

            // Parameters based on draw type
            match self.draw_type {
                DrawType::NonIndexed => self.render_non_indexed_params(ui),
                DrawType::Indexed => self.render_indexed_params(ui),
                DrawType::Indirect => self.render_indirect_params(ui),
                DrawType::IndexedIndirect => self.render_indexed_indirect_params(ui),
            }

            ui.add_space(10.0);

            // Action buttons
            ui.group(|ui| {
                ui.label(egui::RichText::new("Actions").strong());
                ui.separator();
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    if ui.button("✓ Validate").clicked() && self.validate() {
                        self.success_message = Some("Parameters are valid!".to_string());
                    }

                    if ui.button("🔄 Reset").clicked() {
                        self.reset();
                    }
                });
            });

            ui.add_space(10.0);

            // Error and success messages
            if let Some(error) = &self.validation_error {
                ui.colored_label(egui::Color32::RED, format!("❌ Error: {}", error));
                ui.add_space(5.0);
            }

            if let Some(success) = &self.success_message {
                ui.colored_label(egui::Color32::GREEN, format!("✓ {}", success));
                ui.add_space(5.0);
            }

            ui.add_space(10.0);

            // Command summary
            ui.group(|ui| {
                ui.label(egui::RichText::new("Command Summary").strong());
                ui.separator();
                ui.add_space(5.0);

                let summary = self.get_summary();
                ui.label(egui::RichText::new("Generated Draw Call:").weak());
                ui.label(egui::RichText::new(summary).code());
            });

            ui.add_space(10.0);

            // Information section
            ui.group(|ui| {
                ui.label(egui::RichText::new("ℹ️ Information").strong());
                ui.separator();
                ui.add_space(5.0);

                match self.draw_type {
                    DrawType::NonIndexed => {
                        ui.label("• Vertex Count: Number of vertices to draw");
                        ui.label("• First Vertex: Starting vertex index in the vertex buffer");
                        ui.label("• Instance Count: Number of instances to draw");
                        ui.label("• First Instance: Starting instance index");
                    }
                    DrawType::Indexed => {
                        ui.label("• Index Count: Number of indices to draw from the index buffer");
                        ui.label("• First Index: Starting index in the index buffer");
                        ui.label(
                            "• Base Vertex: Offset added to each index before indexing vertices",
                        );
                        ui.label("• Instance Count: Number of instances to draw");
                        ui.label("• First Instance: Starting instance index");
                    }
                    DrawType::Indirect | DrawType::IndexedIndirect => {
                        ui.label("• Indirect Offset: Byte offset in the indirect buffer");
                        ui.label(
                            "• The indirect buffer must contain properly formatted draw parameters",
                        );
                        ui.label("• Requires INDIRECT usage flag on the buffer");
                    }
                }
            });
        });
    }

    fn render_non_indexed_params(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label(egui::RichText::new("Vertex Parameters").strong());
            ui.separator();
            ui.add_space(5.0);

            egui::Grid::new("non_indexed_grid")
                .num_columns(2)
                .spacing([10.0, 8.0])
                .show(ui, |ui| {
                    ui.label("Vertex Count:")
                        .webgpu_tooltip(
                            crate::tooltip::draw::VERTEX_COUNT.description,
                            crate::tooltip::draw::VERTEX_COUNT.spec_anchor,
                        );
                    ui.add(
                        egui::TextEdit::singleline(&mut self.vertex_count_input)
                            .desired_width(100.0),
                    );
                    ui.end_row();

                    ui.label("First Vertex:")
                        .webgpu_tooltip(
                            crate::tooltip::draw::FIRST_VERTEX.description,
                            crate::tooltip::draw::FIRST_VERTEX.spec_anchor,
                        );
                    ui.add(
                        egui::TextEdit::singleline(&mut self.first_vertex_input)
                            .desired_width(100.0),
                    );
                    ui.end_row();

                    ui.label("Instance Count:")
                        .webgpu_tooltip(
                            crate::tooltip::draw::INSTANCE_COUNT.description,
                            crate::tooltip::draw::INSTANCE_COUNT.spec_anchor,
                        );
                    ui.add(
                        egui::TextEdit::singleline(&mut self.instance_count_input)
                            .desired_width(100.0),
                    );
                    ui.end_row();

                    ui.label("First Instance:")
                        .webgpu_tooltip(
                            crate::tooltip::draw::FIRST_INSTANCE.description,
                            crate::tooltip::draw::FIRST_INSTANCE.spec_anchor,
                        );
                    ui.add(
                        egui::TextEdit::singleline(&mut self.first_instance_input)
                            .desired_width(100.0),
                    );
                    ui.end_row();
                });
        });
    }

    fn render_indexed_params(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label(egui::RichText::new("Index Parameters").strong());
            ui.separator();
            ui.add_space(5.0);

            egui::Grid::new("indexed_grid")
                .num_columns(2)
                .spacing([10.0, 8.0])
                .show(ui, |ui| {
                    ui.label("Index Count:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.index_count_input)
                            .desired_width(100.0),
                    );
                    ui.end_row();

                    ui.label("First Index:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.first_index_input)
                            .desired_width(100.0),
                    );
                    ui.end_row();

                    ui.label("Base Vertex:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.base_vertex_input)
                            .desired_width(100.0),
                    );
                    ui.end_row();

                    ui.label("Instance Count:")
                        .webgpu_tooltip(
                            crate::tooltip::draw::INSTANCE_COUNT.description,
                            crate::tooltip::draw::INSTANCE_COUNT.spec_anchor,
                        );
                    ui.add(
                        egui::TextEdit::singleline(&mut self.instance_count_input)
                            .desired_width(100.0),
                    );
                    ui.end_row();

                    ui.label("First Instance:")
                        .webgpu_tooltip(
                            crate::tooltip::draw::FIRST_INSTANCE.description,
                            crate::tooltip::draw::FIRST_INSTANCE.spec_anchor,
                        );
                    ui.add(
                        egui::TextEdit::singleline(&mut self.first_instance_input)
                            .desired_width(100.0),
                    );
                    ui.end_row();
                });
        });
    }

    fn render_indirect_params(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label(egui::RichText::new("Indirect Parameters").strong());
            ui.separator();
            ui.add_space(5.0);

            egui::Grid::new("indirect_grid")
                .num_columns(2)
                .spacing([10.0, 8.0])
                .show(ui, |ui| {
                    ui.label("Indirect Offset (bytes):");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.indirect_offset_input)
                            .desired_width(100.0),
                    );
                    ui.end_row();
                });

            ui.add_space(5.0);
            ui.label(
                egui::RichText::new("Note: Requires an indirect buffer with INDIRECT usage")
                    .weak()
                    .italics(),
            );
        });
    }

    fn render_indexed_indirect_params(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label(egui::RichText::new("Indexed Indirect Parameters").strong());
            ui.separator();
            ui.add_space(5.0);

            egui::Grid::new("indexed_indirect_grid")
                .num_columns(2)
                .spacing([10.0, 8.0])
                .show(ui, |ui| {
                    ui.label("Indirect Offset (bytes):");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.indirect_offset_input)
                            .desired_width(100.0),
                    );
                    ui.end_row();
                });

            ui.add_space(5.0);
            ui.label(
                egui::RichText::new("Note: Requires an indirect buffer with INDIRECT usage")
                    .weak()
                    .italics(),
            );
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_command_panel_new() {
        let panel = DrawCommandPanel::new();
        assert_eq!(panel.draw_type, DrawType::NonIndexed);
        assert_eq!(panel.vertex_count_input, "3");
        assert_eq!(panel.first_vertex_input, "0");
        assert_eq!(panel.instance_count_input, "1");
        assert_eq!(panel.first_instance_input, "0");
        assert_eq!(panel.index_count_input, "6");
        assert_eq!(panel.base_vertex_input, "0");
        assert_eq!(panel.first_index_input, "0");
        assert_eq!(panel.indirect_offset_input, "0");
        assert!(panel.validation_error.is_none());
        assert!(panel.success_message.is_none());
    }

    #[test]
    fn test_draw_command_panel_default() {
        let panel = DrawCommandPanel::default();
        assert_eq!(panel.draw_type, DrawType::NonIndexed);
    }

    #[test]
    fn test_validate_non_indexed_success() {
        let mut panel = DrawCommandPanel::new();
        panel.draw_type = DrawType::NonIndexed;
        panel.vertex_count_input = "3".to_string();
        panel.first_vertex_input = "0".to_string();
        panel.instance_count_input = "1".to_string();
        panel.first_instance_input = "0".to_string();

        assert!(panel.validate());
        assert!(panel.validation_error.is_none());
    }

    #[test]
    fn test_validate_non_indexed_invalid_vertex_count() {
        let mut panel = DrawCommandPanel::new();
        panel.vertex_count_input = "abc".to_string();

        assert!(!panel.validate());
        assert!(panel.validation_error.is_some());
        assert!(panel
            .validation_error
            .as_ref()
            .unwrap()
            .contains("Vertex count"));
    }

    #[test]
    fn test_validate_non_indexed_zero_vertex_count() {
        let mut panel = DrawCommandPanel::new();
        panel.vertex_count_input = "0".to_string();

        assert!(!panel.validate());
        assert!(panel.validation_error.is_some());
        assert!(panel
            .validation_error
            .as_ref()
            .unwrap()
            .contains("greater than 0"));
    }

    #[test]
    fn test_validate_indexed_success() {
        let mut panel = DrawCommandPanel::new();
        panel.draw_type = DrawType::Indexed;
        panel.index_count_input = "6".to_string();
        panel.first_index_input = "0".to_string();
        panel.base_vertex_input = "0".to_string();
        panel.instance_count_input = "1".to_string();
        panel.first_instance_input = "0".to_string();

        assert!(panel.validate());
        assert!(panel.validation_error.is_none());
    }

    #[test]
    fn test_validate_indexed_invalid_base_vertex() {
        let mut panel = DrawCommandPanel::new();
        panel.draw_type = DrawType::Indexed;
        panel.base_vertex_input = "not_a_number".to_string();

        assert!(!panel.validate());
        assert!(panel.validation_error.is_some());
        assert!(panel
            .validation_error
            .as_ref()
            .unwrap()
            .contains("Base vertex"));
    }

    #[test]
    fn test_validate_indexed_negative_base_vertex() {
        let mut panel = DrawCommandPanel::new();
        panel.draw_type = DrawType::Indexed;
        panel.base_vertex_input = "-5".to_string();

        // Should succeed because base_vertex is an i32 and can be negative
        assert!(panel.validate());
    }

    #[test]
    fn test_validate_indirect_success() {
        let mut panel = DrawCommandPanel::new();
        panel.draw_type = DrawType::Indirect;
        panel.indirect_offset_input = "0".to_string();

        assert!(panel.validate());
        assert!(panel.validation_error.is_none());
    }

    #[test]
    fn test_validate_indirect_invalid_offset() {
        let mut panel = DrawCommandPanel::new();
        panel.draw_type = DrawType::Indirect;
        panel.indirect_offset_input = "invalid".to_string();

        assert!(!panel.validate());
        assert!(panel.validation_error.is_some());
        assert!(panel
            .validation_error
            .as_ref()
            .unwrap()
            .contains("Indirect offset"));
    }

    #[test]
    fn test_reset() {
        let mut panel = DrawCommandPanel::new();
        panel.vertex_count_input = "100".to_string();
        panel.validation_error = Some("Test error".to_string());

        panel.reset();

        assert_eq!(panel.vertex_count_input, "3");
        assert!(panel.validation_error.is_none());
    }

    #[test]
    fn test_get_summary_non_indexed() {
        let mut panel = DrawCommandPanel::new();
        panel.draw_type = DrawType::NonIndexed;
        panel.vertex_count_input = "3".to_string();
        panel.first_vertex_input = "0".to_string();
        panel.instance_count_input = "1".to_string();
        panel.first_instance_input = "0".to_string();

        let summary = panel.get_summary();
        assert!(summary.contains("draw(vertices: 0..3, instances: 0..1)"));
    }

    #[test]
    fn test_get_summary_indexed() {
        let mut panel = DrawCommandPanel::new();
        panel.draw_type = DrawType::Indexed;
        panel.index_count_input = "6".to_string();
        panel.first_index_input = "0".to_string();
        panel.base_vertex_input = "0".to_string();
        panel.instance_count_input = "1".to_string();
        panel.first_instance_input = "0".to_string();

        let summary = panel.get_summary();
        assert!(summary.contains("draw_indexed(indices: 0..6, base_vertex: 0, instances: 0..1)"));
    }

    #[test]
    fn test_get_summary_indirect() {
        let mut panel = DrawCommandPanel::new();
        panel.draw_type = DrawType::Indirect;
        panel.indirect_offset_input = "64".to_string();

        let summary = panel.get_summary();
        assert!(summary.contains("draw_indirect(offset: 64)"));
    }

    #[test]
    fn test_draw_type_enum() {
        let non_indexed = DrawType::NonIndexed;
        let indexed = DrawType::Indexed;
        let indirect = DrawType::Indirect;
        let indexed_indirect = DrawType::IndexedIndirect;

        assert_eq!(non_indexed, DrawType::NonIndexed);
        assert_eq!(indexed, DrawType::Indexed);
        assert_eq!(indirect, DrawType::Indirect);
        assert_eq!(indexed_indirect, DrawType::IndexedIndirect);

        assert_ne!(non_indexed, indexed);
        assert_ne!(indirect, indexed_indirect);
    }
}
