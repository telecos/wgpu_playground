mod common;

use common::create_test_device;
use wgpu_playground_core::render_bundle_encoder::{RenderBundleDescriptor, RenderBundleEncoderOps};
use wgpu_playground_core::render_pass_encoder::{
    Color, RenderPassColorAttachment, RenderPassDescriptor, RenderPassEncoder,
};

// Helper function to create a test shader module
fn create_test_shader(device: &wgpu::Device) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Test Shader"),
        source: wgpu::ShaderSource::Wgsl(
            r#"
            @vertex
            fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
                let x = f32(i32(in_vertex_index) - 1);
                let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
                return vec4<f32>(x, y, 0.0, 1.0);
            }

            @fragment
            fn fs_main() -> @location(0) vec4<f32> {
                return vec4<f32>(1.0, 0.0, 0.0, 1.0);
            }
            "#
            .into(),
        ),
    })
}

// Helper function to create a test render pipeline
fn create_test_pipeline(
    device: &wgpu::Device,
    shader: &wgpu::ShaderModule,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Test Pipeline"),
        layout: None,
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview_mask: None,
        cache: None,
    })
}

#[test]
fn test_render_bundle_creation() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a render bundle descriptor
        let bundle_descriptor = RenderBundleDescriptor::new()
            .with_label("Test Render Bundle")
            .with_color_format(wgpu::TextureFormat::Bgra8UnormSrgb);

        // Validate the descriptor
        assert!(bundle_descriptor.validate().is_ok());

        // Create render bundle encoder
        let encoder = RenderBundleEncoderOps::new(&device, &bundle_descriptor);
        assert!(encoder.is_ok());
    });
}

#[test]
fn test_render_bundle_descriptor_validation() {
    // Empty descriptor should fail validation
    let descriptor = RenderBundleDescriptor::new();
    assert!(descriptor.validate().is_err());

    // Descriptor with color format should pass
    let descriptor =
        RenderBundleDescriptor::new().with_color_format(wgpu::TextureFormat::Bgra8UnormSrgb);
    assert!(descriptor.validate().is_ok());

    // Descriptor with depth format should pass
    let descriptor =
        RenderBundleDescriptor::new().with_depth_stencil_format(wgpu::TextureFormat::Depth32Float);
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_render_bundle_recording() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let shader = create_test_shader(&device);
        let pipeline = create_test_pipeline(&device, &shader);

        // Create a render bundle
        let bundle_descriptor = RenderBundleDescriptor::new()
            .with_label("Test Render Bundle")
            .with_color_format(wgpu::TextureFormat::Bgra8UnormSrgb);

        let mut encoder = RenderBundleEncoderOps::new(&device, &bundle_descriptor)
            .expect("Failed to create render bundle encoder");

        // Record draw commands
        encoder.set_pipeline(&pipeline);
        encoder.draw(0..3, 0..1);

        // Finish the bundle
        let _bundle = encoder.finish();
    });
}

#[test]
fn test_render_bundle_execution() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let shader = create_test_shader(&device);
        let pipeline = create_test_pipeline(&device, &shader);

        // Create a render bundle
        let bundle_descriptor = RenderBundleDescriptor::new()
            .with_label("Test Render Bundle")
            .with_color_format(wgpu::TextureFormat::Bgra8UnormSrgb);

        let mut bundle_encoder = RenderBundleEncoderOps::new(&device, &bundle_descriptor)
            .expect("Failed to create render bundle encoder");

        bundle_encoder.set_pipeline(&pipeline);
        bundle_encoder.draw(0..3, 0..1);
        let bundle = bundle_encoder.finish();

        // Create a texture to render to
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Test Texture"),
            size: wgpu::Extent3d {
                width: 256,
                height: 256,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create a command encoder and render pass
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Test Encoder"),
        });

        let descriptor = RenderPassDescriptor::new()
            .with_label("Test Render Pass")
            .with_color_attachment(RenderPassColorAttachment::clear(&view, Color::BLACK));

        let mut render_pass = RenderPassEncoder::begin(&mut encoder, &descriptor)
            .expect("Failed to create render pass");

        // Execute the render bundle
        render_pass.execute_bundle(&bundle);

        // Finish the render pass and encoder
        drop(render_pass);
        let command_buffer = encoder.finish();

        // Submit the commands
        queue.submit(std::iter::once(command_buffer));

        // Wait for the GPU to finish
        let _ = device.poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: None,
        });
    });
}

#[test]
fn test_render_bundle_multiple_execution() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let shader = create_test_shader(&device);
        let pipeline = create_test_pipeline(&device, &shader);

        // Create two render bundles
        let bundle_descriptor = RenderBundleDescriptor::new()
            .with_label("Test Render Bundle 1")
            .with_color_format(wgpu::TextureFormat::Bgra8UnormSrgb);

        let mut bundle_encoder1 = RenderBundleEncoderOps::new(&device, &bundle_descriptor)
            .expect("Failed to create render bundle encoder 1");
        bundle_encoder1.set_pipeline(&pipeline);
        bundle_encoder1.draw(0..3, 0..1);
        let bundle1 = bundle_encoder1.finish();

        let bundle_descriptor = RenderBundleDescriptor::new()
            .with_label("Test Render Bundle 2")
            .with_color_format(wgpu::TextureFormat::Bgra8UnormSrgb);

        let mut bundle_encoder2 = RenderBundleEncoderOps::new(&device, &bundle_descriptor)
            .expect("Failed to create render bundle encoder 2");
        bundle_encoder2.set_pipeline(&pipeline);
        bundle_encoder2.draw(0..3, 0..1);
        let bundle2 = bundle_encoder2.finish();

        // Create a texture to render to
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Test Texture"),
            size: wgpu::Extent3d {
                width: 256,
                height: 256,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create a command encoder and render pass
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Test Encoder"),
        });

        let descriptor = RenderPassDescriptor::new()
            .with_label("Test Render Pass")
            .with_color_attachment(RenderPassColorAttachment::clear(&view, Color::BLACK));

        let mut render_pass = RenderPassEncoder::begin(&mut encoder, &descriptor)
            .expect("Failed to create render pass");

        // Execute both render bundles
        render_pass.execute_bundles(&[&bundle1, &bundle2]);

        // Finish the render pass and encoder
        drop(render_pass);
        let command_buffer = encoder.finish();

        // Submit the commands
        queue.submit(std::iter::once(command_buffer));

        // Wait for the GPU to finish
        let _ = device.poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: None,
        });
    });
}
