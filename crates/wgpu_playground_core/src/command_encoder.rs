use wgpu::{
    BindGroup, Buffer, CommandBuffer, CommandEncoder, ComputePassDescriptor, ComputePipeline,
    Device,
};

/// Wrapper for GPU command encoder
pub struct CommandEncoderOps {
    encoder: CommandEncoder,
}

impl CommandEncoderOps {
    /// Create a new command encoder
    ///
    /// # Arguments
    ///
    /// * `device` - The GPU device
    /// * `label` - Optional debug label
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use wgpu_playground_core::command_encoder::CommandEncoderOps;
    /// # let device: &wgpu::Device = todo!();
    /// let encoder = CommandEncoderOps::new(device, Some("My Encoder"));
    /// ```
    pub fn new(device: &Device, label: Option<&str>) -> Self {
        let encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label });
        Self { encoder }
    }

    /// Begin a compute pass
    ///
    /// # Arguments
    ///
    /// * `label` - Optional debug label for the compute pass
    ///
    /// # Returns
    ///
    /// Returns a ComputePassEncoderOps for recording compute commands
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use wgpu_playground_core::command_encoder::CommandEncoderOps;
    /// # let device: &wgpu::Device = todo!();
    /// let mut encoder = CommandEncoderOps::new(device, None);
    /// let mut compute_pass = encoder.begin_compute_pass(Some("My Compute Pass"));
    /// // ... record compute commands
    /// drop(compute_pass); // End compute pass
    /// ```
    pub fn begin_compute_pass(&mut self, label: Option<&str>) -> ComputePassEncoderOps<'_> {
        let descriptor = ComputePassDescriptor {
            label,
            timestamp_writes: None,
        };
        let pass = self.encoder.begin_compute_pass(&descriptor);
        ComputePassEncoderOps { pass }
    }

    /// Finish encoding and create a command buffer
    ///
    /// # Returns
    ///
    /// Returns the created command buffer ready for submission
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use wgpu_playground_core::command_encoder::CommandEncoderOps;
    /// # let device: &wgpu::Device = todo!();
    /// # let queue: &wgpu::Queue = todo!();
    /// let encoder = CommandEncoderOps::new(device, None);
    /// let command_buffer = encoder.finish();
    /// queue.submit(std::iter::once(command_buffer));
    /// ```
    pub fn finish(self) -> CommandBuffer {
        self.encoder.finish()
    }

    /// Get a mutable reference to the underlying encoder
    pub fn inner_mut(&mut self) -> &mut CommandEncoder {
        &mut self.encoder
    }
}

/// Wrapper for compute pass encoder with dispatch operations
pub struct ComputePassEncoderOps<'a> {
    pass: wgpu::ComputePass<'a>,
}

impl<'a> ComputePassEncoderOps<'a> {
    /// Set the active compute pipeline
    ///
    /// # Arguments
    ///
    /// * `pipeline` - The compute pipeline to use
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use wgpu_playground_core::command_encoder::CommandEncoderOps;
    /// # let device: &wgpu::Device = todo!();
    /// # let pipeline: &wgpu::ComputePipeline = todo!();
    /// let mut encoder = CommandEncoderOps::new(device, None);
    /// let mut compute_pass = encoder.begin_compute_pass(None);
    /// compute_pass.set_pipeline(pipeline);
    /// ```
    pub fn set_pipeline(&mut self, pipeline: &'a ComputePipeline) {
        self.pass.set_pipeline(pipeline);
    }

    /// Set a bind group for the compute pass
    ///
    /// # Arguments
    ///
    /// * `index` - The bind group index (must match pipeline layout)
    /// * `bind_group` - The bind group to bind
    /// * `offsets` - Dynamic offsets for dynamic uniform/storage buffers (pass empty slice if none)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use wgpu_playground_core::command_encoder::CommandEncoderOps;
    /// # let device: &wgpu::Device = todo!();
    /// # let bind_group: &wgpu::BindGroup = todo!();
    /// let mut encoder = CommandEncoderOps::new(device, None);
    /// let mut compute_pass = encoder.begin_compute_pass(None);
    /// compute_pass.set_bind_group(0, bind_group, &[]);
    /// ```
    pub fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: &'a BindGroup,
        offsets: &[wgpu::DynamicOffset],
    ) {
        self.pass.set_bind_group(index, bind_group, offsets);
    }

    /// Dispatch compute workgroups
    ///
    /// Dispatches a grid of workgroups to execute the compute shader.
    ///
    /// # Arguments
    ///
    /// * `workgroup_count_x` - Number of workgroups in X dimension
    /// * `workgroup_count_y` - Number of workgroups in Y dimension
    /// * `workgroup_count_z` - Number of workgroups in Z dimension
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use wgpu_playground_core::command_encoder::CommandEncoderOps;
    /// # let device: &wgpu::Device = todo!();
    /// # let pipeline: &wgpu::ComputePipeline = todo!();
    /// let mut encoder = CommandEncoderOps::new(device, None);
    /// let mut compute_pass = encoder.begin_compute_pass(None);
    /// compute_pass.set_pipeline(pipeline);
    /// compute_pass.dispatch_workgroups(64, 1, 1); // Dispatch 64 workgroups in X
    /// ```
    pub fn dispatch_workgroups(
        &mut self,
        workgroup_count_x: u32,
        workgroup_count_y: u32,
        workgroup_count_z: u32,
    ) {
        self.pass
            .dispatch_workgroups(workgroup_count_x, workgroup_count_y, workgroup_count_z);
    }

    /// Dispatch compute workgroups indirectly
    ///
    /// Reads dispatch parameters from a buffer.
    ///
    /// # Arguments
    ///
    /// * `indirect_buffer` - Buffer containing dispatch parameters (3 u32s: x, y, z)
    /// * `indirect_offset` - Byte offset into the buffer (must be multiple of 4)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use wgpu_playground_core::command_encoder::CommandEncoderOps;
    /// # let device: &wgpu::Device = todo!();
    /// # let pipeline: &wgpu::ComputePipeline = todo!();
    /// # let indirect_buffer: &wgpu::Buffer = todo!();
    /// let mut encoder = CommandEncoderOps::new(device, None);
    /// let mut compute_pass = encoder.begin_compute_pass(None);
    /// compute_pass.set_pipeline(pipeline);
    /// compute_pass.dispatch_workgroups_indirect(indirect_buffer, 0);
    /// ```
    pub fn dispatch_workgroups_indirect(&mut self, indirect_buffer: &'a Buffer, indirect_offset: u64) {
        self.pass
            .dispatch_workgroups_indirect(indirect_buffer, indirect_offset);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a test device
    async fn create_test_device() -> Option<wgpu::Device> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .await?;

        adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: Some("Test Device"),
                    memory_hints: Default::default(),
                },
                None,
            )
            .await
            .ok()
            .map(|(device, _)| device)
    }

    #[test]
    fn test_command_encoder_creation() {
        pollster::block_on(async {
            let Some(device) = create_test_device().await else {
                eprintln!("Skipping test: No GPU adapter available");
                return;
            };

            let encoder = CommandEncoderOps::new(&device, Some("Test Encoder"));
            let _command_buffer = encoder.finish();
        });
    }

    #[test]
    fn test_compute_pass_creation() {
        pollster::block_on(async {
            let Some(device) = create_test_device().await else {
                eprintln!("Skipping test: No GPU adapter available");
                return;
            };

            let mut encoder = CommandEncoderOps::new(&device, None);
            let _compute_pass = encoder.begin_compute_pass(Some("Test Compute Pass"));
        });
    }
}
