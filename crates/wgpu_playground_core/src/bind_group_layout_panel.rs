use crate::bind_group::{
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, SamplerBindingType,
    StorageTextureAccess, TextureSampleType, TextureViewDimension,
};
use wgpu::ShaderStages;

/// Represents a single bind group layout entry being configured in the UI
#[derive(Debug, Clone)]
struct EntryConfig {
    /// Binding number
    binding_number: String,
    /// Visibility flags
    visibility_vertex: bool,
    visibility_fragment: bool,
    visibility_compute: bool,
    /// Resource type selection
    resource_type: ResourceTypeConfig,
}

/// Configuration for different resource types
#[derive(Debug, Clone, PartialEq)]
enum ResourceTypeConfig {
    UniformBuffer {
        has_dynamic_offset: bool,
        min_binding_size: String,
    },
    StorageBuffer {
        has_dynamic_offset: bool,
        min_binding_size: String,
        read_only: bool,
    },
    Texture {
        sample_type: TextureSampleTypeConfig,
        view_dimension: TextureViewDimension,
        multisampled: bool,
    },
    Sampler {
        sampler_type: SamplerBindingType,
    },
    StorageTexture {
        access: StorageTextureAccess,
        format: wgpu::TextureFormat,
        view_dimension: TextureViewDimension,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TextureSampleTypeConfig {
    FloatFilterable,
    FloatNonFilterable,
    Sint,
    Uint,
    Depth,
}

impl Default for EntryConfig {
    fn default() -> Self {
        Self {
            binding_number: "0".to_string(),
            visibility_vertex: true,
            visibility_fragment: false,
            visibility_compute: false,
            resource_type: ResourceTypeConfig::UniformBuffer {
                has_dynamic_offset: false,
                min_binding_size: String::new(),
            },
        }
    }
}

impl EntryConfig {
    /// Parse the entry configuration and create a BindGroupLayoutEntry
    fn to_bind_group_layout_entry(&self) -> Result<BindGroupLayoutEntry, String> {
        let binding = self
            .binding_number
            .parse::<u32>()
            .map_err(|_| "Binding number must be a valid unsigned integer")?;

        let mut visibility = ShaderStages::empty();
        if self.visibility_vertex {
            visibility |= ShaderStages::VERTEX;
        }
        if self.visibility_fragment {
            visibility |= ShaderStages::FRAGMENT;
        }
        if self.visibility_compute {
            visibility |= ShaderStages::COMPUTE;
        }

        if visibility.is_empty() {
            return Err("At least one visibility flag must be selected".to_string());
        }

        let binding_type = match &self.resource_type {
            ResourceTypeConfig::UniformBuffer {
                has_dynamic_offset,
                min_binding_size,
            } => {
                let min_size = if min_binding_size.is_empty() {
                    None
                } else {
                    Some(
                        min_binding_size
                            .parse::<u64>()
                            .map_err(|_| "Min binding size must be a valid number")?
                            .try_into()
                            .map_err(|_| "Min binding size must be greater than 0")?,
                    )
                };
                BindingType::UniformBuffer {
                    has_dynamic_offset: *has_dynamic_offset,
                    min_binding_size: min_size,
                }
            }
            ResourceTypeConfig::StorageBuffer {
                has_dynamic_offset,
                min_binding_size,
                read_only,
            } => {
                let min_size = if min_binding_size.is_empty() {
                    None
                } else {
                    Some(
                        min_binding_size
                            .parse::<u64>()
                            .map_err(|_| "Min binding size must be a valid number")?
                            .try_into()
                            .map_err(|_| "Min binding size must be greater than 0")?,
                    )
                };
                BindingType::StorageBuffer {
                    has_dynamic_offset: *has_dynamic_offset,
                    min_binding_size: min_size,
                    read_only: *read_only,
                }
            }
            ResourceTypeConfig::Texture {
                sample_type,
                view_dimension,
                multisampled,
            } => {
                let sample_type = match sample_type {
                    TextureSampleTypeConfig::FloatFilterable => {
                        TextureSampleType::Float { filterable: true }
                    }
                    TextureSampleTypeConfig::FloatNonFilterable => {
                        TextureSampleType::Float { filterable: false }
                    }
                    TextureSampleTypeConfig::Sint => TextureSampleType::Sint,
                    TextureSampleTypeConfig::Uint => TextureSampleType::Uint,
                    TextureSampleTypeConfig::Depth => TextureSampleType::Depth,
                };
                BindingType::Texture {
                    sample_type,
                    view_dimension: *view_dimension,
                    multisampled: *multisampled,
                }
            }
            ResourceTypeConfig::Sampler { sampler_type } => BindingType::Sampler {
                sampler_type: *sampler_type,
            },
            ResourceTypeConfig::StorageTexture {
                access,
                format,
                view_dimension,
            } => BindingType::StorageTexture {
                access: *access,
                format: *format,
                view_dimension: *view_dimension,
            },
        };

        Ok(BindGroupLayoutEntry::new(binding, visibility, binding_type))
    }
}

/// UI panel for creating and configuring bind group layouts
pub struct BindGroupLayoutPanel {
    /// Current bind group layout descriptor being configured
    descriptor: BindGroupLayoutDescriptor,
    /// Label input text
    label_input: String,
    /// List of entry configurations
    entries: Vec<EntryConfig>,
    /// Validation error message
    validation_error: Option<String>,
    /// Success message
    success_message: Option<String>,
}

impl Default for BindGroupLayoutPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl BindGroupLayoutPanel {
    /// Create a new bind group layout panel with default values
    pub fn new() -> Self {
        Self {
            descriptor: BindGroupLayoutDescriptor::default(),
            label_input: String::new(),
            entries: vec![EntryConfig::default()],
            validation_error: None,
            success_message: None,
        }
    }

    /// Add a new entry to the configuration
    fn add_entry(&mut self) {
        self.entries.push(EntryConfig::default());
    }

    /// Remove an entry at the specified index
    fn remove_entry(&mut self, index: usize) {
        if self.entries.len() > 1 {
            self.entries.remove(index);
        }
    }

    /// Update the internal descriptor based on current UI state
    fn update_descriptor(&mut self) -> Result<(), String> {
        let label = if self.label_input.is_empty() {
            None
        } else {
            Some(self.label_input.as_str())
        };

        let mut descriptor = BindGroupLayoutDescriptor::new(label);

        for entry in &self.entries {
            let layout_entry = entry.to_bind_group_layout_entry()?;
            descriptor = descriptor.with_entry(layout_entry);
        }

        self.descriptor = descriptor;
        Ok(())
    }

    /// Validate the current configuration
    fn validate(&mut self) -> bool {
        match self.update_descriptor() {
            Ok(()) => match self.descriptor.validate() {
                Ok(()) => {
                    self.validation_error = None;
                    true
                }
                Err(e) => {
                    self.validation_error = Some(e.to_string());
                    self.success_message = None;
                    false
                }
            },
            Err(e) => {
                self.validation_error = Some(e);
                self.success_message = None;
                false
            }
        }
    }

    /// Render the bind group layout configuration UI
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("🔗 Bind Group Layout Configuration");
            ui.label("Configure bind group layouts with dynamic entry addition.");
            ui.add_space(10.0);

            // Layout Label
            ui.group(|ui| {
                ui.heading("Layout Properties");
                ui.add_space(5.0);

                egui::Grid::new("layout_properties")
                    .num_columns(2)
                    .spacing([10.0, 8.0])
                    .show(ui, |ui| {
                        ui.label("Label:");
                        ui.text_edit_singleline(&mut self.label_input);
                        ui.end_row();
                    });
            });

            ui.add_space(10.0);

            // Entries
            ui.group(|ui| {
                ui.heading("Bind Group Layout Entries");
                ui.label("Configure each binding entry in the layout:");
                ui.add_space(10.0);

                let mut entry_to_remove: Option<usize> = None;
                let entries_len = self.entries.len();

                for (index, entry) in self.entries.iter_mut().enumerate() {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.heading(format!("Entry {}", index));
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if entries_len > 1 && ui.button("🗑 Remove").clicked() {
                                    entry_to_remove = Some(index);
                                }
                            });
                        });

                        ui.add_space(5.0);

                        // Binding number
                        egui::Grid::new(format!("entry_{}_binding", index))
                            .num_columns(2)
                            .spacing([10.0, 8.0])
                            .show(ui, |ui| {
                                ui.label("Binding Number:");
                                ui.text_edit_singleline(&mut entry.binding_number);
                                ui.end_row();
                            });

                        ui.add_space(5.0);

                        // Visibility flags
                        ui.label("Visibility:");
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut entry.visibility_vertex, "Vertex")
                                .on_hover_text("Visible in vertex shaders");
                            ui.checkbox(&mut entry.visibility_fragment, "Fragment")
                                .on_hover_text("Visible in fragment shaders");
                            ui.checkbox(&mut entry.visibility_compute, "Compute")
                                .on_hover_text("Visible in compute shaders");
                        });

                        ui.add_space(5.0);

                        // Resource Type
                        ui.label("Resource Type:");
                        Self::render_resource_type_config(ui, entry, index);
                    });

                    ui.add_space(5.0);
                }

                if let Some(index) = entry_to_remove {
                    self.remove_entry(index);
                }

                ui.add_space(10.0);

                if ui.button("➕ Add Entry").clicked() {
                    self.add_entry();
                }
            });

            ui.add_space(15.0);

            // Validation and Creation
            ui.horizontal(|ui| {
                if ui.button("🔍 Validate").clicked() && self.validate() {
                    self.success_message = Some("✓ Configuration is valid".to_string());
                }

                if ui.button("✨ Create Layout").clicked() && self.validate() {
                    self.success_message = Some(
                        "✓ Configuration is valid. In a full implementation, the layout would be created here."
                            .to_string(),
                    );
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

            // Configuration Summary
            if self.update_descriptor().is_ok() {
                ui.group(|ui| {
                    ui.heading("Configuration Summary");
                    ui.add_space(5.0);

                    ui.monospace(format!(
                        "Label: {}",
                        self.descriptor.label().unwrap_or("<none>")
                    ));
                    ui.monospace(format!("Entries: {}", self.descriptor.entries().len()));

                    ui.add_space(5.0);

                    for (i, entry) in self.descriptor.entries().iter().enumerate() {
                        ui.label(format!("Entry {}:", i));
                        ui.monospace(format!("  Binding: {}", entry.binding));
                        ui.monospace(format!("  Visibility: {:?}", entry.visibility));
                        ui.monospace(format!("  Type: {}", Self::binding_type_display(&entry.ty)));
                    }
                });
            }
        });
    }

    fn render_resource_type_config(ui: &mut egui::Ui, entry: &mut EntryConfig, index: usize) {
        // Determine current resource type for the combo box
        let current_type = match &entry.resource_type {
            ResourceTypeConfig::UniformBuffer { .. } => "Uniform Buffer",
            ResourceTypeConfig::StorageBuffer { .. } => "Storage Buffer",
            ResourceTypeConfig::Texture { .. } => "Texture",
            ResourceTypeConfig::Sampler { .. } => "Sampler",
            ResourceTypeConfig::StorageTexture { .. } => "Storage Texture",
        };

        egui::ComboBox::from_id_salt(format!("resource_type_{}", index))
            .selected_text(current_type)
            .show_ui(ui, |ui| {
                if ui.selectable_label(current_type == "Uniform Buffer", "Uniform Buffer").clicked() {
                    entry.resource_type = ResourceTypeConfig::UniformBuffer {
                        has_dynamic_offset: false,
                        min_binding_size: String::new(),
                    };
                }
                if ui.selectable_label(current_type == "Storage Buffer", "Storage Buffer").clicked() {
                    entry.resource_type = ResourceTypeConfig::StorageBuffer {
                        has_dynamic_offset: false,
                        min_binding_size: String::new(),
                        read_only: false,
                    };
                }
                if ui.selectable_label(current_type == "Texture", "Texture").clicked() {
                    entry.resource_type = ResourceTypeConfig::Texture {
                        sample_type: TextureSampleTypeConfig::FloatFilterable,
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    };
                }
                if ui.selectable_label(current_type == "Sampler", "Sampler").clicked() {
                    entry.resource_type = ResourceTypeConfig::Sampler {
                        sampler_type: SamplerBindingType::Filtering,
                    };
                }
                if ui.selectable_label(current_type == "Storage Texture", "Storage Texture").clicked() {
                    entry.resource_type = ResourceTypeConfig::StorageTexture {
                        access: StorageTextureAccess::WriteOnly,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        view_dimension: TextureViewDimension::D2,
                    };
                }
            });

        ui.add_space(5.0);

        // Render specific configuration for each resource type
        match &mut entry.resource_type {
            ResourceTypeConfig::UniformBuffer {
                has_dynamic_offset,
                min_binding_size,
            } => {
                ui.checkbox(has_dynamic_offset, "Has Dynamic Offset")
                    .on_hover_text("Enable dynamic offsets for this buffer");
                ui.horizontal(|ui| {
                    ui.label("Min Binding Size:");
                    ui.text_edit_singleline(min_binding_size)
                        .on_hover_text("Minimum size in bytes (optional)");
                });
            }
            ResourceTypeConfig::StorageBuffer {
                has_dynamic_offset,
                min_binding_size,
                read_only,
            } => {
                ui.checkbox(has_dynamic_offset, "Has Dynamic Offset")
                    .on_hover_text("Enable dynamic offsets for this buffer");
                ui.checkbox(read_only, "Read Only")
                    .on_hover_text("Buffer is read-only in shaders");
                ui.horizontal(|ui| {
                    ui.label("Min Binding Size:");
                    ui.text_edit_singleline(min_binding_size)
                        .on_hover_text("Minimum size in bytes (optional)");
                });
            }
            ResourceTypeConfig::Texture {
                sample_type,
                view_dimension,
                multisampled,
            } => {
                ui.horizontal(|ui| {
                    ui.label("Sample Type:");
                    egui::ComboBox::from_id_salt(format!("sample_type_{}", index))
                        .selected_text(format!("{:?}", sample_type))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                sample_type,
                                TextureSampleTypeConfig::FloatFilterable,
                                "Float (Filterable)",
                            );
                            ui.selectable_value(
                                sample_type,
                                TextureSampleTypeConfig::FloatNonFilterable,
                                "Float (Non-Filterable)",
                            );
                            ui.selectable_value(sample_type, TextureSampleTypeConfig::Sint, "Sint");
                            ui.selectable_value(sample_type, TextureSampleTypeConfig::Uint, "Uint");
                            ui.selectable_value(sample_type, TextureSampleTypeConfig::Depth, "Depth");
                        });
                });
                ui.horizontal(|ui| {
                    ui.label("View Dimension:");
                    egui::ComboBox::from_id_salt(format!("view_dim_{}", index))
                        .selected_text(format!("{:?}", view_dimension))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(view_dimension, TextureViewDimension::D1, "1D");
                            ui.selectable_value(view_dimension, TextureViewDimension::D2, "2D");
                            ui.selectable_value(view_dimension, TextureViewDimension::D2Array, "2D Array");
                            ui.selectable_value(view_dimension, TextureViewDimension::Cube, "Cube");
                            ui.selectable_value(view_dimension, TextureViewDimension::CubeArray, "Cube Array");
                            ui.selectable_value(view_dimension, TextureViewDimension::D3, "3D");
                        });
                });
                ui.checkbox(multisampled, "Multisampled")
                    .on_hover_text("Texture uses multisampling");
            }
            ResourceTypeConfig::Sampler { sampler_type } => {
                ui.horizontal(|ui| {
                    ui.label("Sampler Type:");
                    egui::ComboBox::from_id_salt(format!("sampler_type_{}", index))
                        .selected_text(format!("{:?}", sampler_type))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                sampler_type,
                                SamplerBindingType::Filtering,
                                "Filtering",
                            );
                            ui.selectable_value(
                                sampler_type,
                                SamplerBindingType::NonFiltering,
                                "Non-Filtering",
                            );
                            ui.selectable_value(
                                sampler_type,
                                SamplerBindingType::Comparison,
                                "Comparison",
                            );
                        });
                });
            }
            ResourceTypeConfig::StorageTexture {
                access,
                format,
                view_dimension,
            } => {
                ui.horizontal(|ui| {
                    ui.label("Access:");
                    egui::ComboBox::from_id_salt(format!("access_{}", index))
                        .selected_text(format!("{:?}", access))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                access,
                                StorageTextureAccess::WriteOnly,
                                "Write Only",
                            );
                            ui.selectable_value(
                                access,
                                StorageTextureAccess::ReadOnly,
                                "Read Only",
                            );
                            ui.selectable_value(
                                access,
                                StorageTextureAccess::ReadWrite,
                                "Read Write",
                            );
                        });
                });
                ui.horizontal(|ui| {
                    ui.label("Format:");
                    egui::ComboBox::from_id_salt(format!("format_{}", index))
                        .selected_text(format!("{:?}", format))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(format, wgpu::TextureFormat::Rgba8Unorm, "Rgba8Unorm");
                            ui.selectable_value(format, wgpu::TextureFormat::Rgba16Float, "Rgba16Float");
                            ui.selectable_value(format, wgpu::TextureFormat::Rgba32Float, "Rgba32Float");
                            ui.selectable_value(format, wgpu::TextureFormat::R32Float, "R32Float");
                            ui.selectable_value(format, wgpu::TextureFormat::R32Uint, "R32Uint");
                            ui.selectable_value(format, wgpu::TextureFormat::R32Sint, "R32Sint");
                        });
                });
                ui.horizontal(|ui| {
                    ui.label("View Dimension:");
                    egui::ComboBox::from_id_salt(format!("storage_view_dim_{}", index))
                        .selected_text(format!("{:?}", view_dimension))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(view_dimension, TextureViewDimension::D1, "1D");
                            ui.selectable_value(view_dimension, TextureViewDimension::D2, "2D");
                            ui.selectable_value(view_dimension, TextureViewDimension::D2Array, "2D Array");
                            ui.selectable_value(view_dimension, TextureViewDimension::Cube, "Cube");
                            ui.selectable_value(view_dimension, TextureViewDimension::CubeArray, "Cube Array");
                            ui.selectable_value(view_dimension, TextureViewDimension::D3, "3D");
                        });
                });
            }
        }
    }

    fn binding_type_display(binding_type: &BindingType) -> String {
        match binding_type {
            BindingType::UniformBuffer {
                has_dynamic_offset,
                min_binding_size,
            } => {
                let mut desc = "Uniform Buffer".to_string();
                if *has_dynamic_offset {
                    desc.push_str(" (dynamic)");
                }
                if let Some(size) = min_binding_size {
                    desc.push_str(&format!(" (min: {})", size));
                }
                desc
            }
            BindingType::StorageBuffer {
                has_dynamic_offset,
                min_binding_size,
                read_only,
            } => {
                let mut desc = "Storage Buffer".to_string();
                if *read_only {
                    desc.push_str(" (read-only)");
                }
                if *has_dynamic_offset {
                    desc.push_str(" (dynamic)");
                }
                if let Some(size) = min_binding_size {
                    desc.push_str(&format!(" (min: {})", size));
                }
                desc
            }
            BindingType::Texture {
                sample_type,
                view_dimension,
                multisampled,
            } => {
                let mut desc = format!("Texture ({:?}, {:?})", sample_type, view_dimension);
                if *multisampled {
                    desc.push_str(" (multisampled)");
                }
                desc
            }
            BindingType::Sampler { sampler_type } => {
                format!("Sampler ({:?})", sampler_type)
            }
            BindingType::StorageTexture {
                access,
                format,
                view_dimension,
            } => {
                format!(
                    "Storage Texture ({:?}, {:?}, {:?})",
                    access, format, view_dimension
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panel_creation() {
        let panel = BindGroupLayoutPanel::new();
        assert_eq!(panel.label_input, "");
        assert_eq!(panel.entries.len(), 1);
        assert!(panel.validation_error.is_none());
        assert!(panel.success_message.is_none());
    }

    #[test]
    fn test_panel_default() {
        let panel = BindGroupLayoutPanel::default();
        assert_eq!(panel.entries.len(), 1);
    }

    #[test]
    fn test_add_entry() {
        let mut panel = BindGroupLayoutPanel::new();
        panel.add_entry();
        assert_eq!(panel.entries.len(), 2);
    }

    #[test]
    fn test_remove_entry() {
        let mut panel = BindGroupLayoutPanel::new();
        panel.add_entry();
        panel.add_entry();
        assert_eq!(panel.entries.len(), 3);
        panel.remove_entry(1);
        assert_eq!(panel.entries.len(), 2);
    }

    #[test]
    fn test_remove_last_entry_keeps_one() {
        let mut panel = BindGroupLayoutPanel::new();
        assert_eq!(panel.entries.len(), 1);
        panel.remove_entry(0);
        assert_eq!(panel.entries.len(), 1); // Should keep at least one entry
    }

    #[test]
    fn test_entry_config_default() {
        let entry = EntryConfig::default();
        assert_eq!(entry.binding_number, "0");
        assert!(entry.visibility_vertex);
        assert!(!entry.visibility_fragment);
        assert!(!entry.visibility_compute);
    }

    #[test]
    fn test_entry_to_bind_group_layout_entry_success() {
        let entry = EntryConfig {
            binding_number: "1".to_string(),
            visibility_vertex: true,
            visibility_fragment: true,
            visibility_compute: false,
            resource_type: ResourceTypeConfig::UniformBuffer {
                has_dynamic_offset: false,
                min_binding_size: String::new(),
            },
        };

        let result = entry.to_bind_group_layout_entry();
        assert!(result.is_ok());
        let layout_entry = result.unwrap();
        assert_eq!(layout_entry.binding, 1);
        assert!(layout_entry.visibility.contains(ShaderStages::VERTEX));
        assert!(layout_entry.visibility.contains(ShaderStages::FRAGMENT));
        assert!(!layout_entry.visibility.contains(ShaderStages::COMPUTE));
    }

    #[test]
    fn test_entry_no_visibility_fails() {
        let entry = EntryConfig {
            binding_number: "0".to_string(),
            visibility_vertex: false,
            visibility_fragment: false,
            visibility_compute: false,
            resource_type: ResourceTypeConfig::UniformBuffer {
                has_dynamic_offset: false,
                min_binding_size: String::new(),
            },
        };

        let result = entry.to_bind_group_layout_entry();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("At least one visibility flag"));
    }

    #[test]
    fn test_entry_invalid_binding_number() {
        let entry = EntryConfig {
            binding_number: "invalid".to_string(),
            visibility_vertex: true,
            visibility_fragment: false,
            visibility_compute: false,
            resource_type: ResourceTypeConfig::UniformBuffer {
                has_dynamic_offset: false,
                min_binding_size: String::new(),
            },
        };

        let result = entry.to_bind_group_layout_entry();
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_success() {
        let mut panel = BindGroupLayoutPanel::new();
        panel.entries[0].binding_number = "0".to_string();
        panel.entries[0].visibility_vertex = true;

        let valid = panel.validate();
        assert!(valid);
        assert!(panel.validation_error.is_none());
    }

    #[test]
    fn test_validate_duplicate_bindings() {
        let mut panel = BindGroupLayoutPanel::new();
        panel.add_entry();
        panel.entries[0].binding_number = "0".to_string();
        panel.entries[0].visibility_vertex = true;
        panel.entries[1].binding_number = "0".to_string(); // Duplicate
        panel.entries[1].visibility_fragment = true;

        let valid = panel.validate();
        assert!(!valid);
        assert!(panel.validation_error.is_some());
    }

    #[test]
    fn test_storage_buffer_resource_type() {
        let entry = EntryConfig {
            binding_number: "0".to_string(),
            visibility_compute: true,
            visibility_vertex: false,
            visibility_fragment: false,
            resource_type: ResourceTypeConfig::StorageBuffer {
                has_dynamic_offset: false,
                min_binding_size: String::new(),
                read_only: true,
            },
        };

        let result = entry.to_bind_group_layout_entry();
        assert!(result.is_ok());
    }

    #[test]
    fn test_texture_resource_type() {
        let entry = EntryConfig {
            binding_number: "0".to_string(),
            visibility_fragment: true,
            visibility_vertex: false,
            visibility_compute: false,
            resource_type: ResourceTypeConfig::Texture {
                sample_type: TextureSampleTypeConfig::FloatFilterable,
                view_dimension: TextureViewDimension::D2,
                multisampled: false,
            },
        };

        let result = entry.to_bind_group_layout_entry();
        assert!(result.is_ok());
    }

    #[test]
    fn test_sampler_resource_type() {
        let entry = EntryConfig {
            binding_number: "0".to_string(),
            visibility_fragment: true,
            visibility_vertex: false,
            visibility_compute: false,
            resource_type: ResourceTypeConfig::Sampler {
                sampler_type: SamplerBindingType::Filtering,
            },
        };

        let result = entry.to_bind_group_layout_entry();
        assert!(result.is_ok());
    }

    #[test]
    fn test_storage_texture_resource_type() {
        let entry = EntryConfig {
            binding_number: "0".to_string(),
            visibility_compute: true,
            visibility_vertex: false,
            visibility_fragment: false,
            resource_type: ResourceTypeConfig::StorageTexture {
                access: StorageTextureAccess::WriteOnly,
                format: wgpu::TextureFormat::Rgba8Unorm,
                view_dimension: TextureViewDimension::D2,
            },
        };

        let result = entry.to_bind_group_layout_entry();
        assert!(result.is_ok());
    }
}
