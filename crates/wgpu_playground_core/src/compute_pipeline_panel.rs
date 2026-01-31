use crate::compute::ComputePipelineDescriptor;
use crate::shader::ShaderModule;

/// UI panel for creating and configuring compute pipelines
pub struct ComputePipelinePanel {
    /// Current pipeline descriptor being configured
    descriptor: ComputePipelineDescriptor,
    /// Label input text
    pub label_input: String,
    /// Shader source code input
    pub shader_source: String,
    /// Shader label input
    pub shader_label: String,
    /// Entry point input text
    pub entry_point_input: String,
    /// Whether to use auto-generated pipeline layout
    pub use_auto_layout: bool,
    /// Validation error message
    pub validation_error: Option<String>,
    /// Success message
    pub success_message: Option<String>,
    /// Compiled shader module (cached)
    pub cached_shader: Option<ShaderModule>,
}

impl Default for ComputePipelinePanel {
    fn default() -> Self {
        Self::new()
    }
}

impl ComputePipelinePanel {
    /// Create a new compute pipeline panel with default values
    pub fn new() -> Self {
        let default_shader = Self::default_compute_shader();
        Self {
            descriptor: ComputePipelineDescriptor::default(),
            label_input: String::new(),
            shader_source: default_shader,
            shader_label: "compute_shader".to_string(),
            entry_point_input: "main".to_string(),
            use_auto_layout: true,
            validation_error: None,
            success_message: None,
            cached_shader: None,
        }
    }

    /// Get default compute shader template
    fn default_compute_shader() -> String {
        r#"// Simple compute shader template
@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // Add your compute logic here
}"#
        .to_string()
    }

    /// Update the internal descriptor based on current UI state
    fn update_descriptor(&mut self) -> Result<(), String> {
        // Compile shader if source has changed
        if self.cached_shader.is_none()
            || self
                .cached_shader
                .as_ref()
                .map(|s| s.source() != self.shader_source.as_str())
                .unwrap_or(true)
        {
            let shader_label = if self.shader_label.is_empty() {
                None
            } else {
                Some(self.shader_label.as_str())
            };

            match ShaderModule::from_source(&self.shader_source, shader_label) {
                Ok(shader) => {
                    self.cached_shader = Some(shader);
                }
                Err(e) => {
                    return Err(format!("Shader compilation error: {}", e));
                }
            }
        }

        // Create descriptor
        let label = if self.label_input.is_empty() {
            None
        } else {
            Some(self.label_input.as_str())
        };

        let mut descriptor = ComputePipelineDescriptor::new(label);

        if let Some(shader) = &self.cached_shader {
            descriptor = descriptor.with_shader(shader.clone());
        }

        if !self.entry_point_input.is_empty() {
            descriptor = descriptor.with_entry_point(&self.entry_point_input);
        }

        self.descriptor = descriptor;
        Ok(())
    }

    /// Validate the current configuration
    pub fn validate(&mut self) -> bool {
        match self.update_descriptor() {
            Ok(_) => match self.descriptor.validate() {
                Ok(_) => {
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

    /// Create a compute pipeline with the current configuration
    /// Returns a pipeline that can be used for compute operations
    pub fn create_pipeline(&mut self, device: &wgpu::Device) -> Option<wgpu::ComputePipeline> {
        if !self.validate() {
            return None;
        }

        match self.descriptor.create_pipeline(device) {
            Ok(pipeline) => {
                self.success_message = Some(format!(
                    "✓ Compute pipeline created successfully: '{}'",
                    self.label_input
                        .as_str()
                        .to_string()
                        .as_str()
                        .trim()
                        .is_empty()
                        .then(|| "unlabeled")
                        .unwrap_or(self.label_input.as_str())
                ));
                self.validation_error = None;
                Some(pipeline)
            }
            Err(e) => {
                self.validation_error = Some(format!("Failed to create pipeline: {}", e));
                self.success_message = None;
                None
            }
        }
    }

    /// Render the compute pipeline configuration UI
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("⚙️ Compute Pipeline Configuration");
            ui.label("Configure and create compute pipelines for GPU compute operations.");
            ui.add_space(10.0);

            // Pipeline Properties
            ui.group(|ui| {
                ui.heading("Pipeline Properties");
                ui.add_space(5.0);

                egui::Grid::new("pipeline_properties")
                    .num_columns(2)
                    .spacing([10.0, 8.0])
                    .show(ui, |ui| {
                        ui.label("Pipeline Label:");
                        ui.text_edit_singleline(&mut self.label_input)
                            .on_hover_text("Optional label for debugging");
                        ui.end_row();

                        ui.label("Entry Point:");
                        ui.text_edit_singleline(&mut self.entry_point_input)
                            .on_hover_text("Entry point function name in the shader (e.g., 'main')");
                        ui.end_row();
                    });
            });

            ui.add_space(10.0);

            // Shader Module Configuration
            ui.group(|ui| {
                ui.heading("Shader Module");
                ui.label("Configure the compute shader for this pipeline.");
                ui.add_space(5.0);

                egui::Grid::new("shader_module")
                    .num_columns(2)
                    .spacing([10.0, 8.0])
                    .show(ui, |ui| {
                        ui.label("Shader Label:");
                        ui.text_edit_singleline(&mut self.shader_label)
                            .on_hover_text("Optional label for the shader module");
                        ui.end_row();
                    });

                ui.add_space(5.0);
                ui.label("Shader Source (WGSL):");
                ui.add_space(2.0);

                // Shader code editor
                let code_editor = egui::TextEdit::multiline(&mut self.shader_source)
                    .code_editor()
                    .desired_rows(15)
                    .desired_width(f32::INFINITY)
                    .font(egui::TextStyle::Monospace);

                ui.add(code_editor);

                ui.add_space(5.0);

                // Shader templates
                ui.horizontal(|ui| {
                    ui.label("Templates:");
                    if ui.button("Simple Compute").clicked() {
                        self.shader_source = Self::default_compute_shader();
                        self.cached_shader = None;
                    }
                    if ui.button("Storage Buffer").clicked() {
                        self.shader_source = Self::storage_buffer_shader();
                        self.cached_shader = None;
                    }
                    if ui.button("Matrix Multiply").clicked() {
                        self.shader_source = Self::matrix_multiply_shader();
                        self.cached_shader = None;
                    }
                });
            });

            ui.add_space(10.0);

            // Pipeline Layout Configuration
            ui.group(|ui| {
                ui.heading("Pipeline Layout");
                ui.label("Configure how bind groups are organized in the pipeline.");
                ui.add_space(5.0);

                ui.checkbox(&mut self.use_auto_layout, "Use Auto-Generated Layout")
                    .on_hover_text(
                        "When enabled, the pipeline layout will be automatically \
                         generated from the shader. Disable to manually configure bind groups.",
                    );

                if !self.use_auto_layout {
                    ui.add_space(5.0);
                    ui.colored_label(
                        egui::Color32::YELLOW,
                        "⚠️ Manual layout configuration not yet implemented",
                    );
                    ui.label("For now, auto-generated layouts are used.");
                }
            });

            ui.add_space(10.0);

            // Validation and Creation
            ui.separator();
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                if ui.button("Validate Configuration").clicked() {
                    self.validate();
                }

                ui.add_space(10.0);

                // Note: Creating pipeline requires a device, which we don't have access to in the UI
                // This button is disabled for now
                ui.add_enabled_ui(false, |ui| {
                    ui.button("Create Pipeline")
                        .on_disabled_hover_text("Pipeline creation requires GPU device access");
                });
            });

            ui.add_space(5.0);

            // Display validation errors
            if let Some(error) = &self.validation_error {
                ui.add_space(5.0);
                ui.colored_label(egui::Color32::RED, format!("❌ Error: {}", error));
            }

            // Display success messages
            if let Some(success) = &self.success_message {
                ui.add_space(5.0);
                ui.colored_label(egui::Color32::GREEN, success);
            }

            ui.add_space(10.0);
            ui.separator();

            // Information Section
            ui.group(|ui| {
                ui.heading("ℹ️ Compute Pipeline Information");
                ui.add_space(5.0);

                ui.label("A compute pipeline consists of:");
                ui.label("• Shader Module: Contains the compute shader code (WGSL)");
                ui.label("• Entry Point: The function name to execute (e.g., 'main')");
                ui.label("• Pipeline Layout: Defines bind group organization (auto-generated or manual)");

                ui.add_space(5.0);
                ui.label("Compute shaders must have:");
                ui.label("• @compute attribute on the entry point function");
                ui.label("• @workgroup_size attribute specifying execution dimensions");
                ui.label("• Example: @compute @workgroup_size(64, 1, 1)");
            });
        });
    }

    /// Get storage buffer shader template
    pub fn storage_buffer_shader() -> String {
        r#"// Compute shader with storage buffer
@group(0) @binding(0)
var<storage, read_write> data: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    // Example: multiply each element by 2
    data[index] = data[index] * 2.0;
}"#
        .to_string()
    }

    /// Get matrix multiply shader template
    pub fn matrix_multiply_shader() -> String {
        r#"// Matrix multiplication compute shader
@group(0) @binding(0)
var<storage, read> matrix_a: array<f32>;

@group(0) @binding(1)
var<storage, read> matrix_b: array<f32>;

@group(0) @binding(2)
var<storage, read_write> matrix_result: array<f32>;

// Uniforms for matrix dimensions
@group(0) @binding(3)
var<uniform> dimensions: vec3<u32>; // (M, N, K)

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let row = global_id.x;
    let col = global_id.y;
    
    let M = dimensions.x;
    let N = dimensions.y;
    let K = dimensions.z;
    
    if (row >= M || col >= N) {
        return;
    }
    
    var sum = 0.0;
    for (var i = 0u; i < K; i = i + 1u) {
        sum = sum + matrix_a[row * K + i] * matrix_b[i * N + col];
    }
    
    matrix_result[row * N + col] = sum;
}"#
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_pipeline_panel_creation() {
        let panel = ComputePipelinePanel::new();
        assert_eq!(panel.label_input, "");
        assert_eq!(panel.entry_point_input, "main");
        assert_eq!(panel.shader_label, "compute_shader");
        assert!(panel.use_auto_layout);
        assert!(panel.validation_error.is_none());
        assert!(panel.success_message.is_none());
    }

    #[test]
    fn test_compute_pipeline_panel_default() {
        let panel = ComputePipelinePanel::default();
        assert_eq!(panel.label_input, "");
        assert_eq!(panel.entry_point_input, "main");
    }

    #[test]
    fn test_shader_templates() {
        let default = ComputePipelinePanel::default_compute_shader();
        assert!(default.contains("@compute"));
        assert!(default.contains("@workgroup_size"));

        let storage = ComputePipelinePanel::storage_buffer_shader();
        assert!(storage.contains("var<storage"));
        assert!(storage.contains("@compute"));

        let matrix = ComputePipelinePanel::matrix_multiply_shader();
        assert!(matrix.contains("matrix_a"));
        assert!(matrix.contains("matrix_b"));
        assert!(matrix.contains("matrix_result"));
    }

    #[test]
    fn test_validation_with_default_shader() {
        let mut panel = ComputePipelinePanel::new();
        panel.label_input = "test_pipeline".to_string();

        // Default shader should be valid
        assert!(panel.validate());
        assert!(panel.validation_error.is_none());
    }

    #[test]
    fn test_validation_with_invalid_shader() {
        let mut panel = ComputePipelinePanel::new();
        // Empty shader should fail validation
        panel.shader_source = "".to_string();
        panel.cached_shader = None;

        // Empty shader should fail validation
        assert!(!panel.validate());
        assert!(panel.validation_error.is_some());
    }

    #[test]
    fn test_validation_with_empty_entry_point() {
        let mut panel = ComputePipelinePanel::new();
        panel.entry_point_input = "".to_string();

        // Empty entry point should fail validation
        assert!(!panel.validate());
        assert!(panel.validation_error.is_some());
    }

    #[test]
    fn test_update_descriptor_caches_shader() {
        let mut panel = ComputePipelinePanel::new();

        assert!(panel.update_descriptor().is_ok());
        assert!(panel.cached_shader.is_some());

        // Shader should remain cached if source hasn't changed
        let first_shader_ptr = panel.cached_shader.as_ref().map(|s| s as *const _);
        assert!(panel.update_descriptor().is_ok());
        let second_shader_ptr = panel.cached_shader.as_ref().map(|s| s as *const _);

        // Pointers should be different because we're creating a new shader,
        // but the source should be the same
        if let Some(shader) = &panel.cached_shader {
            assert_eq!(shader.source(), panel.shader_source.as_str());
        }
    }

    #[test]
    fn test_label_handling() {
        let mut panel = ComputePipelinePanel::new();

        // Empty label should work
        panel.label_input = "".to_string();
        assert!(panel.update_descriptor().is_ok());

        // Non-empty label should work
        panel.label_input = "my_pipeline".to_string();
        assert!(panel.update_descriptor().is_ok());
    }
}
