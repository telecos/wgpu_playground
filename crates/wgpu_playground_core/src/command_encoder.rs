use wgpu::{Buffer, CommandEncoder, Extent3d, ImageCopyBuffer, ImageCopyTexture};

/// Error types for command encoder operations
#[derive(Debug)]
pub enum CommandEncoderError {
    /// Invalid buffer copy size
    InvalidCopySize(String),
    /// Invalid buffer offset
    InvalidOffset(String),
    /// Invalid texture copy parameters
    InvalidTextureCopy(String),
}

impl std::fmt::Display for CommandEncoderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandEncoderError::InvalidCopySize(msg) => {
                write!(f, "Invalid copy size: {}", msg)
            }
            CommandEncoderError::InvalidOffset(msg) => {
                write!(f, "Invalid offset: {}", msg)
            }
            CommandEncoderError::InvalidTextureCopy(msg) => {
                write!(f, "Invalid texture copy: {}", msg)
            }
        }
    }
}

impl std::error::Error for CommandEncoderError {}

/// Abstraction for GPU command encoder operations
///
/// This wrapper provides validated copy operations between buffers and textures.
pub struct CommandEncoderOps<'a> {
    encoder: &'a mut CommandEncoder,
}

impl<'a> CommandEncoderOps<'a> {
    /// Create a new command encoder operations wrapper
    ///
    /// # Arguments
    ///
    /// * `encoder` - Mutable reference to a wgpu CommandEncoder
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use wgpu_playground_core::command_encoder::CommandEncoderOps;
    /// # let mut encoder: wgpu::CommandEncoder = todo!();
    /// let mut encoder_ops = CommandEncoderOps::new(&mut encoder);
    /// ```
    pub fn new(encoder: &'a mut CommandEncoder) -> Self {
        Self { encoder }
    }

    /// Copy data from one buffer to another
    ///
    /// This operation validates that:
    /// - Copy size is greater than 0
    /// - Source offset + size <= source buffer size
    /// - Destination offset + size <= destination buffer size
    /// - All offsets and sizes are properly aligned (multiples of 4)
    ///
    /// # Arguments
    ///
    /// * `source` - Source buffer (must have COPY_SRC usage)
    /// * `source_offset` - Offset in bytes in the source buffer
    /// * `destination` - Destination buffer (must have COPY_DST usage)
    /// * `destination_offset` - Offset in bytes in the destination buffer
    /// * `size` - Number of bytes to copy
    ///
    /// # Returns
    ///
    /// Returns Ok(()) if validation passes, otherwise returns CommandEncoderError
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use wgpu_playground_core::command_encoder::CommandEncoderOps;
    /// # let mut encoder: wgpu::CommandEncoder = todo!();
    /// # let src_buffer: wgpu::Buffer = todo!();
    /// # let dst_buffer: wgpu::Buffer = todo!();
    /// let mut encoder_ops = CommandEncoderOps::new(&mut encoder);
    /// encoder_ops.copy_buffer_to_buffer(&src_buffer, 0, &dst_buffer, 0, 256).unwrap();
    /// ```
    pub fn copy_buffer_to_buffer(
        &mut self,
        source: &Buffer,
        source_offset: u64,
        destination: &Buffer,
        destination_offset: u64,
        size: u64,
    ) -> Result<(), CommandEncoderError> {
        // Validate size
        if size == 0 {
            return Err(CommandEncoderError::InvalidCopySize(
                "Copy size must be greater than 0".to_string(),
            ));
        }

        // Validate size alignment (must be multiple of 4)
        if size % 4 != 0 {
            return Err(CommandEncoderError::InvalidCopySize(format!(
                "Copy size must be a multiple of 4, got {}",
                size
            )));
        }

        // Validate source offset alignment
        if source_offset % 4 != 0 {
            return Err(CommandEncoderError::InvalidOffset(format!(
                "Source offset must be a multiple of 4, got {}",
                source_offset
            )));
        }

        // Validate destination offset alignment
        if destination_offset % 4 != 0 {
            return Err(CommandEncoderError::InvalidOffset(format!(
                "Destination offset must be a multiple of 4, got {}",
                destination_offset
            )));
        }

        // Validate source buffer bounds
        let source_end = source_offset.checked_add(size);
        if source_end.is_none() {
            return Err(CommandEncoderError::InvalidCopySize(
                "Source copy range causes integer overflow".to_string(),
            ));
        }
        if source_end.unwrap() > source.size() {
            return Err(CommandEncoderError::InvalidCopySize(format!(
                "Source copy range (offset: {}, size: {}) exceeds buffer size ({})",
                source_offset,
                size,
                source.size()
            )));
        }

        // Validate destination buffer bounds
        let destination_end = destination_offset.checked_add(size);
        if destination_end.is_none() {
            return Err(CommandEncoderError::InvalidCopySize(
                "Destination copy range causes integer overflow".to_string(),
            ));
        }
        if destination_end.unwrap() > destination.size() {
            return Err(CommandEncoderError::InvalidCopySize(format!(
                "Destination copy range (offset: {}, size: {}) exceeds buffer size ({})",
                destination_offset,
                size,
                destination.size()
            )));
        }

        // Perform the copy
        self.encoder
            .copy_buffer_to_buffer(source, source_offset, destination, destination_offset, size);

        Ok(())
    }

    /// Copy data from a buffer to a texture
    ///
    /// This operation validates texture and buffer copy parameters.
    ///
    /// # Arguments
    ///
    /// * `source` - Source buffer with layout information
    /// * `destination` - Destination texture with copy information
    /// * `copy_size` - The size of the texture region to copy
    ///
    /// # Returns
    ///
    /// Returns Ok(()) if validation passes, otherwise returns CommandEncoderError
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use wgpu_playground_core::command_encoder::CommandEncoderOps;
    /// # let mut encoder: wgpu::CommandEncoder = todo!();
    /// # let buffer: wgpu::Buffer = todo!();
    /// # let texture: wgpu::Texture = todo!();
    /// let mut encoder_ops = CommandEncoderOps::new(&mut encoder);
    /// encoder_ops.copy_buffer_to_texture(
    ///     wgpu::ImageCopyBuffer {
    ///         buffer: &buffer,
    ///         layout: wgpu::ImageDataLayout {
    ///             offset: 0,
    ///             bytes_per_row: Some(256),
    ///             rows_per_image: Some(256),
    ///         },
    ///     },
    ///     wgpu::ImageCopyTexture {
    ///         texture: &texture,
    ///         mip_level: 0,
    ///         origin: wgpu::Origin3d::ZERO,
    ///         aspect: wgpu::TextureAspect::All,
    ///     },
    ///     wgpu::Extent3d {
    ///         width: 256,
    ///         height: 256,
    ///         depth_or_array_layers: 1,
    ///     },
    /// ).unwrap();
    /// ```
    pub fn copy_buffer_to_texture(
        &mut self,
        source: ImageCopyBuffer,
        destination: ImageCopyTexture,
        copy_size: Extent3d,
    ) -> Result<(), CommandEncoderError> {
        // Validate copy size
        if copy_size.width == 0 || copy_size.height == 0 || copy_size.depth_or_array_layers == 0 {
            return Err(CommandEncoderError::InvalidTextureCopy(
                "Copy size dimensions must all be greater than 0".to_string(),
            ));
        }

        // Validate that bytes_per_row is provided for textures with height > 1
        if copy_size.height > 1 && source.layout.bytes_per_row.is_none() {
            return Err(CommandEncoderError::InvalidTextureCopy(
                "bytes_per_row must be specified when copying textures with height > 1".to_string(),
            ));
        }

        // Validate that rows_per_image is provided for 3D textures or arrays
        if copy_size.depth_or_array_layers > 1 && source.layout.rows_per_image.is_none() {
            return Err(CommandEncoderError::InvalidTextureCopy(
                "rows_per_image must be specified when copying 3D textures or texture arrays"
                    .to_string(),
            ));
        }

        // Perform the copy
        self.encoder
            .copy_buffer_to_texture(source, destination, copy_size);

        Ok(())
    }

    /// Copy data from a texture to a buffer
    ///
    /// This operation validates texture and buffer copy parameters.
    ///
    /// # Arguments
    ///
    /// * `source` - Source texture with copy information
    /// * `destination` - Destination buffer with layout information
    /// * `copy_size` - The size of the texture region to copy
    ///
    /// # Returns
    ///
    /// Returns Ok(()) if validation passes, otherwise returns CommandEncoderError
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use wgpu_playground_core::command_encoder::CommandEncoderOps;
    /// # let mut encoder: wgpu::CommandEncoder = todo!();
    /// # let buffer: wgpu::Buffer = todo!();
    /// # let texture: wgpu::Texture = todo!();
    /// let mut encoder_ops = CommandEncoderOps::new(&mut encoder);
    /// encoder_ops.copy_texture_to_buffer(
    ///     wgpu::ImageCopyTexture {
    ///         texture: &texture,
    ///         mip_level: 0,
    ///         origin: wgpu::Origin3d::ZERO,
    ///         aspect: wgpu::TextureAspect::All,
    ///     },
    ///     wgpu::ImageCopyBuffer {
    ///         buffer: &buffer,
    ///         layout: wgpu::ImageDataLayout {
    ///             offset: 0,
    ///             bytes_per_row: Some(256),
    ///             rows_per_image: Some(256),
    ///         },
    ///     },
    ///     wgpu::Extent3d {
    ///         width: 256,
    ///         height: 256,
    ///         depth_or_array_layers: 1,
    ///     },
    /// ).unwrap();
    /// ```
    pub fn copy_texture_to_buffer(
        &mut self,
        source: ImageCopyTexture,
        destination: ImageCopyBuffer,
        copy_size: Extent3d,
    ) -> Result<(), CommandEncoderError> {
        // Validate copy size
        if copy_size.width == 0 || copy_size.height == 0 || copy_size.depth_or_array_layers == 0 {
            return Err(CommandEncoderError::InvalidTextureCopy(
                "Copy size dimensions must all be greater than 0".to_string(),
            ));
        }

        // Validate that bytes_per_row is provided for textures with height > 1
        if copy_size.height > 1 && destination.layout.bytes_per_row.is_none() {
            return Err(CommandEncoderError::InvalidTextureCopy(
                "bytes_per_row must be specified when copying textures with height > 1".to_string(),
            ));
        }

        // Validate that rows_per_image is provided for 3D textures or arrays
        if copy_size.depth_or_array_layers > 1 && destination.layout.rows_per_image.is_none() {
            return Err(CommandEncoderError::InvalidTextureCopy(
                "rows_per_image must be specified when copying 3D textures or texture arrays"
                    .to_string(),
            ));
        }

        // Perform the copy
        self.encoder
            .copy_texture_to_buffer(source, destination, copy_size);

        Ok(())
    }

    /// Get a reference to the underlying encoder
    pub fn inner(&self) -> &CommandEncoder {
        self.encoder
    }

    /// Get a mutable reference to the underlying encoder
    pub fn inner_mut(&mut self) -> &mut CommandEncoder {
        self.encoder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_encoder_error_display() {
        let err = CommandEncoderError::InvalidCopySize("test size error".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("Invalid copy size"));
        assert!(msg.contains("test size error"));

        let err = CommandEncoderError::InvalidOffset("test offset error".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("Invalid offset"));
        assert!(msg.contains("test offset error"));

        let err = CommandEncoderError::InvalidTextureCopy("test texture error".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("Invalid texture copy"));
        assert!(msg.contains("test texture error"));
    }

    #[test]
    fn test_command_encoder_error_is_error() {
        let err = CommandEncoderError::InvalidCopySize("test".to_string());
        let _: &dyn std::error::Error = &err;
    }
}
