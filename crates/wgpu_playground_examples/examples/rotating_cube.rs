/// Rotating 3D Cube Example
///
/// This example demonstrates advanced WebGPU features:
/// - 3D cube geometry with 8 vertices and 12 triangles (using index buffer)
/// - Uniform buffers for transformation matrices (model-view-projection)
/// - Depth testing to correctly render front/back faces
/// - Animation loop with rotation over time
///
/// Run with: cargo run --package wgpu_playground_examples --example rotating_cube
use glam::{Mat4, Vec3};
use wgpu::util::DeviceExt;
use wgpu_playground_core::shader::ShaderModule;

/// Vertex structure for 3D cube
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

// Safety: Vertex is repr(C) with simple primitive types
unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

impl Vertex {
    fn new(position: [f32; 3], color: [f32; 3]) -> Self {
        Self { position, color }
    }
}

/// Uniform buffer structure for transformation matrices
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Uniforms {
    model_view_proj: [[f32; 4]; 4],
}

// Safety: Uniforms is repr(C) with simple primitive types
unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}

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
                label: Some("Cube Device"),
                memory_hints: Default::default(),
            },
            None,
        )
        .await
        .ok()
}

/// Create cube vertices (8 corners with different colors)
fn create_cube_vertices() -> Vec<Vertex> {
    vec![
        // Front face (z = 0.5) - Red tones
        Vertex::new([-0.5, -0.5, 0.5], [1.0, 0.0, 0.0]), // 0: bottom-left-front
        Vertex::new([0.5, -0.5, 0.5], [1.0, 0.5, 0.0]),  // 1: bottom-right-front
        Vertex::new([0.5, 0.5, 0.5], [1.0, 1.0, 0.0]),   // 2: top-right-front
        Vertex::new([-0.5, 0.5, 0.5], [1.0, 0.0, 0.5]),  // 3: top-left-front
        // Back face (z = -0.5) - Blue tones
        Vertex::new([-0.5, -0.5, -0.5], [0.0, 0.0, 1.0]), // 4: bottom-left-back
        Vertex::new([0.5, -0.5, -0.5], [0.0, 0.5, 1.0]),  // 5: bottom-right-back
        Vertex::new([0.5, 0.5, -0.5], [0.0, 1.0, 1.0]),   // 6: top-right-back
        Vertex::new([-0.5, 0.5, -0.5], [0.5, 0.0, 1.0]),  // 7: top-left-back
    ]
}

/// Create cube indices (6 faces, 2 triangles per face, 3 indices per triangle)
fn create_cube_indices() -> Vec<u16> {
    vec![
        // Front face (vertices 0-3)
        0, 1, 2, 2, 3, 0,
        // Back face (vertices 4-7)
        5, 4, 7, 7, 6, 5,
        // Top face (vertices 3, 2, 6, 7)
        3, 2, 6, 6, 7, 3,
        // Bottom face (vertices 4, 5, 1, 0)
        4, 5, 1, 1, 0, 4,
        // Right face (vertices 1, 5, 6, 2)
        1, 5, 6, 6, 2, 1,
        // Left face (vertices 4, 0, 3, 7)
        4, 0, 3, 3, 7, 4,
    ]
}

/// Create transformation matrices for the cube
fn create_transform_matrix(rotation_radians: f32, aspect_ratio: f32) -> Mat4 {
    // Model matrix: rotate around Y and X axes
    let model = Mat4::from_rotation_y(rotation_radians)
        * Mat4::from_rotation_x(rotation_radians * 0.5);

    // View matrix: camera at (0, 0, 3) looking at origin
    let view = Mat4::look_at_rh(
        Vec3::new(0.0, 0.0, 3.0), // camera position
        Vec3::new(0.0, 0.0, 0.0), // look at point
        Vec3::new(0.0, 1.0, 0.0), // up vector
    );

    // Projection matrix: perspective with 45° FOV
    let projection = Mat4::perspective_rh(
        45.0_f32.to_radians(), // field of view
        aspect_ratio,          // aspect ratio
        0.1,                   // near plane
        100.0,                 // far plane
    );

    // Combine: projection * view * model
    projection * view * model
}

/// Create a texture for rendering to
fn create_render_texture(device: &wgpu::Device, width: u32, height: u32) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Render Texture"),
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

/// Create a depth texture for depth testing
fn create_depth_texture(device: &wgpu::Device, width: u32, height: u32) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Depth Texture"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth24Plus,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    })
}

fn main() {
    env_logger::init();

    println!("=== Rotating 3D Cube Example ===\n");

    // Create device and queue
    let device_queue = pollster::block_on(create_device());
    if device_queue.is_none() {
        eprintln!("Failed to create GPU device");
        return;
    }

    let (device, queue) = device_queue.unwrap();
    println!("✓ GPU device created\n");

    // Create cube geometry
    let vertices = create_cube_vertices();
    let indices = create_cube_indices();

    println!("Cube geometry:");
    println!("  {} vertices (8 corners)", vertices.len());
    println!("  {} indices (12 triangles, 6 faces)", indices.len());
    println!();

    // Create vertex buffer
    let vertex_data = bytemuck::cast_slice(&vertices);
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Cube Vertex Buffer"),
        contents: vertex_data,
        usage: wgpu::BufferUsages::VERTEX,
    });
    println!("✓ Vertex buffer created ({} bytes)", vertex_data.len());

    // Create index buffer
    let index_data = bytemuck::cast_slice(&indices);
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Cube Index Buffer"),
        contents: index_data,
        usage: wgpu::BufferUsages::INDEX,
    });
    println!("✓ Index buffer created ({} bytes)", index_data.len());

    // Load and compile shader
    let shader = ShaderModule::from_file("rotating_cube.wgsl", Some("cube_shader"))
        .expect("Failed to load cube shader");
    println!("✓ Shader loaded and compiled");

    // Create shader module
    let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("cube_shader"),
        source: wgpu::ShaderSource::Wgsl(shader.source().into()),
    });

    // Create uniform buffer
    let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Uniform Buffer"),
        size: std::mem::size_of::<Uniforms>() as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    println!("✓ Uniform buffer created ({} bytes)", std::mem::size_of::<Uniforms>());

    // Create bind group layout
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Bind Group Layout"),
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

    // Create bind group
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Bind Group"),
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: uniform_buffer.as_entire_binding(),
        }],
    });
    println!("✓ Bind group created\n");

    // Define vertex buffer layout
    let vertex_buffer_layout = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as u64,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            // Position attribute
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0,
            },
            // Color attribute
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: std::mem::size_of::<[f32; 3]>() as u64,
                shader_location: 1,
            },
        ],
    };

    // Create pipeline layout
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Cube Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    // Create render pipeline with depth testing
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("cube_pipeline"),
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
    println!("✓ Render pipeline created with depth testing\n");

    // Render dimensions
    let width = 800;
    let height = 600;
    let aspect_ratio = width as f32 / height as f32;

    // Create render and depth textures
    let render_texture = create_render_texture(&device, width, height);
    let render_view = render_texture.create_view(&wgpu::TextureViewDescriptor::default());

    let depth_texture = create_depth_texture(&device, width, height);
    let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
    println!("✓ Render target created ({}x{})", width, height);
    println!("✓ Depth buffer created\n");

    // Simulate animation frames
    println!("Rendering animation frames:");
    let num_frames = 5;
    for frame in 0..num_frames {
        // Calculate rotation based on frame
        let rotation = (frame as f32 / num_frames as f32) * std::f32::consts::PI * 2.0;

        // Update transformation matrix
        let transform = create_transform_matrix(rotation, aspect_ratio);
        let uniforms = Uniforms {
            model_view_proj: transform.to_cols_array_2d(),
        };

        // Update uniform buffer
        queue.write_buffer(&uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));

        // Create command encoder
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Cube Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Cube Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &render_view,
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
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&pipeline);
            render_pass.set_bind_group(0, &bind_group, &[]);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..indices.len() as u32, 0, 0..1);
        }

        // Submit commands
        queue.submit(std::iter::once(encoder.finish()));
        device.poll(wgpu::Maintain::Wait);

        println!("  Frame {}: rotation = {:.2} radians", frame, rotation);
    }

    println!("\n✓ All frames rendered successfully\n");

    println!("=== Rotating Cube Example Complete ===");
    println!("\nThe 3D cube was successfully rendered with:");
    println!("  • 8 vertices defining cube corners");
    println!("  • 36 indices defining 12 triangles (6 faces)");
    println!("  • Uniform buffer with model-view-projection matrix");
    println!("  • Depth testing enabled (Depth24Plus format)");
    println!("  • Back-face culling enabled");
    println!("  • Rotation animation over {} frames", num_frames);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_size() {
        // Verify vertex structure has correct size
        assert_eq!(std::mem::size_of::<Vertex>(), 24); // 3*f32 + 3*f32 = 24 bytes
    }

    #[test]
    fn test_uniforms_size() {
        // Verify uniforms structure has correct size
        assert_eq!(std::mem::size_of::<Uniforms>(), 64); // 4x4 matrix = 64 bytes
    }

    #[test]
    fn test_cube_vertices() {
        let vertices = create_cube_vertices();
        assert_eq!(vertices.len(), 8); // 8 corners of a cube
    }

    #[test]
    fn test_cube_indices() {
        let indices = create_cube_indices();
        assert_eq!(indices.len(), 36); // 6 faces * 2 triangles * 3 indices
    }

    #[test]
    fn test_vertices_are_pod() {
        let vertices = create_cube_vertices();
        let bytes = bytemuck::cast_slice::<Vertex, u8>(&vertices);
        assert_eq!(bytes.len(), 8 * std::mem::size_of::<Vertex>());
    }

    #[test]
    fn test_transform_matrix() {
        let transform = create_transform_matrix(0.0, 1.0);
        // Matrix should be valid (not NaN or infinite)
        let array = transform.to_cols_array_2d();
        for row in array.iter() {
            for val in row.iter() {
                assert!(val.is_finite());
            }
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
