/// 3MF Viewer Example
///
/// This example demonstrates loading and rendering a 3MF file using lib3mf:
/// - Parsing a 3MF file with the lib3mf crate
/// - Extracting mesh geometry (vertices and triangles)
/// - Computing per-face normals for flat shading
/// - Uploading mesh data to GPU buffers
/// - Rendering with a simple directional lighting shader
///
/// Run with: cargo run --package wgpu_playground_examples --example threedmf_viewer
use glam::{Mat4, Vec3};
use lib3mf::Model;
use wgpu::util::DeviceExt;
use wgpu_playground_core::{assets, shader::ShaderModule};

/// Vertex structure for 3D rendering with position and normal
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
}

// Safety: Vertex is repr(C) with simple primitive types
unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

/// Uniform buffer for transformation and lighting
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Uniforms {
    model_view_proj: [[f32; 4]; 4],
    model: [[f32; 4]; 4],
    light_dir: [f32; 4],
}

// Safety: Uniforms is repr(C) with simple primitive types
unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}

/// Create a wgpu device and queue
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
            label: Some("3MF Viewer Device"),
            memory_hints: Default::default(),
            experimental_features: Default::default(),
            trace: Default::default(),
        })
        .await
        .ok()
}

/// Convenience result type for model loading errors
type LoadResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Load a 3MF file from the assets/models directory
fn load_3mf_model(filename: &str) -> LoadResult<Model> {
    let data = assets::load_model(filename)?;
    let cursor = std::io::Cursor::new(data);
    let model = Model::from_reader(cursor)?;
    Ok(model)
}

/// Extract renderable geometry from a lib3mf Model.
///
/// Returns a flat list of vertices with per-face normals (flat shading).
/// Each triangle is expanded into 3 separate vertices so that every vertex
/// carries the face normal of its triangle, ensuring correct flat shading.
fn extract_mesh_data(model: &Model) -> Vec<Vertex> {
    let mut vertices = Vec::new();

    for obj in &model.resources.objects {
        let Some(ref mesh) = obj.mesh else {
            continue;
        };

        for tri in &mesh.triangles {
            let v0 = &mesh.vertices[tri.v1];
            let v1 = &mesh.vertices[tri.v2];
            let v2 = &mesh.vertices[tri.v3];

            let p0 = Vec3::new(v0.x as f32, v0.y as f32, v0.z as f32);
            let p1 = Vec3::new(v1.x as f32, v1.y as f32, v1.z as f32);
            let p2 = Vec3::new(v2.x as f32, v2.y as f32, v2.z as f32);

            // Compute face normal via cross product
            let edge1 = p1 - p0;
            let edge2 = p2 - p0;
            let normal = edge1.cross(edge2).normalize_or_zero();
            let normal_arr = normal.to_array();

            vertices.push(Vertex {
                position: p0.to_array(),
                normal: normal_arr,
            });
            vertices.push(Vertex {
                position: p1.to_array(),
                normal: normal_arr,
            });
            vertices.push(Vertex {
                position: p2.to_array(),
                normal: normal_arr,
            });
        }
    }

    vertices
}

/// Compute the axis-aligned bounding box centre and a uniform scale factor.
///
/// The scale factor normalises the model to fit within a [-1, 1] cube.
fn compute_normalisation(vertices: &[Vertex]) -> (Vec3, f32) {
    if vertices.is_empty() {
        return (Vec3::ZERO, 1.0);
    }

    let mut min = Vec3::splat(f32::MAX);
    let mut max = Vec3::splat(f32::MIN);
    for v in vertices {
        let p = Vec3::from(v.position);
        min = min.min(p);
        max = max.max(p);
    }

    let center = (min + max) * 0.5;
    let extents = max - min;
    let max_extent = extents.max_element().max(f32::EPSILON);
    // Normalise to fit within a [-1, 1] cube
    let scale = 2.0 / max_extent;

    (center, scale)
}

/// Create the render texture
fn create_render_texture(device: &wgpu::Device, width: u32, height: u32) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("3MF Render Texture"),
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

/// Create the depth texture
fn create_depth_texture(device: &wgpu::Device, width: u32, height: u32) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("3MF Depth Texture"),
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

    println!("=== 3MF Viewer Example ===\n");
    println!("This example demonstrates:");
    println!("  \u{2022} Loading a 3MF file with the lib3mf crate");
    println!("  \u{2022} Extracting mesh geometry (vertices and triangles)");
    println!("  \u{2022} Computing per-face normals for flat shading");
    println!("  \u{2022} Rendering with directional lighting\n");

    // ----- Load 3MF model -----
    println!("Loading 3MF model...");
    let model = match load_3mf_model("cube.3mf") {
        Ok(m) => {
            println!("\u{2713} 3MF file loaded successfully");
            println!(
                "  Unit: {}",
                if m.unit.is_empty() {
                    "millimeter"
                } else {
                    &m.unit
                }
            );
            println!("  Objects: {}", m.resources.objects.len());
            for (i, obj) in m.resources.objects.iter().enumerate() {
                if let Some(ref mesh) = obj.mesh {
                    println!(
                        "  Object {}: '{}' \u{2014} {} vertices, {} triangles",
                        i,
                        obj.name.as_deref().unwrap_or("<unnamed>"),
                        mesh.vertices.len(),
                        mesh.triangles.len()
                    );
                }
            }
            println!();
            m
        }
        Err(e) => {
            eprintln!("Failed to load 3MF file: {}", e);
            return;
        }
    };

    // ----- Extract mesh data -----
    let vertices = extract_mesh_data(&model);
    if vertices.is_empty() {
        eprintln!("No renderable mesh data found in the 3MF file");
        return;
    }
    let triangle_count = vertices.len() / 3;
    println!("Mesh data extracted:");
    println!(
        "  {} triangles \u{2192} {} vertices (flat shading)",
        triangle_count,
        vertices.len()
    );

    let (center, scale) = compute_normalisation(&vertices);
    println!(
        "  Bounding box centre: ({:.2}, {:.2}, {:.2}), scale: {:.4}",
        center.x, center.y, center.z, scale
    );
    println!();

    // ----- Create GPU device -----
    let device_queue = pollster::block_on(create_device());
    let Some((device, queue)) = device_queue else {
        eprintln!("Failed to create GPU device");
        return;
    };
    println!("\u{2713} GPU device created\n");

    // ----- Upload geometry -----
    let vertex_data = bytemuck::cast_slice(&vertices);
    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("3MF Vertex Buffer"),
        contents: vertex_data,
        usage: wgpu::BufferUsages::VERTEX,
    });
    println!(
        "\u{2713} Vertex buffer created ({} bytes)",
        vertex_data.len()
    );

    // ----- Load shader -----
    let shader = ShaderModule::from_file("threedmf_viewer.wgsl", Some("threedmf_viewer"))
        .expect("Failed to load 3MF viewer shader");
    let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("threedmf_viewer"),
        source: wgpu::ShaderSource::Wgsl(shader.source().into()),
    });
    println!("\u{2713} Shader loaded and compiled");

    // ----- Create uniform buffer -----
    let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("3MF Uniform Buffer"),
        size: std::mem::size_of::<Uniforms>() as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("3MF Bind Group Layout"),
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

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("3MF Bind Group"),
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: uniform_buffer.as_entire_binding(),
        }],
    });

    // ----- Create render pipeline -----
    let vertex_buffer_layout = wgpu::VertexBufferLayout {
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
    };

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("3MF Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        immediate_size: 0,
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("threedmf_pipeline"),
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
    println!("\u{2713} Render pipeline created\n");

    // ----- Render targets -----
    let width = 800;
    let height = 600;
    let aspect_ratio = width as f32 / height as f32;

    let render_texture = create_render_texture(&device, width, height);
    let render_view = render_texture.create_view(&wgpu::TextureViewDescriptor::default());
    let depth_texture = create_depth_texture(&device, width, height);
    let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
    println!("\u{2713} Render targets created ({}x{})\n", width, height);

    // ----- Build transforms -----
    // Centre and scale the model so it fits in clip space
    let model_matrix = Mat4::from_scale(Vec3::splat(scale)) * Mat4::from_translation(-center);

    // Camera positioned to look at the origin from slightly above and in front
    let camera_pos = Vec3::new(0.0, 1.5, 3.5);
    let view = Mat4::look_at_rh(camera_pos, Vec3::ZERO, Vec3::Y);
    let proj = Mat4::perspective_rh(45.0_f32.to_radians(), aspect_ratio, 0.01, 100.0);
    let model_view_proj = proj * view * model_matrix;

    // Directional light coming from above-left
    let light_dir = Vec3::new(-0.5, -1.0, -0.5).normalize();

    let uniforms = Uniforms {
        model_view_proj: model_view_proj.to_cols_array_2d(),
        model: model_matrix.to_cols_array_2d(),
        light_dir: [light_dir.x, light_dir.y, light_dir.z, 0.0],
    };
    queue.write_buffer(&uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
    println!("\u{2713} Uniforms uploaded");

    // ----- Render -----
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("3MF Render Encoder"),
    });

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("3MF Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &render_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.08,
                        g: 0.08,
                        b: 0.1,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
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
            multiview_mask: None,
        });

        render_pass.set_pipeline(&pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw(0..vertices.len() as u32, 0..1);
    }

    queue.submit(std::iter::once(encoder.finish()));
    let _ = device.poll(wgpu::PollType::Wait {
        submission_index: None,
        timeout: None,
    });

    println!("\u{2713} Render commands submitted and completed\n");

    println!("=== 3MF Viewer Example Complete ===");
    println!("\nThe 3MF model was successfully loaded and rendered with:");
    println!("  \u{2022} lib3mf crate for 3MF file parsing");
    println!("  \u{2022} {} triangles from the mesh", triangle_count);
    println!("  \u{2022} Per-face normals computed from triangle edges");
    println!("  \u{2022} Depth testing with Depth24Plus format");
    println!("  \u{2022} Back-face culling");
    println!("  \u{2022} Directional lighting with ambient component");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_size() {
        // 3 * f32 (position) + 3 * f32 (normal) = 24 bytes
        assert_eq!(std::mem::size_of::<Vertex>(), 24);
    }

    #[test]
    fn test_uniforms_size() {
        // 4x4 f32 (mvp) + 4x4 f32 (model) + 4 * f32 (light_dir) = 144 bytes
        assert_eq!(std::mem::size_of::<Uniforms>(), 144);
    }

    #[test]
    fn test_vertices_are_pod() {
        let v = Vertex {
            position: [1.0, 2.0, 3.0],
            normal: [0.0, 1.0, 0.0],
        };
        let bytes = bytemuck::bytes_of(&v);
        assert_eq!(bytes.len(), std::mem::size_of::<Vertex>());
    }

    #[test]
    fn test_extract_mesh_data_empty_model() {
        let model = Model::new();
        let vertices = extract_mesh_data(&model);
        assert!(vertices.is_empty());
    }

    #[test]
    fn test_extract_mesh_data_single_triangle() {
        use lib3mf::{BuildItem, Mesh, Object, Triangle, Vertex as V3mfVertex};

        let mut model = Model::new();
        let mut mesh = Mesh::new();

        mesh.vertices.push(V3mfVertex::new(0.0, 0.0, 0.0));
        mesh.vertices.push(V3mfVertex::new(1.0, 0.0, 0.0));
        mesh.vertices.push(V3mfVertex::new(0.5, 1.0, 0.0));
        mesh.triangles.push(Triangle::new(0, 1, 2));

        let mut obj = Object::new(1);
        obj.mesh = Some(mesh);
        model.resources.objects.push(obj);
        model.build.items.push(BuildItem::new(1));

        let vertices = extract_mesh_data(&model);
        // One triangle = 3 vertices
        assert_eq!(vertices.len(), 3);
        // Normal should point in the +Z direction for a CCW triangle in the XY plane
        for v in &vertices {
            assert!((v.normal[2] - 1.0_f32).abs() < 1e-5);
        }
    }

    #[test]
    fn test_compute_normalisation_empty() {
        let (center, scale) = compute_normalisation(&[]);
        assert_eq!(center, Vec3::ZERO);
        assert_eq!(scale, 1.0);
    }

    #[test]
    fn test_compute_normalisation_cube() {
        // A 10x10x10 cube centred at (5, 5, 5)
        let vertices = vec![
            Vertex {
                position: [0.0, 0.0, 0.0],
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [10.0, 10.0, 10.0],
                normal: [0.0, 0.0, 1.0],
            },
        ];
        let (center, scale) = compute_normalisation(&vertices);
        assert!((center.x - 5.0).abs() < 1e-5);
        assert!((center.y - 5.0).abs() < 1e-5);
        assert!((center.z - 5.0).abs() < 1e-5);
        // max_extent = 10.0, scale = 2.0 / 10.0 = 0.2
        assert!((scale - 0.2_f32).abs() < 1e-5);
    }

    #[test]
    fn test_load_3mf_model() {
        let model = load_3mf_model("cube.3mf");
        assert!(
            model.is_ok(),
            "Expected cube.3mf to load: {:?}",
            model.err()
        );
        let model = model.unwrap();
        assert_eq!(model.resources.objects.len(), 1);
        let mesh = model.resources.objects[0].mesh.as_ref().unwrap();
        assert_eq!(mesh.vertices.len(), 8);
        assert_eq!(mesh.triangles.len(), 12);
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
