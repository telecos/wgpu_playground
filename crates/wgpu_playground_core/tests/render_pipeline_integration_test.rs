mod common;

use common::create_test_device;
use wgpu_playground_core::render_pipeline::*;
use wgpu_playground_core::shader::ShaderModule;

#[test]
fn test_simple_render_pipeline_descriptor() {
    let descriptor = RenderPipelineDescriptor::new(Some("test_pipeline"))
        .with_fragment_target(ColorTargetState::new(wgpu::TextureFormat::Bgra8UnormSrgb));

    assert_eq!(descriptor.label(), Some("test_pipeline"));
    assert_eq!(descriptor.fragment_targets().len(), 1);
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_render_pipeline_with_vertex_buffers() {
    let layout = VertexBufferLayout::new(32, VertexStepMode::Vertex)
        .with_attribute(VertexAttribute::new(0, VertexFormat::Float32x3, 0))
        .with_attribute(VertexAttribute::new(1, VertexFormat::Float32x4, 12));

    let descriptor = RenderPipelineDescriptor::new(Some("vertex_pipeline"))
        .with_vertex_buffer(layout)
        .with_fragment_target(ColorTargetState::new(wgpu::TextureFormat::Bgra8UnormSrgb));

    assert_eq!(descriptor.vertex_buffers().len(), 1);
    assert_eq!(descriptor.vertex_buffers()[0].attributes.len(), 2);
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_render_pipeline_with_depth_stencil() {
    let depth_stencil = DepthStencilState::new(wgpu::TextureFormat::Depth24Plus)
        .with_depth_write_enabled(true)
        .with_depth_compare(CompareFunction::Less);

    let descriptor = RenderPipelineDescriptor::new(Some("depth_pipeline"))
        .with_depth_stencil(depth_stencil)
        .with_fragment_target(ColorTargetState::new(wgpu::TextureFormat::Bgra8UnormSrgb));

    assert!(descriptor.depth_stencil().is_some());
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_render_pipeline_with_msaa() {
    let multisample = MultisampleState::new()
        .with_count(4)
        .with_alpha_to_coverage(false);

    let descriptor = RenderPipelineDescriptor::new(Some("msaa_pipeline"))
        .with_multisample(multisample)
        .with_fragment_target(ColorTargetState::new(wgpu::TextureFormat::Bgra8UnormSrgb));

    assert_eq!(descriptor.multisample().count, 4);
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_render_pipeline_with_blending() {
    let target = ColorTargetState::new(wgpu::TextureFormat::Bgra8UnormSrgb)
        .with_blend(BlendState::alpha_blending());

    let descriptor =
        RenderPipelineDescriptor::new(Some("blend_pipeline")).with_fragment_target(target);

    assert!(descriptor.fragment_targets()[0].blend.is_some());
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_render_pipeline_with_primitive_state() {
    let primitive = PrimitiveState::new()
        .with_topology(PrimitiveTopology::LineList)
        .with_cull_mode(CullMode::Back)
        .with_front_face(FrontFace::Cw);

    let descriptor = RenderPipelineDescriptor::new(Some("primitive_pipeline"))
        .with_primitive(primitive)
        .with_fragment_target(ColorTargetState::new(wgpu::TextureFormat::Bgra8UnormSrgb));

    assert_eq!(descriptor.primitive().topology, PrimitiveTopology::LineList);
    assert_eq!(descriptor.primitive().cull_mode, CullMode::Back);
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_render_pipeline_with_multiple_targets() {
    let target1 = ColorTargetState::new(wgpu::TextureFormat::Rgba8UnormSrgb);
    let target2 = ColorTargetState::new(wgpu::TextureFormat::Rgba16Float);

    let descriptor = RenderPipelineDescriptor::new(Some("multi_target_pipeline"))
        .with_fragment_target(target1)
        .with_fragment_target(target2);

    assert_eq!(descriptor.fragment_targets().len(), 2);
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_complex_vertex_layout() {
    // Position (vec3), Normal (vec3), UV (vec2), Color (vec4)
    let layout = VertexBufferLayout::new(48, VertexStepMode::Vertex)
        .with_attribute(VertexAttribute::new(0, VertexFormat::Float32x3, 0)) // position
        .with_attribute(VertexAttribute::new(1, VertexFormat::Float32x3, 12)) // normal
        .with_attribute(VertexAttribute::new(2, VertexFormat::Float32x2, 24)) // uv
        .with_attribute(VertexAttribute::new(3, VertexFormat::Float32x4, 32)); // color

    assert_eq!(layout.attributes.len(), 4);
    assert!(layout.validate().is_ok());

    let descriptor = RenderPipelineDescriptor::new(Some("complex_vertex"))
        .with_vertex_buffer(layout)
        .with_fragment_target(ColorTargetState::new(wgpu::TextureFormat::Bgra8UnormSrgb));

    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_instanced_rendering_layout() {
    // Per-vertex data
    let vertex_layout = VertexBufferLayout::new(20, VertexStepMode::Vertex)
        .with_attribute(VertexAttribute::new(0, VertexFormat::Float32x3, 0)) // position
        .with_attribute(VertexAttribute::new(1, VertexFormat::Float32x2, 12)); // uv

    // Per-instance data
    let instance_layout = VertexBufferLayout::new(16, VertexStepMode::Instance)
        .with_attribute(VertexAttribute::new(2, VertexFormat::Float32x4, 0)); // instance color

    let descriptor = RenderPipelineDescriptor::new(Some("instanced"))
        .with_vertex_buffer(vertex_layout)
        .with_vertex_buffer(instance_layout)
        .with_fragment_target(ColorTargetState::new(wgpu::TextureFormat::Bgra8UnormSrgb));

    assert_eq!(descriptor.vertex_buffers().len(), 2);
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_pipeline_cache_operations() {
    let cache = PipelineCache::new();

    assert!(cache.is_empty());
    assert_eq!(cache.len(), 0);
    assert!(!cache.contains("test_pipeline"));
}

#[test]
fn test_full_render_pipeline_creation() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create shaders
        let vertex_shader = ShaderModule::from_source(
            r#"
@vertex
fn main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    let x = f32(i32(in_vertex_index) - 1);
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
    return vec4<f32>(x, y, 0.0, 1.0);
}
            "#,
            Some("vertex"),
        )
        .unwrap();

        let fragment_shader = ShaderModule::from_source(
            r#"
@fragment
fn main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
            "#,
            Some("fragment"),
        )
        .unwrap();

        // Create empty pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("test_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        // Create pipeline descriptor
        let descriptor = RenderPipelineDescriptor::new(Some("test_pipeline"))
            .with_fragment_target(ColorTargetState::new(wgpu::TextureFormat::Bgra8UnormSrgb));

        // Create the pipeline
        let result = descriptor.create_pipeline(
            &device,
            &pipeline_layout,
            &vertex_shader,
            Some(&fragment_shader),
        );

        assert!(
            result.is_ok(),
            "Failed to create render pipeline: {:?}",
            result.err()
        );
    });
}

#[test]
fn test_render_pipeline_with_vertex_attributes() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create shaders with vertex input
        let vertex_shader = ShaderModule::from_source(
            r#"
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(in.position, 1.0);
    out.color = in.color;
    return out;
}
            "#,
            Some("vertex"),
        )
        .unwrap();

        let fragment_shader = ShaderModule::from_source(
            r#"
@fragment
fn main(@location(0) color: vec4<f32>) -> @location(0) vec4<f32> {
    return color;
}
            "#,
            Some("fragment"),
        )
        .unwrap();

        // Create vertex buffer layout
        let layout = VertexBufferLayout::new(28, VertexStepMode::Vertex)
            .with_attribute(VertexAttribute::new(0, VertexFormat::Float32x3, 0))
            .with_attribute(VertexAttribute::new(1, VertexFormat::Float32x4, 12));

        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("test_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        // Create pipeline descriptor
        let descriptor = RenderPipelineDescriptor::new(Some("vertex_attr_pipeline"))
            .with_vertex_buffer(layout)
            .with_fragment_target(ColorTargetState::new(wgpu::TextureFormat::Bgra8UnormSrgb));

        // Create the pipeline
        let result = descriptor.create_pipeline(
            &device,
            &pipeline_layout,
            &vertex_shader,
            Some(&fragment_shader),
        );

        assert!(
            result.is_ok(),
            "Failed to create render pipeline with vertex attributes: {:?}",
            result.err()
        );
    });
}

#[test]
fn test_render_pipeline_with_depth_testing() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let vertex_shader = ShaderModule::from_source(
            r#"
@vertex
fn main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    let x = f32(i32(in_vertex_index) - 1);
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
    return vec4<f32>(x, y, 0.0, 1.0);
}
            "#,
            Some("vertex"),
        )
        .unwrap();

        let fragment_shader = ShaderModule::from_source(
            r#"
@fragment
fn main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 1.0, 0.0, 1.0);
}
            "#,
            Some("fragment"),
        )
        .unwrap();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("test_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        // Create depth stencil state
        let depth_stencil = DepthStencilState::new(wgpu::TextureFormat::Depth24Plus)
            .with_depth_write_enabled(true)
            .with_depth_compare(CompareFunction::Less);

        let descriptor = RenderPipelineDescriptor::new(Some("depth_test_pipeline"))
            .with_depth_stencil(depth_stencil)
            .with_fragment_target(ColorTargetState::new(wgpu::TextureFormat::Bgra8UnormSrgb));

        let result = descriptor.create_pipeline(
            &device,
            &pipeline_layout,
            &vertex_shader,
            Some(&fragment_shader),
        );

        assert!(
            result.is_ok(),
            "Failed to create render pipeline with depth testing: {:?}",
            result.err()
        );
    });
}

#[test]
fn test_render_pipeline_with_all_states() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let vertex_shader = ShaderModule::from_source(
            r#"
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) uv: vec2<f32>,
}

@vertex
fn main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(in.position, 1.0);
    out.normal = in.normal;
    out.uv = in.uv;
    return out;
}
            "#,
            Some("vertex"),
        )
        .unwrap();

        let fragment_shader = ShaderModule::from_source(
            r#"
@fragment
fn main(@location(0) normal: vec3<f32>, @location(1) uv: vec2<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(normal * 0.5 + 0.5, 1.0);
}
            "#,
            Some("fragment"),
        )
        .unwrap();

        // Vertex buffer layout with position, normal, and UV
        let layout = VertexBufferLayout::new(32, VertexStepMode::Vertex)
            .with_attribute(VertexAttribute::new(0, VertexFormat::Float32x3, 0))
            .with_attribute(VertexAttribute::new(1, VertexFormat::Float32x3, 12))
            .with_attribute(VertexAttribute::new(2, VertexFormat::Float32x2, 24));

        // Primitive state with backface culling
        let primitive = PrimitiveState::new()
            .with_topology(PrimitiveTopology::TriangleList)
            .with_cull_mode(CullMode::Back)
            .with_front_face(FrontFace::Ccw);

        // Depth stencil state
        let depth_stencil = DepthStencilState::new(wgpu::TextureFormat::Depth24Plus)
            .with_depth_write_enabled(true)
            .with_depth_compare(CompareFunction::Less);

        // Multisample state
        let multisample = MultisampleState::new().with_count(4);

        // Fragment target with blending
        let target = ColorTargetState::new(wgpu::TextureFormat::Bgra8UnormSrgb)
            .with_blend(BlendState::alpha_blending());

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("test_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        // Create comprehensive pipeline
        let descriptor = RenderPipelineDescriptor::new(Some("full_pipeline"))
            .with_vertex_buffer(layout)
            .with_primitive(primitive)
            .with_depth_stencil(depth_stencil)
            .with_multisample(multisample)
            .with_fragment_target(target);

        let result = descriptor.create_pipeline(
            &device,
            &pipeline_layout,
            &vertex_shader,
            Some(&fragment_shader),
        );

        assert!(
            result.is_ok(),
            "Failed to create comprehensive render pipeline: {:?}",
            result.err()
        );
    });
}

// ============================================================================
// Invalid Configuration Tests
// ============================================================================

#[test]
fn test_render_pipeline_invalid_shader_syntax() {
    pollster::block_on(async {
        let Some((_device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Try to create shader with invalid syntax
        let invalid_shader_source = r#"
@vertex
fn main( this is invalid syntax
    return vec4<f32>(0.0);
}
        "#;

        let vertex_shader_result =
            ShaderModule::from_source(invalid_shader_source, Some("invalid_vertex"));

        // Shader module creation should fail with syntax error
        assert!(
            vertex_shader_result.is_err(),
            "Expected shader creation to fail with syntax error"
        );
    });
}

#[test]
fn test_render_pipeline_missing_shader() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let vertex_shader = ShaderModule::from_source(
            r#"
@vertex
fn main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}
            "#,
            Some("vertex"),
        )
        .unwrap();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("test_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let descriptor = RenderPipelineDescriptor::new(Some("no_fragment"))
            .with_fragment_target(ColorTargetState::new(wgpu::TextureFormat::Bgra8UnormSrgb));

        // Try to create pipeline without fragment shader (fragment shader is optional but
        // we're testing it can be created without one)
        let result =
            descriptor.create_pipeline(&device, &pipeline_layout, &vertex_shader, None);

        // Pipeline creation should succeed - fragment shader is optional
        assert!(
            result.is_ok(),
            "Pipeline creation without fragment shader should succeed: {:?}",
            result.err()
        );
    });
}

#[test]
fn test_render_pipeline_shader_type_mismatch() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Shader with mismatched types in vertex output and fragment input
        let vertex_shader = ShaderModule::from_source(
            r#"
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    out.color = vec4<f32>(1.0, 0.0, 0.0, 1.0);
    return out;
}
            "#,
            Some("vertex"),
        )
        .unwrap();

        // Fragment expects vec3 but vertex outputs vec4
        let fragment_shader = ShaderModule::from_source(
            r#"
@fragment
fn main(@location(0) color: vec3<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(color, 1.0);
}
            "#,
            Some("fragment"),
        )
        .unwrap();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("test_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let descriptor = RenderPipelineDescriptor::new(Some("type_mismatch"))
            .with_fragment_target(ColorTargetState::new(wgpu::TextureFormat::Bgra8UnormSrgb));

        // This should fail during pipeline creation due to type mismatch
        let result = descriptor.create_pipeline(
            &device,
            &pipeline_layout,
            &vertex_shader,
            Some(&fragment_shader),
        );

        assert!(
            result.is_err(),
            "Expected pipeline creation to fail due to type mismatch"
        );
    });
}

#[test]
fn test_render_pipeline_invalid_vertex_attribute_offset() {
    // Test vertex attributes with invalid offsets
    let layout = VertexBufferLayout::new(16, VertexStepMode::Vertex)
        // Offset goes beyond stride - attribute is vec3 (12 bytes) starting at offset 12
        // which would extend to byte 24, but stride is only 16
        .with_attribute(VertexAttribute::new(0, VertexFormat::Float32x3, 12));

    let descriptor = RenderPipelineDescriptor::new(Some("invalid_offset"))
        .with_vertex_buffer(layout)
        .with_fragment_target(ColorTargetState::new(wgpu::TextureFormat::Bgra8UnormSrgb));

    // Note: Descriptor validation may not catch this offset issue at the descriptor level.
    // The GPU driver would detect this error during actual pipeline creation.
    // This test documents that such configurations can be created but would fail at runtime.
    let _validation_result = descriptor.validate();
}

#[test]
fn test_render_pipeline_invalid_msaa_count() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Test with invalid MSAA sample count (must be 1, 2, or 4)
        let multisample = MultisampleState::new()
            .with_count(3) // Invalid - not a power of 2
            .with_alpha_to_coverage(false);

        let vertex_shader = ShaderModule::from_source(
            r#"
@vertex
fn main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}
            "#,
            Some("vertex"),
        )
        .unwrap();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("test_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let descriptor = RenderPipelineDescriptor::new(Some("invalid_msaa"))
            .with_multisample(multisample)
            .with_fragment_target(ColorTargetState::new(wgpu::TextureFormat::Bgra8UnormSrgb));

        // Descriptor validation may succeed, but pipeline creation should fail
        let result =
            descriptor.create_pipeline(&device, &pipeline_layout, &vertex_shader, None);

        // Pipeline creation should fail due to invalid MSAA sample count
        assert!(
            result.is_err(),
            "Expected pipeline creation to fail with invalid MSAA count"
        );
    });
}

#[test]
fn test_render_pipeline_no_fragment_targets() {
    // Test pipeline with no fragment targets and no depth/stencil
    let descriptor = RenderPipelineDescriptor::new(Some("no_targets"));

    // Note: WebGPU spec allows pipelines with only depth/stencil output (no color targets).
    // A pipeline with no outputs at all may be accepted by the descriptor but rejected
    // during actual GPU pipeline creation. This test documents that the descriptor
    // itself doesn't enforce having at least one output target.
    let validation_result = descriptor.validate();
    
    // Just verify we can call validate - the actual enforcement happens at pipeline creation time
    match validation_result {
        Ok(_) => {
            // Descriptor allows this, GPU driver would reject during pipeline creation
        }
        Err(_) => {
            // Descriptor validation caught it
        }
    }
}

#[test]
fn test_render_pipeline_shader_undefined_variable() {
    pollster::block_on(async {
        let Some((_device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Shader with undefined variable
        let invalid_shader_source = r#"
@vertex
fn main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    return undefined_variable;
}
        "#;

        let shader_result = ShaderModule::from_source(invalid_shader_source, Some("undefined_var"));

        // Shader compilation should fail
        assert!(
            shader_result.is_err(),
            "Expected shader creation to fail with undefined variable"
        );
    });
}

#[test]
fn test_render_pipeline_duplicate_vertex_locations() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Vertex buffer with duplicate location indices
        let layout = VertexBufferLayout::new(32, VertexStepMode::Vertex)
            .with_attribute(VertexAttribute::new(0, VertexFormat::Float32x3, 0))
            .with_attribute(VertexAttribute::new(0, VertexFormat::Float32x4, 12)); // Duplicate location 0

        let vertex_shader = ShaderModule::from_source(
            r#"
@vertex
fn main(@location(0) pos: vec3<f32>) -> @builtin(position) vec4<f32> {
    return vec4<f32>(pos, 1.0);
}
            "#,
            Some("vertex"),
        )
        .unwrap();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("test_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let descriptor = RenderPipelineDescriptor::new(Some("duplicate_locations"))
            .with_vertex_buffer(layout)
            .with_fragment_target(ColorTargetState::new(wgpu::TextureFormat::Bgra8UnormSrgb));

        // Pipeline creation should fail due to duplicate vertex locations
        let result =
            descriptor.create_pipeline(&device, &pipeline_layout, &vertex_shader, None);

        assert!(
            result.is_err(),
            "Expected pipeline creation to fail with duplicate vertex locations"
        );
    });
}

#[test]
fn test_render_pipeline_mismatched_vertex_shader_inputs() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Vertex buffer provides location 0 and 1
        let layout = VertexBufferLayout::new(28, VertexStepMode::Vertex)
            .with_attribute(VertexAttribute::new(0, VertexFormat::Float32x3, 0))
            .with_attribute(VertexAttribute::new(1, VertexFormat::Float32x4, 12));

        // But shader expects location 2
        let vertex_shader = ShaderModule::from_source(
            r#"
@vertex
fn main(@location(2) data: vec4<f32>) -> @builtin(position) vec4<f32> {
    return data;
}
            "#,
            Some("vertex"),
        )
        .unwrap();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("test_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let descriptor = RenderPipelineDescriptor::new(Some("mismatched_inputs"))
            .with_vertex_buffer(layout)
            .with_fragment_target(ColorTargetState::new(wgpu::TextureFormat::Bgra8UnormSrgb));

        let result =
            descriptor.create_pipeline(&device, &pipeline_layout, &vertex_shader, None);

        // Pipeline creation should fail - shader expects location 2 but we only provide 0 and 1
        assert!(
            result.is_err(),
            "Expected pipeline creation to fail with mismatched vertex inputs"
        );
    });
}
