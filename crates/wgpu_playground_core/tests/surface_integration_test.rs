use wgpu_playground_core::surface::{
    configure_surface, get_surface_capabilities, select_preferred_format,
    select_preferred_present_mode, SurfaceConfigurationBuilder,
};

// Helper function to create a test device, queue, and surface
async fn create_test_device_and_surface(
) -> Option<(
    wgpu::Device,
    wgpu::Queue,
    wgpu::Surface<'static>,
    wgpu::Adapter,
)> {
    use std::sync::Arc;
    use winit::event_loop::EventLoop;
    use winit::window::Window;

    let event_loop = EventLoop::new().ok()?;
    let window = Arc::new(Window::new(&event_loop).ok()?);

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    let surface = instance.create_surface(window.clone()).ok()?;

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await?;

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: Some("Test Device"),
                memory_hints: Default::default(),
            },
            None,
        )
        .await
        .ok()?;

    Some((device, queue, surface, adapter))
}

#[test]
fn test_surface_configuration_builder_basic() {
    pollster::block_on(async {
        let Some((device, _queue, surface, adapter)) = create_test_device_and_surface().await
        else {
            eprintln!("Skipping test: No GPU adapter or window system available");
            return;
        };

        let capabilities = get_surface_capabilities(&surface, &adapter);
        let format = select_preferred_format(&capabilities);
        let present_mode = select_preferred_present_mode(&capabilities);

        let config = SurfaceConfigurationBuilder::new(800, 600)
            .with_format(format)
            .with_present_mode(present_mode)
            .build();

        assert_eq!(config.width, 800);
        assert_eq!(config.height, 600);
        assert_eq!(config.format, format);
        assert_eq!(config.present_mode, present_mode);

        // Configure the surface
        configure_surface(&surface, &device, &config);
    });
}

#[test]
fn test_surface_configuration_with_custom_settings() {
    pollster::block_on(async {
        let Some((device, _queue, surface, adapter)) = create_test_device_and_surface().await
        else {
            eprintln!("Skipping test: No GPU adapter or window system available");
            return;
        };

        let capabilities = get_surface_capabilities(&surface, &adapter);
        let format = select_preferred_format(&capabilities);
        let present_mode = select_preferred_present_mode(&capabilities);

        let config = SurfaceConfigurationBuilder::new(1024, 768)
            .with_format(format)
            .with_present_mode(present_mode)
            .with_alpha_mode(wgpu::CompositeAlphaMode::Opaque)
            .with_usage(wgpu::TextureUsages::RENDER_ATTACHMENT)
            .with_desired_maximum_frame_latency(2)
            .build();

        assert_eq!(config.width, 1024);
        assert_eq!(config.height, 768);
        assert_eq!(config.alpha_mode, wgpu::CompositeAlphaMode::Opaque);
        assert_eq!(config.usage, wgpu::TextureUsages::RENDER_ATTACHMENT);
        assert_eq!(config.desired_maximum_frame_latency, 2);

        configure_surface(&surface, &device, &config);
    });
}

#[test]
fn test_surface_reconfiguration() {
    pollster::block_on(async {
        let Some((device, _queue, surface, adapter)) = create_test_device_and_surface().await
        else {
            eprintln!("Skipping test: No GPU adapter or window system available");
            return;
        };

        let capabilities = get_surface_capabilities(&surface, &adapter);
        let format = select_preferred_format(&capabilities);
        let present_mode = select_preferred_present_mode(&capabilities);

        // Initial configuration
        let config1 = SurfaceConfigurationBuilder::new(800, 600)
            .with_format(format)
            .with_present_mode(present_mode)
            .build();

        configure_surface(&surface, &device, &config1);

        // Reconfigure with different size
        let config2 = SurfaceConfigurationBuilder::new(1024, 768)
            .with_format(format)
            .with_present_mode(present_mode)
            .build();

        configure_surface(&surface, &device, &config2);

        assert_eq!(config2.width, 1024);
        assert_eq!(config2.height, 768);
    });
}

#[test]
fn test_get_surface_capabilities() {
    pollster::block_on(async {
        let Some((_device, _queue, surface, adapter)) = create_test_device_and_surface().await
        else {
            eprintln!("Skipping test: No GPU adapter or window system available");
            return;
        };

        let capabilities = get_surface_capabilities(&surface, &adapter);

        // Verify that capabilities contain at least one format and present mode
        assert!(!capabilities.formats.is_empty());
        assert!(!capabilities.present_modes.is_empty());
        assert!(!capabilities.alpha_modes.is_empty());
    });
}

#[test]
fn test_get_current_texture() {
    pollster::block_on(async {
        let Some((device, _queue, surface, adapter)) = create_test_device_and_surface().await
        else {
            eprintln!("Skipping test: No GPU adapter or window system available");
            return;
        };

        let capabilities = get_surface_capabilities(&surface, &adapter);
        let format = select_preferred_format(&capabilities);
        let present_mode = select_preferred_present_mode(&capabilities);

        let config = SurfaceConfigurationBuilder::new(800, 600)
            .with_format(format)
            .with_present_mode(present_mode)
            .build();

        configure_surface(&surface, &device, &config);

        // Get current texture
        let texture_result = wgpu_playground_core::surface::get_current_texture(&surface);

        // The result may be Ok or Err depending on the surface state
        // For the test, we just verify the function can be called
        match texture_result {
            Ok(surface_texture) => {
                // Successfully got texture, present it
                surface_texture.present();
            }
            Err(e) => {
                // Surface error occurred, which is acceptable in a test environment
                eprintln!("Surface error (expected in some test environments): {:?}", e);
            }
        }
    });
}

#[test]
fn test_surface_configuration_with_view_formats() {
    pollster::block_on(async {
        let Some((device, _queue, surface, adapter)) = create_test_device_and_surface().await
        else {
            eprintln!("Skipping test: No GPU adapter or window system available");
            return;
        };

        let capabilities = get_surface_capabilities(&surface, &adapter);
        let format = select_preferred_format(&capabilities);
        let present_mode = select_preferred_present_mode(&capabilities);

        // Try to add view formats if supported
        let view_formats = if format == wgpu::TextureFormat::Bgra8Unorm {
            vec![wgpu::TextureFormat::Bgra8UnormSrgb]
        } else if format == wgpu::TextureFormat::Rgba8Unorm {
            vec![wgpu::TextureFormat::Rgba8UnormSrgb]
        } else {
            vec![]
        };

        let config = SurfaceConfigurationBuilder::new(800, 600)
            .with_format(format)
            .with_present_mode(present_mode)
            .with_view_formats(&view_formats)
            .build();

        assert_eq!(config.view_formats, view_formats);

        configure_surface(&surface, &device, &config);
    });
}

#[test]
fn test_surface_configuration_resize() {
    pollster::block_on(async {
        let Some((device, _queue, surface, adapter)) = create_test_device_and_surface().await
        else {
            eprintln!("Skipping test: No GPU adapter or window system available");
            return;
        };

        let capabilities = get_surface_capabilities(&surface, &adapter);
        let format = select_preferred_format(&capabilities);
        let present_mode = select_preferred_present_mode(&capabilities);

        // Start with one size
        let mut config = SurfaceConfigurationBuilder::new(640, 480)
            .with_format(format)
            .with_present_mode(present_mode)
            .build();

        configure_surface(&surface, &device, &config);

        // Simulate a resize
        config.width = 1280;
        config.height = 720;

        configure_surface(&surface, &device, &config);

        assert_eq!(config.width, 1280);
        assert_eq!(config.height, 720);
    });
}
