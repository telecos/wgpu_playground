// 3MF Viewer Shader
// Renders a 3D mesh loaded from a 3MF file with simple directional lighting

struct Uniforms {
    model_view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
    light_dir: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_normal: vec3<f32>,
    @location(1) world_position: vec3<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    let world_pos = uniforms.model * vec4<f32>(input.position, 1.0);
    output.clip_position = uniforms.model_view_proj * vec4<f32>(input.position, 1.0);
    output.world_position = world_pos.xyz;
    // Transform normal by the model matrix (no non-uniform scaling assumed)
    output.world_normal = normalize((uniforms.model * vec4<f32>(input.normal, 0.0)).xyz);
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(input.world_normal);
    let light_direction = normalize(-uniforms.light_dir.xyz);

    // Ambient lighting
    let ambient_strength = 0.15;
    let ambient = ambient_strength * vec3<f32>(1.0, 1.0, 1.0);

    // Diffuse lighting (Lambert's cosine law)
    let diff = max(dot(normal, light_direction), 0.0);
    let diffuse = diff * vec3<f32>(0.9, 0.9, 0.9);

    // Base color (steel blue-gray, typical for 3D print previews)
    let base_color = vec3<f32>(0.4, 0.6, 0.8);
    let final_color = base_color * (ambient + diffuse);

    return vec4<f32>(final_color, 1.0);
}
