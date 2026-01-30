use std::fmt;
use wgpu::{Device, QuerySet};

/// Errors that can occur during query set operations
#[derive(Debug)]
pub enum QuerySetError {
    /// Invalid query set configuration
    InvalidConfiguration(String),
    /// Invalid query count
    InvalidCount(String),
}

impl fmt::Display for QuerySetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuerySetError::InvalidConfiguration(msg) => {
                write!(f, "Invalid query set configuration: {}", msg)
            }
            QuerySetError::InvalidCount(msg) => {
                write!(f, "Invalid query count: {}", msg)
            }
        }
    }
}

impl std::error::Error for QuerySetError {}

/// Type of queries supported by a query set
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryType {
    /// Occlusion queries - count the number of samples that pass depth/stencil testing
    Occlusion,
    /// Timestamp queries - capture GPU timestamps for performance measurement
    Timestamp,
}

impl QueryType {
    /// Convert to wgpu::QueryType
    pub fn to_wgpu(&self) -> wgpu::QueryType {
        match self {
            QueryType::Occlusion => wgpu::QueryType::Occlusion,
            QueryType::Timestamp => wgpu::QueryType::Timestamp,
        }
    }

    /// Create from wgpu::QueryType
    pub fn from_wgpu(query_type: wgpu::QueryType) -> Self {
        match query_type {
            wgpu::QueryType::Occlusion => QueryType::Occlusion,
            wgpu::QueryType::Timestamp => QueryType::Timestamp,
            _ => QueryType::Timestamp, // Default fallback
        }
    }
}

/// Descriptor for creating a GPU query set
///
/// Query sets are used to capture GPU metrics like occlusion testing results
/// and timestamps for performance profiling.
#[derive(Debug, Clone)]
pub struct QuerySetDescriptor {
    /// Optional label for debugging
    label: Option<String>,
    /// Type of queries this set will contain
    query_type: QueryType,
    /// Number of queries in the set
    count: u32,
}

impl QuerySetDescriptor {
    /// Create a new query set descriptor
    ///
    /// # Arguments
    /// * `label` - Optional label for debugging
    /// * `query_type` - Type of queries (Occlusion or Timestamp)
    /// * `count` - Number of queries in the set
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::query_set::{QuerySetDescriptor, QueryType};
    ///
    /// let descriptor = QuerySetDescriptor::new(
    ///     Some("timestamp_queries"),
    ///     QueryType::Timestamp,
    ///     2
    /// );
    /// ```
    pub fn new(label: Option<&str>, query_type: QueryType, count: u32) -> Self {
        Self {
            label: label.map(String::from),
            query_type,
            count,
        }
    }

    /// Get the label
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Get the query type
    pub fn query_type(&self) -> QueryType {
        self.query_type
    }

    /// Get the count
    pub fn count(&self) -> u32 {
        self.count
    }

    /// Validate the query set descriptor
    ///
    /// Checks for:
    /// - Non-zero count
    ///
    /// # Returns
    /// Ok(()) if valid, Err with QuerySetError if invalid
    pub fn validate(&self) -> Result<(), QuerySetError> {
        if self.count == 0 {
            return Err(QuerySetError::InvalidCount(
                "Query set count must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }

    /// Convert to wgpu::QuerySetDescriptor
    pub fn to_wgpu(&self) -> wgpu::QuerySetDescriptor<'_> {
        wgpu::QuerySetDescriptor {
            label: self.label.as_deref(),
            ty: self.query_type.to_wgpu(),
            count: self.count,
        }
    }

    /// Create a GPU query set from this descriptor
    ///
    /// This method validates the descriptor and creates the actual query set.
    ///
    /// # Arguments
    /// * `device` - The wgpu device to create the query set on
    ///
    /// # Returns
    /// A Result containing the QuerySet or a QuerySetError
    ///
    /// # Examples
    /// ```no_run
    /// use wgpu_playground_core::query_set::{QuerySetDescriptor, QueryType};
    /// # fn example(device: &wgpu::Device) {
    /// let descriptor = QuerySetDescriptor::new(
    ///     Some("my_queries"),
    ///     QueryType::Timestamp,
    ///     4
    /// );
    ///
    /// let query_set = descriptor.create_query_set(device).unwrap();
    /// # }
    /// ```
    pub fn create_query_set(&self, device: &Device) -> Result<QuerySet, QuerySetError> {
        self.validate()?;
        Ok(device.create_query_set(&self.to_wgpu()))
    }
}

impl Default for QuerySetDescriptor {
    fn default() -> Self {
        Self::new(None, QueryType::Timestamp, 2)
    }
}

/// Helper functions for query set operations
pub struct QuerySetOps;

impl QuerySetOps {
    /// Resolve query results from a query set into a buffer
    ///
    /// This operation copies query results from the query set into a buffer
    /// for CPU access. The buffer must have QUERY_RESOLVE usage.
    ///
    /// # Arguments
    /// * `encoder` - The command encoder to record the resolve operation
    /// * `query_set` - The query set to resolve
    /// * `query_range` - Range of queries to resolve (start..end)
    /// * `destination` - Buffer to write results to (must have QUERY_RESOLVE usage)
    /// * `destination_offset` - Offset in bytes in the destination buffer
    ///
    /// # Panics
    /// Panics if the destination buffer doesn't have QUERY_RESOLVE usage
    ///
    /// # Examples
    /// ```no_run
    /// use wgpu_playground_core::query_set::{QuerySetOps, QuerySetDescriptor, QueryType};
    /// use wgpu_playground_core::buffer::{BufferDescriptor, BufferUsages};
    /// use std::ops::Range;
    /// # fn example(device: &wgpu::Device) {
    /// let query_set_desc = QuerySetDescriptor::new(
    ///     Some("queries"),
    ///     QueryType::Timestamp,
    ///     4
    /// );
    /// let query_set = query_set_desc.create_query_set(device).unwrap();
    ///
    /// // Buffer to receive query results (8 bytes per timestamp)
    /// let buffer_desc = BufferDescriptor::new(
    ///     Some("query_results"),
    ///     32, // 4 queries * 8 bytes
    ///     BufferUsages::QUERY_RESOLVE | BufferUsages::MAP_READ | BufferUsages::COPY_SRC
    /// );
    /// let buffer = buffer_desc.create_buffer(device).unwrap();
    ///
    /// let mut encoder = device.create_command_encoder(&Default::default());
    /// QuerySetOps::resolve_query_set(&mut encoder, &query_set, 0..4, &buffer, 0);
    /// # }
    /// ```
    pub fn resolve_query_set(
        encoder: &mut wgpu::CommandEncoder,
        query_set: &QuerySet,
        query_range: std::ops::Range<u32>,
        destination: &wgpu::Buffer,
        destination_offset: u64,
    ) {
        encoder.resolve_query_set(query_set, query_range, destination, destination_offset);
    }

    /// Write a timestamp to a query set
    ///
    /// Records a GPU timestamp at the current point in command execution.
    /// Requires the TIMESTAMP_QUERY feature to be enabled on the device.
    ///
    /// # Arguments
    /// * `encoder` - The command encoder to record the timestamp
    /// * `query_set` - The query set to write to (must be Timestamp type)
    /// * `query_index` - Index of the query to write
    ///
    /// # Examples
    /// ```no_run
    /// use wgpu_playground_core::query_set::{QuerySetOps, QuerySetDescriptor, QueryType};
    /// # fn example(device: &wgpu::Device) {
    /// let query_set_desc = QuerySetDescriptor::new(
    ///     Some("timestamps"),
    ///     QueryType::Timestamp,
    ///     2
    /// );
    /// let query_set = query_set_desc.create_query_set(device).unwrap();
    ///
    /// let mut encoder = device.create_command_encoder(&Default::default());
    /// QuerySetOps::write_timestamp(&mut encoder, &query_set, 0);
    /// // ... GPU work ...
    /// QuerySetOps::write_timestamp(&mut encoder, &query_set, 1);
    /// # }
    /// ```
    pub fn write_timestamp(
        encoder: &mut wgpu::CommandEncoder,
        query_set: &QuerySet,
        query_index: u32,
    ) {
        encoder.write_timestamp(query_set, query_index);
    }

    /// Begin an occlusion query in a render pass
    ///
    /// Starts counting samples that pass depth/stencil testing.
    ///
    /// # Arguments
    /// * `render_pass` - The render pass encoder
    /// * `query_set` - The query set to write to (must be Occlusion type)
    /// * `query_index` - Index of the query to write
    ///
    /// # Examples
    /// ```no_run
    /// use wgpu_playground_core::query_set::{QuerySetOps, QuerySetDescriptor, QueryType};
    /// # fn example(device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder) {
    /// let query_set_desc = QuerySetDescriptor::new(
    ///     Some("occlusion"),
    ///     QueryType::Occlusion,
    ///     1
    /// );
    /// let query_set = query_set_desc.create_query_set(device).unwrap();
    ///
    /// let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    ///     label: Some("render_pass"),
    ///     color_attachments: &[],
    ///     depth_stencil_attachment: None,
    ///     timestamp_writes: None,
    ///     occlusion_query_set: Some(&query_set),
    /// });
    ///
    /// QuerySetOps::begin_occlusion_query(&mut render_pass, 0);
    /// // ... draw calls ...
    /// QuerySetOps::end_occlusion_query(&mut render_pass);
    /// # }
    /// ```
    pub fn begin_occlusion_query(render_pass: &mut wgpu::RenderPass<'_>, query_index: u32) {
        render_pass.begin_occlusion_query(query_index);
    }

    /// End an occlusion query in a render pass
    ///
    /// Stops the current occlusion query.
    ///
    /// # Arguments
    /// * `render_pass` - The render pass encoder
    ///
    /// # Examples
    /// ```no_run
    /// use wgpu_playground_core::query_set::QuerySetOps;
    /// # fn example(render_pass: &mut wgpu::RenderPass<'_>) {
    /// QuerySetOps::begin_occlusion_query(render_pass, 0);
    /// // ... draw calls ...
    /// QuerySetOps::end_occlusion_query(render_pass);
    /// # }
    /// ```
    pub fn end_occlusion_query(render_pass: &mut wgpu::RenderPass<'_>) {
        render_pass.end_occlusion_query();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_type_conversion() {
        let occlusion = QueryType::Occlusion;
        assert_eq!(
            QueryType::from_wgpu(occlusion.to_wgpu()),
            QueryType::Occlusion
        );

        let timestamp = QueryType::Timestamp;
        assert_eq!(
            QueryType::from_wgpu(timestamp.to_wgpu()),
            QueryType::Timestamp
        );
    }

    #[test]
    fn test_query_set_descriptor_creation() {
        let descriptor = QuerySetDescriptor::new(Some("test_queries"), QueryType::Timestamp, 4);

        assert_eq!(descriptor.label(), Some("test_queries"));
        assert_eq!(descriptor.query_type(), QueryType::Timestamp);
        assert_eq!(descriptor.count(), 4);
    }

    #[test]
    fn test_query_set_descriptor_validation_success() {
        let descriptor = QuerySetDescriptor::new(Some("valid_queries"), QueryType::Occlusion, 2);

        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn test_query_set_descriptor_validation_zero_count() {
        let descriptor = QuerySetDescriptor::new(None, QueryType::Timestamp, 0);

        let result = descriptor.validate();
        assert!(result.is_err());

        match result {
            Err(QuerySetError::InvalidCount(msg)) => {
                assert!(msg.contains("greater than 0"));
            }
            _ => panic!("Expected InvalidCount error"),
        }
    }

    #[test]
    fn test_query_set_descriptor_default() {
        let descriptor = QuerySetDescriptor::default();
        assert_eq!(descriptor.count(), 2);
        assert_eq!(descriptor.query_type(), QueryType::Timestamp);
    }

    #[test]
    fn test_query_set_error_display() {
        let err = QuerySetError::InvalidConfiguration("test error".to_string());
        assert_eq!(
            err.to_string(),
            "Invalid query set configuration: test error"
        );

        let err = QuerySetError::InvalidCount("count error".to_string());
        assert_eq!(err.to_string(), "Invalid query count: count error");
    }

    #[test]
    fn test_query_types() {
        let occlusion = QueryType::Occlusion;
        let timestamp = QueryType::Timestamp;

        assert_ne!(occlusion, timestamp);
        assert_eq!(occlusion, QueryType::Occlusion);
        assert_eq!(timestamp, QueryType::Timestamp);
    }
}
