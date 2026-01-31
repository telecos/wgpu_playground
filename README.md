# wgpu_playground

Repository for experimenting WebGPU capabilities in Rust

## Overview

This is an interactive tool for experimenting with the wgpu crate's WebGPU API capabilities. It provides a graphical user interface built with egui that allows you to explore and test various WebGPU features including rendering pipelines, compute shaders, and ML inferencing operations.

## Features

- **GPU Adapter Selection**: Choose from available GPU adapters with detailed properties and configure power preferences
- **Device Information**: View detailed information about your GPU, including adapter info, device limits, and supported features
- **Rendering APIs**: Experiment with render pipelines, shaders, buffers, textures, and advanced rendering techniques
- **Compute/ML APIs**: Test compute pipelines, storage buffers, and machine learning operations

## User Interface

The application provides a tabbed interface with four main sections:

1. **Adapter Selection Tab**: Choose and configure GPU adapters:
   - View all available GPU adapters with detailed properties
   - Select adapter by name, vendor, device type, and backend
   - Configure power preference (None, Low Power, High Performance)
   - Filter adapters by backend (Vulkan, Metal, DX12, OpenGL, etc.)

2. **Device Info Tab**: Displays comprehensive information about your GPU adapter, including:
   - Adapter details (name, vendor, backend)
   - Device limits (texture dimensions, buffer sizes, workgroup limits, etc.)
   - Supported features

3. **Rendering Tab**: Provides tools for experimenting with rendering APIs (planned features):
   - Render pipeline configuration
   - Shader editing and testing
   - Buffer and vertex data management
   - Texture operations and sampling
   - Render pass configuration
   - Advanced rendering techniques (instancing, MSAA, etc.)

4. **Compute/ML Tab**: Tools for compute shader and ML operations (planned features):
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

### Backend Selection

The application supports multiple WebGPU backend implementations (similar to Chromium's Dawn). You can select which backend to use via the `WGPU_BACKEND` environment variable:

```bash
# Use Vulkan backend
WGPU_BACKEND=vulkan cargo run --release

# Use Metal backend (macOS/iOS)
WGPU_BACKEND=metal cargo run --release

# Use DirectX 12 backend (Windows)
WGPU_BACKEND=dx12 cargo run --release

# Use OpenGL backend
WGPU_BACKEND=gl cargo run --release

# Use primary backends (default for platform)
WGPU_BACKEND=primary cargo run --release

# Use all available backends (default if not specified)
WGPU_BACKEND=all cargo run --release
```

**Available Backend Options:**
- `vulkan` or `vk` - Vulkan API (Windows, Linux, Android, macOS via MoltenVK)
- `metal` or `mtl` - Metal API (macOS, iOS)
- `dx12`, `d3d12`, or `directx12` - DirectX 12 API (Windows 10+)
- `gl` or `opengl` - OpenGL/OpenGL ES API
- `primary` - Platform's primary backends (Vulkan, Metal, DX12, Browser WebGPU)
- `all` - All available backends (default)

The active backend is displayed prominently in the **Device Info** tab of the application.

## Troubleshooting

### DirectX 12 Resource State Errors on AMD GPUs (Windows)

If you see console error messages like:

```
[ERROR wgpu_hal::auxil::dxgi::exception] ID3D12CommandQueue1::ExecuteCommandLists: 
Resource state (0x4: D3D12_RESOURCE_STATE_RENDER_TARGET) of resource is invalid for use as a PRESENT_SOURCE.
Expected State Bits (all): 0x0: D3D12_RESOURCE_STATE_[COMMON|PRESENT]
INVALID_SUBRESOURCE_STATE
```

This is a known issue with the wgpu DirectX 12 backend on AMD GPUs (and occasionally other GPUs). These errors are validation warnings from the DirectX 12 debug layer (typically seen when running debug builds or with GPU validation enabled) and do not affect visual rendering in most cases. They occur due to missing or incorrect resource state transitions in the wgpu-hal DirectX 12 backend.

**Note:** These errors may not appear in release builds without debug layers, but are common during development.

**Solution: Use the Vulkan Backend**

The most effective workaround is to use the Vulkan backend instead of DirectX 12:

```bash
# Windows Command Prompt
set WGPU_BACKEND=vulkan
cargo run --release

# Windows PowerShell
$env:WGPU_BACKEND="vulkan"
cargo run --release

# Or run directly
WGPU_BACKEND=vulkan cargo run --release
```

The application will automatically prefer the Vulkan backend on Windows when no explicit backend is specified, but you can force DirectX 12 with `WGPU_BACKEND=dx12` if needed.

**Note:** This issue is being tracked upstream in the wgpu project. The errors are cosmetic and typically don't cause rendering issues, but they can spam the console log. Using Vulkan avoids the issue entirely.

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

## Code Coverage

This project uses `cargo-llvm-cov` for code coverage reporting. See [COVERAGE.md](COVERAGE.md) for details on:

- Running coverage locally
- Coverage thresholds and targets
- CI integration
- Viewing coverage reports

Current coverage: 62% (see CI artifacts for detailed reports)

## Documentation

- **[GUI_FRAMEWORK_EVALUATION.md](GUI_FRAMEWORK_EVALUATION.md)** - Detailed evaluation and rationale for selecting egui as the GUI framework, including comparison with iced and imgui-wgpu
- **[PLAN.md](PLAN.md)** - Complete project roadmap with implementation phases
- **[UI_MOCKUP.md](UI_MOCKUP.md)** - UI design and layout documentation
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Development guidelines and contribution instructions

## License

MIT
