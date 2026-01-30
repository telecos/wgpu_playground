use std::fmt;
use wgpu::{ComputePipeline, Device, PipelineLayout, ShaderModule};

/// Errors that can occur during compute pipeline operations
#[derive(Debug)]
pub enum ComputePipelineError {
    /// Failed to create compute pipeline
    CreationFailed(String),
    /// Invalid configuration
    InvalidConfiguration(String),
    /// Missing required shader
    MissingShader(String),
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
            ComputePipelineError::MissingShader(msg) => write!(f, "Missing shader: {}", msg),
        }
    }
}

impl std::error::Error for ComputePipelineError {}

/// Descriptor for creating a compute pipeline
#[derive(Debug, Clone)]
pub struct ComputePipelineDescriptor<'a> {
    /// Debug label for the pipeline
    pub label: Option<&'a str>,
    /// Pipeline layout for the compute pipeline
    pub layout: Option<&'a PipelineLayout>,
    /// Shader module containing the compute shader
    pub module: &'a ShaderModule,
    /// Entry point function name in the shader
    pub entry_point: &'a str,
}

impl<'a> ComputePipelineDescriptor<'a> {
    /// Create a new compute pipeline descriptor
    ///
    /// # Arguments
    ///
    /// * `module` - The shader module containing the compute shader
    /// * `entry_point` - The name of the entry point function
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use wgpu_playground_core::compute_pipeline::ComputePipelineDescriptor;
    /// # let module: &wgpu::ShaderModule = todo!();
    /// let descriptor = ComputePipelineDescriptor::new(module, "main");
    /// ```
    pub fn new(module: &'a ShaderModule, entry_point: &'a str) -> Self {
        Self {
            label: None,
            layout: None,
            module,
            entry_point,
        }
    }

    /// Set the debug label for the pipeline
    pub fn with_label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    /// Set the pipeline layout
    pub fn with_layout(mut self, layout: &'a PipelineLayout) -> Self {
        self.layout = Some(layout);
        self
    }

    /// Convert to wgpu::ComputePipelineDescriptor
    pub fn to_wgpu(&self) -> wgpu::ComputePipelineDescriptor<'_> {
        wgpu::ComputePipelineDescriptor {
            label: self.label,
            layout: self.layout,
            module: self.module,
            entry_point: self.entry_point,
            compilation_options: Default::default(),
            cache: None,
        }
    }
}

/// Create a compute pipeline from a descriptor
///
/// # Arguments
///
/// * `device` - The GPU device
/// * `descriptor` - The compute pipeline descriptor
///
/// # Returns
///
/// Returns the created compute pipeline
///
/// # Examples
///
/// ```no_run
/// # use wgpu_playground_core::compute_pipeline::{create_compute_pipeline, ComputePipelineDescriptor};
/// # let device: &wgpu::Device = todo!();
/// # let module: &wgpu::ShaderModule = todo!();
/// let descriptor = ComputePipelineDescriptor::new(module, "main");
/// let pipeline = create_compute_pipeline(device, &descriptor);
/// ```
pub fn create_compute_pipeline(
    device: &Device,
    descriptor: &ComputePipelineDescriptor,
) -> ComputePipeline {
    device.create_compute_pipeline(&descriptor.to_wgpu())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_pipeline_error_display() {
        let err = ComputePipelineError::CreationFailed("test error".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("Compute pipeline creation failed"));
        assert!(msg.contains("test error"));

        let err = ComputePipelineError::InvalidConfiguration("config error".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("Invalid pipeline configuration"));
        assert!(msg.contains("config error"));

        let err = ComputePipelineError::MissingShader("shader error".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("Missing shader"));
        assert!(msg.contains("shader error"));
    }

    #[test]
    fn test_compute_pipeline_error_is_error() {
        let err = ComputePipelineError::CreationFailed("test".to_string());
        let _: &dyn std::error::Error = &err;
    }
}
