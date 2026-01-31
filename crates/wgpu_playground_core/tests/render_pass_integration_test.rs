mod common;

use common::create_test_device;
use wgpu_playground_core::render_pass_encoder::{
    Color, DepthStencilOps, IndexFormat, LoadOp, RenderPassColorAttachment,
    RenderPassDepthStencilAttachment, RenderPassDescriptor, RenderPassEncoder, RenderPassError,
    StoreOp,
};

#[test]
fn test_color_attachment_creation() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

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
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let attachment = RenderPassColorAttachment::clear(&view, Color::BLACK);
        assert_eq!(attachment.load_op, LoadOp::Clear(Color::BLACK));
        assert_eq!(attachment.store_op, StoreOp::Store);
        assert!(attachment.resolve_target.is_none());

        let attachment = RenderPassColorAttachment::load(&view);
        assert_eq!(attachment.load_op, LoadOp::Load);
        assert_eq!(attachment.store_op, StoreOp::Store);
    });
}

#[test]
fn test_color_attachment_with_resolve() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

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
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let resolve_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let attachment = RenderPassColorAttachment::clear(&view, Color::RED)
            .with_resolve_target(&resolve_view)
            .with_store_op(StoreOp::Discard);

        assert!(attachment.resolve_target.is_some());
        assert_eq!(attachment.store_op, StoreOp::Discard);
    });
}

#[test]
fn test_depth_stencil_attachment_creation() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: 256,
                height: 256,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let attachment =
            RenderPassDepthStencilAttachment::new(&view, DepthStencilOps::clear_depth(1.0));

        assert_eq!(attachment.ops.depth_load_op, LoadOp::Clear(1.0));
        assert_eq!(attachment.ops.depth_store_op, StoreOp::Store);
    });
}

#[test]
fn test_render_pass_descriptor_validation() {
    let descriptor = RenderPassDescriptor::new();
    assert!(descriptor.validate().is_err());

    if let Err(e) = descriptor.validate() {
        match e {
            RenderPassError::InvalidConfiguration(_) => {}
            _ => panic!("Expected InvalidConfiguration error"),
        }
    }
}

#[test]
fn test_render_pass_descriptor_with_color_attachment() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

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
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let descriptor = RenderPassDescriptor::new()
            .with_label("Test Render Pass")
            .with_color_attachment(RenderPassColorAttachment::clear(&view, Color::BLACK));

        assert_eq!(descriptor.label, Some("Test Render Pass"));
        assert_eq!(descriptor.color_attachments.len(), 1);
        assert!(descriptor.validate().is_ok());
    });
}

#[test]
fn test_render_pass_encoder_creation() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

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
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let descriptor = RenderPassDescriptor::new()
            .with_label("Test Render Pass")
            .with_color_attachment(RenderPassColorAttachment::clear(&view, Color::BLACK));

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Test Encoder"),
        });

        let result = RenderPassEncoder::begin(&mut encoder, &descriptor);
        assert!(result.is_ok());
    });
}

#[test]
fn test_render_pass_encoder_with_depth() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let color_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Color Texture"),
            size: wgpu::Extent3d {
                width: 256,
                height: 256,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: 256,
                height: 256,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let color_view = color_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let descriptor = RenderPassDescriptor::new()
            .with_label("Test Render Pass with Depth")
            .with_color_attachment(RenderPassColorAttachment::clear(&color_view, Color::BLACK))
            .with_depth_stencil_attachment(RenderPassDepthStencilAttachment::new(
                &depth_view,
                DepthStencilOps::clear_depth(1.0),
            ));

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Test Encoder"),
        });

        let result = RenderPassEncoder::begin(&mut encoder, &descriptor);
        assert!(result.is_ok());
    });
}

#[test]
fn test_render_pass_draw_commands() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

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
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let descriptor = RenderPassDescriptor::new()
            .with_label("Test Render Pass")
            .with_color_attachment(RenderPassColorAttachment::clear(&view, Color::BLACK));

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Test Encoder"),
        });

        {
            let mut render_pass = RenderPassEncoder::begin(&mut encoder, &descriptor).unwrap();

            // Test draw command (without pipeline - just checking API)
            render_pass.draw(0..3, 0..1);
        }

        // Submit the command buffer
        queue.submit(std::iter::once(encoder.finish()));
    });
}

#[test]
fn test_render_pass_indexed_draw() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

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
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let descriptor = RenderPassDescriptor::new()
            .with_label("Test Render Pass")
            .with_color_attachment(RenderPassColorAttachment::clear(&view, Color::GREEN));

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Test Encoder"),
        });

        {
            let mut render_pass = RenderPassEncoder::begin(&mut encoder, &descriptor).unwrap();

            // Test indexed draw command
            render_pass.draw_indexed(0..6, 0, 0..1);
        }

        queue.submit(std::iter::once(encoder.finish()));
    });
}

#[test]
fn test_render_pass_indirect_draw() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

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
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create an indirect buffer
        let indirect_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Indirect Buffer"),
            size: 16, // Size for DrawIndirect struct
            usage: wgpu::BufferUsages::INDIRECT | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let descriptor = RenderPassDescriptor::new()
            .with_label("Test Render Pass")
            .with_color_attachment(RenderPassColorAttachment::clear(&view, Color::BLUE));

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Test Encoder"),
        });

        {
            let mut render_pass = RenderPassEncoder::begin(&mut encoder, &descriptor).unwrap();

            // Test indirect draw command
            render_pass.draw_indirect(&indirect_buffer, 0);
        }

        queue.submit(std::iter::once(encoder.finish()));
    });
}

#[test]
fn test_render_pass_indexed_indirect_draw() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

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
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create an indirect buffer
        let indirect_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Indexed Indirect Buffer"),
            size: 20, // Size for DrawIndexedIndirect struct
            usage: wgpu::BufferUsages::INDIRECT | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let descriptor = RenderPassDescriptor::new()
            .with_label("Test Render Pass")
            .with_color_attachment(RenderPassColorAttachment::clear(&view, Color::WHITE));

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Test Encoder"),
        });

        {
            let mut render_pass = RenderPassEncoder::begin(&mut encoder, &descriptor).unwrap();

            // Test indexed indirect draw command
            render_pass.draw_indexed_indirect(&indirect_buffer, 0);
        }

        queue.submit(std::iter::once(encoder.finish()));
    });
}

#[test]
fn test_index_format() {
    let uint16 = IndexFormat::Uint16;
    let uint32 = IndexFormat::Uint32;

    let wgpu_uint16 = uint16.to_wgpu();
    let wgpu_uint32 = uint32.to_wgpu();

    assert_eq!(wgpu_uint16, wgpu::IndexFormat::Uint16);
    assert_eq!(wgpu_uint32, wgpu::IndexFormat::Uint32);
}

#[test]
fn test_render_pass_viewport_and_scissor() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

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
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let descriptor = RenderPassDescriptor::new()
            .with_color_attachment(RenderPassColorAttachment::clear(&view, Color::BLACK));

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Test Encoder"),
        });

        {
            let mut render_pass = RenderPassEncoder::begin(&mut encoder, &descriptor).unwrap();

            // Test viewport
            render_pass.set_viewport(0.0, 0.0, 256.0, 256.0, 0.0, 1.0);

            // Test scissor rect
            render_pass.set_scissor_rect(0, 0, 256, 256);
        }

        queue.submit(std::iter::once(encoder.finish()));
    });
}

#[test]
fn test_render_pass_set_vertex_buffer() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

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
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create a vertex buffer
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: 256,
            usage: wgpu::BufferUsages::VERTEX,
            mapped_at_creation: false,
        });

        let descriptor = RenderPassDescriptor::new()
            .with_color_attachment(RenderPassColorAttachment::clear(&view, Color::BLACK));

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Test Encoder"),
        });

        {
            let mut render_pass = RenderPassEncoder::begin(&mut encoder, &descriptor).unwrap();

            // Test set_vertex_buffer
            render_pass.set_vertex_buffer(0, &vertex_buffer, 0, None);
        }

        queue.submit(std::iter::once(encoder.finish()));
    });
}

#[test]
fn test_render_pass_set_index_buffer() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

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
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create an index buffer
        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Index Buffer"),
            size: 256,
            usage: wgpu::BufferUsages::INDEX,
            mapped_at_creation: false,
        });

        let descriptor = RenderPassDescriptor::new()
            .with_color_attachment(RenderPassColorAttachment::clear(&view, Color::BLACK));

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Test Encoder"),
        });

        {
            let mut render_pass = RenderPassEncoder::begin(&mut encoder, &descriptor).unwrap();

            // Test set_index_buffer
            render_pass.set_index_buffer(&index_buffer, IndexFormat::Uint32, 0, None);
        }

        queue.submit(std::iter::once(encoder.finish()));
    });
}

#[test]
fn test_render_pass_set_bind_group() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

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
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create a simple bind group
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Test Bind Group Layout"),
            entries: &[],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Test Bind Group"),
            layout: &bind_group_layout,
            entries: &[],
        });

        let descriptor = RenderPassDescriptor::new()
            .with_color_attachment(RenderPassColorAttachment::clear(&view, Color::BLACK));

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Test Encoder"),
        });

        {
            let mut render_pass = RenderPassEncoder::begin(&mut encoder, &descriptor).unwrap();

            // Test set_bind_group
            render_pass.set_bind_group(0, &bind_group, &[]);
        }

        queue.submit(std::iter::once(encoder.finish()));
    });
}

#[test]
fn test_render_pass_stencil_and_blend() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

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
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let descriptor = RenderPassDescriptor::new()
            .with_color_attachment(RenderPassColorAttachment::clear(&view, Color::BLACK));

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Test Encoder"),
        });

        {
            let mut render_pass = RenderPassEncoder::begin(&mut encoder, &descriptor).unwrap();

            // Test set_stencil_reference
            render_pass.set_stencil_reference(0xFF);

            // Test set_blend_constant
            render_pass.set_blend_constant(Color::WHITE);
        }

        queue.submit(std::iter::once(encoder.finish()));
    });
}

#[test]
fn test_render_pass_set_pipeline() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

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
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create a simple shader module
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Test Shader"),
            source: wgpu::ShaderSource::Wgsl(
                r#"
                @vertex
                fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
                    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
                }

                @fragment
                fn fs_main() -> @location(0) vec4<f32> {
                    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
                }
                "#.into()
            ),
        });

        // Create a render pipeline
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Test Pipeline"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let descriptor = RenderPassDescriptor::new()
            .with_color_attachment(RenderPassColorAttachment::clear(&view, Color::BLACK));

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Test Encoder"),
        });

        {
            let mut render_pass = RenderPassEncoder::begin(&mut encoder, &descriptor).unwrap();

            // Test set_pipeline
            render_pass.set_pipeline(&pipeline);
        }

        queue.submit(std::iter::once(encoder.finish()));
    });
}
