/// Multisampling (MSAA) Anti-Aliasing Example
///
/// This example demonstrates Multi-Sample Anti-Aliasing (MSAA) for smooth edge rendering:
/// - Creating MSAA render targets with sample_count > 1
/// - Resolve operations to transfer MSAA results to regular textures
/// - Comparison between non-MSAA (aliased) and MSAA (anti-aliased) rendering
///
/// The example renders a rotating triangle twice:
/// 1. Without MSAA (sample_count = 1) - shows jagged edges
/// 2. With 4x MSAA (sample_count = 4) - shows smooth edges
///
/// Run with: cargo run --package wgpu_playground_examples --example multisampling
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
                label: Some("Multisampling Device"),
                memory_hints: Default::default(),
            },
            None,
        )
        .await
        .ok()
}

/// Create a regular (non-MSAA) render texture
fn create_render_texture(
    device: &wgpu::Device,
    width: u32,
    height: u32,
    label: &str,
) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size: wgpu::Extent3d {
            width,
            height,
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

/// Create a multisampled (MSAA) texture for anti-aliasing
fn create_multisampled_texture(
    device: &wgpu::Device,
    width: u32,
    height: u32,
    sample_count: u32,
    label: &str,
) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count, // Key difference: sample_count > 1 enables MSAA
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    })
}

/// Create a render pipeline with specified sample count
fn create_pipeline(
    device: &wgpu::Device,
    shader_module: &wgpu::ShaderModule,
    sample_count: u32,
    label: &str,
) -> wgpu::RenderPipeline {
    // Define vertex buffer layout
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

    // Create pipeline layout
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(&format!("{} Layout", label)),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    // Create render pipeline
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(label),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: shader_module,
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
            count: sample_count, // Configure MSAA sample count
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        fragment: Some(wgpu::FragmentState {
            module: shader_module,
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
    })
}

fn main() {
    env_logger::init();

    println!("=== Multisampling (MSAA) Anti-Aliasing Example ===\n");

    // Create device and queue
    let device_queue = pollster::block_on(create_device());
    if device_queue.is_none() {
        eprintln!("Failed to create GPU device");
        return;
    }

    let (device, queue) = device_queue.unwrap();
    println!("✓ GPU device created\n");

    // Define triangle vertices - rotated for better edge visibility
    // Using a thin triangle to emphasize aliasing artifacts
    let vertices = vec![
        Vertex::new([0.0, 0.7], [1.0, 0.3, 0.3]),   // Top - Red
        Vertex::new([-0.3, -0.5], [0.3, 1.0, 0.3]), // Bottom-left - Green
        Vertex::new([0.3, -0.5], [0.3, 0.3, 1.0]),  // Bottom-right - Blue
    ];

    println!("Triangle geometry:");
    println!("  {} vertices forming a thin triangle", vertices.len());
    println!("  Rotated to emphasize edge aliasing\n");

    // Create vertex buffer
    let vertex_data = bytemuck::cast_slice(&vertices);
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Triangle Vertex Buffer"),
        contents: vertex_data,
        usage: wgpu::BufferUsages::VERTEX,
    });
    println!("✓ Vertex buffer created ({} bytes)", vertex_data.len());

    // Load and compile shader
    let shader = ShaderModule::from_file("multisampling.wgsl", Some("multisampling_shader"))
        .expect("Failed to load multisampling shader");
    println!("✓ Shader loaded and compiled\n");

    // Create shader module
    let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("multisampling_shader"),
        source: wgpu::ShaderSource::Wgsl(shader.source().into()),
    });

    // Render dimensions
    let width = 800;
    let height = 600;

    // === Part 1: Render WITHOUT multisampling (aliased) ===
    println!("--- Rendering WITHOUT MSAA (sample_count = 1) ---");

    let no_msaa_pipeline = create_pipeline(&device, &shader_module, 1, "No MSAA Pipeline");
    println!("✓ Non-MSAA pipeline created (sample_count = 1)");

    let no_msaa_texture = create_render_texture(&device, width, height, "No MSAA Render Texture");
    let no_msaa_view = no_msaa_texture.create_view(&wgpu::TextureViewDescriptor::default());
    println!("✓ Non-MSAA render target created ({}x{})", width, height);

    // Render without MSAA
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("No MSAA Encoder"),
    });

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("No MSAA Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &no_msaa_view,
                resolve_target: None, // No resolve needed for non-MSAA
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.05,
                        g: 0.05,
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

        render_pass.set_pipeline(&no_msaa_pipeline);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw(0..3, 0..1);
    }

    queue.submit(std::iter::once(encoder.finish()));
    device.poll(wgpu::Maintain::Wait);
    println!("✓ Non-MSAA rendering complete\n");

    // === Part 2: Render WITH 4x multisampling (anti-aliased) ===
    println!("--- Rendering WITH 4x MSAA (sample_count = 4) ---");

    let msaa_sample_count = 4;
    let msaa_pipeline =
        create_pipeline(&device, &shader_module, msaa_sample_count, "MSAA Pipeline");
    println!(
        "✓ MSAA pipeline created (sample_count = {})",
        msaa_sample_count
    );

    // Create MSAA texture (multisampled)
    let msaa_texture = create_multisampled_texture(
        &device,
        width,
        height,
        msaa_sample_count,
        "MSAA Render Texture",
    );
    let msaa_view = msaa_texture.create_view(&wgpu::TextureViewDescriptor::default());
    println!(
        "✓ MSAA render target created ({}x{}, {} samples)",
        width, height, msaa_sample_count
    );

    // Create resolve texture (regular texture to receive resolved MSAA result)
    let resolve_texture = create_render_texture(&device, width, height, "Resolve Texture");
    let resolve_view = resolve_texture.create_view(&wgpu::TextureViewDescriptor::default());
    println!("✓ Resolve target created (for MSAA resolution)");

    // Render with MSAA
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("MSAA Encoder"),
    });

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("MSAA Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &msaa_view,                    // Render to MSAA texture
                resolve_target: Some(&resolve_view), // Resolve to regular texture
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.05,
                        g: 0.05,
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

        render_pass.set_pipeline(&msaa_pipeline);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw(0..3, 0..1);
    }

    queue.submit(std::iter::once(encoder.finish()));
    device.poll(wgpu::Maintain::Wait);
    println!("✓ MSAA rendering complete");
    println!("✓ Resolve operation complete (MSAA → regular texture)\n");

    // Summary
    println!("=== Multisampling Example Complete ===");
    println!("\nThis example demonstrated:");
    println!("  • Non-MSAA rendering (sample_count = 1)");
    println!("    - Renders directly to a regular texture");
    println!("    - Shows jagged edges (aliasing artifacts)");
    println!();
    println!(
        "  • 4x MSAA rendering (sample_count = {}):",
        msaa_sample_count
    );
    println!("    - Renders to a multisampled texture");
    println!("    - Uses resolve_target for automatic resolve operation");
    println!("    - Produces smooth edges (anti-aliasing)");
    println!();
    println!("Key MSAA concepts:");
    println!("  1. Multisampled texture: sample_count > 1");
    println!("  2. Render pipeline: multisample.count must match texture");
    println!("  3. Resolve operation: transfers MSAA texture → regular texture");
    println!("  4. The GPU automatically averages multiple samples per pixel");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_size() {
        // Verify vertex structure has correct size
        // 2 floats (position) + 3 floats (color) = 5 floats × 4 bytes = 20 bytes
        assert_eq!(std::mem::size_of::<Vertex>(), 20);
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
            Vertex::new([0.0, 0.7], [1.0, 0.3, 0.3]),
            Vertex::new([-0.3, -0.5], [0.3, 1.0, 0.3]),
            Vertex::new([0.3, -0.5], [0.3, 0.3, 1.0]),
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

    #[test]
    fn test_msaa_sample_counts() {
        // Test that common MSAA sample counts are valid powers of 2
        let valid_counts: [u32; 5] = [1, 2, 4, 8, 16];
        for count in valid_counts {
            assert!(count.is_power_of_two(), "MSAA count must be power of 2");
            assert!(count <= 16, "MSAA count should be <= 16 for compatibility");
        }
    }
}
