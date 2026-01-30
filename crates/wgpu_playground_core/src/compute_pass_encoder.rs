use std::fmt;
use wgpu::{Buffer, CommandEncoder, ComputePass, ComputePipeline};

/// Errors that can occur during compute pass operations
#[derive(Debug)]
pub enum ComputePassError {
    /// Invalid compute pass configuration
    InvalidConfiguration(String),
}

impl fmt::Display for ComputePassError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComputePassError::InvalidConfiguration(msg) => {
                write!(f, "Invalid compute pass configuration: {}", msg)
            }
        }
    }
}

impl std::error::Error for ComputePassError {}

/// Compute pass descriptor
///
/// Describes a compute pass that will execute compute shaders.
/// Unlike render passes, compute passes don't have attachments.
#[derive(Debug)]
pub struct ComputePassDescriptor<'a> {
    /// Label for debugging
    pub label: Option<&'a str>,
}

impl<'a> ComputePassDescriptor<'a> {
    /// Create a new compute pass descriptor
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::compute_pass_encoder::ComputePassDescriptor;
    ///
    /// let descriptor = ComputePassDescriptor::new();
    /// ```
    pub fn new() -> Self {
        Self { label: None }
    }

    /// Set the label for debugging
    ///
    /// # Arguments
    /// * `label` - The debug label
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::compute_pass_encoder::ComputePassDescriptor;
    ///
    /// let descriptor = ComputePassDescriptor::new()
    ///     .with_label("My Compute Pass");
    /// ```
    pub fn with_label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    /// Validate the compute pass descriptor
    ///
    /// Currently, compute passes don't have complex validation requirements,
    /// but this method is provided for consistency and future extensibility.
    pub fn validate(&self) -> Result<(), ComputePassError> {
        Ok(())
    }
}

impl<'a> Default for ComputePassDescriptor<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// Wrapper for wgpu::ComputePass with convenient methods
///
/// ComputePassEncoder provides methods to record compute operations including
/// pipeline binding, bind group setting, and dispatch operations.
///
/// # Examples
/// ```no_run
/// # use wgpu_playground_core::compute_pass_encoder::{ComputePassEncoder, ComputePassDescriptor};
/// # let mut encoder: wgpu::CommandEncoder = todo!();
/// # let pipeline: &wgpu::ComputePipeline = todo!();
/// # let bind_group: &wgpu::BindGroup = todo!();
/// let descriptor = ComputePassDescriptor::new()
///     .with_label("My Compute Pass");
///
/// let mut compute_pass = ComputePassEncoder::begin(&mut encoder, &descriptor).unwrap();
/// compute_pass.set_pipeline(pipeline);
/// compute_pass.set_bind_group(0, bind_group, &[]);
/// compute_pass.dispatch(64, 1, 1);
/// ```
pub struct ComputePassEncoder<'a> {
    pass: ComputePass<'a>,
}

impl<'a> ComputePassEncoder<'a> {
    /// Create a new compute pass encoder from a command encoder
    ///
    /// # Arguments
    /// * `encoder` - The command encoder
    /// * `descriptor` - The compute pass descriptor
    ///
    /// # Returns
    /// Returns a Result containing the compute pass encoder or an error
    ///
    /// # Examples
    /// ```no_run
    /// # use wgpu_playground_core::compute_pass_encoder::{ComputePassEncoder, ComputePassDescriptor};
    /// # let mut encoder: wgpu::CommandEncoder = todo!();
    /// let descriptor = ComputePassDescriptor::new()
    ///     .with_label("My Compute Pass");
    ///
    /// let compute_pass = ComputePassEncoder::begin(&mut encoder, &descriptor).unwrap();
    /// ```
    pub fn begin(
        encoder: &'a mut CommandEncoder,
        descriptor: &ComputePassDescriptor<'a>,
    ) -> Result<Self, ComputePassError> {
        descriptor.validate()?;

        let pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: descriptor.label,
            timestamp_writes: None,
        });

        Ok(Self { pass })
    }

    /// Set the current compute pipeline
    ///
    /// # Arguments
    /// * `pipeline` - The compute pipeline to use
    ///
    /// # Examples
    /// ```no_run
    /// # use wgpu_playground_core::compute_pass_encoder::ComputePassEncoder;
    /// # let mut compute_pass: ComputePassEncoder = todo!();
    /// # let pipeline: &wgpu::ComputePipeline = todo!();
    /// compute_pass.set_pipeline(pipeline);
    /// ```
    pub fn set_pipeline(&mut self, pipeline: &'a ComputePipeline) {
        self.pass.set_pipeline(pipeline);
    }

    /// Set the bind group for the given index
    ///
    /// # Arguments
    /// * `index` - The bind group index
    /// * `bind_group` - The bind group to bind
    /// * `offsets` - Dynamic offsets for dynamic uniform/storage buffers
    ///
    /// # Examples
    /// ```no_run
    /// # use wgpu_playground_core::compute_pass_encoder::ComputePassEncoder;
    /// # let mut compute_pass: ComputePassEncoder = todo!();
    /// # let bind_group: &wgpu::BindGroup = todo!();
    /// compute_pass.set_bind_group(0, bind_group, &[]);
    /// ```
    pub fn set_bind_group(&mut self, index: u32, bind_group: &'a wgpu::BindGroup, offsets: &[u32]) {
        self.pass.set_bind_group(index, bind_group, offsets);
    }

    /// Dispatch compute work
    ///
    /// Dispatches a grid of workgroups with the specified dimensions.
    /// Each dimension represents the number of workgroups to dispatch in that axis.
    ///
    /// # Arguments
    /// * `workgroups_x` - Number of workgroups in the X dimension
    /// * `workgroups_y` - Number of workgroups in the Y dimension
    /// * `workgroups_z` - Number of workgroups in the Z dimension
    ///
    /// # Examples
    /// ```no_run
    /// # use wgpu_playground_core::compute_pass_encoder::ComputePassEncoder;
    /// # let mut compute_pass: ComputePassEncoder = todo!();
    /// // Dispatch 64 workgroups in X, 1 in Y and Z
    /// compute_pass.dispatch(64, 1, 1);
    /// ```
    pub fn dispatch(&mut self, workgroups_x: u32, workgroups_y: u32, workgroups_z: u32) {
        self.pass.dispatch_workgroups(workgroups_x, workgroups_y, workgroups_z);
    }

    /// Dispatch compute work using an indirect buffer
    ///
    /// The indirect buffer must contain a structure with three u32 values
    /// representing workgroups_x, workgroups_y, and workgroups_z.
    ///
    /// # Arguments
    /// * `indirect_buffer` - Buffer containing dispatch parameters
    /// * `indirect_offset` - Offset in bytes into the indirect buffer
    ///
    /// # Examples
    /// ```no_run
    /// # use wgpu_playground_core::compute_pass_encoder::ComputePassEncoder;
    /// # let mut compute_pass: ComputePassEncoder = todo!();
    /// # let indirect_buffer: &wgpu::Buffer = todo!();
    /// compute_pass.dispatch_indirect(indirect_buffer, 0);
    /// ```
    pub fn dispatch_indirect(&mut self, indirect_buffer: &'a Buffer, indirect_offset: u64) {
        self.pass.dispatch_workgroups_indirect(indirect_buffer, indirect_offset);
    }

    /// Insert a debug marker
    ///
    /// # Arguments
    /// * `label` - The debug marker label
    ///
    /// # Examples
    /// ```no_run
    /// # use wgpu_playground_core::compute_pass_encoder::ComputePassEncoder;
    /// # let mut compute_pass: ComputePassEncoder = todo!();
    /// compute_pass.insert_debug_marker("First compute step");
    /// ```
    pub fn insert_debug_marker(&mut self, label: &str) {
        self.pass.insert_debug_marker(label);
    }

    /// Push a debug group
    ///
    /// # Arguments
    /// * `label` - The debug group label
    ///
    /// # Examples
    /// ```no_run
    /// # use wgpu_playground_core::compute_pass_encoder::ComputePassEncoder;
    /// # let mut compute_pass: ComputePassEncoder = todo!();
    /// compute_pass.push_debug_group("Compute operations");
    /// // ... compute operations ...
    /// compute_pass.pop_debug_group();
    /// ```
    pub fn push_debug_group(&mut self, label: &str) {
        self.pass.push_debug_group(label);
    }

    /// Pop a debug group
    ///
    /// # Examples
    /// ```no_run
    /// # use wgpu_playground_core::compute_pass_encoder::ComputePassEncoder;
    /// # let mut compute_pass: ComputePassEncoder = todo!();
    /// compute_pass.push_debug_group("Compute operations");
    /// // ... compute operations ...
    /// compute_pass.pop_debug_group();
    /// ```
    pub fn pop_debug_group(&mut self) {
        self.pass.pop_debug_group();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_pass_error_display() {
        let err = ComputePassError::InvalidConfiguration("test".to_string());
        assert!(format!("{}", err).contains("Invalid compute pass configuration"));
    }

    #[test]
    fn test_compute_pass_descriptor_creation() {
        let descriptor = ComputePassDescriptor::new();
        assert_eq!(descriptor.label, None);
    }

    #[test]
    fn test_compute_pass_descriptor_with_label() {
        let descriptor = ComputePassDescriptor::new()
            .with_label("test_pass");
        assert_eq!(descriptor.label, Some("test_pass"));
    }

    #[test]
    fn test_compute_pass_descriptor_default() {
        let descriptor = ComputePassDescriptor::default();
        assert_eq!(descriptor.label, None);
    }

    #[test]
    fn test_compute_pass_descriptor_validation() {
        let descriptor = ComputePassDescriptor::new();
        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn test_compute_pass_descriptor_builder() {
        let descriptor = ComputePassDescriptor::new()
            .with_label("my_compute_pass");
        assert_eq!(descriptor.label, Some("my_compute_pass"));
        assert!(descriptor.validate().is_ok());
    }
}
