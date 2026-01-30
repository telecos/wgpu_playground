use wgpu_playground_core::surface::{
    select_preferred_format, select_preferred_present_mode, SurfaceConfigurationBuilder,
};

#[test]
fn test_surface_configuration_builder_basic() {
    let config = SurfaceConfigurationBuilder::new(800, 600)
        .with_format(wgpu::TextureFormat::Bgra8Unorm)
        .with_present_mode(wgpu::PresentMode::Fifo)
        .build();

    assert_eq!(config.width, 800);
    assert_eq!(config.height, 600);
    assert_eq!(config.format, wgpu::TextureFormat::Bgra8Unorm);
    assert_eq!(config.present_mode, wgpu::PresentMode::Fifo);
}

#[test]
fn test_surface_configuration_with_custom_settings() {
    let config = SurfaceConfigurationBuilder::new(1024, 768)
        .with_format(wgpu::TextureFormat::Rgba8Unorm)
        .with_present_mode(wgpu::PresentMode::Mailbox)
        .with_alpha_mode(wgpu::CompositeAlphaMode::Opaque)
        .with_usage(wgpu::TextureUsages::RENDER_ATTACHMENT)
        .with_desired_maximum_frame_latency(2)
        .build();

    assert_eq!(config.width, 1024);
    assert_eq!(config.height, 768);
    assert_eq!(config.format, wgpu::TextureFormat::Rgba8Unorm);
    assert_eq!(config.present_mode, wgpu::PresentMode::Mailbox);
    assert_eq!(config.alpha_mode, wgpu::CompositeAlphaMode::Opaque);
    assert_eq!(config.usage, wgpu::TextureUsages::RENDER_ATTACHMENT);
    assert_eq!(config.desired_maximum_frame_latency, 2);
}

#[test]
fn test_surface_configuration_builder_chaining() {
    let config = SurfaceConfigurationBuilder::new(640, 480)
        .with_format(wgpu::TextureFormat::Rgba16Float)
        .with_present_mode(wgpu::PresentMode::Immediate)
        .with_alpha_mode(wgpu::CompositeAlphaMode::PreMultiplied)
        .with_usage(wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC)
        .with_desired_maximum_frame_latency(1)
        .build();

    assert_eq!(config.width, 640);
    assert_eq!(config.height, 480);
    assert_eq!(config.format, wgpu::TextureFormat::Rgba16Float);
    assert_eq!(config.present_mode, wgpu::PresentMode::Immediate);
    assert_eq!(config.alpha_mode, wgpu::CompositeAlphaMode::PreMultiplied);
    assert_eq!(
        config.usage,
        wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC
    );
    assert_eq!(config.desired_maximum_frame_latency, 1);
}

#[test]
fn test_surface_configuration_with_view_formats() {
    let view_formats = vec![
        wgpu::TextureFormat::Bgra8Unorm,
        wgpu::TextureFormat::Bgra8UnormSrgb,
    ];
    let config = SurfaceConfigurationBuilder::new(800, 600)
        .with_format(wgpu::TextureFormat::Bgra8Unorm)
        .with_present_mode(wgpu::PresentMode::Fifo)
        .with_view_formats(&view_formats)
        .build();

    assert_eq!(config.view_formats, view_formats);
}

#[test]
fn test_surface_configuration_resize() {
    let mut config = SurfaceConfigurationBuilder::new(640, 480)
        .with_format(wgpu::TextureFormat::Bgra8Unorm)
        .with_present_mode(wgpu::PresentMode::Fifo)
        .build();

    assert_eq!(config.width, 640);
    assert_eq!(config.height, 480);

    // Simulate a resize
    config.width = 1280;
    config.height = 720;

    assert_eq!(config.width, 1280);
    assert_eq!(config.height, 720);
}

#[test]
fn test_select_preferred_format_with_srgb() {
    let capabilities = wgpu::SurfaceCapabilities {
        formats: vec![
            wgpu::TextureFormat::Bgra8Unorm,
            wgpu::TextureFormat::Rgba8UnormSrgb,
            wgpu::TextureFormat::Rgba8Unorm,
        ],
        present_modes: vec![wgpu::PresentMode::Fifo],
        alpha_modes: vec![wgpu::CompositeAlphaMode::Opaque],
        usages: wgpu::TextureUsages::RENDER_ATTACHMENT,
    };

    let format = select_preferred_format(&capabilities);
    assert_eq!(format, wgpu::TextureFormat::Rgba8UnormSrgb);
}

#[test]
fn test_select_preferred_format_without_srgb() {
    let capabilities = wgpu::SurfaceCapabilities {
        formats: vec![
            wgpu::TextureFormat::Bgra8Unorm,
            wgpu::TextureFormat::Rgba8Unorm,
        ],
        present_modes: vec![wgpu::PresentMode::Fifo],
        alpha_modes: vec![wgpu::CompositeAlphaMode::Opaque],
        usages: wgpu::TextureUsages::RENDER_ATTACHMENT,
    };

    let format = select_preferred_format(&capabilities);
    assert_eq!(format, wgpu::TextureFormat::Bgra8Unorm);
}

#[test]
fn test_select_preferred_present_mode_with_mailbox() {
    let capabilities = wgpu::SurfaceCapabilities {
        formats: vec![wgpu::TextureFormat::Bgra8Unorm],
        present_modes: vec![
            wgpu::PresentMode::Fifo,
            wgpu::PresentMode::Mailbox,
            wgpu::PresentMode::Immediate,
        ],
        alpha_modes: vec![wgpu::CompositeAlphaMode::Opaque],
        usages: wgpu::TextureUsages::RENDER_ATTACHMENT,
    };

    let present_mode = select_preferred_present_mode(&capabilities);
    assert_eq!(present_mode, wgpu::PresentMode::Mailbox);
}

#[test]
fn test_select_preferred_present_mode_without_mailbox() {
    let capabilities = wgpu::SurfaceCapabilities {
        formats: vec![wgpu::TextureFormat::Bgra8Unorm],
        present_modes: vec![wgpu::PresentMode::Fifo, wgpu::PresentMode::Immediate],
        alpha_modes: vec![wgpu::CompositeAlphaMode::Opaque],
        usages: wgpu::TextureUsages::RENDER_ATTACHMENT,
    };

    let present_mode = select_preferred_present_mode(&capabilities);
    assert_eq!(present_mode, wgpu::PresentMode::Fifo);
}
