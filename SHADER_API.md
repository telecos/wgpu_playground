# WGSL Shader Module API

This document describes the shader module compilation API implemented in `wgpu_playground_core::shader`.

## Overview

The shader module provides a safe and convenient way to load and compile WGSL (WebGPU Shading Language) shaders. It supports:

- Loading shaders from files
- Loading shaders from inline strings
- Proper error handling with descriptive error messages
- Integration with wgpu's shader compilation

## Basic Usage

### Loading from Inline Source

```rust
use wgpu_playground_core::shader::ShaderModule;

let shader = ShaderModule::from_source(
    r#"
    @vertex
    fn vs_main(@builtin(vertex_index) index: u32) -> @builtin(position) vec4<f32> {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }
    "#,
    Some("my_shader")
).unwrap();
```

### Loading from File

```rust
use wgpu_playground_core::shader::ShaderModule;

// Loads from assets/shaders/example.wgsl
let shader = ShaderModule::from_file("example.wgsl", Some("example")).unwrap();
```

### Creating wgpu Shader Modules

```rust
// After creating a ShaderModule, convert it to a wgpu::ShaderModule
let wgpu_module = shader.create_module(&device);

// Use in a pipeline
let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
    // ... other fields ...
    vertex: wgpu::VertexState {
        module: &wgpu_module,
        entry_point: "vs_main",
        // ...
    },
    // ...
});
```

## Error Handling

The API provides two error types:

- `ShaderError::LoadError` - Failed to load shader file (file not found, permission denied, etc.)
- `ShaderError::InvalidSource` - Invalid shader source (e.g., empty string)

Note: Shader compilation errors are handled by wgpu during `create_module()`. In debug mode, wgpu will panic with detailed error messages. In release mode, it will log errors.

```rust
match ShaderModule::from_file("shader.wgsl", None) {
    Ok(shader) => println!("Loaded successfully!"),
    Err(ShaderError::LoadError(e)) => println!("Failed to load: {}", e),
    Err(ShaderError::InvalidSource(msg)) => println!("Invalid source: {}", msg),
}
```

## Advanced Usage

### Using ShaderSource Enum

For more flexibility, use the `ShaderSource` enum:

```rust
use wgpu_playground_core::shader::{ShaderModule, ShaderSource};

let source = if use_file {
    ShaderSource::File("shader.wgsl".to_string())
} else {
    ShaderSource::Inline(shader_code.to_string())
};

let shader = ShaderModule::new(source, Some("dynamic_shader")).unwrap();
```

## Examples

Run the shader loading example:

```bash
cargo run --package wgpu_playground_examples --example shader_loading
```

## API Reference

### `ShaderModule`

Main type for managing WGSL shaders.

**Methods:**
- `new(source: ShaderSource, label: Option<&str>) -> Result<Self, ShaderError>`
- `from_source(source: &str, label: Option<&str>) -> Result<Self, ShaderError>`
- `from_file(filename: &str, label: Option<&str>) -> Result<Self, ShaderError>`
- `source(&self) -> &str` - Get shader source code
- `label(&self) -> Option<&str>` - Get shader label
- `create_module(&self, device: &wgpu::Device) -> wgpu::ShaderModule` - Create wgpu module

### `ShaderSource`

Enum for specifying shader source:
- `File(String)` - Load from file in `assets/shaders/`
- `Inline(String)` - Use inline WGSL code

### `ShaderError`

Error type for shader operations:
- `LoadError(std::io::Error)` - File loading failed
- `InvalidSource(String)` - Invalid shader source

## Security Considerations

- File loading validates filenames to prevent path traversal attacks
- Only files in the `assets/shaders/` directory can be loaded
- Filenames cannot contain `..`, `/`, or `\`
