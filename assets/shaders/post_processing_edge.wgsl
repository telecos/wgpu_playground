// Edge detection post-processing shader
// Applies Sobel edge detection to the input texture

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
    let texel_size = 1.0 / vec2<f32>(textureDimensions(input_texture));
    
    // Sample the 3x3 neighborhood
    let tl = textureSample(input_texture, texture_sampler, input.tex_coords + vec2<f32>(-1.0, -1.0) * texel_size).r;
    let tc = textureSample(input_texture, texture_sampler, input.tex_coords + vec2<f32>(0.0, -1.0) * texel_size).r;
    let tr = textureSample(input_texture, texture_sampler, input.tex_coords + vec2<f32>(1.0, -1.0) * texel_size).r;
    
    let ml = textureSample(input_texture, texture_sampler, input.tex_coords + vec2<f32>(-1.0, 0.0) * texel_size).r;
    let mr = textureSample(input_texture, texture_sampler, input.tex_coords + vec2<f32>(1.0, 0.0) * texel_size).r;
    
    let bl = textureSample(input_texture, texture_sampler, input.tex_coords + vec2<f32>(-1.0, 1.0) * texel_size).r;
    let bc = textureSample(input_texture, texture_sampler, input.tex_coords + vec2<f32>(0.0, 1.0) * texel_size).r;
    let br = textureSample(input_texture, texture_sampler, input.tex_coords + vec2<f32>(1.0, 1.0) * texel_size).r;
    
    // Sobel operator
    let gx = -tl - 2.0 * ml - bl + tr + 2.0 * mr + br;
    let gy = -tl - 2.0 * tc - tr + bl + 2.0 * bc + br;
    
    // Calculate edge magnitude
    let edge = sqrt(gx * gx + gy * gy);
    
    // Invert for white edges on black background
    let final_color = vec3<f32>(edge);
    
    return vec4<f32>(final_color, 1.0);
}
