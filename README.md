# wgpu_playground

Repository for experimenting WebGPU capabilities in Rust

## Overview

This is an interactive tool for experimenting with the wgpu crate's WebGPU API capabilities. It provides a graphical user interface built with egui that allows you to explore and test various WebGPU features including rendering pipelines, compute shaders, and ML inferencing operations.

## Features

- **GPU Adapter Selection**: Choose from available GPU adapters with detailed properties and configure power preferences
- **Device Information**: View detailed information about your GPU, including adapter info, device limits, and supported features
- **Device Configuration**: Configure device features and limits before device creation
- **Buffer Configuration**: Create and configure GPU buffers with custom parameters including size, usage flags, and mapping options
- **Rendering APIs**: Experiment with render pipelines, shaders, buffers, textures, and advanced rendering techniques
- **Compute/ML APIs**: Test compute pipelines, storage buffers, and machine learning operations

## User Interface

The application provides a tabbed interface with six main sections:

1. **Adapter Selection Tab**: Choose and configure GPU adapters:
   - View all available GPU adapters with detailed properties
   - Select adapter by name, vendor, device type, and backend
   - Configure power preference (None, Low Power, High Performance)
   - Filter adapters by backend (Vulkan, Metal, DX12, OpenGL, etc.)

2. **Device Config Tab**: Configure device settings:
   - Enable/disable WebGPU features (texture compression, shader features, etc.)
   - Adjust device limits to your needs
   - View adapter capabilities and constraints

3. **Device Info Tab**: Displays comprehensive information about your GPU adapter, including:
   - Adapter details (name, vendor, backend)
   - Device limits (texture dimensions, buffer sizes, workgroup limits, etc.)
   - Supported features

4. **Rendering Tab**: Provides tools for experimenting with rendering APIs:
   - **Example Gallery**: Browse and explore WebGPU shader examples with descriptions and source code
   - **WGSL Shader Editor**: Interactive shader editor with:
     - Syntax highlighting for WGSL keywords, types, and functions
     - Line numbers for easier code navigation
     - File loading from assets/shaders directory
     - Inline editing and validation
     - Real-time compilation with error reporting
     - Load example shaders or write your own
   - Render pipeline configuration (planned)
   - Buffer and vertex data management (planned)
   - Texture operations and sampling (planned)
   - Render pass configuration (planned)
   - Advanced rendering techniques (planned)

5. **Buffer Config Tab**: Create and configure GPU buffers:
   - Set buffer size with validation
   - Select usage flags via checkboxes (VERTEX, INDEX, UNIFORM, STORAGE, INDIRECT, COPY_SRC, COPY_DST, MAP_READ, MAP_WRITE, QUERY_RESOLVE)
   - Optional label for debugging
   - Mapped-at-creation option
   - Real-time validation with error messages
   - Configuration summary display

6. **Compute/ML Tab**: Tools for compute shader and ML operations (planned features):
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

### Examples

The project includes standalone examples demonstrating various WebGPU features:

#### Triangle Example

A classic "Hello Triangle" example showing basic rendering setup:

```bash
cargo run --package wgpu_playground_examples --example triangle
```

This example demonstrates:
- Creating a vertex buffer with position and color data
- Loading and compiling WGSL shaders
- Setting up a render pipeline
- Executing a render pass with draw commands

**Example output:**
```
=== Triangle Rendering Example ===
Using adapter: NVIDIA GeForce RTX 3080
Backend: Vulkan

Triangle vertices:
  Vertex 0: pos(0.0, 0.5), color(1.0, 0.0, 0.0)   # Red
  Vertex 1: pos(-0.5, -0.5), color(0.0, 1.0, 0.0) # Green
  Vertex 2: pos(0.5, -0.5), color(0.0, 0.0, 1.0)  # Blue

✓ Vertex buffer created (60 bytes)
✓ Shader loaded and compiled
✓ Render pipeline created
✓ Render target created (800x600)
✓ Render pass configured
✓ Render commands submitted to GPU
✓ Rendering complete
```

Other available examples:
- `backend_selection` - Enumerate and select GPU backends
- `compute_pass` - Compute shader operations
- `shader_loading` - Load and compile WGSL shaders
- `error_handling` - Error handling patterns

### WebGPU Implementation

The playground supports different WebGPU implementations:

- **wgpu** (default): Pure Rust implementation used by Firefox
  - Fast, safe, and cross-platform
  - Actively maintained by the gfx-rs team
  - Production-ready with full WebGPU support
  - https://github.com/gfx-rs/wgpu

- **Dawn** (build infrastructure complete): C++ implementation used by Chromium
  - Google's reference implementation
  - Built automatically from source using CMake
  - Cross-platform support (Windows D3D12, Linux Vulkan, macOS Metal)
  - Requires Git, CMake, C++ compiler, Python 3
  - **Status**: Build system functional, runtime integration in progress
  - https://dawn.googlesource.com/dawn

#### Selecting Implementation

By default, the application uses the **wgpu** implementation. You can select the implementation in two ways:

**1. Compile-time (feature flags):**
```bash
# Default: wgpu implementation
cargo build --release

# Enable Dawn (builds from source, requires CMake)
cargo build --release --features dawn
```

**First Dawn build requirements:**
- Git (clone repository)
- CMake 3.16+ (build configuration)
- C++ compiler with C++20 support
- Python 3 (dependency scripts)
- 10-30 minutes build time (one-time)

**Install build tools:**
```bash
# Ubuntu/Debian:
sudo apt-get install git cmake build-essential python3 libvulkan-dev

# macOS (Homebrew):
brew install git cmake python3

# Windows:
# Install Visual Studio with C++ support, CMake, Git, Python 3
```

**2. Runtime (environment variable):**
```bash
# Use wgpu implementation (default)
WEBGPU_IMPL=wgpu cargo run --release

# Use Dawn implementation (requires --features dawn)
WEBGPU_IMPL=dawn cargo run --release --features dawn
```

The active WebGPU implementation and its status is displayed in the **Device Info** and **Adapter Selection** tabs.

### Backend Selection

Within a WebGPU implementation, you can select which graphics API backend to use via the `WGPU_BACKEND` environment variable:

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

- **wgpu_playground_examples** (`crates/wgpu_playground_examples/`) - Example programs
  - `examples/triangle.rs` - Basic triangle rendering example
  - `examples/backend_selection.rs` - GPU backend enumeration and selection
  - `examples/compute_pass.rs` - Compute shader operations
  - `examples/shader_loading.rs` - WGSL shader loading and compilation
  - `examples/error_handling.rs` - Error handling patterns

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

## Continuous Integration

This project uses optimized CI testing to ensure code quality while maintaining fast build times:

- **Comprehensive Linux Tests**: Full test suite with unit, integration, and doc tests
- **Platform Compatibility Tests**: Quick library tests on macOS and Windows
- **Test Reporting**: JUnit XML reports with PR comments and check results
- **~70% faster CI**: Optimized from 9 parallel jobs to 3 strategic jobs

See [docs/CI_TESTING.md](docs/CI_TESTING.md) for detailed information on:
- Test job configuration and optimization strategy
- Running tests locally
- Test reporting and failure notifications
- Performance characteristics and troubleshooting

## Documentation

- **[SHADER_EDITOR.md](docs/SHADER_EDITOR.md)** - Complete guide to the WGSL Shader Editor including usage examples, tips, and API reference
- **[WEBGPU_IMPLEMENTATIONS.md](docs/WEBGPU_IMPLEMENTATIONS.md)** - Guide to WebGPU implementations (wgpu vs Dawn), architecture, and how to switch between them
- **[GUI_FRAMEWORK_EVALUATION.md](GUI_FRAMEWORK_EVALUATION.md)** - Detailed evaluation and rationale for selecting egui as the GUI framework, including comparison with iced and imgui-wgpu
- **[PLAN.md](PLAN.md)** - Complete project roadmap with implementation phases
- **[UI_MOCKUP.md](UI_MOCKUP.md)** - UI design and layout documentation
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Development guidelines and contribution instructions

## License

MIT
