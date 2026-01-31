use std::fmt;
use wgpu::{Buffer, Device, RenderBundle, RenderBundleEncoder, RenderPipeline};

use crate::render_pass_encoder::IndexFormat;

/// Errors that can occur during render bundle operations
#[derive(Debug)]
pub enum RenderBundleError {
    /// Invalid render bundle configuration
    InvalidConfiguration(String),
}

impl fmt::Display for RenderBundleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderBundleError::InvalidConfiguration(msg) => {
                write!(f, "Invalid render bundle configuration: {}", msg)
            }
        }
    }
}

impl std::error::Error for RenderBundleError {}

/// Descriptor for creating a render bundle encoder
#[derive(Debug, Clone)]
pub struct RenderBundleDescriptor<'a> {
    /// Label for debugging
    pub label: Option<&'a str>,
    /// Color attachment formats
    pub color_formats: Vec<Option<wgpu::TextureFormat>>,
    /// Depth-stencil attachment format (optional)
    pub depth_stencil_format: Option<wgpu::TextureFormat>,
    /// Sample count for multisampling
    pub sample_count: u32,
}

impl<'a> RenderBundleDescriptor<'a> {
    /// Create a new render bundle descriptor
    ///
    /// # Examples
    /// ```no_run
    /// # use wgpu_playground_core::render_bundle_encoder::RenderBundleDescriptor;
    /// let descriptor = RenderBundleDescriptor::new()
    ///     .with_label("My Render Bundle")
    ///     .with_color_format(wgpu::TextureFormat::Bgra8UnormSrgb);
    /// ```
    pub fn new() -> Self {
        Self {
            label: None,
            color_formats: Vec::new(),
            depth_stencil_format: None,
            sample_count: 1,
        }
    }

    /// Set the label for debugging
    pub fn with_label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    /// Add a color attachment format
    pub fn with_color_format(mut self, format: wgpu::TextureFormat) -> Self {
        self.color_formats.push(Some(format));
        self
    }

    /// Set the depth-stencil attachment format
    pub fn with_depth_stencil_format(mut self, format: wgpu::TextureFormat) -> Self {
        self.depth_stencil_format = Some(format);
        self
    }

    /// Set the sample count for multisampling
    pub fn with_sample_count(mut self, sample_count: u32) -> Self {
        self.sample_count = sample_count;
        self
    }

    /// Validate the render bundle descriptor
    pub fn validate(&self) -> Result<(), RenderBundleError> {
        if self.color_formats.is_empty() && self.depth_stencil_format.is_none() {
            return Err(RenderBundleError::InvalidConfiguration(
                "Render bundle must have at least one color format or a depth-stencil format"
                    .to_string(),
            ));
        }
        Ok(())
    }

    /// Convert to wgpu::RenderBundleEncoderDescriptor
    fn to_wgpu(&self) -> wgpu::RenderBundleEncoderDescriptor<'_> {
        wgpu::RenderBundleEncoderDescriptor {
            label: self.label,
            color_formats: &self.color_formats,
            depth_stencil: self
                .depth_stencil_format
                .map(|format| wgpu::RenderBundleDepthStencil {
                    format,
                    depth_read_only: false,
                    stencil_read_only: false,
                }),
            sample_count: self.sample_count,
            multiview: None,
        }
    }
}

impl<'a> Default for RenderBundleDescriptor<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// Wrapper for wgpu::RenderBundleEncoder with convenient methods
///
/// A render bundle encoder records a set of draw commands that can be
/// replayed multiple times without re-recording them. This is useful for
/// optimizing scenes with repeated geometry.
///
/// # Examples
/// ```no_run
/// # use wgpu_playground_core::render_bundle_encoder::{RenderBundleEncoderOps, RenderBundleDescriptor};
/// # let device: &wgpu::Device = todo!();
/// # let pipeline: &wgpu::RenderPipeline = todo!();
/// # let vertex_buffer: &wgpu::Buffer = todo!();
/// let descriptor = RenderBundleDescriptor::new()
///     .with_label("My Render Bundle")
///     .with_color_format(wgpu::TextureFormat::Bgra8UnormSrgb);
///
/// let mut encoder = RenderBundleEncoderOps::new(device, &descriptor).unwrap();
/// encoder.set_pipeline(pipeline);
/// encoder.set_vertex_buffer(0, vertex_buffer, 0, None);
/// encoder.draw(0..3, 0..1);
/// let bundle = encoder.finish();
/// ```
pub struct RenderBundleEncoderOps<'a> {
    encoder: RenderBundleEncoder<'a>,
}

impl<'a> RenderBundleEncoderOps<'a> {
    /// Create a new render bundle encoder
    ///
    /// # Arguments
    /// * `device` - The GPU device
    /// * `descriptor` - The render bundle descriptor
    ///
    /// # Returns
    /// Returns a Result containing the render bundle encoder or an error
    ///
    /// # Examples
    /// ```no_run
    /// # use wgpu_playground_core::render_bundle_encoder::{RenderBundleEncoderOps, RenderBundleDescriptor};
    /// # let device: &wgpu::Device = todo!();
    /// let descriptor = RenderBundleDescriptor::new()
    ///     .with_color_format(wgpu::TextureFormat::Bgra8UnormSrgb);
    ///
    /// let encoder = RenderBundleEncoderOps::new(device, &descriptor).unwrap();
    /// ```
    pub fn new(
        device: &'a Device,
        descriptor: &RenderBundleDescriptor<'_>,
    ) -> Result<Self, RenderBundleError> {
        descriptor.validate()?;

        let encoder = device.create_render_bundle_encoder(&descriptor.to_wgpu());

        Ok(Self { encoder })
    }

    /// Set the current render pipeline
    ///
    /// # Arguments
    /// * `pipeline` - The render pipeline to use
    pub fn set_pipeline(&mut self, pipeline: &'a RenderPipeline) {
        self.encoder.set_pipeline(pipeline);
    }

    /// Set the vertex buffer for the given slot
    ///
    /// # Arguments
    /// * `slot` - The vertex buffer slot
    /// * `buffer` - The buffer to bind
    /// * `offset` - Offset in bytes into the buffer
    /// * `size` - Size of the buffer region to bind (None means to the end)
    pub fn set_vertex_buffer(
        &mut self,
        slot: u32,
        buffer: &'a Buffer,
        offset: u64,
        size: Option<u64>,
    ) {
        let buffer_slice = if let Some(s) = size {
            buffer.slice(offset..offset + s)
        } else {
            buffer.slice(offset..)
        };
        self.encoder.set_vertex_buffer(slot, buffer_slice);
    }

    /// Set the index buffer
    ///
    /// # Arguments
    /// * `buffer` - The buffer to use as index buffer
    /// * `format` - The format of indices (Uint16 or Uint32)
    /// * `offset` - Offset in bytes into the buffer
    /// * `size` - Size of the buffer region to bind (None means to the end)
    pub fn set_index_buffer(
        &mut self,
        buffer: &'a Buffer,
        format: IndexFormat,
        offset: u64,
        size: Option<u64>,
    ) {
        let buffer_slice = if let Some(s) = size {
            buffer.slice(offset..offset + s)
        } else {
            buffer.slice(offset..)
        };
        self.encoder
            .set_index_buffer(buffer_slice, format.to_wgpu());
    }

    /// Set the bind group for the given index
    ///
    /// # Arguments
    /// * `index` - The bind group index
    /// * `bind_group` - The bind group to bind
    /// * `offsets` - Dynamic offsets for dynamic uniform/storage buffers
    pub fn set_bind_group(&mut self, index: u32, bind_group: &'a wgpu::BindGroup, offsets: &[u32]) {
        self.encoder.set_bind_group(index, bind_group, offsets);
    }

    /// Draw primitives
    ///
    /// # Arguments
    /// * `vertices` - Range of vertices to draw
    /// * `instances` - Range of instances to draw
    ///
    /// # Examples
    /// ```no_run
    /// # use wgpu_playground_core::render_bundle_encoder::RenderBundleEncoderOps;
    /// # let mut encoder: RenderBundleEncoderOps = todo!();
    /// // Draw 3 vertices (e.g., a triangle) with 1 instance
    /// encoder.draw(0..3, 0..1);
    /// ```
    pub fn draw(&mut self, vertices: std::ops::Range<u32>, instances: std::ops::Range<u32>) {
        self.encoder.draw(vertices, instances);
    }

    /// Draw indexed primitives
    ///
    /// # Arguments
    /// * `indices` - Range of indices to draw
    /// * `base_vertex` - Offset added to each index before indexing into the vertex buffer
    /// * `instances` - Range of instances to draw
    ///
    /// # Examples
    /// ```no_run
    /// # use wgpu_playground_core::render_bundle_encoder::RenderBundleEncoderOps;
    /// # let mut encoder: RenderBundleEncoderOps = todo!();
    /// // Draw 6 indices with 1 instance
    /// encoder.draw_indexed(0..6, 0, 0..1);
    /// ```
    pub fn draw_indexed(
        &mut self,
        indices: std::ops::Range<u32>,
        base_vertex: i32,
        instances: std::ops::Range<u32>,
    ) {
        self.encoder.draw_indexed(indices, base_vertex, instances);
    }

    /// Draw primitives using indirect buffer
    ///
    /// # Arguments
    /// * `indirect_buffer` - Buffer containing draw parameters
    /// * `indirect_offset` - Offset in bytes into the indirect buffer
    ///
    /// # Examples
    /// ```no_run
    /// # use wgpu_playground_core::render_bundle_encoder::RenderBundleEncoderOps;
    /// # let mut encoder: RenderBundleEncoderOps = todo!();
    /// # let indirect_buffer: &wgpu::Buffer = todo!();
    /// encoder.draw_indirect(indirect_buffer, 0);
    /// ```
    pub fn draw_indirect(&mut self, indirect_buffer: &'a Buffer, indirect_offset: u64) {
        self.encoder.draw_indirect(indirect_buffer, indirect_offset);
    }

    /// Draw indexed primitives using indirect buffer
    ///
    /// # Arguments
    /// * `indirect_buffer` - Buffer containing draw parameters
    /// * `indirect_offset` - Offset in bytes into the indirect buffer
    ///
    /// # Examples
    /// ```no_run
    /// # use wgpu_playground_core::render_bundle_encoder::RenderBundleEncoderOps;
    /// # let mut encoder: RenderBundleEncoderOps = todo!();
    /// # let indirect_buffer: &wgpu::Buffer = todo!();
    /// encoder.draw_indexed_indirect(indirect_buffer, 0);
    /// ```
    pub fn draw_indexed_indirect(&mut self, indirect_buffer: &'a Buffer, indirect_offset: u64) {
        self.encoder
            .draw_indexed_indirect(indirect_buffer, indirect_offset);
    }

    /// Finish encoding and return the render bundle
    ///
    /// This consumes the encoder and produces a render bundle that can
    /// be executed in render passes.
    ///
    /// # Returns
    /// A render bundle containing all recorded commands
    ///
    /// # Examples
    /// ```no_run
    /// # use wgpu_playground_core::render_bundle_encoder::RenderBundleEncoderOps;
    /// # let mut encoder: RenderBundleEncoderOps = todo!();
    /// let bundle = encoder.finish();
    /// ```
    pub fn finish(self) -> RenderBundle {
        self.encoder
            .finish(&wgpu::RenderBundleDescriptor { label: None })
    }

    /// Finish encoding with a label and return the render bundle
    ///
    /// # Arguments
    /// * `label` - Optional label for debugging
    ///
    /// # Returns
    /// A render bundle containing all recorded commands
    pub fn finish_with_label(self, label: Option<&str>) -> RenderBundle {
        self.encoder.finish(&wgpu::RenderBundleDescriptor { label })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_bundle_error_display() {
        let err = RenderBundleError::InvalidConfiguration("test".to_string());
        assert!(format!("{}", err).contains("Invalid render bundle configuration"));
    }

    #[test]
    fn test_render_bundle_descriptor_validation() {
        let descriptor = RenderBundleDescriptor::new();
        assert!(descriptor.validate().is_err());

        let descriptor =
            RenderBundleDescriptor::new().with_color_format(wgpu::TextureFormat::Bgra8UnormSrgb);
        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn test_render_bundle_descriptor_builder() {
        let descriptor = RenderBundleDescriptor::new()
            .with_label("test")
            .with_color_format(wgpu::TextureFormat::Bgra8UnormSrgb)
            .with_sample_count(4);

        assert_eq!(descriptor.label, Some("test"));
        assert_eq!(descriptor.color_formats.len(), 1);
        assert_eq!(descriptor.sample_count, 4);
    }
}
