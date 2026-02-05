/// GPU Particle System Example
///
/// This example demonstrates a complete GPU particle system using:
/// - Compute shaders to update particle physics (position, velocity, lifetime)
/// - Storage buffers shared between compute and render pipelines
/// - Instanced rendering to efficiently draw thousands of particles
/// - Dynamic buffer updates to inject new particles
///
/// WebGPU APIs Exercised:
/// - Compute pipelines for particle simulation
/// - Storage buffers (STORAGE + VERTEX usage for sharing)
/// - Instanced rendering with per-instance attributes
/// - Dynamic buffer updates via queue.write_buffer
/// - Multiple passes (compute then render) in a single frame
///
/// Run with: cargo run --package wgpu_playground_examples --example particle_system
use wgpu::util::DeviceExt;
use std::f32::consts::PI;

/// Number of particles in the system
const NUM_PARTICLES: u32 = 10000;

/// Workgroup size for compute shader (must match shader)
const WORKGROUP_SIZE: u32 = 256;

/// Time step for physics simulation
const TIME_STEP: f32 = 0.016; // ~60 FPS

/// Particle structure shared between CPU and GPU
/// This must match the structure in both compute and render shaders
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Particle {
    position: [f32; 2],  // x, y position in clip space [-1, 1]
    velocity: [f32; 2],  // x, y velocity
    color: [f32; 4],     // r, g, b, a
    lifetime: f32,       // remaining lifetime in seconds
    size: f32,           // particle size multiplier
    _padding: [f32; 2],  // padding to 48 bytes for alignment
}

// Safety: Particle is repr(C) with only f32 fields
unsafe impl bytemuck::Pod for Particle {}
unsafe impl bytemuck::Zeroable for Particle {}

impl Particle {
    /// Create a new particle with given parameters
    fn new(
        position: [f32; 2],
        velocity: [f32; 2],
        color: [f32; 4],
        lifetime: f32,
        size: f32,
    ) -> Self {
        Self {
            position,
            velocity,
            color,
            lifetime,
            size,
            _padding: [0.0; 2],
        }
    }

    /// Create a dead particle (lifetime = 0)
    fn dead() -> Self {
        Self {
            position: [0.0, 0.0],
            velocity: [0.0, 0.0],
            color: [0.0, 0.0, 0.0, 0.0],
            lifetime: 0.0,
            size: 0.0,
            _padding: [0.0; 2],
        }
    }

    /// Check if particle is alive
    fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }
}

/// Simulation parameters passed to compute shader
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct SimulationParams {
    delta_time: f32,
    gravity: f32,
    damping: f32,
    max_lifetime: f32,
}

unsafe impl bytemuck::Pod for SimulationParams {}
unsafe impl bytemuck::Zeroable for SimulationParams {}

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
    println!("Backend: {:?}\n", adapter.get_info().backend);

    adapter
        .request_device(&wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            label: Some("Particle System Device"),
            memory_hints: Default::default(),
            experimental_features: Default::default(),
            trace: Default::default(),
        })
        .await
        .ok()
}

/// Initialize particles - start with all particles dead
fn initialize_particles() -> Vec<Particle> {
    vec![Particle::dead(); NUM_PARTICLES as usize]
}

/// Spawn new particles at the emitter position
fn spawn_particles(
    particles: &mut [Particle],
    count: usize,
    emitter_pos: [f32; 2],
    emitter_angle: f32,
) -> usize {
    let mut spawned = 0;
    
    for particle in particles.iter_mut() {
        if spawned >= count {
            break;
        }
        
        if !particle.is_alive() {
            // Generate random angle within cone
            let angle_variance = PI / 6.0; // 30 degree cone
            let angle = emitter_angle + (rand::random::<f32>() - 0.5) * angle_variance;
            
            // Random speed
            let speed = 0.3 + rand::random::<f32>() * 0.3;
            
            // Random color variation (warm colors: red to yellow)
            let color_variation = rand::random::<f32>();
            let color = [
                1.0,
                0.5 + color_variation * 0.5,
                0.2,
                1.0,
            ];
            
            // Random lifetime
            let lifetime = 2.0 + rand::random::<f32>() * 2.0;
            
            // Random size
            let size = 0.003 + rand::random::<f32>() * 0.003;
            
            *particle = Particle::new(
                emitter_pos,
                [angle.cos() * speed, angle.sin() * speed],
                color,
                lifetime,
                size,
            );
            
            spawned += 1;
        }
    }
    
    spawned
}

/// Create a render texture for rendering to
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

fn main() {
    env_logger::init();
    
    println!("=== GPU Particle System Example ===\n");
    println!("This example demonstrates:");
    println!("  • Compute shader for particle physics simulation");
    println!("  • Storage buffers shared between compute and render");
    println!("  • Instanced rendering for efficient particle display");
    println!("  • Dynamic particle spawning with queue.write_buffer\n");
    
    // Create device and queue
    let device_queue = pollster::block_on(create_device());
    if device_queue.is_none() {
        eprintln!("Failed to create GPU device");
        return;
    }
    
    let (device, queue) = device_queue.unwrap();
    println!("✓ GPU device created\n");
    
    // Initialize particle data
    let mut particles = initialize_particles();
    println!("Initialized {} particles (all dead initially)", NUM_PARTICLES);
    
    // Create shared particle buffer (STORAGE for compute + VERTEX for rendering)
    let particle_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Particle Buffer"),
        contents: bytemuck::cast_slice(&particles),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::VERTEX
            | wgpu::BufferUsages::COPY_DST,
    });
    
    println!("✓ Created particle buffer");
    println!("  - Size: {} bytes ({} particles × {} bytes)",
        particles.len() * std::mem::size_of::<Particle>(),
        particles.len(),
        std::mem::size_of::<Particle>()
    );
    println!("  - Usage: STORAGE (compute access) + VERTEX (render access) + COPY_DST (updates)\n");
    
    // Create simulation parameters buffer
    let sim_params = SimulationParams {
        delta_time: TIME_STEP,
        gravity: -0.5,
        damping: 0.98,
        max_lifetime: 4.0, // Maximum expected lifetime (matches spawn range)
    };
    
    let params_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Simulation Parameters"),
        contents: bytemuck::cast_slice(&[sim_params]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });
    
    println!("✓ Created simulation parameters buffer");
    println!("  - Delta time: {} seconds", sim_params.delta_time);
    println!("  - Gravity: {}", sim_params.gravity);
    println!("  - Damping: {}", sim_params.damping);
    println!("  - Max lifetime: {} seconds\n", sim_params.max_lifetime);
    
    // === COMPUTE PIPELINE SETUP ===
    println!("Setting up compute pipeline...");
    
    let compute_shader_source = include_str!("../shaders/particle_system_compute.wgsl");
    
    let compute_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Particle Compute Shader"),
        source: wgpu::ShaderSource::Wgsl(compute_shader_source.into()),
    });
    
    // Compute bind group layout
    let compute_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Compute Bind Group Layout"),
            entries: &[
                // Particle buffer (read-write)
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Simulation parameters (read-only)
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
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
        entry_point: Some("main"),
        compilation_options: Default::default(),
        cache: None,
    });
    
    let compute_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Compute Bind Group"),
        layout: &compute_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: particle_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: params_buffer.as_entire_binding(),
            },
        ],
    });
    
    println!("✓ Compute pipeline created\n");
    
    // === RENDER PIPELINE SETUP ===
    println!("Setting up render pipeline...");
    
    let render_shader_source = include_str!("../shaders/particle_system_render.wgsl");
    
    let render_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Particle Render Shader"),
        source: wgpu::ShaderSource::Wgsl(render_shader_source.into()),
    });
    
    // Vertex buffer layout for quad vertices (instanced rendering)
    // We'll use a simple quad and instance it for each particle
    let quad_vertices: &[f32] = &[
        -1.0, -1.0,  // bottom-left
         1.0, -1.0,  // bottom-right
         1.0,  1.0,  // top-right
        -1.0,  1.0,  // top-left
    ];
    
    let quad_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Quad Vertex Buffer"),
        contents: bytemuck::cast_slice(quad_vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });
    
    let quad_indices: &[u16] = &[0, 1, 2, 0, 2, 3];
    
    let quad_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Quad Index Buffer"),
        contents: bytemuck::cast_slice(quad_indices),
        usage: wgpu::BufferUsages::INDEX,
    });
    
    // Vertex buffer layout for quad (per-vertex)
    let quad_layout = wgpu::VertexBufferLayout {
        array_stride: 2 * std::mem::size_of::<f32>() as u64,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &[wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x2,
            offset: 0,
            shader_location: 0,
        }],
    };
    
    // Particle buffer layout (per-instance)
    let particle_layout = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Particle>() as u64,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &[
            // Position
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 0,
                shader_location: 1,
            },
            // Velocity (skip in rendering, but part of struct)
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: 8,
                shader_location: 2,
            },
            // Color
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x4,
                offset: 16,
                shader_location: 3,
            },
            // Lifetime
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32,
                offset: 32,
                shader_location: 4,
            },
            // Size
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32,
                offset: 36,
                shader_location: 5,
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
            entry_point: Some("vs_main"),
            compilation_options: Default::default(),
            buffers: &[quad_layout, particle_layout],
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
            module: &render_shader,
            entry_point: Some("fs_main"),
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
    println!("  - Using instanced rendering (one quad per particle)");
    println!("  - Alpha blending enabled for smooth particles\n");
    
    // Create render texture
    let render_texture = create_render_texture(&device, 800, 600);
    let texture_view = render_texture.create_view(&wgpu::TextureViewDescriptor::default());
    
    // === SIMULATION LOOP ===
    println!("Running particle simulation...\n");
    
    let num_frames = 10;
    for frame in 0..num_frames {
        println!("Frame {}:", frame);
        
        // Spawn new particles every frame
        if frame % 2 == 0 {
            // Spawn from bottom center, shooting upward
            let spawned = spawn_particles(
                &mut particles,
                50,
                [0.0, -0.8],
                PI / 2.0, // straight up
            );
            
            if spawned > 0 {
                // Update buffer with new particles
                queue.write_buffer(&particle_buffer, 0, bytemuck::cast_slice(&particles));
                println!("  ✓ Spawned {} new particles", spawned);
            }
        }
        
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
            
            // Calculate number of workgroups needed to process all particles (rounded up)
            // Each workgroup processes WORKGROUP_SIZE particles
            let workgroup_count = NUM_PARTICLES.div_ceil(WORKGROUP_SIZE);
            compute_pass.dispatch_workgroups(workgroup_count, 1, 1);
        }
        println!("  ✓ Compute pass: Updated particle physics");
        
        // 2. Render particles
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some(&format!("Frame {} Render Pass", frame)),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
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
            
            render_pass.set_pipeline(&render_pipeline);
            render_pass.set_vertex_buffer(0, quad_buffer.slice(..));
            render_pass.set_vertex_buffer(1, particle_buffer.slice(..));
            render_pass.set_index_buffer(quad_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..6, 0, 0..NUM_PARTICLES);
        }
        
        let alive_count = particles.iter().filter(|p| p.is_alive()).count();
        println!("  ✓ Render pass: Drew {} active particles (instanced)", alive_count);
        
        // Submit commands
        queue.submit(std::iter::once(encoder.finish()));
        let _ = device.poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: None,
        });
        println!("  ✓ Frame complete\n");
    }
    
    println!("=== Particle System Example Complete ===\n");
    println!("Successfully demonstrated:");
    println!("  ✓ Compute shader particle physics simulation");
    println!("    - Position and velocity integration");
    println!("    - Gravity and damping forces");
    println!("    - Lifetime management");
    println!("  ✓ Storage buffer sharing (STORAGE + VERTEX usage)");
    println!("  ✓ Instanced rendering ({} instances per frame)", NUM_PARTICLES);
    println!("  ✓ Dynamic particle spawning with queue.write_buffer");
    println!("  ✓ Compute-to-render pipeline integration");
    println!("\nKey WebGPU Concepts:");
    println!("  • Compute pipelines: GPU-accelerated physics simulation");
    println!("  • Storage buffers: Shared data between compute and render");
    println!("  • Instanced rendering: Efficient drawing of many objects");
    println!("  • Dynamic updates: CPU-to-GPU data transfer per frame");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_particle_size() {
        // Verify particle structure size (should be multiple of 16 for alignment)
        let size = std::mem::size_of::<Particle>();
        assert_eq!(size, 48);
        assert_eq!(size % 16, 0);
    }
    
    #[test]
    fn test_particle_creation() {
        let p = Particle::new(
            [1.0, 2.0],
            [0.5, 0.5],
            [1.0, 0.0, 0.0, 1.0],
            3.0,
            0.01,
        );
        assert_eq!(p.position, [1.0, 2.0]);
        assert_eq!(p.velocity, [0.5, 0.5]);
        assert_eq!(p.color, [1.0, 0.0, 0.0, 1.0]);
        assert_eq!(p.lifetime, 3.0);
        assert_eq!(p.size, 0.01);
    }
    
    #[test]
    fn test_dead_particle() {
        let p = Particle::dead();
        assert!(!p.is_alive());
        assert_eq!(p.lifetime, 0.0);
    }
    
    #[test]
    fn test_particle_lifetime() {
        let alive = Particle::new([0.0, 0.0], [0.0, 0.0], [1.0, 1.0, 1.0, 1.0], 1.0, 0.01);
        let dead = Particle::new([0.0, 0.0], [0.0, 0.0], [1.0, 1.0, 1.0, 1.0], 0.0, 0.01);
        
        assert!(alive.is_alive());
        assert!(!dead.is_alive());
    }
    
    #[test]
    fn test_particles_are_pod() {
        let particles = vec![
            Particle::new([0.0, 0.5], [0.1, 0.1], [1.0, 0.0, 0.0, 1.0], 2.0, 0.01),
            Particle::new([-0.5, -0.5], [0.1, -0.1], [0.0, 1.0, 0.0, 1.0], 3.0, 0.015),
        ];
        
        let bytes = bytemuck::cast_slice::<Particle, u8>(&particles);
        assert_eq!(bytes.len(), 2 * std::mem::size_of::<Particle>());
    }
    
    #[test]
    fn test_initialize_particles() {
        let particles = initialize_particles();
        assert_eq!(particles.len(), NUM_PARTICLES as usize);
        
        // All should be dead initially
        for particle in particles.iter() {
            assert!(!particle.is_alive());
        }
    }
    
    #[test]
    fn test_spawn_particles() {
        let mut particles = initialize_particles();
        
        // Spawn 10 particles
        let spawned = spawn_particles(&mut particles, 10, [0.0, 0.0], 0.0);
        assert_eq!(spawned, 10);
        
        // Count alive particles
        let alive = particles.iter().filter(|p| p.is_alive()).count();
        assert_eq!(alive, 10);
    }
    
    #[test]
    fn test_simulation_params_size() {
        assert_eq!(std::mem::size_of::<SimulationParams>(), 16);
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
