use wgpu::{
    Device, Extent3d, Texture, TextureAspect, TextureDimension, TextureFormat, TextureUsages,
    TextureView, TextureViewDescriptor, TextureViewDimension,
};

/// Builder for creating GPU textures with flexible configuration
///
/// This builder provides a fluent interface for creating textures with all
/// supported formats, dimensions, and usage flags.
///
/// # Examples
///
/// ```no_run
/// use wgpu_playground_core::texture::TextureBuilder;
/// # async fn example(device: &wgpu::Device) {
/// // Create a simple 2D RGBA texture
/// let texture = TextureBuilder::new()
///     .with_size(256, 256, 1)
///     .with_format(wgpu::TextureFormat::Rgba8Unorm)
///     .with_usage(wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST)
///     .build(device);
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct TextureBuilder {
    size: Extent3d,
    mip_level_count: u32,
    sample_count: u32,
    dimension: TextureDimension,
    format: TextureFormat,
    usage: TextureUsages,
    label: Option<String>,
    view_formats: Vec<TextureFormat>,
}

impl Default for TextureBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TextureBuilder {
    /// Create a new texture builder with default values
    ///
    /// Default configuration:
    /// - Size: 1x1x1
    /// - Format: Rgba8Unorm
    /// - Dimension: 2D
    /// - Mip levels: 1
    /// - Sample count: 1
    /// - Usage: TEXTURE_BINDING | COPY_DST
    pub fn new() -> Self {
        Self {
            size: Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            label: None,
            view_formats: Vec::new(),
        }
    }

    /// Set the texture size
    ///
    /// # Arguments
    /// * `width` - Width in pixels
    /// * `height` - Height in pixels (1 for 1D textures)
    /// * `depth_or_array_layers` - Depth for 3D textures or array layers for 2D/cube textures
    pub fn with_size(mut self, width: u32, height: u32, depth_or_array_layers: u32) -> Self {
        self.size = Extent3d {
            width,
            height,
            depth_or_array_layers,
        };
        self
    }

    /// Set the texture format
    ///
    /// # Arguments
    /// * `format` - The texture format (e.g., Rgba8Unorm, Bgra8Unorm, Depth32Float, etc.)
    pub fn with_format(mut self, format: TextureFormat) -> Self {
        self.format = format;
        self
    }

    /// Set the texture dimension
    ///
    /// # Arguments
    /// * `dimension` - The texture dimension (D1, D2, or D3)
    pub fn with_dimension(mut self, dimension: TextureDimension) -> Self {
        self.dimension = dimension;
        self
    }

    /// Set the texture usage flags
    ///
    /// # Arguments
    /// * `usage` - Bitflags specifying how the texture will be used
    pub fn with_usage(mut self, usage: TextureUsages) -> Self {
        self.usage = usage;
        self
    }

    /// Set the number of mip levels
    ///
    /// # Arguments
    /// * `mip_level_count` - Number of mip levels (must be >= 1)
    pub fn with_mip_levels(mut self, mip_level_count: u32) -> Self {
        self.mip_level_count = mip_level_count;
        self
    }

    /// Set the sample count for multisampling
    ///
    /// # Arguments
    /// * `sample_count` - Number of samples (1, 2, 4, 8, 16, or 32)
    pub fn with_sample_count(mut self, sample_count: u32) -> Self {
        self.sample_count = sample_count;
        self
    }

    /// Set a debug label for the texture
    ///
    /// # Arguments
    /// * `label` - A label to help identify this texture in debugging tools
    pub fn with_label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    /// Set view formats for the texture
    ///
    /// View formats allow creating texture views with different formats than the base texture.
    ///
    /// # Arguments
    /// * `formats` - Slice of formats that can be used to create views of this texture
    pub fn with_view_formats(mut self, formats: &[TextureFormat]) -> Self {
        self.view_formats = formats.to_vec();
        self
    }

    /// Build the texture on the given device
    ///
    /// # Arguments
    /// * `device` - The GPU device to create the texture on
    ///
    /// # Returns
    /// A new GPU texture
    pub fn build(&self, device: &Device) -> Texture {
        log::debug!(
            "Creating texture: label={:?}, size={}x{}x{}, format={:?}, dimension={:?}, mip_levels={}, samples={}",
            self.label,
            self.size.width,
            self.size.height,
            self.size.depth_or_array_layers,
            self.format,
            self.dimension,
            self.mip_level_count,
            self.sample_count
        );

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: self.label.as_deref(),
            size: self.size,
            mip_level_count: self.mip_level_count,
            sample_count: self.sample_count,
            dimension: self.dimension,
            format: self.format,
            usage: self.usage,
            view_formats: &self.view_formats,
        });

        log::trace!("Texture created successfully");
        texture
    }

    /// Helper method to create a 1D texture
    ///
    /// # Arguments
    /// * `width` - Width in pixels
    pub fn texture_1d(width: u32) -> Self {
        Self::new()
            .with_dimension(TextureDimension::D1)
            .with_size(width, 1, 1)
    }

    /// Helper method to create a 2D texture
    ///
    /// # Arguments
    /// * `width` - Width in pixels
    /// * `height` - Height in pixels
    pub fn texture_2d(width: u32, height: u32) -> Self {
        Self::new()
            .with_dimension(TextureDimension::D2)
            .with_size(width, height, 1)
    }

    /// Helper method to create a 3D texture
    ///
    /// # Arguments
    /// * `width` - Width in pixels
    /// * `height` - Height in pixels
    /// * `depth` - Depth in pixels
    pub fn texture_3d(width: u32, height: u32, depth: u32) -> Self {
        Self::new()
            .with_dimension(TextureDimension::D3)
            .with_size(width, height, depth)
    }

    /// Helper method to create a cube texture
    ///
    /// # Arguments
    /// * `size` - Size of each cube face (width and height)
    pub fn texture_cube(size: u32) -> Self {
        Self::new()
            .with_dimension(TextureDimension::D2)
            .with_size(size, size, 6) // Cube textures have 6 array layers
    }

    /// Helper method to create a 2D array texture
    ///
    /// # Arguments
    /// * `width` - Width in pixels
    /// * `height` - Height in pixels
    /// * `array_layers` - Number of array layers
    pub fn texture_2d_array(width: u32, height: u32, array_layers: u32) -> Self {
        Self::new()
            .with_dimension(TextureDimension::D2)
            .with_size(width, height, array_layers)
    }
}

/// Builder for creating texture views with flexible configuration
///
/// This builder provides a fluent interface for creating texture views with
/// different aspects, formats, dimensions, and mip/layer ranges.
///
/// # Examples
///
/// ```no_run
/// use wgpu_playground_core::texture::{TextureBuilder, TextureViewBuilder};
/// # async fn example(device: &wgpu::Device) {
/// let texture = TextureBuilder::new()
///     .with_size(256, 256, 1)
///     .with_mip_levels(4)
///     .build(device);
///
/// // Create a view of the first mip level only
/// let view = TextureViewBuilder::new()
///     .with_mip_level_range(0, 1)
///     .build(&texture);
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct TextureViewBuilder {
    label: Option<String>,
    format: Option<TextureFormat>,
    dimension: Option<TextureViewDimension>,
    aspect: TextureAspect,
    base_mip_level: u32,
    mip_level_count: Option<u32>,
    base_array_layer: u32,
    array_layer_count: Option<u32>,
}

impl Default for TextureViewBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TextureViewBuilder {
    /// Create a new texture view builder with default values
    ///
    /// Default configuration:
    /// - Format: None (uses texture's format)
    /// - Dimension: None (uses texture's dimension)
    /// - Aspect: All
    /// - Mip levels: All
    /// - Array layers: All
    pub fn new() -> Self {
        Self {
            label: None,
            format: None,
            dimension: None,
            aspect: TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
        }
    }

    /// Set a debug label for the texture view
    ///
    /// # Arguments
    /// * `label` - A label to help identify this view in debugging tools
    pub fn with_label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    /// Set the texture format for the view
    ///
    /// The format must be compatible with the base texture format.
    ///
    /// # Arguments
    /// * `format` - The texture format for the view
    pub fn with_format(mut self, format: TextureFormat) -> Self {
        self.format = Some(format);
        self
    }

    /// Set the texture view dimension
    ///
    /// # Arguments
    /// * `dimension` - The view dimension (D1, D2, D2Array, Cube, CubeArray, D3)
    pub fn with_dimension(mut self, dimension: TextureViewDimension) -> Self {
        self.dimension = Some(dimension);
        self
    }

    /// Set the texture aspect
    ///
    /// # Arguments
    /// * `aspect` - The aspect to view (All, StencilOnly, DepthOnly, Plane0, Plane1, Plane2)
    pub fn with_aspect(mut self, aspect: TextureAspect) -> Self {
        self.aspect = aspect;
        self
    }

    /// Set the mip level range for the view
    ///
    /// # Arguments
    /// * `base_mip_level` - The first mip level to include
    /// * `mip_level_count` - Number of mip levels to include
    pub fn with_mip_level_range(mut self, base_mip_level: u32, mip_level_count: u32) -> Self {
        self.base_mip_level = base_mip_level;
        self.mip_level_count = Some(mip_level_count);
        self
    }

    /// Set the array layer range for the view
    ///
    /// # Arguments
    /// * `base_array_layer` - The first array layer to include
    /// * `array_layer_count` - Number of array layers to include
    pub fn with_array_layer_range(mut self, base_array_layer: u32, array_layer_count: u32) -> Self {
        self.base_array_layer = base_array_layer;
        self.array_layer_count = Some(array_layer_count);
        self
    }

    /// Build the texture view from the given texture
    ///
    /// # Arguments
    /// * `texture` - The base texture to create a view from
    ///
    /// # Returns
    /// A new texture view
    pub fn build(&self, texture: &Texture) -> TextureView {
        log::debug!(
            "Creating texture view: label={:?}, format={:?}, dimension={:?}",
            self.label,
            self.format,
            self.dimension
        );

        let view = texture.create_view(&TextureViewDescriptor {
            label: self.label.as_deref(),
            format: self.format,
            dimension: self.dimension,
            aspect: self.aspect,
            base_mip_level: self.base_mip_level,
            mip_level_count: self.mip_level_count,
            base_array_layer: self.base_array_layer,
            array_layer_count: self.array_layer_count,
        });

        log::trace!("Texture view created successfully");
        view
    }

    /// Helper method to create a depth-only view
    pub fn depth_only() -> Self {
        Self::new().with_aspect(TextureAspect::DepthOnly)
    }

    /// Helper method to create a stencil-only view
    pub fn stencil_only() -> Self {
        Self::new().with_aspect(TextureAspect::StencilOnly)
    }

    /// Helper method to create a cube view from a 2D array texture
    pub fn as_cube() -> Self {
        Self::new()
            .with_dimension(TextureViewDimension::Cube)
            .with_array_layer_range(0, 6)
    }

    /// Helper method to create a 2D array view
    pub fn as_2d_array(layer_count: u32) -> Self {
        Self::new()
            .with_dimension(TextureViewDimension::D2Array)
            .with_array_layer_range(0, layer_count)
    }
}

/// Helper function to create a simple 2D texture
///
/// # Arguments
/// * `device` - The GPU device
/// * `width` - Width in pixels
/// * `height` - Height in pixels
/// * `format` - Texture format
/// * `usage` - Usage flags
///
/// # Returns
/// A new 2D texture
///
/// # Examples
/// ```no_run
/// use wgpu_playground_core::texture::create_texture_2d;
/// # async fn example(device: &wgpu::Device) {
/// let texture = create_texture_2d(
///     device,
///     256,
///     256,
///     wgpu::TextureFormat::Rgba8Unorm,
///     wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
/// );
/// # }
/// ```
pub fn create_texture_2d(
    device: &Device,
    width: u32,
    height: u32,
    format: TextureFormat,
    usage: TextureUsages,
) -> Texture {
    TextureBuilder::texture_2d(width, height)
        .with_format(format)
        .with_usage(usage)
        .build(device)
}

/// Helper function to create a simple texture view
///
/// # Arguments
/// * `texture` - The base texture
///
/// # Returns
/// A new texture view with default settings
pub fn create_default_view(texture: &Texture) -> TextureView {
    TextureViewBuilder::new().build(texture)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_builder_default() {
        let builder = TextureBuilder::new();
        assert_eq!(builder.size.width, 1);
        assert_eq!(builder.size.height, 1);
        assert_eq!(builder.size.depth_or_array_layers, 1);
        assert_eq!(builder.mip_level_count, 1);
        assert_eq!(builder.sample_count, 1);
        assert_eq!(builder.dimension, TextureDimension::D2);
        assert_eq!(builder.format, TextureFormat::Rgba8Unorm);
    }

    #[test]
    fn test_texture_builder_with_size() {
        let builder = TextureBuilder::new().with_size(256, 256, 1);
        assert_eq!(builder.size.width, 256);
        assert_eq!(builder.size.height, 256);
        assert_eq!(builder.size.depth_or_array_layers, 1);
    }

    #[test]
    fn test_texture_builder_with_format() {
        let builder = TextureBuilder::new().with_format(TextureFormat::Bgra8Unorm);
        assert_eq!(builder.format, TextureFormat::Bgra8Unorm);
    }

    #[test]
    fn test_texture_builder_with_dimension() {
        let builder = TextureBuilder::new().with_dimension(TextureDimension::D3);
        assert_eq!(builder.dimension, TextureDimension::D3);
    }

    #[test]
    fn test_texture_builder_with_mip_levels() {
        let builder = TextureBuilder::new().with_mip_levels(4);
        assert_eq!(builder.mip_level_count, 4);
    }

    #[test]
    fn test_texture_builder_with_sample_count() {
        let builder = TextureBuilder::new().with_sample_count(4);
        assert_eq!(builder.sample_count, 4);
    }

    #[test]
    fn test_texture_builder_with_label() {
        let builder = TextureBuilder::new().with_label("test_texture");
        assert_eq!(builder.label, Some("test_texture".to_string()));
    }

    #[test]
    fn test_texture_builder_with_view_formats() {
        let formats = vec![TextureFormat::Rgba8Unorm, TextureFormat::Rgba8UnormSrgb];
        let builder = TextureBuilder::new().with_view_formats(&formats);
        assert_eq!(builder.view_formats, formats);
    }

    #[test]
    fn test_texture_builder_1d() {
        let builder = TextureBuilder::texture_1d(256);
        assert_eq!(builder.dimension, TextureDimension::D1);
        assert_eq!(builder.size.width, 256);
        assert_eq!(builder.size.height, 1);
        assert_eq!(builder.size.depth_or_array_layers, 1);
    }

    #[test]
    fn test_texture_builder_2d() {
        let builder = TextureBuilder::texture_2d(256, 256);
        assert_eq!(builder.dimension, TextureDimension::D2);
        assert_eq!(builder.size.width, 256);
        assert_eq!(builder.size.height, 256);
        assert_eq!(builder.size.depth_or_array_layers, 1);
    }

    #[test]
    fn test_texture_builder_3d() {
        let builder = TextureBuilder::texture_3d(64, 64, 64);
        assert_eq!(builder.dimension, TextureDimension::D3);
        assert_eq!(builder.size.width, 64);
        assert_eq!(builder.size.height, 64);
        assert_eq!(builder.size.depth_or_array_layers, 64);
    }

    #[test]
    fn test_texture_builder_cube() {
        let builder = TextureBuilder::texture_cube(256);
        assert_eq!(builder.dimension, TextureDimension::D2);
        assert_eq!(builder.size.width, 256);
        assert_eq!(builder.size.height, 256);
        assert_eq!(builder.size.depth_or_array_layers, 6);
    }

    #[test]
    fn test_texture_builder_2d_array() {
        let builder = TextureBuilder::texture_2d_array(128, 128, 4);
        assert_eq!(builder.dimension, TextureDimension::D2);
        assert_eq!(builder.size.width, 128);
        assert_eq!(builder.size.height, 128);
        assert_eq!(builder.size.depth_or_array_layers, 4);
    }

    #[test]
    fn test_texture_view_builder_default() {
        let builder = TextureViewBuilder::new();
        assert_eq!(builder.aspect, TextureAspect::All);
        assert_eq!(builder.base_mip_level, 0);
        assert_eq!(builder.mip_level_count, None);
        assert_eq!(builder.base_array_layer, 0);
        assert_eq!(builder.array_layer_count, None);
    }

    #[test]
    fn test_texture_view_builder_with_label() {
        let builder = TextureViewBuilder::new().with_label("test_view");
        assert_eq!(builder.label, Some("test_view".to_string()));
    }

    #[test]
    fn test_texture_view_builder_with_format() {
        let builder = TextureViewBuilder::new().with_format(TextureFormat::Rgba8UnormSrgb);
        assert_eq!(builder.format, Some(TextureFormat::Rgba8UnormSrgb));
    }

    #[test]
    fn test_texture_view_builder_with_dimension() {
        let builder = TextureViewBuilder::new().with_dimension(TextureViewDimension::Cube);
        assert_eq!(builder.dimension, Some(TextureViewDimension::Cube));
    }

    #[test]
    fn test_texture_view_builder_with_aspect() {
        let builder = TextureViewBuilder::new().with_aspect(TextureAspect::DepthOnly);
        assert_eq!(builder.aspect, TextureAspect::DepthOnly);
    }

    #[test]
    fn test_texture_view_builder_with_mip_level_range() {
        let builder = TextureViewBuilder::new().with_mip_level_range(1, 3);
        assert_eq!(builder.base_mip_level, 1);
        assert_eq!(builder.mip_level_count, Some(3));
    }

    #[test]
    fn test_texture_view_builder_with_array_layer_range() {
        let builder = TextureViewBuilder::new().with_array_layer_range(2, 4);
        assert_eq!(builder.base_array_layer, 2);
        assert_eq!(builder.array_layer_count, Some(4));
    }

    #[test]
    fn test_texture_view_builder_depth_only() {
        let builder = TextureViewBuilder::depth_only();
        assert_eq!(builder.aspect, TextureAspect::DepthOnly);
    }

    #[test]
    fn test_texture_view_builder_stencil_only() {
        let builder = TextureViewBuilder::stencil_only();
        assert_eq!(builder.aspect, TextureAspect::StencilOnly);
    }

    #[test]
    fn test_texture_view_builder_as_cube() {
        let builder = TextureViewBuilder::as_cube();
        assert_eq!(builder.dimension, Some(TextureViewDimension::Cube));
        assert_eq!(builder.base_array_layer, 0);
        assert_eq!(builder.array_layer_count, Some(6));
    }

    #[test]
    fn test_texture_view_builder_as_2d_array() {
        let builder = TextureViewBuilder::as_2d_array(8);
        assert_eq!(builder.dimension, Some(TextureViewDimension::D2Array));
        assert_eq!(builder.base_array_layer, 0);
        assert_eq!(builder.array_layer_count, Some(8));
    }

    #[test]
    fn test_builder_chaining() {
        let builder = TextureBuilder::new()
            .with_size(512, 512, 1)
            .with_format(TextureFormat::Rgba16Float)
            .with_dimension(TextureDimension::D2)
            .with_mip_levels(5)
            .with_sample_count(1)
            .with_label("chained_texture")
            .with_usage(TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING);

        assert_eq!(builder.size.width, 512);
        assert_eq!(builder.size.height, 512);
        assert_eq!(builder.format, TextureFormat::Rgba16Float);
        assert_eq!(builder.dimension, TextureDimension::D2);
        assert_eq!(builder.mip_level_count, 5);
        assert_eq!(builder.sample_count, 1);
        assert_eq!(builder.label, Some("chained_texture".to_string()));
    }
}
