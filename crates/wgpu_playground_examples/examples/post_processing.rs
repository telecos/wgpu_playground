/// Post-Processing Effects Example
///
/// This example demonstrates advanced post-processing techniques in WebGPU:
/// - **First Pass**: Renders a colorful triangle to an offscreen texture
/// - **Second Pass**: Applies post-processing effects (blur, grayscale, edge detection)
/// - **Final Pass**: Displays the processed result
///
/// This demonstrates:
/// - Multiple render-to-texture passes (framebuffers)
/// - Multiple render pipelines for different effects
/// - Texture sampling in fragment shaders
/// - Full-screen quad rendering
/// - Common post-processing effects used in real applications
///
/// Run with: cargo run --package wgpu_playground_examples --example post_processing
use wgpu::util::DeviceExt;
use wgpu_playground_core::shader::ShaderModule;

/// Vertex structure for the first pass (scene with color)
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

/// Vertex structure for fullscreen quad rendering
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct QuadVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

unsafe impl bytemuck::Pod for QuadVertex {}
unsafe impl bytemuck::Zeroable for QuadVertex {}

impl QuadVertex {
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
            label: Some("Post-Processing Device"),
            memory_hints: Default::default(),
            experimental_features: Default::default(),
            trace: Default::default(),
        })
        .await
        .ok()
}

/// Create a texture that can be both rendered to and sampled from
fn create_render_texture(
    device: &wgpu::Device,
    width: u32,
    height: u32,
    label: &str,
) -> (wgpu::Texture, wgpu::TextureView) {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
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
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    (texture, view)
}

/// Create fullscreen quad vertices
fn create_quad_vertices() -> Vec<QuadVertex> {
    vec![
        // First triangle
        QuadVertex::new([-1.0, -1.0], [0.0, 1.0]), // Bottom-left
        QuadVertex::new([1.0, -1.0], [1.0, 1.0]),  // Bottom-right
        QuadVertex::new([1.0, 1.0], [1.0, 0.0]),   // Top-right
        // Second triangle
        QuadVertex::new([-1.0, -1.0], [0.0, 1.0]), // Bottom-left
        QuadVertex::new([1.0, 1.0], [1.0, 0.0]),   // Top-right
        QuadVertex::new([-1.0, 1.0], [0.0, 0.0]),  // Top-left
    ]
}

fn main() {
    env_logger::init();

    println!("=== Post-Processing Effects Example ===\n");
    println!("This example demonstrates multiple post-processing effects:");
    println!("  Pass 1: Render scene to texture");
    println!("  Pass 2: Apply blur effect");
    println!("  Pass 3: Apply grayscale effect");
    println!("  Pass 4: Apply edge detection effect");
    println!("  Pass 5: Display final result\n");

    // Create device and queue
    let device_queue = pollster::block_on(create_device());
    if device_queue.is_none() {
        eprintln!("Failed to create GPU device");
        return;
    }

    let (device, queue) = device_queue.unwrap();
    println!("✓ GPU device created\n");

    // Dimensions
    let width = 800;
    let height = 600;

    // ===== SCENE PASS SETUP =====
    println!("=== Scene Pass Setup ===");

    // Create scene vertices (colorful triangle)
    let scene_vertices = vec![
        ColorVertex::new([0.0, 0.7], [1.0, 0.0, 0.0]),    // Top - Red
        ColorVertex::new([-0.6, -0.4], [0.0, 1.0, 0.0]),  // Bottom-left - Green
        ColorVertex::new([0.6, -0.4], [0.0, 0.0, 1.0]),   // Bottom-right - Blue
    ];

    let scene_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Scene Vertex Buffer"),
        contents: bytemuck::cast_slice(&scene_vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });

    // Load scene shader
    let scene_shader = ShaderModule::from_file(
        "post_processing_scene.wgsl",
        Some("post_processing_scene_shader"),
    )
    .expect("Failed to load scene shader");

    let scene_shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("post_processing_scene_shader"),
        source: wgpu::ShaderSource::Wgsl(scene_shader.source().into()),
    });

    // Create scene pipeline
    let scene_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("scene_pipeline"),
        layout: None,
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
    println!("✓ Scene pipeline created");

    // Create render textures
    let (_scene_texture, scene_view) = create_render_texture(&device, width, height, "Scene Texture");
    let (_blur_texture, blur_view) = create_render_texture(&device, width, height, "Blur Texture");
    let (_grayscale_texture, grayscale_view) = create_render_texture(&device, width, height, "Grayscale Texture");
    let (_edge_texture, edge_view) = create_render_texture(&device, width, height, "Edge Detection Texture");
    let output_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Output Texture"),
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
    });
    let output_view = output_texture.create_view(&wgpu::TextureViewDescriptor::default());
    println!("✓ Render textures created\n");

    // ===== POST-PROCESSING PIPELINES SETUP =====
    println!("=== Post-Processing Pipelines Setup ===");

    // Create fullscreen quad vertices
    let quad_vertices = create_quad_vertices();
    let quad_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Quad Vertex Buffer"),
        contents: bytemuck::cast_slice(&quad_vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });

    // Create sampler
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

    // Create bind group layout (same for all post-processing passes)
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Post-Processing Bind Group Layout"),
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

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Post-Processing Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    // Load post-processing shaders
    let blur_shader = ShaderModule::from_file(
        "post_processing_blur.wgsl",
        Some("post_processing_blur_shader"),
    )
    .expect("Failed to load blur shader");

    let grayscale_shader = ShaderModule::from_file(
        "post_processing_grayscale.wgsl",
        Some("post_processing_grayscale_shader"),
    )
    .expect("Failed to load grayscale shader");

    let edge_shader = ShaderModule::from_file(
        "post_processing_edge.wgsl",
        Some("post_processing_edge_shader"),
    )
    .expect("Failed to load edge detection shader");

    let passthrough_shader = ShaderModule::from_file(
        "post_processing_passthrough.wgsl",
        Some("post_processing_passthrough_shader"),
    )
    .expect("Failed to load passthrough shader");

    // Create shader modules
    let blur_shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("post_processing_blur_shader"),
        source: wgpu::ShaderSource::Wgsl(blur_shader.source().into()),
    });

    let grayscale_shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("post_processing_grayscale_shader"),
        source: wgpu::ShaderSource::Wgsl(grayscale_shader.source().into()),
    });

    let edge_shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("post_processing_edge_shader"),
        source: wgpu::ShaderSource::Wgsl(edge_shader.source().into()),
    });

    let passthrough_shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("post_processing_passthrough_shader"),
        source: wgpu::ShaderSource::Wgsl(passthrough_shader.source().into()),
    });

    // Create blur pipeline
    let blur_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("blur_pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &blur_shader_module,
            entry_point: Some("vs_main"),
            compilation_options: Default::default(),
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<QuadVertex>() as u64,
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
            module: &blur_shader_module,
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

    // Create grayscale pipeline
    let grayscale_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("grayscale_pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &grayscale_shader_module,
            entry_point: Some("vs_main"),
            compilation_options: Default::default(),
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<QuadVertex>() as u64,
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
            module: &grayscale_shader_module,
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

    // Create edge detection pipeline
    let edge_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("edge_pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &edge_shader_module,
            entry_point: Some("vs_main"),
            compilation_options: Default::default(),
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<QuadVertex>() as u64,
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
            module: &edge_shader_module,
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

    // Create passthrough pipeline (for final display)
    let passthrough_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("passthrough_pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &passthrough_shader_module,
            entry_point: Some("vs_main"),
            compilation_options: Default::default(),
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<QuadVertex>() as u64,
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
            module: &passthrough_shader_module,
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

    println!("✓ All post-processing pipelines created\n");

    // Create bind groups
    let blur_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Blur Bind Group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&scene_view),
            },
        ],
    });

    let grayscale_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Grayscale Bind Group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&blur_view),
            },
        ],
    });

    let edge_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Edge Detection Bind Group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&grayscale_view),
            },
        ],
    });

    let final_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Final Display Bind Group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&edge_view),
            },
        ],
    });

    println!("✓ All bind groups created\n");

    // ===== RENDERING =====
    println!("=== Executing Multi-Pass Post-Processing ===\n");

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Post-Processing Encoder"),
    });

    // Pass 1: Render scene to texture
    {
        println!("Pass 1: Rendering scene...");
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Scene Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &scene_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.1,
                        b: 0.15,
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

        render_pass.set_pipeline(&scene_pipeline);
        render_pass.set_vertex_buffer(0, scene_vertex_buffer.slice(..));
        render_pass.draw(0..3, 0..1);
        println!("  ✓ Scene rendered");
    }

    // Pass 2: Apply blur
    {
        println!("Pass 2: Applying blur effect...");
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Blur Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &blur_view,
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

        render_pass.set_pipeline(&blur_pipeline);
        render_pass.set_bind_group(0, &blur_bind_group, &[]);
        render_pass.set_vertex_buffer(0, quad_vertex_buffer.slice(..));
        render_pass.draw(0..6, 0..1);
        println!("  ✓ Blur applied");
    }

    // Pass 3: Apply grayscale
    {
        println!("Pass 3: Applying grayscale effect...");
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Grayscale Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &grayscale_view,
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

        render_pass.set_pipeline(&grayscale_pipeline);
        render_pass.set_bind_group(0, &grayscale_bind_group, &[]);
        render_pass.set_vertex_buffer(0, quad_vertex_buffer.slice(..));
        render_pass.draw(0..6, 0..1);
        println!("  ✓ Grayscale applied");
    }

    // Pass 4: Apply edge detection
    {
        println!("Pass 4: Applying edge detection...");
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Edge Detection Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &edge_view,
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

        render_pass.set_pipeline(&edge_pipeline);
        render_pass.set_bind_group(0, &edge_bind_group, &[]);
        render_pass.set_vertex_buffer(0, quad_vertex_buffer.slice(..));
        render_pass.draw(0..6, 0..1);
        println!("  ✓ Edge detection applied");
    }

    // Pass 5: Display final result
    {
        println!("Pass 5: Displaying final result...");
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Final Display Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &output_view,
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

        render_pass.set_pipeline(&passthrough_pipeline);
        render_pass.set_bind_group(0, &final_bind_group, &[]);
        render_pass.set_vertex_buffer(0, quad_vertex_buffer.slice(..));
        render_pass.draw(0..6, 0..1);
        println!("  ✓ Final result displayed");
    }

    // Submit commands
    queue.submit(std::iter::once(encoder.finish()));
    println!("\n✓ All render commands submitted to GPU");

    // Wait for completion
    let _ = device.poll(wgpu::PollType::Wait {
        submission_index: None,
        timeout: None,
    });
    println!("✓ Rendering complete\n");

    println!("=== Post-Processing Example Complete ===");
    println!("\nThis example successfully demonstrated:");
    println!("  • Multiple render-to-texture passes (5 passes total)");
    println!("  • Scene rendering to offscreen texture");
    println!("  • Blur post-processing effect");
    println!("  • Grayscale post-processing effect");
    println!("  • Edge detection post-processing effect");
    println!("  • Multiple render pipelines with different shaders");
    println!("  • Texture sampling in fragment shaders");
    println!("  • Full-screen quad rendering");
    println!("\nKey WebGPU concepts:");
    println!("  - Render-to-texture workflow (framebuffers)");
    println!("  - Multiple render pipelines for different effects");
    println!("  - Bind groups for texture and sampler resources");
    println!("  - Multi-pass rendering with intermediate textures");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_vertex_size() {
        assert_eq!(std::mem::size_of::<ColorVertex>(), 20); // 2*f32 + 3*f32
    }

    #[test]
    fn test_quad_vertex_size() {
        assert_eq!(std::mem::size_of::<QuadVertex>(), 16); // 2*f32 + 2*f32
    }

    #[test]
    fn test_quad_vertices_count() {
        let vertices = create_quad_vertices();
        assert_eq!(vertices.len(), 6); // Two triangles
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
