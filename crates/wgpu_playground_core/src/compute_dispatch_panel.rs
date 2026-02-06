use crate::tooltip::compute;

/// UI panel for configuring and executing compute dispatch commands
pub struct ComputeDispatchPanel {
    // Dispatch type selection
    dispatch_type: DispatchType,

    // Direct dispatch parameters (workgroup counts)
    workgroups_x_input: String,
    workgroups_y_input: String,
    workgroups_z_input: String,

    // Indirect dispatch parameters
    indirect_offset_input: String,
    selected_buffer_index: Option<usize>,

    // Validation and messaging
    validation_error: Option<String>,
    success_message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DispatchType {
    Direct,
    Indirect,
}

/// Type alias for workgroup counts (X, Y, Z)
type WorkgroupCounts = (u32, u32, u32);

impl Default for ComputeDispatchPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl ComputeDispatchPanel {
    /// Create a new compute dispatch panel with default values
    pub fn new() -> Self {
        Self {
            dispatch_type: DispatchType::Direct,
            workgroups_x_input: "1".to_string(),
            workgroups_y_input: "1".to_string(),
            workgroups_z_input: "1".to_string(),
            indirect_offset_input: "0".to_string(),
            selected_buffer_index: None,
            validation_error: None,
            success_message: None,
        }
    }

    /// Parse workgroup counts from input strings
    /// Returns Ok((x, y, z)) if all values are valid and non-zero
    fn parse_workgroups(&self) -> Result<WorkgroupCounts, String> {
        // Parse X
        let workgroups_x = self
            .workgroups_x_input
            .parse::<u32>()
            .map_err(|_| "Workgroups X must be a valid number".to_string())?;

        // Parse Y
        let workgroups_y = self
            .workgroups_y_input
            .parse::<u32>()
            .map_err(|_| "Workgroups Y must be a valid number".to_string())?;

        // Parse Z
        let workgroups_z = self
            .workgroups_z_input
            .parse::<u32>()
            .map_err(|_| "Workgroups Z must be a valid number".to_string())?;

        // Check for zero values
        if workgroups_x == 0 {
            return Err("Workgroups X must be greater than 0".to_string());
        }
        if workgroups_y == 0 {
            return Err("Workgroups Y must be greater than 0".to_string());
        }
        if workgroups_z == 0 {
            return Err("Workgroups Z must be greater than 0".to_string());
        }

        Ok((workgroups_x, workgroups_y, workgroups_z))
    }

    /// Validate the current parameters
    fn validate(&mut self) -> bool {
        self.validation_error = None;
        self.success_message = None;

        match self.dispatch_type {
            DispatchType::Direct => {
                if let Err(error) = self.parse_workgroups() {
                    self.validation_error = Some(error);
                    return false;
                }
            }
            DispatchType::Indirect => {
                // Validate indirect offset
                if self.indirect_offset_input.parse::<u64>().is_err() {
                    self.validation_error =
                        Some("Indirect offset must be a valid number".to_string());
                    return false;
                }

                // Check if a buffer is selected
                if self.selected_buffer_index.is_none() {
                    self.validation_error =
                        Some("Please select an indirect buffer for indirect dispatch".to_string());
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

    /// Get a summary of the current dispatch command
    fn get_summary(&self) -> String {
        match self.dispatch_type {
            DispatchType::Direct => {
                let (workgroups_x, workgroups_y, workgroups_z) =
                    self.parse_workgroups().unwrap_or((0, 0, 0));

                format!(
                    "dispatch_workgroups({}, {}, {})",
                    workgroups_x, workgroups_y, workgroups_z
                )
            }
            DispatchType::Indirect => {
                let offset = self.indirect_offset_input.parse::<u64>().unwrap_or(0);
                let buffer_info = if let Some(idx) = self.selected_buffer_index {
                    format!("buffer_{}", idx)
                } else {
                    "no buffer selected".to_string()
                };

                format!(
                    "dispatch_workgroups_indirect({}, offset: {})",
                    buffer_info, offset
                )
            }
        }
    }

    /// Render the UI
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("🧮 Compute Dispatch Configuration");
            ui.separator();
            ui.label(
                "Configure and preview compute dispatch parameters for GPU compute operations.",
            );
            ui.add_space(10.0);

            // Dispatch type selection
            ui.group(|ui| {
                ui.label(egui::RichText::new("Dispatch Type Selection").strong());
                ui.separator();
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.dispatch_type, DispatchType::Direct, "Direct");
                    ui.selectable_value(
                        &mut self.dispatch_type,
                        DispatchType::Indirect,
                        "Indirect",
                    );
                });

                ui.add_space(5.0);

                // Description based on selected type
                let description = match self.dispatch_type {
                    DispatchType::Direct => {
                        "Dispatch with explicit workgroup counts for X, Y, Z dimensions"
                    }
                    DispatchType::Indirect => {
                        "Dispatch with workgroup counts stored in a GPU buffer"
                    }
                };
                ui.label(egui::RichText::new(description).weak().italics());
            });

            ui.add_space(10.0);

            // Parameters based on dispatch type
            match self.dispatch_type {
                DispatchType::Direct => self.render_direct_params(ui),
                DispatchType::Indirect => self.render_indirect_params(ui),
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
                ui.label(egui::RichText::new("Generated Dispatch Call:").weak());
                ui.label(egui::RichText::new(summary).code());
            });

            ui.add_space(10.0);

            // Information section
            ui.group(|ui| {
                ui.label(egui::RichText::new("ℹ️ Information").strong());
                ui.separator();
                ui.add_space(5.0);

                match self.dispatch_type {
                    DispatchType::Direct => {
                        ui.label("• Workgroups X: Number of workgroups in the X dimension");
                        ui.label("• Workgroups Y: Number of workgroups in the Y dimension");
                        ui.label("• Workgroups Z: Number of workgroups in the Z dimension");
                        ui.add_space(5.0);
                        ui.label("Each workgroup executes the compute shader with the");
                        ui.label(
                            "workgroup size specified in the shader's @workgroup_size attribute.",
                        );
                        ui.add_space(5.0);
                        ui.label(
                            egui::RichText::new("Example: @workgroup_size(64, 1, 1)")
                                .weak()
                                .italics(),
                        );
                    }
                    DispatchType::Indirect => {
                        ui.label("• Indirect Buffer: Buffer containing dispatch parameters");
                        ui.label("• Indirect Offset: Byte offset in the indirect buffer");
                        ui.add_space(5.0);
                        ui.label("The indirect buffer must contain three u32 values:");
                        ui.label("  - workgroups_x (u32)");
                        ui.label("  - workgroups_y (u32)");
                        ui.label("  - workgroups_z (u32)");
                        ui.add_space(5.0);
                        ui.label("The buffer must have the INDIRECT usage flag set.");
                    }
                }
            });
        });
    }

    fn render_direct_params(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label(egui::RichText::new("Workgroup Counts").strong());
            ui.separator();
            ui.add_space(5.0);

            egui::Grid::new("direct_dispatch_grid")
                .num_columns(2)
                .spacing([10.0, 8.0])
                .show(ui, |ui| {
                    ui.label("Workgroups X:");
                    compute::WORKGROUP_COUNT_X.apply(ui.add(
                        egui::TextEdit::singleline(&mut self.workgroups_x_input)
                            .desired_width(100.0),
                    ));
                    ui.end_row();

                    ui.label("Workgroups Y:");
                    compute::WORKGROUP_COUNT_Y.apply(ui.add(
                        egui::TextEdit::singleline(&mut self.workgroups_y_input)
                            .desired_width(100.0),
                    ));
                    ui.end_row();

                    ui.label("Workgroups Z:");
                    compute::WORKGROUP_COUNT_Z.apply(ui.add(
                        egui::TextEdit::singleline(&mut self.workgroups_z_input)
                            .desired_width(100.0),
                    ));
                    ui.end_row();
                });

            ui.add_space(5.0);
            ui.label(
                egui::RichText::new(
                    "Note: Total invocations = workgroups * workgroup_size (from shader)",
                )
                .weak()
                .italics(),
            );
        });
    }

    fn render_indirect_params(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label(egui::RichText::new("Indirect Dispatch Parameters").strong());
            ui.separator();
            ui.add_space(5.0);

            egui::Grid::new("indirect_dispatch_grid")
                .num_columns(2)
                .spacing([10.0, 8.0])
                .show(ui, |ui| {
                    ui.label("Indirect Buffer:");
                    egui::ComboBox::from_id_salt("indirect_buffer_selection")
                        .selected_text(
                            self.selected_buffer_index
                                .map(|i| format!("Buffer {}", i))
                                .unwrap_or_else(|| "Select a buffer...".to_string()),
                        )
                        .show_ui(ui, |ui| {
                            // Placeholder for buffer selection
                            // In a real implementation, this would list available buffers
                            ui.selectable_value(
                                &mut self.selected_buffer_index,
                                Some(0),
                                "Buffer 0 (placeholder)",
                            );
                            ui.selectable_value(
                                &mut self.selected_buffer_index,
                                Some(1),
                                "Buffer 1 (placeholder)",
                            );
                        });
                    ui.end_row();

                    ui.label("Indirect Offset (bytes):");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.indirect_offset_input)
                            .desired_width(100.0),
                    )
                    .on_hover_text(
                        "Byte offset into the indirect buffer where dispatch parameters are stored",
                    );
                    ui.end_row();
                });

            ui.add_space(5.0);
            ui.label(
                egui::RichText::new(
                    "Note: Buffer must have INDIRECT usage flag and contain 3 u32 values",
                )
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
    fn test_compute_dispatch_panel_new() {
        let panel = ComputeDispatchPanel::new();
        assert_eq!(panel.dispatch_type, DispatchType::Direct);
        assert_eq!(panel.workgroups_x_input, "1");
        assert_eq!(panel.workgroups_y_input, "1");
        assert_eq!(panel.workgroups_z_input, "1");
        assert_eq!(panel.indirect_offset_input, "0");
        assert!(panel.selected_buffer_index.is_none());
        assert!(panel.validation_error.is_none());
        assert!(panel.success_message.is_none());
    }

    #[test]
    fn test_compute_dispatch_panel_default() {
        let panel = ComputeDispatchPanel::default();
        assert_eq!(panel.dispatch_type, DispatchType::Direct);
    }

    #[test]
    fn test_validate_direct_success() {
        let mut panel = ComputeDispatchPanel::new();
        panel.dispatch_type = DispatchType::Direct;
        panel.workgroups_x_input = "64".to_string();
        panel.workgroups_y_input = "1".to_string();
        panel.workgroups_z_input = "1".to_string();

        assert!(panel.validate());
        assert!(panel.validation_error.is_none());
    }

    #[test]
    fn test_validate_direct_invalid_x() {
        let mut panel = ComputeDispatchPanel::new();
        panel.workgroups_x_input = "abc".to_string();

        assert!(!panel.validate());
        assert!(panel.validation_error.is_some());
        assert!(panel
            .validation_error
            .as_ref()
            .unwrap()
            .contains("Workgroups X"));
    }

    #[test]
    fn test_validate_direct_zero_x() {
        let mut panel = ComputeDispatchPanel::new();
        panel.workgroups_x_input = "0".to_string();

        assert!(!panel.validate());
        assert!(panel.validation_error.is_some());
        assert!(panel
            .validation_error
            .as_ref()
            .unwrap()
            .contains("greater than 0"));
    }

    #[test]
    fn test_validate_direct_invalid_y() {
        let mut panel = ComputeDispatchPanel::new();
        panel.workgroups_y_input = "not_a_number".to_string();

        assert!(!panel.validate());
        assert!(panel.validation_error.is_some());
        assert!(panel
            .validation_error
            .as_ref()
            .unwrap()
            .contains("Workgroups Y"));
    }

    #[test]
    fn test_validate_direct_zero_y() {
        let mut panel = ComputeDispatchPanel::new();
        panel.workgroups_y_input = "0".to_string();

        assert!(!panel.validate());
        assert!(panel.validation_error.is_some());
        assert!(panel
            .validation_error
            .as_ref()
            .unwrap()
            .contains("Workgroups Y"));
    }

    #[test]
    fn test_validate_direct_invalid_z() {
        let mut panel = ComputeDispatchPanel::new();
        panel.workgroups_z_input = "xyz".to_string();

        assert!(!panel.validate());
        assert!(panel.validation_error.is_some());
        assert!(panel
            .validation_error
            .as_ref()
            .unwrap()
            .contains("Workgroups Z"));
    }

    #[test]
    fn test_validate_direct_zero_z() {
        let mut panel = ComputeDispatchPanel::new();
        panel.workgroups_z_input = "0".to_string();

        assert!(!panel.validate());
        assert!(panel.validation_error.is_some());
        assert!(panel
            .validation_error
            .as_ref()
            .unwrap()
            .contains("Workgroups Z"));
    }

    #[test]
    fn test_validate_indirect_success() {
        let mut panel = ComputeDispatchPanel::new();
        panel.dispatch_type = DispatchType::Indirect;
        panel.indirect_offset_input = "0".to_string();
        panel.selected_buffer_index = Some(0);

        assert!(panel.validate());
        assert!(panel.validation_error.is_none());
    }

    #[test]
    fn test_validate_indirect_invalid_offset() {
        let mut panel = ComputeDispatchPanel::new();
        panel.dispatch_type = DispatchType::Indirect;
        panel.indirect_offset_input = "invalid".to_string();
        panel.selected_buffer_index = Some(0);

        assert!(!panel.validate());
        assert!(panel.validation_error.is_some());
        assert!(panel
            .validation_error
            .as_ref()
            .unwrap()
            .contains("Indirect offset"));
    }

    #[test]
    fn test_validate_indirect_no_buffer_selected() {
        let mut panel = ComputeDispatchPanel::new();
        panel.dispatch_type = DispatchType::Indirect;
        panel.indirect_offset_input = "0".to_string();
        panel.selected_buffer_index = None;

        assert!(!panel.validate());
        assert!(panel.validation_error.is_some());
        assert!(panel
            .validation_error
            .as_ref()
            .unwrap()
            .contains("select an indirect buffer"));
    }

    #[test]
    fn test_reset() {
        let mut panel = ComputeDispatchPanel::new();
        panel.workgroups_x_input = "100".to_string();
        panel.validation_error = Some("Test error".to_string());
        panel.selected_buffer_index = Some(5);

        panel.reset();

        assert_eq!(panel.workgroups_x_input, "1");
        assert!(panel.validation_error.is_none());
        assert!(panel.selected_buffer_index.is_none());
    }

    #[test]
    fn test_get_summary_direct() {
        let mut panel = ComputeDispatchPanel::new();
        panel.dispatch_type = DispatchType::Direct;
        panel.workgroups_x_input = "64".to_string();
        panel.workgroups_y_input = "8".to_string();
        panel.workgroups_z_input = "1".to_string();

        let summary = panel.get_summary();
        assert!(summary.contains("dispatch_workgroups(64, 8, 1)"));
    }

    #[test]
    fn test_get_summary_indirect() {
        let mut panel = ComputeDispatchPanel::new();
        panel.dispatch_type = DispatchType::Indirect;
        panel.indirect_offset_input = "64".to_string();
        panel.selected_buffer_index = Some(2);

        let summary = panel.get_summary();
        assert!(summary.contains("dispatch_workgroups_indirect"));
        assert!(summary.contains("buffer_2"));
        assert!(summary.contains("offset: 64"));
    }

    #[test]
    fn test_get_summary_indirect_no_buffer() {
        let mut panel = ComputeDispatchPanel::new();
        panel.dispatch_type = DispatchType::Indirect;
        panel.indirect_offset_input = "0".to_string();
        panel.selected_buffer_index = None;

        let summary = panel.get_summary();
        assert!(summary.contains("no buffer selected"));
    }

    #[test]
    fn test_dispatch_type_enum() {
        let direct = DispatchType::Direct;
        let indirect = DispatchType::Indirect;

        assert_eq!(direct, DispatchType::Direct);
        assert_eq!(indirect, DispatchType::Indirect);
        assert_ne!(direct, indirect);
    }

    #[test]
    fn test_validate_clears_messages() {
        let mut panel = ComputeDispatchPanel::new();
        panel.validation_error = Some("Old error".to_string());
        panel.success_message = Some("Old success".to_string());

        panel.validate();

        // After validation, old messages should be cleared
        // Success or error will be set based on validation result
        assert!(
            panel.validation_error.is_none() || panel.success_message.is_none(),
            "Both error and success should not be set at the same time"
        );
    }

    #[test]
    fn test_workgroups_large_values() {
        let mut panel = ComputeDispatchPanel::new();
        panel.workgroups_x_input = "65535".to_string();
        panel.workgroups_y_input = "65535".to_string();
        panel.workgroups_z_input = "65535".to_string();

        assert!(panel.validate());
        assert!(panel.validation_error.is_none());
    }
}
