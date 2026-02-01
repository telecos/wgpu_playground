# Compute Pipeline Configuration UI

## Overview

The Compute Pipeline Configuration UI provides a comprehensive interface for creating and configuring WebGPU compute pipelines. This feature enables users to experiment with compute shaders, configure pipeline parameters, and understand compute pipeline creation.

## Features

### 1. Pipeline Properties

**Pipeline Label**
- Optional label for debugging purposes
- Helps identify pipelines in GPU profiling tools
- Default: empty (displayed as "unlabeled" in success messages)

**Entry Point**
- The name of the compute function in the shader
- Default: "main"
- Must match a function decorated with `@compute` in the shader

### 2. Shader Module Configuration

**Shader Label**
- Optional label for the shader module
- Default: "compute_shader"

**Shader Source Code Editor**
- Multi-line code editor with monospace font
- Syntax is WGSL (WebGPU Shading Language)
- Real-time validation on user request
- Support for complex compute shaders

**Built-in Templates**

The UI provides three shader templates to help users get started:

1. **Simple Compute**
   ```wgsl
   // Simple compute shader template
   @compute @workgroup_size(64)
   fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
       // Add your compute logic here
   }
   ```
   - Basic template with 64 work items per workgroup
   - Good starting point for learning compute shaders

2. **Storage Buffer**
   ```wgsl
   // Compute shader with storage buffer
   @group(0) @binding(0)
   var<storage, read_write> data: array<f32>;

   @compute @workgroup_size(64)
   fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
       let index = global_id.x;
       // Example: multiply each element by 2
       data[index] = data[index] * 2.0;
   }
   ```
   - Demonstrates storage buffer usage
   - Shows how to read and write GPU memory
   - Useful for parallel data processing

3. **Matrix Multiply**
   ```wgsl
   // Matrix multiplication compute shader
   @group(0) @binding(0)
   var<storage, read> matrix_a: array<f32>;

   @group(0) @binding(1)
   var<storage, read> matrix_b: array<f32>;

   @group(0) @binding(2)
   var<storage, read_write> matrix_result: array<f32>;

   // Uniforms for matrix dimensions
   @group(0) @binding(3)
   var<uniform> dimensions: vec3<u32>; // (M, N, K)

   @compute @workgroup_size(8, 8, 1)
   fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
       let row = global_id.x;
       let col = global_id.y;
       
       let M = dimensions.x;
       let N = dimensions.y;
       let K = dimensions.z;
       
       if (row >= M || col >= N) {
           return;
       }
       
       var sum = 0.0;
       for (var i = 0u; i < K; i = i + 1u) {
           sum = sum + matrix_a[row * K + i] * matrix_b[i * N + col];
       }
       
       matrix_result[row * N + col] = sum;
   }
   ```
   - Full matrix multiplication implementation
   - Shows multiple storage buffers and uniforms
   - 2D workgroup configuration (8x8)
   - Realistic GPU compute example

### 3. Pipeline Layout Configuration

**Auto-Generated Layout**
- Checkbox to enable/disable automatic layout generation
- When enabled (default), the pipeline layout is inferred from shader bindings
- Manual configuration planned for future enhancement

### 4. Validation and Feedback

**Validation Button**
- Validates the current configuration
- Checks:
  - Shader source is not empty
  - Shader has valid WGSL syntax
  - Entry point is specified and not empty
  - Entry point matches a compute function in the shader

**Create Pipeline Button** (Future Enhancement)
- Currently disabled as it requires GPU device access
- Will create the actual compute pipeline when enabled
- Would validate and compile the shader on the GPU

**Error Messages**
- Displayed in red with ❌ icon
- Specific error details:
  - "Shader compilation error: ..." for invalid WGSL
  - "Missing entry point: ..." when entry point is not set
  - "Entry point name cannot be empty" for whitespace-only entry points

**Success Messages**
- Displayed in green with ✓ icon
- Shows pipeline label or "unlabeled" if no label provided
- Example: "✓ Compute pipeline created successfully: 'my_pipeline'"

### 5. Information Section

The panel includes an informative section explaining:
- Components of a compute pipeline
- Required shader attributes
- Workgroup size specifications

## Usage Example

1. **Navigate to Compute Pipeline Tab**
   - Click "⚙️ Compute Pipeline" in the sidebar

2. **Configure Pipeline**
   - Enter a pipeline label (optional): "my_compute_pipeline"
   - Enter entry point: "main" (or your function name)

3. **Choose or Write Shader**
   - Option A: Click a template button (Simple, Storage Buffer, or Matrix Multiply)
   - Option B: Write custom WGSL code in the editor

4. **Validate Configuration**
   - Click "Validate Configuration"
   - Check for error messages or success confirmation

5. **Review Configuration**
   - Ensure all settings are correct
   - Auto-generated layout is used by default

## Technical Details

### Shader Caching
- The panel caches compiled shader modules
- Shader is only recompiled when source changes
- Improves performance for repeated validations

### Pipeline Descriptor
- Uses `ComputePipelineDescriptor` from the core library
- Builder pattern for configuration
- Validates before pipeline creation

### Integration Testing
- 10 integration tests validate all functionality
- Tests include:
  - Panel initialization
  - Custom configuration
  - Validation errors
  - Pipeline creation with GPU
  - All three shader templates
  - Shader caching mechanism
  - Error message handling

## Future Enhancements

Planned improvements include:
- Manual pipeline layout configuration UI
- Workgroup size configuration in UI
- Shader compilation constants
- Pipeline statistics and optimization hints
- Integration with bind group configuration
- Live shader compilation feedback
- Shader debugging support

## Location in Codebase

- **Panel Implementation**: `crates/wgpu_playground_core/src/compute_pipeline_panel.rs`
- **Integration**: `crates/wgpu_playground_gui/src/app.rs`
- **Tests**: `crates/wgpu_playground_core/tests/compute_pipeline_panel_integration_test.rs`

## See Also

- [Compute Pipeline API](../crates/wgpu_playground_core/src/compute.rs)
- [Shader Module API](../crates/wgpu_playground_core/src/shader.rs)
- [Pipeline Layout API](../crates/wgpu_playground_core/src/pipeline_layout.rs)
