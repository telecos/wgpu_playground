/// Render-to-Texture Example
///
/// This example demonstrates multi-pass rendering in WebGPU:
/// - **First Pass**: Renders a colorful triangle to an offscreen texture (framebuffer)
/// - **Second Pass**: Uses that texture to display it on a fullscreen quad
///
/// This demonstrates:
/// - Creating a texture suitable for rendering (RENDER_ATTACHMENT usage)
/// - Using a texture as both a render target and a shader resource
/// - Multi-pass rendering workflow
/// - Texture sampling in fragment shaders
/// - Bind groups for texture and sampler resources
///
/// Run with: cargo run --package wgpu_playground_examples --example render_to_texture
use wgpu::util::DeviceExt;
use wgpu_playground_core::shader::ShaderModule;

/// Vertex structure for the first pass (triangle with color)
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct ColorVertex {
    position: [f32; 2],
    color: [f32; 3],
}

unsafe impl bytemuck::Pod for ColorVertex {}
unsafe impl bytemuck::Zeroable for ColorVertex {}

impl ColorVertex {
    fn new(position: [f32; 2], color: [f32; 3]) -> Self {
        Self { position, color }
    }
}

/// Vertex structure for the second pass (quad with texture coordinates)
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct TexturedVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

unsafe impl bytemuck::Pod for TexturedVertex {}
unsafe impl bytemuck::Zeroable for TexturedVertex {}

impl TexturedVertex {
    fn new(position: [f32; 2], tex_coords: [f32; 2]) -> Self {
        Self {
            position,
            tex_coords,
        }
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
            label: Some("Render-to-Texture Device"),
            memory_hints: Default::default(),
            experimental_features: Default::default(),
            trace: Default::default(),
        })
        .await
        .ok()
}

/// Create a texture that can be both rendered to and sampled from
fn create_offscreen_texture(
    device: &wgpu::Device,
    width: u32,
    height: u32,
) -> (wgpu::Texture, wgpu::TextureView) {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Offscreen Render Texture"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        // Important: needs RENDER_ATTACHMENT for rendering to it, and TEXTURE_BINDING for sampling
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    (texture, view)
}

/// Create a texture for the final output
fn create_output_texture(device: &wgpu::Device, width: u32, height: u32) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Final Output Texture"),
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

fn main() {
    env_logger::init();

    println!("=== Render-to-Texture Example ===\n");
    println!("This example demonstrates multi-pass rendering:");
    println!("  Pass 1: Render triangle to offscreen texture");
    println!("  Pass 2: Display texture on fullscreen quad\n");

    // Create device and queue
    let device_queue = pollster::block_on(create_device());
    if device_queue.is_none() {
        eprintln!("Failed to create GPU device");
        return;
    }

    let (device, queue) = device_queue.unwrap();
    println!("✓ GPU device created\n");

    // Dimensions
    let offscreen_width = 512;
    let offscreen_height = 512;
    let output_width = 800;
    let output_height = 600;

    // ===== FIRST PASS SETUP =====
    println!("=== First Pass Setup (Render Triangle to Texture) ===");

    // Create triangle vertices for first pass
    let triangle_vertices = vec![
        ColorVertex::new([0.0, 0.6], [1.0, 0.0, 0.0]), // Top - Red
        ColorVertex::new([-0.5, -0.3], [0.0, 1.0, 0.0]), // Bottom-left - Green
        ColorVertex::new([0.5, -0.3], [0.0, 0.0, 1.0]), // Bottom-right - Blue
    ];

    let triangle_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Triangle Vertex Buffer"),
        contents: bytemuck::cast_slice(&triangle_vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });
    println!(
        "✓ Triangle vertex buffer created ({} bytes)",
        std::mem::size_of_val(&triangle_vertices[..])
    );

    // Load first pass shader
    let scene_shader = ShaderModule::from_file(
        "render_to_texture_scene.wgsl",
        Some("render_to_texture_scene_shader"),
    )
    .expect("Failed to load scene shader");

    let scene_shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("render_to_texture_scene_shader"),
        source: wgpu::ShaderSource::Wgsl(scene_shader.source().into()),
    });
    println!("✓ Scene shader loaded and compiled");

    // Create first pass pipeline
    let scene_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Scene Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let scene_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("scene_pipeline"),
        layout: Some(&scene_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &scene_shader_module,
            entry_point: Some("vs_main"),
            compilation_options: Default::default(),
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<ColorVertex>() as u64,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[
                    wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x2,
                        offset: 0,
                        shader_location: 0,
                    },
                    wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x3,
                        offset: std::mem::size_of::<[f32; 2]>() as u64,
                        shader_location: 1,
                    },
                ],
            }],
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
            module: &scene_shader_module,
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
    println!("✓ Scene render pipeline created");

    // Create offscreen texture (framebuffer)
    let (_offscreen_texture, offscreen_view) =
        create_offscreen_texture(&device, offscreen_width, offscreen_height);
    println!(
        "✓ Offscreen texture created ({}x{})",
        offscreen_width, offscreen_height
    );
    println!("  - Usage: RENDER_ATTACHMENT | TEXTURE_BINDING\n");

    // ===== SECOND PASS SETUP =====
    println!("=== Second Pass Setup (Display Texture on Quad) ===");

    // Create fullscreen quad vertices for second pass
    let quad_vertices = vec![
        // First triangle
        TexturedVertex::new([-1.0, -1.0], [0.0, 1.0]), // Bottom-left
        TexturedVertex::new([1.0, -1.0], [1.0, 1.0]),  // Bottom-right
        TexturedVertex::new([1.0, 1.0], [1.0, 0.0]),   // Top-right
        // Second triangle
        TexturedVertex::new([-1.0, -1.0], [0.0, 1.0]), // Bottom-left
        TexturedVertex::new([1.0, 1.0], [1.0, 0.0]),   // Top-right
        TexturedVertex::new([-1.0, 1.0], [0.0, 0.0]),  // Top-left
    ];

    let quad_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Quad Vertex Buffer"),
        contents: bytemuck::cast_slice(&quad_vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });
    println!(
        "✓ Quad vertex buffer created ({} bytes)",
        std::mem::size_of_val(&quad_vertices[..])
    );

    // Create sampler for texture sampling
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("Texture Sampler"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });
    println!("✓ Texture sampler created");

    // Load second pass shader
    let display_shader = ShaderModule::from_file(
        "render_to_texture_display.wgsl",
        Some("render_to_texture_display_shader"),
    )
    .expect("Failed to load display shader");

    let display_shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("render_to_texture_display_shader"),
        source: wgpu::ShaderSource::Wgsl(display_shader.source().into()),
    });
    println!("✓ Display shader loaded and compiled");

    // Create bind group layout for texture and sampler
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Texture Bind Group Layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
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

    // Create bind group with the offscreen texture
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
                resource: wgpu::BindingResource::TextureView(&offscreen_view),
            },
        ],
    });
    println!("✓ Bind group created (texture + sampler)");

    // Create second pass pipeline
    let display_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Display Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let display_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("display_pipeline"),
        layout: Some(&display_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &display_shader_module,
            entry_point: Some("vs_main"),
            compilation_options: Default::default(),
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<TexturedVertex>() as u64,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[
                    wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x2,
                        offset: 0,
                        shader_location: 0,
                    },
                    wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x2,
                        offset: std::mem::size_of::<[f32; 2]>() as u64,
                        shader_location: 1,
                    },
                ],
            }],
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
            module: &display_shader_module,
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
    println!("✓ Display render pipeline created\n");

    // Create final output texture
    let output_texture = create_output_texture(&device, output_width, output_height);
    let output_view = output_texture.create_view(&wgpu::TextureViewDescriptor::default());
    println!(
        "✓ Final output texture created ({}x{})\n",
        output_width, output_height
    );

    // ===== RENDERING =====
    println!("=== Executing Multi-Pass Rendering ===\n");

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render-to-Texture Encoder"),
    });

    // FIRST PASS: Render triangle to offscreen texture
    {
        println!("Pass 1: Rendering triangle to offscreen texture...");
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("First Pass - Render to Texture"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &offscreen_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.2,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store, // Important: Store the result for next pass
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&scene_pipeline);
        render_pass.set_vertex_buffer(0, triangle_vertex_buffer.slice(..));
        render_pass.draw(0..3, 0..1);
        println!("  ✓ Triangle rendered to offscreen texture");
    }

    // SECOND PASS: Display offscreen texture on quad
    {
        println!("Pass 2: Displaying offscreen texture on quad...");
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Second Pass - Display Texture"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &output_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
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

        render_pass.set_pipeline(&display_pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.set_vertex_buffer(0, quad_vertex_buffer.slice(..));
        render_pass.draw(0..6, 0..1);
        println!("  ✓ Offscreen texture sampled and displayed on quad");
    }

    // Submit commands
    queue.submit(std::iter::once(encoder.finish()));
    println!("\n✓ Render commands submitted to GPU");

    // Wait for completion
    let _ = device.poll(wgpu::PollType::Wait {
        submission_index: None,
        timeout: None,
    });
    println!("✓ Rendering complete\n");

    println!("=== Render-to-Texture Example Complete ===");
    println!("\nThis example successfully demonstrated:");
    println!("  • Creating an offscreen texture as a framebuffer");
    println!("  • First pass: Rendering a triangle to the offscreen texture");
    println!("  • Second pass: Using the texture as input to render a textured quad");
    println!("  • Multi-pass rendering workflow");
    println!("  • Texture sampling with bind groups");
    println!("\nKey concepts:");
    println!("  - Offscreen texture usage: RENDER_ATTACHMENT | TEXTURE_BINDING");
    println!("  - First pass StoreOp::Store preserves rendered content");
    println!("  - Second pass samples the texture via bind group");
    println!("  - Two separate render pipelines for different passes");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_vertex_size() {
        assert_eq!(std::mem::size_of::<ColorVertex>(), 20); // 2*f32 + 3*f32 = 20 bytes
    }

    #[test]
    fn test_textured_vertex_size() {
        assert_eq!(std::mem::size_of::<TexturedVertex>(), 16); // 2*f32 + 2*f32 = 16 bytes
    }

    #[test]
    fn test_color_vertex_creation() {
        let v = ColorVertex::new([1.0, 2.0], [0.5, 0.5, 0.5]);
        assert_eq!(v.position, [1.0, 2.0]);
        assert_eq!(v.color, [0.5, 0.5, 0.5]);
    }

    #[test]
    fn test_textured_vertex_creation() {
        let v = TexturedVertex::new([1.0, 2.0], [0.5, 0.5]);
        assert_eq!(v.position, [1.0, 2.0]);
        assert_eq!(v.tex_coords, [0.5, 0.5]);
    }

    #[test]
    fn test_vertices_are_pod() {
        let color_vertices = vec![ColorVertex::new([0.0, 0.5], [1.0, 0.0, 0.0])];
        let bytes = bytemuck::cast_slice::<ColorVertex, u8>(&color_vertices);
        assert_eq!(bytes.len(), std::mem::size_of::<ColorVertex>());

        let textured_vertices = vec![TexturedVertex::new([0.0, 0.5], [0.0, 0.0])];
        let bytes = bytemuck::cast_slice::<TexturedVertex, u8>(&textured_vertices);
        assert_eq!(bytes.len(), std::mem::size_of::<TexturedVertex>());
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
