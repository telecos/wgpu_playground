use crate::buffer::{BufferDescriptor, BufferUsages};

/// UI panel for creating and configuring GPU buffers
pub struct BufferPanel {
    /// Current buffer descriptor being configured
    descriptor: BufferDescriptor,
    /// Label input text
    label_input: String,
    /// Size input text (as string for user input)
    size_input: String,
    /// Usage flags state
    usage_vertex: bool,
    usage_index: bool,
    usage_uniform: bool,
    usage_storage: bool,
    usage_indirect: bool,
    usage_copy_src: bool,
    usage_copy_dst: bool,
    usage_map_read: bool,
    usage_map_write: bool,
    usage_query_resolve: bool,
    /// Mapped at creation flag
    mapped_at_creation: bool,
    /// Validation error message
    validation_error: Option<String>,
    /// Success message
    success_message: Option<String>,
}

impl Default for BufferPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl BufferPanel {
    /// Create a new buffer panel with default values
    pub fn new() -> Self {
        Self {
            descriptor: BufferDescriptor::default(),
            label_input: String::new(),
            size_input: "256".to_string(),
            usage_vertex: false,
            usage_index: false,
            usage_uniform: false,
            usage_storage: false,
            usage_indirect: false,
            usage_copy_src: false,
            usage_copy_dst: true, // Default to COPY_DST like BufferDescriptor::default()
            usage_map_read: false,
            usage_map_write: false,
            usage_query_resolve: false,
            mapped_at_creation: false,
            validation_error: None,
            success_message: None,
        }
    }

    /// Update the internal descriptor based on current UI state
    fn update_descriptor(&mut self) {
        // Parse size
        let size = self.size_input.parse::<u64>().unwrap_or(0);

        // Build usage flags
        let mut usage = BufferUsages::empty();
        if self.usage_vertex {
            usage |= BufferUsages::VERTEX;
        }
        if self.usage_index {
            usage |= BufferUsages::INDEX;
        }
        if self.usage_uniform {
            usage |= BufferUsages::UNIFORM;
        }
        if self.usage_storage {
            usage |= BufferUsages::STORAGE;
        }
        if self.usage_indirect {
            usage |= BufferUsages::INDIRECT;
        }
        if self.usage_copy_src {
            usage |= BufferUsages::COPY_SRC;
        }
        if self.usage_copy_dst {
            usage |= BufferUsages::COPY_DST;
        }
        if self.usage_map_read {
            usage |= BufferUsages::MAP_READ;
        }
        if self.usage_map_write {
            usage |= BufferUsages::MAP_WRITE;
        }
        if self.usage_query_resolve {
            usage |= BufferUsages::QUERY_RESOLVE;
        }

        // Create descriptor
        let label = if self.label_input.is_empty() {
            None
        } else {
            Some(self.label_input.as_str())
        };

        self.descriptor = BufferDescriptor::new(label, size, usage)
            .with_mapped_at_creation(self.mapped_at_creation);
    }

    /// Validate the current configuration
    fn validate(&mut self) -> bool {
        self.update_descriptor();

        match self.descriptor.validate() {
            Ok(_) => {
                self.validation_error = None;
                true
            }
            Err(e) => {
                self.validation_error = Some(e.to_string());
                self.success_message = None;
                false
            }
        }
    }

    /// Create a buffer with the current configuration
    /// Returns a descriptor that can be used to create the buffer
    pub fn create_buffer(&mut self, device: &wgpu::Device) -> Option<wgpu::Buffer> {
        if !self.validate() {
            return None;
        }

        match self.descriptor.create_buffer(device) {
            Ok(buffer) => {
                self.success_message = Some(format!(
                    "✓ Buffer created successfully: {} bytes",
                    self.descriptor.size()
                ));
                self.validation_error = None;
                Some(buffer)
            }
            Err(e) => {
                self.validation_error = Some(format!("Failed to create buffer: {}", e));
                self.success_message = None;
                None
            }
        }
    }

    /// Render the buffer configuration UI
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("📐 Buffer Configuration");
            ui.label("Configure and create GPU buffers with custom parameters.");
            ui.add_space(10.0);

            // Buffer Label
            ui.group(|ui| {
                ui.heading("Buffer Properties");
                ui.add_space(5.0);

                egui::Grid::new("buffer_properties")
                    .num_columns(2)
                    .spacing([10.0, 8.0])
                    .show(ui, |ui| {
                        ui.label("Label:");
                        ui.text_edit_singleline(&mut self.label_input);
                        ui.end_row();

                        ui.label("Size (bytes):");
                        ui.text_edit_singleline(&mut self.size_input);
                        ui.end_row();
                    });
            });

            ui.add_space(10.0);

            // Usage Flags
            ui.group(|ui| {
                ui.heading("Usage Flags");
                ui.label("Select how the buffer will be used (multiple flags can be selected):");
                ui.add_space(5.0);

                egui::Grid::new("usage_flags")
                    .num_columns(2)
                    .spacing([10.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        Self::render_usage_checkbox(ui, "VERTEX", &mut self.usage_vertex, "Buffer can be used as a vertex buffer");
                        Self::render_usage_checkbox(ui, "INDEX", &mut self.usage_index, "Buffer can be used as an index buffer");
                        Self::render_usage_checkbox(ui, "UNIFORM", &mut self.usage_uniform, "Buffer can be used as a uniform buffer");
                        Self::render_usage_checkbox(ui, "STORAGE", &mut self.usage_storage, "Buffer can be used as a storage buffer");
                        Self::render_usage_checkbox(ui, "INDIRECT", &mut self.usage_indirect, "Buffer can be used for indirect draw commands");
                        Self::render_usage_checkbox(ui, "COPY_SRC", &mut self.usage_copy_src, "Buffer can be used as a copy source");
                        Self::render_usage_checkbox(ui, "COPY_DST", &mut self.usage_copy_dst, "Buffer can be used as a copy destination");
                        Self::render_usage_checkbox(ui, "MAP_READ", &mut self.usage_map_read, "Buffer can be mapped for reading");
                        Self::render_usage_checkbox(ui, "MAP_WRITE", &mut self.usage_map_write, "Buffer can be mapped for writing");
                        Self::render_usage_checkbox(ui, "QUERY_RESOLVE", &mut self.usage_query_resolve, "Buffer can be used to resolve query results");
                    });

                ui.add_space(5.0);
                ui.colored_label(
                    egui::Color32::from_rgb(200, 200, 100),
                    "💡 Note: MAP_READ and MAP_WRITE cannot be used together"
                );
            });

            ui.add_space(10.0);

            // Additional Options
            ui.group(|ui| {
                ui.heading("Additional Options");
                ui.add_space(5.0);

                ui.checkbox(&mut self.mapped_at_creation, "Mapped at creation")
                    .on_hover_text("Whether the buffer should be mapped immediately after creation");
            });

            ui.add_space(15.0);

            // Validation and Creation
            ui.horizontal(|ui| {
                if ui.button("🔍 Validate").clicked() {
                    self.validate();
                }

                if ui.button("✨ Create Buffer").clicked() {
                    // Note: In the actual implementation, we would need a device reference
                    // For now, we just validate
                    if self.validate() {
                        self.success_message = Some(
                            "✓ Configuration is valid. In a full implementation, the buffer would be created here."
                                .to_string(),
                        );
                    }
                }

                if ui.button("🔄 Reset").clicked() {
                    *self = Self::new();
                }
            });

            ui.add_space(10.0);

            // Display validation errors or success messages
            if let Some(error) = &self.validation_error {
                ui.colored_label(egui::Color32::RED, format!("❌ {}", error));
            }

            if let Some(success) = &self.success_message {
                ui.colored_label(egui::Color32::GREEN, success);
            }

            ui.add_space(15.0);

            // Current Configuration Summary
            ui.group(|ui| {
                ui.heading("Configuration Summary");
                ui.add_space(5.0);

                self.update_descriptor();

                ui.monospace(format!(
                    "Label: {}",
                    self.descriptor.label().unwrap_or("<none>")
                ));
                ui.monospace(format!("Size: {} bytes", self.descriptor.size()));
                ui.monospace(format!(
                    "Mapped at creation: {}",
                    self.descriptor.mapped_at_creation()
                ));

                ui.add_space(5.0);
                ui.label("Usage flags:");
                let usage = self.descriptor.usage();
                if usage.is_empty() {
                    ui.monospace("  (none)");
                } else {
                    if usage.contains(BufferUsages::VERTEX) {
                        ui.monospace("  • VERTEX");
                    }
                    if usage.contains(BufferUsages::INDEX) {
                        ui.monospace("  • INDEX");
                    }
                    if usage.contains(BufferUsages::UNIFORM) {
                        ui.monospace("  • UNIFORM");
                    }
                    if usage.contains(BufferUsages::STORAGE) {
                        ui.monospace("  • STORAGE");
                    }
                    if usage.contains(BufferUsages::INDIRECT) {
                        ui.monospace("  • INDIRECT");
                    }
                    if usage.contains(BufferUsages::COPY_SRC) {
                        ui.monospace("  • COPY_SRC");
                    }
                    if usage.contains(BufferUsages::COPY_DST) {
                        ui.monospace("  • COPY_DST");
                    }
                    if usage.contains(BufferUsages::MAP_READ) {
                        ui.monospace("  • MAP_READ");
                    }
                    if usage.contains(BufferUsages::MAP_WRITE) {
                        ui.monospace("  • MAP_WRITE");
                    }
                    if usage.contains(BufferUsages::QUERY_RESOLVE) {
                        ui.monospace("  • QUERY_RESOLVE");
                    }
                }
            });
        });
    }

    fn render_usage_checkbox(ui: &mut egui::Ui, label: &str, value: &mut bool, tooltip: &str) {
        ui.checkbox(value, label).on_hover_text(tooltip);
        ui.end_row();
    }

    /// Export the current state to a serializable format
    pub fn export_state(&self) -> crate::state::BufferPanelState {
        crate::state::BufferPanelState {
            label: self.label_input.clone(),
            size: self.size_input.clone(),
            usage_vertex: self.usage_vertex,
            usage_index: self.usage_index,
            usage_uniform: self.usage_uniform,
            usage_storage: self.usage_storage,
            usage_indirect: self.usage_indirect,
            usage_copy_src: self.usage_copy_src,
            usage_copy_dst: self.usage_copy_dst,
            usage_map_read: self.usage_map_read,
            usage_map_write: self.usage_map_write,
            usage_query_resolve: self.usage_query_resolve,
            mapped_at_creation: self.mapped_at_creation,
        }
    }

    /// Import state from a serializable format
    pub fn import_state(&mut self, state: &crate::state::BufferPanelState) {
        self.label_input = state.label.clone();
        self.size_input = state.size.clone();
        self.usage_vertex = state.usage_vertex;
        self.usage_index = state.usage_index;
        self.usage_uniform = state.usage_uniform;
        self.usage_storage = state.usage_storage;
        self.usage_indirect = state.usage_indirect;
        self.usage_copy_src = state.usage_copy_src;
        self.usage_copy_dst = state.usage_copy_dst;
        self.usage_map_read = state.usage_map_read;
        self.usage_map_write = state.usage_map_write;
        self.usage_query_resolve = state.usage_query_resolve;
        self.mapped_at_creation = state.mapped_at_creation;
        self.validation_error = None;
        self.success_message = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_panel_creation() {
        let panel = BufferPanel::new();
        assert_eq!(panel.size_input, "256");
        assert_eq!(panel.label_input, "");
        assert!(panel.usage_copy_dst);
        assert!(!panel.usage_vertex);
        assert!(!panel.mapped_at_creation);
    }

    #[test]
    fn test_buffer_panel_default() {
        let panel = BufferPanel::default();
        assert_eq!(panel.size_input, "256");
    }

    #[test]
    fn test_update_descriptor() {
        let mut panel = BufferPanel::new();
        panel.label_input = "test_buffer".to_string();
        panel.size_input = "1024".to_string();
        panel.usage_vertex = true;
        panel.usage_copy_dst = true;

        panel.update_descriptor();

        assert_eq!(panel.descriptor.label(), Some("test_buffer"));
        assert_eq!(panel.descriptor.size(), 1024);
        assert!(panel.descriptor.usage().contains(BufferUsages::VERTEX));
        assert!(panel.descriptor.usage().contains(BufferUsages::COPY_DST));
    }

    #[test]
    fn test_validate_success() {
        let mut panel = BufferPanel::new();
        panel.size_input = "256".to_string();
        panel.usage_uniform = true;

        let valid = panel.validate();
        assert!(valid);
        assert!(panel.validation_error.is_none());
    }

    #[test]
    fn test_validate_zero_size() {
        let mut panel = BufferPanel::new();
        panel.size_input = "0".to_string();
        panel.usage_uniform = true;

        let valid = panel.validate();
        assert!(!valid);
        assert!(panel.validation_error.is_some());
        assert!(panel
            .validation_error
            .as_ref()
            .unwrap()
            .contains("greater than 0"));
    }

    #[test]
    fn test_validate_no_usage() {
        let mut panel = BufferPanel::new();
        panel.size_input = "256".to_string();
        // Clear all usage flags
        panel.usage_copy_dst = false;

        let valid = panel.validate();
        assert!(!valid);
        assert!(panel.validation_error.is_some());
        assert!(panel
            .validation_error
            .as_ref()
            .unwrap()
            .contains("at least one usage flag"));
    }

    #[test]
    fn test_validate_map_read_and_write() {
        let mut panel = BufferPanel::new();
        panel.size_input = "256".to_string();
        panel.usage_map_read = true;
        panel.usage_map_write = true;

        let valid = panel.validate();
        assert!(!valid);
        assert!(panel.validation_error.is_some());
        assert!(panel
            .validation_error
            .as_ref()
            .unwrap()
            .contains("MAP_READ and MAP_WRITE"));
    }

    #[test]
    fn test_validate_invalid_size_input() {
        let mut panel = BufferPanel::new();
        panel.size_input = "not_a_number".to_string();
        panel.usage_uniform = true;

        let valid = panel.validate();
        assert!(!valid);
        assert!(panel.validation_error.is_some());
    }

    #[test]
    fn test_mapped_at_creation() {
        let mut panel = BufferPanel::new();
        panel.mapped_at_creation = true;
        panel.size_input = "256".to_string();
        panel.usage_map_write = true;

        panel.update_descriptor();

        assert!(panel.descriptor.mapped_at_creation());
    }

    #[test]
    fn test_empty_label() {
        let mut panel = BufferPanel::new();
        panel.label_input = "".to_string();
        panel.size_input = "256".to_string();
        panel.usage_uniform = true;

        panel.update_descriptor();

        assert_eq!(panel.descriptor.label(), None);
    }

    #[test]
    fn test_all_usage_flags() {
        let mut panel = BufferPanel::new();
        panel.size_input = "1024".to_string();
        panel.usage_vertex = true;
        panel.usage_index = true;
        panel.usage_uniform = true;
        panel.usage_storage = true;
        panel.usage_indirect = true;
        panel.usage_copy_src = true;
        panel.usage_copy_dst = true;
        panel.usage_query_resolve = true;
        // Not setting MAP_READ and MAP_WRITE together as they conflict

        panel.update_descriptor();

        let usage = panel.descriptor.usage();
        assert!(usage.contains(BufferUsages::VERTEX));
        assert!(usage.contains(BufferUsages::INDEX));
        assert!(usage.contains(BufferUsages::UNIFORM));
        assert!(usage.contains(BufferUsages::STORAGE));
        assert!(usage.contains(BufferUsages::INDIRECT));
        assert!(usage.contains(BufferUsages::COPY_SRC));
        assert!(usage.contains(BufferUsages::COPY_DST));
        assert!(usage.contains(BufferUsages::QUERY_RESOLVE));
    }

    // GUI Interaction Tests - Simulating User Workflows

    #[test]
    fn test_gui_interaction_create_vertex_buffer_workflow() {
        let mut panel = BufferPanel::new();

        // User types a label
        panel.label_input = "my_vertex_buffer".to_string();

        // User types a size
        panel.size_input = "1024".to_string();

        // User clicks VERTEX checkbox
        panel.usage_vertex = true;

        // User validates
        let is_valid = panel.validate();
        assert!(is_valid);
        assert!(panel.validation_error.is_none());

        // Verify final state
        assert_eq!(panel.descriptor.label(), Some("my_vertex_buffer"));
        assert_eq!(panel.descriptor.size(), 1024);
        assert!(panel.descriptor.usage().contains(BufferUsages::VERTEX));
    }

    #[test]
    fn test_gui_interaction_incremental_size_input() {
        let mut panel = BufferPanel::new();

        // Simulate user typing size character by character
        panel.size_input = "1".to_string();
        panel.update_descriptor();
        assert_eq!(panel.descriptor.size(), 1);

        panel.size_input = "12".to_string();
        panel.update_descriptor();
        assert_eq!(panel.descriptor.size(), 12);

        panel.size_input = "128".to_string();
        panel.update_descriptor();
        assert_eq!(panel.descriptor.size(), 128);

        panel.size_input = "1280".to_string();
        panel.update_descriptor();
        assert_eq!(panel.descriptor.size(), 1280);
    }

    #[test]
    fn test_gui_interaction_error_then_fix() {
        let mut panel = BufferPanel::new();

        // User makes an error - enters zero size
        panel.size_input = "0".to_string();
        panel.usage_uniform = true;

        // Validation fails
        assert!(!panel.validate());
        assert!(panel.validation_error.is_some());

        // User fixes the error
        panel.size_input = "256".to_string();

        // Validation now succeeds
        assert!(panel.validate());
        assert!(panel.validation_error.is_none());
    }

    #[test]
    fn test_gui_interaction_toggle_usage_flags() {
        let mut panel = BufferPanel::new();
        panel.size_input = "512".to_string();

        // User selects STORAGE
        panel.usage_storage = true;
        panel.update_descriptor();
        assert!(panel.descriptor.usage().contains(BufferUsages::STORAGE));

        // User also selects COPY_SRC
        panel.usage_copy_src = true;
        panel.update_descriptor();
        assert!(panel.descriptor.usage().contains(BufferUsages::COPY_SRC));

        // User deselects COPY_DST
        panel.usage_copy_dst = false;
        panel.update_descriptor();
        assert!(!panel.descriptor.usage().contains(BufferUsages::COPY_DST));
    }

    #[test]
    fn test_gui_interaction_mapped_at_creation_toggle() {
        let mut panel = BufferPanel::new();
        panel.size_input = "256".to_string();
        panel.usage_map_write = true;

        // User toggles mapped_at_creation on
        panel.mapped_at_creation = true;
        panel.update_descriptor();
        assert!(panel.descriptor.mapped_at_creation());

        // User toggles it off
        panel.mapped_at_creation = false;
        panel.update_descriptor();
        assert!(!panel.descriptor.mapped_at_creation());
    }

    #[test]
    #[allow(unused_assignments)]
    fn test_gui_interaction_reset_workflow() {
        let mut panel = BufferPanel::new();

        // User makes changes
        panel.label_input = "test_buffer".to_string();
        panel.size_input = "2048".to_string();
        panel.usage_vertex = true;
        panel.usage_uniform = true;
        panel.mapped_at_creation = true;

        // User clicks Reset (simulated by creating new instance)
        panel = BufferPanel::new();

        // Everything back to defaults
        assert_eq!(panel.label_input, "");
        assert_eq!(panel.size_input, "256");
        assert!(!panel.usage_vertex);
        assert!(!panel.usage_uniform);
        assert!(!panel.mapped_at_creation);
        assert!(panel.usage_copy_dst); // Default
    }

    #[test]
    fn test_gui_interaction_invalid_then_valid_usage() {
        let mut panel = BufferPanel::new();
        panel.size_input = "256".to_string();

        // User selects MAP_READ and MAP_WRITE (invalid)
        panel.usage_map_read = true;
        panel.usage_map_write = true;
        panel.usage_copy_dst = false;

        assert!(!panel.validate());
        assert!(panel
            .validation_error
            .as_ref()
            .unwrap()
            .contains("MAP_READ and MAP_WRITE"));

        // User deselects MAP_WRITE
        panel.usage_map_write = false;

        // Now valid
        assert!(panel.validate());
        assert!(panel.validation_error.is_none());
    }
}
