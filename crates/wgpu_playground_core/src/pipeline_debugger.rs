/// Information about a shader module in a pipeline
#[derive(Debug, Clone)]
pub struct ShaderInfo {
    /// Shader source code
    pub source: String,
    /// Entry point name
    pub entry_point: String,
    /// Shader stage (vertex, fragment, compute)
    pub stage: ShaderStage,
}

/// Shader stage type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderStage {
    /// Vertex shader
    Vertex,
    /// Fragment shader
    Fragment,
    /// Compute shader
    Compute,
}

impl ShaderStage {
    /// Get a human-readable name
    pub fn as_str(&self) -> &'static str {
        match self {
            ShaderStage::Vertex => "Vertex",
            ShaderStage::Fragment => "Fragment",
            ShaderStage::Compute => "Compute",
        }
    }
}

/// Pipeline configuration details
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    /// Optional label
    pub label: Option<String>,
    /// Primitive topology (for render pipelines)
    pub topology: Option<String>,
    /// Depth/stencil state enabled
    pub has_depth_stencil: bool,
    /// Number of color targets (for render pipelines)
    pub color_target_count: usize,
    /// Blend state enabled
    pub has_blending: bool,
    /// Multisampling count
    pub sample_count: u32,
}

/// Validation message severity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationSeverity {
    /// Informational message
    Info,
    /// Warning message
    Warning,
    /// Error message
    Error,
}

impl ValidationSeverity {
    /// Get color for UI display
    pub fn color(&self) -> egui::Color32 {
        match self {
            ValidationSeverity::Info => egui::Color32::LIGHT_BLUE,
            ValidationSeverity::Warning => egui::Color32::YELLOW,
            ValidationSeverity::Error => egui::Color32::RED,
        }
    }

    /// Get icon for UI display
    pub fn icon(&self) -> &'static str {
        match self {
            ValidationSeverity::Info => "ℹ️",
            ValidationSeverity::Warning => "⚠️",
            ValidationSeverity::Error => "❌",
        }
    }
}

/// Validation message
#[derive(Debug, Clone)]
pub struct ValidationMessage {
    /// Severity level
    pub severity: ValidationSeverity,
    /// Message text
    pub message: String,
}

/// Pipeline debugging information
#[derive(Debug, Clone)]
pub struct PipelineDebugInfo {
    /// Pipeline configuration
    pub config: PipelineConfig,
    /// Shader modules in the pipeline
    pub shaders: Vec<ShaderInfo>,
    /// Validation messages
    pub validation_messages: Vec<ValidationMessage>,
}

/// Debugger for GPU pipelines
///
/// This utility provides debugging information for render and compute pipelines,
/// including shader source code, pipeline configuration, and validation messages.
pub struct PipelineDebugger {
    /// Currently loaded pipeline debug info
    debug_info: Option<PipelineDebugInfo>,
    /// Selected shader index for viewing
    selected_shader_index: usize,
    /// Whether to show all validation messages or just errors
    show_all_messages: bool,
    /// Error message if loading failed
    error_message: Option<String>,
}

impl Default for PipelineDebugger {
    fn default() -> Self {
        Self::new()
    }
}

impl PipelineDebugger {
    /// Create a new pipeline debugger
    pub fn new() -> Self {
        Self {
            debug_info: None,
            selected_shader_index: 0,
            show_all_messages: true,
            error_message: None,
        }
    }

    /// Get the currently loaded debug info
    pub fn debug_info(&self) -> Option<&PipelineDebugInfo> {
        self.debug_info.as_ref()
    }

    /// Clear the loaded debug info
    pub fn clear(&mut self) {
        self.debug_info = None;
        self.error_message = None;
        self.selected_shader_index = 0;
    }

    /// Load pipeline debug information
    ///
    /// # Arguments
    /// * `info` - The pipeline debug information to load
    pub fn load_pipeline(&mut self, info: PipelineDebugInfo) {
        self.debug_info = Some(info);
        self.error_message = None;
        self.selected_shader_index = 0;
    }

    /// Set an error message
    pub fn set_error(&mut self, error: String) {
        self.error_message = Some(error);
    }

    /// Check if there's an error
    pub fn has_error(&self) -> bool {
        self.error_message.is_some()
    }

    /// Get the error message if any
    pub fn error_message(&self) -> Option<&str> {
        self.error_message.as_deref()
    }

    /// Count validation messages by severity
    pub fn count_messages_by_severity(&self, severity: ValidationSeverity) -> usize {
        self.debug_info
            .as_ref()
            .map(|info| {
                info.validation_messages
                    .iter()
                    .filter(|msg| msg.severity == severity)
                    .count()
            })
            .unwrap_or(0)
    }

    /// Render pipeline configuration
    fn render_config(&self, ui: &mut egui::Ui, config: &PipelineConfig) {
        ui.group(|ui| {
            ui.heading("Pipeline Configuration");
            
            egui::Grid::new("pipeline_config")
                .num_columns(2)
                .spacing([10.0, 3.0])
                .show(ui, |ui| {
                    if let Some(label) = &config.label {
                        ui.label("Label:");
                        ui.monospace(label);
                        ui.end_row();
                    }

                    if let Some(topology) = &config.topology {
                        ui.label("Topology:");
                        ui.monospace(topology);
                        ui.end_row();
                    }

                    ui.label("Color Targets:");
                    ui.label(config.color_target_count.to_string());
                    ui.end_row();

                    ui.label("Depth/Stencil:");
                    ui.label(if config.has_depth_stencil { "Enabled" } else { "Disabled" });
                    ui.end_row();

                    ui.label("Blending:");
                    ui.label(if config.has_blending { "Enabled" } else { "Disabled" });
                    ui.end_row();

                    ui.label("Sample Count:");
                    ui.label(config.sample_count.to_string());
                    ui.end_row();
                });
        });
    }

    /// Render shader information
    fn render_shaders(&self, ui: &mut egui::Ui, shaders: &[ShaderInfo], selected_shader_index: &mut usize) {
        ui.group(|ui| {
            ui.heading("Shaders");
            
            // Shader tabs
            ui.horizontal(|ui| {
                for (i, shader) in shaders.iter().enumerate() {
                    if ui.selectable_label(
                        *selected_shader_index == i,
                        format!("{} ({})", shader.stage.as_str(), shader.entry_point)
                    ).clicked() {
                        *selected_shader_index = i;
                    }
                }
            });

            ui.separator();

            // Display selected shader source
            if let Some(shader) = shaders.get(*selected_shader_index) {
                ui.label(format!("Entry Point: {}", shader.entry_point));
                ui.separator();
                
                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut shader.source.as_str())
                                .font(egui::TextStyle::Monospace)
                                .code_editor()
                                .desired_width(f32::INFINITY)
                        );
                    });
            }
        });
    }

    /// Render validation messages
    fn render_validation(&self, ui: &mut egui::Ui, messages: &[ValidationMessage], show_all_messages: &mut bool) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.heading("Validation Messages");
                ui.separator();
                
                let error_count = self.count_messages_by_severity(ValidationSeverity::Error);
                let warning_count = self.count_messages_by_severity(ValidationSeverity::Warning);
                let info_count = self.count_messages_by_severity(ValidationSeverity::Info);
                
                if error_count > 0 {
                    ui.colored_label(egui::Color32::RED, format!("❌ {} errors", error_count));
                }
                if warning_count > 0 {
                    ui.colored_label(egui::Color32::YELLOW, format!("⚠️ {} warnings", warning_count));
                }
                if info_count > 0 {
                    ui.colored_label(egui::Color32::LIGHT_BLUE, format!("ℹ️ {} info", info_count));
                }
            });

            ui.checkbox(show_all_messages, "Show all messages");
            ui.separator();

            if messages.is_empty() {
                ui.colored_label(egui::Color32::GREEN, "✓ No validation messages");
            } else {
                egui::ScrollArea::vertical()
                    .max_height(200.0)
                    .show(ui, |ui| {
                        for msg in messages {
                            if *show_all_messages || msg.severity == ValidationSeverity::Error {
                                ui.horizontal(|ui| {
                                    ui.label(msg.severity.icon());
                                    ui.colored_label(msg.severity.color(), &msg.message);
                                });
                            }
                        }
                    });
            }
        });
    }

    /// Render the pipeline debugger UI
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Pipeline Debugger");
        ui.separator();

        // Display error if any
        if let Some(error) = &self.error_message {
            ui.colored_label(egui::Color32::RED, format!("Error: {}", error));
            ui.separator();
        }

        if let Some(info) = self.debug_info.as_ref() {
            // Make local copies of mutable state
            let mut selected_shader = self.selected_shader_index;
            let mut show_all = self.show_all_messages;
            
            // Configuration
            self.render_config(ui, &info.config);
            ui.add_space(10.0);

            // Shaders
            if !info.shaders.is_empty() {
                self.render_shaders(ui, &info.shaders, &mut selected_shader);
                ui.add_space(10.0);
            }

            // Validation messages
            self.render_validation(ui, &info.validation_messages, &mut show_all);
            
            // Update state
            self.selected_shader_index = selected_shader;
            self.show_all_messages = show_all;
        } else if self.error_message.is_none() {
            ui.label("Select a pipeline from the Resource Inspector to debug it");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_pipeline() -> PipelineDebugInfo {
        PipelineDebugInfo {
            config: PipelineConfig {
                label: Some("Test Pipeline".to_string()),
                topology: Some("TriangleList".to_string()),
                has_depth_stencil: true,
                color_target_count: 1,
                has_blending: false,
                sample_count: 1,
            },
            shaders: vec![
                ShaderInfo {
                    source: "@vertex\nfn main() {}".to_string(),
                    entry_point: "main".to_string(),
                    stage: ShaderStage::Vertex,
                },
                ShaderInfo {
                    source: "@fragment\nfn main() {}".to_string(),
                    entry_point: "main".to_string(),
                    stage: ShaderStage::Fragment,
                },
            ],
            validation_messages: vec![
                ValidationMessage {
                    severity: ValidationSeverity::Warning,
                    message: "Unused variable".to_string(),
                },
            ],
        }
    }

    #[test]
    fn test_pipeline_debugger_creation() {
        let debugger = PipelineDebugger::new();
        assert!(debugger.debug_info().is_none());
        assert!(!debugger.has_error());
    }

    #[test]
    fn test_load_pipeline() {
        let mut debugger = PipelineDebugger::new();
        let pipeline = create_test_pipeline();
        debugger.load_pipeline(pipeline);
        assert!(debugger.debug_info().is_some());
        assert!(!debugger.has_error());
    }

    #[test]
    fn test_clear() {
        let mut debugger = PipelineDebugger::new();
        debugger.load_pipeline(create_test_pipeline());
        assert!(debugger.debug_info().is_some());
        debugger.clear();
        assert!(debugger.debug_info().is_none());
    }

    #[test]
    fn test_set_error() {
        let mut debugger = PipelineDebugger::new();
        debugger.set_error("Test error".to_string());
        assert!(debugger.has_error());
        assert_eq!(debugger.error_message(), Some("Test error"));
    }

    #[test]
    fn test_shader_stage_names() {
        assert_eq!(ShaderStage::Vertex.as_str(), "Vertex");
        assert_eq!(ShaderStage::Fragment.as_str(), "Fragment");
        assert_eq!(ShaderStage::Compute.as_str(), "Compute");
    }

    #[test]
    fn test_validation_severity() {
        let error = ValidationSeverity::Error;
        let warning = ValidationSeverity::Warning;
        let info = ValidationSeverity::Info;
        
        assert_eq!(error.icon(), "❌");
        assert_eq!(warning.icon(), "⚠️");
        assert_eq!(info.icon(), "ℹ️");
    }

    #[test]
    fn test_count_messages_by_severity() {
        let mut debugger = PipelineDebugger::new();
        let mut pipeline = create_test_pipeline();
        pipeline.validation_messages.push(ValidationMessage {
            severity: ValidationSeverity::Error,
            message: "Error message".to_string(),
        });
        debugger.load_pipeline(pipeline);
        
        assert_eq!(debugger.count_messages_by_severity(ValidationSeverity::Error), 1);
        assert_eq!(debugger.count_messages_by_severity(ValidationSeverity::Warning), 1);
        assert_eq!(debugger.count_messages_by_severity(ValidationSeverity::Info), 0);
    }

    #[test]
    fn test_pipeline_config() {
        let config = PipelineConfig {
            label: Some("Test".to_string()),
            topology: Some("TriangleList".to_string()),
            has_depth_stencil: true,
            color_target_count: 2,
            has_blending: true,
            sample_count: 4,
        };
        
        assert_eq!(config.label, Some("Test".to_string()));
        assert_eq!(config.color_target_count, 2);
        assert_eq!(config.sample_count, 4);
    }
}
