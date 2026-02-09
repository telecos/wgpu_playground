//! Visual regression tests for rendering output
//!
//! These tests capture GPU rendering output and compare against reference images
//! to catch visual regressions.

mod common;

use common::create_test_device;
use wgpu::util::DeviceExt;
use wgpu_playground_core::assert_visual_match;
use wgpu_playground_core::visual_regression::test_utils::*;
use wgpu_playground_core::visual_regression::*;

/// Vertex structure for simple rendering tests
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

/// Shader source for visual regression tests
const SIMPLE_SHADER: &str = r#"
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(in.position, 0.0, 1.0);
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
"#;

/// Renders a simple triangle for visual regression testing
fn render_triangle(device: &wgpu::Device, queue: &wgpu::Queue) -> wgpu::Texture {
    let texture = create_test_render_target(device, 256, 256);

    // Create vertices for a centered triangle
    let vertices = vec![
        Vertex {
            position: [0.0, 0.5],
            color: [1.0, 0.0, 0.0], // Red
        },
        Vertex {
            position: [-0.5, -0.5],
            color: [0.0, 1.0, 0.0], // Green
        },
        Vertex {
            position: [0.5, -0.5],
            color: [0.0, 0.0, 1.0], // Blue
        },
    ];

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Triangle Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Simple Shader"),
        source: wgpu::ShaderSource::Wgsl(SIMPLE_SHADER.into()),
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Triangle Pipeline"),
        layout: None,
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Vertex>() as u64,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[
                    wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x2,
                        offset: 0,
                        shader_location: 0,
                    },
                    wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x3,
                        offset: 8,
                        shader_location: 1,
                    },
                ],
            }],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview_mask: None,
        cache: None,
    });

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Triangle Render Encoder"),
    });

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Triangle Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&pipeline);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw(0..3, 0..1);
    }

    queue.submit(Some(encoder.finish()));

    texture
}

/// Renders a simple quad with a solid color
fn render_solid_quad(device: &wgpu::Device, queue: &wgpu::Queue) -> wgpu::Texture {
    let texture = create_test_render_target(device, 256, 256);

    // Create vertices for a quad covering the screen
    let vertices = vec![
        Vertex {
            position: [-1.0, -1.0],
            color: [0.2, 0.3, 0.8], // Blue
        },
        Vertex {
            position: [1.0, -1.0],
            color: [0.2, 0.3, 0.8],
        },
        Vertex {
            position: [1.0, 1.0],
            color: [0.2, 0.3, 0.8],
        },
        Vertex {
            position: [-1.0, -1.0],
            color: [0.2, 0.3, 0.8],
        },
        Vertex {
            position: [1.0, 1.0],
            color: [0.2, 0.3, 0.8],
        },
        Vertex {
            position: [-1.0, 1.0],
            color: [0.2, 0.3, 0.8],
        },
    ];

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Quad Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Simple Shader"),
        source: wgpu::ShaderSource::Wgsl(SIMPLE_SHADER.into()),
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Quad Pipeline"),
        layout: None,
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Vertex>() as u64,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[
                    wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x2,
                        offset: 0,
                        shader_location: 0,
                    },
                    wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x3,
                        offset: 8,
                        shader_location: 1,
                    },
                ],
            }],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview_mask: None,
        cache: None,
    });

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Quad Render Encoder"),
    });

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Quad Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&pipeline);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw(0..6, 0..1);
    }

    queue.submit(Some(encoder.finish()));

    texture
}

#[test]
fn test_visual_regression_triangle() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping visual regression test: No GPU adapter available");
            return;
        };

        let result = run_visual_test(
            "triangle",
            &device,
            &queue,
            render_triangle,
            ComparisonConfig::default(),
        )
        .await;

        match result {
            Ok(comparison) => {
                assert_visual_match!(comparison);
                println!("✓ Triangle visual regression test passed");
                println!("  Difference: {:.4}%", comparison.difference * 100.0);
            }
            Err(VisualRegressionError::ReferenceLoadError(msg)) => {
                eprintln!("Note: {}", msg);
                eprintln!("Run with UPDATE_VISUAL_REFERENCES=1 to create reference image");
            }
            Err(e) => panic!("Visual regression test failed: {}", e),
        }
    });
}

#[test]
fn test_visual_regression_solid_quad() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping visual regression test: No GPU adapter available");
            return;
        };

        let result = run_visual_test(
            "solid_quad",
            &device,
            &queue,
            render_solid_quad,
            ComparisonConfig::default(),
        )
        .await;

        match result {
            Ok(comparison) => {
                assert_visual_match!(comparison);
                println!("✓ Solid quad visual regression test passed");
                println!("  Difference: {:.4}%", comparison.difference * 100.0);
            }
            Err(VisualRegressionError::ReferenceLoadError(msg)) => {
                eprintln!("Note: {}", msg);
                eprintln!("Run with UPDATE_VISUAL_REFERENCES=1 to create reference image");
            }
            Err(e) => panic!("Visual regression test failed: {}", e),
        }
    });
}

#[test]
fn test_capture_texture_basic() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a simple render target
        let texture = create_test_render_target(&device, 64, 64);

        // Capture it
        let image = capture_texture(&device, &queue, &texture).await.unwrap();

        // Verify dimensions
        assert_eq!(image.dimensions(), (64, 64));
        println!("✓ Texture capture test passed");
    });
}
