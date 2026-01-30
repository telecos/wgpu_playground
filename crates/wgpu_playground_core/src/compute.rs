use std::fmt;
use wgpu::{ComputePipeline, Device, PipelineLayout};

use crate::shader::ShaderModule;

/// Errors that can occur during compute pipeline operations
#[derive(Debug)]
pub enum ComputePipelineError {
    /// Failed to create compute pipeline
    CreationFailed(String),
    /// Invalid configuration
    InvalidConfiguration(String),
    /// Missing required shader module
    MissingShader,
    /// Missing entry point
    MissingEntryPoint,
}

impl fmt::Display for ComputePipelineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComputePipelineError::CreationFailed(msg) => {
                write!(f, "Compute pipeline creation failed: {}", msg)
            }
            ComputePipelineError::InvalidConfiguration(msg) => {
                write!(f, "Invalid pipeline configuration: {}", msg)
            }
            ComputePipelineError::MissingShader => {
                write!(f, "Missing shader module: compute pipeline requires a shader")
            }
            ComputePipelineError::MissingEntryPoint => {
                write!(f, "Missing entry point: compute pipeline requires an entry point")
            }
        }
    }
}

impl std::error::Error for ComputePipelineError {}

/// Descriptor for creating a compute pipeline
///
/// This descriptor follows the builder pattern to configure a compute pipeline.
/// At minimum, a shader module and entry point must be provided.
///
/// # Examples
/// ```no_run
/// use wgpu_playground_core::compute::ComputePipelineDescriptor;
/// use wgpu_playground_core::shader::ShaderModule;
/// # async fn example(device: &wgpu::Device) {
/// let shader = ShaderModule::from_source(
///     "@compute @workgroup_size(1) fn main() {}",
///     Some("compute_shader")
/// ).unwrap();
///
/// let descriptor = ComputePipelineDescriptor::new(Some("my_pipeline"))
///     .with_shader(shader)
///     .with_entry_point("main");
///
/// let pipeline = descriptor.create_pipeline(device).unwrap();
/// # }
/// ```
pub struct ComputePipelineDescriptor {
    /// Optional label for debugging
    label: Option<String>,
    /// The compute shader module
    shader: Option<ShaderModule>,
    /// Entry point function name in the shader
    entry_point: Option<String>,
    /// Optional pipeline layout (if None, will be auto-generated)
    layout: Option<PipelineLayout>,
}

impl ComputePipelineDescriptor {
    /// Create a new compute pipeline descriptor
    ///
    /// # Arguments
    /// * `label` - Optional label for debugging
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::compute::ComputePipelineDescriptor;
    ///
    /// let descriptor = ComputePipelineDescriptor::new(Some("my_compute_pipeline"));
    /// ```
    pub fn new(label: Option<&str>) -> Self {
        Self {
            label: label.map(String::from),
            shader: None,
            entry_point: None,
            layout: None,
        }
    }

    /// Set the shader module for this compute pipeline
    ///
    /// # Arguments
    /// * `shader` - The shader module containing the compute shader
    ///
    /// # Returns
    /// Self for method chaining
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::compute::ComputePipelineDescriptor;
    /// use wgpu_playground_core::shader::ShaderModule;
    ///
    /// let shader = ShaderModule::from_source(
    ///     "@compute @workgroup_size(1) fn main() {}",
    ///     Some("compute")
    /// ).unwrap();
    ///
    /// let descriptor = ComputePipelineDescriptor::new(Some("pipeline"))
    ///     .with_shader(shader);
    /// ```
    pub fn with_shader(mut self, shader: ShaderModule) -> Self {
        self.shader = Some(shader);
        self
    }

    /// Set the entry point function name
    ///
    /// # Arguments
    /// * `entry_point` - The name of the entry point function in the shader
    ///
    /// # Returns
    /// Self for method chaining
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::compute::ComputePipelineDescriptor;
    ///
    /// let descriptor = ComputePipelineDescriptor::new(Some("pipeline"))
    ///     .with_entry_point("main");
    /// ```
    pub fn with_entry_point(mut self, entry_point: &str) -> Self {
        self.entry_point = Some(entry_point.to_string());
        self
    }

    /// Set the pipeline layout
    ///
    /// If not provided, the pipeline layout will be automatically generated from the shader.
    ///
    /// # Arguments
    /// * `layout` - The pipeline layout defining bind groups
    ///
    /// # Returns
    /// Self for method chaining
    ///
    /// # Examples
    /// ```no_run
    /// use wgpu_playground_core::compute::ComputePipelineDescriptor;
    /// # async fn example(device: &wgpu::Device, layout: wgpu::PipelineLayout) {
    /// let descriptor = ComputePipelineDescriptor::new(Some("pipeline"))
    ///     .with_layout(layout);
    /// # }
    /// ```
    pub fn with_layout(mut self, layout: PipelineLayout) -> Self {
        self.layout = Some(layout);
        self
    }

    /// Get the label
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Get the shader module
    pub fn shader(&self) -> Option<&ShaderModule> {
        self.shader.as_ref()
    }

    /// Get the entry point
    pub fn entry_point(&self) -> Option<&str> {
        self.entry_point.as_deref()
    }

    /// Get the pipeline layout
    pub fn layout(&self) -> Option<&PipelineLayout> {
        self.layout.as_ref()
    }

    /// Validate the compute pipeline descriptor
    ///
    /// Checks for:
    /// - Shader module must be provided
    /// - Entry point must be provided
    /// - Entry point name must not be empty
    ///
    /// # Returns
    /// Ok(()) if valid, Err with ComputePipelineError if invalid
    pub fn validate(&self) -> Result<(), ComputePipelineError> {
        if self.shader.is_none() {
            return Err(ComputePipelineError::MissingShader);
        }

        if self.entry_point.is_none() {
            return Err(ComputePipelineError::MissingEntryPoint);
        }

        if let Some(entry_point) = &self.entry_point {
            if entry_point.trim().is_empty() {
                return Err(ComputePipelineError::InvalidConfiguration(
                    "Entry point name cannot be empty".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Create a wgpu compute pipeline from this descriptor
    ///
    /// This method validates the descriptor and creates the actual compute pipeline.
    /// The shader will be compiled during this process.
    ///
    /// # Arguments
    /// * `device` - The wgpu device to create the pipeline on
    ///
    /// # Returns
    /// A Result containing the ComputePipeline or a ComputePipelineError
    ///
    /// # Examples
    /// ```no_run
    /// use wgpu_playground_core::compute::ComputePipelineDescriptor;
    /// use wgpu_playground_core::shader::ShaderModule;
    /// # async fn example(device: &wgpu::Device) {
    /// let shader = ShaderModule::from_source(
    ///     "@compute @workgroup_size(1) fn main() {}",
    ///     Some("compute")
    /// ).unwrap();
    ///
    /// let descriptor = ComputePipelineDescriptor::new(Some("my_pipeline"))
    ///     .with_shader(shader)
    ///     .with_entry_point("main");
    ///
    /// let pipeline = descriptor.create_pipeline(device).unwrap();
    /// # }
    /// ```
    pub fn create_pipeline(
        &self,
        device: &Device,
    ) -> Result<ComputePipeline, ComputePipelineError> {
        self.validate()?;

        let shader = self.shader.as_ref().unwrap();
        let entry_point = self.entry_point.as_ref().unwrap();

        let shader_module = shader.create_module(device);

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: self.label.as_deref(),
            layout: self.layout.as_ref(),
            module: &shader_module,
            entry_point,
            compilation_options: Default::default(),
            cache: None,
        });

        Ok(pipeline)
    }
}

impl Default for ComputePipelineDescriptor {
    fn default() -> Self {
        Self::new(None)
    }
}

pub struct ComputePanel {
    // Placeholder for compute/ML experiments
}

impl Default for ComputePanel {
    fn default() -> Self {
        Self::new()
    }
}

impl ComputePanel {
    pub fn new() -> Self {
        Self {}
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("Compute & ML Inferencing APIs");
            ui.separator();
            ui.label("This section will provide tools to experiment with:");
            ui.add_space(10.0);

            ui.group(|ui| {
                ui.heading("⚙️ Compute Pipelines");
                ui.label("• Create and configure compute pipelines");
                ui.label("• Compute shader experimentation");
                ui.label("• Workgroup size configuration");
                ui.label("• Pipeline layout and bind groups");
            });

            ui.add_space(10.0);

            ui.group(|ui| {
                ui.heading("💾 Storage Buffers");
                ui.label("• Storage buffer creation for compute");
                ui.label("• Read/write buffer operations");
                ui.label("• Buffer to buffer copy");
                ui.label("• Staging buffers for CPU-GPU transfer");
                ui.label("• Buffer mapping for results retrieval");
            });

            ui.add_space(10.0);

            ui.group(|ui| {
                ui.heading("🧮 Compute Operations");
                ui.label("• Dispatch compute shaders");
                ui.label("• Indirect compute dispatch");
                ui.label("• Multiple compute passes");
                ui.label("• Synchronization and barriers");
            });

            ui.add_space(10.0);

            ui.group(|ui| {
                ui.heading("🤖 ML Inferencing Use Cases");
                ui.label("• Matrix multiplication (core ML operation)");
                ui.label("• Convolution operations");
                ui.label("• Activation functions (ReLU, sigmoid, etc.)");
                ui.label("• Tensor operations");
                ui.label("• Pooling operations (max, average)");
                ui.label("• Batch normalization");
                ui.label("• Simple neural network layers");
            });

            ui.add_space(10.0);

            ui.group(|ui| {
                ui.heading("📊 Example Workloads");
                ui.label("• Image processing (filters, transformations)");
                ui.label("• Data parallel algorithms");
                ui.label("• Reduction operations");
                ui.label("• Prefix sum / scan");
                ui.label("• Sorting algorithms on GPU");
                ui.label("• Ray tracing computations");
            });

            ui.add_space(10.0);

            ui.group(|ui| {
                ui.heading("🔧 Advanced Compute");
                ui.label("• Shared memory usage in workgroups");
                ui.label("• Atomic operations");
                ui.label("• Subgroup operations (if supported)");
                ui.label("• Compute shader debugging techniques");
                ui.label("• Performance profiling");
            });

            ui.add_space(20.0);
            ui.colored_label(
                egui::Color32::YELLOW,
                "⚠️ Placeholder - Implementation planned in future issues",
            );
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_pipeline_descriptor_creation() {
        let descriptor = ComputePipelineDescriptor::new(Some("test_compute_pipeline"));
        assert_eq!(descriptor.label(), Some("test_compute_pipeline"));
        assert!(descriptor.shader().is_none());
        assert!(descriptor.entry_point().is_none());
        assert!(descriptor.layout().is_none());
    }

    #[test]
    fn test_compute_pipeline_descriptor_with_entry_point() {
        let descriptor =
            ComputePipelineDescriptor::new(Some("test")).with_entry_point("main");

        assert_eq!(descriptor.entry_point(), Some("main"));
    }

    #[test]
    fn test_compute_pipeline_descriptor_with_shader() {
        let shader = ShaderModule::from_source(
            "@compute @workgroup_size(1) fn main() {}",
            Some("compute"),
        )
        .unwrap();

        let descriptor =
            ComputePipelineDescriptor::new(Some("test")).with_shader(shader.clone());

        assert!(descriptor.shader().is_some());
        assert_eq!(
            descriptor.shader().unwrap().source(),
            "@compute @workgroup_size(1) fn main() {}"
        );
    }

    #[test]
    fn test_compute_pipeline_validation_missing_shader() {
        let descriptor = ComputePipelineDescriptor::new(Some("test")).with_entry_point("main");

        let result = descriptor.validate();
        assert!(result.is_err());
        match result {
            Err(ComputePipelineError::MissingShader) => {}
            _ => panic!("Expected MissingShader error"),
        }
    }

    #[test]
    fn test_compute_pipeline_validation_missing_entry_point() {
        let shader =
            ShaderModule::from_source("@compute @workgroup_size(1) fn main() {}", Some("compute"))
                .unwrap();

        let descriptor = ComputePipelineDescriptor::new(Some("test")).with_shader(shader);

        let result = descriptor.validate();
        assert!(result.is_err());
        match result {
            Err(ComputePipelineError::MissingEntryPoint) => {}
            _ => panic!("Expected MissingEntryPoint error"),
        }
    }

    #[test]
    fn test_compute_pipeline_validation_empty_entry_point() {
        let shader =
            ShaderModule::from_source("@compute @workgroup_size(1) fn main() {}", Some("compute"))
                .unwrap();

        let descriptor = ComputePipelineDescriptor::new(Some("test"))
            .with_shader(shader)
            .with_entry_point("   ");

        let result = descriptor.validate();
        assert!(result.is_err());
        match result {
            Err(ComputePipelineError::InvalidConfiguration(msg)) => {
                assert!(msg.contains("Entry point name cannot be empty"));
            }
            _ => panic!("Expected InvalidConfiguration error"),
        }
    }

    #[test]
    fn test_compute_pipeline_validation_success() {
        let shader =
            ShaderModule::from_source("@compute @workgroup_size(1) fn main() {}", Some("compute"))
                .unwrap();

        let descriptor = ComputePipelineDescriptor::new(Some("test"))
            .with_shader(shader)
            .with_entry_point("main");

        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn test_compute_pipeline_descriptor_default() {
        let descriptor = ComputePipelineDescriptor::default();
        assert_eq!(descriptor.label(), None);
        assert!(descriptor.shader().is_none());
        assert!(descriptor.entry_point().is_none());
    }

    #[test]
    fn test_compute_pipeline_error_display() {
        let err = ComputePipelineError::CreationFailed("test error".to_string());
        assert_eq!(
            err.to_string(),
            "Compute pipeline creation failed: test error"
        );

        let err = ComputePipelineError::InvalidConfiguration("config error".to_string());
        assert_eq!(
            err.to_string(),
            "Invalid pipeline configuration: config error"
        );

        let err = ComputePipelineError::MissingShader;
        assert!(err.to_string().contains("Missing shader module"));

        let err = ComputePipelineError::MissingEntryPoint;
        assert!(err.to_string().contains("Missing entry point"));
    }

    #[test]
    fn test_compute_pipeline_builder_pattern() {
        let shader =
            ShaderModule::from_source("@compute @workgroup_size(1) fn compute() {}", Some("test"))
                .unwrap();

        let descriptor = ComputePipelineDescriptor::new(Some("my_pipeline"))
            .with_shader(shader)
            .with_entry_point("compute");

        assert_eq!(descriptor.label(), Some("my_pipeline"));
        assert_eq!(descriptor.entry_point(), Some("compute"));
        assert!(descriptor.shader().is_some());
        assert!(descriptor.validate().is_ok());
    }
}
