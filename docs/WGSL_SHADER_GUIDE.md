# WGSL Shader Development Guide

This guide covers writing WGSL (WebGPU Shading Language) shaders in the wgpu_playground. Whether you're new to WGSL or looking to understand advanced features, this guide will help you write effective shaders.

## Table of Contents

- [Introduction to WGSL](#introduction-to-wgsl)
- [Shader Structure](#shader-structure)
- [Data Types](#data-types)
- [Built-in Functions](#built-in-functions)
- [Vertex Shaders](#vertex-shaders)
- [Fragment Shaders](#fragment-shaders)
- [Compute Shaders](#compute-shaders)
- [Uniforms and Bindings](#uniforms-and-bindings)
- [Debugging Techniques](#debugging-techniques)
- [Best Practices](#best-practices)
- [Common Patterns](#common-patterns)
- [Resources](#resources)

## Introduction to WGSL

WGSL (WebGPU Shading Language) is the shader language for WebGPU. It's designed to be:

- **Safe**: Strong typing with compile-time validation
- **Portable**: Works consistently across different GPU vendors
- **Modern**: Clean syntax inspired by Rust and other modern languages
- **Explicit**: Clear about data layout, memory access, and pipeline stages

### Why WGSL?

Unlike GLSL or HLSL, WGSL is the native shader language for WebGPU, offering:
- Predictable behavior across platforms
- Better error messages during compilation
- Modern language features (structs, functions, clear entry points)
- Direct mapping to GPU execution model

## Shader Structure

### Basic Shader Anatomy

A WGSL shader consists of:

1. **Type definitions** (structs)
2. **Global variables** (uniforms, textures, samplers)
3. **Functions** (including entry points)
4. **Entry point attributes** (`@vertex`, `@fragment`, `@compute`)

**Example: Simple Triangle Shader**

```wgsl
// Type definition
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

// Vertex shader entry point
@vertex
fn vs_main(
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>,
) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4<f32>(position, 0.0, 1.0);
    output.color = color;
    return output;
}

// Fragment shader entry point
@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(input.color, 1.0);
}
```

### Entry Points

WGSL uses attributes to mark shader entry points:

- **`@vertex`**: Vertex shader (processes each vertex)
- **`@fragment`**: Fragment/pixel shader (colors each pixel)
- **`@compute`**: Compute shader (general-purpose GPU computation)

**Important**: Each shader module can have multiple entry points. When creating a render pipeline, you specify which entry point to use via the `entry_point` parameter.

## Data Types

### Scalar Types

```wgsl
// Boolean
var flag: bool = true;

// Signed integers (32-bit)
var count: i32 = 42;

// Unsigned integers (32-bit)
var index: u32 = 10u;

// Floating point (32-bit)
var temperature: f32 = 98.6;
```

### Vector Types

WGSL provides 2, 3, and 4-component vectors:

```wgsl
// 2D vector
var position: vec2<f32> = vec2<f32>(0.5, 0.5);

// 3D vector (common for RGB colors or 3D positions)
var color: vec3<f32> = vec3<f32>(1.0, 0.0, 0.0);  // Red

// 4D vector (common for RGBA colors or homogeneous coordinates)
var rgba: vec4<f32> = vec4<f32>(1.0, 0.5, 0.0, 1.0);  // Orange
```

**Vector constructors:**

```wgsl
// From scalar (broadcasts to all components)
var ones = vec3<f32>(1.0);  // (1.0, 1.0, 1.0)

// From components
var pos = vec3<f32>(1.0, 2.0, 3.0);

// Mixed (swizzling)
var xy = vec2<f32>(1.0, 2.0);
var xyz = vec3<f32>(xy, 3.0);  // Combines vec2 with scalar
```

**Vector swizzling:**

```wgsl
var color = vec4<f32>(1.0, 0.5, 0.2, 1.0);
var rgb = color.rgb;    // vec3<f32>(1.0, 0.5, 0.2)
var rg = color.rg;      // vec2<f32>(1.0, 0.5)
var alpha = color.a;    // f32: 1.0

// Also works with xyzw notation
var position = vec3<f32>(1.0, 2.0, 3.0);
var xy = position.xy;   // vec2<f32>(1.0, 2.0)
```

### Matrix Types

Matrices are column-major by default:

```wgsl
// 2x2 matrix
var m2: mat2x2<f32> = mat2x2<f32>(
    1.0, 0.0,  // First column
    0.0, 1.0   // Second column
);

// 3x3 matrix
var m3: mat3x3<f32>;

// 4x4 matrix (common for transformations)
var transform: mat4x4<f32> = mat4x4<f32>(
    1.0, 0.0, 0.0, 0.0,  // Column 1
    0.0, 1.0, 0.0, 0.0,  // Column 2
    0.0, 0.0, 1.0, 0.0,  // Column 3
    0.0, 0.0, 0.0, 1.0   // Column 4
);
```

### Arrays

```wgsl
// Fixed-size array
var positions: array<vec2<f32>, 3> = array<vec2<f32>, 3>(
    vec2<f32>(0.0, 0.5),
    vec2<f32>(-0.5, -0.5),
    vec2<f32>(0.5, -0.5)
);

// Accessing array elements
let first_pos = positions[0];
```

### Structs

Structs group related data:

```wgsl
struct Particle {
    position: vec3<f32>,
    velocity: vec3<f32>,
    color: vec4<f32>,
    lifetime: f32,
}

// Creating struct instances
var p: Particle;
p.position = vec3<f32>(0.0, 0.0, 0.0);
p.velocity = vec3<f32>(1.0, 0.0, 0.0);
p.color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
p.lifetime = 5.0;
```

## Built-in Functions

### Mathematical Functions

**Basic arithmetic:**
```wgsl
abs(x)          // Absolute value
sign(x)         // Sign (-1, 0, or 1)
floor(x)        // Round down
ceil(x)         // Round up
round(x)        // Round to nearest
fract(x)        // Fractional part
min(a, b)       // Minimum
max(a, b)       // Maximum
clamp(x, min, max)  // Clamp to range
```

**Trigonometry:**
```wgsl
sin(x)          // Sine
cos(x)          // Cosine
tan(x)          // Tangent
asin(x)         // Arc sine
acos(x)         // Arc cosine
atan(x)         // Arc tangent
atan2(y, x)     // Arc tangent of y/x
```

**Exponentials and logarithms:**
```wgsl
pow(x, y)       // x raised to power y
exp(x)          // e raised to power x
exp2(x)         // 2 raised to power x
log(x)          // Natural logarithm
log2(x)         // Base-2 logarithm
sqrt(x)         // Square root
inverseSqrt(x)  // 1 / sqrt(x)
```

### Vector Functions

```wgsl
// Length and normalization
length(v)       // Vector length
distance(a, b)  // Distance between points
normalize(v)    // Unit vector in same direction

// Dot and cross products
dot(a, b)       // Dot product
cross(a, b)     // Cross product (vec3 only)

// Interpolation
mix(a, b, t)    // Linear interpolation: a * (1-t) + b * t
smoothstep(edge0, edge1, x)  // Smooth Hermite interpolation
step(edge, x)   // 0 if x < edge, else 1

// Component-wise operations
reflect(v, n)   // Reflect v across normal n
refract(v, n, eta)  // Refract v through surface with normal n
```

**Example: Normalizing a vector**
```wgsl
var direction = vec3<f32>(3.0, 4.0, 0.0);
var unit_direction = normalize(direction);  // vec3(0.6, 0.8, 0.0)
```

### Matrix Functions

```wgsl
// Matrix operations
transpose(m)    // Transpose matrix
determinant(m)  // Matrix determinant

// Matrix-vector multiplication
var transformed = matrix * vector;
```

### Texture Sampling Functions

```wgsl
// Sample texture at UV coordinates
textureSample(t, s, coords)        // Sample with sampler
textureLoad(t, coords, level)      // Load without sampler (integer coords)
textureDimensions(t)               // Get texture size
textureNumLevels(t)                // Get mipmap level count
```

**Example: Sample a 2D texture**
```wgsl
@group(0) @binding(0) var my_texture: texture_2d<f32>;
@group(0) @binding(1) var my_sampler: sampler;

@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    return textureSample(my_texture, my_sampler, uv);
}
```

### Utility Functions

```wgsl
// Select based on condition (per-component for vectors)
select(false_value, true_value, condition)

// All/any for boolean vectors
all(bvec)       // True if all components are true
any(bvec)       // True if any component is true

// Bitwise operations
countOneBits(x)     // Count number of 1 bits
reverseBits(x)      // Reverse bit order
```

## Vertex Shaders

Vertex shaders process each vertex and output its position in clip space.

### Basic Vertex Shader

```wgsl
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = vec4<f32>(input.position, 1.0);
    output.color = input.color;
    return output;
}
```

### Vertex Shader with Transformations

```wgsl
struct Uniforms {
    model_view_proj: mat4x4<f32>,
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
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    // Apply transformation matrix
    output.clip_position = uniforms.model_view_proj * vec4<f32>(input.position, 1.0);
    output.color = input.color;
    return output;
}
```

### Built-in Vertex Inputs

```wgsl
@vertex
fn vs_main(
    @builtin(vertex_index) vertex_idx: u32,      // Current vertex index
    @builtin(instance_index) instance_idx: u32,  // Current instance index
) -> @builtin(position) vec4<f32> {
    // Generate vertex positions procedurally
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 0.5),
        vec2<f32>(-0.5, -0.5),
        vec2<f32>(0.5, -0.5)
    );
    let pos = positions[vertex_idx];
    return vec4<f32>(pos, 0.0, 1.0);
}
```

## Fragment Shaders

Fragment shaders determine the color of each pixel.

### Basic Fragment Shader

```wgsl
@fragment
fn fs_main(@location(0) color: vec3<f32>) -> @location(0) vec4<f32> {
    // Output color with full opacity
    return vec4<f32>(color, 1.0);
}
```

### Fragment Shader with Texture

```wgsl
@group(0) @binding(0) var my_texture: texture_2d<f32>;
@group(0) @binding(1) var my_sampler: sampler;

struct FragmentInput {
    @location(0) uv: vec2<f32>,
    @location(1) color: vec3<f32>,
}

@fragment
fn fs_main(input: FragmentInput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(my_texture, my_sampler, input.uv);
    // Multiply texture color by vertex color
    return tex_color * vec4<f32>(input.color, 1.0);
}
```

### Fragment Shader with Lighting

```wgsl
struct FragmentInput {
    @location(0) world_position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec3<f32>,
}

@fragment
fn fs_main(input: FragmentInput) -> @location(0) vec4<f32> {
    // Simple directional light
    let light_dir = normalize(vec3<f32>(1.0, 1.0, 1.0));
    let normal = normalize(input.normal);
    
    // Diffuse lighting (Lambertian)
    let diffuse = max(dot(normal, light_dir), 0.0);
    
    // Ambient lighting
    let ambient = 0.2;
    
    // Combine lighting
    let lighting = ambient + diffuse;
    let final_color = input.color * lighting;
    
    return vec4<f32>(final_color, 1.0);
}
```

### Built-in Fragment Inputs

```wgsl
@fragment
fn fs_main(
    @builtin(position) frag_coord: vec4<f32>,  // Screen-space position
    @builtin(front_facing) is_front: bool,     // Is front-facing?
) -> @location(0) vec4<f32> {
    // Use fragment coordinate for effects
    let x = frag_coord.x / 800.0;  // Normalize by screen width
    let y = frag_coord.y / 600.0;  // Normalize by screen height
    
    // Different color for front/back faces
    if is_front {
        return vec4<f32>(x, y, 0.0, 1.0);
    } else {
        return vec4<f32>(1.0 - x, 1.0 - y, 1.0, 1.0);
    }
}
```

## Compute Shaders

Compute shaders perform general-purpose computation on the GPU.

### Basic Compute Shader

```wgsl
@group(0) @binding(0)
var<storage, read_write> data: array<f32>;

@compute
@workgroup_size(64)
fn cs_main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    // Double each value
    data[index] = data[index] * 2.0;
}
```

### Particle Update Compute Shader

```wgsl
struct Particle {
    position: vec3<f32>,
    velocity: vec3<f32>,
}

@group(0) @binding(0)
var<storage, read_write> particles: array<Particle>;

@group(0) @binding(1)
var<uniform> delta_time: f32;

@compute
@workgroup_size(64)
fn cs_main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    
    // Update position based on velocity
    particles[index].position += particles[index].velocity * delta_time;
    
    // Apply gravity
    particles[index].velocity.y -= 9.8 * delta_time;
    
    // Bounce off ground
    if particles[index].position.y < 0.0 {
        particles[index].position.y = 0.0;
        particles[index].velocity.y = -particles[index].velocity.y * 0.8;
    }
}
```

### Built-in Compute Inputs

```wgsl
@compute
@workgroup_size(8, 8, 1)
fn cs_main(
    @builtin(global_invocation_id) global_id: vec3<u32>,      // Global thread ID
    @builtin(local_invocation_id) local_id: vec3<u32>,        // Local thread ID within workgroup
    @builtin(workgroup_id) workgroup_id: vec3<u32>,           // Workgroup ID
    @builtin(local_invocation_index) local_index: u32,        // Flattened local thread ID
    @builtin(num_workgroups) num_workgroups: vec3<u32>,       // Number of workgroups dispatched
) {
    // Process 2D image
    let pixel_x = global_id.x;
    let pixel_y = global_id.y;
    
    // ... process pixel at (pixel_x, pixel_y) ...
}
```

## Uniforms and Bindings

### Uniform Buffers

Uniform buffers provide read-only data to shaders:

```wgsl
struct CameraUniforms {
    view_proj: mat4x4<f32>,
    position: vec3<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniforms;

@vertex
fn vs_main(@location(0) position: vec3<f32>) -> @builtin(position) vec4<f32> {
    return camera.view_proj * vec4<f32>(position, 1.0);
}
```

### Storage Buffers

Storage buffers allow read/write access (in compute shaders):

```wgsl
// Read-only storage buffer
@group(0) @binding(0)
var<storage, read> input_data: array<f32>;

// Read-write storage buffer
@group(0) @binding(1)
var<storage, read_write> output_data: array<f32>;

@compute
@workgroup_size(64)
fn cs_main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    output_data[index] = input_data[index] * 2.0;
}
```

### Textures and Samplers

```wgsl
// Texture and sampler bindings
@group(0) @binding(0) var my_texture: texture_2d<f32>;
@group(0) @binding(1) var my_sampler: sampler;

// Different texture types
var tex_2d: texture_2d<f32>;           // 2D texture
var tex_2d_array: texture_2d_array<f32>;  // 2D texture array
var tex_cube: texture_cube<f32>;       // Cubemap texture
var tex_3d: texture_3d<f32>;           // 3D texture
var depth_tex: texture_depth_2d;       // Depth texture

@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    return textureSample(my_texture, my_sampler, uv);
}
```

### Bind Groups

Bind groups organize related resources:

```wgsl
// Bind group 0: Camera uniforms
@group(0) @binding(0)
var<uniform> camera: CameraUniforms;

// Bind group 1: Material properties
@group(1) @binding(0) var base_color_texture: texture_2d<f32>;
@group(1) @binding(1) var base_color_sampler: sampler;
@group(1) @binding(2)
var<uniform> material: MaterialUniforms;
```

## Debugging Techniques

### 1. Color-Based Debugging

Output specific colors to visualize values:

```wgsl
@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    // Visualize UV coordinates as colors
    return vec4<f32>(uv.x, uv.y, 0.0, 1.0);
}
```

**Debugging normals:**
```wgsl
@fragment
fn fs_main(@location(0) normal: vec3<f32>) -> @location(0) vec4<f32> {
    // Map normal from [-1, 1] to [0, 1] for visualization
    let color = (normal + 1.0) * 0.5;
    return vec4<f32>(color, 1.0);
}
```

### 2. Value Range Visualization

Use color gradients to visualize scalar values:

```wgsl
@fragment
fn fs_main(@location(0) depth: f32) -> @location(0) vec4<f32> {
    // Visualize depth as grayscale
    let gray = clamp(depth, 0.0, 1.0);
    return vec4<f32>(gray, gray, gray, 1.0);
}
```

### 3. Conditional Coloring

Highlight specific conditions:

```wgsl
@fragment
fn fs_main(@location(0) value: f32) -> @location(0) vec4<f32> {
    // Red if negative, green if positive
    if value < 0.0 {
        return vec4<f32>(1.0, 0.0, 0.0, 1.0);  // Red
    } else {
        return vec4<f32>(0.0, 1.0, 0.0, 1.0);  // Green
    }
}
```

### 4. Step-by-Step Isolation

Comment out parts of your shader to isolate issues:

```wgsl
@fragment
fn fs_main(input: FragmentInput) -> @location(0) vec4<f32> {
    // var final_color = compute_lighting(input);
    // var final_color = apply_fog(final_color, input.depth);
    
    // Test: Just return base color
    var final_color = input.color;
    
    return vec4<f32>(final_color, 1.0);
}
```

### 5. Validation Messages

Check wgpu compilation output for detailed error messages:

```rust
// In your Rust code
let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
    label: Some("my_shader"),
    source: wgpu::ShaderSource::Wgsl(shader_code.into()),
});

// wgpu will print detailed errors if compilation fails
```

### 6. Print Debugging (Limited)

WGSL doesn't have print statements, but you can:

- Output values as colors (see #1 above)
- Write values to storage buffers and read them back on the CPU
- Use GPU debuggers like RenderDoc or PIX

### 7. Simplified Test Cases

Create minimal test shaders:

```wgsl
// Minimal vertex shader
@vertex
fn vs_main(@builtin(vertex_index) idx: u32) -> @builtin(position) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);  // Single point at origin
}

// Minimal fragment shader
@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 1.0, 1.0);  // Solid magenta
}
```

### 8. Using RenderDoc

RenderDoc is a graphics debugger that can:
- Capture frames
- Inspect shader inputs/outputs
- View texture contents
- Profile GPU performance

To use with wgpu_playground:
1. Install RenderDoc
2. Launch wgpu_playground through RenderDoc
3. Capture a frame (F12 by default)
4. Inspect shader stages and resources

## Best Practices

### 1. Use Descriptive Names

```wgsl
// Good
struct CameraTransform {
    view_matrix: mat4x4<f32>,
    projection_matrix: mat4x4<f32>,
}

// Avoid
struct CT {
    vm: mat4x4<f32>,
    pm: mat4x4<f32>,
}
```

### 2. Group Related Data in Structs

```wgsl
// Good
struct Material {
    base_color: vec4<f32>,
    metallic: f32,
    roughness: f32,
}

// Less organized
@group(1) @binding(0)
var<uniform> base_color: vec4<f32>;
@group(1) @binding(1)
var<uniform> metallic: f32;
@group(1) @binding(2)
var<uniform> roughness: f32>;
```

### 3. Normalize Your Vectors

Always normalize vectors before using them in lighting calculations:

```wgsl
let normal = normalize(input.normal);
let light_dir = normalize(light_position - input.world_pos);
let view_dir = normalize(camera_pos - input.world_pos);
```

### 4. Use `let` for Read-Only Values

```wgsl
// Use 'let' for values that won't change
let half_size = size * 0.5;
let color_factor = 1.0 / 255.0;

// Use 'var' only when you need to modify the value
var accumulator = 0.0;
accumulator += value1;
accumulator += value2;
```

### 5. Avoid Complex Branching in Fragment Shaders

Fragment shaders run for every pixel, so branches can hurt performance:

```wgsl
// Try to avoid
if (condition1) {
    if (condition2) {
        // deeply nested logic
    }
}

// Prefer simpler logic or use select()
let result = select(false_value, true_value, condition);
```

### 6. Leverage Built-in Functions

Use built-in functions instead of implementing your own:

```wgsl
// Good: Use built-in
let distance = distance(point1, point2);
let normalized = normalize(vector);

// Avoid: Reimplementing
let diff = point1 - point2;
let distance = sqrt(dot(diff, diff));
```

### 7. Mind Precision

Be aware of floating-point precision:

```wgsl
// May lose precision
let very_small = 0.000001;
let very_large = 1000000.0;
let result = very_large + very_small;  // May not change very_large

// Use appropriate ranges and consider normalizing values
```

### 8. Comment Non-Obvious Code

```wgsl
// Good: Explain the "why"
// Convert from tangent space to world space
let normal_world = normalize(tbn_matrix * normal_tangent);

// Avoid obvious comments
// Add velocity to position
position += velocity;
```

## Common Patterns

### Transformation Matrix Helper

```wgsl
// Create rotation matrix around Y axis
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

// Create translation matrix
fn translation_matrix(offset: vec3<f32>) -> mat4x4<f32> {
    return mat4x4<f32>(
        vec4<f32>(1.0, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, 1.0, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(offset.x, offset.y, offset.z, 1.0),
    );
}
```

### Procedural Texture Generation

```wgsl
// Checkerboard pattern
fn checkerboard(uv: vec2<f32>, frequency: f32) -> f32 {
    let checker = floor(uv.x * frequency) + floor(uv.y * frequency);
    return fract(checker * 0.5) * 2.0;  // 0.0 or 1.0
}

@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let pattern = checkerboard(uv, 8.0);
    return vec4<f32>(pattern, pattern, pattern, 1.0);
}
```

### Simple Gradient

```wgsl
@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let color1 = vec3<f32>(1.0, 0.0, 0.0);  // Red
    let color2 = vec3<f32>(0.0, 0.0, 1.0);  // Blue
    let color = mix(color1, color2, uv.y);
    return vec4<f32>(color, 1.0);
}
```

### Phong Lighting

```wgsl
struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
}

fn calculate_phong(
    normal: vec3<f32>,
    view_dir: vec3<f32>,
    light_dir: vec3<f32>,
    light_color: vec3<f32>,
    material_color: vec3<f32>,
) -> vec3<f32> {
    // Ambient
    let ambient = 0.1 * material_color;
    
    // Diffuse
    let diffuse = max(dot(normal, light_dir), 0.0) * material_color * light_color;
    
    // Specular
    let reflect_dir = reflect(-light_dir, normal);
    let spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32.0);
    let specular = spec * light_color;
    
    return ambient + diffuse + specular;
}
```

## Resources

### Official Documentation

- **[WGSL Specification](https://www.w3.org/TR/WGSL/)** - Complete language specification
- **[WebGPU Specification](https://www.w3.org/TR/webgpu/)** - WebGPU API documentation
- **[wgpu Documentation](https://docs.rs/wgpu/)** - Rust wgpu crate documentation

### Tutorials

- **[WebGPU Fundamentals](https://webgpufundamentals.org/)** - Comprehensive WebGPU and WGSL tutorials
- **[Learn wgpu](https://sotrh.github.io/learn-wgpu/)** - Rust-focused WebGPU tutorial
- **[WebGPU Samples](https://webgpu.github.io/webgpu-samples/)** - Official WebGPU sample collection

### Tools

- **[RenderDoc](https://renderdoc.org/)** - Graphics debugger for frame capture and analysis
- **[WGSL Playground](https://takahirox.github.io/wgsl-sandbox/)** - Online WGSL shader editor

### Related Documentation in This Repository

- **[Shader Editor Guide](SHADER_EDITOR.md)** - Using the built-in shader editor
- **[Shader API Documentation](../SHADER_API.md)** - Shader loading API reference
- **[User Guide](USER_GUIDE.md)** - General playground usage
- **[Quick Start Guide](QUICK_START.md)** - Get started quickly

## Example Shaders in This Repository

The `assets/shaders/` directory contains several example shaders:

- **`triangle.wgsl`** - Basic vertex and fragment shader
- **`rotating_cube.wgsl`** - 3D transformations with uniform buffers
- **`textured_quad.wgsl`** - Texture sampling
- **`instanced_rendering.wgsl`** - Per-instance attributes and transformations
- **`multisampling.wgsl`** - Anti-aliasing example
- **`render_to_texture_scene.wgsl`** - Multi-pass rendering

Load these shaders in the shader editor to study real-world examples!

---

**Happy shader coding!** If you have questions or run into issues, check the [User Guide](USER_GUIDE.md) or consult the official WGSL specification.
