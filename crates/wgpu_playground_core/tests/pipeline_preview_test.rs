mod common;

use common::create_test_device;
use wgpu_playground_core::pipeline_preview::RenderPipelinePreviewState;
use wgpu_playground_core::render_pipeline::{
    BlendState, DepthStencilState, MultisampleState, PrimitiveState,
};

#[test]
fn test_pipeline_preview_initialization() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let mut preview = RenderPipelinePreviewState::new();
        preview.initialize(&device);

        assert_eq!(preview.size(), (256, 256));
    });
}

#[test]
fn test_pipeline_preview_update_and_render() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let mut preview = RenderPipelinePreviewState::new();
        preview.initialize(&device);

        let primitive = PrimitiveState::default();
        let depth_stencil = DepthStencilState::new(wgpu::TextureFormat::Depth24Plus);
        let blend = BlendState::alpha_blending();
        let multisample = MultisampleState::default();

        preview.update_pipeline(
            &device,
            &primitive,
            Some(&depth_stencil),
            Some(&blend),
            &multisample,
        );

        let result = preview.render(&device, &queue, 0.016);
        assert!(result.is_some());
    });
}
