# wgpu_playground

Repository for experimenting WebGPU capabilities in Rust

## Overview

This is an interactive tool for experimenting with the wgpu crate's WebGPU API capabilities. It provides a graphical user interface built with egui that allows you to explore and test various WebGPU features including rendering pipelines, compute shaders, and ML inferencing operations.

## Features

- **Device Information**: View detailed information about your GPU, including adapter info, device limits, and supported features
- **Rendering APIs**: Experiment with render pipelines, shaders, buffers, textures, and advanced rendering techniques
- **Compute/ML APIs**: Test compute pipelines, storage buffers, and machine learning operations

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

## Project Structure

- `src/main.rs` - Main application entry point and window management
- `src/app.rs` - Main UI application structure and tab management
- `src/device_info.rs` - GPU device information display
- `src/rendering.rs` - Rendering APIs experimentation panel
- `src/compute.rs` - Compute and ML APIs experimentation panel

## Development Status

This is currently a skeleton/framework for the full application. See [PLAN.md](PLAN.md) for planned features and implementation roadmap.

## License

MIT
