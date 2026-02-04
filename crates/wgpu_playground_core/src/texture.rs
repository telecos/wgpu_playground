use wgpu::{
    Device, Extent3d, Texture, TextureAspect, TextureDimension, TextureFormat, TextureUsages,
    TextureView, TextureViewDescriptor, TextureViewDimension, Origin3d, TexelCopyTextureInfo,
    TexelCopyBufferLayout,
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
            usage: None,
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

/// Load a texture from image file bytes
///
/// Supports PNG, JPEG, and other formats supported by the image crate.
///
/// # Arguments
/// * `device` - The GPU device
/// * `queue` - The GPU queue for uploading data
/// * `bytes` - The image file bytes
/// * `label` - Optional label for the texture
///
/// # Returns
/// Result containing the texture and its dimensions, or an error message
///
/// # Examples
/// ```no_run
/// use wgpu_playground_core::texture::load_texture_from_bytes;
/// # async fn example(device: &wgpu::Device, queue: &wgpu::Queue) {
/// let image_bytes = std::fs::read("image.png").unwrap();
/// let (texture, width, height) = load_texture_from_bytes(
///     device,
///     queue,
///     &image_bytes,
///     Some("Loaded Image"),
/// ).unwrap();
/// # }
/// ```
pub fn load_texture_from_bytes(
    device: &Device,
    queue: &wgpu::Queue,
    bytes: &[u8],
    label: Option<&str>,
) -> Result<(Texture, u32, u32), String> {
    use image::GenericImageView;

    // Decode the image
    let img = image::load_from_memory(bytes)
        .map_err(|e| format!("Failed to decode image: {}", e))?;

    let rgba = img.to_rgba8();
    let dimensions = img.dimensions();

    // Create the texture
    let texture = TextureBuilder::new()
        .with_size(dimensions.0, dimensions.1, 1)
        .with_format(TextureFormat::Rgba8UnormSrgb)
        .with_usage(TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::COPY_SRC)
        .with_label(label.unwrap_or("Loaded Texture"))
        .build(device);

    // Upload the image data to the texture
    queue.write_texture(
        TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: Origin3d::ZERO,
            aspect: TextureAspect::All,
        },
        &rgba,
        TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4 * dimensions.0),
            rows_per_image: Some(dimensions.1),
        },
        Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        },
    );

    log::info!(
        "Loaded texture from bytes: {}x{}, format: Rgba8UnormSrgb",
        dimensions.0,
        dimensions.1
    );

    Ok((texture, dimensions.0, dimensions.1))
}

/// Export texture data to image file bytes
///
/// Exports texture data as PNG format.
///
/// # Arguments
/// * `device` - The GPU device
/// * `queue` - The GPU queue
/// * `texture` - The texture to export
/// * `width` - Texture width
/// * `height` - Texture height
///
/// # Returns
/// Result containing the PNG file bytes, or an error message
///
/// # Examples
/// ```no_run
/// use wgpu_playground_core::texture::export_texture_to_bytes;
/// # async fn example(device: &wgpu::Device, queue: &wgpu::Queue, texture: &wgpu::Texture) {
/// let png_bytes = export_texture_to_bytes(
///     device,
///     queue,
///     texture,
///     256,
///     256,
/// ).await.unwrap();
/// std::fs::write("exported.png", png_bytes).unwrap();
/// # }
/// ```
pub async fn export_texture_to_bytes(
    device: &Device,
    queue: &wgpu::Queue,
    texture: &Texture,
    width: u32,
    height: u32,
) -> Result<Vec<u8>, String> {
    use image::{ImageBuffer, Rgba};

    // Create a buffer to read the texture data
    let buffer_size = (width * height * 4) as wgpu::BufferAddress;
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Texture Export Buffer"),
        size: buffer_size,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    // Create a command encoder to copy texture to buffer
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Texture Export Encoder"),
    });

    encoder.copy_texture_to_buffer(
        TexelCopyTextureInfo {
            texture,
            mip_level: 0,
            origin: Origin3d::ZERO,
            aspect: TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &buffer,
            layout: TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
        },
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );

    queue.submit(Some(encoder.finish()));

    // Map the buffer to read the data
    let buffer_slice = buffer.slice(..);
    let (sender, receiver) = futures_channel::oneshot::channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
        let _ = sender.send(result);
    });

    let _ = device.poll(wgpu::PollType::Wait {
        submission_index: None,
        timeout: None,
    });

    receiver
        .await
        .map_err(|_| {
            log::error!("Failed to receive buffer mapping result - receiver was dropped");
            "Failed to receive buffer mapping result".to_string()
        })?
        .map_err(|e| format!("Failed to map buffer: {:?}", e))?;

    let data = buffer_slice.get_mapped_range();
    let rgba_data: Vec<u8> = data.to_vec();
    drop(data);
    buffer.unmap();

    // Create an image from the RGBA data
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, rgba_data)
        .ok_or_else(|| "Failed to create image buffer from texture data".to_string())?;

    // Encode to PNG
    let mut png_bytes = Vec::new();
    img.write_to(
        &mut std::io::Cursor::new(&mut png_bytes),
        image::ImageFormat::Png,
    )
    .map_err(|e| format!("Failed to encode PNG: {}", e))?;

    log::info!("Exported texture to PNG: {}x{} ({} bytes)", width, height, png_bytes.len());

    Ok(png_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::GenericImageView;

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

    // Edge Cases and Boundary Tests

    #[test]
    fn test_texture_builder_minimum_size() {
        // Test minimum valid texture size (1x1x1)
        let builder = TextureBuilder::new().with_size(1, 1, 1);
        assert_eq!(builder.size.width, 1);
        assert_eq!(builder.size.height, 1);
        assert_eq!(builder.size.depth_or_array_layers, 1);
    }

    #[test]
    fn test_texture_builder_large_size() {
        // Test large texture dimensions (within typical GPU limits)
        let builder = TextureBuilder::new().with_size(8192, 8192, 1);
        assert_eq!(builder.size.width, 8192);
        assert_eq!(builder.size.height, 8192);
    }

    #[test]
    fn test_texture_builder_1d_edge_cases() {
        // Test 1D texture with minimum width
        let builder = TextureBuilder::texture_1d(1);
        assert_eq!(builder.size.width, 1);
        assert_eq!(builder.dimension, TextureDimension::D1);

        // Test 1D texture with large width
        let builder = TextureBuilder::texture_1d(16384);
        assert_eq!(builder.size.width, 16384);
    }

    #[test]
    fn test_texture_builder_2d_edge_cases() {
        // Test 2D texture with minimum dimensions
        let builder = TextureBuilder::texture_2d(1, 1);
        assert_eq!(builder.size.width, 1);
        assert_eq!(builder.size.height, 1);

        // Test 2D texture with non-square dimensions
        let builder = TextureBuilder::texture_2d(256, 128);
        assert_eq!(builder.size.width, 256);
        assert_eq!(builder.size.height, 128);
    }

    #[test]
    fn test_texture_builder_3d_edge_cases() {
        // Test 3D texture with minimum dimensions
        let builder = TextureBuilder::texture_3d(1, 1, 1);
        assert_eq!(builder.size.width, 1);
        assert_eq!(builder.size.height, 1);
        assert_eq!(builder.size.depth_or_array_layers, 1);

        // Test 3D texture with non-uniform dimensions
        let builder = TextureBuilder::texture_3d(128, 64, 32);
        assert_eq!(builder.size.width, 128);
        assert_eq!(builder.size.height, 64);
        assert_eq!(builder.size.depth_or_array_layers, 32);
    }

    #[test]
    fn test_texture_builder_array_edge_cases() {
        // Test array with single layer
        let builder = TextureBuilder::texture_2d_array(256, 256, 1);
        assert_eq!(builder.size.depth_or_array_layers, 1);

        // Test array with many layers
        let builder = TextureBuilder::texture_2d_array(256, 256, 256);
        assert_eq!(builder.size.depth_or_array_layers, 256);
    }

    #[test]
    fn test_texture_builder_cube_minimum_size() {
        // Test cube texture with minimum size
        let builder = TextureBuilder::texture_cube(1);
        assert_eq!(builder.size.width, 1);
        assert_eq!(builder.size.height, 1);
        assert_eq!(builder.size.depth_or_array_layers, 6);
    }

    #[test]
    fn test_texture_builder_mip_levels_edge_cases() {
        // Test single mip level
        let builder = TextureBuilder::new().with_mip_levels(1);
        assert_eq!(builder.mip_level_count, 1);

        // Test multiple mip levels
        let builder = TextureBuilder::new().with_mip_levels(10);
        assert_eq!(builder.mip_level_count, 10);

        // Test maximum reasonable mip levels for a 256x256 texture (log2(256) + 1 = 9)
        let builder = TextureBuilder::texture_2d(256, 256).with_mip_levels(9);
        assert_eq!(builder.mip_level_count, 9);
    }

    #[test]
    fn test_texture_builder_sample_count_values() {
        // Test all valid MSAA sample counts
        for &count in &[1, 2, 4, 8, 16, 32] {
            let builder = TextureBuilder::new().with_sample_count(count);
            assert_eq!(builder.sample_count, count);
        }
    }

    // Format Support Tests

    #[test]
    fn test_texture_builder_color_formats() {
        // Test various color formats
        let color_formats = vec![
            TextureFormat::Rgba8Unorm,
            TextureFormat::Rgba8UnormSrgb,
            TextureFormat::Bgra8Unorm,
            TextureFormat::Bgra8UnormSrgb,
            TextureFormat::Rgba16Float,
            TextureFormat::Rgba32Float,
            TextureFormat::Rgb10a2Unorm,
            TextureFormat::R8Unorm,
            TextureFormat::R8Snorm,
            TextureFormat::R8Uint,
            TextureFormat::R8Sint,
            TextureFormat::R16Uint,
            TextureFormat::R16Sint,
            TextureFormat::R16Float,
            TextureFormat::Rg8Unorm,
            TextureFormat::Rg8Snorm,
            TextureFormat::Rg8Uint,
            TextureFormat::Rg8Sint,
            TextureFormat::Rg16Uint,
            TextureFormat::Rg16Sint,
            TextureFormat::Rg16Float,
            TextureFormat::Rgba16Uint,
            TextureFormat::Rgba16Sint,
            TextureFormat::Rgba32Uint,
            TextureFormat::Rgba32Sint,
        ];

        for format in color_formats {
            let builder = TextureBuilder::new().with_format(format);
            assert_eq!(builder.format, format);
        }
    }

    #[test]
    fn test_texture_builder_depth_stencil_formats() {
        // Test depth and stencil formats
        let depth_stencil_formats = vec![
            TextureFormat::Depth32Float,
            TextureFormat::Depth24Plus,
            TextureFormat::Depth24PlusStencil8,
            TextureFormat::Stencil8,
        ];

        for format in depth_stencil_formats {
            let builder = TextureBuilder::new().with_format(format);
            assert_eq!(builder.format, format);
        }
    }

    #[test]
    fn test_texture_builder_compressed_formats() {
        // Test compressed texture formats
        let compressed_formats = vec![
            TextureFormat::Bc1RgbaUnorm,
            TextureFormat::Bc1RgbaUnormSrgb,
            TextureFormat::Bc2RgbaUnorm,
            TextureFormat::Bc2RgbaUnormSrgb,
            TextureFormat::Bc3RgbaUnorm,
            TextureFormat::Bc3RgbaUnormSrgb,
            TextureFormat::Bc4RUnorm,
            TextureFormat::Bc4RSnorm,
            TextureFormat::Bc5RgUnorm,
            TextureFormat::Bc5RgSnorm,
            TextureFormat::Bc6hRgbUfloat,
            TextureFormat::Bc6hRgbFloat,
            TextureFormat::Bc7RgbaUnorm,
            TextureFormat::Bc7RgbaUnormSrgb,
        ];

        for format in compressed_formats {
            let builder = TextureBuilder::new().with_format(format);
            assert_eq!(builder.format, format);
        }
    }

    // Dimension Validation Tests

    #[test]
    fn test_texture_dimensions() {
        // Test all texture dimensions
        let builder = TextureBuilder::new().with_dimension(TextureDimension::D1);
        assert_eq!(builder.dimension, TextureDimension::D1);

        let builder = TextureBuilder::new().with_dimension(TextureDimension::D2);
        assert_eq!(builder.dimension, TextureDimension::D2);

        let builder = TextureBuilder::new().with_dimension(TextureDimension::D3);
        assert_eq!(builder.dimension, TextureDimension::D3);
    }

    #[test]
    fn test_texture_1d_constraints() {
        // 1D textures should have height = 1 and depth/array = 1 for single texture
        let builder = TextureBuilder::texture_1d(512);
        assert_eq!(builder.dimension, TextureDimension::D1);
        assert_eq!(builder.size.height, 1);
        assert_eq!(builder.size.depth_or_array_layers, 1);
    }

    #[test]
    fn test_texture_2d_constraints() {
        // 2D textures can have arbitrary width and height, depth/array = 1 for single texture
        let builder = TextureBuilder::texture_2d(1024, 768);
        assert_eq!(builder.dimension, TextureDimension::D2);
        assert_eq!(builder.size.depth_or_array_layers, 1);
    }

    #[test]
    fn test_texture_3d_constraints() {
        // 3D textures can have arbitrary width, height, and depth
        let builder = TextureBuilder::texture_3d(64, 128, 256);
        assert_eq!(builder.dimension, TextureDimension::D3);
        assert_eq!(builder.size.width, 64);
        assert_eq!(builder.size.height, 128);
        assert_eq!(builder.size.depth_or_array_layers, 256);
    }

    #[test]
    fn test_cube_texture_constraints() {
        // Cube textures must be square and have 6 array layers
        let builder = TextureBuilder::texture_cube(512);
        assert_eq!(builder.dimension, TextureDimension::D2);
        assert_eq!(builder.size.width, 512);
        assert_eq!(builder.size.height, 512);
        assert_eq!(builder.size.depth_or_array_layers, 6);
    }

    // Usage Flags Tests

    #[test]
    fn test_texture_usage_combinations() {
        // Test various usage flag combinations
        let usage_combinations = vec![
            TextureUsages::TEXTURE_BINDING,
            TextureUsages::COPY_DST,
            TextureUsages::COPY_SRC,
            TextureUsages::RENDER_ATTACHMENT,
            TextureUsages::STORAGE_BINDING,
            TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_SRC,
            TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            TextureUsages::STORAGE_BINDING | TextureUsages::COPY_DST,
            TextureUsages::RENDER_ATTACHMENT | TextureUsages::COPY_SRC,
            TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::COPY_SRC,
        ];

        for usage in usage_combinations {
            let builder = TextureBuilder::new().with_usage(usage);
            assert_eq!(builder.usage, usage);
        }
    }

    #[test]
    fn test_texture_empty_label() {
        // Test with empty string label
        let builder = TextureBuilder::new().with_label("");
        assert_eq!(builder.label, Some("".to_string()));
    }

    #[test]
    fn test_texture_long_label() {
        // Test with long label string
        let long_label = "a".repeat(1000);
        let builder = TextureBuilder::new().with_label(&long_label);
        assert_eq!(builder.label, Some(long_label));
    }

    #[test]
    fn test_texture_view_formats_empty() {
        // Test with empty view formats
        let builder = TextureBuilder::new().with_view_formats(&[]);
        assert_eq!(builder.view_formats, vec![]);
    }

    #[test]
    fn test_texture_view_formats_multiple() {
        // Test with multiple compatible view formats
        let formats = vec![TextureFormat::Rgba8Unorm, TextureFormat::Rgba8UnormSrgb];
        let builder = TextureBuilder::new().with_view_formats(&formats);
        assert_eq!(builder.view_formats, formats);
    }

    // Texture View Builder Tests

    #[test]
    fn test_texture_view_all_dimensions() {
        // Test all texture view dimensions
        let dimensions = vec![
            TextureViewDimension::D1,
            TextureViewDimension::D2,
            TextureViewDimension::D2Array,
            TextureViewDimension::Cube,
            TextureViewDimension::CubeArray,
            TextureViewDimension::D3,
        ];

        for dimension in dimensions {
            let builder = TextureViewBuilder::new().with_dimension(dimension);
            assert_eq!(builder.dimension, Some(dimension));
        }
    }

    #[test]
    fn test_texture_view_all_aspects() {
        // Test all texture aspects
        let aspects = vec![
            TextureAspect::All,
            TextureAspect::StencilOnly,
            TextureAspect::DepthOnly,
        ];

        for aspect in aspects {
            let builder = TextureViewBuilder::new().with_aspect(aspect);
            assert_eq!(builder.aspect, aspect);
        }
    }

    #[test]
    fn test_texture_view_mip_range_edge_cases() {
        // Test single mip level
        let builder = TextureViewBuilder::new().with_mip_level_range(0, 1);
        assert_eq!(builder.base_mip_level, 0);
        assert_eq!(builder.mip_level_count, Some(1));

        // Test middle mip levels
        let builder = TextureViewBuilder::new().with_mip_level_range(2, 3);
        assert_eq!(builder.base_mip_level, 2);
        assert_eq!(builder.mip_level_count, Some(3));

        // Test last mip level
        let builder = TextureViewBuilder::new().with_mip_level_range(8, 1);
        assert_eq!(builder.base_mip_level, 8);
        assert_eq!(builder.mip_level_count, Some(1));
    }

    #[test]
    fn test_texture_view_array_range_edge_cases() {
        // Test single array layer
        let builder = TextureViewBuilder::new().with_array_layer_range(0, 1);
        assert_eq!(builder.base_array_layer, 0);
        assert_eq!(builder.array_layer_count, Some(1));

        // Test middle array layers
        let builder = TextureViewBuilder::new().with_array_layer_range(5, 10);
        assert_eq!(builder.base_array_layer, 5);
        assert_eq!(builder.array_layer_count, Some(10));
    }

    #[test]
    fn test_texture_view_cube_array() {
        // Cube arrays require multiples of 6 layers
        let builder = TextureViewBuilder::new()
            .with_dimension(TextureViewDimension::CubeArray)
            .with_array_layer_range(0, 12); // 2 cubes
        assert_eq!(builder.dimension, Some(TextureViewDimension::CubeArray));
        assert_eq!(builder.array_layer_count, Some(12));
    }

    #[test]
    fn test_default_trait_implementations() {
        // Test Default trait for TextureBuilder
        let default_texture_builder = TextureBuilder::default();
        let new_texture_builder = TextureBuilder::new();
        assert_eq!(
            default_texture_builder.size.width,
            new_texture_builder.size.width
        );
        assert_eq!(
            default_texture_builder.size.height,
            new_texture_builder.size.height
        );
        assert_eq!(default_texture_builder.format, new_texture_builder.format);

        // Test Default trait for TextureViewBuilder
        let default_view_builder = TextureViewBuilder::default();
        let new_view_builder = TextureViewBuilder::new();
        assert_eq!(default_view_builder.aspect, new_view_builder.aspect);
        assert_eq!(
            default_view_builder.base_mip_level,
            new_view_builder.base_mip_level
        );
    }

    #[test]
    fn test_texture_builder_clone() {
        // Test that builders are clonable
        let builder = TextureBuilder::new()
            .with_size(256, 256, 1)
            .with_format(TextureFormat::Rgba8Unorm)
            .with_label("test");

        let cloned = builder.clone();
        assert_eq!(builder.size.width, cloned.size.width);
        assert_eq!(builder.format, cloned.format);
        assert_eq!(builder.label, cloned.label);
    }

    #[test]
    fn test_texture_view_builder_clone() {
        // Test that view builders are clonable
        let builder = TextureViewBuilder::new()
            .with_label("test_view")
            .with_mip_level_range(0, 4);

        let cloned = builder.clone();
        assert_eq!(builder.label, cloned.label);
        assert_eq!(builder.base_mip_level, cloned.base_mip_level);
        assert_eq!(builder.mip_level_count, cloned.mip_level_count);
    }

    #[test]
    fn test_load_texture_from_bytes_invalid_data() {
        // Test loading from invalid image data
        let invalid_bytes = vec![0u8; 100];
        
        // This should fail since it's not valid image data
        assert!(image::load_from_memory(&invalid_bytes).is_err());
    }

    #[test]
    fn test_load_texture_from_bytes_png_format() {
        // Create a minimal valid PNG (1x1 pixel, white)
        let png_data = vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
            0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, // 1x1 dimensions
            0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53,
            0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, // IDAT chunk
            0x54, 0x08, 0xD7, 0x63, 0xF8, 0xFF, 0xFF, 0x3F,
            0x00, 0x05, 0xFE, 0x02, 0xFE, 0xDC, 0xCC, 0x59,
            0xE7, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, // IEND chunk
            0x44, 0xAE, 0x42, 0x60, 0x82,
        ];

        // This should successfully decode
        let result = image::load_from_memory(&png_data);
        assert!(result.is_ok());
        
        if let Ok(img) = result {
            assert_eq!(img.dimensions(), (1, 1));
        }
    }
}
