use wgpu::{ComputePipeline, Device, PipelineLayout, ShaderModule};

/// Descriptor for creating a compute pipeline
#[derive(Debug, Clone)]
pub struct ComputePipelineDescriptor<'a> {
    /// Debug label for the pipeline
    label: Option<&'a str>,
    /// Pipeline layout for the compute pipeline
    layout: Option<&'a PipelineLayout>,
    /// Shader module containing the compute shader
    module: &'a ShaderModule,
    /// Entry point function name in the shader
    entry_point: &'a str,
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
    // Tests for compute pipeline are in integration tests
}
