mod common;

use common::create_test_device;
use wgpu_playground_core::buffer::BufferUsages;
use wgpu_playground_core::buffer_preview::BufferPreviewState;

#[test]
fn test_buffer_preview_initialization() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let mut preview = BufferPreviewState::new();
        preview.initialize(&device);

        // Verify preview was initialized
        let (width, height) = preview.size();
        assert_eq!(width, 256);
        assert_eq!(height, 256);
    });
}

#[test]
fn test_buffer_preview_vertex_rendering() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let mut preview = BufferPreviewState::new();
        preview.initialize(&device);

        // Render with vertex buffer usage
        let usage = BufferUsages::VERTEX;
        let result = preview.render(&device, &queue, usage, 0.016);

        // Verify render was successful
        assert!(result.is_some());
    });
}

#[test]
fn test_buffer_preview_uniform_rendering() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let mut preview = BufferPreviewState::new();
        preview.initialize(&device);

        // Render with uniform buffer usage
        let usage = BufferUsages::UNIFORM;
        let result = preview.render(&device, &queue, usage, 0.016);

        // Verify render was successful
        assert!(result.is_some());
    });
}

#[test]
fn test_buffer_preview_default() {
    let preview = BufferPreviewState::default();
    let (width, height) = preview.size();
    assert_eq!(width, 256);
    assert_eq!(height, 256);
}

#[test]
fn test_buffer_preview_animation_time() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let mut preview = BufferPreviewState::new();
        preview.initialize(&device);

        let usage = BufferUsages::UNIFORM;
        
        // Render multiple frames to test animation
        for _ in 0..10 {
            let result = preview.render(&device, &queue, usage, 0.016);
            assert!(result.is_some());
        }
    });
}
