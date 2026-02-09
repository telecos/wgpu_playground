/// Lighting and Shadows Example
///
/// This example demonstrates advanced WebGPU features:
/// - Multi-pass rendering (shadow pass + main rendering pass)
/// - Depth textures with TEXTURE_BINDING usage for shadow mapping
/// - Comparison samplers for hardware PCF (Percentage Closer Filtering)
/// - Multiple bind groups per pipeline
/// - Uniform buffers with camera, light, and shadow data
/// - Directional and point lights
/// - Basic Phong lighting model
///
/// Run with: cargo run --package wgpu_playground_examples --example lighting_shadows
use glam::{Mat4, Vec3, Vec4};
use wgpu::util::DeviceExt;
use wgpu_playground_core::shader::ShaderModule;

/// Vertex structure for 3D geometry with normals
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
}

// Safety: Vertex is repr(C) with simple primitive types
unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

impl Vertex {
    fn new(position: [f32; 3], normal: [f32; 3]) -> Self {
        Self { position, normal }
    }
}

/// Type alias for geometry data (vertices and indices)
type GeometryData = (Vec<Vertex>, Vec<u16>);

/// Camera uniform buffer
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct CameraUniforms {
    view_proj: [[f32; 4]; 4],
    camera_pos: [f32; 4], // w component unused, for alignment
}

unsafe impl bytemuck::Pod for CameraUniforms {}
unsafe impl bytemuck::Zeroable for CameraUniforms {}

/// Light uniform buffer
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct LightUniforms {
    // Directional light
    dir_light_direction: [f32; 4], // w unused
    dir_light_color: [f32; 4],     // w = intensity
    // Point light
    point_light_position: [f32; 4], // w unused
    point_light_color: [f32; 4],    // w = intensity
}

unsafe impl bytemuck::Pod for LightUniforms {}
unsafe impl bytemuck::Zeroable for LightUniforms {}

/// Shadow uniform buffer
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct ShadowUniforms {
    light_view_proj: [[f32; 4]; 4],
}

unsafe impl bytemuck::Pod for ShadowUniforms {}
unsafe impl bytemuck::Zeroable for ShadowUniforms {}

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
            label: Some("Lighting Device"),
            memory_hints: Default::default(),
            experimental_features: Default::default(),
            trace: Default::default(),
        })
        .await
        .ok()
}

/// Create a cube with normals
fn create_cube_geometry() -> GeometryData {
    let vertices = vec![
        // Front face (z = 0.5) - Normal pointing forward
        Vertex::new([-0.5, -0.5, 0.5], [0.0, 0.0, 1.0]),
        Vertex::new([0.5, -0.5, 0.5], [0.0, 0.0, 1.0]),
        Vertex::new([0.5, 0.5, 0.5], [0.0, 0.0, 1.0]),
        Vertex::new([-0.5, 0.5, 0.5], [0.0, 0.0, 1.0]),
        // Back face (z = -0.5) - Normal pointing backward
        Vertex::new([-0.5, -0.5, -0.5], [0.0, 0.0, -1.0]),
        Vertex::new([0.5, -0.5, -0.5], [0.0, 0.0, -1.0]),
        Vertex::new([0.5, 0.5, -0.5], [0.0, 0.0, -1.0]),
        Vertex::new([-0.5, 0.5, -0.5], [0.0, 0.0, -1.0]),
        // Top face (y = 0.5) - Normal pointing up
        Vertex::new([-0.5, 0.5, -0.5], [0.0, 1.0, 0.0]),
        Vertex::new([0.5, 0.5, -0.5], [0.0, 1.0, 0.0]),
        Vertex::new([0.5, 0.5, 0.5], [0.0, 1.0, 0.0]),
        Vertex::new([-0.5, 0.5, 0.5], [0.0, 1.0, 0.0]),
        // Bottom face (y = -0.5) - Normal pointing down
        Vertex::new([-0.5, -0.5, -0.5], [0.0, -1.0, 0.0]),
        Vertex::new([0.5, -0.5, -0.5], [0.0, -1.0, 0.0]),
        Vertex::new([0.5, -0.5, 0.5], [0.0, -1.0, 0.0]),
        Vertex::new([-0.5, -0.5, 0.5], [0.0, -1.0, 0.0]),
        // Right face (x = 0.5) - Normal pointing right
        Vertex::new([0.5, -0.5, -0.5], [1.0, 0.0, 0.0]),
        Vertex::new([0.5, 0.5, -0.5], [1.0, 0.0, 0.0]),
        Vertex::new([0.5, 0.5, 0.5], [1.0, 0.0, 0.0]),
        Vertex::new([0.5, -0.5, 0.5], [1.0, 0.0, 0.0]),
        // Left face (x = -0.5) - Normal pointing left
        Vertex::new([-0.5, -0.5, -0.5], [-1.0, 0.0, 0.0]),
        Vertex::new([-0.5, 0.5, -0.5], [-1.0, 0.0, 0.0]),
        Vertex::new([-0.5, 0.5, 0.5], [-1.0, 0.0, 0.0]),
        Vertex::new([-0.5, -0.5, 0.5], [-1.0, 0.0, 0.0]),
    ];

    let indices = vec![
        0, 1, 2, 2, 3, 0, // Front
        5, 4, 7, 7, 6, 5, // Back
        8, 9, 10, 10, 11, 8, // Top
        13, 12, 15, 15, 14, 13, // Bottom
        16, 17, 18, 18, 19, 16, // Right
        21, 20, 23, 23, 22, 21, // Left
    ];

    (vertices, indices)
}

/// Create a ground plane
fn create_ground_plane() -> GeometryData {
    let size = 5.0;
    let vertices = vec![
        Vertex::new([-size, -1.0, -size], [0.0, 1.0, 0.0]),
        Vertex::new([size, -1.0, -size], [0.0, 1.0, 0.0]),
        Vertex::new([size, -1.0, size], [0.0, 1.0, 0.0]),
        Vertex::new([-size, -1.0, size], [0.0, 1.0, 0.0]),
    ];

    let indices = vec![0, 1, 2, 2, 3, 0];

    (vertices, indices)
}

/// Create shadow depth texture
fn create_shadow_texture(device: &wgpu::Device, size: u32) -> (wgpu::Texture, wgpu::TextureView) {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Shadow Depth Texture"),
        size: wgpu::Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        // CRITICAL: TEXTURE_BINDING allows sampling the depth texture in shaders
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    (texture, view)
}

/// Create comparison sampler for shadow mapping
fn create_shadow_sampler(device: &wgpu::Device) -> wgpu::Sampler {
    device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("Shadow Sampler"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::MipmapFilterMode::Nearest,
        compare: Some(wgpu::CompareFunction::LessEqual), // Comparison sampler for shadow mapping
        ..Default::default()
    })
}

fn main() {
    env_logger::init();

    println!("=== Lighting and Shadows Example ===\n");
    println!("This example demonstrates:");
    println!("  • Multi-pass rendering (shadow pass + main pass)");
    println!("  • Depth textures with TEXTURE_BINDING usage");
    println!("  • Comparison samplers for shadow mapping");
    println!("  • Multiple bind groups per pipeline");
    println!("  • Directional and point lights\n");

    // Create device and queue
    let device_queue = pollster::block_on(create_device());
    if device_queue.is_none() {
        eprintln!("Failed to create GPU device");
        return;
    }

    let (device, queue) = device_queue.unwrap();
    println!("✓ GPU device created\n");

    // Create geometry
    let (cube_vertices, cube_indices) = create_cube_geometry();
    let (plane_vertices, plane_indices) = create_ground_plane();

    println!("Geometry created:");
    println!(
        "  Cube: {} vertices, {} indices",
        cube_vertices.len(),
        cube_indices.len()
    );
    println!(
        "  Plane: {} vertices, {} indices",
        plane_vertices.len(),
        plane_indices.len()
    );

    // Combine geometry
    let mut all_vertices = cube_vertices.clone();

    let mut all_indices = cube_indices;
    let plane_index_offset = all_vertices.len() as u16;
    all_vertices.extend_from_slice(&plane_vertices);
    all_indices.extend(plane_indices.iter().map(|i| i + plane_index_offset));

    // Create vertex and index buffers
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&all_vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&all_indices),
        usage: wgpu::BufferUsages::INDEX,
    });

    println!("✓ Vertex and index buffers created\n");

    // Create uniform buffers
    let camera_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Camera Uniform Buffer"),
        size: std::mem::size_of::<CameraUniforms>() as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let light_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Light Uniform Buffer"),
        size: std::mem::size_of::<LightUniforms>() as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let shadow_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Shadow Uniform Buffer"),
        size: std::mem::size_of::<ShadowUniforms>() as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    println!("✓ Uniform buffers created\n");

    // Create shadow texture and sampler
    let shadow_size = 1024;
    let (_shadow_texture, shadow_view) = create_shadow_texture(&device, shadow_size);
    let shadow_sampler = create_shadow_sampler(&device);
    println!(
        "✓ Shadow depth texture created ({}x{})",
        shadow_size, shadow_size
    );
    println!("✓ Shadow comparison sampler created\n");

    // ===== SHADOW PASS SETUP =====
    println!("=== Shadow Pass Setup ===");

    let shadow_shader =
        ShaderModule::from_file("lighting_shadows_shadow.wgsl", Some("shadow_shader"))
            .expect("Failed to load shadow shader");

    let shadow_shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("shadow_shader"),
        source: wgpu::ShaderSource::Wgsl(shadow_shader.source().into()),
    });
    println!("✓ Shadow shader loaded");

    // Shadow pass bind group layout
    let shadow_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Shadow Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

    let shadow_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Shadow Bind Group"),
        layout: &shadow_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: shadow_uniform_buffer.as_entire_binding(),
        }],
    });

    // Shadow pass pipeline
    let shadow_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Shadow Pipeline Layout"),
        bind_group_layouts: &[&shadow_bind_group_layout],
        immediate_size: 0,
    });

    let shadow_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("shadow_pipeline"),
        layout: Some(&shadow_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shadow_shader_module,
            entry_point: Some("vs_main"),
            compilation_options: Default::default(),
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Vertex>() as u64,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                }],
            }],
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState::default(),
        fragment: None, // No fragment shader needed for shadow pass
        multiview_mask: None,
        cache: None,
    });
    println!("✓ Shadow pipeline created\n");

    // ===== MAIN PASS SETUP =====
    println!("=== Main Pass Setup ===");

    let main_shader = ShaderModule::from_file("lighting_shadows_main.wgsl", Some("main_shader"))
        .expect("Failed to load main shader");

    let main_shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("main_shader"),
        source: wgpu::ShaderSource::Wgsl(main_shader.source().into()),
    });
    println!("✓ Main shader loaded");

    // Main pass bind group layouts
    let camera_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

    let light_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Light Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

    let shadow_map_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Shadow Map Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Depth,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

    // Create bind groups
    let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Camera Bind Group"),
        layout: &camera_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: camera_uniform_buffer.as_entire_binding(),
        }],
    });

    let light_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Light Bind Group"),
        layout: &light_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: light_uniform_buffer.as_entire_binding(),
        }],
    });

    let shadow_map_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Shadow Map Bind Group"),
        layout: &shadow_map_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&shadow_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&shadow_sampler),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: shadow_uniform_buffer.as_entire_binding(),
            },
        ],
    });

    println!("✓ Multiple bind groups created (camera, light, shadow map)");

    // Main pass pipeline
    let main_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Main Pipeline Layout"),
        bind_group_layouts: &[
            &camera_bind_group_layout,
            &light_bind_group_layout,
            &shadow_map_bind_group_layout,
        ],
        immediate_size: 0,
    });

    let main_depth_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Main Depth Texture"),
        size: wgpu::Extent3d {
            width: 800,
            height: 600,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth24Plus,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    let main_depth_view = main_depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

    let main_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("main_pipeline"),
        layout: Some(&main_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &main_shader_module,
            entry_point: Some("vs_main"),
            compilation_options: Default::default(),
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Vertex>() as u64,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[
                    wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x3,
                        offset: 0,
                        shader_location: 0,
                    },
                    wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x3,
                        offset: std::mem::size_of::<[f32; 3]>() as u64,
                        shader_location: 1,
                    },
                ],
            }],
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth24Plus,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState::default(),
        fragment: Some(wgpu::FragmentState {
            module: &main_shader_module,
            entry_point: Some("fs_main"),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview_mask: None,
        cache: None,
    });
    println!("✓ Main pipeline created\n");

    // Create output texture
    let output_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Output Texture"),
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
    });
    let output_view = output_texture.create_view(&wgpu::TextureViewDescriptor::default());

    // Setup scene
    let aspect_ratio = 800.0 / 600.0;

    // Camera setup
    let camera_pos = Vec3::new(3.0, 3.0, 5.0);
    let view = Mat4::look_at_rh(camera_pos, Vec3::ZERO, Vec3::Y);
    let projection = Mat4::perspective_rh(45.0_f32.to_radians(), aspect_ratio, 0.1, 100.0);
    let view_proj = projection * view;

    let camera_uniforms = CameraUniforms {
        view_proj: view_proj.to_cols_array_2d(),
        camera_pos: Vec4::from((camera_pos, 1.0)).into(),
    };
    queue.write_buffer(
        &camera_uniform_buffer,
        0,
        bytemuck::cast_slice(&[camera_uniforms]),
    );

    // Light setup
    let dir_light_dir = Vec3::new(-0.3, -1.0, -0.5).normalize();
    let light_uniforms = LightUniforms {
        dir_light_direction: Vec4::from((dir_light_dir, 0.0)).into(),
        dir_light_color: [1.0, 1.0, 0.9, 0.6], // Warm white, medium intensity
        point_light_position: Vec4::from((Vec3::new(2.0, 2.0, 2.0), 0.0)).into(),
        point_light_color: [0.9, 0.8, 1.0, 1.0], // Cool white, full intensity
    };
    queue.write_buffer(
        &light_uniform_buffer,
        0,
        bytemuck::cast_slice(&[light_uniforms]),
    );

    // Shadow matrix setup (from light's perspective)
    let light_pos = Vec3::new(-3.0, 5.0, -2.0);
    let light_view = Mat4::look_at_rh(light_pos, Vec3::ZERO, Vec3::Y);
    let light_projection = Mat4::orthographic_rh(-8.0, 8.0, -8.0, 8.0, 0.1, 20.0);
    let light_view_proj = light_projection * light_view;

    let shadow_uniforms = ShadowUniforms {
        light_view_proj: light_view_proj.to_cols_array_2d(),
    };
    queue.write_buffer(
        &shadow_uniform_buffer,
        0,
        bytemuck::cast_slice(&[shadow_uniforms]),
    );

    println!("=== Rendering ===\n");

    // Create command encoder
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });

    // PASS 1: Shadow Pass
    {
        println!("Pass 1: Rendering shadow map from light's perspective...");
        let mut shadow_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Shadow Pass"),
            color_attachments: &[],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &shadow_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        shadow_pass.set_pipeline(&shadow_pipeline);
        shadow_pass.set_bind_group(0, &shadow_bind_group, &[]);
        shadow_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        shadow_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        shadow_pass.draw_indexed(0..all_indices.len() as u32, 0, 0..1);
        println!("  ✓ Shadow map rendered");
    }

    // PASS 2: Main Pass
    {
        println!("Pass 2: Rendering scene with lighting and shadows...");
        let mut main_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Main Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &output_view,
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
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &main_depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        main_pass.set_pipeline(&main_pipeline);
        main_pass.set_bind_group(0, &camera_bind_group, &[]);
        main_pass.set_bind_group(1, &light_bind_group, &[]);
        main_pass.set_bind_group(2, &shadow_map_bind_group, &[]);
        main_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        main_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        main_pass.draw_indexed(0..all_indices.len() as u32, 0, 0..1);
        println!("  ✓ Scene rendered with lighting and shadows");
    }

    // Submit commands
    queue.submit(std::iter::once(encoder.finish()));
    println!("\n✓ Render commands submitted");

    // Wait for completion
    let _ = device.poll(wgpu::PollType::Wait {
        submission_index: None,
        timeout: None,
    });
    println!("✓ Rendering complete\n");

    println!("=== Lighting and Shadows Example Complete ===");
    println!("\nThis example successfully demonstrated:");
    println!("  • Multi-pass rendering (shadow pass -> main pass)");
    println!("  • Shadow depth texture with TEXTURE_BINDING usage");
    println!("  • Comparison sampler for hardware PCF");
    println!("  • Three bind groups in main pass:");
    println!("    - Bind group 0: Camera uniforms");
    println!("    - Bind group 1: Light uniforms");
    println!("    - Bind group 2: Shadow map + sampler + shadow matrix");
    println!("  • Directional light (sun-like)");
    println!("  • Point light (local light source)");
    println!("  • Phong lighting model with ambient, diffuse, and specular");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_size() {
        assert_eq!(std::mem::size_of::<Vertex>(), 24); // 3*f32 + 3*f32 = 24 bytes
    }

    #[test]
    fn test_camera_uniforms_size() {
        assert_eq!(std::mem::size_of::<CameraUniforms>(), 80); // mat4x4 + vec4 = 64 + 16
    }

    #[test]
    fn test_light_uniforms_size() {
        assert_eq!(std::mem::size_of::<LightUniforms>(), 64); // 4 * vec4 = 64 bytes
    }

    #[test]
    fn test_shadow_uniforms_size() {
        assert_eq!(std::mem::size_of::<ShadowUniforms>(), 64); // mat4x4 = 64 bytes
    }

    #[test]
    fn test_cube_geometry() {
        let (vertices, indices) = create_cube_geometry();
        assert_eq!(vertices.len(), 24); // 6 faces * 4 vertices per face
        assert_eq!(indices.len(), 36); // 6 faces * 2 triangles * 3 indices
    }

    #[test]
    fn test_ground_plane() {
        let (vertices, indices) = create_ground_plane();
        assert_eq!(vertices.len(), 4);
        assert_eq!(indices.len(), 6); // 2 triangles * 3 indices
    }

    #[test]
    fn test_vertices_are_pod() {
        let (vertices, _) = create_cube_geometry();
        let bytes = bytemuck::cast_slice::<Vertex, u8>(&vertices);
        assert_eq!(bytes.len(), vertices.len() * std::mem::size_of::<Vertex>());
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
