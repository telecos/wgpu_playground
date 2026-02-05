// Blur post-processing shader
// Applies a simple box blur effect to the input texture

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
    // Simple box blur with 9-tap kernel (3x3)
    let texel_size = 1.0 / vec2<f32>(textureDimensions(input_texture));
    
    var color = vec3<f32>(0.0);
    var total_weight = 0.0;
    
    // 3x3 box blur kernel
    for (var x = -1; x <= 1; x++) {
        for (var y = -1; y <= 1; y++) {
            let offset = vec2<f32>(f32(x), f32(y)) * texel_size;
            let sample_coords = input.tex_coords + offset;
            color += textureSample(input_texture, texture_sampler, sample_coords).rgb;
            total_weight += 1.0;
        }
    }
    
    color /= total_weight;
    return vec4<f32>(color, 1.0);
}
