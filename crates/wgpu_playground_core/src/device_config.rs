use wgpu::{Features, Limits};

/// Configuration for device creation
#[derive(Debug, Clone)]
pub struct DeviceConfig {
    pub features: Features,
    pub limits: Limits,
}

impl Default for DeviceConfig {
    fn default() -> Self {
        Self {
            features: Features::empty(),
            limits: Limits::default(),
        }
    }
}

impl DeviceConfig {
    /// Create a new device configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Create configuration from adapter capabilities
    pub fn from_adapter(adapter: &wgpu::Adapter) -> Self {
        Self {
            features: adapter.features(),
            limits: adapter.limits(),
        }
    }

    /// Set a specific feature
    pub fn set_feature(&mut self, feature: Features, enabled: bool) {
        if enabled {
            self.features.insert(feature);
        } else {
            self.features.remove(feature);
        }
    }

    /// Check if a feature is enabled
    pub fn has_feature(&self, feature: Features) -> bool {
        self.features.contains(feature)
    }
}

/// UI panel for configuring device features and limits before device creation
pub struct DeviceConfigPanel {
    config: DeviceConfig,
    adapter_features: Features,
    adapter_limits: Limits,
}

impl DeviceConfigPanel {
    /// Create a new device configuration panel
    pub fn new(adapter: &wgpu::Adapter) -> Self {
        Self {
            config: DeviceConfig::default(),
            adapter_features: adapter.features(),
            adapter_limits: adapter.limits(),
        }
    }

    /// Get the current device configuration
    pub fn get_config(&self) -> &DeviceConfig {
        &self.config
    }

    /// Render the configuration UI
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("⚙️ Device Configuration");
            ui.label("Configure features and limits before device creation.");
            ui.add_space(10.0);

            // Features section
            ui.heading("Available Features");
            ui.separator();
            ui.label("Enable or disable WebGPU features for your device:");
            ui.add_space(5.0);

            self.render_features_ui(ui);
            ui.add_space(20.0);

            // Limits section
            ui.heading("Device Limits");
            ui.separator();
            ui.label("Adjust device limits (values cannot exceed adapter capabilities):");
            ui.add_space(5.0);

            self.render_limits_ui(ui);
        });
    }

    fn render_features_ui(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("features_grid")
            .num_columns(2)
            .spacing([10.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                // List all available features
                self.render_feature_checkbox(ui, "Depth Clip Control", Features::DEPTH_CLIP_CONTROL);
                self.render_feature_checkbox(ui, "Depth32Float Stencil8", Features::DEPTH32FLOAT_STENCIL8);
                self.render_feature_checkbox(ui, "Timestamp Query", Features::TIMESTAMP_QUERY);
                self.render_feature_checkbox(ui, "Pipeline Statistics Query", Features::PIPELINE_STATISTICS_QUERY);
                self.render_feature_checkbox(ui, "Texture Compression BC", Features::TEXTURE_COMPRESSION_BC);
                self.render_feature_checkbox(ui, "Texture Compression ETC2", Features::TEXTURE_COMPRESSION_ETC2);
                self.render_feature_checkbox(ui, "Texture Compression ASTC", Features::TEXTURE_COMPRESSION_ASTC);
                self.render_feature_checkbox(ui, "Indirect First Instance", Features::INDIRECT_FIRST_INSTANCE);
                self.render_feature_checkbox(ui, "Shader F16", Features::SHADER_F16);
                self.render_feature_checkbox(ui, "RG11B10UFloat Renderable", Features::RG11B10UFLOAT_RENDERABLE);
                self.render_feature_checkbox(ui, "BGRA8UnormStorage", Features::BGRA8UNORM_STORAGE);
                self.render_feature_checkbox(ui, "Float32 Filterable", Features::FLOAT32_FILTERABLE);
                self.render_feature_checkbox(ui, "Shader Primitive Index", Features::SHADER_PRIMITIVE_INDEX);
            });
    }

    fn render_feature_checkbox(&mut self, ui: &mut egui::Ui, label: &str, feature: Features) {
        let adapter_supports = self.adapter_features.contains(feature);
        let mut enabled = self.config.has_feature(feature);

        ui.add_enabled_ui(adapter_supports, |ui| {
            if ui.checkbox(&mut enabled, label).changed() {
                self.config.set_feature(feature, enabled);
            }
        });

        if adapter_supports {
            ui.label("✓ Supported");
        } else {
            ui.label("✗ Not supported");
        }
        ui.end_row();
    }

    fn render_limits_ui(&mut self, ui: &mut egui::Ui) {
        ui.label("💡 Tip: Lower values may improve compatibility but reduce capabilities.");
        ui.add_space(5.0);

        egui::Grid::new("limits_grid")
            .num_columns(3)
            .spacing([10.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Limit");
                ui.label("Current Value");
                ui.label("Max Available");
                ui.end_row();

                // Texture limits
                ui.label("Max Texture Dimension 1D");
                ui.label(self.config.limits.max_texture_dimension_1d.to_string());
                ui.label(self.adapter_limits.max_texture_dimension_1d.to_string());
                ui.end_row();

                ui.label("Max Texture Dimension 2D");
                ui.label(self.config.limits.max_texture_dimension_2d.to_string());
                ui.label(self.adapter_limits.max_texture_dimension_2d.to_string());
                ui.end_row();

                ui.label("Max Texture Dimension 3D");
                ui.label(self.config.limits.max_texture_dimension_3d.to_string());
                ui.label(self.adapter_limits.max_texture_dimension_3d.to_string());
                ui.end_row();

                ui.label("Max Texture Array Layers");
                ui.label(self.config.limits.max_texture_array_layers.to_string());
                ui.label(self.adapter_limits.max_texture_array_layers.to_string());
                ui.end_row();

                // Bind group limits
                ui.label("Max Bind Groups");
                ui.label(self.config.limits.max_bind_groups.to_string());
                ui.label(self.adapter_limits.max_bind_groups.to_string());
                ui.end_row();

                ui.label("Max Bindings Per Bind Group");
                ui.label(self.config.limits.max_bindings_per_bind_group.to_string());
                ui.label(self.adapter_limits.max_bindings_per_bind_group.to_string());
                ui.end_row();

                // Buffer limits
                ui.label("Max Uniform Buffer Binding Size");
                ui.label(self.config.limits.max_uniform_buffer_binding_size.to_string());
                ui.label(self.adapter_limits.max_uniform_buffer_binding_size.to_string());
                ui.end_row();

                ui.label("Max Storage Buffer Binding Size");
                ui.label(self.config.limits.max_storage_buffer_binding_size.to_string());
                ui.label(self.adapter_limits.max_storage_buffer_binding_size.to_string());
                ui.end_row();

                ui.label("Max Buffer Size");
                ui.label(self.config.limits.max_buffer_size.to_string());
                ui.label(self.adapter_limits.max_buffer_size.to_string());
                ui.end_row();

                // Compute limits
                ui.label("Max Compute Workgroup Size X");
                ui.label(self.config.limits.max_compute_workgroup_size_x.to_string());
                ui.label(self.adapter_limits.max_compute_workgroup_size_x.to_string());
                ui.end_row();

                ui.label("Max Compute Workgroup Size Y");
                ui.label(self.config.limits.max_compute_workgroup_size_y.to_string());
                ui.label(self.adapter_limits.max_compute_workgroup_size_y.to_string());
                ui.end_row();

                ui.label("Max Compute Workgroup Size Z");
                ui.label(self.config.limits.max_compute_workgroup_size_z.to_string());
                ui.label(self.adapter_limits.max_compute_workgroup_size_z.to_string());
                ui.end_row();

                ui.label("Max Compute Invocations Per Workgroup");
                ui.label(self.config.limits.max_compute_invocations_per_workgroup.to_string());
                ui.label(self.adapter_limits.max_compute_invocations_per_workgroup.to_string());
                ui.end_row();

                ui.label("Max Compute Workgroup Storage Size");
                ui.label(self.config.limits.max_compute_workgroup_storage_size.to_string());
                ui.label(self.adapter_limits.max_compute_workgroup_storage_size.to_string());
                ui.end_row();

                ui.label("Max Compute Workgroups Per Dimension");
                ui.label(self.config.limits.max_compute_workgroups_per_dimension.to_string());
                ui.label(self.adapter_limits.max_compute_workgroups_per_dimension.to_string());
                ui.end_row();
            });
    }
}
