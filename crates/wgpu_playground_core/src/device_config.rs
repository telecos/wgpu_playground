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
            ui.label("Configure features and limits for device creation.");

            ui.add_space(5.0);
            ui.colored_label(
                egui::Color32::from_rgb(255, 200, 100),
                "ℹ️ Note: This panel shows available features and limits. In the current version, \
                the device is created at startup with default settings. This UI can be used to \
                explore what features and limits your adapter supports.",
            );
            ui.add_space(10.0);

            // Features section
            ui.heading("Available Features");
            ui.separator();
            ui.label("Enable or disable WebGPU features:");
            ui.add_space(5.0);

            self.render_features_ui(ui);
            ui.add_space(20.0);

            // Limits section
            ui.heading("Device Limits");
            ui.separator();
            ui.label("Adjust device limits (values are clamped to adapter capabilities):");
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
                self.render_feature_checkbox(
                    ui,
                    "Depth Clip Control",
                    Features::DEPTH_CLIP_CONTROL,
                );
                self.render_feature_checkbox(
                    ui,
                    "Depth32Float Stencil8",
                    Features::DEPTH32FLOAT_STENCIL8,
                );
                self.render_feature_checkbox(ui, "Timestamp Query", Features::TIMESTAMP_QUERY);
                self.render_feature_checkbox(
                    ui,
                    "Pipeline Statistics Query",
                    Features::PIPELINE_STATISTICS_QUERY,
                );
                self.render_feature_checkbox(
                    ui,
                    "Texture Compression BC",
                    Features::TEXTURE_COMPRESSION_BC,
                );
                self.render_feature_checkbox(
                    ui,
                    "Texture Compression ETC2",
                    Features::TEXTURE_COMPRESSION_ETC2,
                );
                self.render_feature_checkbox(
                    ui,
                    "Texture Compression ASTC",
                    Features::TEXTURE_COMPRESSION_ASTC,
                );
                self.render_feature_checkbox(
                    ui,
                    "Indirect First Instance",
                    Features::INDIRECT_FIRST_INSTANCE,
                );
                self.render_feature_checkbox(ui, "Shader F16", Features::SHADER_F16);
                self.render_feature_checkbox(
                    ui,
                    "RG11B10UFloat Renderable",
                    Features::RG11B10UFLOAT_RENDERABLE,
                );
                self.render_feature_checkbox(ui, "BGRA8UnormStorage", Features::BGRA8UNORM_STORAGE);
                self.render_feature_checkbox(
                    ui,
                    "Float32 Filterable",
                    Features::FLOAT32_FILTERABLE,
                );
                self.render_feature_checkbox(
                    ui,
                    "Shader Primitive Index",
                    Features::SHADER_PRIMITIVE_INDEX,
                );
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
        ui.label("💡 Tip: Adjust limits as needed. Values are clamped to adapter maximum.");
        ui.add_space(5.0);

        let adapter_limits = &self.adapter_limits;
        let config_limits = &mut self.config.limits;

        egui::Grid::new("limits_grid")
            .num_columns(2)
            .spacing([10.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                // Texture limits
                Self::render_limit_u32(
                    ui,
                    "Max Texture Dimension 1D",
                    &mut config_limits.max_texture_dimension_1d,
                    adapter_limits.max_texture_dimension_1d,
                    1,
                );

                Self::render_limit_u32(
                    ui,
                    "Max Texture Dimension 2D",
                    &mut config_limits.max_texture_dimension_2d,
                    adapter_limits.max_texture_dimension_2d,
                    1,
                );

                Self::render_limit_u32(
                    ui,
                    "Max Texture Dimension 3D",
                    &mut config_limits.max_texture_dimension_3d,
                    adapter_limits.max_texture_dimension_3d,
                    1,
                );

                Self::render_limit_u32(
                    ui,
                    "Max Texture Array Layers",
                    &mut config_limits.max_texture_array_layers,
                    adapter_limits.max_texture_array_layers,
                    1,
                );

                // Bind group limits
                Self::render_limit_u32(
                    ui,
                    "Max Bind Groups",
                    &mut config_limits.max_bind_groups,
                    adapter_limits.max_bind_groups,
                    1,
                );

                Self::render_limit_u32(
                    ui,
                    "Max Bindings Per Bind Group",
                    &mut config_limits.max_bindings_per_bind_group,
                    adapter_limits.max_bindings_per_bind_group,
                    1,
                );

                // Buffer limits
                Self::render_limit_u32(
                    ui,
                    "Max Uniform Buffer Binding Size",
                    &mut config_limits.max_uniform_buffer_binding_size,
                    adapter_limits.max_uniform_buffer_binding_size,
                    1,
                );

                Self::render_limit_u32(
                    ui,
                    "Max Storage Buffer Binding Size",
                    &mut config_limits.max_storage_buffer_binding_size,
                    adapter_limits.max_storage_buffer_binding_size,
                    1,
                );

                Self::render_limit_u64(
                    ui,
                    "Max Buffer Size",
                    &mut config_limits.max_buffer_size,
                    adapter_limits.max_buffer_size,
                    1,
                );

                // Vertex limits
                Self::render_limit_u32(
                    ui,
                    "Max Vertex Buffers",
                    &mut config_limits.max_vertex_buffers,
                    adapter_limits.max_vertex_buffers,
                    1,
                );

                Self::render_limit_u32(
                    ui,
                    "Max Vertex Attributes",
                    &mut config_limits.max_vertex_attributes,
                    adapter_limits.max_vertex_attributes,
                    1,
                );

                // Compute limits
                Self::render_limit_u32(
                    ui,
                    "Max Compute Workgroup Size X",
                    &mut config_limits.max_compute_workgroup_size_x,
                    adapter_limits.max_compute_workgroup_size_x,
                    1,
                );

                Self::render_limit_u32(
                    ui,
                    "Max Compute Workgroup Size Y",
                    &mut config_limits.max_compute_workgroup_size_y,
                    adapter_limits.max_compute_workgroup_size_y,
                    1,
                );

                Self::render_limit_u32(
                    ui,
                    "Max Compute Workgroup Size Z",
                    &mut config_limits.max_compute_workgroup_size_z,
                    adapter_limits.max_compute_workgroup_size_z,
                    1,
                );

                Self::render_limit_u32(
                    ui,
                    "Max Compute Invocations Per Workgroup",
                    &mut config_limits.max_compute_invocations_per_workgroup,
                    adapter_limits.max_compute_invocations_per_workgroup,
                    1,
                );

                Self::render_limit_u32(
                    ui,
                    "Max Compute Workgroup Storage Size",
                    &mut config_limits.max_compute_workgroup_storage_size,
                    adapter_limits.max_compute_workgroup_storage_size,
                    1,
                );

                Self::render_limit_u32(
                    ui,
                    "Max Compute Workgroups Per Dimension",
                    &mut config_limits.max_compute_workgroups_per_dimension,
                    adapter_limits.max_compute_workgroups_per_dimension,
                    1,
                );
            });

        ui.add_space(10.0);
        ui.horizontal(|ui| {
            if ui.button("Reset to Default").clicked() {
                self.config.limits = Limits::default();
            }
            if ui.button("Use Maximum Available").clicked() {
                self.config.limits = self.adapter_limits.clone();
            }
        });
    }

    fn render_limit_u32(ui: &mut egui::Ui, label: &str, value: &mut u32, max: u32, min: u32) {
        ui.label(label);
        let mut temp_value = *value;
        if ui
            .add(egui::DragValue::new(&mut temp_value).range(min..=max))
            .changed()
        {
            *value = temp_value;
        }
        ui.end_row();
    }

    fn render_limit_u64(ui: &mut egui::Ui, label: &str, value: &mut u64, max: u64, min: u64) {
        ui.label(label);
        let mut temp_value = *value;
        if ui
            .add(egui::DragValue::new(&mut temp_value).range(min..=max))
            .changed()
        {
            *value = temp_value;
        }
        ui.end_row();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_config_default() {
        let config = DeviceConfig::default();
        assert_eq!(config.features, Features::empty());
        assert_eq!(
            config.limits.max_texture_dimension_2d,
            Limits::default().max_texture_dimension_2d
        );
    }

    #[test]
    fn test_device_config_new() {
        let config = DeviceConfig::new();
        assert_eq!(config.features, Features::empty());
    }

    #[test]
    fn test_device_config_set_feature() {
        let mut config = DeviceConfig::new();

        // Enable a feature
        config.set_feature(Features::TIMESTAMP_QUERY, true);
        assert!(config.has_feature(Features::TIMESTAMP_QUERY));

        // Disable a feature
        config.set_feature(Features::TIMESTAMP_QUERY, false);
        assert!(!config.has_feature(Features::TIMESTAMP_QUERY));
    }

    #[test]
    fn test_device_config_has_feature() {
        let mut config = DeviceConfig::new();
        config.features = Features::DEPTH_CLIP_CONTROL | Features::TIMESTAMP_QUERY;

        assert!(config.has_feature(Features::DEPTH_CLIP_CONTROL));
        assert!(config.has_feature(Features::TIMESTAMP_QUERY));
        assert!(!config.has_feature(Features::SHADER_F16));
    }
}
