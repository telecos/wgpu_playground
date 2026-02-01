// Instanced Rendering Shader
// Demonstrates per-instance attributes for rendering multiple objects efficiently

// Uniform buffer for view and projection matrices
struct Uniforms {
    view_proj: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

// Per-vertex attributes (shared across all instances)
struct VertexInput {
    @location(0) position: vec3<f32>,
}

// Per-instance attributes (unique for each instance)
struct InstanceInput {
    @location(1) instance_position: vec3<f32>,
    @location(2) instance_rotation: f32,
    @location(3) instance_scale: f32,
    @location(4) instance_color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

// Helper function to create a rotation matrix around Y axis
fn rotation_y(angle: f32) -> mat4x4<f32> {
    let c = cos(angle);
    let s = sin(angle);
    return mat4x4<f32>(
        vec4<f32>(c, 0.0, s, 0.0),
        vec4<f32>(0.0, 1.0, 0.0, 0.0),
        vec4<f32>(-s, 0.0, c, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0),
    );
}

// Helper function to create a scale matrix
fn scale_matrix(s: f32) -> mat4x4<f32> {
    return mat4x4<f32>(
        vec4<f32>(s, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, s, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, s, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0),
    );
}

// Helper function to create a translation matrix
fn translation_matrix(offset: vec3<f32>) -> mat4x4<f32> {
    return mat4x4<f32>(
        vec4<f32>(1.0, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, 1.0, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(offset.x, offset.y, offset.z, 1.0),
    );
}

@vertex
fn vs_main(
    vertex: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var output: VertexOutput;
    
    // Build model matrix from instance attributes
    let translation = translation_matrix(instance.instance_position);
    let rotation = rotation_y(instance.instance_rotation);
    let scale = scale_matrix(instance.instance_scale);
    let model = translation * rotation * scale;
    
    // Apply transformations
    output.clip_position = uniforms.view_proj * model * vec4<f32>(vertex.position, 1.0);
    output.color = instance.instance_color;
    
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(input.color, 1.0);
}
