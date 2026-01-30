use wgpu_playground_core::buffer::{BufferDescriptor, BufferUsages};
use wgpu_playground_core::command_encoder::CommandEncoderOps;
use wgpu_playground_core::compute::ComputePipelineDescriptor;
use wgpu_playground_core::compute_pass_encoder::{ComputePassDescriptor, ComputePassEncoder};
use wgpu_playground_core::shader::ShaderModule;

// Helper function to create a test device and queue
async fn create_test_device() -> Option<(wgpu::Device, wgpu::Queue)> {
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
                label: Some("Test Device"),
                memory_hints: Default::default(),
            },
            None,
        )
        .await
        .ok()
}

#[test]
fn test_compute_pass_descriptor_creation() {
    let descriptor = ComputePassDescriptor::new();
    assert_eq!(descriptor.label, None);
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_compute_pass_descriptor_with_label() {
    let descriptor = ComputePassDescriptor::new().with_label("test_pass");
    assert_eq!(descriptor.label, Some("test_pass"));
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_compute_pass_encoder_begin() {
    let device_queue = pollster::block_on(create_test_device());
    if device_queue.is_none() {
        eprintln!("Skipping test: could not create wgpu device");
        return;
    }

    let (device, _queue) = device_queue.unwrap();
    let mut encoder_ops = CommandEncoderOps::new(&device, Some("Test Encoder"));
    let encoder = encoder_ops.inner_mut();

    let descriptor = ComputePassDescriptor::new().with_label("Test Compute Pass");

    let compute_pass = ComputePassEncoder::begin(encoder, &descriptor);
    assert!(compute_pass.is_ok(), "Failed to begin compute pass");
}

#[test]
fn test_compute_pass_set_pipeline() {
    let device_queue = pollster::block_on(create_test_device());
    if device_queue.is_none() {
        eprintln!("Skipping test: could not create wgpu device");
        return;
    }

    let (device, _queue) = device_queue.unwrap();

    // Create a simple compute shader
    let shader_source = r#"
@compute @workgroup_size(1)
fn main() {
    // Empty compute shader
}
"#;

    let shader = ShaderModule::from_source(shader_source, Some("test_compute")).unwrap();

    let pipeline_descriptor = ComputePipelineDescriptor::new(Some("test_pipeline"))
        .with_shader(shader)
        .with_entry_point("main");

    let pipeline = pipeline_descriptor.create_pipeline(&device).unwrap();

    let mut encoder_ops = CommandEncoderOps::new(&device, Some("Test Encoder"));
    let encoder = encoder_ops.inner_mut();

    let descriptor = ComputePassDescriptor::new().with_label("Test Compute Pass");

    let mut compute_pass = ComputePassEncoder::begin(encoder, &descriptor).unwrap();
    compute_pass.set_pipeline(&pipeline);
    // If we get here without panic, the test passes
}

#[test]
fn test_compute_pass_dispatch() {
    let device_queue = pollster::block_on(create_test_device());
    if device_queue.is_none() {
        eprintln!("Skipping test: could not create wgpu device");
        return;
    }

    let (device, queue) = device_queue.unwrap();

    // Create a simple compute shader that writes to a storage buffer
    let shader_source = r#"
@group(0) @binding(0)
var<storage, read_write> output: array<u32>;

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    output[global_id.x] = global_id.x;
}
"#;

    let shader = ShaderModule::from_source(shader_source, Some("test_compute")).unwrap();

    let pipeline_descriptor = ComputePipelineDescriptor::new(Some("test_pipeline"))
        .with_shader(shader)
        .with_entry_point("main");

    let pipeline = pipeline_descriptor.create_pipeline(&device).unwrap();

    // Create a storage buffer
    let buffer_size = 64 * std::mem::size_of::<u32>() as u64;
    let buffer_descriptor = BufferDescriptor::new(
        Some("Storage Buffer"),
        buffer_size,
        BufferUsages::STORAGE | BufferUsages::COPY_SRC,
    );

    let storage_buffer = buffer_descriptor.create_buffer(&device).unwrap();

    // Create bind group
    let bind_group_layout = pipeline.get_bind_group_layout(0);
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("test_bind_group"),
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: storage_buffer.as_entire_binding(),
        }],
    });

    // Create command encoder and compute pass
    let mut encoder_ops = CommandEncoderOps::new(&device, Some("Test Encoder"));
    let encoder = encoder_ops.inner_mut();

    let descriptor = ComputePassDescriptor::new().with_label("Test Compute Pass");

    {
        let mut compute_pass = ComputePassEncoder::begin(encoder, &descriptor).unwrap();
        compute_pass.set_pipeline(&pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.dispatch(64, 1, 1);
    }

    let command_buffer = encoder_ops.finish();
    queue.submit(std::iter::once(command_buffer));

    // If we get here, the compute pass was successfully executed
}

#[test]
fn test_compute_pass_dispatch_indirect() {
    let device_queue = pollster::block_on(create_test_device());
    if device_queue.is_none() {
        eprintln!("Skipping test: could not create wgpu device");
        return;
    }

    let (device, queue) = device_queue.unwrap();

    // Create a simple compute shader
    let shader_source = r#"
@group(0) @binding(0)
var<storage, read_write> output: array<u32>;

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    output[global_id.x] = global_id.x;
}
"#;

    let shader = ShaderModule::from_source(shader_source, Some("test_compute")).unwrap();

    let pipeline_descriptor = ComputePipelineDescriptor::new(Some("test_pipeline"))
        .with_shader(shader)
        .with_entry_point("main");

    let pipeline = pipeline_descriptor.create_pipeline(&device).unwrap();

    // Create a storage buffer
    let buffer_size = 64 * std::mem::size_of::<u32>() as u64;
    let buffer_descriptor = BufferDescriptor::new(
        Some("Storage Buffer"),
        buffer_size,
        BufferUsages::STORAGE | BufferUsages::COPY_SRC,
    );

    let storage_buffer = buffer_descriptor.create_buffer(&device).unwrap();

    // Create an indirect buffer with dispatch parameters (x=64, y=1, z=1)
    let indirect_buffer_descriptor = BufferDescriptor::new(
        Some("Indirect Buffer"),
        12, // 3 u32 values
        BufferUsages::INDIRECT | BufferUsages::COPY_DST,
    );

    let indirect_buffer = indirect_buffer_descriptor.create_buffer(&device).unwrap();

    // Write dispatch parameters to the indirect buffer
    queue.write_buffer(&indirect_buffer, 0, bytemuck::cast_slice(&[64u32, 1u32, 1u32]));

    // Create bind group
    let bind_group_layout = pipeline.get_bind_group_layout(0);
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("test_bind_group"),
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: storage_buffer.as_entire_binding(),
        }],
    });

    // Create command encoder and compute pass
    let mut encoder_ops = CommandEncoderOps::new(&device, Some("Test Encoder"));
    let encoder = encoder_ops.inner_mut();

    let descriptor = ComputePassDescriptor::new().with_label("Test Compute Pass");

    {
        let mut compute_pass = ComputePassEncoder::begin(encoder, &descriptor).unwrap();
        compute_pass.set_pipeline(&pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.dispatch_indirect(&indirect_buffer, 0);
    }

    let command_buffer = encoder_ops.finish();
    queue.submit(std::iter::once(command_buffer));

    // If we get here, the indirect compute pass was successfully executed
}

#[test]
fn test_compute_pass_multiple_dispatches() {
    let device_queue = pollster::block_on(create_test_device());
    if device_queue.is_none() {
        eprintln!("Skipping test: could not create wgpu device");
        return;
    }

    let (device, queue) = device_queue.unwrap();

    // Create a compute shader
    let shader_source = r#"
@group(0) @binding(0)
var<storage, read_write> data: array<u32>;

@compute @workgroup_size(8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    data[index] = data[index] + 1u;
}
"#;

    let shader = ShaderModule::from_source(shader_source, Some("test_compute")).unwrap();

    let pipeline_descriptor = ComputePipelineDescriptor::new(Some("test_pipeline"))
        .with_shader(shader)
        .with_entry_point("main");

    let pipeline = pipeline_descriptor.create_pipeline(&device).unwrap();

    // Create a storage buffer
    let buffer_size = 64 * std::mem::size_of::<u32>() as u64;
    let buffer_descriptor = BufferDescriptor::new(
        Some("Storage Buffer"),
        buffer_size,
        BufferUsages::STORAGE | BufferUsages::COPY_SRC,
    );

    let storage_buffer = buffer_descriptor.create_buffer(&device).unwrap();

    // Create bind group
    let bind_group_layout = pipeline.get_bind_group_layout(0);
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("test_bind_group"),
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: storage_buffer.as_entire_binding(),
        }],
    });

    // Create command encoder and compute pass with multiple dispatches
    let mut encoder_ops = CommandEncoderOps::new(&device, Some("Test Encoder"));
    let encoder = encoder_ops.inner_mut();

    let descriptor = ComputePassDescriptor::new().with_label("Test Compute Pass");

    {
        let mut compute_pass = ComputePassEncoder::begin(encoder, &descriptor).unwrap();
        compute_pass.set_pipeline(&pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        
        // Multiple dispatches in the same pass
        compute_pass.dispatch(8, 1, 1); // Dispatch 8 workgroups (64 threads)
        compute_pass.dispatch(8, 1, 1); // Dispatch again
    }

    let command_buffer = encoder_ops.finish();
    queue.submit(std::iter::once(command_buffer));

    // If we get here, multiple dispatches were successfully executed
}

#[test]
fn test_compute_pass_debug_markers() {
    let device_queue = pollster::block_on(create_test_device());
    if device_queue.is_none() {
        eprintln!("Skipping test: could not create wgpu device");
        return;
    }

    let (device, _queue) = device_queue.unwrap();

    let mut encoder_ops = CommandEncoderOps::new(&device, Some("Test Encoder"));
    let encoder = encoder_ops.inner_mut();

    let descriptor = ComputePassDescriptor::new().with_label("Test Compute Pass");

    {
        let mut compute_pass = ComputePassEncoder::begin(encoder, &descriptor).unwrap();
        compute_pass.push_debug_group("Test Group");
        compute_pass.insert_debug_marker("Test Marker");
        compute_pass.pop_debug_group();
    }

    // If we get here, debug markers were successfully added
}

#[test]
fn test_compute_pass_with_bind_groups() {
    let device_queue = pollster::block_on(create_test_device());
    if device_queue.is_none() {
        eprintln!("Skipping test: could not create wgpu device");
        return;
    }

    let (device, queue) = device_queue.unwrap();

    // Create a compute shader with multiple bind groups
    let shader_source = r#"
@group(0) @binding(0)
var<storage, read> input: array<u32>;

@group(1) @binding(0)
var<storage, read_write> output: array<u32>;

@compute @workgroup_size(8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    output[index] = input[index] * 2u;
}
"#;

    let shader = ShaderModule::from_source(shader_source, Some("test_compute")).unwrap();

    let pipeline_descriptor = ComputePipelineDescriptor::new(Some("test_pipeline"))
        .with_shader(shader)
        .with_entry_point("main");

    let pipeline = pipeline_descriptor.create_pipeline(&device).unwrap();

    // Create storage buffers
    let buffer_size = 64 * std::mem::size_of::<u32>() as u64;
    
    let input_buffer_descriptor = BufferDescriptor::new(
        Some("Input Buffer"),
        buffer_size,
        BufferUsages::STORAGE,
    );

    let input_buffer = input_buffer_descriptor.create_buffer(&device).unwrap();

    let output_buffer_descriptor = BufferDescriptor::new(
        Some("Output Buffer"),
        buffer_size,
        BufferUsages::STORAGE | BufferUsages::COPY_SRC,
    );

    let output_buffer = output_buffer_descriptor.create_buffer(&device).unwrap();

    // Create bind groups
    let bind_group_layout_0 = pipeline.get_bind_group_layout(0);
    let bind_group_0 = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("bind_group_0"),
        layout: &bind_group_layout_0,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: input_buffer.as_entire_binding(),
        }],
    });

    let bind_group_layout_1 = pipeline.get_bind_group_layout(1);
    let bind_group_1 = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("bind_group_1"),
        layout: &bind_group_layout_1,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: output_buffer.as_entire_binding(),
        }],
    });

    // Create command encoder and compute pass
    let mut encoder_ops = CommandEncoderOps::new(&device, Some("Test Encoder"));
    let encoder = encoder_ops.inner_mut();

    let descriptor = ComputePassDescriptor::new().with_label("Test Compute Pass");

    {
        let mut compute_pass = ComputePassEncoder::begin(encoder, &descriptor).unwrap();
        compute_pass.set_pipeline(&pipeline);
        compute_pass.set_bind_group(0, &bind_group_0, &[]);
        compute_pass.set_bind_group(1, &bind_group_1, &[]);
        compute_pass.dispatch(8, 1, 1);
    }

    let command_buffer = encoder_ops.finish();
    queue.submit(std::iter::once(command_buffer));

    // If we get here, multiple bind groups were successfully set
}
