/// Instanced Rendering Example
///
/// This example demonstrates efficient rendering of multiple objects using instancing:
/// - Creating instance buffers with per-instance attributes
/// - Rendering multiple objects with a single draw call
/// - Per-instance transformation (position, rotation, scale)
/// - Per-instance colors
///
/// Run with: cargo run --package wgpu_playground_examples --example instanced_rendering
use glam::{Mat4, Vec3};
use wgpu::util::DeviceExt;
use wgpu_playground_core::shader::ShaderModule;

/// Vertex structure for cube geometry (shared across all instances)
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 3],
}

// Safety: Vertex is repr(C) with simple primitive types
unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

impl Vertex {
    fn new(position: [f32; 3]) -> Self {
        Self { position }
    }
}

/// Instance data structure (per-instance attributes)
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct InstanceData {
    position: [f32; 3],
    rotation: f32,
    scale: f32,
    _padding: [f32; 3], // Padding to align to 16 bytes
    color: [f32; 3],
    _padding2: f32, // Padding for next instance
}

// Safety: InstanceData is repr(C) with simple primitive types
unsafe impl bytemuck::Pod for InstanceData {}
unsafe impl bytemuck::Zeroable for InstanceData {}

impl InstanceData {
    fn new(position: [f32; 3], rotation: f32, scale: f32, color: [f32; 3]) -> Self {
        Self {
            position,
            rotation,
            scale,
            _padding: [0.0; 3],
            color,
            _padding2: 0.0,
        }
    }
}

/// Uniform buffer structure for view and projection matrices
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Uniforms {
    view_proj: [[f32; 4]; 4],
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
                label: Some("Instanced Rendering Device"),
                memory_hints: Default::default(),
            },
            None,
        )
        .await
        .ok()
}

/// Create cube vertices (simple cube without per-vertex colors)
fn create_cube_vertices() -> Vec<Vertex> {
    vec![
        // Front face (z = 0.5)
        Vertex::new([-0.5, -0.5, 0.5]),  // 0: bottom-left-front
        Vertex::new([0.5, -0.5, 0.5]),   // 1: bottom-right-front
        Vertex::new([0.5, 0.5, 0.5]),    // 2: top-right-front
        Vertex::new([-0.5, 0.5, 0.5]),   // 3: top-left-front
        // Back face (z = -0.5)
        Vertex::new([-0.5, -0.5, -0.5]), // 4: bottom-left-back
        Vertex::new([0.5, -0.5, -0.5]),  // 5: bottom-right-back
        Vertex::new([0.5, 0.5, -0.5]),   // 6: top-right-back
        Vertex::new([-0.5, 0.5, -0.5]),  // 7: top-left-back
    ]
}

/// Create cube indices (6 faces, 2 triangles per face)
fn create_cube_indices() -> Vec<u16> {
    vec![
        // Front face
        0, 1, 2, 2, 3, 0, // Back face
        5, 4, 7, 7, 6, 5, // Top face
        3, 2, 6, 6, 7, 3, // Bottom face
        4, 5, 1, 1, 0, 4, // Right face
        1, 5, 6, 6, 2, 1, // Left face
        4, 0, 3, 3, 7, 4,
    ]
}

/// Create instance data for multiple cubes in a grid pattern
fn create_instances() -> Vec<InstanceData> {
    let mut instances = Vec::new();
    
    // Create a 5x5 grid of cubes with varying properties
    for x in 0..5 {
        for z in 0..5 {
            let pos_x = (x as f32 - 2.0) * 2.5;
            let pos_z = (z as f32 - 2.0) * 2.5;
            
            // Vary rotation based on position
            let rotation = (x as f32 + z as f32) * 0.3;
            
            // Vary scale slightly
            let scale = 0.5 + ((x + z) as f32 * 0.05);
            
            // Create a color gradient across the grid
            let color = [
                x as f32 / 4.0,           // Red increases with x
                0.5,                       // Constant green
                z as f32 / 4.0,           // Blue increases with z
            ];
            
            instances.push(InstanceData::new(
                [pos_x, 0.0, pos_z],
                rotation,
                scale,
                color,
            ));
        }
    }
    
    instances
}

/// Create view and projection matrices
fn create_view_proj_matrix(aspect_ratio: f32) -> Mat4 {
    // View matrix: camera positioned to see the grid
    let view = Mat4::look_at_rh(
        Vec3::new(0.0, 8.0, 12.0),   // camera position (above and back)
        Vec3::new(0.0, 0.0, 0.0),    // look at center
        Vec3::new(0.0, 1.0, 0.0),    // up vector
    );

    // Projection matrix: perspective
    let projection = Mat4::perspective_rh(
        45.0_f32.to_radians(),
        aspect_ratio,
        0.1,
        100.0,
    );

    projection * view
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

    println!("=== Instanced Rendering Example ===\n");

    // Create device and queue
    let device_queue = pollster::block_on(create_device());
    if device_queue.is_none() {
        eprintln!("Failed to create GPU device");
        return;
    }

    let (device, queue) = device_queue.unwrap();
    println!("✓ GPU device created\n");

    // Create cube geometry (shared by all instances)
    let vertices = create_cube_vertices();
    let indices = create_cube_indices();

    println!("Base cube geometry:");
    println!("  {} vertices", vertices.len());
    println!("  {} indices ({} triangles)", indices.len(), indices.len() / 3);
    println!();

    // Create instance data
    let instances = create_instances();
    println!("Instance data:");
    println!("  {} instances (5x5 grid)", instances.len());
    println!("  Total objects rendered: {}", instances.len());
    println!();

    // Create vertex buffer
    let vertex_data = bytemuck::cast_slice(&vertices);
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: vertex_data,
        usage: wgpu::BufferUsages::VERTEX,
    });
    println!("✓ Vertex buffer created ({} bytes)", vertex_data.len());

    // Create instance buffer
    let instance_data = bytemuck::cast_slice(&instances);
    let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Instance Buffer"),
        contents: instance_data,
        usage: wgpu::BufferUsages::VERTEX,
    });
    println!("✓ Instance buffer created ({} bytes)", instance_data.len());

    // Create index buffer
    let index_data = bytemuck::cast_slice(&indices);
    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: index_data,
        usage: wgpu::BufferUsages::INDEX,
    });
    println!("✓ Index buffer created ({} bytes)", index_data.len());

    // Load and compile shader
    let shader = ShaderModule::from_file("instanced_rendering.wgsl", Some("instanced_shader"))
        .expect("Failed to load instanced rendering shader");
    println!("✓ Shader loaded and compiled");

    // Create shader module
    let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("instanced_shader"),
        source: wgpu::ShaderSource::Wgsl(shader.source().into()),
    });

    // Create uniform buffer
    let width = 800;
    let height = 600;
    let aspect_ratio = width as f32 / height as f32;
    
    let view_proj = create_view_proj_matrix(aspect_ratio);
    let uniforms = Uniforms {
        view_proj: view_proj.to_cols_array_2d(),
    };

    let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Uniform Buffer"),
        contents: bytemuck::cast_slice(&[uniforms]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
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

    // Define vertex buffer layout (per-vertex attributes)
    let vertex_buffer_layout = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as u64,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x3,
            offset: 0,
            shader_location: 0,
        }],
    };

    // Define instance buffer layout (per-instance attributes)
    let instance_buffer_layout = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<InstanceData>() as u64,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &[
            // Instance position
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
                shader_location: 1,
            },
            // Instance rotation
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32,
                offset: std::mem::size_of::<[f32; 3]>() as u64,
                shader_location: 2,
            },
            // Instance scale
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32,
                offset: (std::mem::size_of::<[f32; 3]>() + std::mem::size_of::<f32>()) as u64,
                shader_location: 3,
            },
            // Instance color (with padding offset)
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: (std::mem::size_of::<[f32; 3]>() + std::mem::size_of::<f32>() + std::mem::size_of::<f32>() + std::mem::size_of::<[f32; 3]>()) as u64,
                shader_location: 4,
            },
        ],
    };

    // Create pipeline layout
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    // Create render pipeline with instancing
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("instanced_pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader_module,
            entry_point: "vs_main",
            compilation_options: Default::default(),
            buffers: &[vertex_buffer_layout, instance_buffer_layout],
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
    println!("✓ Render pipeline created with instancing support\n");

    // Create render and depth textures
    let render_texture = create_render_texture(&device, width, height);
    let render_view = render_texture.create_view(&wgpu::TextureViewDescriptor::default());

    let depth_texture = create_depth_texture(&device, width, height);
    let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
    println!("✓ Render target created ({}x{})", width, height);
    println!("✓ Depth buffer created\n");

    // Create command encoder and render pass
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Instanced Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &render_view,
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
        render_pass.set_vertex_buffer(1, instance_buffer.slice(..));
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        
        // Draw all instances with a single draw call!
        render_pass.draw_indexed(0..indices.len() as u32, 0, 0..instances.len() as u32);
        
        println!("✓ Render pass configured");
        println!("  - Clear color: dark blue (0.1, 0.1, 0.15)");
        println!("  - Drawing {} instances with 1 draw call", instances.len());
        println!("  - Total triangles: {}", indices.len() / 3 * instances.len());
    }

    // Submit commands
    queue.submit(std::iter::once(encoder.finish()));
    println!("\n✓ Render commands submitted to GPU");

    // Wait for completion
    device.poll(wgpu::Maintain::Wait);
    println!("✓ Rendering complete\n");

    println!("=== Instanced Rendering Example Complete ===");
    println!("\nSuccessfully demonstrated instanced rendering with:");
    println!("  • {} cube instances rendered in a single draw call", instances.len());
    println!("  • Per-instance attributes:");
    println!("    - Position (3D translation)");
    println!("    - Rotation (around Y axis)");
    println!("    - Scale (uniform scaling)");
    println!("    - Color (RGB values)");
    println!("  • Instance buffer with step mode = Instance");
    println!("  • Vertex buffer with step mode = Vertex");
    println!("  • Depth testing enabled");
    println!("  • Back-face culling enabled");
    println!("\nPerformance benefits:");
    println!("  • Single draw call vs {} draw calls", instances.len());
    println!("  • Reduced CPU overhead");
    println!("  • Efficient GPU processing");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_size() {
        assert_eq!(std::mem::size_of::<Vertex>(), 12); // 3*f32 = 12 bytes
    }

    #[test]
    fn test_instance_data_size() {
        // InstanceData should be aligned properly
        let size = std::mem::size_of::<InstanceData>();
        assert_eq!(size, 32); // Properly aligned for GPU
    }

    #[test]
    fn test_uniforms_size() {
        assert_eq!(std::mem::size_of::<Uniforms>(), 64); // 4x4 matrix = 64 bytes
    }

    #[test]
    fn test_cube_vertices() {
        let vertices = create_cube_vertices();
        assert_eq!(vertices.len(), 8); // 8 corners
    }

    #[test]
    fn test_cube_indices() {
        let indices = create_cube_indices();
        assert_eq!(indices.len(), 36); // 6 faces * 2 triangles * 3 indices
    }

    #[test]
    fn test_create_instances() {
        let instances = create_instances();
        assert_eq!(instances.len(), 25); // 5x5 grid
        
        // Verify each instance has valid data
        for instance in instances.iter() {
            // Scale should be in reasonable range
            assert!(instance.scale > 0.0 && instance.scale < 2.0);
            // Color components should be in [0, 1] range
            assert!(instance.color[0] >= 0.0 && instance.color[0] <= 1.0);
            assert!(instance.color[1] >= 0.0 && instance.color[1] <= 1.0);
            assert!(instance.color[2] >= 0.0 && instance.color[2] <= 1.0);
        }
    }

    #[test]
    fn test_vertices_are_pod() {
        let vertices = create_cube_vertices();
        let bytes = bytemuck::cast_slice::<Vertex, u8>(&vertices);
        assert_eq!(bytes.len(), 8 * std::mem::size_of::<Vertex>());
    }

    #[test]
    fn test_instances_are_pod() {
        let instances = create_instances();
        let bytes = bytemuck::cast_slice::<InstanceData, u8>(&instances);
        assert_eq!(bytes.len(), 25 * std::mem::size_of::<InstanceData>());
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
