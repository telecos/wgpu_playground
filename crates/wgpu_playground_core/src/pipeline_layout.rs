use std::fmt;
use wgpu::{BindGroupLayout, Device, PipelineLayout, ShaderStages};

/// Errors that can occur during pipeline layout operations
#[derive(Debug)]
pub enum PipelineLayoutError {
    /// Invalid layout configuration
    InvalidLayout(String),
    /// Too many bind group layouts
    TooManyBindGroupLayouts(usize),
    /// Invalid push constant range
    InvalidPushConstantRange(String),
}

impl fmt::Display for PipelineLayoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PipelineLayoutError::InvalidLayout(msg) => write!(f, "Invalid layout: {}", msg),
            PipelineLayoutError::TooManyBindGroupLayouts(count) => {
                write!(f, "Too many bind group layouts: {}", count)
            }
            PipelineLayoutError::InvalidPushConstantRange(msg) => {
                write!(f, "Invalid push constant range: {}", msg)
            }
        }
    }
}

impl std::error::Error for PipelineLayoutError {}

/// Describes a push constant range for a pipeline
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PushConstantRange {
    /// Shader stages where this push constant range is visible
    pub stages: ShaderStages,
    /// Start offset of the push constant range in bytes
    pub start: u32,
    /// End offset of the push constant range in bytes
    pub end: u32,
}

impl PushConstantRange {
    /// Create a new push constant range
    ///
    /// # Arguments
    /// * `stages` - Shader stages where this range is visible
    /// * `start` - Start offset in bytes (must be aligned to 4 bytes)
    /// * `end` - End offset in bytes (must be aligned to 4 bytes)
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::pipeline_layout::PushConstantRange;
    /// use wgpu::ShaderStages;
    ///
    /// let range = PushConstantRange::new(
    ///     ShaderStages::VERTEX | ShaderStages::FRAGMENT,
    ///     0,
    ///     64
    /// );
    /// ```
    pub fn new(stages: ShaderStages, start: u32, end: u32) -> Self {
        Self { stages, start, end }
    }

    /// Get the size of the range in bytes
    pub fn size(&self) -> u32 {
        self.end - self.start
    }

    /// Convert to wgpu::PushConstantRange
    pub fn to_wgpu(&self) -> wgpu::PushConstantRange {
        wgpu::PushConstantRange {
            stages: self.stages,
            range: self.start..self.end,
        }
    }

    /// Validate the push constant range
    ///
    /// Checks for:
    /// - Start must be less than end
    /// - Start and end must be aligned to 4 bytes
    /// - Shader stages must not be empty
    ///
    /// # Returns
    /// Ok(()) if valid, Err with PipelineLayoutError if invalid
    pub fn validate(&self) -> Result<(), PipelineLayoutError> {
        if self.start >= self.end {
            return Err(PipelineLayoutError::InvalidPushConstantRange(format!(
                "Push constant range start ({}) must be less than end ({})",
                self.start, self.end
            )));
        }

        if self.start % 4 != 0 {
            return Err(PipelineLayoutError::InvalidPushConstantRange(format!(
                "Push constant range start ({}) must be aligned to 4 bytes",
                self.start
            )));
        }

        if self.end % 4 != 0 {
            return Err(PipelineLayoutError::InvalidPushConstantRange(format!(
                "Push constant range end ({}) must be aligned to 4 bytes",
                self.end
            )));
        }

        if self.stages.is_empty() {
            return Err(PipelineLayoutError::InvalidPushConstantRange(
                "Push constant range must be visible to at least one shader stage".to_string(),
            ));
        }

        Ok(())
    }
}

/// Descriptor for creating a pipeline layout
#[derive(Debug, Clone)]
pub struct PipelineLayoutDescriptor<'a> {
    /// Optional label for debugging
    label: Option<String>,
    /// Bind group layouts that will be used with this pipeline layout
    bind_group_layouts: Vec<&'a BindGroupLayout>,
    /// Push constant ranges (if supported)
    push_constant_ranges: Vec<PushConstantRange>,
}

impl<'a> PipelineLayoutDescriptor<'a> {
    /// Create a new pipeline layout descriptor
    ///
    /// # Arguments
    /// * `label` - Optional label for debugging
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::pipeline_layout::PipelineLayoutDescriptor;
    ///
    /// let descriptor = PipelineLayoutDescriptor::new(Some("my_pipeline_layout"));
    /// ```
    pub fn new(label: Option<&str>) -> Self {
        Self {
            label: label.map(String::from),
            bind_group_layouts: Vec::new(),
            push_constant_ranges: Vec::new(),
        }
    }

    /// Add a bind group layout to the pipeline layout
    ///
    /// # Arguments
    /// * `layout` - The bind group layout to add
    ///
    /// # Returns
    /// Self for method chaining
    ///
    /// # Examples
    /// ```no_run
    /// use wgpu_playground_core::pipeline_layout::PipelineLayoutDescriptor;
    /// # async fn example(device: &wgpu::Device, layout: &wgpu::BindGroupLayout) {
    /// let descriptor = PipelineLayoutDescriptor::new(Some("my_layout"))
    ///     .with_bind_group_layout(layout);
    /// # }
    /// ```
    pub fn with_bind_group_layout(mut self, layout: &'a BindGroupLayout) -> Self {
        self.bind_group_layouts.push(layout);
        self
    }

    /// Add multiple bind group layouts to the pipeline layout
    ///
    /// # Arguments
    /// * `layouts` - A slice of bind group layouts to add
    ///
    /// # Returns
    /// Self for method chaining
    pub fn with_bind_group_layouts(mut self, layouts: &[&'a BindGroupLayout]) -> Self {
        self.bind_group_layouts.extend_from_slice(layouts);
        self
    }

    /// Add a push constant range to the pipeline layout
    ///
    /// # Arguments
    /// * `range` - The push constant range to add
    ///
    /// # Returns
    /// Self for method chaining
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::pipeline_layout::{PipelineLayoutDescriptor, PushConstantRange};
    /// use wgpu::ShaderStages;
    ///
    /// let descriptor = PipelineLayoutDescriptor::new(Some("my_layout"))
    ///     .with_push_constant_range(PushConstantRange::new(
    ///         ShaderStages::VERTEX,
    ///         0,
    ///         64
    ///     ));
    /// ```
    pub fn with_push_constant_range(mut self, range: PushConstantRange) -> Self {
        self.push_constant_ranges.push(range);
        self
    }

    /// Add multiple push constant ranges to the pipeline layout
    ///
    /// # Arguments
    /// * `ranges` - A slice of push constant ranges to add
    ///
    /// # Returns
    /// Self for method chaining
    pub fn with_push_constant_ranges(mut self, ranges: &[PushConstantRange]) -> Self {
        self.push_constant_ranges.extend_from_slice(ranges);
        self
    }

    /// Get the label
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Get the bind group layouts
    pub fn bind_group_layouts(&self) -> &[&'a BindGroupLayout] {
        &self.bind_group_layouts
    }

    /// Get the push constant ranges
    pub fn push_constant_ranges(&self) -> &[PushConstantRange] {
        &self.push_constant_ranges
    }

    /// Validate the pipeline layout descriptor
    ///
    /// Checks for:
    /// - Maximum number of bind group layouts (typically 4)
    /// - Valid push constant ranges
    /// - No overlapping push constant ranges for the same stage
    ///
    /// # Returns
    /// Ok(()) if valid, Err with PipelineLayoutError if invalid
    pub fn validate(&self) -> Result<(), PipelineLayoutError> {
        // WebGPU spec typically allows up to 4 bind group layouts
        const MAX_BIND_GROUP_LAYOUTS: usize = 4;
        if self.bind_group_layouts.len() > MAX_BIND_GROUP_LAYOUTS {
            return Err(PipelineLayoutError::TooManyBindGroupLayouts(
                self.bind_group_layouts.len(),
            ));
        }

        // Validate each push constant range
        for range in &self.push_constant_ranges {
            range.validate()?;
        }

        // Check for overlapping push constant ranges in the same stage
        for i in 0..self.push_constant_ranges.len() {
            for j in (i + 1)..self.push_constant_ranges.len() {
                let range_a = &self.push_constant_ranges[i];
                let range_b = &self.push_constant_ranges[j];

                // Check if ranges share any shader stages
                if range_a.stages.intersects(range_b.stages) {
                    // Check if the ranges overlap
                    let ranges_overlap = range_a.start < range_b.end && range_b.start < range_a.end;
                    if ranges_overlap {
                        return Err(PipelineLayoutError::InvalidPushConstantRange(format!(
                            "Push constant ranges overlap: [{}, {}) and [{}, {})",
                            range_a.start, range_a.end, range_b.start, range_b.end
                        )));
                    }
                }
            }
        }

        Ok(())
    }

    /// Create a wgpu pipeline layout from this descriptor
    ///
    /// This method validates the descriptor and creates the actual pipeline layout.
    ///
    /// # Arguments
    /// * `device` - The wgpu device to create the layout on
    ///
    /// # Returns
    /// A Result containing the PipelineLayout or a PipelineLayoutError
    ///
    /// # Examples
    /// ```no_run
    /// use wgpu_playground_core::pipeline_layout::PipelineLayoutDescriptor;
    /// # async fn example(device: &wgpu::Device, layout: &wgpu::BindGroupLayout) {
    /// let descriptor = PipelineLayoutDescriptor::new(Some("my_pipeline_layout"))
    ///     .with_bind_group_layout(layout);
    ///
    /// let pipeline_layout = descriptor.create_layout(device).unwrap();
    /// # }
    /// ```
    pub fn create_layout(&self, device: &Device) -> Result<PipelineLayout, PipelineLayoutError> {
        self.validate()?;

        let wgpu_push_constant_ranges: Vec<wgpu::PushConstantRange> = self
            .push_constant_ranges
            .iter()
            .map(|r| r.to_wgpu())
            .collect();

        Ok(
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: self.label.as_deref(),
                bind_group_layouts: &self.bind_group_layouts,
                push_constant_ranges: &wgpu_push_constant_ranges,
            }),
        )
    }
}

impl<'a> Default for PipelineLayoutDescriptor<'a> {
    fn default() -> Self {
        Self::new(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_constant_range_creation() {
        let range = PushConstantRange::new(ShaderStages::VERTEX, 0, 64);

        assert_eq!(range.stages, ShaderStages::VERTEX);
        assert_eq!(range.start, 0);
        assert_eq!(range.end, 64);
        assert_eq!(range.size(), 64);
    }

    #[test]
    fn test_push_constant_range_validation_success() {
        let range = PushConstantRange::new(ShaderStages::VERTEX | ShaderStages::FRAGMENT, 0, 128);
        assert!(range.validate().is_ok());
    }

    #[test]
    fn test_push_constant_range_validation_start_greater_than_end() {
        let range = PushConstantRange::new(ShaderStages::VERTEX, 64, 32);
        let result = range.validate();

        assert!(result.is_err());
        match result {
            Err(PipelineLayoutError::InvalidPushConstantRange(msg)) => {
                assert!(msg.contains("start") && msg.contains("less than end"));
            }
            _ => panic!("Expected InvalidPushConstantRange error"),
        }
    }

    #[test]
    fn test_push_constant_range_validation_unaligned_start() {
        let range = PushConstantRange::new(ShaderStages::VERTEX, 1, 64);
        let result = range.validate();

        assert!(result.is_err());
        match result {
            Err(PipelineLayoutError::InvalidPushConstantRange(msg)) => {
                assert!(msg.contains("start") && msg.contains("aligned to 4 bytes"));
            }
            _ => panic!("Expected InvalidPushConstantRange error"),
        }
    }

    #[test]
    fn test_push_constant_range_validation_unaligned_end() {
        let range = PushConstantRange::new(ShaderStages::VERTEX, 0, 63);
        let result = range.validate();

        assert!(result.is_err());
        match result {
            Err(PipelineLayoutError::InvalidPushConstantRange(msg)) => {
                assert!(msg.contains("end") && msg.contains("aligned to 4 bytes"));
            }
            _ => panic!("Expected InvalidPushConstantRange error"),
        }
    }

    #[test]
    fn test_push_constant_range_validation_empty_stages() {
        let range = PushConstantRange::new(ShaderStages::empty(), 0, 64);
        let result = range.validate();

        assert!(result.is_err());
        match result {
            Err(PipelineLayoutError::InvalidPushConstantRange(msg)) => {
                assert!(msg.contains("at least one shader stage"));
            }
            _ => panic!("Expected InvalidPushConstantRange error"),
        }
    }

    #[test]
    fn test_pipeline_layout_descriptor_creation() {
        let descriptor = PipelineLayoutDescriptor::new(Some("test_pipeline_layout"));
        assert_eq!(descriptor.label(), Some("test_pipeline_layout"));
        assert_eq!(descriptor.bind_group_layouts().len(), 0);
        assert_eq!(descriptor.push_constant_ranges().len(), 0);
    }

    #[test]
    fn test_pipeline_layout_descriptor_with_push_constant() {
        let range = PushConstantRange::new(ShaderStages::VERTEX, 0, 64);
        let descriptor =
            PipelineLayoutDescriptor::new(Some("test")).with_push_constant_range(range.clone());

        assert_eq!(descriptor.push_constant_ranges().len(), 1);
        assert_eq!(descriptor.push_constant_ranges()[0], range);
    }

    #[test]
    fn test_pipeline_layout_descriptor_with_multiple_push_constants() {
        let ranges = vec![
            PushConstantRange::new(ShaderStages::VERTEX, 0, 64),
            PushConstantRange::new(ShaderStages::FRAGMENT, 64, 128),
        ];

        let descriptor =
            PipelineLayoutDescriptor::new(Some("multi_range")).with_push_constant_ranges(&ranges);

        assert_eq!(descriptor.push_constant_ranges().len(), 2);
        assert_eq!(descriptor.push_constant_ranges()[0].start, 0);
        assert_eq!(descriptor.push_constant_ranges()[0].end, 64);
        assert_eq!(descriptor.push_constant_ranges()[1].start, 64);
        assert_eq!(descriptor.push_constant_ranges()[1].end, 128);
    }

    #[test]
    fn test_pipeline_layout_validation_success_empty() {
        let descriptor = PipelineLayoutDescriptor::new(None);
        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn test_pipeline_layout_validation_overlapping_push_constants() {
        let ranges = vec![
            PushConstantRange::new(ShaderStages::VERTEX, 0, 64),
            PushConstantRange::new(ShaderStages::VERTEX, 32, 96), // Overlaps with first range
        ];

        let descriptor = PipelineLayoutDescriptor::new(None).with_push_constant_ranges(&ranges);

        let result = descriptor.validate();
        assert!(result.is_err());
        match result {
            Err(PipelineLayoutError::InvalidPushConstantRange(msg)) => {
                assert!(msg.contains("overlap"));
            }
            _ => panic!("Expected InvalidPushConstantRange error for overlapping ranges"),
        }
    }

    #[test]
    fn test_pipeline_layout_validation_non_overlapping_push_constants_same_stage() {
        let ranges = vec![
            PushConstantRange::new(ShaderStages::VERTEX, 0, 64),
            PushConstantRange::new(ShaderStages::VERTEX, 64, 128), // Adjacent, not overlapping
        ];

        let descriptor = PipelineLayoutDescriptor::new(None).with_push_constant_ranges(&ranges);

        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn test_pipeline_layout_validation_overlapping_push_constants_different_stages() {
        let ranges = vec![
            PushConstantRange::new(ShaderStages::VERTEX, 0, 64),
            PushConstantRange::new(ShaderStages::FRAGMENT, 32, 96), // Overlapping range but different stage
        ];

        let descriptor = PipelineLayoutDescriptor::new(None).with_push_constant_ranges(&ranges);

        // This should be OK since ranges are for different stages
        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn test_pipeline_layout_validation_overlapping_push_constants_shared_stage() {
        let ranges = vec![
            PushConstantRange::new(ShaderStages::VERTEX | ShaderStages::FRAGMENT, 0, 64),
            PushConstantRange::new(ShaderStages::FRAGMENT, 32, 96), // Overlaps and shares FRAGMENT stage
        ];

        let descriptor = PipelineLayoutDescriptor::new(None).with_push_constant_ranges(&ranges);

        let result = descriptor.validate();
        assert!(result.is_err());
        match result {
            Err(PipelineLayoutError::InvalidPushConstantRange(msg)) => {
                assert!(msg.contains("overlap"));
            }
            _ => panic!("Expected InvalidPushConstantRange error for overlapping ranges"),
        }
    }

    #[test]
    fn test_error_display() {
        let err = PipelineLayoutError::InvalidLayout("test error".to_string());
        assert_eq!(err.to_string(), "Invalid layout: test error");

        let err = PipelineLayoutError::TooManyBindGroupLayouts(5);
        assert_eq!(err.to_string(), "Too many bind group layouts: 5");

        let err = PipelineLayoutError::InvalidPushConstantRange("test range error".to_string());
        assert_eq!(
            err.to_string(),
            "Invalid push constant range: test range error"
        );
    }

    #[test]
    fn test_push_constant_range_to_wgpu() {
        let range = PushConstantRange::new(ShaderStages::VERTEX | ShaderStages::FRAGMENT, 0, 128);
        let wgpu_range = range.to_wgpu();

        assert_eq!(
            wgpu_range.stages,
            ShaderStages::VERTEX | ShaderStages::FRAGMENT
        );
        assert_eq!(wgpu_range.range.start, 0);
        assert_eq!(wgpu_range.range.end, 128);
    }

    #[test]
    fn test_default_pipeline_layout_descriptor() {
        let descriptor = PipelineLayoutDescriptor::default();
        assert_eq!(descriptor.label(), None);
        assert_eq!(descriptor.bind_group_layouts().len(), 0);
        assert_eq!(descriptor.push_constant_ranges().len(), 0);
    }

    #[test]
    fn test_push_constant_range_size() {
        let range = PushConstantRange::new(ShaderStages::VERTEX, 0, 64);
        assert_eq!(range.size(), 64);

        let range2 = PushConstantRange::new(ShaderStages::FRAGMENT, 32, 96);
        assert_eq!(range2.size(), 64);
    }

    #[test]
    fn test_multiple_shader_stages() {
        let range = PushConstantRange::new(
            ShaderStages::VERTEX | ShaderStages::FRAGMENT | ShaderStages::COMPUTE,
            0,
            128,
        );

        assert!(range.stages.contains(ShaderStages::VERTEX));
        assert!(range.stages.contains(ShaderStages::FRAGMENT));
        assert!(range.stages.contains(ShaderStages::COMPUTE));
        assert!(range.validate().is_ok());
    }
}
