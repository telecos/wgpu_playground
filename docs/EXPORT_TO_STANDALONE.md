# Export to Standalone Project

The wgpu_playground allows you to export your current configuration as a complete, runnable standalone Rust project. This feature is useful for:

- Creating a starting point for a new wgpu project
- Sharing your playground configuration with others
- Converting playground experiments into production code
- Learning wgpu API usage by example

## How to Export

There are two ways to export your project:

### 1. From the Menu Bar

Click the **📦 Export Project** button in the top menu bar. This will export the full playground state including:

- All configured buffers
- All configured textures
- All configured samplers  
- Shader code from the shader editor
- Render pipeline configurations
- Compute pipeline configurations
- Bind group configurations

The project will be exported to your home directory with the name you configured.

### 2. From the Rendering Panel (Example-based)

In the Rendering/Examples section, you can export individual examples by:

1. Select an example from the list
2. Enter a project name in the text field
3. Click **📦 Export Project**

This exports a basic example-based project with the shader code from that example.

## What Gets Exported

The export creates a complete Rust project with the following structure:

```
project_name/
├── Cargo.toml          # Project manifest with dependencies
├── src/
│   └── main.rs         # Complete application code
└── README.md           # Documentation and build instructions
```

### Cargo.toml

The generated `Cargo.toml` includes all necessary dependencies:

- `wgpu = "27.0"` - WebGPU implementation
- `winit = "0.30"` - Window and event handling
- `pollster = "0.4"` - Async runtime for simple applications
- `env_logger = "0.11"` - Logging support
- `log = "0.4"` - Logging facade
- `bytemuck = { version = "1.19", features = ["derive"] }` - Pod type casting

### main.rs

The generated `main.rs` includes:

**Imports**: All necessary imports from winit and wgpu

**Shader Code**: Your custom shader code embedded as a constant

**State Struct**: A complete State struct containing:
- Surface, device, queue, and configuration
- All configured buffers
- All configured textures and texture views
- All configured samplers
- Shader modules
- Render and/or compute pipelines

**State Implementation**: Full implementation including:
- `new()` - Async initialization function that sets up:
  - wgpu instance and surface
  - Adapter selection
  - Device and queue creation
  - Surface configuration
  - All configured resources (buffers, textures, samplers, shaders, pipelines)
- `resize()` - Window resize handling
- `render()` - Render loop with proper error handling

**Main Function**: Complete event loop with:
- Environment logger initialization
- Window creation using winit 0.30 API
- Event handling (close, resize, redraw)
- Proper error handling for surface errors

## Building and Running the Exported Project

Once exported, navigate to the project directory and:

```bash
cd ~/project_name   # Or wherever it was exported

# Build the project
cargo build --release

# Run the project
cargo run --release
```

## Requirements

To build and run the exported project, you need:

- Rust (latest stable version)
- A GPU with WebGPU support:
  - Vulkan on Linux/Windows
  - Metal on macOS
  - DirectX 12 on Windows

## Customizing the Exported Project

The exported project is a complete, independent Rust application. You can:

1. **Add more resources**: Create additional buffers, textures, etc.
2. **Extend the render loop**: Add more drawing commands
3. **Add UI**: Integrate with egui or other UI frameworks
4. **Add compute shaders**: Implement compute pipelines
5. **Handle input**: Add keyboard/mouse handling in the event loop
6. **Load models**: Add model loading with tobj or gltf
7. **Add textures**: Load images with image crate

## Example Export

Here's what a simple exported project might look like:

### Configuration
- Shader: Custom WGSL vertex + fragment shader
- Buffer: Vertex buffer (1024 bytes, VERTEX | COPY_DST usage)
- Texture: Render target (256x256, RGBA8, TEXTURE_BINDING usage)
- Pipeline: Render pipeline with vs_main and fs_main entry points

### Generated Code Structure

The generated code will have:
- Proper buffer creation with specified size and usage flags
- Texture creation with specified dimensions and format
- Shader module creation from the WGSL source
- Render pipeline with configured vertex and fragment shaders
- Complete render loop that clears and presents

## Limitations

Current limitations of the export feature:

1. **Vertex Layouts**: Basic vertex buffer layouts are included, but may need customization for complex vertex structures
2. **Bind Groups**: Basic bind group support is included but may need manual configuration
3. **Advanced Features**: Some advanced wgpu features may require manual implementation
4. **Assets**: Texture/model assets are not embedded; paths would need to be configured manually
5. **Event Handling**: Basic window events are handled, but you may want to add keyboard/mouse input

## Future Enhancements

Planned improvements:

- [ ] More complete bind group generation
- [ ] Vertex layout generation from buffer configurations
- [ ] Asset embedding options
- [ ] Multiple render pass support
- [ ] Compute dispatch generation
- [ ] More customization options in the export dialog

## Troubleshooting

### Compilation Errors

If the exported project doesn't compile:

1. **Check Rust version**: Ensure you have the latest stable Rust
2. **Update dependencies**: Run `cargo update` to get latest compatible versions
3. **Check GPU support**: Ensure your GPU supports Vulkan/Metal/DirectX 12
4. **Review shader code**: WGSL syntax errors may need to be fixed

### Runtime Errors

If the project compiles but doesn't run:

1. **Check error messages**: Look for adapter/device creation failures
2. **Check GPU drivers**: Update to latest graphics drivers
3. **Try different backends**: wgpu will try different backends automatically
4. **Check shader compatibility**: Some shaders may not work on all backends

## Getting Help

If you encounter issues:

1. Check the generated README.md for specific build instructions
2. Review the [wgpu documentation](https://docs.rs/wgpu/)
3. Check the [winit documentation](https://docs.rs/winit/)
4. Open an issue on the wgpu_playground repository

## License

Exported projects are independent code generated from your configuration. You own the exported code and can license it however you choose.
