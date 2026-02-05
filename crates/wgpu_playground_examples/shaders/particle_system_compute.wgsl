// Particle System Compute Shader
//
// This shader updates particle physics on the GPU:
// - Integrates velocity into position
// - Applies gravity and damping
// - Updates lifetime
// - Performs boundary checks

// Particle structure (must match Rust struct)
struct Particle {
    position: vec2<f32>,
    velocity: vec2<f32>,
    color: vec4<f32>,
    lifetime: f32,
    size: f32,
    _padding: vec2<f32>,
}

// Simulation parameters
struct SimulationParams {
    delta_time: f32,
    gravity: f32,
    damping: f32,
    _padding: f32,
}

// Particle buffer (read-write storage)
@group(0) @binding(0)
var<storage, read_write> particles: array<Particle>;

// Simulation parameters (read-only uniform)
@group(0) @binding(1)
var<uniform> params: SimulationParams;

@compute @workgroup_size(256)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    
    // Bounds check
    if (index >= arrayLength(&particles)) {
        return;
    }
    
    // Skip dead particles
    if (particles[index].lifetime <= 0.0) {
        return;
    }
    
    // Update lifetime
    particles[index].lifetime -= params.delta_time;
    
    // Kill particle if lifetime expired
    if (particles[index].lifetime <= 0.0) {
        particles[index].lifetime = 0.0;
        particles[index].color.a = 0.0;
        return;
    }
    
    // Apply gravity (downward force)
    particles[index].velocity.y += params.gravity * params.delta_time;
    
    // Apply damping (air resistance)
    particles[index].velocity *= params.damping;
    
    // Integrate velocity into position
    particles[index].position += particles[index].velocity * params.delta_time;
    
    // Boundary checks with bounce
    // Left/Right boundaries
    if (particles[index].position.x < -1.0) {
        particles[index].position.x = -1.0;
        particles[index].velocity.x *= -0.5; // bounce with energy loss
    } else if (particles[index].position.x > 1.0) {
        particles[index].position.x = 1.0;
        particles[index].velocity.x *= -0.5;
    }
    
    // Top/Bottom boundaries
    if (particles[index].position.y < -1.0) {
        particles[index].position.y = -1.0;
        particles[index].velocity.y *= -0.3; // less bouncy on ground
    } else if (particles[index].position.y > 1.0) {
        particles[index].position.y = 1.0;
        particles[index].velocity.y *= -0.5;
    }
    
    // Fade out particles as they approach death
    // This creates a nice visual effect
    let lifetime_ratio = particles[index].lifetime / 4.0; // assume max lifetime is ~4 seconds
    particles[index].color.a = min(lifetime_ratio, 1.0);
}
