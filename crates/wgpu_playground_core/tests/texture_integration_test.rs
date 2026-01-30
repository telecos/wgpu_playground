use wgpu_playground_core::texture::{
    create_default_view, create_texture_2d, TextureBuilder, TextureViewBuilder,
};

// Helper function to create a test device and queue
async fn create_test_device() -> Option<(wgpu::Device, wgpu::Queue)> {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await?;

    adapter
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
        .ok()
}

#[test]
fn test_texture_builder_default_creation() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let texture = TextureBuilder::new().build(&device);

        // Verify texture was created
        assert_eq!(texture.width(), 1);
        assert_eq!(texture.height(), 1);
        assert_eq!(texture.depth_or_array_layers(), 1);
        assert_eq!(texture.mip_level_count(), 1);
        assert_eq!(texture.sample_count(), 1);
        assert_eq!(texture.dimension(), wgpu::TextureDimension::D2);
        assert_eq!(texture.format(), wgpu::TextureFormat::Rgba8Unorm);
    });
}

#[test]
fn test_texture_2d_creation() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let texture = TextureBuilder::texture_2d(256, 256)
            .with_format(wgpu::TextureFormat::Rgba8Unorm)
            .with_usage(wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST)
            .build(&device);

        assert_eq!(texture.width(), 256);
        assert_eq!(texture.height(), 256);
        assert_eq!(texture.depth_or_array_layers(), 1);
        assert_eq!(texture.dimension(), wgpu::TextureDimension::D2);
    });
}

#[test]
fn test_texture_1d_creation() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let texture = TextureBuilder::texture_1d(256)
            .with_format(wgpu::TextureFormat::Rgba8Unorm)
            .with_usage(wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST)
            .build(&device);

        assert_eq!(texture.width(), 256);
        assert_eq!(texture.height(), 1);
        assert_eq!(texture.depth_or_array_layers(), 1);
        assert_eq!(texture.dimension(), wgpu::TextureDimension::D1);
    });
}

#[test]
fn test_texture_3d_creation() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let texture = TextureBuilder::texture_3d(64, 64, 64)
            .with_format(wgpu::TextureFormat::Rgba8Unorm)
            .with_usage(wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST)
            .build(&device);

        assert_eq!(texture.width(), 64);
        assert_eq!(texture.height(), 64);
        assert_eq!(texture.depth_or_array_layers(), 64);
        assert_eq!(texture.dimension(), wgpu::TextureDimension::D3);
    });
}

#[test]
fn test_texture_cube_creation() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let texture = TextureBuilder::texture_cube(256)
            .with_format(wgpu::TextureFormat::Rgba8Unorm)
            .with_usage(wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST)
            .build(&device);

        assert_eq!(texture.width(), 256);
        assert_eq!(texture.height(), 256);
        assert_eq!(texture.depth_or_array_layers(), 6); // 6 faces
        assert_eq!(texture.dimension(), wgpu::TextureDimension::D2);
    });
}

#[test]
fn test_texture_2d_array_creation() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let texture = TextureBuilder::texture_2d_array(128, 128, 4)
            .with_format(wgpu::TextureFormat::Rgba8Unorm)
            .with_usage(wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST)
            .build(&device);

        assert_eq!(texture.width(), 128);
        assert_eq!(texture.height(), 128);
        assert_eq!(texture.depth_or_array_layers(), 4);
        assert_eq!(texture.dimension(), wgpu::TextureDimension::D2);
    });
}

#[test]
fn test_texture_with_mip_levels() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let texture = TextureBuilder::texture_2d(256, 256)
            .with_mip_levels(5)
            .with_format(wgpu::TextureFormat::Rgba8Unorm)
            .with_usage(wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST)
            .build(&device);

        assert_eq!(texture.mip_level_count(), 5);
    });
}

#[test]
fn test_texture_with_different_formats() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Test various texture formats
        let formats = vec![
            wgpu::TextureFormat::Rgba8Unorm,
            wgpu::TextureFormat::Bgra8Unorm,
            wgpu::TextureFormat::Rgba16Float,
            wgpu::TextureFormat::Rgba32Float,
            wgpu::TextureFormat::R8Unorm,
            wgpu::TextureFormat::Rg8Unorm,
        ];

        for format in formats {
            let texture = TextureBuilder::texture_2d(64, 64)
                .with_format(format)
                .with_usage(wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST)
                .build(&device);

            assert_eq!(texture.format(), format);
        }
    });
}

#[test]
fn test_texture_with_usage_flags() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Test various usage combinations
        let usages = vec![
            wgpu::TextureUsages::TEXTURE_BINDING,
            wgpu::TextureUsages::COPY_DST,
            wgpu::TextureUsages::COPY_SRC,
            wgpu::TextureUsages::RENDER_ATTACHMENT,
            wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        ];

        for usage in usages {
            let _texture = TextureBuilder::texture_2d(64, 64)
                .with_format(wgpu::TextureFormat::Rgba8Unorm)
                .with_usage(usage)
                .build(&device);

            // Just verify it doesn't panic
        }
    });
}

#[test]
fn test_texture_with_label() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let _texture = TextureBuilder::texture_2d(64, 64)
            .with_label("test_texture")
            .with_format(wgpu::TextureFormat::Rgba8Unorm)
            .with_usage(wgpu::TextureUsages::TEXTURE_BINDING)
            .build(&device);

        // Label is set internally, no way to verify from outside
        // This test just ensures it doesn't panic
    });
}

#[test]
fn test_texture_view_default_creation() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let texture = TextureBuilder::texture_2d(256, 256)
            .with_format(wgpu::TextureFormat::Rgba8Unorm)
            .with_usage(wgpu::TextureUsages::TEXTURE_BINDING)
            .build(&device);

        let _view = TextureViewBuilder::new().build(&texture);

        // View creation successful - no panic
    });
}

#[test]
fn test_texture_view_with_mip_level_range() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let texture = TextureBuilder::texture_2d(256, 256)
            .with_mip_levels(5)
            .with_format(wgpu::TextureFormat::Rgba8Unorm)
            .with_usage(wgpu::TextureUsages::TEXTURE_BINDING)
            .build(&device);

        // Create a view of the first 3 mip levels
        let _view = TextureViewBuilder::new()
            .with_mip_level_range(0, 3)
            .build(&texture);

        // Create a view of the last 2 mip levels
        let _view2 = TextureViewBuilder::new()
            .with_mip_level_range(3, 2)
            .build(&texture);

        // View creation successful - no panic
    });
}

#[test]
fn test_texture_view_with_array_layer_range() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let texture = TextureBuilder::texture_2d_array(128, 128, 8)
            .with_format(wgpu::TextureFormat::Rgba8Unorm)
            .with_usage(wgpu::TextureUsages::TEXTURE_BINDING)
            .build(&device);

        // Create a view of the first 4 layers
        let _view = TextureViewBuilder::new()
            .with_array_layer_range(0, 4)
            .build(&texture);

        // Create a view of layers 4-7
        let _view2 = TextureViewBuilder::new()
            .with_array_layer_range(4, 4)
            .build(&texture);

        // View creation successful - no panic
    });
}

#[test]
fn test_texture_view_depth_only() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let texture = TextureBuilder::texture_2d(256, 256)
            .with_format(wgpu::TextureFormat::Depth24PlusStencil8)
            .with_usage(wgpu::TextureUsages::RENDER_ATTACHMENT)
            .build(&device);

        // Create a depth-only view
        let _view = TextureViewBuilder::depth_only().build(&texture);

        // View creation successful - no panic
    });
}

#[test]
fn test_texture_view_stencil_only() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let texture = TextureBuilder::texture_2d(256, 256)
            .with_format(wgpu::TextureFormat::Depth24PlusStencil8)
            .with_usage(wgpu::TextureUsages::RENDER_ATTACHMENT)
            .build(&device);

        // Create a stencil-only view
        let _view = TextureViewBuilder::stencil_only().build(&texture);

        // View creation successful - no panic
    });
}

#[test]
fn test_texture_view_as_cube() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let texture = TextureBuilder::texture_cube(256)
            .with_format(wgpu::TextureFormat::Rgba8Unorm)
            .with_usage(wgpu::TextureUsages::TEXTURE_BINDING)
            .build(&device);

        // Create a cube view
        let _view = TextureViewBuilder::as_cube().build(&texture);

        // View creation successful - no panic
    });
}

#[test]
fn test_texture_view_as_2d_array() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let texture = TextureBuilder::texture_2d_array(128, 128, 8)
            .with_format(wgpu::TextureFormat::Rgba8Unorm)
            .with_usage(wgpu::TextureUsages::TEXTURE_BINDING)
            .build(&device);

        // Create a 2D array view
        let _view = TextureViewBuilder::as_2d_array(8).build(&texture);

        // View creation successful - no panic
    });
}

#[test]
fn test_create_texture_2d_helper() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let texture = create_texture_2d(
            &device,
            256,
            256,
            wgpu::TextureFormat::Rgba8Unorm,
            wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        );

        assert_eq!(texture.width(), 256);
        assert_eq!(texture.height(), 256);
        assert_eq!(texture.format(), wgpu::TextureFormat::Rgba8Unorm);
    });
}

#[test]
fn test_create_default_view_helper() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let texture = TextureBuilder::texture_2d(256, 256)
            .with_format(wgpu::TextureFormat::Rgba8Unorm)
            .with_usage(wgpu::TextureUsages::TEXTURE_BINDING)
            .build(&device);

        let _view = create_default_view(&texture);

        // View creation successful - no panic
    });
}

#[test]
fn test_texture_with_view_formats() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let view_formats = vec![
            wgpu::TextureFormat::Rgba8Unorm,
            wgpu::TextureFormat::Rgba8UnormSrgb,
        ];

        let texture = TextureBuilder::texture_2d(256, 256)
            .with_format(wgpu::TextureFormat::Rgba8Unorm)
            .with_view_formats(&view_formats)
            .with_usage(wgpu::TextureUsages::TEXTURE_BINDING)
            .build(&device);

        // Create a view with sRGB format
        let _view = TextureViewBuilder::new()
            .with_format(wgpu::TextureFormat::Rgba8UnormSrgb)
            .build(&texture);

        // View creation successful - no panic
    });
}

#[test]
fn test_texture_multisampled() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let texture = TextureBuilder::texture_2d(256, 256)
            .with_format(wgpu::TextureFormat::Rgba8Unorm)
            .with_sample_count(4)
            .with_usage(wgpu::TextureUsages::RENDER_ATTACHMENT)
            .build(&device);

        assert_eq!(texture.sample_count(), 4);
    });
}

#[test]
fn test_texture_depth_format() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let texture = TextureBuilder::texture_2d(256, 256)
            .with_format(wgpu::TextureFormat::Depth32Float)
            .with_usage(wgpu::TextureUsages::RENDER_ATTACHMENT)
            .build(&device);

        assert_eq!(texture.format(), wgpu::TextureFormat::Depth32Float);
    });
}

#[test]
fn test_texture_storage_format() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let texture = TextureBuilder::texture_2d(256, 256)
            .with_format(wgpu::TextureFormat::Rgba8Unorm)
            .with_usage(wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING)
            .build(&device);

        assert_eq!(texture.format(), wgpu::TextureFormat::Rgba8Unorm);
    });
}

#[test]
fn test_texture_render_attachment() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let texture = TextureBuilder::texture_2d(256, 256)
            .with_format(wgpu::TextureFormat::Rgba8Unorm)
            .with_usage(wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC)
            .build(&device);

        assert_eq!(texture.format(), wgpu::TextureFormat::Rgba8Unorm);
    });
}

#[test]
fn test_texture_copy_operations() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let texture = TextureBuilder::texture_2d(256, 256)
            .with_format(wgpu::TextureFormat::Rgba8Unorm)
            .with_usage(
                wgpu::TextureUsages::COPY_SRC
                    | wgpu::TextureUsages::COPY_DST
                    | wgpu::TextureUsages::TEXTURE_BINDING,
            )
            .build(&device);

        assert_eq!(texture.width(), 256);
        assert_eq!(texture.height(), 256);
    });
}

#[test]
fn test_complex_texture_builder_chain() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let texture = TextureBuilder::new()
            .with_size(512, 512, 1)
            .with_format(wgpu::TextureFormat::Rgba16Float)
            .with_dimension(wgpu::TextureDimension::D2)
            .with_mip_levels(6)
            .with_sample_count(1)
            .with_label("complex_texture")
            .with_usage(
                wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            )
            .build(&device);

        assert_eq!(texture.width(), 512);
        assert_eq!(texture.height(), 512);
        assert_eq!(texture.format(), wgpu::TextureFormat::Rgba16Float);
        assert_eq!(texture.mip_level_count(), 6);
        assert_eq!(texture.sample_count(), 1);
    });
}
