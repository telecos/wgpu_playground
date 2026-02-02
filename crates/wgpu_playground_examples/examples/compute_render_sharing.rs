/// Compute-Render Buffer Sharing Example
///
/// This example demonstrates:
/// - Creating a compute shader that processes array data (particle simulation)
/// - Creating a render pipeline that visualizes the computed data
/// - Sharing a buffer between compute and render pipelines
/// - Using STORAGE + VERTEX buffer usage flags for sharing
///
/// The compute shader updates particle positions based on simple physics,
/// and the render pipeline draws the particles as points.
///
/// Run with: cargo run --package wgpu_playground_examples --example compute_render_sharing
use wgpu::util::DeviceExt;

/// Workgroup size used in the compute shader
/// This must match the @workgroup_size annotation in the compute shader
const WORKGROUP_SIZE: u32 = 64;

/// Particle structure used by both compute and render shaders
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Particle {
    position: [f32; 2], // x, y position
    velocity: [f32; 2], // x, y velocity
    color: [f32; 4],    // r, g, b, a color
}

// Safety: Particle is repr(C) with simple primitive types
unsafe impl bytemuck::Pod for Particle {}
unsafe impl bytemuck::Zeroable for Particle {}

impl Particle {
    fn new(position: [f32; 2], velocity: [f32; 2], color: [f32; 4]) -> Self {
        Self {
            position,
            velocity,
            color,
        }
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
    println!("Backend: {:?}\n", adapter.get_info().backend);

    adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: Some("Compute-Render Device"),
                memory_hints: Default::default(),
            },
            None,
        )
        .await
        .ok()
}

/// Create a render texture for rendering to
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

    println!("=== Compute-Render Buffer Sharing Example ===\n");

    // Create device and queue
    let device_queue = pollster::block_on(create_device());
    if device_queue.is_none() {
        eprintln!("Failed to create GPU device");
        return;
    }

    let (device, queue) = device_queue.unwrap();

    // Initialize particle data
    let num_particles = 1024;
    let mut particles = Vec::with_capacity(num_particles);

    for i in 0..num_particles {
        let angle = (i as f32 / num_particles as f32) * std::f32::consts::TAU; // TAU = 2π (full circle)
        let radius = 0.3;
        particles.push(Particle::new(
            [angle.cos() * radius, angle.sin() * radius],
            [angle.cos() * 0.001, angle.sin() * 0.001],
            [
                (i as f32 / num_particles as f32),
                1.0 - (i as f32 / num_particles as f32),
                0.5,
                1.0,
            ],
        ));
    }

    println!("Created {} particles arranged in a circle", num_particles);
    println!("  - Each particle has position, velocity, and color");
    println!("  - Particles will be updated by compute shader");
    println!("  - Same buffer will be used for vertex data in render pass\n");

    // Create shared buffer with STORAGE + VERTEX usage
    // This allows the buffer to be used by both compute and render pipelines
    let particle_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Particle Buffer"),
        contents: bytemuck::cast_slice(&particles),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::VERTEX
            | wgpu::BufferUsages::COPY_DST,
    });

    println!("✓ Created shared buffer with STORAGE + VERTEX usage");
    println!(
        "  - Size: {} bytes ({} particles)",
        particles.len() * std::mem::size_of::<Particle>(),
        particles.len()
    );
    println!("  - STORAGE flag: allows compute shader access");
    println!("  - VERTEX flag: allows render pipeline access\n");

    // === COMPUTE PIPELINE SETUP ===
    println!("Setting up compute pipeline...");

    // Create compute shader that updates particle positions
    let compute_shader_source = r#"
struct Particle {
    position: vec2<f32>,
    velocity: vec2<f32>,
    color: vec4<f32>,
}

@group(0) @binding(0)
var<storage, read_write> particles: array<Particle>;

@compute @workgroup_size(64)  // Must match WORKGROUP_SIZE constant
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if (index >= arrayLength(&particles)) {
        return;
    }

    // Simple physics: update position based on velocity
    particles[index].position += particles[index].velocity;

    // Bounce off walls (simple boundary check)
    if (abs(particles[index].position.x) > 0.9) {
        particles[index].velocity.x *= -1.0;
    }
    if (abs(particles[index].position.y) > 0.9) {
        particles[index].velocity.y *= -1.0;
    }

    // Add a slight circular motion
    let angle = 0.01;
    let cos_a = cos(angle);
    let sin_a = sin(angle);
    let old_vel = particles[index].velocity;
    particles[index].velocity.x = old_vel.x * cos_a - old_vel.y * sin_a;
    particles[index].velocity.y = old_vel.x * sin_a + old_vel.y * cos_a;
}
"#;

    let compute_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Particle Compute Shader"),
        source: wgpu::ShaderSource::Wgsl(compute_shader_source.into()),
    });

    // Create bind group layout for compute pipeline
    let compute_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Compute Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

    let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Compute Pipeline Layout"),
        bind_group_layouts: &[&compute_bind_group_layout],
        push_constant_ranges: &[],
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Particle Compute Pipeline"),
        layout: Some(&compute_pipeline_layout),
        module: &compute_shader,
        entry_point: "main",
        compilation_options: Default::default(),
        cache: None,
    });

    let compute_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Compute Bind Group"),
        layout: &compute_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: particle_buffer.as_entire_binding(),
        }],
    });

    println!("✓ Compute pipeline created");
    println!("  - Updates particle positions based on velocity");
    println!("  - Applies boundary collision detection");
    println!("  - Adds circular motion effect\n");

    // === RENDER PIPELINE SETUP ===
    println!("Setting up render pipeline...");

    // Create render shader that draws particles as colored points
    let render_shader_source = r#"
struct Particle {
    @location(0) position: vec2<f32>,
    @location(1) velocity: vec2<f32>,
    @location(2) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(particle: Particle) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(particle.position, 0.0, 1.0);
    out.color = particle.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
"#;

    let render_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Particle Render Shader"),
        source: wgpu::ShaderSource::Wgsl(render_shader_source.into()),
    });

    // Define vertex buffer layout for particles
    let vertex_buffer_layout = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Particle>() as u64,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[
            // Position attribute
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 0,
                shader_location: 0,
            },
            // Velocity attribute (not used in rendering, but part of structure)
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 8,
                shader_location: 1,
            },
            // Color attribute
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x4,
                offset: 16,
                shader_location: 2,
            },
        ],
    };

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Particle Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &render_shader,
            entry_point: "vs_main",
            compilation_options: Default::default(),
            buffers: &[vertex_buffer_layout],
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::PointList,
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
            module: &render_shader,
            entry_point: "fs_main",
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview: None,
        cache: None,
    });

    println!("✓ Render pipeline created");
    println!("  - Draws particles as points (PointList topology)");
    println!("  - Uses particle position and color from buffer");
    println!("  - Alpha blending enabled for nice visual effect\n");

    // === EXECUTE COMPUTE AND RENDER PASSES ===
    println!("Executing compute and render operations...\n");

    // Create render texture
    let render_texture = create_render_texture(&device);
    let texture_view = render_texture.create_view(&wgpu::TextureViewDescriptor::default());

    // Simulate 3 frames to show the compute shader in action
    for frame in 0..3 {
        println!("Frame {}:", frame);

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some(&format!("Frame {} Encoder", frame)),
        });

        // 1. Run compute shader to update particles
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some(&format!("Frame {} Compute Pass", frame)),
                timestamp_writes: None,
            });

            compute_pass.set_pipeline(&compute_pipeline);
            compute_pass.set_bind_group(0, &compute_bind_group, &[]);

            // Calculate number of workgroups needed to cover all particles
            let workgroup_count = (num_particles as u32).div_ceil(WORKGROUP_SIZE);
            compute_pass.dispatch_workgroups(workgroup_count, 1, 1);
        }
        println!("  ✓ Compute pass: Updated particle positions");

        // 2. Render the particles using the same buffer
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some(&format!("Frame {} Render Pass", frame)),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
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

            render_pass.set_pipeline(&render_pipeline);
            render_pass.set_vertex_buffer(0, particle_buffer.slice(..));
            render_pass.draw(0..num_particles as u32, 0..1);
        }
        println!(
            "  ✓ Render pass: Drew {} particles as points",
            num_particles
        );

        // Submit commands
        queue.submit(std::iter::once(encoder.finish()));
        device.poll(wgpu::Maintain::Wait);
        println!("  ✓ Commands submitted and completed\n");
    }

    println!("=== Example Complete ===\n");
    println!("This example demonstrated:");
    println!("  ✓ Creating a shared buffer with STORAGE + VERTEX usage flags");
    println!("  ✓ Compute shader that processes particle data");
    println!("  ✓ Render pipeline that visualizes the same data");
    println!("  ✓ Buffer sharing between compute and render pipelines");
    println!("  ✓ Multiple frames showing data updates from compute shader");
    println!("\nKey Concepts:");
    println!("  • STORAGE usage: Enables read/write access in compute shaders");
    println!("  • VERTEX usage: Enables use as vertex buffer in render pipeline");
    println!("  • Combined usage allows seamless data flow from compute to render");
    println!("  • Same buffer used without copying, maximizing GPU efficiency");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_size() {
        // Verify particle structure has correct size
        // 2*f32 + 2*f32 + 4*f32 = 32 bytes
        assert_eq!(std::mem::size_of::<Particle>(), 32);
    }

    #[test]
    fn test_particle_creation() {
        let p = Particle::new([1.0, 2.0], [0.5, 0.5], [1.0, 0.0, 0.0, 1.0]);
        assert_eq!(p.position, [1.0, 2.0]);
        assert_eq!(p.velocity, [0.5, 0.5]);
        assert_eq!(p.color, [1.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_particles_are_pod() {
        // Test that particles can be cast to bytes
        let particles = vec![
            Particle::new([0.0, 0.5], [0.1, 0.1], [1.0, 0.0, 0.0, 1.0]),
            Particle::new([-0.5, -0.5], [0.1, -0.1], [0.0, 1.0, 0.0, 1.0]),
        ];

        let bytes = bytemuck::cast_slice::<Particle, u8>(&particles);
        assert_eq!(bytes.len(), 2 * std::mem::size_of::<Particle>());
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
