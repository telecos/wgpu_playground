mod common;

use common::create_test_device;
use wgpu_playground_core::texture_preview::TexturePreviewState;

#[test]
fn test_texture_preview_initialization() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let mut preview = TexturePreviewState::new();
        preview.initialize(&device);

        // Verify preview state was initialized
        let (width, height) = preview.size();
        assert_eq!(width, 256);
        assert_eq!(height, 256);
        assert!(!preview.has_texture()); // No texture loaded yet
    });
}

#[test]
fn test_texture_preview_procedural_generation() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let mut preview = TexturePreviewState::new();
        preview.initialize(&device);

        // Generate procedural texture
        preview.generate_procedural_texture(&device, &queue, 256, 256);

        assert!(preview.has_texture());
    });
}

#[test]
fn test_texture_preview_image_loading() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let mut preview = TexturePreviewState::new();
        preview.initialize(&device);

        // Create simple test image data (2x2 red texture)
        let image_data: Vec<u8> = vec![
            255, 0, 0, 255, // Red pixel
            255, 0, 0, 255, // Red pixel
            255, 0, 0, 255, // Red pixel
            255, 0, 0, 255, // Red pixel
        ];

        preview.update_from_image_data(&device, &queue, &image_data, 2, 2);

        assert!(preview.has_texture());
    });
}

#[test]
fn test_texture_preview_render() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let mut preview = TexturePreviewState::new();
        preview.initialize(&device);

        // Generate procedural texture
        preview.generate_procedural_texture(&device, &queue, 128, 128);

        // Render preview
        let result = preview.render(&device, &queue);

        // Verify render returned a texture view
        assert!(result.is_some());
    });
}

#[test]
fn test_texture_preview_different_sizes() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let sizes = vec![(64, 64), (128, 128), (256, 256), (512, 512)];

        for (width, height) in sizes {
            let mut preview = TexturePreviewState::new();
            preview.initialize(&device);
            preview.generate_procedural_texture(&device, &queue, width, height);

            assert!(preview.has_texture());
        }
    });
}
