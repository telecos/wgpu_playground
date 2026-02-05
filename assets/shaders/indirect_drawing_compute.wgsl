// Compute shader for generating indirect draw parameters
// This demonstrates GPU-driven rendering where draw parameters are computed on the GPU

// DrawIndirect structure matches wgpu::util::DrawIndirect
// struct DrawIndirect {
//     vertex_count: u32,     // Number of vertices to draw
//     instance_count: u32,   // Number of instances to draw
//     first_vertex: u32,     // First vertex to draw
//     first_instance: u32,   // First instance to draw
// }

// DrawIndexedIndirect structure matches wgpu::util::DrawIndexedIndirect
// struct DrawIndexedIndirect {
//     index_count: u32,      // Number of indices to draw
//     instance_count: u32,   // Number of instances to draw
//     first_index: u32,      // First index to draw
//     base_vertex: i32,      // Value added to vertex index before indexing into vertex buffer
//     first_instance: u32,   // First instance to draw
// }

@group(0) @binding(0)
var<storage, read_write> draw_params: array<u32>;

@group(0) @binding(1)
var<storage, read_write> indexed_draw_params: array<u32>;

@compute @workgroup_size(1)
fn generate_draw_params() {
    // Generate parameters for drawIndirect
    // Draw 3 vertices (triangle), 1 instance, starting at vertex 0
    draw_params[0] = 3u;  // vertex_count
    draw_params[1] = 1u;  // instance_count
    draw_params[2] = 0u;  // first_vertex
    draw_params[3] = 0u;  // first_instance
    
    // Generate parameters for drawIndexedIndirect
    // Draw 6 indices (two triangles/quad), 1 instance
    indexed_draw_params[0] = 6u;  // index_count
    indexed_draw_params[1] = 1u;  // instance_count
    indexed_draw_params[2] = 0u;  // first_index
    indexed_draw_params[3] = 0;   // base_vertex (i32 cast as u32)
    indexed_draw_params[4] = 0u;  // first_instance
}

@compute @workgroup_size(1)
fn generate_multi_instance_params() {
    // Generate parameters for multiple instances
    draw_params[0] = 3u;  // vertex_count - 3 vertices per triangle
    draw_params[1] = 4u;  // instance_count - draw 4 instances
    draw_params[2] = 0u;  // first_vertex
    draw_params[3] = 0u;  // first_instance
}
