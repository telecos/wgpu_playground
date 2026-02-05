//! Backend cross-validation testing
//!
//! Simple integration test demonstrating how to compare wgpu-rs rendering
//! against reference images, with extensibility for Dawn when available.

mod common;

use common::create_test_device;
use wgpu::util::DeviceExt;
use wgpu_playground_core::visual_regression::{capture_texture, compare_with_reference, ComparisonConfig};
use wgpu_playground_core::visual_regression::test_utils::create_test_render_target;

#[repr(C)]
#[derive(Copy, Clone)]
struct SimpleVertex {
    pos: [f32; 2],
    col: [f32; 3],
}

unsafe impl bytemuck::Pod for SimpleVertex {}
unsafe impl bytemuck::Zeroable for SimpleVertex {}

const VERTEX_SHADER: &str = r#"
struct VIn {
    @location(0) pos: vec2<f32>,
    @location(1) col: vec3<f32>,
}

struct VOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) col: vec3<f32>,
}

@vertex
fn vs_main(input: VIn) -> VOut {
    var output: VOut;
    output.pos = vec4<f32>(input.pos, 0.0, 1.0);
    output.col = input.col;
    return output;
}

@fragment
fn fs_main(input: VOut) -> @location(0) vec4<f32> {
    return vec4<f32>(input.col, 1.0);
}
"#;

fn draw_colored_triangle(dev: &wgpu::Device, q: &wgpu::Queue) -> wgpu::Texture {
    let target = create_test_render_target(dev, 256, 256);

    let verts = vec![
        SimpleVertex { pos: [0.0, 0.5], col: [1.0, 0.0, 0.0] },
        SimpleVertex { pos: [-0.5, -0.5], col: [0.0, 1.0, 0.0] },
        SimpleVertex { pos: [0.5, -0.5], col: [0.0, 0.0, 1.0] },
    ];

    let vbuf = dev.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Tri Verts"),
        contents: bytemuck::cast_slice(&verts),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let shader = dev.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Tri Shader"),
        source: wgpu::ShaderSource::Wgsl(VERTEX_SHADER.into()),
    });

    let pipeline = dev.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Tri Pipeline"),
        layout: None,
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<SimpleVertex>() as u64,
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
        multiview: None,
        cache: None,
    });

    let view = target.create_view(&wgpu::TextureViewDescriptor::default());
    let mut enc = dev.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Tri Encoder"),
    });

    {
        let mut rpass = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Tri Pass"),
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

        rpass.set_pipeline(&pipeline);
        rpass.set_vertex_buffer(0, vbuf.slice(..));
        rpass.draw(0..3, 0..1);
    }

    q.submit(Some(enc.finish()));
    target
}

#[test]
fn backend_cross_check_triangle() {
    pollster::block_on(async {
        let Some((dev, q)) = create_test_device().await else {
            eprintln!("Skipping: no adapter");
            return;
        };

        let tex = draw_colored_triangle(&dev, &q);
        let img = capture_texture(&dev, &q, &tex).await.unwrap();

        let cfg = ComparisonConfig::default();
        match compare_with_reference(&img, "xcheck_triangle", cfg) {
            Ok(cmp) => {
                println!("✓ Cross-check passed (Δ={:.4}%)", cmp.difference * 100.0);
                assert!(cmp.is_match);
            }
            Err(e) => {
                if e.to_string().contains("not found") {
                    eprintln!("Note: {}", e);
                    eprintln!("Set UPDATE_VISUAL_REFERENCES=1 to create baseline");
                } else {
                    panic!("Failed: {}", e);
                }
            }
        }
    });
}
