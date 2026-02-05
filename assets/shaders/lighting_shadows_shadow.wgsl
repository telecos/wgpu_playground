// Shadow Pass Shader
// Renders the scene from the light's perspective to generate a shadow map

struct ShadowUniforms {
    light_view_proj: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> shadow_uniforms: ShadowUniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = shadow_uniforms.light_view_proj * vec4<f32>(input.position, 1.0);
    return output;
}

// No fragment shader needed - we only write to the depth buffer
