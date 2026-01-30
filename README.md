# wgpu_playground

Repository for experimenting WebGPU capabilities in Rust

## Overview

This is an interactive tool for experimenting with the wgpu crate's WebGPU API capabilities. It provides a graphical user interface built with egui that allows you to explore and test various WebGPU features including rendering pipelines, compute shaders, and ML inferencing operations.

## Features

- **Device Information**: View detailed information about your GPU, including adapter info, device limits, and supported features
- **Rendering APIs**: Experiment with render pipelines, shaders, buffers, textures, and advanced rendering techniques
- **Compute/ML APIs**: Test compute pipelines, storage buffers, and machine learning operations

## User Interface

The application provides a tabbed interface with three main sections:

1. **Device Info Tab**: Displays comprehensive information about your GPU adapter, including:
   - Adapter details (name, vendor, backend)
   - Device limits (texture dimensions, buffer sizes, workgroup limits, etc.)
   - Supported features

2. **Rendering Tab**: Provides tools for experimenting with rendering APIs (planned features):
   - Render pipeline configuration
   - Shader editing and testing
   - Buffer and vertex data management
   - Texture operations and sampling
   - Render pass configuration
   - Advanced rendering techniques (instancing, MSAA, etc.)

3. **Compute/ML Tab**: Tools for compute shader and ML operations (planned features):
   - Compute pipeline setup
   - Storage buffer management
   - Matrix operations
   - Convolution and pooling operations
   - Neural network layer implementations
   - Performance profiling

## Building and Running

### Prerequisites

- Rust (latest stable version)
- A GPU with WebGPU support (Vulkan, Metal, or DirectX 12)

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run --release
```

**Note:** This application requires a display/window system to run. On Linux, ensure you have either X11 or Wayland available. On headless systems, the application won't run as it requires GPU rendering capabilities.

## Project Structure

This project uses a Cargo workspace structure with the following crates:

- **wgpu_playground_core** (`crates/wgpu_playground_core/`) - Core WebGPU functionality
  - `src/device_info.rs` - GPU device information display
  - `src/rendering.rs` - Rendering APIs experimentation panel
  - `src/compute.rs` - Compute and ML APIs experimentation panel
  - `src/assets.rs` - Asset loading infrastructure for shaders, textures, and models

- **wgpu_playground_gui** (`crates/wgpu_playground_gui/`) - GUI application
  - `src/main.rs` - Main application entry point and window management
  - `src/app.rs` - Main UI application structure and tab management

- **wgpu_playground_examples** (`crates/wgpu_playground_examples/`) - Example programs (planned)

- **assets/** - Static assets directory
  - `shaders/` - WGSL shader files
  - `textures/` - Texture assets (PNG, JPG, etc.)
  - `models/` - 3D model files (OBJ, GLTF, etc.)

## Development Status

This is currently a skeleton/framework for the full application. See [PLAN.md](PLAN.md) for planned features and implementation roadmap.

## License

MIT
