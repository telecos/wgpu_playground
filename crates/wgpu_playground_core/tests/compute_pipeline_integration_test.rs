use wgpu_playground_core::compute::*;
use wgpu_playground_core::pipeline_layout::PipelineLayoutDescriptor;
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
fn test_simple_compute_pipeline_descriptor() {
    let shader =
        ShaderModule::from_source("@compute @workgroup_size(1) fn main() {}", Some("compute"))
            .unwrap();

    let descriptor = ComputePipelineDescriptor::new(Some("test_pipeline"))
        .with_shader(shader)
        .with_entry_point("main");

    assert_eq!(descriptor.label(), Some("test_pipeline"));
    assert_eq!(descriptor.entry_point(), Some("main"));
    assert!(descriptor.shader().is_some());
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_compute_pipeline_descriptor_validation_errors() {
    // Test missing shader
    let descriptor = ComputePipelineDescriptor::new(Some("test")).with_entry_point("main");
    assert!(descriptor.validate().is_err());

    // Test missing entry point
    let shader =
        ShaderModule::from_source("@compute @workgroup_size(1) fn main() {}", Some("compute"))
            .unwrap();
    let descriptor = ComputePipelineDescriptor::new(Some("test")).with_shader(shader);
    assert!(descriptor.validate().is_err());
}

#[test]
fn test_compute_pipeline_with_workgroup_size() {
    let shader_source = r#"
@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // Simple compute shader
}
"#;

    let shader = ShaderModule::from_source(shader_source, Some("compute_8x8")).unwrap();

    let descriptor = ComputePipelineDescriptor::new(Some("workgroup_pipeline"))
        .with_shader(shader)
        .with_entry_point("main");

    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_compute_pipeline_creation() {
    let device_queue = pollster::block_on(create_test_device());
    if device_queue.is_none() {
        eprintln!("Skipping test: could not create wgpu device");
        return;
    }

    let (device, _queue) = device_queue.unwrap();

    let shader_source = r#"
@compute @workgroup_size(1)
fn main() {
    // Empty compute shader for testing
}
"#;

    let shader = ShaderModule::from_source(shader_source, Some("test_compute")).unwrap();

    let descriptor = ComputePipelineDescriptor::new(Some("test_pipeline"))
        .with_shader(shader)
        .with_entry_point("main");

    let pipeline = descriptor.create_pipeline(&device);
    assert!(pipeline.is_ok(), "Failed to create compute pipeline");
}

#[test]
fn test_compute_pipeline_with_storage_buffer() {
    let device_queue = pollster::block_on(create_test_device());
    if device_queue.is_none() {
        eprintln!("Skipping test: could not create wgpu device");
        return;
    }

    let (device, _queue) = device_queue.unwrap();

    let shader_source = r#"
@group(0) @binding(0)
var<storage, read_write> data: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    data[index] = f32(index);
}
"#;

    let shader = ShaderModule::from_source(shader_source, Some("storage_compute")).unwrap();

    let descriptor = ComputePipelineDescriptor::new(Some("storage_pipeline"))
        .with_shader(shader)
        .with_entry_point("main");

    let pipeline = descriptor.create_pipeline(&device);
    assert!(
        pipeline.is_ok(),
        "Failed to create compute pipeline with storage buffer"
    );
}

#[test]
fn test_compute_pipeline_with_custom_layout() {
    let device_queue = pollster::block_on(create_test_device());
    if device_queue.is_none() {
        eprintln!("Skipping test: could not create wgpu device");
        return;
    }

    let (device, _queue) = device_queue.unwrap();

    // Create a simple bind group layout
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("compute_bind_group_layout"),
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

    let pipeline_layout_descriptor = PipelineLayoutDescriptor::new(Some("compute_layout"))
        .with_bind_group_layout(&bind_group_layout);

    let pipeline_layout = pipeline_layout_descriptor.create_layout(&device).unwrap();

    let shader_source = r#"
@group(0) @binding(0)
var<storage, read_write> data: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    data[index] = f32(index) * 2.0;
}
"#;

    let shader = ShaderModule::from_source(shader_source, Some("custom_layout_compute")).unwrap();

    let descriptor = ComputePipelineDescriptor::new(Some("custom_layout_pipeline"))
        .with_shader(shader)
        .with_entry_point("main")
        .with_layout(pipeline_layout);

    let pipeline = descriptor.create_pipeline(&device);
    assert!(
        pipeline.is_ok(),
        "Failed to create compute pipeline with custom layout"
    );
}

#[test]
fn test_compute_pipeline_multiple_buffers() {
    let device_queue = pollster::block_on(create_test_device());
    if device_queue.is_none() {
        eprintln!("Skipping test: could not create wgpu device");
        return;
    }

    let (device, _queue) = device_queue.unwrap();

    let shader_source = r#"
@group(0) @binding(0)
var<storage, read> input_a: array<f32>;

@group(0) @binding(1)
var<storage, read> input_b: array<f32>;

@group(0) @binding(2)
var<storage, read_write> output: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    output[index] = input_a[index] + input_b[index];
}
"#;

    let shader = ShaderModule::from_source(shader_source, Some("multi_buffer_compute")).unwrap();

    let descriptor = ComputePipelineDescriptor::new(Some("multi_buffer_pipeline"))
        .with_shader(shader)
        .with_entry_point("main");

    let pipeline = descriptor.create_pipeline(&device);
    assert!(
        pipeline.is_ok(),
        "Failed to create compute pipeline with multiple buffers"
    );
}

#[test]
fn test_compute_pipeline_with_uniform() {
    let device_queue = pollster::block_on(create_test_device());
    if device_queue.is_none() {
        eprintln!("Skipping test: could not create wgpu device");
        return;
    }

    let (device, _queue) = device_queue.unwrap();

    let shader_source = r#"
struct Params {
    multiplier: f32,
    offset: f32,
}

@group(0) @binding(0)
var<uniform> params: Params;

@group(0) @binding(1)
var<storage, read_write> data: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    data[index] = f32(index) * params.multiplier + params.offset;
}
"#;

    let shader = ShaderModule::from_source(shader_source, Some("uniform_compute")).unwrap();

    let descriptor = ComputePipelineDescriptor::new(Some("uniform_pipeline"))
        .with_shader(shader)
        .with_entry_point("main");

    let pipeline = descriptor.create_pipeline(&device);
    assert!(
        pipeline.is_ok(),
        "Failed to create compute pipeline with uniform buffer"
    );
}
