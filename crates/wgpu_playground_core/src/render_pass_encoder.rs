use std::fmt;
use wgpu::{Buffer, CommandEncoder, RenderBundle, RenderPass, RenderPipeline, TextureView};

/// Errors that can occur during render pass operations
#[derive(Debug)]
pub enum RenderPassError {
    /// Invalid render pass configuration
    InvalidConfiguration(String),
    /// Invalid attachment configuration
    InvalidAttachment(String),
    /// Missing required attachment
    MissingAttachment(String),
}

impl fmt::Display for RenderPassError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderPassError::InvalidConfiguration(msg) => {
                write!(f, "Invalid render pass configuration: {}", msg)
            }
            RenderPassError::InvalidAttachment(msg) => {
                write!(f, "Invalid attachment configuration: {}", msg)
            }
            RenderPassError::MissingAttachment(msg) => {
                write!(f, "Missing required attachment: {}", msg)
            }
        }
    }
}

impl std::error::Error for RenderPassError {}

/// Load operation for an attachment
///
/// Determines what to do with the attachment at the start of the render pass
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoadOp<T> {
    /// Clear the attachment with the specified value
    Clear(T),
    /// Load the existing contents of the attachment
    Load,
}

impl LoadOp<Color> {
    /// Convert to wgpu::LoadOp for Color
    pub fn to_wgpu(&self) -> wgpu::LoadOp<wgpu::Color> {
        match self {
            LoadOp::Clear(color) => wgpu::LoadOp::Clear(color.to_wgpu()),
            LoadOp::Load => wgpu::LoadOp::Load,
        }
    }
}

impl LoadOp<f32> {
    /// Convert to wgpu::LoadOp for f32
    pub fn to_wgpu(&self) -> wgpu::LoadOp<f32> {
        match self {
            LoadOp::Clear(value) => wgpu::LoadOp::Clear(*value),
            LoadOp::Load => wgpu::LoadOp::Load,
        }
    }
}

impl LoadOp<u32> {
    /// Convert to wgpu::LoadOp for u32
    pub fn to_wgpu(&self) -> wgpu::LoadOp<u32> {
        match self {
            LoadOp::Clear(value) => wgpu::LoadOp::Clear(*value),
            LoadOp::Load => wgpu::LoadOp::Load,
        }
    }
}

/// Store operation for an attachment
///
/// Determines what to do with the attachment at the end of the render pass
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreOp {
    /// Store the contents of the attachment
    Store,
    /// Discard the contents of the attachment
    Discard,
}

impl StoreOp {
    /// Convert to wgpu::StoreOp
    pub fn to_wgpu(&self) -> wgpu::StoreOp {
        match self {
            StoreOp::Store => wgpu::StoreOp::Store,
            StoreOp::Discard => wgpu::StoreOp::Discard,
        }
    }
}

/// RGBA color value
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    /// Red component (0.0 to 1.0)
    pub r: f64,
    /// Green component (0.0 to 1.0)
    pub g: f64,
    /// Blue component (0.0 to 1.0)
    pub b: f64,
    /// Alpha component (0.0 to 1.0)
    pub a: f64,
}

impl Color {
    /// Create a new color
    pub const fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }

    /// Black color (0, 0, 0, 1)
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0, 1.0);

    /// White color (1, 1, 1, 1)
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0, 1.0);

    /// Red color (1, 0, 0, 1)
    pub const RED: Self = Self::new(1.0, 0.0, 0.0, 1.0);

    /// Green color (0, 1, 0, 1)
    pub const GREEN: Self = Self::new(0.0, 1.0, 0.0, 1.0);

    /// Blue color (0, 0, 1, 1)
    pub const BLUE: Self = Self::new(0.0, 0.0, 1.0, 1.0);

    /// Transparent color (0, 0, 0, 0)
    pub const TRANSPARENT: Self = Self::new(0.0, 0.0, 0.0, 0.0);

    /// Convert to wgpu::Color
    pub fn to_wgpu(&self) -> wgpu::Color {
        wgpu::Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}

/// Render pass color attachment
#[derive(Debug)]
pub struct RenderPassColorAttachment<'a> {
    /// The view to render to
    pub view: &'a TextureView,
    /// The view to resolve multisampled data to (optional)
    pub resolve_target: Option<&'a TextureView>,
    /// Load operation for this attachment
    pub load_op: LoadOp<Color>,
    /// Store operation for this attachment
    pub store_op: StoreOp,
}

impl<'a> RenderPassColorAttachment<'a> {
    /// Create a new color attachment with clear operation
    ///
    /// # Arguments
    /// * `view` - The texture view to render to
    /// * `clear_color` - The color to clear the attachment to
    ///
    /// # Examples
    /// ```no_run
    /// # use wgpu_playground_core::render_pass_encoder::{RenderPassColorAttachment, Color};
    /// # let view: &wgpu::TextureView = todo!();
    /// let attachment = RenderPassColorAttachment::clear(view, Color::BLACK);
    /// ```
    pub fn clear(view: &'a TextureView, clear_color: Color) -> Self {
        Self {
            view,
            resolve_target: None,
            load_op: LoadOp::Clear(clear_color),
            store_op: StoreOp::Store,
        }
    }

    /// Create a new color attachment with load operation
    ///
    /// # Arguments
    /// * `view` - The texture view to render to
    ///
    /// # Examples
    /// ```no_run
    /// # use wgpu_playground_core::render_pass_encoder::RenderPassColorAttachment;
    /// # let view: &wgpu::TextureView = todo!();
    /// let attachment = RenderPassColorAttachment::load(view);
    /// ```
    pub fn load(view: &'a TextureView) -> Self {
        Self {
            view,
            resolve_target: None,
            load_op: LoadOp::Load,
            store_op: StoreOp::Store,
        }
    }

    /// Set the resolve target for multisampling
    pub fn with_resolve_target(mut self, resolve_target: &'a TextureView) -> Self {
        self.resolve_target = Some(resolve_target);
        self
    }

    /// Set the store operation
    pub fn with_store_op(mut self, store_op: StoreOp) -> Self {
        self.store_op = store_op;
        self
    }

    /// Convert to wgpu::RenderPassColorAttachment
    pub fn to_wgpu(&self) -> wgpu::RenderPassColorAttachment<'_> {
        wgpu::RenderPassColorAttachment {
            view: self.view,
            resolve_target: self.resolve_target,
            ops: wgpu::Operations {
                load: self.load_op.to_wgpu(),
                store: self.store_op.to_wgpu(),
            },
            depth_slice: None,
        }
    }
}

/// Depth-stencil operations
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DepthStencilOps {
    /// Load operation for depth
    pub depth_load_op: LoadOp<f32>,
    /// Store operation for depth
    pub depth_store_op: StoreOp,
    /// Load operation for stencil
    pub stencil_load_op: LoadOp<u32>,
    /// Store operation for stencil
    pub stencil_store_op: StoreOp,
}

impl DepthStencilOps {
    /// Create depth-stencil ops that clears depth to the given value
    pub fn clear_depth(depth: f32) -> Self {
        Self {
            depth_load_op: LoadOp::Clear(depth),
            depth_store_op: StoreOp::Store,
            stencil_load_op: LoadOp::Load,
            stencil_store_op: StoreOp::Store,
        }
    }

    /// Create depth-stencil ops that loads existing depth
    pub fn load_depth() -> Self {
        Self {
            depth_load_op: LoadOp::Load,
            depth_store_op: StoreOp::Store,
            stencil_load_op: LoadOp::Load,
            stencil_store_op: StoreOp::Store,
        }
    }

    /// Create depth-stencil ops that clears both depth and stencil
    pub fn clear_depth_stencil(depth: f32, stencil: u32) -> Self {
        Self {
            depth_load_op: LoadOp::Clear(depth),
            depth_store_op: StoreOp::Store,
            stencil_load_op: LoadOp::Clear(stencil),
            stencil_store_op: StoreOp::Store,
        }
    }
}

/// Render pass depth-stencil attachment
#[derive(Debug)]
pub struct RenderPassDepthStencilAttachment<'a> {
    /// The view to use as depth-stencil attachment
    pub view: &'a TextureView,
    /// Depth-stencil operations
    pub ops: DepthStencilOps,
}

impl<'a> RenderPassDepthStencilAttachment<'a> {
    /// Create a new depth-stencil attachment
    ///
    /// # Arguments
    /// * `view` - The texture view to use as depth-stencil attachment
    /// * `ops` - The depth-stencil operations
    ///
    /// # Examples
    /// ```no_run
    /// # use wgpu_playground_core::render_pass_encoder::{RenderPassDepthStencilAttachment, DepthStencilOps};
    /// # let view: &wgpu::TextureView = todo!();
    /// let attachment = RenderPassDepthStencilAttachment::new(
    ///     view,
    ///     DepthStencilOps::clear_depth(1.0)
    /// );
    /// ```
    pub fn new(view: &'a TextureView, ops: DepthStencilOps) -> Self {
        Self { view, ops }
    }

    /// Convert to wgpu::RenderPassDepthStencilAttachment
    pub fn to_wgpu(&self) -> wgpu::RenderPassDepthStencilAttachment<'_> {
        wgpu::RenderPassDepthStencilAttachment {
            view: self.view,
            depth_ops: Some(wgpu::Operations {
                load: self.ops.depth_load_op.to_wgpu(),
                store: self.ops.depth_store_op.to_wgpu(),
            }),
            stencil_ops: Some(wgpu::Operations {
                load: self.ops.stencil_load_op.to_wgpu(),
                store: self.ops.stencil_store_op.to_wgpu(),
            }),
        }
    }
}

/// Render pass descriptor
#[derive(Debug)]
pub struct RenderPassDescriptor<'a> {
    /// Label for debugging
    pub label: Option<&'a str>,
    /// Color attachments
    pub color_attachments: Vec<Option<RenderPassColorAttachment<'a>>>,
    /// Depth-stencil attachment (optional)
    pub depth_stencil_attachment: Option<RenderPassDepthStencilAttachment<'a>>,
}

impl<'a> RenderPassDescriptor<'a> {
    /// Create a new render pass descriptor
    pub fn new() -> Self {
        Self {
            label: None,
            color_attachments: Vec::new(),
            depth_stencil_attachment: None,
        }
    }

    /// Set the label for debugging
    pub fn with_label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    /// Add a color attachment
    pub fn with_color_attachment(mut self, attachment: RenderPassColorAttachment<'a>) -> Self {
        self.color_attachments.push(Some(attachment));
        self
    }

    /// Set the depth-stencil attachment
    pub fn with_depth_stencil_attachment(
        mut self,
        attachment: RenderPassDepthStencilAttachment<'a>,
    ) -> Self {
        self.depth_stencil_attachment = Some(attachment);
        self
    }

    /// Validate the render pass descriptor
    pub fn validate(&self) -> Result<(), RenderPassError> {
        if self.color_attachments.is_empty() && self.depth_stencil_attachment.is_none() {
            return Err(RenderPassError::InvalidConfiguration(
                "Render pass must have at least one color attachment or a depth-stencil attachment"
                    .to_string(),
            ));
        }
        Ok(())
    }
}

impl<'a> Default for RenderPassDescriptor<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// Wrapper for wgpu::RenderPass with convenient methods
pub struct RenderPassEncoder<'a> {
    pass: RenderPass<'a>,
}

impl<'a> RenderPassEncoder<'a> {
    /// Create a new render pass encoder from a command encoder
    ///
    /// # Arguments
    /// * `encoder` - The command encoder
    /// * `descriptor` - The render pass descriptor
    ///
    /// # Returns
    /// Returns a Result containing the render pass encoder or an error
    ///
    /// # Examples
    /// ```no_run
    /// # use wgpu_playground_core::render_pass_encoder::{RenderPassEncoder, RenderPassDescriptor, RenderPassColorAttachment, Color};
    /// # let mut encoder: wgpu::CommandEncoder = todo!();
    /// # let view: &wgpu::TextureView = todo!();
    /// let descriptor = RenderPassDescriptor::new()
    ///     .with_label("My Render Pass")
    ///     .with_color_attachment(RenderPassColorAttachment::clear(view, Color::BLACK));
    ///
    /// let render_pass = RenderPassEncoder::begin(&mut encoder, &descriptor).unwrap();
    /// ```
    pub fn begin(
        encoder: &'a mut CommandEncoder,
        descriptor: &RenderPassDescriptor<'a>,
    ) -> Result<Self, RenderPassError> {
        descriptor.validate()?;

        // Track API usage
        #[cfg(not(test))]
        {
            use crate::api_coverage::{ApiCategory, ApiCoverageTracker};
            ApiCoverageTracker::global().record(ApiCategory::RenderPass, "begin_render_pass");
        }

        let color_attachments: Vec<_> = descriptor
            .color_attachments
            .iter()
            .map(|a| a.as_ref().map(|att| att.to_wgpu()))
            .collect();

        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: descriptor.label,
            color_attachments: &color_attachments,
            depth_stencil_attachment: descriptor
                .depth_stencil_attachment
                .as_ref()
                .map(|a| a.to_wgpu()),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        Ok(Self { pass })
    }

    /// Set the current render pipeline
    ///
    /// # Arguments
    /// * `pipeline` - The render pipeline to use
    pub fn set_pipeline(&mut self, pipeline: &'a RenderPipeline) {
        // Track API usage
        #[cfg(not(test))]
        {
            use crate::api_coverage::{ApiCategory, ApiCoverageTracker};
            ApiCoverageTracker::global().record(ApiCategory::RenderPass, "set_pipeline");
        }

        self.pass.set_pipeline(pipeline);
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
        self.pass.set_vertex_buffer(slot, buffer_slice);
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
        self.pass.set_index_buffer(buffer_slice, format.to_wgpu());
    }

    /// Set the bind group for the given index
    ///
    /// # Arguments
    /// * `index` - The bind group index
    /// * `bind_group` - The bind group to bind
    /// * `offsets` - Dynamic offsets for dynamic uniform/storage buffers
    pub fn set_bind_group(&mut self, index: u32, bind_group: &'a wgpu::BindGroup, offsets: &[u32]) {
        self.pass.set_bind_group(index, bind_group, offsets);
    }

    /// Draw primitives
    ///
    /// # Arguments
    /// * `vertices` - Range of vertices to draw
    /// * `instances` - Range of instances to draw
    ///
    /// # Examples
    /// ```no_run
    /// # use wgpu_playground_core::render_pass_encoder::RenderPassEncoder;
    /// # let mut render_pass: RenderPassEncoder = todo!();
    /// // Draw 3 vertices (e.g., a triangle) with 1 instance
    /// render_pass.draw(0..3, 0..1);
    /// ```
    pub fn draw(&mut self, vertices: std::ops::Range<u32>, instances: std::ops::Range<u32>) {
        // Track API usage
        #[cfg(not(test))]
        {
            use crate::api_coverage::{ApiCategory, ApiCoverageTracker};
            ApiCoverageTracker::global().record(ApiCategory::RenderPass, "draw");
        }

        self.pass.draw(vertices, instances);
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
    /// # use wgpu_playground_core::render_pass_encoder::RenderPassEncoder;
    /// # let mut render_pass: RenderPassEncoder = todo!();
    /// // Draw 6 indices with 1 instance
    /// render_pass.draw_indexed(0..6, 0, 0..1);
    /// ```
    pub fn draw_indexed(
        &mut self,
        indices: std::ops::Range<u32>,
        base_vertex: i32,
        instances: std::ops::Range<u32>,
    ) {
        self.pass.draw_indexed(indices, base_vertex, instances);
    }

    /// Draw primitives using indirect buffer
    ///
    /// # Arguments
    /// * `indirect_buffer` - Buffer containing draw parameters
    /// * `indirect_offset` - Offset in bytes into the indirect buffer
    ///
    /// # Examples
    /// ```no_run
    /// # use wgpu_playground_core::render_pass_encoder::RenderPassEncoder;
    /// # let mut render_pass: RenderPassEncoder = todo!();
    /// # let indirect_buffer: &wgpu::Buffer = todo!();
    /// render_pass.draw_indirect(indirect_buffer, 0);
    /// ```
    pub fn draw_indirect(&mut self, indirect_buffer: &'a Buffer, indirect_offset: u64) {
        self.pass.draw_indirect(indirect_buffer, indirect_offset);
    }

    /// Draw indexed primitives using indirect buffer
    ///
    /// # Arguments
    /// * `indirect_buffer` - Buffer containing draw parameters
    /// * `indirect_offset` - Offset in bytes into the indirect buffer
    ///
    /// # Examples
    /// ```no_run
    /// # use wgpu_playground_core::render_pass_encoder::RenderPassEncoder;
    /// # let mut render_pass: RenderPassEncoder = todo!();
    /// # let indirect_buffer: &wgpu::Buffer = todo!();
    /// render_pass.draw_indexed_indirect(indirect_buffer, 0);
    /// ```
    pub fn draw_indexed_indirect(&mut self, indirect_buffer: &'a Buffer, indirect_offset: u64) {
        self.pass
            .draw_indexed_indirect(indirect_buffer, indirect_offset);
    }

    /// Set the scissor rectangle
    ///
    /// # Arguments
    /// * `x` - X coordinate of the scissor rectangle
    /// * `y` - Y coordinate of the scissor rectangle
    /// * `width` - Width of the scissor rectangle
    /// * `height` - Height of the scissor rectangle
    pub fn set_scissor_rect(&mut self, x: u32, y: u32, width: u32, height: u32) {
        self.pass.set_scissor_rect(x, y, width, height);
    }

    /// Set the viewport
    ///
    /// # Arguments
    /// * `x` - X coordinate of the viewport
    /// * `y` - Y coordinate of the viewport
    /// * `width` - Width of the viewport
    /// * `height` - Height of the viewport
    /// * `min_depth` - Minimum depth value (0.0 to 1.0)
    /// * `max_depth` - Maximum depth value (0.0 to 1.0)
    pub fn set_viewport(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        min_depth: f32,
        max_depth: f32,
    ) {
        self.pass
            .set_viewport(x, y, width, height, min_depth, max_depth);
    }

    /// Set the stencil reference value
    ///
    /// # Arguments
    /// * `reference` - The stencil reference value
    pub fn set_stencil_reference(&mut self, reference: u32) {
        self.pass.set_stencil_reference(reference);
    }

    /// Set the blend constant color
    ///
    /// # Arguments
    /// * `color` - The blend constant color
    pub fn set_blend_constant(&mut self, color: Color) {
        self.pass.set_blend_constant(color.to_wgpu());
    }

    /// Execute a pre-recorded render bundle
    ///
    /// Executes all the commands in the given render bundle. This is more
    /// efficient than re-recording the same commands each frame.
    ///
    /// # Arguments
    /// * `bundle` - The render bundle to execute
    ///
    /// # Examples
    /// ```no_run
    /// # use wgpu_playground_core::render_pass_encoder::RenderPassEncoder;
    /// # let mut render_pass: RenderPassEncoder = todo!();
    /// # let bundle: &wgpu::RenderBundle = todo!();
    /// render_pass.execute_bundle(bundle);
    /// ```
    pub fn execute_bundle(&mut self, bundle: &'a RenderBundle) {
        self.pass.execute_bundles(std::iter::once(bundle));
    }

    /// Execute multiple pre-recorded render bundles
    ///
    /// Executes all the commands in the given render bundles in order.
    /// This is more efficient than re-recording the same commands each frame.
    ///
    /// # Arguments
    /// * `bundles` - The render bundles to execute
    ///
    /// # Examples
    /// ```no_run
    /// # use wgpu_playground_core::render_pass_encoder::RenderPassEncoder;
    /// # let mut render_pass: RenderPassEncoder = todo!();
    /// # let bundles: &[&wgpu::RenderBundle] = todo!();
    /// render_pass.execute_bundles(bundles);
    /// ```
    pub fn execute_bundles(&mut self, bundles: &[&'a RenderBundle]) {
        self.pass.execute_bundles(bundles.iter().copied());
    }
}

/// Index format for index buffers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexFormat {
    /// 16-bit unsigned integer indices
    Uint16,
    /// 32-bit unsigned integer indices
    Uint32,
}

impl IndexFormat {
    /// Convert to wgpu::IndexFormat
    pub fn to_wgpu(&self) -> wgpu::IndexFormat {
        match self {
            IndexFormat::Uint16 => wgpu::IndexFormat::Uint16,
            IndexFormat::Uint32 => wgpu::IndexFormat::Uint32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_pass_error_display() {
        let err = RenderPassError::InvalidConfiguration("test".to_string());
        assert!(format!("{}", err).contains("Invalid render pass configuration"));

        let err = RenderPassError::InvalidAttachment("test".to_string());
        assert!(format!("{}", err).contains("Invalid attachment configuration"));

        let err = RenderPassError::MissingAttachment("test".to_string());
        assert!(format!("{}", err).contains("Missing required attachment"));
    }

    #[test]
    fn test_load_op() {
        let clear = LoadOp::Clear(Color::BLACK);
        let load = LoadOp::<Color>::Load;

        assert_eq!(clear, LoadOp::Clear(Color::BLACK));
        assert_eq!(load, LoadOp::Load);
    }

    #[test]
    fn test_store_op() {
        let store = StoreOp::Store;
        let discard = StoreOp::Discard;

        assert_eq!(store, StoreOp::Store);
        assert_eq!(discard, StoreOp::Discard);
    }

    #[test]
    fn test_color_constants() {
        assert_eq!(Color::BLACK, Color::new(0.0, 0.0, 0.0, 1.0));
        assert_eq!(Color::WHITE, Color::new(1.0, 1.0, 1.0, 1.0));
        assert_eq!(Color::RED, Color::new(1.0, 0.0, 0.0, 1.0));
        assert_eq!(Color::GREEN, Color::new(0.0, 1.0, 0.0, 1.0));
        assert_eq!(Color::BLUE, Color::new(0.0, 0.0, 1.0, 1.0));
        assert_eq!(Color::TRANSPARENT, Color::new(0.0, 0.0, 0.0, 0.0));
    }

    #[test]
    fn test_depth_stencil_ops() {
        let ops = DepthStencilOps::clear_depth(1.0);
        assert_eq!(ops.depth_load_op, LoadOp::Clear(1.0));
        assert_eq!(ops.depth_store_op, StoreOp::Store);

        let ops = DepthStencilOps::load_depth();
        assert_eq!(ops.depth_load_op, LoadOp::Load);
        assert_eq!(ops.depth_store_op, StoreOp::Store);

        let ops = DepthStencilOps::clear_depth_stencil(1.0, 0);
        assert_eq!(ops.depth_load_op, LoadOp::Clear(1.0));
        assert_eq!(ops.stencil_load_op, LoadOp::Clear(0));
    }

    #[test]
    fn test_render_pass_descriptor_validation() {
        let descriptor = RenderPassDescriptor::new();
        assert!(descriptor.validate().is_err());
    }

    #[test]
    fn test_render_pass_descriptor_builder() {
        let descriptor = RenderPassDescriptor::new().with_label("test");
        assert_eq!(descriptor.label, Some("test"));
    }

    #[test]
    fn test_index_format() {
        let uint16 = IndexFormat::Uint16;
        let uint32 = IndexFormat::Uint32;

        assert_eq!(uint16, IndexFormat::Uint16);
        assert_eq!(uint32, IndexFormat::Uint32);
    }
}
