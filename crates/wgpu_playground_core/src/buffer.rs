use std::fmt;
use wgpu::{Buffer, BufferAsyncError, BufferView, Device, MapMode};

/// Errors that can occur during buffer operations
#[derive(Debug)]
pub enum BufferError {
    /// Failed to create buffer
    CreationFailed(String),
    /// Failed to map buffer
    MapFailed(String),
    /// Buffer is already mapped
    AlreadyMapped,
    /// Buffer is not mapped
    NotMapped,
    /// Invalid buffer size
    InvalidSize(String),
    /// Invalid buffer usage
    InvalidUsage(String),
}

impl fmt::Display for BufferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BufferError::CreationFailed(msg) => write!(f, "Buffer creation failed: {}", msg),
            BufferError::MapFailed(msg) => write!(f, "Buffer mapping failed: {}", msg),
            BufferError::AlreadyMapped => write!(f, "Buffer is already mapped"),
            BufferError::NotMapped => write!(f, "Buffer is not mapped"),
            BufferError::InvalidSize(msg) => write!(f, "Invalid buffer size: {}", msg),
            BufferError::InvalidUsage(msg) => write!(f, "Invalid buffer usage: {}", msg),
        }
    }
}

impl std::error::Error for BufferError {}

impl From<BufferAsyncError> for BufferError {
    fn from(err: BufferAsyncError) -> Self {
        BufferError::MapFailed(err.to_string())
    }
}

/// Buffer usage flags
///
/// These flags determine how a buffer can be used in GPU operations.
/// Multiple flags can be combined using the bitwise OR operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferUsages {
    bits: u32,
}

impl BufferUsages {
    /// Empty usage flags
    pub const NONE: Self = Self { bits: 0 };

    /// Buffer can be used as a vertex buffer
    pub const VERTEX: Self = Self { bits: 1 << 0 };

    /// Buffer can be used as an index buffer
    pub const INDEX: Self = Self { bits: 1 << 1 };

    /// Buffer can be used as a uniform buffer
    pub const UNIFORM: Self = Self { bits: 1 << 2 };

    /// Buffer can be used as a storage buffer
    pub const STORAGE: Self = Self { bits: 1 << 3 };

    /// Buffer can be used as an indirect buffer (for draw indirect commands)
    pub const INDIRECT: Self = Self { bits: 1 << 4 };

    /// Buffer can be used as a copy source
    pub const COPY_SRC: Self = Self { bits: 1 << 5 };

    /// Buffer can be used as a copy destination
    pub const COPY_DST: Self = Self { bits: 1 << 6 };

    /// Buffer can be mapped for reading
    pub const MAP_READ: Self = Self { bits: 1 << 7 };

    /// Buffer can be mapped for writing
    pub const MAP_WRITE: Self = Self { bits: 1 << 8 };

    /// Buffer can be used to resolve query results
    pub const QUERY_RESOLVE: Self = Self { bits: 1 << 9 };

    /// Create empty buffer usages
    pub const fn empty() -> Self {
        Self::NONE
    }

    /// Check if this usage set contains all flags from another set
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits & other.bits) == other.bits
    }

    /// Check if this usage set is empty
    pub const fn is_empty(&self) -> bool {
        self.bits == 0
    }

    /// Combine this usage with another
    pub const fn union(self, other: Self) -> Self {
        Self {
            bits: self.bits | other.bits,
        }
    }

    /// Convert to wgpu::BufferUsages
    pub fn to_wgpu(&self) -> wgpu::BufferUsages {
        let mut usage = wgpu::BufferUsages::empty();

        if self.contains(Self::VERTEX) {
            usage |= wgpu::BufferUsages::VERTEX;
        }
        if self.contains(Self::INDEX) {
            usage |= wgpu::BufferUsages::INDEX;
        }
        if self.contains(Self::UNIFORM) {
            usage |= wgpu::BufferUsages::UNIFORM;
        }
        if self.contains(Self::STORAGE) {
            usage |= wgpu::BufferUsages::STORAGE;
        }
        if self.contains(Self::INDIRECT) {
            usage |= wgpu::BufferUsages::INDIRECT;
        }
        if self.contains(Self::COPY_SRC) {
            usage |= wgpu::BufferUsages::COPY_SRC;
        }
        if self.contains(Self::COPY_DST) {
            usage |= wgpu::BufferUsages::COPY_DST;
        }
        if self.contains(Self::MAP_READ) {
            usage |= wgpu::BufferUsages::MAP_READ;
        }
        if self.contains(Self::MAP_WRITE) {
            usage |= wgpu::BufferUsages::MAP_WRITE;
        }
        if self.contains(Self::QUERY_RESOLVE) {
            usage |= wgpu::BufferUsages::QUERY_RESOLVE;
        }

        usage
    }

    /// Create from wgpu::BufferUsages
    pub fn from_wgpu(usage: wgpu::BufferUsages) -> Self {
        let mut result = Self::empty();

        if usage.contains(wgpu::BufferUsages::VERTEX) {
            result = result.union(Self::VERTEX);
        }
        if usage.contains(wgpu::BufferUsages::INDEX) {
            result = result.union(Self::INDEX);
        }
        if usage.contains(wgpu::BufferUsages::UNIFORM) {
            result = result.union(Self::UNIFORM);
        }
        if usage.contains(wgpu::BufferUsages::STORAGE) {
            result = result.union(Self::STORAGE);
        }
        if usage.contains(wgpu::BufferUsages::INDIRECT) {
            result = result.union(Self::INDIRECT);
        }
        if usage.contains(wgpu::BufferUsages::COPY_SRC) {
            result = result.union(Self::COPY_SRC);
        }
        if usage.contains(wgpu::BufferUsages::COPY_DST) {
            result = result.union(Self::COPY_DST);
        }
        if usage.contains(wgpu::BufferUsages::MAP_READ) {
            result = result.union(Self::MAP_READ);
        }
        if usage.contains(wgpu::BufferUsages::MAP_WRITE) {
            result = result.union(Self::MAP_WRITE);
        }
        if usage.contains(wgpu::BufferUsages::QUERY_RESOLVE) {
            result = result.union(Self::QUERY_RESOLVE);
        }

        result
    }
}

impl std::ops::BitOr for BufferUsages {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl std::ops::BitOrAssign for BufferUsages {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

/// Descriptor for creating a GPU buffer
#[derive(Debug, Clone)]
pub struct BufferDescriptor {
    /// Optional label for debugging
    label: Option<String>,
    /// Size of the buffer in bytes
    size: u64,
    /// How the buffer will be used
    usage: BufferUsages,
    /// Whether the buffer should be mapped at creation
    mapped_at_creation: bool,
}

impl BufferDescriptor {
    /// Create a new buffer descriptor
    ///
    /// # Arguments
    /// * `label` - Optional label for debugging
    /// * `size` - Size of the buffer in bytes
    /// * `usage` - How the buffer will be used
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::buffer::{BufferDescriptor, BufferUsages};
    ///
    /// let descriptor = BufferDescriptor::new(
    ///     Some("vertex_buffer"),
    ///     1024,
    ///     BufferUsages::VERTEX | BufferUsages::COPY_DST
    /// );
    /// ```
    pub fn new(label: Option<&str>, size: u64, usage: BufferUsages) -> Self {
        Self {
            label: label.map(String::from),
            size,
            usage,
            mapped_at_creation: false,
        }
    }

    /// Set whether the buffer should be mapped at creation
    ///
    /// When a buffer is mapped at creation, it can be immediately written to
    /// without an additional map operation. This is useful for uploading initial data.
    ///
    /// # Arguments
    ///
    /// * `mapped` - Whether to map the buffer at creation
    ///
    /// # Returns
    ///
    /// Self for method chaining
    ///
    /// # Examples
    ///
    /// Create and immediately write to a buffer:
    /// ```no_run
    /// use wgpu_playground_core::buffer::{BufferDescriptor, BufferUsages};
    /// # fn example(device: &wgpu::Device) {
    /// let descriptor = BufferDescriptor::new(
    ///     Some("vertex_buffer"),
    ///     256,
    ///     BufferUsages::VERTEX | BufferUsages::COPY_DST
    /// ).with_mapped_at_creation(true);
    ///
    /// let buffer = descriptor.create_buffer(device).unwrap();
    /// // Buffer is now mapped and ready for writing
    /// // Use buffer.slice(..).get_mapped_range_mut() to write data
    /// // Then buffer.unmap() when done
    /// # }
    /// ```
    ///
    /// # Notes
    ///
    /// - The buffer starts in a mapped state after creation
    /// - You must call `unmap()` before using the buffer in GPU operations
    /// - More efficient than creating then mapping when you have data to upload
    pub fn with_mapped_at_creation(mut self, mapped: bool) -> Self {
        self.mapped_at_creation = mapped;
        self
    }

    /// Get the label
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Get the size
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Get the usage flags
    pub fn usage(&self) -> BufferUsages {
        self.usage
    }

    /// Get the mapped_at_creation flag
    pub fn mapped_at_creation(&self) -> bool {
        self.mapped_at_creation
    }

    /// Validate the buffer descriptor
    ///
    /// Checks for:
    /// - Non-zero size
    /// - Valid usage combinations
    ///
    /// # Returns
    /// Ok(()) if valid, Err with BufferError if invalid
    pub fn validate(&self) -> Result<(), BufferError> {
        if self.size == 0 {
            return Err(BufferError::InvalidSize(
                "Buffer size must be greater than 0".to_string(),
            ));
        }

        if self.usage.is_empty() {
            return Err(BufferError::InvalidUsage(
                "Buffer must have at least one usage flag".to_string(),
            ));
        }

        // MAP_READ and MAP_WRITE cannot be used together
        if self.usage.contains(BufferUsages::MAP_READ)
            && self.usage.contains(BufferUsages::MAP_WRITE)
        {
            return Err(BufferError::InvalidUsage(
                "Buffer cannot have both MAP_READ and MAP_WRITE flags".to_string(),
            ));
        }

        Ok(())
    }

    /// Convert to wgpu::BufferDescriptor
    pub fn to_wgpu(&self) -> wgpu::BufferDescriptor<'_> {
        wgpu::BufferDescriptor {
            label: self.label.as_deref(),
            size: self.size,
            usage: self.usage.to_wgpu(),
            mapped_at_creation: self.mapped_at_creation,
        }
    }

    /// Create a GPU buffer from this descriptor
    ///
    /// This method validates the descriptor and creates the actual buffer.
    ///
    /// # Arguments
    /// * `device` - The wgpu device to create the buffer on
    ///
    /// # Returns
    /// A Result containing the Buffer or a BufferError
    ///
    /// # Examples
    /// ```no_run
    /// use wgpu_playground_core::buffer::{BufferDescriptor, BufferUsages};
    /// # async fn example(device: &wgpu::Device) {
    /// let descriptor = BufferDescriptor::new(
    ///     Some("my_buffer"),
    ///     256,
    ///     BufferUsages::UNIFORM | BufferUsages::COPY_DST
    /// );
    ///
    /// let buffer = descriptor.create_buffer(device).unwrap();
    /// # }
    /// ```
    pub fn create_buffer(&self, device: &Device) -> Result<Buffer, BufferError> {
        self.validate()?;
        log::debug!(
            "Creating buffer: label={:?}, size={}, usage={:?}",
            self.label,
            self.size,
            self.usage
        );

        // Track API usage
        #[cfg(not(test))]
        {
            use crate::api_coverage::{ApiCategory, ApiCoverageTracker};
            ApiCoverageTracker::global().record(ApiCategory::Buffer, "create_buffer");
        }

        let buffer = device.create_buffer(&self.to_wgpu());
        log::trace!("Buffer created successfully");
        Ok(buffer)
    }
}

impl Default for BufferDescriptor {
    fn default() -> Self {
        Self::new(None, 64, BufferUsages::COPY_DST)
    }
}

/// Helper functions for buffer mapping operations
pub struct BufferOps;

impl BufferOps {
    /// Map a buffer for reading
    ///
    /// This is an asynchronous operation that must complete before the buffer can be read.
    ///
    /// # Arguments
    /// * `buffer` - The buffer to map
    ///
    /// # Returns
    /// A future that resolves when the mapping is complete
    ///
    /// # Panics
    /// Panics if the buffer was not created with MAP_READ usage
    ///
    /// # Examples
    /// ```no_run
    /// use wgpu_playground_core::buffer::{BufferOps, BufferDescriptor, BufferUsages};
    /// # async fn example(device: &wgpu::Device) {
    /// let descriptor = BufferDescriptor::new(
    ///     Some("read_buffer"),
    ///     256,
    ///     BufferUsages::MAP_READ | BufferUsages::COPY_DST
    /// );
    /// let buffer = descriptor.create_buffer(device).unwrap();
    ///
    /// BufferOps::map_read(&buffer).await.unwrap();
    /// // ... read from buffer ...
    /// BufferOps::unmap(&buffer);
    /// # }
    /// ```
    pub async fn map_read(buffer: &Buffer) -> Result<(), BufferError> {
        log::debug!("Mapping buffer for reading");

        // Track API usage
        #[cfg(not(test))]
        {
            use crate::api_coverage::{ApiCategory, ApiCoverageTracker};
            ApiCoverageTracker::global().record(ApiCategory::Buffer, "map_read");
        }

        let (sender, receiver) = futures_channel::oneshot::channel();

        buffer.slice(..).map_async(MapMode::Read, move |result| {
            let _ = sender.send(result);
        });

        receiver.await.unwrap()?;
        log::trace!("Buffer mapped for reading successfully");
        Ok(())
    }

    /// Map a buffer for writing
    ///
    /// This is an asynchronous operation that must complete before the buffer can be written.
    ///
    /// # Arguments
    /// * `buffer` - The buffer to map
    ///
    /// # Returns
    /// A future that resolves when the mapping is complete
    ///
    /// # Panics
    /// Panics if the buffer was not created with MAP_WRITE usage
    ///
    /// # Examples
    /// ```no_run
    /// use wgpu_playground_core::buffer::{BufferOps, BufferDescriptor, BufferUsages};
    /// # async fn example(device: &wgpu::Device) {
    /// let descriptor = BufferDescriptor::new(
    ///     Some("write_buffer"),
    ///     256,
    ///     BufferUsages::MAP_WRITE | BufferUsages::COPY_SRC
    /// );
    /// let buffer = descriptor.create_buffer(device).unwrap();
    ///
    /// BufferOps::map_write(&buffer).await.unwrap();
    /// // ... write to buffer ...
    /// BufferOps::unmap(&buffer);
    /// # }
    /// ```
    pub async fn map_write(buffer: &Buffer) -> Result<(), BufferError> {
        log::debug!("Mapping buffer for writing");

        // Track API usage
        #[cfg(not(test))]
        {
            use crate::api_coverage::{ApiCategory, ApiCoverageTracker};
            ApiCoverageTracker::global().record(ApiCategory::Buffer, "map_write");
        }

        let (sender, receiver) = futures_channel::oneshot::channel();

        buffer.slice(..).map_async(MapMode::Write, move |result| {
            let _ = sender.send(result);
        });

        receiver.await.unwrap()?;
        log::trace!("Buffer mapped for writing successfully");
        Ok(())
    }

    /// Unmap a previously mapped buffer
    ///
    /// # Arguments
    /// * `buffer` - The buffer to unmap
    ///
    /// # Examples
    /// ```no_run
    /// use wgpu_playground_core::buffer::{BufferOps, BufferDescriptor, BufferUsages};
    /// # async fn example(device: &wgpu::Device) {
    /// # let descriptor = BufferDescriptor::new(Some("buffer"), 256, BufferUsages::MAP_READ | BufferUsages::COPY_DST);
    /// # let buffer = descriptor.create_buffer(device).unwrap();
    /// BufferOps::map_read(&buffer).await.unwrap();
    /// // ... use buffer ...
    /// BufferOps::unmap(&buffer);
    /// # }
    /// ```
    pub fn unmap(buffer: &Buffer) {
        log::trace!("Unmapping buffer");
        buffer.unmap();
    }

    /// Get a read-only view of a mapped buffer
    ///
    /// # Arguments
    /// * `buffer` - The buffer to get a view of (must be mapped for reading)
    ///
    /// # Returns
    /// A BufferView that can be used to read the buffer data
    ///
    /// # Examples
    /// ```no_run
    /// use wgpu_playground_core::buffer::{BufferOps, BufferDescriptor, BufferUsages};
    /// # async fn example(device: &wgpu::Device) {
    /// # let descriptor = BufferDescriptor::new(Some("buffer"), 256, BufferUsages::MAP_READ | BufferUsages::COPY_DST);
    /// # let buffer = descriptor.create_buffer(device).unwrap();
    /// BufferOps::map_read(&buffer).await.unwrap();
    /// let view = BufferOps::get_mapped_range(&buffer);
    /// // ... read from view ...
    /// drop(view);
    /// BufferOps::unmap(&buffer);
    /// # }
    /// ```
    pub fn get_mapped_range(buffer: &Buffer) -> BufferView {
        buffer.slice(..).get_mapped_range()
    }

    /// Get a mutable view of a mapped buffer
    ///
    /// # Arguments
    /// * `buffer` - The buffer to get a view of (must be mapped for writing)
    ///
    /// # Returns
    /// A mutable BufferView that can be used to write to the buffer
    ///
    /// # Examples
    /// ```no_run
    /// use wgpu_playground_core::buffer::{BufferOps, BufferDescriptor, BufferUsages};
    /// # async fn example(device: &wgpu::Device) {
    /// # let descriptor = BufferDescriptor::new(Some("buffer"), 256, BufferUsages::MAP_WRITE | BufferUsages::COPY_SRC);
    /// # let buffer = descriptor.create_buffer(device).unwrap();
    /// BufferOps::map_write(&buffer).await.unwrap();
    /// let mut view = BufferOps::get_mapped_range_mut(&buffer);
    /// // ... write to view ...
    /// drop(view);
    /// BufferOps::unmap(&buffer);
    /// # }
    /// ```
    pub fn get_mapped_range_mut(buffer: &Buffer) -> wgpu::BufferViewMut {
        buffer.slice(..).get_mapped_range_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_usages_empty() {
        let usage = BufferUsages::empty();
        assert!(usage.is_empty());
    }

    #[test]
    fn test_buffer_usages_single_flag() {
        let usage = BufferUsages::VERTEX;
        assert!(usage.contains(BufferUsages::VERTEX));
        assert!(!usage.contains(BufferUsages::INDEX));
    }

    #[test]
    fn test_buffer_usages_multiple_flags() {
        let usage = BufferUsages::VERTEX | BufferUsages::INDEX;
        assert!(usage.contains(BufferUsages::VERTEX));
        assert!(usage.contains(BufferUsages::INDEX));
        assert!(!usage.contains(BufferUsages::UNIFORM));
    }

    #[test]
    fn test_buffer_usages_all_flags() {
        let usage = BufferUsages::VERTEX
            | BufferUsages::INDEX
            | BufferUsages::UNIFORM
            | BufferUsages::STORAGE
            | BufferUsages::INDIRECT
            | BufferUsages::COPY_SRC
            | BufferUsages::COPY_DST
            | BufferUsages::QUERY_RESOLVE;

        assert!(usage.contains(BufferUsages::VERTEX));
        assert!(usage.contains(BufferUsages::INDEX));
        assert!(usage.contains(BufferUsages::UNIFORM));
        assert!(usage.contains(BufferUsages::STORAGE));
        assert!(usage.contains(BufferUsages::INDIRECT));
        assert!(usage.contains(BufferUsages::COPY_SRC));
        assert!(usage.contains(BufferUsages::COPY_DST));
        assert!(usage.contains(BufferUsages::QUERY_RESOLVE));
    }

    #[test]
    fn test_buffer_usages_union() {
        let usage1 = BufferUsages::VERTEX;
        let usage2 = BufferUsages::INDEX;
        let combined = usage1.union(usage2);

        assert!(combined.contains(BufferUsages::VERTEX));
        assert!(combined.contains(BufferUsages::INDEX));
    }

    #[test]
    fn test_buffer_descriptor_creation() {
        let descriptor = BufferDescriptor::new(
            Some("test_buffer"),
            1024,
            BufferUsages::VERTEX | BufferUsages::COPY_DST,
        );

        assert_eq!(descriptor.label(), Some("test_buffer"));
        assert_eq!(descriptor.size(), 1024);
        assert!(descriptor.usage().contains(BufferUsages::VERTEX));
        assert!(descriptor.usage().contains(BufferUsages::COPY_DST));
        assert!(!descriptor.mapped_at_creation());
    }

    #[test]
    fn test_buffer_descriptor_mapped_at_creation() {
        let descriptor =
            BufferDescriptor::new(None, 256, BufferUsages::MAP_WRITE | BufferUsages::COPY_SRC)
                .with_mapped_at_creation(true);

        assert!(descriptor.mapped_at_creation());
    }

    #[test]
    fn test_buffer_descriptor_validation_success() {
        let descriptor = BufferDescriptor::new(
            Some("valid_buffer"),
            256,
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        );

        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn test_buffer_descriptor_validation_zero_size() {
        let descriptor = BufferDescriptor::new(None, 0, BufferUsages::UNIFORM);

        let result = descriptor.validate();
        assert!(result.is_err());

        match result {
            Err(BufferError::InvalidSize(msg)) => {
                assert!(msg.contains("greater than 0"));
            }
            _ => panic!("Expected InvalidSize error"),
        }
    }

    #[test]
    fn test_buffer_descriptor_validation_empty_usage() {
        let descriptor = BufferDescriptor::new(None, 256, BufferUsages::empty());

        let result = descriptor.validate();
        assert!(result.is_err());

        match result {
            Err(BufferError::InvalidUsage(msg)) => {
                assert!(msg.contains("at least one usage flag"));
            }
            _ => panic!("Expected InvalidUsage error"),
        }
    }

    #[test]
    fn test_buffer_descriptor_validation_map_read_and_write() {
        let descriptor =
            BufferDescriptor::new(None, 256, BufferUsages::MAP_READ | BufferUsages::MAP_WRITE);

        let result = descriptor.validate();
        assert!(result.is_err());

        match result {
            Err(BufferError::InvalidUsage(msg)) => {
                assert!(msg.contains("MAP_READ and MAP_WRITE"));
            }
            _ => panic!("Expected InvalidUsage error"),
        }
    }

    #[test]
    fn test_buffer_error_display() {
        let err = BufferError::CreationFailed("test error".to_string());
        assert_eq!(err.to_string(), "Buffer creation failed: test error");

        let err = BufferError::MapFailed("map error".to_string());
        assert_eq!(err.to_string(), "Buffer mapping failed: map error");

        let err = BufferError::AlreadyMapped;
        assert_eq!(err.to_string(), "Buffer is already mapped");

        let err = BufferError::NotMapped;
        assert_eq!(err.to_string(), "Buffer is not mapped");
    }

    #[test]
    fn test_buffer_usages_bitor_assign() {
        let mut usage = BufferUsages::VERTEX;
        usage |= BufferUsages::INDEX;

        assert!(usage.contains(BufferUsages::VERTEX));
        assert!(usage.contains(BufferUsages::INDEX));
    }

    #[test]
    fn test_buffer_usages_map_flags() {
        let read_usage = BufferUsages::MAP_READ | BufferUsages::COPY_DST;
        assert!(read_usage.contains(BufferUsages::MAP_READ));
        assert!(read_usage.contains(BufferUsages::COPY_DST));

        let write_usage = BufferUsages::MAP_WRITE | BufferUsages::COPY_SRC;
        assert!(write_usage.contains(BufferUsages::MAP_WRITE));
        assert!(write_usage.contains(BufferUsages::COPY_SRC));
    }

    #[test]
    fn test_buffer_descriptor_default() {
        let descriptor = BufferDescriptor::default();
        assert_eq!(descriptor.size(), 64);
        assert!(descriptor.usage().contains(BufferUsages::COPY_DST));
    }

    // Additional comprehensive tests for buffer usages

    #[test]
    fn test_buffer_usages_vertex_combinations() {
        // VERTEX can be combined with COPY_DST for data upload
        let usage = BufferUsages::VERTEX | BufferUsages::COPY_DST;
        assert!(usage.contains(BufferUsages::VERTEX));
        assert!(usage.contains(BufferUsages::COPY_DST));

        // VERTEX can be combined with COPY_SRC for reading back
        let usage = BufferUsages::VERTEX | BufferUsages::COPY_SRC;
        assert!(usage.contains(BufferUsages::VERTEX));
        assert!(usage.contains(BufferUsages::COPY_SRC));

        // VERTEX with both COPY flags
        let usage = BufferUsages::VERTEX | BufferUsages::COPY_DST | BufferUsages::COPY_SRC;
        assert!(usage.contains(BufferUsages::VERTEX));
        assert!(usage.contains(BufferUsages::COPY_DST));
        assert!(usage.contains(BufferUsages::COPY_SRC));
    }

    #[test]
    fn test_buffer_usages_index_combinations() {
        let usage = BufferUsages::INDEX | BufferUsages::COPY_DST;
        assert!(usage.contains(BufferUsages::INDEX));
        assert!(usage.contains(BufferUsages::COPY_DST));
    }

    #[test]
    fn test_buffer_usages_uniform_combinations() {
        let usage = BufferUsages::UNIFORM | BufferUsages::COPY_DST;
        assert!(usage.contains(BufferUsages::UNIFORM));
        assert!(usage.contains(BufferUsages::COPY_DST));

        // UNIFORM can be combined with COPY_SRC
        let usage = BufferUsages::UNIFORM | BufferUsages::COPY_SRC | BufferUsages::COPY_DST;
        assert!(usage.contains(BufferUsages::UNIFORM));
        assert!(usage.contains(BufferUsages::COPY_SRC));
        assert!(usage.contains(BufferUsages::COPY_DST));
    }

    #[test]
    fn test_buffer_usages_storage_combinations() {
        // STORAGE often needs both COPY flags for read/write
        let usage = BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC;
        assert!(usage.contains(BufferUsages::STORAGE));
        assert!(usage.contains(BufferUsages::COPY_DST));
        assert!(usage.contains(BufferUsages::COPY_SRC));
    }

    #[test]
    fn test_buffer_usages_indirect_combinations() {
        let usage = BufferUsages::INDIRECT | BufferUsages::COPY_DST;
        assert!(usage.contains(BufferUsages::INDIRECT));
        assert!(usage.contains(BufferUsages::COPY_DST));

        // INDIRECT with STORAGE for compute-generated indirect commands
        let usage = BufferUsages::INDIRECT | BufferUsages::STORAGE | BufferUsages::COPY_DST;
        assert!(usage.contains(BufferUsages::INDIRECT));
        assert!(usage.contains(BufferUsages::STORAGE));
        assert!(usage.contains(BufferUsages::COPY_DST));
    }

    #[test]
    fn test_buffer_usages_query_resolve_combinations() {
        let usage = BufferUsages::QUERY_RESOLVE | BufferUsages::COPY_SRC;
        assert!(usage.contains(BufferUsages::QUERY_RESOLVE));
        assert!(usage.contains(BufferUsages::COPY_SRC));
    }

    #[test]
    fn test_buffer_usages_map_read_combinations() {
        // MAP_READ is typically combined with COPY_DST
        let usage = BufferUsages::MAP_READ | BufferUsages::COPY_DST;
        assert!(usage.contains(BufferUsages::MAP_READ));
        assert!(usage.contains(BufferUsages::COPY_DST));
        assert!(!usage.contains(BufferUsages::MAP_WRITE));
    }

    #[test]
    fn test_buffer_usages_map_write_combinations() {
        // MAP_WRITE is typically combined with COPY_SRC
        let usage = BufferUsages::MAP_WRITE | BufferUsages::COPY_SRC;
        assert!(usage.contains(BufferUsages::MAP_WRITE));
        assert!(usage.contains(BufferUsages::COPY_SRC));
        assert!(!usage.contains(BufferUsages::MAP_READ));
    }

    #[test]
    fn test_buffer_usages_copy_only() {
        // Buffer used only for copying
        let usage = BufferUsages::COPY_SRC | BufferUsages::COPY_DST;
        assert!(usage.contains(BufferUsages::COPY_SRC));
        assert!(usage.contains(BufferUsages::COPY_DST));
        assert!(!usage.contains(BufferUsages::VERTEX));
        assert!(!usage.contains(BufferUsages::UNIFORM));
    }

    #[test]
    fn test_buffer_usages_all_gpu_usages() {
        // All GPU-side usage flags (no MAP flags)
        let usage = BufferUsages::VERTEX
            | BufferUsages::INDEX
            | BufferUsages::UNIFORM
            | BufferUsages::STORAGE
            | BufferUsages::INDIRECT
            | BufferUsages::COPY_SRC
            | BufferUsages::COPY_DST
            | BufferUsages::QUERY_RESOLVE;

        assert!(usage.contains(BufferUsages::VERTEX));
        assert!(usage.contains(BufferUsages::INDEX));
        assert!(usage.contains(BufferUsages::UNIFORM));
        assert!(usage.contains(BufferUsages::STORAGE));
        assert!(usage.contains(BufferUsages::INDIRECT));
        assert!(usage.contains(BufferUsages::COPY_SRC));
        assert!(usage.contains(BufferUsages::COPY_DST));
        assert!(usage.contains(BufferUsages::QUERY_RESOLVE));
        assert!(!usage.contains(BufferUsages::MAP_READ));
        assert!(!usage.contains(BufferUsages::MAP_WRITE));
    }

    #[test]
    fn test_buffer_usages_contains_empty() {
        let usage = BufferUsages::VERTEX;
        assert!(usage.contains(BufferUsages::empty()));
    }

    #[test]
    fn test_buffer_usages_multiple_union() {
        let usage1 = BufferUsages::VERTEX;
        let usage2 = BufferUsages::INDEX;
        let usage3 = BufferUsages::UNIFORM;

        let combined = usage1.union(usage2).union(usage3);
        assert!(combined.contains(BufferUsages::VERTEX));
        assert!(combined.contains(BufferUsages::INDEX));
        assert!(combined.contains(BufferUsages::UNIFORM));
    }

    #[test]
    fn test_buffer_usages_bitor_multiple() {
        let usage = BufferUsages::VERTEX | BufferUsages::INDEX | BufferUsages::UNIFORM;
        assert!(usage.contains(BufferUsages::VERTEX));
        assert!(usage.contains(BufferUsages::INDEX));
        assert!(usage.contains(BufferUsages::UNIFORM));
    }

    #[test]
    fn test_buffer_usages_none_constant() {
        assert_eq!(BufferUsages::NONE.bits, 0);
        assert!(BufferUsages::NONE.is_empty());
        assert_eq!(BufferUsages::NONE, BufferUsages::empty());
    }

    // BufferDescriptor tests

    #[test]
    fn test_buffer_descriptor_no_label() {
        let descriptor = BufferDescriptor::new(None, 512, BufferUsages::VERTEX);
        assert_eq!(descriptor.label(), None);
        assert_eq!(descriptor.size(), 512);
    }

    #[test]
    fn test_buffer_descriptor_with_label() {
        let descriptor = BufferDescriptor::new(Some("test"), 512, BufferUsages::VERTEX);
        assert_eq!(descriptor.label(), Some("test"));
    }

    #[test]
    fn test_buffer_descriptor_large_size() {
        let large_size = 1024 * 1024 * 100; // 100 MB
        let descriptor = BufferDescriptor::new(Some("large"), large_size, BufferUsages::STORAGE);
        assert_eq!(descriptor.size(), large_size);
    }

    #[test]
    fn test_buffer_descriptor_small_size() {
        let descriptor = BufferDescriptor::new(Some("small"), 16, BufferUsages::UNIFORM);
        assert_eq!(descriptor.size(), 16);
        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn test_buffer_descriptor_mapped_at_creation_default() {
        let descriptor = BufferDescriptor::new(Some("test"), 256, BufferUsages::VERTEX);
        assert!(!descriptor.mapped_at_creation());
    }

    #[test]
    fn test_buffer_descriptor_chaining() {
        let descriptor = BufferDescriptor::new(Some("test"), 256, BufferUsages::VERTEX)
            .with_mapped_at_creation(true);

        assert_eq!(descriptor.label(), Some("test"));
        assert_eq!(descriptor.size(), 256);
        assert!(descriptor.usage().contains(BufferUsages::VERTEX));
        assert!(descriptor.mapped_at_creation());
    }

    #[test]
    fn test_buffer_descriptor_validation_vertex_buffer() {
        let descriptor = BufferDescriptor::new(
            Some("vertex"),
            1024,
            BufferUsages::VERTEX | BufferUsages::COPY_DST,
        );
        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn test_buffer_descriptor_validation_index_buffer() {
        let descriptor = BufferDescriptor::new(
            Some("index"),
            512,
            BufferUsages::INDEX | BufferUsages::COPY_DST,
        );
        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn test_buffer_descriptor_validation_uniform_buffer() {
        let descriptor = BufferDescriptor::new(
            Some("uniform"),
            256,
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        );
        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn test_buffer_descriptor_validation_storage_buffer() {
        let descriptor = BufferDescriptor::new(
            Some("storage"),
            2048,
            BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
        );
        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn test_buffer_descriptor_validation_map_read_valid() {
        let descriptor = BufferDescriptor::new(
            Some("read"),
            256,
            BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        );
        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn test_buffer_descriptor_validation_map_write_valid() {
        let descriptor = BufferDescriptor::new(
            Some("write"),
            256,
            BufferUsages::MAP_WRITE | BufferUsages::COPY_SRC,
        );
        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn test_buffer_descriptor_validation_staging_buffer() {
        // Staging buffer for uploading data
        let descriptor = BufferDescriptor::new(
            Some("staging_upload"),
            1024,
            BufferUsages::MAP_WRITE | BufferUsages::COPY_SRC,
        );
        assert!(descriptor.validate().is_ok());

        // Staging buffer for downloading data
        let descriptor = BufferDescriptor::new(
            Some("staging_download"),
            1024,
            BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        );
        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn test_buffer_descriptor_validation_all_valid_combinations() {
        // Test many valid combinations
        let valid_combinations = vec![
            BufferUsages::VERTEX | BufferUsages::COPY_DST,
            BufferUsages::INDEX | BufferUsages::COPY_DST,
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            BufferUsages::STORAGE | BufferUsages::COPY_DST,
            BufferUsages::INDIRECT | BufferUsages::COPY_DST,
            BufferUsages::QUERY_RESOLVE | BufferUsages::COPY_SRC,
            BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            BufferUsages::MAP_WRITE | BufferUsages::COPY_SRC,
            BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
            BufferUsages::VERTEX | BufferUsages::INDEX,
            BufferUsages::STORAGE | BufferUsages::INDIRECT,
        ];

        for usage in valid_combinations {
            let descriptor = BufferDescriptor::new(Some("test"), 256, usage);
            assert!(
                descriptor.validate().is_ok(),
                "Expected usage {:?} to be valid",
                usage
            );
        }
    }

    // Error condition tests

    #[test]
    fn test_buffer_error_invalid_size_message() {
        let descriptor = BufferDescriptor::new(Some("invalid"), 0, BufferUsages::VERTEX);
        match descriptor.validate() {
            Err(BufferError::InvalidSize(msg)) => {
                assert!(msg.contains("greater than 0"));
            }
            _ => panic!("Expected InvalidSize error"),
        }
    }

    #[test]
    fn test_buffer_error_invalid_usage_message() {
        let descriptor = BufferDescriptor::new(Some("invalid"), 256, BufferUsages::empty());
        match descriptor.validate() {
            Err(BufferError::InvalidUsage(msg)) => {
                assert!(msg.contains("at least one usage flag"));
            }
            _ => panic!("Expected InvalidUsage error"),
        }
    }

    #[test]
    fn test_buffer_error_map_read_write_conflict() {
        let descriptor = BufferDescriptor::new(
            Some("invalid"),
            256,
            BufferUsages::MAP_READ | BufferUsages::MAP_WRITE,
        );
        match descriptor.validate() {
            Err(BufferError::InvalidUsage(msg)) => {
                assert!(msg.contains("MAP_READ and MAP_WRITE"));
            }
            _ => panic!("Expected InvalidUsage error"),
        }
    }

    #[test]
    fn test_buffer_error_display_all_variants() {
        let errors = vec![
            (
                BufferError::CreationFailed("creation".to_string()),
                "Buffer creation failed: creation",
            ),
            (
                BufferError::MapFailed("mapping".to_string()),
                "Buffer mapping failed: mapping",
            ),
            (BufferError::AlreadyMapped, "Buffer is already mapped"),
            (BufferError::NotMapped, "Buffer is not mapped"),
            (
                BufferError::InvalidSize("size error".to_string()),
                "Invalid buffer size: size error",
            ),
            (
                BufferError::InvalidUsage("usage error".to_string()),
                "Invalid buffer usage: usage error",
            ),
        ];

        for (error, expected) in errors {
            assert_eq!(error.to_string(), expected);
        }
    }

    #[test]
    fn test_buffer_error_is_error_trait() {
        // Verify BufferError implements std::error::Error
        let err: Box<dyn std::error::Error> =
            Box::new(BufferError::CreationFailed("test".to_string()));
        assert!(err.to_string().contains("Buffer creation failed"));
    }

    // BufferUsages conversion tests

    #[test]
    fn test_buffer_usages_to_wgpu_conversion() {
        let usage = BufferUsages::VERTEX | BufferUsages::COPY_DST;
        let wgpu_usage = usage.to_wgpu();
        assert!(wgpu_usage.contains(wgpu::BufferUsages::VERTEX));
        assert!(wgpu_usage.contains(wgpu::BufferUsages::COPY_DST));
    }

    #[test]
    fn test_buffer_usages_from_wgpu_conversion() {
        let wgpu_usage = wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST;
        let usage = BufferUsages::from_wgpu(wgpu_usage);
        assert!(usage.contains(BufferUsages::VERTEX));
        assert!(usage.contains(BufferUsages::COPY_DST));
    }

    #[test]
    fn test_buffer_usages_round_trip_conversion() {
        let original = BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC;
        let wgpu_usage = original.to_wgpu();
        let converted = BufferUsages::from_wgpu(wgpu_usage);

        assert_eq!(original, converted);
    }

    #[test]
    fn test_buffer_usages_all_flags_conversion() {
        let usage = BufferUsages::VERTEX
            | BufferUsages::INDEX
            | BufferUsages::UNIFORM
            | BufferUsages::STORAGE
            | BufferUsages::INDIRECT
            | BufferUsages::COPY_SRC
            | BufferUsages::COPY_DST
            | BufferUsages::MAP_READ
            | BufferUsages::QUERY_RESOLVE;

        let wgpu_usage = usage.to_wgpu();
        let converted = BufferUsages::from_wgpu(wgpu_usage);

        // Verify all flags are preserved (except MAP_WRITE which wasn't in original)
        assert!(converted.contains(BufferUsages::VERTEX));
        assert!(converted.contains(BufferUsages::INDEX));
        assert!(converted.contains(BufferUsages::UNIFORM));
        assert!(converted.contains(BufferUsages::STORAGE));
        assert!(converted.contains(BufferUsages::INDIRECT));
        assert!(converted.contains(BufferUsages::COPY_SRC));
        assert!(converted.contains(BufferUsages::COPY_DST));
        assert!(converted.contains(BufferUsages::MAP_READ));
        assert!(converted.contains(BufferUsages::QUERY_RESOLVE));
    }

    #[test]
    fn test_buffer_usages_empty_conversion() {
        let usage = BufferUsages::empty();
        let wgpu_usage = usage.to_wgpu();
        assert!(wgpu_usage.is_empty());

        let converted = BufferUsages::from_wgpu(wgpu_usage);
        assert!(converted.is_empty());
    }

    #[test]
    fn test_buffer_descriptor_to_wgpu_descriptor() {
        let descriptor = BufferDescriptor::new(
            Some("test_buffer"),
            1024,
            BufferUsages::VERTEX | BufferUsages::COPY_DST,
        )
        .with_mapped_at_creation(true);

        let wgpu_descriptor = descriptor.to_wgpu();

        assert_eq!(wgpu_descriptor.label, Some("test_buffer"));
        assert_eq!(wgpu_descriptor.size, 1024);
        assert!(wgpu_descriptor.usage.contains(wgpu::BufferUsages::VERTEX));
        assert!(wgpu_descriptor.usage.contains(wgpu::BufferUsages::COPY_DST));
        assert!(wgpu_descriptor.mapped_at_creation);
    }

    // Edge case tests

    #[test]
    fn test_buffer_descriptor_minimum_valid_size() {
        let descriptor = BufferDescriptor::new(Some("min"), 1, BufferUsages::VERTEX);
        assert!(descriptor.validate().is_ok());
        assert_eq!(descriptor.size(), 1);
    }

    #[test]
    fn test_buffer_descriptor_maximum_u64_size() {
        // While this might not be allocatable, it should validate
        let descriptor = BufferDescriptor::new(Some("max"), u64::MAX, BufferUsages::VERTEX);
        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn test_buffer_usages_equality() {
        let usage1 = BufferUsages::VERTEX | BufferUsages::COPY_DST;
        let usage2 = BufferUsages::VERTEX | BufferUsages::COPY_DST;
        let usage3 = BufferUsages::VERTEX;

        assert_eq!(usage1, usage2);
        assert_ne!(usage1, usage3);
    }

    #[test]
    fn test_buffer_descriptor_clone() {
        let original = BufferDescriptor::new(
            Some("original"),
            512,
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        );

        let cloned = original.clone();
        assert_eq!(original.label(), cloned.label());
        assert_eq!(original.size(), cloned.size());
        assert_eq!(original.usage(), cloned.usage());
        assert_eq!(original.mapped_at_creation(), cloned.mapped_at_creation());
    }

    #[test]
    fn test_buffer_usages_debug_format() {
        let usage = BufferUsages::VERTEX;
        let debug_str = format!("{:?}", usage);
        assert!(debug_str.contains("BufferUsages"));
    }

    #[test]
    fn test_buffer_descriptor_debug_format() {
        let descriptor = BufferDescriptor::new(Some("test"), 256, BufferUsages::VERTEX);
        let debug_str = format!("{:?}", descriptor);
        assert!(debug_str.contains("BufferDescriptor"));
    }
}
