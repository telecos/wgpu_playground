// Grayscale post-processing shader
// Converts the input texture to grayscale using luminance weights

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@group(0) @binding(0)
var texture_sampler: sampler;

@group(0) @binding(1)
var input_texture: texture_2d<f32>;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4<f32>(input.position, 0.0, 1.0);
    output.tex_coords = input.tex_coords;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(input_texture, texture_sampler, input.tex_coords).rgb;
    
    // Convert to grayscale using standard luminance weights
    // These weights account for human perception of different colors
    let luminance = dot(color, vec3<f32>(0.2126, 0.7152, 0.0722));
    
    return vec4<f32>(vec3<f32>(luminance), 1.0);
}
