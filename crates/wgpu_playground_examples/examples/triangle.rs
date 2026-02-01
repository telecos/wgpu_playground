/// Triangle rendering example
///
/// This example demonstrates the classic "Hello Triangle" with WebGPU:
/// - Creating a vertex buffer with triangle vertices
/// - Loading and compiling WGSL shaders
/// - Setting up a render pipeline
/// - Executing a render pass with draw commands
///
/// Run with: cargo run --package wgpu_playground_examples --example triangle
use wgpu::util::DeviceExt;
use wgpu_playground_core::shader::ShaderModule;

/// Vertex structure for triangle
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

// Safety: Vertex is repr(C) with simple primitive types
unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

impl Vertex {
    fn new(position: [f32; 2], color: [f32; 3]) -> Self {
        Self { position, color }
    }
}

/// Create GPU device and queue
async fn create_device() -> Option<(wgpu::Device, wgpu::Queue)> {
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

    println!("Using adapter: {}", adapter.get_info().name);
    println!("Backend: {:?}", adapter.get_info().backend);

    adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: Some("Triangle Device"),
                memory_hints: Default::default(),
            },
            None,
        )
        .await
        .ok()
}

/// Create a texture for rendering to
fn create_render_texture(device: &wgpu::Device) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Render Texture"),
        size: wgpu::Extent3d {
            width: 800,
            height: 600,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    })
}

fn main() {
    env_logger::init();

    println!("=== Triangle Rendering Example ===\n");

    // Create device and queue
    let device_queue = pollster::block_on(create_device());
    if device_queue.is_none() {
        eprintln!("Failed to create GPU device");
        return;
    }

    let (device, queue) = device_queue.unwrap();
    println!("✓ GPU device created\n");

    // Define triangle vertices (position + color)
    // Triangle with red, green, and blue vertices
    let vertices = vec![
        Vertex::new([0.0, 0.5], [1.0, 0.0, 0.0]),   // Top - Red
        Vertex::new([-0.5, -0.5], [0.0, 1.0, 0.0]), // Bottom-left - Green
        Vertex::new([0.5, -0.5], [0.0, 0.0, 1.0]),  // Bottom-right - Blue
    ];

    println!("Triangle vertices:");
    for (i, v) in vertices.iter().enumerate() {
        println!(
            "  Vertex {}: pos({:.1}, {:.1}), color({:.1}, {:.1}, {:.1})",
            i, v.position[0], v.position[1], v.color[0], v.color[1], v.color[2]
        );
    }
    println!();

    // Create vertex buffer using wgpu directly
    let vertex_data = bytemuck::cast_slice(&vertices);
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Triangle Vertex Buffer"),
        contents: vertex_data,
        usage: wgpu::BufferUsages::VERTEX,
    });
    println!("✓ Vertex buffer created ({} bytes)", vertex_data.len());

    // Load and compile shader
    let shader = ShaderModule::from_file("triangle.wgsl", Some("triangle_shader"))
        .expect("Failed to load triangle shader");
    println!("✓ Shader loaded and compiled\n");

    // Create shader module using wgpu
    let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("triangle_shader"),
        source: wgpu::ShaderSource::Wgsl(shader.source().into()),
    });

    // Define vertex buffer layout using wgpu
    let vertex_buffer_layout = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as u64,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            // Position attribute
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 0,
                shader_location: 0,
            },
            // Color attribute
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: std::mem::size_of::<[f32; 2]>() as u64,
                shader_location: 1,
            },
        ],
    };

    // Create pipeline layout (empty for this simple example)
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Triangle Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    // Create render pipeline using wgpu directly
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("triangle_pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader_module,
            entry_point: "vs_main",
            compilation_options: Default::default(),
            buffers: &[vertex_buffer_layout],
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader_module,
            entry_point: "fs_main",
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview: None,
        cache: None,
    });
    println!("✓ Render pipeline created");

    // Create render texture
    let render_texture = create_render_texture(&device);
    let texture_view = render_texture.create_view(&wgpu::TextureViewDescriptor::default());
    println!("✓ Render target created (800x600)\n");

    // Create command encoder and render pass
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Triangle Render Encoder"),
    });

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Triangle Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.1,
                        b: 0.1,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&pipeline);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw(0..3, 0..1);
        println!("✓ Render pass configured");
        println!("  - Clear color: dark gray (0.1, 0.1, 0.1)");
        println!("  - Drawing 3 vertices (1 triangle)");
    }

    // Submit commands
    queue.submit(std::iter::once(encoder.finish()));
    println!("\n✓ Render commands submitted to GPU");

    // Wait for completion
    device.poll(wgpu::Maintain::Wait);
    println!("✓ Rendering complete\n");

    println!("=== Triangle Example Complete ===");
    println!("\nThe triangle was successfully rendered with:");
    println!("  • Vertex buffer with position and color data");
    println!("  • WGSL shader (vertex + fragment stages)");
    println!("  • Render pipeline with blend state");
    println!("  • Render pass with clear and draw operations");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_size() {
        // Verify vertex structure has correct size
        assert_eq!(std::mem::size_of::<Vertex>(), 20); // 2*f32 + 3*f32 = 20 bytes
    }

    #[test]
    fn test_vertex_creation() {
        let v = Vertex::new([1.0, 2.0], [0.5, 0.5, 0.5]);
        assert_eq!(v.position, [1.0, 2.0]);
        assert_eq!(v.color, [0.5, 0.5, 0.5]);
    }

    #[test]
    fn test_vertices_are_pod() {
        // Test that vertices can be cast to bytes
        let vertices = vec![
            Vertex::new([0.0, 0.5], [1.0, 0.0, 0.0]),
            Vertex::new([-0.5, -0.5], [0.0, 1.0, 0.0]),
            Vertex::new([0.5, -0.5], [0.0, 0.0, 1.0]),
        ];

        let bytes = bytemuck::cast_slice::<Vertex, u8>(&vertices);
        assert_eq!(bytes.len(), 3 * std::mem::size_of::<Vertex>());
    }

    #[tokio::test]
    async fn test_device_creation() {
        // Test that we can create a device
        let result = create_device().await;
        // This may fail in CI environments without GPU, so we just check it returns
        match result {
            Some(_) => println!("Device created successfully"),
            None => println!("No GPU available (expected in CI)"),
        }
    }
}
