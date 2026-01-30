use wgpu::{
    CommandBuffer, CommandEncoder, CommandEncoderDescriptor, Device, Extent3d, ImageCopyBuffer,
    ImageCopyTexture,
};

/// Error types for command encoder operations
#[derive(Debug)]
pub enum CommandEncoderError {
    /// Failed to create command encoder
    CreationFailed(String),
    /// Failed to encode command
    EncodeFailed(String),
    /// Invalid copy operation
    InvalidCopy(String),
}

impl std::fmt::Display for CommandEncoderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandEncoderError::CreationFailed(msg) => {
                write!(f, "Command encoder creation failed: {}", msg)
            }
            CommandEncoderError::EncodeFailed(msg) => {
                write!(f, "Command encode operation failed: {}", msg)
            }
            CommandEncoderError::InvalidCopy(msg) => {
                write!(f, "Invalid copy operation: {}", msg)
            }
        }
    }
}

impl std::error::Error for CommandEncoderError {}

/// Abstraction for GPU command encoder operations
///
/// The CommandEncoderOps wraps a wgpu::CommandEncoder and provides
/// convenient methods for recording GPU commands. Commands recorded
/// on the encoder are not executed until the resulting command buffer
/// is submitted to a queue.
///
/// # Example
///
/// ```no_run
/// # use wgpu_playground_core::command_encoder::CommandEncoderOps;
/// # let device: &wgpu::Device = todo!();
/// # let queue: &wgpu::Queue = todo!();
/// # let src_buffer: &wgpu::Buffer = todo!();
/// # let dst_buffer: &wgpu::Buffer = todo!();
/// // Create a command encoder
/// let mut encoder_ops = CommandEncoderOps::new(device, Some("My Encoder"));
///
/// // Record commands
/// encoder_ops.copy_buffer_to_buffer(src_buffer, 0, dst_buffer, 0, 256);
///
/// // Finish encoding and get command buffer
/// let command_buffer = encoder_ops.finish();
///
/// // Submit to queue
/// queue.submit(std::iter::once(command_buffer));
/// ```
pub struct CommandEncoderOps {
    encoder: CommandEncoder,
}

impl CommandEncoderOps {
    /// Create a new command encoder
    ///
    /// # Arguments
    ///
    /// * `device` - The GPU device
    /// * `label` - Optional label for debugging
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use wgpu_playground_core::command_encoder::CommandEncoderOps;
    /// # let device: &wgpu::Device = todo!();
    /// let encoder = CommandEncoderOps::new(device, Some("My Encoder"));
    /// ```
    pub fn new(device: &Device, label: Option<&str>) -> Self {
        let encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: label.map(|s| s.into()),
        });
        Self { encoder }
    }

    /// Copy data from one buffer to another
    ///
    /// # Arguments
    ///
    /// * `source` - Source buffer
    /// * `source_offset` - Byte offset in the source buffer
    /// * `destination` - Destination buffer
    /// * `destination_offset` - Byte offset in the destination buffer
    /// * `size` - Number of bytes to copy
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use wgpu_playground_core::command_encoder::CommandEncoderOps;
    /// # let device: &wgpu::Device = todo!();
    /// # let src: &wgpu::Buffer = todo!();
    /// # let dst: &wgpu::Buffer = todo!();
    /// let mut encoder = CommandEncoderOps::new(device, Some("Copy Encoder"));
    /// encoder.copy_buffer_to_buffer(src, 0, dst, 0, 256);
    /// ```
    pub fn copy_buffer_to_buffer(
        &mut self,
        source: &wgpu::Buffer,
        source_offset: wgpu::BufferAddress,
        destination: &wgpu::Buffer,
        destination_offset: wgpu::BufferAddress,
        size: wgpu::BufferAddress,
    ) {
        self.encoder
            .copy_buffer_to_buffer(source, source_offset, destination, destination_offset, size);
    }

    /// Copy data from a buffer to a texture
    ///
    /// # Arguments
    ///
    /// * `source` - Source buffer and layout information
    /// * `destination` - Destination texture and target information
    /// * `copy_size` - Size of the region to copy
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use wgpu_playground_core::command_encoder::CommandEncoderOps;
    /// # let device: &wgpu::Device = todo!();
    /// # let buffer: &wgpu::Buffer = todo!();
    /// # let texture: &wgpu::Texture = todo!();
    /// let mut encoder = CommandEncoderOps::new(device, Some("Buffer to Texture"));
    /// encoder.copy_buffer_to_texture(
    ///     wgpu::ImageCopyBuffer {
    ///         buffer,
    ///         layout: wgpu::ImageDataLayout {
    ///             offset: 0,
    ///             bytes_per_row: Some(256 * 4),
    ///             rows_per_image: Some(256),
    ///         },
    ///     },
    ///     wgpu::ImageCopyTexture {
    ///         texture,
    ///         mip_level: 0,
    ///         origin: wgpu::Origin3d::ZERO,
    ///         aspect: wgpu::TextureAspect::All,
    ///     },
    ///     wgpu::Extent3d {
    ///         width: 256,
    ///         height: 256,
    ///         depth_or_array_layers: 1,
    ///     },
    /// );
    /// ```
    pub fn copy_buffer_to_texture(
        &mut self,
        source: ImageCopyBuffer,
        destination: ImageCopyTexture,
        copy_size: Extent3d,
    ) {
        self.encoder
            .copy_buffer_to_texture(source, destination, copy_size);
    }

    /// Copy data from a texture to a buffer
    ///
    /// # Arguments
    ///
    /// * `source` - Source texture and region information
    /// * `destination` - Destination buffer and layout information
    /// * `copy_size` - Size of the region to copy
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use wgpu_playground_core::command_encoder::CommandEncoderOps;
    /// # let device: &wgpu::Device = todo!();
    /// # let texture: &wgpu::Texture = todo!();
    /// # let buffer: &wgpu::Buffer = todo!();
    /// let mut encoder = CommandEncoderOps::new(device, Some("Texture to Buffer"));
    /// encoder.copy_texture_to_buffer(
    ///     wgpu::ImageCopyTexture {
    ///         texture,
    ///         mip_level: 0,
    ///         origin: wgpu::Origin3d::ZERO,
    ///         aspect: wgpu::TextureAspect::All,
    ///     },
    ///     wgpu::ImageCopyBuffer {
    ///         buffer,
    ///         layout: wgpu::ImageDataLayout {
    ///             offset: 0,
    ///             bytes_per_row: Some(256 * 4),
    ///             rows_per_image: Some(256),
    ///         },
    ///     },
    ///     wgpu::Extent3d {
    ///         width: 256,
    ///         height: 256,
    ///         depth_or_array_layers: 1,
    ///     },
    /// );
    /// ```
    pub fn copy_texture_to_buffer(
        &mut self,
        source: ImageCopyTexture,
        destination: ImageCopyBuffer,
        copy_size: Extent3d,
    ) {
        self.encoder
            .copy_texture_to_buffer(source, destination, copy_size);
    }

    /// Copy data from one texture to another
    ///
    /// # Arguments
    ///
    /// * `source` - Source texture and region information
    /// * `destination` - Destination texture and target information
    /// * `copy_size` - Size of the region to copy
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use wgpu_playground_core::command_encoder::CommandEncoderOps;
    /// # let device: &wgpu::Device = todo!();
    /// # let src_texture: &wgpu::Texture = todo!();
    /// # let dst_texture: &wgpu::Texture = todo!();
    /// let mut encoder = CommandEncoderOps::new(device, Some("Texture to Texture"));
    /// encoder.copy_texture_to_texture(
    ///     wgpu::ImageCopyTexture {
    ///         texture: src_texture,
    ///         mip_level: 0,
    ///         origin: wgpu::Origin3d::ZERO,
    ///         aspect: wgpu::TextureAspect::All,
    ///     },
    ///     wgpu::ImageCopyTexture {
    ///         texture: dst_texture,
    ///         mip_level: 0,
    ///         origin: wgpu::Origin3d::ZERO,
    ///         aspect: wgpu::TextureAspect::All,
    ///     },
    ///     wgpu::Extent3d {
    ///         width: 256,
    ///         height: 256,
    ///         depth_or_array_layers: 1,
    ///     },
    /// );
    /// ```
    pub fn copy_texture_to_texture(
        &mut self,
        source: ImageCopyTexture,
        destination: ImageCopyTexture,
        copy_size: Extent3d,
    ) {
        self.encoder
            .copy_texture_to_texture(source, destination, copy_size);
    }

    /// Finish encoding and return the command buffer
    ///
    /// This consumes the encoder and produces a command buffer that can
    /// be submitted to a queue for execution.
    ///
    /// # Returns
    ///
    /// A command buffer containing all recorded commands
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use wgpu_playground_core::command_encoder::CommandEncoderOps;
    /// # let device: &wgpu::Device = todo!();
    /// # let queue: &wgpu::Queue = todo!();
    /// let mut encoder = CommandEncoderOps::new(device, Some("My Encoder"));
    /// // ... record commands ...
    /// let command_buffer = encoder.finish();
    /// queue.submit(std::iter::once(command_buffer));
    /// ```
    pub fn finish(self) -> CommandBuffer {
        self.encoder.finish()
    }

    /// Get a mutable reference to the underlying encoder
    ///
    /// This allows access to additional encoder methods not wrapped by
    /// this abstraction, such as render pass and compute pass creation.
    ///
    /// # Returns
    ///
    /// A mutable reference to the underlying wgpu::CommandEncoder
    pub fn inner_mut(&mut self) -> &mut CommandEncoder {
        &mut self.encoder
    }

    /// Get a reference to the underlying encoder
    ///
    /// # Returns
    ///
    /// A reference to the underlying wgpu::CommandEncoder
    pub fn inner(&self) -> &CommandEncoder {
        &self.encoder
    }
}

/// Helper function to create a command encoder with a label
///
/// # Arguments
///
/// * `device` - The GPU device
/// * `label` - Optional label for debugging
///
/// # Returns
///
/// A new CommandEncoderOps instance
///
/// # Example
///
/// ```no_run
/// # use wgpu_playground_core::command_encoder::create_encoder;
/// # let device: &wgpu::Device = todo!();
/// let encoder = create_encoder(device, Some("My Encoder"));
/// ```
pub fn create_encoder(device: &Device, label: Option<&str>) -> CommandEncoderOps {
    CommandEncoderOps::new(device, label)
}

/// Helper function to create and finish a command buffer with a single copy operation
///
/// # Arguments
///
/// * `device` - The GPU device
/// * `source` - Source buffer
/// * `source_offset` - Byte offset in the source buffer
/// * `destination` - Destination buffer
/// * `destination_offset` - Byte offset in the destination buffer
/// * `size` - Number of bytes to copy
///
/// # Returns
///
/// A command buffer ready to be submitted
///
/// # Example
///
/// ```no_run
/// # use wgpu_playground_core::command_encoder::copy_buffer;
/// # let device: &wgpu::Device = todo!();
/// # let queue: &wgpu::Queue = todo!();
/// # let src: &wgpu::Buffer = todo!();
/// # let dst: &wgpu::Buffer = todo!();
/// let command_buffer = copy_buffer(device, src, 0, dst, 0, 256);
/// queue.submit(std::iter::once(command_buffer));
/// ```
pub fn copy_buffer(
    device: &Device,
    source: &wgpu::Buffer,
    source_offset: wgpu::BufferAddress,
    destination: &wgpu::Buffer,
    destination_offset: wgpu::BufferAddress,
    size: wgpu::BufferAddress,
) -> CommandBuffer {
    let mut encoder = CommandEncoderOps::new(device, Some("Buffer Copy"));
    encoder.copy_buffer_to_buffer(source, source_offset, destination, destination_offset, size);
    encoder.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_encoder_error_display() {
        let err = CommandEncoderError::CreationFailed("test error".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("Command encoder creation failed"));
        assert!(msg.contains("test error"));

        let err = CommandEncoderError::EncodeFailed("encode error".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("Command encode operation failed"));
        assert!(msg.contains("encode error"));

        let err = CommandEncoderError::InvalidCopy("copy error".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("Invalid copy operation"));
        assert!(msg.contains("copy error"));
    }

    #[test]
    fn test_command_encoder_error_is_error() {
        let err = CommandEncoderError::CreationFailed("test".to_string());
        let _: &dyn std::error::Error = &err;
    }
}
