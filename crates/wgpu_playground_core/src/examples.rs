/// Example gallery and metadata
///
/// This module defines preset examples with descriptions and source code

#[derive(Debug, Clone, PartialEq)]
pub enum ExampleCategory {
    Rendering,
    Compute,
}

#[derive(Debug, Clone)]
pub struct Example {
    pub id: &'static str,
    pub name: &'static str,
    pub category: ExampleCategory,
    pub description: &'static str,
    pub source_code: &'static str,
}

impl Example {
    pub const fn new(
        id: &'static str,
        name: &'static str,
        category: ExampleCategory,
        description: &'static str,
        source_code: &'static str,
    ) -> Self {
        Self {
            id,
            name,
            category,
            description,
            source_code,
        }
    }
}

/// Get all available examples
pub fn get_all_examples() -> Vec<Example> {
    vec![
        TRIANGLE_EXAMPLE.clone(),
        CUBE_EXAMPLE.clone(),
        TEXTURE_MAPPING_EXAMPLE.clone(),
        COMPUTE_SHADER_EXAMPLE.clone(),
    ]
}

/// Triangle rendering example
pub static TRIANGLE_EXAMPLE: Example = Example {
    id: "triangle",
    name: "Basic Triangle",
    category: ExampleCategory::Rendering,
    description: "Renders a simple colored triangle using vertex buffers and a basic shader. \
                  This is the classic \"Hello World\" of graphics programming, demonstrating \
                  the fundamental rendering pipeline setup.",
    source_code: r#"// Triangle Rendering Example
// This example demonstrates basic rendering with vertex buffers

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(in.position, 1.0);
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}

// Vertex data:
// Triangle vertices with positions and colors
// Vertex 0: position (0.0, 0.5, 0.0), color (1.0, 0.0, 0.0) - Red
// Vertex 1: position (-0.5, -0.5, 0.0), color (0.0, 1.0, 0.0) - Green
// Vertex 2: position (0.5, -0.5, 0.0), color (0.0, 0.0, 1.0) - Blue
"#,
};

/// Cube rendering example
pub static CUBE_EXAMPLE: Example = Example {
    id: "cube",
    name: "Rotating Cube",
    category: ExampleCategory::Rendering,
    description: "Renders a 3D rotating cube with depth testing and perspective projection. \
                  Demonstrates vertex/index buffers, uniform buffers for transformation matrices, \
                  and basic 3D rendering concepts.",
    source_code: r#"// Cube Rendering Example
// This example demonstrates 3D rendering with transformations

struct Uniforms {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let world_position = uniforms.model * vec4<f32>(in.position, 1.0);
    out.clip_position = uniforms.view_proj * world_position;
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}

// Cube vertices: 8 corners with positions and colors
// Index buffer: 36 indices (12 triangles, 2 per face)
// Uniform buffer: Contains model, view, and projection matrices
// The model matrix rotates over time
"#,
};

/// Texture mapping example
pub static TEXTURE_MAPPING_EXAMPLE: Example = Example {
    id: "texture_mapping",
    name: "Texture Mapping",
    category: ExampleCategory::Rendering,
    description: "Demonstrates texture sampling and UV coordinate mapping. Renders a textured quad \
                  showing how to load textures, create samplers, and apply textures in shaders.",
    source_code: r#"// Texture Mapping Example
// This example demonstrates texture sampling

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;

@group(0) @binding(1)
var s_diffuse: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(in.position, 1.0);
    out.tex_coords = in.tex_coords;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Sample the texture at the given UV coordinates
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}

// Quad vertices with UV coordinates (0,0) to (1,1)
// Texture: 2D image loaded from file
// Sampler: Linear filtering with repeat addressing mode
// Demonstrates bind groups for texture and sampler resources
"#,
};

/// Compute shader example
pub static COMPUTE_SHADER_EXAMPLE: Example = Example {
    id: "compute_shader",
    name: "Compute Shader",
    category: ExampleCategory::Compute,
    description: "Demonstrates GPU compute operations using compute shaders. Performs parallel \
                  computation on large datasets using storage buffers and workgroups.",
    source_code: r#"// Compute Shader Example
// This example demonstrates parallel computation on the GPU

@group(0) @binding(0)
var<storage, read> input: array<f32>;

@group(0) @binding(1)
var<storage, read_write> output: array<f32>;

@compute @workgroup_size(256)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    
    // Simple computation: square each input value
    // In practice, this could be matrix operations, image processing,
    // physics simulations, or ML inference
    if (index < arrayLength(&input)) {
        output[index] = input[index] * input[index];
    }
}

// Storage buffers:
// - Input buffer: Array of floats to process
// - Output buffer: Results written by compute shader
// 
// Dispatch: Calculate workgroup count based on data size
// workgroup_count = (data_size + 255) / 256
//
// This demonstrates:
// - Storage buffer usage
// - Workgroup sizing
// - Parallel data processing
// - GPU compute pipeline
"#,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_examples_exist() {
        let examples = get_all_examples();
        assert_eq!(examples.len(), 4);
    }

    #[test]
    fn test_example_ids_unique() {
        let examples = get_all_examples();
        let ids: Vec<&str> = examples.iter().map(|e| e.id).collect();
        let mut unique_ids = ids.clone();
        unique_ids.sort();
        unique_ids.dedup();
        assert_eq!(ids.len(), unique_ids.len(), "Example IDs must be unique");
    }

    #[test]
    fn test_triangle_example() {
        assert_eq!(TRIANGLE_EXAMPLE.id, "triangle");
        assert_eq!(TRIANGLE_EXAMPLE.name, "Basic Triangle");
        assert_eq!(TRIANGLE_EXAMPLE.category, ExampleCategory::Rendering);
        assert!(!TRIANGLE_EXAMPLE.description.is_empty());
        assert!(!TRIANGLE_EXAMPLE.source_code.is_empty());
    }

    #[test]
    fn test_cube_example() {
        assert_eq!(CUBE_EXAMPLE.id, "cube");
        assert_eq!(CUBE_EXAMPLE.name, "Rotating Cube");
        assert_eq!(CUBE_EXAMPLE.category, ExampleCategory::Rendering);
    }

    #[test]
    fn test_texture_mapping_example() {
        assert_eq!(TEXTURE_MAPPING_EXAMPLE.id, "texture_mapping");
        assert_eq!(TEXTURE_MAPPING_EXAMPLE.name, "Texture Mapping");
        assert_eq!(TEXTURE_MAPPING_EXAMPLE.category, ExampleCategory::Rendering);
    }

    #[test]
    fn test_compute_shader_example() {
        assert_eq!(COMPUTE_SHADER_EXAMPLE.id, "compute_shader");
        assert_eq!(COMPUTE_SHADER_EXAMPLE.name, "Compute Shader");
        assert_eq!(COMPUTE_SHADER_EXAMPLE.category, ExampleCategory::Compute);
    }
}
