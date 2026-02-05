// Particle System Render Shader
//
// This shader renders particles using instanced rendering:
// - Each particle is rendered as a textured quad
// - Per-instance attributes: position, color, size, lifetime
// - Alpha blending for nice particle effects

// Vertex input from quad geometry (per-vertex)
struct VertexInput {
    @location(0) quad_pos: vec2<f32>,
}

// Vertex input from particle data (per-instance)
struct InstanceInput {
    @location(1) position: vec2<f32>,
    @location(2) velocity: vec2<f32>,  // not used in rendering but part of struct
    @location(3) color: vec4<f32>,
    @location(4) lifetime: f32,
    @location(5) size: f32,
}

// Vertex shader output / Fragment shader input
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) quad_uv: vec2<f32>,
}

@vertex
fn vs_main(
    vertex: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    
    // Skip dead particles (lifetime <= 0)
    if (instance.lifetime <= 0.0) {
        // Position off-screen
        out.clip_position = vec4<f32>(-10.0, -10.0, 0.0, 1.0);
        out.color = vec4<f32>(0.0, 0.0, 0.0, 0.0);
        out.quad_uv = vec2<f32>(0.0, 0.0);
        return out;
    }
    
    // Scale the quad by particle size
    let scaled_quad = vertex.quad_pos * instance.size;
    
    // Translate to particle position
    let world_pos = instance.position + scaled_quad;
    
    // Output clip position (already in clip space [-1, 1])
    out.clip_position = vec4<f32>(world_pos, 0.0, 1.0);
    
    // Pass color to fragment shader
    out.color = instance.color;
    
    // Convert quad position to UV coordinates [0, 1]
    out.quad_uv = vertex.quad_pos * 0.5 + 0.5;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Calculate distance from center of quad
    let center = vec2<f32>(0.5, 0.5);
    let dist = distance(in.quad_uv, center);
    
    // Create circular particle shape (soft circle)
    let radius = 0.5;
    let softness = 0.2;
    let alpha = 1.0 - smoothstep(radius - softness, radius, dist);
    
    // Apply the circular mask to the particle color
    var color = in.color;
    color.a *= alpha;
    
    // Discard fully transparent pixels (optimization)
    if (color.a < 0.01) {
        discard;
    }
    
    return color;
}
