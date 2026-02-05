/// Render Bundle Example
///
/// This example demonstrates render bundles for command reuse:
/// - Creating a RenderBundleEncoder
/// - Recording reusable draw commands in a bundle
/// - Executing bundles in a render pass with executeBundles
/// - Performance benefits when re-executing the same commands multiple times
///
/// Render bundles allow you to record a sequence of draw commands once and then
/// replay them efficiently across multiple frames or render passes. This reduces
/// CPU overhead and can significantly improve performance for scenes with
/// repeated rendering patterns.
///
/// Run with: cargo run --package wgpu_playground_examples --example render_bundle
use wgpu::util::DeviceExt;
use wgpu_playground_core::shader::ShaderModule;

/// Vertex structure for triangles
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
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await
        .ok()?;

    println!("Using adapter: {}", adapter.get_info().name);
    println!("Backend: {:?}", adapter.get_info().backend);

    adapter
        .request_device(&wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            label: Some("Render Bundle Device"),
            memory_hints: Default::default(),
            experimental_features: Default::default(),
            trace: Default::default(),
        })
        .await
        .ok()
}

/// Create vertices for multiple triangles to demonstrate bundle reuse
fn create_triangle_vertices(offset_x: f32, offset_y: f32, color: [f32; 3]) -> Vec<Vertex> {
    let size = 0.2;
    vec![
        Vertex::new([offset_x, offset_y + size], color),              // Top
        Vertex::new([offset_x - size, offset_y - size], color),       // Bottom-left
        Vertex::new([offset_x + size, offset_y - size], color),       // Bottom-right
    ]
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

    println!("=== Render Bundle Example ===\n");

    // Create device and queue
    let device_queue = pollster::block_on(create_device());
    if device_queue.is_none() {
        eprintln!("Failed to create GPU device");
        return;
    }

    let (device, queue) = device_queue.unwrap();
    println!("✓ GPU device created\n");

    // Create multiple triangle batches to demonstrate render bundle reuse
    println!("Creating triangle geometry:");
    let mut all_vertices = Vec::new();
    
    // First batch - Red triangles
    for i in 0..3 {
        let x = -0.6 + (i as f32 * 0.3);
        let y = 0.5;
        all_vertices.extend_from_slice(&create_triangle_vertices(x, y, [1.0, 0.0, 0.0]));
    }
    
    // Second batch - Green triangles
    for i in 0..3 {
        let x = -0.6 + (i as f32 * 0.3);
        let y = 0.0;
        all_vertices.extend_from_slice(&create_triangle_vertices(x, y, [0.0, 1.0, 0.0]));
    }
    
    // Third batch - Blue triangles
    for i in 0..3 {
        let x = -0.6 + (i as f32 * 0.3);
        let y = -0.5;
        all_vertices.extend_from_slice(&create_triangle_vertices(x, y, [0.0, 0.0, 1.0]));
    }

    println!("  {} triangles ({} vertices)", all_vertices.len() / 3, all_vertices.len());
    println!();

    // Create vertex buffer
    let vertex_data = bytemuck::cast_slice(&all_vertices);
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: vertex_data,
        usage: wgpu::BufferUsages::VERTEX,
    });
    println!("✓ Vertex buffer created ({} bytes)", vertex_data.len());

    // Load and compile shader
    let shader = ShaderModule::from_file("render_bundle.wgsl", Some("render_bundle_shader"))
        .expect("Failed to load render bundle shader");
    println!("✓ Shader loaded and compiled");

    // Create shader module
    let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("render_bundle_shader"),
        source: wgpu::ShaderSource::Wgsl(shader.source().into()),
    });

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
        label: Some("Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    // Create render pipeline
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("render_bundle_pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader_module,
            entry_point: Some("vs_main"),
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
            entry_point: Some("fs_main"),
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
    println!("✓ Render pipeline created\n");

    // ========================================
    // RENDER BUNDLE: Record draw commands once
    // ========================================
    println!("=== Creating Render Bundle ===");
    
    let mut render_bundle_encoder =
        device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
            label: Some("Render Bundle Encoder"),
            color_formats: &[Some(wgpu::TextureFormat::Rgba8UnormSrgb)],
            depth_stencil: None,
            sample_count: 1,
            multiview: None,
        });

    // Record draw commands into the bundle
    render_bundle_encoder.set_pipeline(&pipeline);
    render_bundle_encoder.set_vertex_buffer(0, vertex_buffer.slice(..));
    render_bundle_encoder.draw(0..all_vertices.len() as u32, 0..1);
    
    // Finish the bundle - this creates a reusable command buffer
    let render_bundle = render_bundle_encoder.finish(&wgpu::RenderBundleDescriptor {
        label: Some("Render Bundle"),
    });
    
    println!("✓ Render bundle created and recorded");
    println!("  - Pipeline configured");
    println!("  - Vertex buffer bound");
    println!("  - Draw commands recorded");
    println!("  - Bundle can now be reused across multiple render passes\n");

    // Create render texture
    let render_texture = create_render_texture(&device);
    let texture_view = render_texture.create_view(&wgpu::TextureViewDescriptor::default());
    println!("✓ Render target created (800x600)\n");

    // ========================================
    // Demonstrate bundle execution multiple times
    // ========================================
    println!("=== Executing Render Bundle ===");
    
    // We'll execute the bundle 3 times to simulate multiple frames/passes
    // In a real application, this would be done across multiple frames
    for pass_num in 1..=3 {
        println!("\nRender Pass {}:", pass_num);
        
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some(&format!("Render Encoder {}", pass_num)),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some(&format!("Render Pass {}", pass_num)),
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
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // Execute the pre-recorded bundle!
            // This is much more efficient than re-recording all draw commands
            render_pass.execute_bundles(std::iter::once(&render_bundle));
            
            println!("  ✓ Bundle executed via execute_bundles()");
        }

        queue.submit(std::iter::once(encoder.finish()));
        println!("  ✓ Commands submitted to GPU");
    }

    // Wait for completion
    let _ = device.poll(wgpu::PollType::Wait {
        submission_index: None,
        timeout: None,
    });
    println!("\n✓ All render passes complete\n");

    println!("=== Render Bundle Example Complete ===");
    println!("\nSuccessfully demonstrated render bundles with:");
    println!("  • RenderBundleEncoder - created and configured");
    println!("  • Bundle recording - draw commands recorded once");
    println!("  • execute_bundles() - efficient command replay");
    println!("  • Multiple executions - bundle reused 3 times");
    println!("\nKey WebGPU APIs exercised:");
    println!("  • device.create_render_bundle_encoder()");
    println!("  • RenderBundleEncoder::set_pipeline()");
    println!("  • RenderBundleEncoder::set_vertex_buffer()");
    println!("  • RenderBundleEncoder::draw()");
    println!("  • RenderBundleEncoder::finish()");
    println!("  • RenderPass::execute_bundles()");
    println!("\nPerformance Benefits:");
    println!("  • Draw commands recorded once, executed multiple times");
    println!("  • Reduced CPU overhead per frame");
    println!("  • GPU driver can optimize bundle execution");
    println!("  • Ideal for static or repeated geometry");
    println!("\nUse cases:");
    println!("  • UI rendering (same elements each frame)");
    println!("  • Particle effects (repeated patterns)");
    println!("  • Instanced objects (static draw calls)");
    println!("  • Shadow map generation (same geometry, different view)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_size() {
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
        let vertices = create_triangle_vertices(0.0, 0.0, [1.0, 0.0, 0.0]);
        let bytes = bytemuck::cast_slice::<Vertex, u8>(&vertices);
        assert_eq!(bytes.len(), 3 * std::mem::size_of::<Vertex>());
    }

    #[test]
    fn test_create_triangle_vertices() {
        let vertices = create_triangle_vertices(0.5, 0.5, [1.0, 0.0, 0.0]);
        assert_eq!(vertices.len(), 3);
        
        // All vertices should have the same color
        for v in &vertices {
            assert_eq!(v.color, [1.0, 0.0, 0.0]);
        }
    }

    #[tokio::test]
    async fn test_device_creation() {
        let result = create_device().await;
        match result {
            Some(_) => println!("Device created successfully"),
            None => println!("No GPU available (expected in CI)"),
        }
    }
}
