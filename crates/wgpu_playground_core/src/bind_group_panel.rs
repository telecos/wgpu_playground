use crate::bind_group::{
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, SamplerBindingType,
    StorageTextureAccess, TextureSampleType, TextureViewDimension,
};
use wgpu::ShaderStages;

/// UI panel for creating and configuring bind groups
pub struct BindGroupPanel {
    /// Label input text for bind group layout
    layout_label_input: String,
    /// Label input text for bind group
    bind_group_label_input: String,
    /// Current bind group layout entries
    layout_entries: Vec<BindGroupLayoutEntryConfig>,
    /// Next binding number to use
    next_binding: u32,
    /// Currently selected binding for editing
    selected_binding: Option<usize>,
    /// Available mock resources for binding
    mock_buffers: Vec<MockBuffer>,
    mock_textures: Vec<MockTexture>,
    mock_samplers: Vec<MockSampler>,
    /// Binding assignments (binding number -> resource)
    binding_assignments: Vec<(u32, ResourceAssignment)>,
    /// Validation error message
    validation_error: Option<String>,
    /// Success message
    success_message: Option<String>,
    /// Current UI mode
    ui_mode: UiMode,
}

/// UI mode for the panel
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UiMode {
    /// Creating bind group layout
    CreateLayout,
    /// Binding resources
    BindResources,
}

/// Configuration for a bind group layout entry in the UI
#[derive(Debug, Clone)]
struct BindGroupLayoutEntryConfig {
    binding: u32,
    visibility: ShaderStagesConfig,
    binding_type: BindingTypeConfig,
}

/// Shader stages configuration for UI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ShaderStagesConfig {
    vertex: bool,
    fragment: bool,
    compute: bool,
}

impl ShaderStagesConfig {
    fn to_wgpu(self) -> ShaderStages {
        let mut stages = ShaderStages::empty();
        if self.vertex {
            stages |= ShaderStages::VERTEX;
        }
        if self.fragment {
            stages |= ShaderStages::FRAGMENT;
        }
        if self.compute {
            stages |= ShaderStages::COMPUTE;
        }
        stages
    }

    fn all() -> Self {
        Self {
            vertex: true,
            fragment: true,
            compute: false,
        }
    }
}

/// Binding type configuration for UI
#[derive(Debug, Clone, PartialEq)]
enum BindingTypeConfig {
    UniformBuffer,
    StorageBuffer { read_only: bool },
    Texture,
    Sampler,
    StorageTexture,
}

impl BindingTypeConfig {
    fn to_binding_type(&self) -> BindingType {
        match self {
            BindingTypeConfig::UniformBuffer => BindingType::UniformBuffer {
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            BindingTypeConfig::StorageBuffer { read_only } => BindingType::StorageBuffer {
                has_dynamic_offset: false,
                min_binding_size: None,
                read_only: *read_only,
            },
            BindingTypeConfig::Texture => BindingType::Texture {
                sample_type: TextureSampleType::Float { filterable: true },
                view_dimension: TextureViewDimension::D2,
                multisampled: false,
            },
            BindingTypeConfig::Sampler => BindingType::Sampler {
                sampler_type: SamplerBindingType::Filtering,
            },
            BindingTypeConfig::StorageTexture => BindingType::StorageTexture {
                access: StorageTextureAccess::WriteOnly,
                format: wgpu::TextureFormat::Rgba8Unorm,
                view_dimension: TextureViewDimension::D2,
            },
        }
    }

    fn name(&self) -> &'static str {
        match self {
            BindingTypeConfig::UniformBuffer => "Uniform Buffer",
            BindingTypeConfig::StorageBuffer { read_only: true } => "Storage Buffer (Read-Only)",
            BindingTypeConfig::StorageBuffer { read_only: false } => "Storage Buffer (Read-Write)",
            BindingTypeConfig::Texture => "Texture",
            BindingTypeConfig::Sampler => "Sampler",
            BindingTypeConfig::StorageTexture => "Storage Texture",
        }
    }

    fn all() -> Vec<Self> {
        vec![
            BindingTypeConfig::UniformBuffer,
            BindingTypeConfig::StorageBuffer { read_only: true },
            BindingTypeConfig::StorageBuffer { read_only: false },
            BindingTypeConfig::Texture,
            BindingTypeConfig::Sampler,
            BindingTypeConfig::StorageTexture,
        ]
    }
}

/// Mock buffer for UI demonstration
#[derive(Debug, Clone)]
struct MockBuffer {
    name: String,
    size: u64,
    usage: String,
}

/// Mock texture for UI demonstration
#[derive(Debug, Clone)]
struct MockTexture {
    name: String,
    format: String,
    dimensions: String,
}

/// Mock sampler for UI demonstration
#[derive(Debug, Clone)]
struct MockSampler {
    name: String,
    filter_mode: String,
}

/// Resource assignment for binding
#[derive(Debug, Clone)]
enum ResourceAssignment {
    Buffer(usize),  // Index into mock_buffers
    Texture(usize), // Index into mock_textures
    Sampler(usize), // Index into mock_samplers
}

impl Default for BindGroupPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl BindGroupPanel {
    /// Create a new bind group panel with default values
    pub fn new() -> Self {
        // Create some mock resources for demonstration
        let mock_buffers = vec![
            MockBuffer {
                name: "Uniform Buffer 0".to_string(),
                size: 256,
                usage: "UNIFORM | COPY_DST".to_string(),
            },
            MockBuffer {
                name: "Storage Buffer 1".to_string(),
                size: 1024,
                usage: "STORAGE | COPY_SRC".to_string(),
            },
            MockBuffer {
                name: "Vertex Buffer".to_string(),
                size: 512,
                usage: "VERTEX | COPY_DST".to_string(),
            },
        ];

        let mock_textures = vec![
            MockTexture {
                name: "Color Texture".to_string(),
                format: "Rgba8Unorm".to_string(),
                dimensions: "256x256".to_string(),
            },
            MockTexture {
                name: "Depth Texture".to_string(),
                format: "Depth32Float".to_string(),
                dimensions: "512x512".to_string(),
            },
        ];

        let mock_samplers = vec![
            MockSampler {
                name: "Linear Sampler".to_string(),
                filter_mode: "Linear".to_string(),
            },
            MockSampler {
                name: "Nearest Sampler".to_string(),
                filter_mode: "Nearest".to_string(),
            },
        ];

        Self {
            layout_label_input: String::new(),
            bind_group_label_input: String::new(),
            layout_entries: Vec::new(),
            next_binding: 0,
            selected_binding: None,
            mock_buffers,
            mock_textures,
            mock_samplers,
            binding_assignments: Vec::new(),
            validation_error: None,
            success_message: None,
            ui_mode: UiMode::CreateLayout,
        }
    }

    /// Add a new binding entry
    fn add_binding_entry(&mut self, binding_type: BindingTypeConfig) {
        let entry = BindGroupLayoutEntryConfig {
            binding: self.next_binding,
            visibility: ShaderStagesConfig::all(),
            binding_type,
        };
        self.layout_entries.push(entry);
        self.next_binding += 1;
        self.validation_error = None;
        self.success_message = None;
    }

    /// Remove a binding entry
    fn remove_binding_entry(&mut self, index: usize) {
        if index < self.layout_entries.len() {
            let binding_num = self.layout_entries[index].binding;
            self.layout_entries.remove(index);
            // Also remove any assignments for this binding
            self.binding_assignments.retain(|(b, _)| *b != binding_num);
            if self.selected_binding == Some(index) {
                self.selected_binding = None;
            }
        }
    }

    /// Validate the bind group layout
    fn validate_layout(&mut self) -> bool {
        if self.layout_entries.is_empty() {
            self.validation_error =
                Some("Bind group layout must have at least one entry".to_string());
            self.success_message = None;
            return false;
        }

        // Check that each binding has at least one shader stage
        for entry in &self.layout_entries {
            let stages = entry.visibility.to_wgpu();
            if stages.is_empty() {
                self.validation_error = Some(format!(
                    "Binding {} must be visible in at least one shader stage",
                    entry.binding
                ));
                self.success_message = None;
                return false;
            }
        }

        self.validation_error = None;
        self.success_message = Some("✓ Layout configuration is valid".to_string());
        true
    }

    /// Validate the bind group bindings
    fn validate_bindings(&mut self) -> bool {
        if self.layout_entries.is_empty() {
            self.validation_error =
                Some("Create a layout first before binding resources".to_string());
            self.success_message = None;
            return false;
        }

        // Check that all bindings are assigned
        for entry in &self.layout_entries {
            let has_assignment = self
                .binding_assignments
                .iter()
                .any(|(b, _)| *b == entry.binding);
            if !has_assignment {
                self.validation_error = Some(format!(
                    "Binding {} has no resource assigned",
                    entry.binding
                ));
                self.success_message = None;
                return false;
            }
        }

        self.validation_error = None;
        self.success_message = Some("✓ All bindings are assigned".to_string());
        true
    }

    /// Render the bind group configuration UI
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("🔗 Bind Group Configuration");
            ui.label("Create bind group layouts and assign resources to binding slots.");
            ui.add_space(10.0);

            // Mode selector
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.ui_mode, UiMode::CreateLayout, "1️⃣ Create Layout");
                ui.selectable_value(
                    &mut self.ui_mode,
                    UiMode::BindResources,
                    "2️⃣ Bind Resources",
                );
            });

            ui.add_space(10.0);

            match self.ui_mode {
                UiMode::CreateLayout => self.render_layout_ui(ui),
                UiMode::BindResources => self.render_binding_ui(ui),
            }

            ui.add_space(10.0);

            // Display validation errors or success messages
            if let Some(error) = &self.validation_error {
                ui.colored_label(egui::Color32::RED, format!("❌ {}", error));
            }

            if let Some(success) = &self.success_message {
                ui.colored_label(egui::Color32::GREEN, success);
            }
        });
    }

    /// Render the layout creation UI
    fn render_layout_ui(&mut self, ui: &mut egui::Ui) {
        // Layout properties
        ui.group(|ui| {
            ui.heading("Bind Group Layout");
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Label:");
                ui.text_edit_singleline(&mut self.layout_label_input);
            });
        });

        ui.add_space(10.0);

        // Binding entries
        ui.group(|ui| {
            ui.heading("Binding Entries");
            ui.label("Define the layout slots for resources:");
            ui.add_space(5.0);

            if self.layout_entries.is_empty() {
                ui.label("No binding entries yet. Add one below.");
            } else {
                // Display current entries in a table
                egui::Grid::new("layout_entries_grid")
                    .num_columns(5)
                    .spacing([10.0, 8.0])
                    .striped(true)
                    .show(ui, |ui| {
                        // Header
                        ui.label("Binding");
                        ui.label("Type");
                        ui.label("Vertex");
                        ui.label("Fragment");
                        ui.label("Compute");
                        ui.label(""); // Actions column
                        ui.end_row();

                        // Entries
                        let mut to_remove = None;
                        for (idx, entry) in self.layout_entries.iter_mut().enumerate() {
                            ui.label(format!("{}", entry.binding));
                            ui.label(entry.binding_type.name());

                            ui.checkbox(&mut entry.visibility.vertex, "");
                            ui.checkbox(&mut entry.visibility.fragment, "");
                            ui.checkbox(&mut entry.visibility.compute, "");

                            if ui.button("🗑").clicked() {
                                to_remove = Some(idx);
                            }
                            ui.end_row();
                        }

                        if let Some(idx) = to_remove {
                            self.remove_binding_entry(idx);
                        }
                    });
            }

            ui.add_space(10.0);

            // Add new binding
            ui.label("Add new binding:");
            ui.horizontal_wrapped(|ui| {
                for binding_type in BindingTypeConfig::all() {
                    if ui.button(binding_type.name()).clicked() {
                        self.add_binding_entry(binding_type);
                    }
                }
            });
        });

        ui.add_space(15.0);

        // Actions
        ui.horizontal(|ui| {
            if ui.button("🔍 Validate Layout").clicked() {
                self.validate_layout();
            }

            if ui.button("🔄 Reset").clicked() {
                self.layout_entries.clear();
                self.binding_assignments.clear();
                self.next_binding = 0;
                self.layout_label_input.clear();
                self.bind_group_label_input.clear();
                self.validation_error = None;
                self.success_message = None;
            }
        });
    }

    /// Render the resource binding UI
    fn render_binding_ui(&mut self, ui: &mut egui::Ui) {
        if self.layout_entries.is_empty() {
            ui.colored_label(
                egui::Color32::from_rgb(200, 150, 50),
                "⚠ Create a bind group layout first in the 'Create Layout' tab",
            );
            return;
        }

        // Bind group properties
        ui.group(|ui| {
            ui.heading("Bind Group");
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Label:");
                ui.text_edit_singleline(&mut self.bind_group_label_input);
            });
        });

        ui.add_space(10.0);

        // Display layout summary
        ui.group(|ui| {
            ui.heading("Layout Summary");
            ui.label(format!(
                "Layout: {}",
                if self.layout_label_input.is_empty() {
                    "<unnamed>"
                } else {
                    &self.layout_label_input
                }
            ));
            ui.label(format!("Bindings: {}", self.layout_entries.len()));
        });

        ui.add_space(10.0);

        // Binding assignments
        ui.group(|ui| {
            ui.heading("Resource Assignments");
            ui.label("Assign resources to each binding slot:");
            ui.add_space(5.0);

            // Clone entries data to avoid borrow checker issues
            let entries: Vec<(u32, BindingTypeConfig)> = self
                .layout_entries
                .iter()
                .map(|e| (e.binding, e.binding_type.clone()))
                .collect();

            for (binding, binding_type) in entries {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(format!("Binding {}:", binding));
                        ui.label(format!("({})", binding_type.name()));
                    });

                    // Find current assignment
                    let current_assignment = self
                        .binding_assignments
                        .iter()
                        .find(|(b, _)| *b == binding)
                        .map(|(_, r)| r);

                    // Display assignment status
                    match current_assignment {
                        Some(ResourceAssignment::Buffer(idx)) => {
                            if let Some(buffer) = self.mock_buffers.get(*idx) {
                                ui.label(format!("Assigned: {}", buffer.name));
                            }
                        }
                        Some(ResourceAssignment::Texture(idx)) => {
                            if let Some(texture) = self.mock_textures.get(*idx) {
                                ui.label(format!("Assigned: {}", texture.name));
                            }
                        }
                        Some(ResourceAssignment::Sampler(idx)) => {
                            if let Some(sampler) = self.mock_samplers.get(*idx) {
                                ui.label(format!("Assigned: {}", sampler.name));
                            }
                        }
                        None => {
                            ui.colored_label(
                                egui::Color32::from_rgb(200, 150, 50),
                                "⚠ Not assigned",
                            );
                        }
                    }

                    // Resource selection based on binding type
                    match &binding_type {
                        BindingTypeConfig::UniformBuffer
                        | BindingTypeConfig::StorageBuffer { .. } => {
                            self.render_buffer_selector(ui, binding);
                        }
                        BindingTypeConfig::Texture | BindingTypeConfig::StorageTexture => {
                            self.render_texture_selector(ui, binding);
                        }
                        BindingTypeConfig::Sampler => {
                            self.render_sampler_selector(ui, binding);
                        }
                    }
                });
                ui.add_space(5.0);
            }
        });

        ui.add_space(15.0);

        // Actions
        ui.horizontal(|ui| {
            if ui.button("🔍 Validate Bindings").clicked() {
                self.validate_bindings();
            }

            if ui.button("✨ Create Bind Group").clicked() && self.validate_bindings() {
                self.success_message = Some(
                    "✓ Configuration is valid. In a full implementation, the bind group would be created here."
                        .to_string(),
                );
            }
        });
    }

    /// Render buffer selector for a binding
    fn render_buffer_selector(&mut self, ui: &mut egui::Ui, binding: u32) {
        ui.label("Available Buffers:");
        for (idx, buffer) in self.mock_buffers.iter().enumerate() {
            if ui.button(&buffer.name).clicked() {
                // Remove any existing assignment for this binding
                self.binding_assignments.retain(|(b, _)| *b != binding);
                // Add new assignment
                self.binding_assignments
                    .push((binding, ResourceAssignment::Buffer(idx)));
                self.validation_error = None;
                self.success_message = None;
            }
            ui.label(format!("  {} bytes, {}", buffer.size, buffer.usage));
        }
    }

    /// Render texture selector for a binding
    fn render_texture_selector(&mut self, ui: &mut egui::Ui, binding: u32) {
        ui.label("Available Textures:");
        for (idx, texture) in self.mock_textures.iter().enumerate() {
            if ui.button(&texture.name).clicked() {
                // Remove any existing assignment for this binding
                self.binding_assignments.retain(|(b, _)| *b != binding);
                // Add new assignment
                self.binding_assignments
                    .push((binding, ResourceAssignment::Texture(idx)));
                self.validation_error = None;
                self.success_message = None;
            }
            ui.label(format!("  {}, {}", texture.format, texture.dimensions));
        }
    }

    /// Render sampler selector for a binding
    fn render_sampler_selector(&mut self, ui: &mut egui::Ui, binding: u32) {
        ui.label("Available Samplers:");
        for (idx, sampler) in self.mock_samplers.iter().enumerate() {
            if ui.button(&sampler.name).clicked() {
                // Remove any existing assignment for this binding
                self.binding_assignments.retain(|(b, _)| *b != binding);
                // Add new assignment
                self.binding_assignments
                    .push((binding, ResourceAssignment::Sampler(idx)));
                self.validation_error = None;
                self.success_message = None;
            }
            ui.label(format!("  Filter: {}", sampler.filter_mode));
        }
    }

    /// Get the current bind group layout descriptor
    pub fn get_layout_descriptor(&self) -> Option<BindGroupLayoutDescriptor> {
        if self.layout_entries.is_empty() {
            return None;
        }

        let label = if self.layout_label_input.is_empty() {
            None
        } else {
            Some(self.layout_label_input.as_str())
        };

        let mut descriptor = BindGroupLayoutDescriptor::new(label);
        for entry in &self.layout_entries {
            descriptor = descriptor.with_entry(BindGroupLayoutEntry::new(
                entry.binding,
                entry.visibility.to_wgpu(),
                entry.binding_type.to_binding_type(),
            ));
        }

        Some(descriptor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bind_group_panel_creation() {
        let panel = BindGroupPanel::new();
        assert_eq!(panel.layout_entries.len(), 0);
        assert_eq!(panel.next_binding, 0);
        assert!(panel.mock_buffers.len() > 0);
        assert!(panel.mock_textures.len() > 0);
        assert!(panel.mock_samplers.len() > 0);
    }

    #[test]
    fn test_add_binding_entry() {
        let mut panel = BindGroupPanel::new();
        panel.add_binding_entry(BindingTypeConfig::UniformBuffer);
        assert_eq!(panel.layout_entries.len(), 1);
        assert_eq!(panel.next_binding, 1);
        assert_eq!(panel.layout_entries[0].binding, 0);
    }

    #[test]
    fn test_remove_binding_entry() {
        let mut panel = BindGroupPanel::new();
        panel.add_binding_entry(BindingTypeConfig::UniformBuffer);
        panel.add_binding_entry(BindingTypeConfig::Texture);
        assert_eq!(panel.layout_entries.len(), 2);

        panel.remove_binding_entry(0);
        assert_eq!(panel.layout_entries.len(), 1);
        assert_eq!(
            panel.layout_entries[0].binding_type,
            BindingTypeConfig::Texture
        );
    }

    #[test]
    fn test_validate_layout_empty() {
        let mut panel = BindGroupPanel::new();
        assert!(!panel.validate_layout());
        assert!(panel.validation_error.is_some());
    }

    #[test]
    fn test_validate_layout_valid() {
        let mut panel = BindGroupPanel::new();
        panel.add_binding_entry(BindingTypeConfig::UniformBuffer);
        assert!(panel.validate_layout());
        assert!(panel.validation_error.is_none());
        assert!(panel.success_message.is_some());
    }

    #[test]
    fn test_shader_stages_config_to_wgpu() {
        let config = ShaderStagesConfig {
            vertex: true,
            fragment: true,
            compute: false,
        };
        let stages = config.to_wgpu();
        assert!(stages.contains(ShaderStages::VERTEX));
        assert!(stages.contains(ShaderStages::FRAGMENT));
        assert!(!stages.contains(ShaderStages::COMPUTE));
    }

    #[test]
    fn test_binding_type_config_names() {
        assert_eq!(BindingTypeConfig::UniformBuffer.name(), "Uniform Buffer");
        assert_eq!(
            BindingTypeConfig::StorageBuffer { read_only: true }.name(),
            "Storage Buffer (Read-Only)"
        );
        assert_eq!(BindingTypeConfig::Texture.name(), "Texture");
        assert_eq!(BindingTypeConfig::Sampler.name(), "Sampler");
    }

    #[test]
    fn test_get_layout_descriptor_empty() {
        let panel = BindGroupPanel::new();
        assert!(panel.get_layout_descriptor().is_none());
    }

    #[test]
    fn test_get_layout_descriptor_with_entries() {
        let mut panel = BindGroupPanel::new();
        panel.layout_label_input = "test_layout".to_string();
        panel.add_binding_entry(BindingTypeConfig::UniformBuffer);

        let descriptor = panel.get_layout_descriptor();
        assert!(descriptor.is_some());
        let descriptor = descriptor.unwrap();
        assert_eq!(descriptor.label(), Some("test_layout"));
        assert_eq!(descriptor.entries().len(), 1);
    }

    #[test]
    fn test_validate_bindings_no_layout() {
        let mut panel = BindGroupPanel::new();
        assert!(!panel.validate_bindings());
        assert!(panel.validation_error.is_some());
    }

    #[test]
    fn test_resource_assignment() {
        let mut panel = BindGroupPanel::new();
        panel.add_binding_entry(BindingTypeConfig::UniformBuffer);

        // Assign a buffer to binding 0
        panel
            .binding_assignments
            .push((0, ResourceAssignment::Buffer(0)));

        assert_eq!(panel.binding_assignments.len(), 1);
        assert!(panel.validate_bindings());
    }

    #[test]
    fn test_ui_mode_switching() {
        let mut panel = BindGroupPanel::new();
        assert_eq!(panel.ui_mode, UiMode::CreateLayout);

        panel.ui_mode = UiMode::BindResources;
        assert_eq!(panel.ui_mode, UiMode::BindResources);
    }
}
