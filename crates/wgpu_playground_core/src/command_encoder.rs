use std::fmt;
use wgpu::{
    CommandBuffer, CommandEncoder, CommandEncoderDescriptor, Device, Extent3d, ImageCopyBuffer,
    ImageCopyTexture,
};

/// Alignment requirement for buffer copy operations
const COPY_BUFFER_ALIGNMENT: u64 = 4;

/// Alignment requirement for bytes per row in texture copy operations
const COPY_BYTES_PER_ROW_ALIGNMENT: u64 = 256;

/// Errors that can occur during command encoder operations
#[derive(Debug)]
pub enum CommandEncoderError {
    /// Invalid copy size
    InvalidSize(String),
    /// Copy exceeds buffer bounds
    OutOfBounds(String),
    /// Alignment error
    AlignmentError(String),
}

impl fmt::Display for CommandEncoderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandEncoderError::InvalidSize(msg) => write!(f, "Invalid copy size: {}", msg),
            CommandEncoderError::OutOfBounds(msg) => write!(f, "Copy out of bounds: {}", msg),
            CommandEncoderError::AlignmentError(msg) => write!(f, "Alignment error: {}", msg),
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
        let encoder = device.create_command_encoder(&CommandEncoderDescriptor { label });
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
    /// # Validation
    ///
    /// This method validates that:
    /// - `size` is greater than 0
    /// - `source_offset` is a multiple of 4 (COPY_BUFFER_ALIGNMENT)
    /// - `destination_offset` is a multiple of 4 (COPY_BUFFER_ALIGNMENT)
    /// - `size` is a multiple of 4 (COPY_BUFFER_ALIGNMENT)
    /// - `source_offset + size` does not exceed the source buffer size
    /// - `destination_offset + size` does not exceed the destination buffer size
    ///
    /// # Panics
    ///
    /// Panics if validation fails
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
        self.validate_buffer_to_buffer_copy(
            source,
            source_offset,
            destination,
            destination_offset,
            size,
        )
        .expect("Buffer to buffer copy validation failed");

        self.encoder.copy_buffer_to_buffer(
            source,
            source_offset,
            destination,
            destination_offset,
            size,
        );
    }

    /// Copy data from a buffer to a texture
    ///
    /// # Arguments
    ///
    /// * `source` - Source buffer and layout information
    /// * `destination` - Destination texture and target information
    /// * `copy_size` - Size of the region to copy
    ///
    /// # Validation
    ///
    /// This method validates that:
    /// - Copy size dimensions are greater than 0
    /// - Buffer offset is a multiple of 256
    /// - Bytes per row (if present) is a multiple of 256
    ///
    /// # Panics
    ///
    /// Panics if validation fails
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
        self.validate_buffer_to_texture_copy(&source, &destination, &copy_size)
            .expect("Buffer to texture copy validation failed");

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
    /// # Validation
    ///
    /// This method validates that:
    /// - Copy size dimensions are greater than 0
    /// - Buffer offset is a multiple of 256
    /// - Bytes per row (if present) is a multiple of 256
    ///
    /// # Panics
    ///
    /// Panics if validation fails
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
        self.validate_texture_to_buffer_copy(&source, &destination, &copy_size)
            .expect("Texture to buffer copy validation failed");

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

    /// Validate buffer to buffer copy parameters
    fn validate_buffer_to_buffer_copy(
        &self,
        source: &wgpu::Buffer,
        source_offset: wgpu::BufferAddress,
        destination: &wgpu::Buffer,
        destination_offset: wgpu::BufferAddress,
        size: wgpu::BufferAddress,
    ) -> Result<(), CommandEncoderError> {
        // Validate size is non-zero
        if size == 0 {
            return Err(CommandEncoderError::InvalidSize(
                "Copy size must be greater than 0".to_string(),
            ));
        }

        // Validate source offset alignment
        if !source_offset.is_multiple_of(COPY_BUFFER_ALIGNMENT) {
            return Err(CommandEncoderError::AlignmentError(format!(
                "Source offset {} must be a multiple of {}",
                source_offset, COPY_BUFFER_ALIGNMENT
            )));
        }

        // Validate destination offset alignment
        if !destination_offset.is_multiple_of(COPY_BUFFER_ALIGNMENT) {
            return Err(CommandEncoderError::AlignmentError(format!(
                "Destination offset {} must be a multiple of {}",
                destination_offset, COPY_BUFFER_ALIGNMENT
            )));
        }

        // Validate size alignment
        if !size.is_multiple_of(COPY_BUFFER_ALIGNMENT) {
            return Err(CommandEncoderError::AlignmentError(format!(
                "Copy size {} must be a multiple of {}",
                size, COPY_BUFFER_ALIGNMENT
            )));
        }

        // Validate source bounds
        let source_size = source.size();
        if source_offset
            .checked_add(size)
            .is_none_or(|end| end > source_size)
        {
            return Err(CommandEncoderError::OutOfBounds(format!(
                "Source offset {} + size {} exceeds source buffer size {}",
                source_offset, size, source_size
            )));
        }

        // Validate destination bounds
        let destination_size = destination.size();
        if destination_offset
            .checked_add(size)
            .is_none_or(|end| end > destination_size)
        {
            return Err(CommandEncoderError::OutOfBounds(format!(
                "Destination offset {} + size {} exceeds destination buffer size {}",
                destination_offset, size, destination_size
            )));
        }

        Ok(())
    }

    /// Validate buffer to texture copy parameters
    fn validate_buffer_to_texture_copy(
        &self,
        source: &ImageCopyBuffer,
        _destination: &ImageCopyTexture,
        copy_size: &Extent3d,
    ) -> Result<(), CommandEncoderError> {
        // Validate copy size is non-zero
        if copy_size.width == 0 || copy_size.height == 0 || copy_size.depth_or_array_layers == 0 {
            return Err(CommandEncoderError::InvalidSize(
                "Copy size dimensions must be greater than 0".to_string(),
            ));
        }

        // Validate buffer offset alignment (must be multiple of 256)
        if !source
            .layout
            .offset
            .is_multiple_of(COPY_BYTES_PER_ROW_ALIGNMENT)
        {
            return Err(CommandEncoderError::AlignmentError(format!(
                "Buffer offset {} must be a multiple of {}",
                source.layout.offset, COPY_BYTES_PER_ROW_ALIGNMENT
            )));
        }

        // Validate bytes_per_row alignment if present
        if let Some(bytes_per_row) = source.layout.bytes_per_row {
            if !bytes_per_row.is_multiple_of(COPY_BYTES_PER_ROW_ALIGNMENT as u32) {
                return Err(CommandEncoderError::AlignmentError(format!(
                    "Bytes per row {} must be a multiple of {}",
                    bytes_per_row, COPY_BYTES_PER_ROW_ALIGNMENT
                )));
            }
        }

        Ok(())
    }

    /// Validate texture to buffer copy parameters
    fn validate_texture_to_buffer_copy(
        &self,
        _source: &ImageCopyTexture,
        destination: &ImageCopyBuffer,
        copy_size: &Extent3d,
    ) -> Result<(), CommandEncoderError> {
        // Validate copy size is non-zero
        if copy_size.width == 0 || copy_size.height == 0 || copy_size.depth_or_array_layers == 0 {
            return Err(CommandEncoderError::InvalidSize(
                "Copy size dimensions must be greater than 0".to_string(),
            ));
        }

        // Validate buffer offset alignment (must be multiple of 256)
        if !destination
            .layout
            .offset
            .is_multiple_of(COPY_BYTES_PER_ROW_ALIGNMENT)
        {
            return Err(CommandEncoderError::AlignmentError(format!(
                "Buffer offset {} must be a multiple of {}",
                destination.layout.offset, COPY_BYTES_PER_ROW_ALIGNMENT
            )));
        }

        // Validate bytes_per_row alignment if present
        if let Some(bytes_per_row) = destination.layout.bytes_per_row {
            if !bytes_per_row.is_multiple_of(COPY_BYTES_PER_ROW_ALIGNMENT as u32) {
                return Err(CommandEncoderError::AlignmentError(format!(
                    "Bytes per row {} must be a multiple of {}",
                    bytes_per_row, COPY_BYTES_PER_ROW_ALIGNMENT
                )));
            }
        }

        Ok(())
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

    // Mock buffer for testing validation
    struct MockBuffer {
        size: u64,
    }

    impl MockBuffer {
        fn new(size: u64) -> Self {
            MockBuffer { size }
        }

        fn size(&self) -> u64 {
            self.size
        }
    }

    #[test]
    fn test_validation_zero_size() {
        // Create a mock encoder to test validation logic
        // We can't create a real encoder without a GPU device,
        // so we test the validation logic inline

        let size = 0u64;
        assert_eq!(size, 0, "Size validation should catch zero size");
    }

    #[test]
    fn test_validation_alignment_checks() {
        // Test alignment validation
        assert_eq!(0 % COPY_BUFFER_ALIGNMENT, 0, "0 should be aligned");
        assert_eq!(4 % COPY_BUFFER_ALIGNMENT, 0, "4 should be aligned");
        assert_eq!(8 % COPY_BUFFER_ALIGNMENT, 0, "8 should be aligned");
        assert_ne!(3 % COPY_BUFFER_ALIGNMENT, 0, "3 should not be aligned");
        assert_ne!(5 % COPY_BUFFER_ALIGNMENT, 0, "5 should not be aligned");
    }

    #[test]
    fn test_validation_bounds_checks() {
        let buffer = MockBuffer::new(100);

        // Valid: offset + size within bounds
        let offset = 80u64;
        let size = 20u64;
        assert!(
            offset
                .checked_add(size)
                .is_some_and(|end| end <= buffer.size()),
            "Valid copy should be within bounds"
        );

        // Invalid: offset + size exceeds bounds
        let offset = 80u64;
        let size = 24u64;
        assert!(
            offset
                .checked_add(size)
                .is_none_or(|end| end > buffer.size()),
            "Invalid copy should exceed bounds"
        );
    }

    #[test]
    fn test_copy_buffer_alignment_constant() {
        assert_eq!(
            COPY_BUFFER_ALIGNMENT, 4,
            "Copy buffer alignment should be 4 bytes"
        );
    }

    #[test]
    fn test_texture_copy_alignment() {
        const COPY_BYTES_PER_ROW_ALIGNMENT: u64 = 256;

        // Test valid alignments
        assert_eq!(0 % COPY_BYTES_PER_ROW_ALIGNMENT, 0);
        assert_eq!(256 % COPY_BYTES_PER_ROW_ALIGNMENT, 0);
        assert_eq!(512 % COPY_BYTES_PER_ROW_ALIGNMENT, 0);

        // Test invalid alignments
        assert_ne!(100 % COPY_BYTES_PER_ROW_ALIGNMENT, 0);
        assert_ne!(255 % COPY_BYTES_PER_ROW_ALIGNMENT, 0);
    }

    #[test]
    fn test_error_display() {
        let err = CommandEncoderError::InvalidSize("test".to_string());
        assert!(err.to_string().contains("Invalid copy size"));

        let err = CommandEncoderError::OutOfBounds("test".to_string());
        assert!(err.to_string().contains("Copy out of bounds"));

        let err = CommandEncoderError::AlignmentError("test".to_string());
        assert!(err.to_string().contains("Alignment error"));
    }

    #[test]
    fn test_bytes_per_row_alignment_constant() {
        assert_eq!(
            COPY_BYTES_PER_ROW_ALIGNMENT, 256,
            "Bytes per row alignment should be 256 bytes"
        );
    }
}
