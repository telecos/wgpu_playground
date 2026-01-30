/// Example demonstrating compute pass operations
///
/// This example shows how to:
/// - Create a compute pipeline with a simple shader
/// - Set up storage buffers for input/output
/// - Use compute pass encoder to dispatch work
/// - Use dispatch_indirect for indirect dispatch
///
/// Run with: cargo run --example compute_pass
use wgpu_playground_core::buffer::{BufferDescriptor, BufferUsages};
use wgpu_playground_core::command_encoder::CommandEncoderOps;
use wgpu_playground_core::compute::ComputePipelineDescriptor;
use wgpu_playground_core::compute_pass_encoder::{ComputePassDescriptor, ComputePassEncoder};
use wgpu_playground_core::shader::ShaderModule;

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

    adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: Some("Compute Device"),
                memory_hints: Default::default(),
            },
            None,
        )
        .await
        .ok()
}

fn main() {
    env_logger::init();

    let device_queue = pollster::block_on(create_device());
    if device_queue.is_none() {
        eprintln!("Failed to create GPU device");
        return;
    }

    let (device, queue) = device_queue.unwrap();

    println!("=== Compute Pass Example ===\n");

    // Example 1: Simple compute shader that squares numbers
    example_square_numbers(&device, &queue);

    // Example 2: Compute shader with indirect dispatch
    example_indirect_dispatch(&device, &queue);

    // Example 3: Multiple dispatches in one pass
    example_multiple_dispatches(&device, &queue);

    println!("\n=== All examples completed successfully ===");
}

fn example_square_numbers(device: &wgpu::Device, queue: &wgpu::Queue) {
    println!("Example 1: Square Numbers Compute Shader");

    // Create a compute shader that squares each element
    let shader_source = r#"
@group(0) @binding(0)
var<storage, read> input: array<f32>;

@group(0) @binding(1)
var<storage, read_write> output: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if (index < arrayLength(&output)) {
        output[index] = input[index] * input[index];
    }
}
"#;

    let shader = ShaderModule::from_source(shader_source, Some("square_shader")).unwrap();

    let pipeline_descriptor = ComputePipelineDescriptor::new(Some("square_pipeline"))
        .with_shader(shader)
        .with_entry_point("main");

    let pipeline = pipeline_descriptor.create_pipeline(device).unwrap();

    // Create input data: [1.0, 2.0, 3.0, 4.0, ...]
    let input_data: Vec<f32> = (1..=256).map(|i| i as f32).collect();

    // Convert to bytes for buffer write
    let input_bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(
            input_data.as_ptr() as *const u8,
            input_data.len() * std::mem::size_of::<f32>(),
        )
    };

    // Create buffers
    let buffer_size = input_data.len() * std::mem::size_of::<f32>();

    let input_buffer_descriptor = BufferDescriptor::new(
        Some("Input Buffer"),
        buffer_size as u64,
        BufferUsages::STORAGE | BufferUsages::COPY_DST,
    );
    let input_buffer = input_buffer_descriptor.create_buffer(device).unwrap();

    let output_buffer_descriptor = BufferDescriptor::new(
        Some("Output Buffer"),
        buffer_size as u64,
        BufferUsages::STORAGE | BufferUsages::COPY_SRC,
    );
    let output_buffer = output_buffer_descriptor.create_buffer(device).unwrap();

    // Write input data to buffer
    queue.write_buffer(&input_buffer, 0, input_bytes);

    // Create bind group
    let bind_group_layout = pipeline.get_bind_group_layout(0);
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("square_bind_group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: input_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: output_buffer.as_entire_binding(),
            },
        ],
    });

    // Create command encoder and compute pass
    let mut encoder_ops = CommandEncoderOps::new(device, Some("Square Encoder"));
    let encoder = encoder_ops.inner_mut();

    let descriptor = ComputePassDescriptor::new().with_label("Square Compute Pass");

    {
        let mut compute_pass = ComputePassEncoder::begin(encoder, &descriptor).unwrap();
        compute_pass.set_pipeline(&pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        // Dispatch 4 workgroups (256 threads total, matching our data size)
        compute_pass.dispatch(4, 1, 1);
    }

    let command_buffer = encoder_ops.finish();
    queue.submit(std::iter::once(command_buffer));

    println!("  ✓ Dispatched compute shader to square 256 numbers");
    println!("  ✓ Used 4 workgroups with workgroup_size(64)\n");
}

fn example_indirect_dispatch(device: &wgpu::Device, queue: &wgpu::Queue) {
    println!("Example 2: Indirect Dispatch");

    // Create a simple compute shader
    let shader_source = r#"
@group(0) @binding(0)
var<storage, read_write> data: array<u32>;

@compute @workgroup_size(32)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if (index < arrayLength(&data)) {
        data[index] = index * 2u;
    }
}
"#;

    let shader = ShaderModule::from_source(shader_source, Some("indirect_shader")).unwrap();

    let pipeline_descriptor = ComputePipelineDescriptor::new(Some("indirect_pipeline"))
        .with_shader(shader)
        .with_entry_point("main");

    let pipeline = pipeline_descriptor.create_pipeline(device).unwrap();

    // Create storage buffer
    let buffer_size = 128 * std::mem::size_of::<u32>() as u64;
    let buffer_descriptor = BufferDescriptor::new(
        Some("Data Buffer"),
        buffer_size,
        BufferUsages::STORAGE | BufferUsages::COPY_SRC,
    );
    let storage_buffer = buffer_descriptor.create_buffer(device).unwrap();

    // Create indirect buffer with dispatch parameters
    // workgroups_x = 4, workgroups_y = 1, workgroups_z = 1
    let indirect_buffer_descriptor = BufferDescriptor::new(
        Some("Indirect Buffer"),
        12, // 3 u32 values
        BufferUsages::INDIRECT | BufferUsages::COPY_DST,
    );
    let indirect_buffer = indirect_buffer_descriptor.create_buffer(device).unwrap();

    // Write dispatch parameters (4 workgroups of 32 threads = 128 threads)
    let dispatch_params = [4u32, 1u32, 1u32];
    let dispatch_bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(
            dispatch_params.as_ptr() as *const u8,
            dispatch_params.len() * std::mem::size_of::<u32>(),
        )
    };
    queue.write_buffer(&indirect_buffer, 0, dispatch_bytes);

    // Create bind group
    let bind_group_layout = pipeline.get_bind_group_layout(0);
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("indirect_bind_group"),
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: storage_buffer.as_entire_binding(),
        }],
    });

    // Create command encoder and compute pass
    let mut encoder_ops = CommandEncoderOps::new(device, Some("Indirect Encoder"));
    let encoder = encoder_ops.inner_mut();

    let descriptor = ComputePassDescriptor::new().with_label("Indirect Compute Pass");

    {
        let mut compute_pass = ComputePassEncoder::begin(encoder, &descriptor).unwrap();
        compute_pass.set_pipeline(&pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        // Use indirect dispatch - parameters come from the indirect buffer
        compute_pass.dispatch_indirect(&indirect_buffer, 0);
    }

    let command_buffer = encoder_ops.finish();
    queue.submit(std::iter::once(command_buffer));

    println!("  ✓ Used dispatch_indirect with parameters from buffer");
    println!("  ✓ Dispatch parameters: (4, 1, 1) workgroups\n");
}

fn example_multiple_dispatches(device: &wgpu::Device, queue: &wgpu::Queue) {
    println!("Example 3: Multiple Dispatches");

    // Create a compute shader that increments values
    let shader_source = r#"
@group(0) @binding(0)
var<storage, read_write> data: array<u32>;

@compute @workgroup_size(16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if (index < arrayLength(&data)) {
        data[index] = data[index] + 1u;
    }
}
"#;

    let shader = ShaderModule::from_source(shader_source, Some("increment_shader")).unwrap();

    let pipeline_descriptor = ComputePipelineDescriptor::new(Some("increment_pipeline"))
        .with_shader(shader)
        .with_entry_point("main");

    let pipeline = pipeline_descriptor.create_pipeline(device).unwrap();

    // Create storage buffer
    let buffer_size = 64 * std::mem::size_of::<u32>() as u64;
    let buffer_descriptor = BufferDescriptor::new(
        Some("Data Buffer"),
        buffer_size,
        BufferUsages::STORAGE | BufferUsages::COPY_SRC,
    );
    let storage_buffer = buffer_descriptor.create_buffer(device).unwrap();

    // Create bind group
    let bind_group_layout = pipeline.get_bind_group_layout(0);
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("increment_bind_group"),
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: storage_buffer.as_entire_binding(),
        }],
    });

    // Create command encoder and compute pass
    let mut encoder_ops = CommandEncoderOps::new(device, Some("Multi-Dispatch Encoder"));
    let encoder = encoder_ops.inner_mut();

    let descriptor = ComputePassDescriptor::new().with_label("Multi-Dispatch Pass");

    {
        let mut compute_pass = ComputePassEncoder::begin(encoder, &descriptor).unwrap();
        compute_pass.set_pipeline(&pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);

        // Multiple dispatches in the same pass - each increments the values
        compute_pass.push_debug_group("Increment Operations");
        compute_pass.dispatch(4, 1, 1); // First increment
        compute_pass.insert_debug_marker("After first increment");
        compute_pass.dispatch(4, 1, 1); // Second increment
        compute_pass.insert_debug_marker("After second increment");
        compute_pass.dispatch(4, 1, 1); // Third increment
        compute_pass.pop_debug_group();
    }

    let command_buffer = encoder_ops.finish();
    queue.submit(std::iter::once(command_buffer));

    println!("  ✓ Executed 3 consecutive dispatches in one compute pass");
    println!("  ✓ Used debug markers and groups for organization\n");
}
