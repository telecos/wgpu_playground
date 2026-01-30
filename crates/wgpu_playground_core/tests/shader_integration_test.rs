use wgpu_playground_core::shader::{ShaderModule, ShaderSource};

#[test]
fn test_load_shader_from_file() {
    // Load the example shader that exists in assets/shaders/
    let shader = ShaderModule::from_file("example.wgsl", Some("example_shader"));

    assert!(
        shader.is_ok(),
        "Failed to load example.wgsl: {:?}",
        shader.err()
    );

    let shader = shader.unwrap();
    assert_eq!(shader.label(), Some("example_shader"));

    // Verify the shader contains expected content
    let source = shader.source();
    assert!(
        source.contains("VertexInput"),
        "Shader should contain VertexInput struct"
    );
    assert!(
        source.contains("vs_main"),
        "Shader should contain vs_main function"
    );
    assert!(
        source.contains("fs_main"),
        "Shader should contain fs_main function"
    );
}

#[test]
fn test_load_nonexistent_shader() {
    // Attempt to load a shader that doesn't exist
    let shader = ShaderModule::from_file("nonexistent.wgsl", None);

    assert!(shader.is_err(), "Should fail to load non-existent shader");
}

#[test]
fn test_shader_with_new_and_source_enum() {
    // Test using the ShaderSource enum directly
    let inline_source = ShaderSource::Inline(
        "@vertex fn main() -> @builtin(position) vec4<f32> { return vec4<f32>(0.0); }".to_string(),
    );
    let shader = ShaderModule::new(inline_source, Some("test"));
    assert!(shader.is_ok());

    // Test file source
    let file_source = ShaderSource::File("example.wgsl".to_string());
    let shader = ShaderModule::new(file_source, Some("example"));
    assert!(shader.is_ok());
}

#[test]
fn test_complex_shader_source() {
    let complex_shader = r#"
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> transform: mat4x4<f32>;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = transform * vec4<f32>(in.position, 1.0);
    out.tex_coords = in.tex_coords;
    return out;
}

@group(0) @binding(1)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(2)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
"#;

    let shader = ShaderModule::from_source(complex_shader, Some("complex_shader"));
    assert!(shader.is_ok());

    let shader = shader.unwrap();
    assert!(shader.source().contains("texture_2d"));
    assert!(shader.source().contains("textureSample"));
}
