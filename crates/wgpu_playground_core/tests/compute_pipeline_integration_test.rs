mod common;

use common::create_test_device;
use wgpu_playground_core::compute::*;
use wgpu_playground_core::pipeline_layout::PipelineLayoutDescriptor;
use wgpu_playground_core::shader::ShaderModule;

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

// ============================================================================
// Invalid Configuration Tests
// ============================================================================

#[test]
fn test_compute_pipeline_invalid_shader_syntax() {
    let device_queue = pollster::block_on(create_test_device());
    if device_queue.is_none() {
        eprintln!("Skipping test: could not create wgpu device");
        return;
    }

    // Invalid WGSL syntax
    let invalid_shader = r#"
@compute @workgroup_size(8)
fn main( this is completely invalid {
    let x = undefined syntax;
}
    "#;

    let shader_result = ShaderModule::from_source(invalid_shader, Some("invalid_syntax"));

    // Shader creation should fail
    assert!(
        shader_result.is_err(),
        "Expected shader creation to fail with syntax error"
    );
}

#[test]
fn test_compute_pipeline_nonexistent_entry_point() {
    let device_queue = pollster::block_on(create_test_device());
    if device_queue.is_none() {
        eprintln!("Skipping test: could not create wgpu device");
        return;
    }

    let (device, _queue) = device_queue.unwrap();

    let shader_source = r#"
@compute @workgroup_size(1)
fn compute_main() {
    // The actual function is named compute_main
}
"#;

    let shader = ShaderModule::from_source(shader_source, Some("test_shader")).unwrap();

    // Try to use a non-existent entry point
    let descriptor = ComputePipelineDescriptor::new(Some("wrong_entry"))
        .with_shader(shader)
        .with_entry_point("main"); // Wrong! Should be "compute_main"

    let pipeline = descriptor.create_pipeline(&device);

    // Pipeline creation should fail
    assert!(
        pipeline.is_err(),
        "Expected pipeline creation to fail with non-existent entry point"
    );
}

#[test]
fn test_compute_pipeline_missing_workgroup_size() {
    let device_queue = pollster::block_on(create_test_device());
    if device_queue.is_none() {
        eprintln!("Skipping test: could not create wgpu device");
        return;
    }

    // Shader missing @workgroup_size attribute
    let shader_source = r#"
@compute
fn main() {
    // Missing @workgroup_size attribute
}
"#;

    let shader_result = ShaderModule::from_source(shader_source, Some("no_workgroup"));

    // Shader creation should fail - workgroup_size is required
    assert!(
        shader_result.is_err(),
        "Expected shader creation to fail without workgroup_size"
    );
}

#[test]
fn test_compute_pipeline_invalid_workgroup_size() {
    let device_queue = pollster::block_on(create_test_device());
    if device_queue.is_none() {
        eprintln!("Skipping test: could not create wgpu device");
        return;
    }

    let (device, _queue) = device_queue.unwrap();

    // Workgroup size exceeds typical limits (256 is common max)
    let shader_source = r#"
@compute @workgroup_size(1024, 1024, 1024)
fn main() {
    // Workgroup size way too large
}
"#;

    let shader_result = ShaderModule::from_source(shader_source, Some("huge_workgroup"));

    if let Ok(shader) = shader_result {
        let descriptor = ComputePipelineDescriptor::new(Some("huge_pipeline"))
            .with_shader(shader)
            .with_entry_point("main");

        let pipeline = descriptor.create_pipeline(&device);

        // Pipeline creation should fail due to exceeding workgroup size limits
        assert!(
            pipeline.is_err(),
            "Expected pipeline creation to fail with excessive workgroup size"
        );
    }
}

#[test]
fn test_compute_pipeline_zero_workgroup_size() {
    let device_queue = pollster::block_on(create_test_device());
    if device_queue.is_none() {
        eprintln!("Skipping test: could not create wgpu device");
        return;
    }

    // Zero workgroup size is invalid
    let shader_source = r#"
@compute @workgroup_size(0)
fn main() {
    // Invalid: workgroup size cannot be zero
}
"#;

    let shader_result = ShaderModule::from_source(shader_source, Some("zero_workgroup"));

    // Shader creation should fail
    assert!(
        shader_result.is_err(),
        "Expected shader creation to fail with zero workgroup size"
    );
}

#[test]
fn test_compute_pipeline_undefined_variable() {
    let device_queue = pollster::block_on(create_test_device());
    if device_queue.is_none() {
        eprintln!("Skipping test: could not create wgpu device");
        return;
    }

    // Shader with undefined variable reference
    let shader_source = r#"
@compute @workgroup_size(1)
fn main() {
    let x = undefined_variable;
}
"#;

    let shader_result = ShaderModule::from_source(shader_source, Some("undefined_var"));

    // Shader creation should fail
    assert!(
        shader_result.is_err(),
        "Expected shader creation to fail with undefined variable"
    );
}

#[test]
fn test_compute_pipeline_type_mismatch() {
    let device_queue = pollster::block_on(create_test_device());
    if device_queue.is_none() {
        eprintln!("Skipping test: could not create wgpu device");
        return;
    }

    // Shader with type mismatch
    let shader_source = r#"
@compute @workgroup_size(1)
fn main() {
    let x: f32 = vec4<f32>(1.0, 2.0, 3.0, 4.0);
}
"#;

    let shader_result = ShaderModule::from_source(shader_source, Some("type_mismatch"));

    // Shader creation should fail
    assert!(
        shader_result.is_err(),
        "Expected shader creation to fail with type mismatch"
    );
}

#[test]
fn test_compute_pipeline_buffer_binding_mismatch() {
    let device_queue = pollster::block_on(create_test_device());
    if device_queue.is_none() {
        eprintln!("Skipping test: could not create wgpu device");
        return;
    }

    let (device, _queue) = device_queue.unwrap();

    // Shader expects a storage buffer
    let shader_source = r#"
@group(0) @binding(0)
var<storage, read_write> data: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    data[global_id.x] = f32(global_id.x);
}
"#;

    let shader = ShaderModule::from_source(shader_source, Some("storage_shader")).unwrap();

    // Create a bind group layout with uniform buffer instead of storage
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("wrong_layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform, // Wrong! Shader expects Storage
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });

    let pipeline_layout_descriptor = PipelineLayoutDescriptor::new(Some("wrong_pipeline_layout"))
        .with_bind_group_layout(&bind_group_layout);

    let pipeline_layout = pipeline_layout_descriptor.create_layout(&device).unwrap();

    let descriptor = ComputePipelineDescriptor::new(Some("mismatched_binding"))
        .with_shader(shader)
        .with_entry_point("main")
        .with_layout(pipeline_layout);

    let pipeline = descriptor.create_pipeline(&device);

    // Pipeline creation should fail due to binding type mismatch
    assert!(
        pipeline.is_err(),
        "Expected pipeline creation to fail with binding type mismatch"
    );
}

#[test]
fn test_compute_pipeline_missing_binding() {
    let device_queue = pollster::block_on(create_test_device());
    if device_queue.is_none() {
        eprintln!("Skipping test: could not create wgpu device");
        return;
    }

    let (device, _queue) = device_queue.unwrap();

    // Shader expects binding at @group(0) @binding(0)
    let shader_source = r#"
@group(0) @binding(0)
var<storage, read_write> data: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    data[global_id.x] = f32(global_id.x);
}
"#;

    let shader = ShaderModule::from_source(shader_source, Some("needs_binding")).unwrap();

    // Create pipeline layout with no bindings
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("empty_layout"),
        entries: &[], // No bindings!
    });

    let pipeline_layout_descriptor = PipelineLayoutDescriptor::new(Some("empty_pipeline_layout"))
        .with_bind_group_layout(&bind_group_layout);

    let pipeline_layout = pipeline_layout_descriptor.create_layout(&device).unwrap();

    let descriptor = ComputePipelineDescriptor::new(Some("missing_binding"))
        .with_shader(shader)
        .with_entry_point("main")
        .with_layout(pipeline_layout);

    let pipeline = descriptor.create_pipeline(&device);

    // Pipeline creation should fail - shader requires binding that doesn't exist
    assert!(
        pipeline.is_err(),
        "Expected pipeline creation to fail with missing binding"
    );
}

#[test]
fn test_compute_pipeline_wrong_shader_stage() {
    let device_queue = pollster::block_on(create_test_device());
    if device_queue.is_none() {
        eprintln!("Skipping test: could not create wgpu device");
        return;
    }

    let (device, _queue) = device_queue.unwrap();

    // Vertex shader instead of compute shader
    let shader_source = r#"
@vertex
fn main(@builtin(vertex_index) index: u32) -> @builtin(position) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}
"#;

    let shader_result = ShaderModule::from_source(shader_source, Some("vertex_not_compute"));

    if let Ok(shader) = shader_result {
        let descriptor = ComputePipelineDescriptor::new(Some("wrong_stage"))
            .with_shader(shader)
            .with_entry_point("main");

        let pipeline = descriptor.create_pipeline(&device);

        // Pipeline creation should fail - not a compute shader
        assert!(
            pipeline.is_err(),
            "Expected pipeline creation to fail when using vertex shader for compute pipeline"
        );
    }
}
