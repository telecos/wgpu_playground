/// Indirect Drawing Example
///
/// This example demonstrates GPU-driven rendering using indirect draw commands:
/// - Creating indirect buffers with INDIRECT usage flag
/// - Using compute shaders to generate draw parameters on the GPU
/// - drawIndirect for non-indexed drawing
/// - drawIndexedIndirect for indexed drawing
/// - Buffer-to-buffer copies for parameter management
///
/// Key WebGPU APIs demonstrated:
/// - wgpu::BufferUsages::INDIRECT
/// - RenderPass::draw_indirect()
/// - RenderPass::draw_indexed_indirect()
/// - ComputePass for generating draw parameters
/// - Buffer copies for indirect parameter management
///
/// Run with: cargo run --package wgpu_playground_examples --example indirect_drawing
use wgpu::util::DeviceExt;
use wgpu_playground_core::buffer::{BufferDescriptor, BufferUsages};
use wgpu_playground_core::command_encoder::CommandEncoderOps;
use wgpu_playground_core::compute::ComputePipelineDescriptor;
use wgpu_playground_core::compute_pass_encoder::{ComputePassDescriptor, ComputePassEncoder};
use wgpu_playground_core::render_pass_encoder::{
    Color, RenderPassColorAttachment, RenderPassDescriptor, RenderPassEncoder,
};
use wgpu_playground_core::shader::ShaderModule;

/// Vertex structure with position and color
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

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
            label: Some("Indirect Drawing Device"),
            memory_hints: Default::default(),
            experimental_features: Default::default(),
            trace: Default::default(),
        })
        .await
        .ok()
}

fn main() {
    env_logger::init();

    println!("=== Indirect Drawing Example ===\n");
    println!("This example demonstrates GPU-driven rendering where draw parameters");
    println!("are computed on the GPU and stored in indirect buffers.\n");

    let device_queue = pollster::block_on(create_device());
    if device_queue.is_none() {
        eprintln!("Failed to create GPU device");
        return;
    }

    let (device, queue) = device_queue.unwrap();
    println!("✓ GPU device created\n");

    // Run all examples
    example_draw_indirect(&device, &queue);
    example_draw_indexed_indirect(&device, &queue);
    example_compute_generated_params(&device, &queue);
    example_buffer_copy(&device, &queue);

    println!("\n=== All indirect drawing examples completed successfully ===");
}

/// Example 1: Basic drawIndirect usage
fn example_draw_indirect(device: &wgpu::Device, queue: &wgpu::Queue) {
    println!("Example 1: drawIndirect - Non-indexed drawing");
    println!("Drawing a triangle using indirect buffer for draw parameters\n");

    // Create vertices for a triangle
    let vertices = vec![
        Vertex::new([0.0, 0.5], [1.0, 0.0, 0.0]),   // Top - Red
        Vertex::new([-0.5, -0.5], [0.0, 1.0, 0.0]), // Bottom-left - Green
        Vertex::new([0.5, -0.5], [0.0, 0.0, 1.0]),  // Bottom-right - Blue
    ];

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Triangle Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });

    // Create indirect buffer with DrawIndirect parameters
    // DrawIndirect structure: { vertex_count: u32, instance_count: u32, first_vertex: u32, first_instance: u32 }
    let draw_params: [u32; 4] = [
        3, // vertex_count - draw 3 vertices
        1, // instance_count - 1 instance
        0, // first_vertex - start at vertex 0
        0, // first_instance - start at instance 0
    ];

    let indirect_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Draw Indirect Buffer"),
        contents: bytemuck::cast_slice(&draw_params),
        usage: wgpu::BufferUsages::INDIRECT,
    });

    println!("  Draw parameters in indirect buffer:");
    println!("    vertex_count: {}", draw_params[0]);
    println!("    instance_count: {}", draw_params[1]);
    println!("    first_vertex: {}", draw_params[2]);
    println!("    first_instance: {}", draw_params[3]);

    // Create shader and pipeline
    let shader = ShaderModule::from_file("indirect_drawing.wgsl", Some("indirect_shader"))
        .expect("Failed to load shader");

    let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("indirect_shader"),
        source: wgpu::ShaderSource::Wgsl(shader.source().into()),
    });

    let vertex_buffer_layout = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as u64,
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
    };

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Indirect Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("indirect_pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader_module,
            entry_point: Some("vs_main"),
            compilation_options: Default::default(),
            buffers: &[vertex_buffer_layout],
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
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

    // Create render texture
    let render_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Indirect Render Texture"),
        size: wgpu::Extent3d {
            width: 512,
            height: 512,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });

    let texture_view = render_texture.create_view(&wgpu::TextureViewDescriptor::default());

    // Render using drawIndirect
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Indirect Draw Encoder"),
    });

    let descriptor = RenderPassDescriptor::new()
        .with_label("Indirect Draw Pass")
        .with_color_attachment(RenderPassColorAttachment::clear(
            &texture_view,
            Color::new(0.1, 0.1, 0.1, 1.0),
        ));

    {
        let mut render_pass = RenderPassEncoder::begin(&mut encoder, &descriptor).unwrap();
        render_pass.set_pipeline(&pipeline);
        render_pass.set_vertex_buffer(0, &vertex_buffer, 0, None);
        
        // Use drawIndirect - parameters come from the indirect buffer
        render_pass.draw_indirect(&indirect_buffer, 0);
    }

    queue.submit(std::iter::once(encoder.finish()));
    let _ = device.poll(wgpu::PollType::Wait {
        submission_index: None,
        timeout: None,
    });

    println!("  ✓ Triangle rendered using drawIndirect");
    println!("  ✓ Draw parameters read from GPU buffer\n");
}

/// Example 2: drawIndexedIndirect usage
fn example_draw_indexed_indirect(device: &wgpu::Device, queue: &wgpu::Queue) {
    println!("Example 2: drawIndexedIndirect - Indexed drawing");
    println!("Drawing a quad (2 triangles) using indexed indirect buffer\n");

    // Create vertices for a quad
    let vertices = vec![
        Vertex::new([-0.5, 0.5], [1.0, 0.0, 0.0]),  // Top-left - Red
        Vertex::new([0.5, 0.5], [0.0, 1.0, 0.0]),   // Top-right - Green
        Vertex::new([0.5, -0.5], [0.0, 0.0, 1.0]),  // Bottom-right - Blue
        Vertex::new([-0.5, -0.5], [1.0, 1.0, 0.0]), // Bottom-left - Yellow
    ];

    // Indices for two triangles forming a quad
    let indices: [u16; 6] = [
        0, 1, 2, // First triangle
        0, 2, 3, // Second triangle
    ];

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Quad Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Quad Index Buffer"),
        contents: bytemuck::cast_slice(&indices),
        usage: wgpu::BufferUsages::INDEX,
    });

    // Create indirect buffer with DrawIndexedIndirect parameters
    // DrawIndexedIndirect: { index_count: u32, instance_count: u32, first_index: u32, base_vertex: i32, first_instance: u32 }
    let indexed_draw_params: [u32; 5] = [
        6, // index_count - draw 6 indices (2 triangles)
        1, // instance_count - 1 instance
        0, // first_index - start at index 0
        0, // base_vertex - no vertex offset
        0, // first_instance - start at instance 0
    ];

    let indirect_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Draw Indexed Indirect Buffer"),
        contents: bytemuck::cast_slice(&indexed_draw_params),
        usage: wgpu::BufferUsages::INDIRECT,
    });

    println!("  Draw parameters in indirect buffer:");
    println!("    index_count: {}", indexed_draw_params[0]);
    println!("    instance_count: {}", indexed_draw_params[1]);
    println!("    first_index: {}", indexed_draw_params[2]);
    println!("    base_vertex: {}", indexed_draw_params[3]);
    println!("    first_instance: {}", indexed_draw_params[4]);

    // Create shader and pipeline (reuse from previous example)
    let shader = ShaderModule::from_file("indirect_drawing.wgsl", Some("indirect_shader"))
        .expect("Failed to load shader");

    let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("indirect_shader"),
        source: wgpu::ShaderSource::Wgsl(shader.source().into()),
    });

    let vertex_buffer_layout = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as u64,
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
    };

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Indexed Indirect Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("indexed_indirect_pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader_module,
            entry_point: Some("vs_main"),
            compilation_options: Default::default(),
            buffers: &[vertex_buffer_layout],
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
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

    // Create render texture
    let render_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Indexed Indirect Render Texture"),
        size: wgpu::Extent3d {
            width: 512,
            height: 512,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });

    let texture_view = render_texture.create_view(&wgpu::TextureViewDescriptor::default());

    // Render using drawIndexedIndirect
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Indexed Indirect Draw Encoder"),
    });

    let descriptor = RenderPassDescriptor::new()
        .with_label("Indexed Indirect Draw Pass")
        .with_color_attachment(RenderPassColorAttachment::clear(
            &texture_view,
            Color::new(0.1, 0.1, 0.1, 1.0),
        ));

    {
        let mut render_pass = RenderPassEncoder::begin(&mut encoder, &descriptor).unwrap();
        render_pass.set_pipeline(&pipeline);
        render_pass.set_vertex_buffer(0, &vertex_buffer, 0, None);
        render_pass.set_index_buffer(&index_buffer, wgpu_playground_core::render_pass_encoder::IndexFormat::Uint16, 0, None);
        
        // Use drawIndexedIndirect - parameters come from the indirect buffer
        render_pass.draw_indexed_indirect(&indirect_buffer, 0);
    }

    queue.submit(std::iter::once(encoder.finish()));
    let _ = device.poll(wgpu::PollType::Wait {
        submission_index: None,
        timeout: None,
    });

    println!("  ✓ Quad rendered using drawIndexedIndirect");
    println!("  ✓ Draw parameters read from GPU buffer\n");
}

/// Example 3: Compute shader generating draw parameters
fn example_compute_generated_params(device: &wgpu::Device, queue: &wgpu::Queue) {
    println!("Example 3: GPU-generated draw parameters");
    println!("Using compute shader to generate indirect draw parameters on GPU\n");

    // Create compute shader for generating draw parameters
    let compute_shader =
        ShaderModule::from_file("indirect_drawing_compute.wgsl", Some("param_generator"))
            .expect("Failed to load compute shader");

    let compute_pipeline_descriptor =
        ComputePipelineDescriptor::new(Some("param_generation_pipeline"))
            .with_shader(compute_shader)
            .with_entry_point("generate_draw_params");

    let compute_pipeline = compute_pipeline_descriptor
        .create_pipeline(device)
        .unwrap();

    // Create storage buffers for draw parameters
    // These will be written by the compute shader
    let draw_buffer_descriptor = BufferDescriptor::new(
        Some("Compute-Generated Draw Buffer"),
        16, // 4 u32 values for DrawIndirect
        BufferUsages::STORAGE | BufferUsages::INDIRECT,
    );
    let draw_buffer = draw_buffer_descriptor.create_buffer(device).unwrap();

    let indexed_draw_buffer_descriptor = BufferDescriptor::new(
        Some("Compute-Generated Indexed Draw Buffer"),
        20, // 5 u32 values for DrawIndexedIndirect
        BufferUsages::STORAGE | BufferUsages::INDIRECT,
    );
    let indexed_draw_buffer = indexed_draw_buffer_descriptor.create_buffer(device).unwrap();

    // Create bind group for compute shader
    let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("param_generation_bind_group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: draw_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: indexed_draw_buffer.as_entire_binding(),
            },
        ],
    });

    // Execute compute shader to generate parameters
    let mut encoder_ops = CommandEncoderOps::new(device, Some("Param Generation Encoder"));
    let encoder = encoder_ops.inner_mut();

    let descriptor = ComputePassDescriptor::new().with_label("Parameter Generation Pass");

    {
        let mut compute_pass = ComputePassEncoder::begin(encoder, &descriptor).unwrap();
        compute_pass.set_pipeline(&compute_pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.dispatch(1, 1, 1); // Single workgroup to generate parameters
    }

    let command_buffer = encoder_ops.finish();
    queue.submit(std::iter::once(command_buffer));
    let _ = device.poll(wgpu::PollType::Wait {
        submission_index: None,
        timeout: None,
    });

    println!("  ✓ Compute shader generated draw parameters on GPU");
    println!("  ✓ DrawIndirect buffer: 3 vertices, 1 instance");
    println!("  ✓ DrawIndexedIndirect buffer: 6 indices, 1 instance");
    println!("  ✓ Buffers ready for use with draw_indirect/draw_indexed_indirect\n");
}

/// Example 4: Buffer-to-buffer copies for indirect parameters
fn example_buffer_copy(device: &wgpu::Device, queue: &wgpu::Queue) {
    println!("Example 4: Buffer-to-buffer copies");
    println!("Copying indirect draw parameters between buffers\n");

    // Create source buffer with draw parameters
    let source_params: [u32; 4] = [3, 2, 0, 0]; // 3 vertices, 2 instances
    let source_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Source Parameters"),
        contents: bytemuck::cast_slice(&source_params),
        usage: wgpu::BufferUsages::COPY_SRC,
    });

    // Create destination indirect buffer
    let dest_buffer_descriptor = BufferDescriptor::new(
        Some("Destination Indirect Buffer"),
        16,
        BufferUsages::INDIRECT | BufferUsages::COPY_DST,
    );
    let dest_buffer = dest_buffer_descriptor.create_buffer(device).unwrap();

    // Copy parameters from source to destination
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Buffer Copy Encoder"),
    });

    encoder.copy_buffer_to_buffer(&source_buffer, 0, &dest_buffer, 0, 16);

    queue.submit(std::iter::once(encoder.finish()));
    let _ = device.poll(wgpu::PollType::Wait {
        submission_index: None,
        timeout: None,
    });

    println!("  ✓ Copied draw parameters from source buffer to indirect buffer");
    println!("  ✓ Parameters: {} vertices, {} instances", source_params[0], source_params[1]);
    println!("  ✓ Destination buffer ready for indirect drawing\n");
}
