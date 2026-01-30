use wgpu::{
    CompositeAlphaMode, Device, Instance, PresentMode, Surface, SurfaceConfiguration,
    SurfaceTexture, TextureFormat, TextureUsages,
};

/// Builder for creating and configuring GPU surfaces
///
/// This builder provides a fluent interface for configuring surfaces with
/// format, present mode, alpha mode, and other settings.
///
/// # Examples
///
/// ```no_run
/// use wgpu_playground_core::surface::SurfaceConfigurationBuilder;
/// # async fn example(device: &wgpu::Device, surface: &wgpu::Surface<'_>) {
/// // Create a surface configuration
/// let config = SurfaceConfigurationBuilder::new(800, 600)
///     .with_format(wgpu::TextureFormat::Bgra8Unorm)
///     .with_present_mode(wgpu::PresentMode::Fifo)
///     .with_alpha_mode(wgpu::CompositeAlphaMode::Opaque)
///     .build();
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct SurfaceConfigurationBuilder {
    width: u32,
    height: u32,
    format: TextureFormat,
    usage: TextureUsages,
    present_mode: PresentMode,
    alpha_mode: CompositeAlphaMode,
    view_formats: Vec<TextureFormat>,
    desired_maximum_frame_latency: u32,
}

impl SurfaceConfigurationBuilder {
    /// Create a new surface configuration builder with the specified dimensions
    ///
    /// Default configuration:
    /// - Format: Bgra8Unorm
    /// - Usage: RENDER_ATTACHMENT
    /// - Present mode: Fifo (VSync)
    /// - Alpha mode: Opaque
    /// - Desired maximum frame latency: 2
    ///
    /// # Arguments
    /// * `width` - Width in pixels
    /// * `height` - Height in pixels
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            format: TextureFormat::Bgra8Unorm,
            usage: TextureUsages::RENDER_ATTACHMENT,
            present_mode: PresentMode::Fifo,
            alpha_mode: CompositeAlphaMode::Opaque,
            view_formats: Vec::new(),
            desired_maximum_frame_latency: 2,
        }
    }

    /// Set the texture format for the surface
    ///
    /// # Arguments
    /// * `format` - The texture format (e.g., Bgra8Unorm, Rgba8Unorm)
    pub fn with_format(mut self, format: TextureFormat) -> Self {
        self.format = format;
        self
    }

    /// Set the texture usage flags
    ///
    /// # Arguments
    /// * `usage` - Bitflags specifying how the surface texture will be used
    pub fn with_usage(mut self, usage: TextureUsages) -> Self {
        self.usage = usage;
        self
    }

    /// Set the present mode
    ///
    /// # Arguments
    /// * `present_mode` - The presentation mode (Immediate, Mailbox, Fifo, etc.)
    pub fn with_present_mode(mut self, present_mode: PresentMode) -> Self {
        self.present_mode = present_mode;
        self
    }

    /// Set the alpha composition mode
    ///
    /// # Arguments
    /// * `alpha_mode` - How the alpha channel should be composited (Opaque, PreMultiplied, PostMultiplied, Inherit)
    pub fn with_alpha_mode(mut self, alpha_mode: CompositeAlphaMode) -> Self {
        self.alpha_mode = alpha_mode;
        self
    }

    /// Set view formats for the surface
    ///
    /// View formats allow creating texture views with different formats than the base texture.
    ///
    /// # Arguments
    /// * `formats` - Slice of formats that can be used to create views of this surface
    pub fn with_view_formats(mut self, formats: &[TextureFormat]) -> Self {
        self.view_formats = formats.to_vec();
        self
    }

    /// Set the desired maximum frame latency
    ///
    /// # Arguments
    /// * `latency` - The desired maximum number of frames that can be in-flight at once
    pub fn with_desired_maximum_frame_latency(mut self, latency: u32) -> Self {
        self.desired_maximum_frame_latency = latency;
        self
    }

    /// Update the dimensions of the surface configuration
    ///
    /// # Arguments
    /// * `width` - New width in pixels
    /// * `height` - New height in pixels
    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Build the surface configuration
    ///
    /// # Returns
    /// A new surface configuration
    pub fn build(&self) -> SurfaceConfiguration {
        SurfaceConfiguration {
            usage: self.usage,
            format: self.format,
            width: self.width,
            height: self.height,
            present_mode: self.present_mode,
            alpha_mode: self.alpha_mode,
            view_formats: self.view_formats.clone(),
            desired_maximum_frame_latency: self.desired_maximum_frame_latency,
        }
    }
}

/// Helper function to configure a surface with the given configuration
///
/// # Arguments
/// * `surface` - The surface to configure
/// * `device` - The GPU device
/// * `config` - The surface configuration
///
/// # Examples
/// ```no_run
/// use wgpu_playground_core::surface::{configure_surface, SurfaceConfigurationBuilder};
/// # async fn example(device: &wgpu::Device, surface: &wgpu::Surface<'_>) {
/// let config = SurfaceConfigurationBuilder::new(800, 600).build();
/// configure_surface(surface, device, &config);
/// # }
/// ```
pub fn configure_surface(surface: &Surface, device: &Device, config: &SurfaceConfiguration) {
    surface.configure(device, config);
}

/// Get the current texture from a surface for rendering
///
/// # Arguments
/// * `surface` - The surface to get the texture from
///
/// # Returns
/// The current surface texture, or an error if the surface is invalid
///
/// # Errors
/// Returns a `SurfaceError` if:
/// - The surface was lost and needs to be reconfigured
/// - The presentation system ran out of memory
/// - The surface timed out waiting for the next frame
/// - The surface is outdated and needs to be reconfigured
///
/// # Examples
/// ```no_run
/// use wgpu_playground_core::surface::get_current_texture;
/// # async fn example(surface: &wgpu::Surface<'_>) -> Result<(), wgpu::SurfaceError> {
/// let texture = get_current_texture(surface)?;
/// // Use the texture for rendering
/// texture.present();
/// # Ok(())
/// # }
/// ```
pub fn get_current_texture(surface: &Surface) -> Result<SurfaceTexture, wgpu::SurfaceError> {
    surface.get_current_texture()
}

/// Get the capabilities of a surface for a given adapter
///
/// # Arguments
/// * `surface` - The surface to query
/// * `adapter` - The GPU adapter
///
/// # Returns
/// The surface capabilities including supported formats, present modes, and alpha modes
///
/// # Examples
/// ```no_run
/// use wgpu_playground_core::surface::get_surface_capabilities;
/// # async fn example(surface: &wgpu::Surface<'_>, adapter: &wgpu::Adapter) {
/// let capabilities = get_surface_capabilities(surface, adapter);
/// println!("Supported formats: {:?}", capabilities.formats);
/// println!("Supported present modes: {:?}", capabilities.present_modes);
/// # }
/// ```
pub fn get_surface_capabilities(
    surface: &Surface,
    adapter: &wgpu::Adapter,
) -> wgpu::SurfaceCapabilities {
    surface.get_capabilities(adapter)
}

/// Helper function to select a preferred surface format
///
/// This function attempts to select an sRGB format if available,
/// otherwise falls back to the first available format.
///
/// # Arguments
/// * `capabilities` - The surface capabilities
///
/// # Returns
/// The preferred texture format
///
/// # Panics
/// Panics if no formats are available in the capabilities (which should never happen
/// for a valid surface)
///
/// # Examples
/// ```no_run
/// use wgpu_playground_core::surface::{get_surface_capabilities, select_preferred_format};
/// # async fn example(surface: &wgpu::Surface<'_>, adapter: &wgpu::Adapter) {
/// let capabilities = get_surface_capabilities(surface, adapter);
/// let format = select_preferred_format(&capabilities);
/// # }
/// ```
pub fn select_preferred_format(capabilities: &wgpu::SurfaceCapabilities) -> TextureFormat {
    assert!(
        !capabilities.formats.is_empty(),
        "Surface capabilities must have at least one format"
    );
    capabilities
        .formats
        .iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(capabilities.formats[0])
}

/// Helper function to select a preferred present mode
///
/// This function attempts to select the Mailbox present mode (if available) for lower latency,
/// otherwise falls back to the first available mode (typically Fifo).
///
/// # Arguments
/// * `capabilities` - The surface capabilities
///
/// # Returns
/// The preferred present mode
///
/// # Panics
/// Panics if no present modes are available in the capabilities (which should never happen
/// for a valid surface)
///
/// # Examples
/// ```no_run
/// use wgpu_playground_core::surface::{get_surface_capabilities, select_preferred_present_mode};
/// # async fn example(surface: &wgpu::Surface<'_>, adapter: &wgpu::Adapter) {
/// let capabilities = get_surface_capabilities(surface, adapter);
/// let present_mode = select_preferred_present_mode(&capabilities);
/// # }
/// ```
pub fn select_preferred_present_mode(capabilities: &wgpu::SurfaceCapabilities) -> PresentMode {
    assert!(
        !capabilities.present_modes.is_empty(),
        "Surface capabilities must have at least one present mode"
    );
    if capabilities.present_modes.contains(&PresentMode::Mailbox) {
        PresentMode::Mailbox
    } else {
        capabilities.present_modes[0]
    }
}

/// Create a surface from a window
///
/// # Arguments
/// * `instance` - The wgpu instance
/// * `window` - The window to create the surface for
///
/// # Returns
/// A new surface, or an error if surface creation failed
///
/// # Examples
/// ```no_run
/// use wgpu_playground_core::surface::create_surface;
/// # async fn example(instance: &wgpu::Instance, window: std::sync::Arc<winit::window::Window>) -> Result<(), wgpu::CreateSurfaceError> {
/// let surface = create_surface(instance, window)?;
/// # Ok(())
/// # }
/// ```
pub fn create_surface<W>(
    instance: &Instance,
    window: W,
) -> Result<Surface<'static>, wgpu::CreateSurfaceError>
where
    W: Into<wgpu::SurfaceTarget<'static>>,
{
    instance.create_surface(window)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface_configuration_builder_default() {
        let builder = SurfaceConfigurationBuilder::new(800, 600);
        let config = builder.build();

        assert_eq!(config.width, 800);
        assert_eq!(config.height, 600);
        assert_eq!(config.format, TextureFormat::Bgra8Unorm);
        assert_eq!(config.usage, TextureUsages::RENDER_ATTACHMENT);
        assert_eq!(config.present_mode, PresentMode::Fifo);
        assert_eq!(config.alpha_mode, CompositeAlphaMode::Opaque);
        assert_eq!(config.desired_maximum_frame_latency, 2);
    }

    #[test]
    fn test_surface_configuration_builder_with_format() {
        let builder =
            SurfaceConfigurationBuilder::new(800, 600).with_format(TextureFormat::Rgba8Unorm);
        let config = builder.build();

        assert_eq!(config.format, TextureFormat::Rgba8Unorm);
    }

    #[test]
    fn test_surface_configuration_builder_with_usage() {
        let usage = TextureUsages::RENDER_ATTACHMENT | TextureUsages::COPY_SRC;
        let builder = SurfaceConfigurationBuilder::new(800, 600).with_usage(usage);
        let config = builder.build();

        assert_eq!(config.usage, usage);
    }

    #[test]
    fn test_surface_configuration_builder_with_present_mode() {
        let builder =
            SurfaceConfigurationBuilder::new(800, 600).with_present_mode(PresentMode::Immediate);
        let config = builder.build();

        assert_eq!(config.present_mode, PresentMode::Immediate);
    }

    #[test]
    fn test_surface_configuration_builder_with_alpha_mode() {
        let builder = SurfaceConfigurationBuilder::new(800, 600)
            .with_alpha_mode(CompositeAlphaMode::PreMultiplied);
        let config = builder.build();

        assert_eq!(config.alpha_mode, CompositeAlphaMode::PreMultiplied);
    }

    #[test]
    fn test_surface_configuration_builder_with_view_formats() {
        let formats = vec![TextureFormat::Bgra8Unorm, TextureFormat::Bgra8UnormSrgb];
        let builder = SurfaceConfigurationBuilder::new(800, 600).with_view_formats(&formats);
        let config = builder.build();

        assert_eq!(config.view_formats, formats);
    }

    #[test]
    fn test_surface_configuration_builder_with_desired_maximum_frame_latency() {
        let builder =
            SurfaceConfigurationBuilder::new(800, 600).with_desired_maximum_frame_latency(3);
        let config = builder.build();

        assert_eq!(config.desired_maximum_frame_latency, 3);
    }

    #[test]
    fn test_surface_configuration_builder_with_size() {
        let builder = SurfaceConfigurationBuilder::new(800, 600).with_size(1024, 768);
        let config = builder.build();

        assert_eq!(config.width, 1024);
        assert_eq!(config.height, 768);
    }

    #[test]
    fn test_surface_configuration_builder_chaining() {
        let builder = SurfaceConfigurationBuilder::new(800, 600)
            .with_format(TextureFormat::Rgba16Float)
            .with_usage(TextureUsages::RENDER_ATTACHMENT | TextureUsages::COPY_SRC)
            .with_present_mode(PresentMode::Mailbox)
            .with_alpha_mode(CompositeAlphaMode::PostMultiplied)
            .with_desired_maximum_frame_latency(1);

        let config = builder.build();

        assert_eq!(config.width, 800);
        assert_eq!(config.height, 600);
        assert_eq!(config.format, TextureFormat::Rgba16Float);
        assert_eq!(
            config.usage,
            TextureUsages::RENDER_ATTACHMENT | TextureUsages::COPY_SRC
        );
        assert_eq!(config.present_mode, PresentMode::Mailbox);
        assert_eq!(config.alpha_mode, CompositeAlphaMode::PostMultiplied);
        assert_eq!(config.desired_maximum_frame_latency, 1);
    }

    #[test]
    fn test_select_preferred_format_with_srgb() {
        let capabilities = wgpu::SurfaceCapabilities {
            formats: vec![
                TextureFormat::Bgra8Unorm,
                TextureFormat::Rgba8UnormSrgb,
                TextureFormat::Rgba8Unorm,
            ],
            present_modes: vec![PresentMode::Fifo],
            alpha_modes: vec![CompositeAlphaMode::Opaque],
            usages: TextureUsages::RENDER_ATTACHMENT,
        };

        let format = select_preferred_format(&capabilities);
        assert_eq!(format, TextureFormat::Rgba8UnormSrgb);
    }

    #[test]
    fn test_select_preferred_format_without_srgb() {
        let capabilities = wgpu::SurfaceCapabilities {
            formats: vec![TextureFormat::Bgra8Unorm, TextureFormat::Rgba8Unorm],
            present_modes: vec![PresentMode::Fifo],
            alpha_modes: vec![CompositeAlphaMode::Opaque],
            usages: TextureUsages::RENDER_ATTACHMENT,
        };

        let format = select_preferred_format(&capabilities);
        assert_eq!(format, TextureFormat::Bgra8Unorm);
    }

    #[test]
    fn test_select_preferred_present_mode_with_mailbox() {
        let capabilities = wgpu::SurfaceCapabilities {
            formats: vec![TextureFormat::Bgra8Unorm],
            present_modes: vec![
                PresentMode::Fifo,
                PresentMode::Mailbox,
                PresentMode::Immediate,
            ],
            alpha_modes: vec![CompositeAlphaMode::Opaque],
            usages: TextureUsages::RENDER_ATTACHMENT,
        };

        let present_mode = select_preferred_present_mode(&capabilities);
        assert_eq!(present_mode, PresentMode::Mailbox);
    }

    #[test]
    fn test_select_preferred_present_mode_without_mailbox() {
        let capabilities = wgpu::SurfaceCapabilities {
            formats: vec![TextureFormat::Bgra8Unorm],
            present_modes: vec![PresentMode::Fifo, PresentMode::Immediate],
            alpha_modes: vec![CompositeAlphaMode::Opaque],
            usages: TextureUsages::RENDER_ATTACHMENT,
        };

        let present_mode = select_preferred_present_mode(&capabilities);
        assert_eq!(present_mode, PresentMode::Fifo);
    }
}
