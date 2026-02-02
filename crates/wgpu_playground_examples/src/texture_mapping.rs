use bytemuck::{Pod, Zeroable};
/// Example demonstrating texture creation, sampler configuration, and texture binding
///
/// This example shows how to:
/// - Create a texture programmatically with data
/// - Configure a sampler for texture filtering
/// - Set up vertex data with UV coordinates
/// - Create shaders that sample textures
/// - Bind textures and samplers to shaders
/// - Render a textured quad
use wgpu_playground_core::sampler::{AddressMode, FilterMode, SamplerDescriptor};
use wgpu_playground_core::shader::ShaderModule;
use wgpu_playground_core::texture::TextureBuilder;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

impl Vertex {
    #[allow(dead_code)]
    fn new(position: [f32; 2], tex_coords: [f32; 2]) -> Self {
        Self {
            position,
            tex_coords,
        }
    }
}

/// Create a simple checkerboard texture (8x8 pixels)
#[allow(dead_code)]
fn create_checkerboard_texture_data() -> Vec<u8> {
    const SIZE: usize = 8;
    let mut data = Vec::with_capacity(SIZE * SIZE * 4); // RGBA

    for y in 0..SIZE {
        for x in 0..SIZE {
            // Create checkerboard pattern
            let is_white = (x + y) % 2 == 0;
            let color = if is_white {
                [255, 255, 255, 255] // White
            } else {
                [64, 64, 64, 255] // Dark gray
            };
            data.extend_from_slice(&color);
        }
    }

    data
}

#[allow(dead_code)]
async fn run_texture_example() {
    println!("=== Texture Mapping Example ===\n");

    // Step 1: Initialize wgpu context
    println!("1. Initializing wgpu context...");
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await
        .expect("Failed to find an appropriate adapter");

    println!("   ✓ Adapter: {}", adapter.get_info().name);

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("Device"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: Default::default(),
            experimental_features: Default::default(),
            trace: Default::default(),
        })
        .await
        .expect("Failed to create device");

    println!("   ✓ Device and queue created");

    // Step 2: Create a checkerboard texture
    println!("\n2. Creating checkerboard texture...");
    const TEXTURE_SIZE: u32 = 8;

    let texture = TextureBuilder::new()
        .with_size(TEXTURE_SIZE, TEXTURE_SIZE, 1)
        .with_format(wgpu::TextureFormat::Rgba8Unorm)
        .with_usage(wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST)
        .with_label("Checkerboard Texture")
        .build(&device);

    // Upload texture data
    let texture_data = create_checkerboard_texture_data();
    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &texture_data,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4 * TEXTURE_SIZE),
            rows_per_image: Some(TEXTURE_SIZE),
        },
        wgpu::Extent3d {
            width: TEXTURE_SIZE,
            height: TEXTURE_SIZE,
            depth_or_array_layers: 1,
        },
    );

    println!(
        "   ✓ Texture created: {}x{} pixels",
        TEXTURE_SIZE, TEXTURE_SIZE
    );
    println!("   ✓ Format: Rgba8Unorm");
    println!("   ✓ Data uploaded: {} bytes", texture_data.len());

    // Step 3: Create texture view
    println!("\n3. Creating texture view...");
    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    println!("   ✓ Texture view created");

    // Step 4: Create sampler
    println!("\n4. Creating texture sampler...");
    let sampler_descriptor = SamplerDescriptor::new(Some("Texture Sampler"))
        .with_address_mode(AddressMode::Repeat)
        .with_filter(FilterMode::Nearest);

    let sampler = sampler_descriptor
        .create_sampler(&device)
        .expect("Failed to create sampler");

    println!("   ✓ Sampler created");
    println!("   - Address mode: Repeat");
    println!("   - Filter mode: Nearest");

    // Step 5: Load shader
    println!("\n5. Loading textured quad shader...");
    let shader = ShaderModule::from_file("textured_quad.wgsl", Some("Textured Quad Shader"))
        .expect("Failed to load shader");

    let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: shader.label(),
        source: wgpu::ShaderSource::Wgsl(shader.source().into()),
    });

    println!("   ✓ Shader loaded and compiled");

    // Step 6: Create vertex buffer with quad vertices
    println!("\n6. Creating vertex buffer for textured quad...");

    // Define a quad with two triangles
    // Positions range from -0.5 to 0.5, UVs from 0.0 to 2.0 to show texture repeat
    let vertices = [
        // First triangle
        Vertex::new([-0.5, -0.5], [0.0, 2.0]), // Bottom-left
        Vertex::new([0.5, -0.5], [2.0, 2.0]),  // Bottom-right
        Vertex::new([0.5, 0.5], [2.0, 0.0]),   // Top-right
        // Second triangle
        Vertex::new([-0.5, -0.5], [0.0, 2.0]), // Bottom-left
        Vertex::new([0.5, 0.5], [2.0, 0.0]),   // Top-right
        Vertex::new([-0.5, 0.5], [0.0, 0.0]),  // Top-left
    ];

    let vertex_data = bytemuck::cast_slice(&vertices);
    let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Vertex Buffer"),
        size: vertex_data.len() as wgpu::BufferAddress,
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    queue.write_buffer(&vertex_buffer, 0, vertex_data);

    println!("   ✓ Vertex buffer created");
    println!("   - Vertices: {}", vertices.len());
    println!("   - Buffer size: {} bytes", vertex_data.len());

    // Step 7: Create bind group layout
    println!("\n7. Creating bind group layout...");
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Texture Bind Group Layout"),
        entries: &[
            // Sampler
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            // Texture
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
        ],
    });

    println!("   ✓ Bind group layout created");

    // Step 8: Create bind group
    println!("\n8. Creating bind group...");
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Texture Bind Group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&texture_view),
            },
        ],
    });

    println!("   ✓ Bind group created");
    println!("   - Binding 0: Sampler");
    println!("   - Binding 1: Texture");

    // Step 9: Create render pipeline
    println!("\n9. Creating render pipeline...");
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader_module,
            entry_point: Some("vs_main"),
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[
                    // Position
                    wgpu::VertexAttribute {
                        offset: 0,
                        shader_location: 0,
                        format: wgpu::VertexFormat::Float32x2,
                    },
                    // Tex coords
                    wgpu::VertexAttribute {
                        offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                        shader_location: 1,
                        format: wgpu::VertexFormat::Float32x2,
                    },
                ],
            }],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader_module,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba8Unorm,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    });

    println!("   ✓ Render pipeline created");

    // Step 10: Create output texture
    println!("\n10. Creating output texture for rendering...");
    const OUTPUT_SIZE: u32 = 256;

    let output_texture = TextureBuilder::new()
        .with_size(OUTPUT_SIZE, OUTPUT_SIZE, 1)
        .with_format(wgpu::TextureFormat::Rgba8Unorm)
        .with_usage(wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC)
        .with_label("Output Texture")
        .build(&device);

    let output_view = output_texture.create_view(&wgpu::TextureViewDescriptor::default());
    println!(
        "   ✓ Output texture created: {}x{} pixels",
        OUTPUT_SIZE, OUTPUT_SIZE
    );

    // Step 11: Record and submit rendering commands
    println!("\n11. Recording and submitting render commands...");
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &output_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
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

        render_pass.set_pipeline(&render_pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw(0..vertices.len() as u32, 0..1);
    }

    queue.submit(std::iter::once(encoder.finish()));
    println!("   ✓ Render commands submitted");

    // Step 12: Wait for completion
    println!("\n12. Waiting for GPU to complete...");
    device.poll(wgpu::PollType::Wait {
        submission_index: None,
        timeout: None,
    });
    println!("   ✓ Rendering complete!");

    println!("\n=== Example Summary ===");
    println!("This example demonstrated:");
    println!("  ✓ Creating a texture programmatically (8x8 checkerboard)");
    println!("  ✓ Configuring a sampler (Repeat + Nearest filtering)");
    println!("  ✓ Setting up vertex data with UV coordinates");
    println!("  ✓ Loading and compiling a shader with texture sampling");
    println!("  ✓ Binding textures and samplers to shaders via bind groups");
    println!("  ✓ Rendering a textured quad to an offscreen texture");
    println!("\n=== Example Complete ===");
}

#[tokio::main]
#[allow(dead_code)]
async fn main() {
    env_logger::init();
    run_texture_example().await;
}
