use wgpu::{CommandBuffer, ImageCopyTexture, ImageDataLayout, Queue, Extent3d};

/// Error types for queue operations
///
/// Note: These error types are reserved for future use when implementing
/// more advanced queue operations with error handling. Current operations
/// use panic for errors as per wgpu's API design.
#[derive(Debug)]
pub enum QueueError {
    /// Buffer write operation failed
    BufferWriteFailed(String),
    /// Texture write operation failed
    TextureWriteFailed(String),
    /// Command submission failed
    SubmitFailed(String),
}

impl std::fmt::Display for QueueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueueError::BufferWriteFailed(msg) => {
                write!(f, "Buffer write operation failed: {}", msg)
            }
            QueueError::TextureWriteFailed(msg) => {
                write!(f, "Texture write operation failed: {}", msg)
            }
            QueueError::SubmitFailed(msg) => {
                write!(f, "Command submission failed: {}", msg)
            }
        }
    }
}

impl std::error::Error for QueueError {}

/// Abstraction for GPU queue operations
pub struct QueueOps<'a> {
    queue: &'a Queue,
}

impl<'a> QueueOps<'a> {
    /// Create a new queue operations wrapper
    pub fn new(queue: &'a Queue) -> Self {
        Self { queue }
    }

    /// Submit command buffers to the queue for execution
    ///
    /// # Arguments
    ///
    /// * `command_buffers` - Iterator of command buffers to submit
    ///
    /// # Returns
    ///
    /// Returns a submission index that can be used for synchronization
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use wgpu_playground_core::queue::QueueOps;
    /// # let queue: &wgpu::Queue = todo!();
    /// # let encoder: wgpu::CommandEncoder = todo!();
    /// let queue_ops = QueueOps::new(queue);
    /// let command_buffer = encoder.finish();
    /// let submission_index = queue_ops.submit(std::iter::once(command_buffer));
    /// ```
    pub fn submit<I>(&self, command_buffers: I) -> wgpu::SubmissionIndex
    where
        I: IntoIterator<Item = CommandBuffer>,
    {
        self.queue.submit(command_buffers)
    }

    /// Write data to a GPU buffer
    ///
    /// This operation is asynchronous and queued for execution on the GPU.
    /// Use `device.poll()` or similar mechanisms for explicit synchronization if needed.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The GPU buffer to write to
    /// * `offset` - Byte offset within the buffer
    /// * `data` - Data to write to the buffer
    ///
    /// # Panics
    ///
    /// Panics if the buffer was not created with COPY_DST usage flag
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use wgpu_playground_core::queue::QueueOps;
    /// # let queue: &wgpu::Queue = todo!();
    /// # let buffer: &wgpu::Buffer = todo!();
    /// let queue_ops = QueueOps::new(queue);
    /// let data = [1.0f32, 2.0, 3.0, 4.0];
    /// queue_ops.write_buffer(buffer, 0, bytemuck::cast_slice(&data));
    /// ```
    pub fn write_buffer(&self, buffer: &wgpu::Buffer, offset: wgpu::BufferAddress, data: &[u8]) {
        self.queue.write_buffer(buffer, offset, data);
    }

    /// Write data to a GPU texture
    ///
    /// This operation is asynchronous and queued for execution on the GPU.
    /// Use `device.poll()` or similar mechanisms for explicit synchronization if needed.
    ///
    /// # Arguments
    ///
    /// * `texture` - The GPU texture to write to (via ImageCopyTexture)
    /// * `data` - Data to write to the texture
    /// * `data_layout` - Layout of the texture data (bytes per row, rows per image)
    /// * `size` - Size of the texture region to write
    ///
    /// # Panics
    ///
    /// Panics if the texture was not created with COPY_DST usage flag
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use wgpu_playground_core::queue::QueueOps;
    /// # let queue: &wgpu::Queue = todo!();
    /// # let texture: &wgpu::Texture = todo!();
    /// let queue_ops = QueueOps::new(queue);
    /// let data = vec![0u8; 256 * 256 * 4]; // RGBA data for a 256x256 texture
    /// queue_ops.write_texture(
    ///     wgpu::ImageCopyTexture {
    ///         texture,
    ///         mip_level: 0,
    ///         origin: wgpu::Origin3d::ZERO,
    ///         aspect: wgpu::TextureAspect::All,
    ///     },
    ///     &data,
    ///     wgpu::ImageDataLayout {
    ///         offset: 0,
    ///         bytes_per_row: Some(256 * 4),
    ///         rows_per_image: Some(256),
    ///     },
    ///     wgpu::Extent3d {
    ///         width: 256,
    ///         height: 256,
    ///         depth_or_array_layers: 1,
    ///     },
    /// );
    /// ```
    pub fn write_texture(
        &self,
        texture: ImageCopyTexture,
        data: &[u8],
        data_layout: ImageDataLayout,
        size: Extent3d,
    ) {
        self.queue.write_texture(texture, data, data_layout, size);
    }

    /// Get a reference to the underlying queue
    pub fn inner(&self) -> &Queue {
        self.queue
    }
}

/// Helper function to submit a single command buffer
///
/// # Arguments
///
/// * `queue` - The GPU queue
/// * `command_buffer` - The command buffer to submit
///
/// # Returns
///
/// Returns a submission index
///
/// # Example
///
/// ```no_run
/// # use wgpu_playground_core::queue::submit_single;
/// # let queue: &wgpu::Queue = todo!();
/// # let encoder: wgpu::CommandEncoder = todo!();
/// let command_buffer = encoder.finish();
/// let submission_index = submit_single(queue, command_buffer);
/// ```
pub fn submit_single(queue: &Queue, command_buffer: CommandBuffer) -> wgpu::SubmissionIndex {
    queue.submit(std::iter::once(command_buffer))
}

/// Helper function to write buffer data with type safety
///
/// # Arguments
///
/// * `queue` - The GPU queue
/// * `buffer` - The GPU buffer to write to
/// * `offset` - Byte offset within the buffer
/// * `data` - Typed data to write to the buffer
///
/// # Type Parameters
///
/// * `T` - The type of data being written (must be Pod)
///
/// # Example
///
/// ```no_run
/// # use wgpu_playground_core::queue::write_buffer_typed;
/// # let queue: &wgpu::Queue = todo!();
/// # let buffer: &wgpu::Buffer = todo!();
/// let data = [1.0f32, 2.0, 3.0, 4.0];
/// write_buffer_typed(queue, buffer, 0, &data);
/// ```
pub fn write_buffer_typed<T: bytemuck::Pod>(
    queue: &Queue,
    buffer: &wgpu::Buffer,
    offset: wgpu::BufferAddress,
    data: &[T],
) {
    queue.write_buffer(buffer, offset, bytemuck::cast_slice(data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_error_display() {
        let err = QueueError::BufferWriteFailed("test error".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("Buffer write operation failed"));
        assert!(msg.contains("test error"));

        let err = QueueError::TextureWriteFailed("texture error".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("Texture write operation failed"));
        assert!(msg.contains("texture error"));

        let err = QueueError::SubmitFailed("submit error".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("Command submission failed"));
        assert!(msg.contains("submit error"));
    }

    #[test]
    fn test_queue_error_is_error() {
        let err = QueueError::BufferWriteFailed("test".to_string());
        let _: &dyn std::error::Error = &err;
    }
}
