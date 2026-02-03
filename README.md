# wgpu_playground

Repository for experimenting WebGPU capabilities in Rust

## Quick Start

New to wgpu_playground? Get started quickly:

📖 **[Quick Start Guide](docs/QUICK_START.md)** - Get up and running in 5 minutes

📚 **[User Guide](docs/USER_GUIDE.md)** - Comprehensive guide with tutorials and workflows

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

#### Texture Mapping Example

A comprehensive example demonstrating texture creation, sampling, and binding:

```bash
cargo run --package wgpu_playground_examples --bin texture_mapping
```

This example demonstrates:
- Creating a texture programmatically with data (8x8 checkerboard pattern)
- Configuring a sampler for texture filtering
- Setting up vertex data with UV coordinates
- Loading and compiling shaders with texture sampling
- Binding textures and samplers to shaders via bind groups
- Rendering a textured quad to an offscreen texture

**Example output:**
```
=== Texture Mapping Example ===

1. Initializing wgpu context...
   ✓ Adapter: NVIDIA GeForce RTX 3080
   ✓ Device and queue created

2. Creating checkerboard texture...
   ✓ Texture created: 8x8 pixels
   ✓ Format: Rgba8Unorm
   ✓ Data uploaded: 256 bytes

3. Creating texture view...
   ✓ Texture view created

4. Creating texture sampler...
   ✓ Sampler created
   - Address mode: Repeat
   - Filter mode: Nearest

5. Loading textured quad shader...
   ✓ Shader loaded and compiled

6. Creating vertex buffer for textured quad...
   ✓ Vertex buffer created
   - Vertices: 6
   - Buffer size: 96 bytes

7. Creating bind group layout...
   ✓ Bind group layout created

8. Creating bind group...
   ✓ Bind group created
   - Binding 0: Sampler
   - Binding 1: Texture

9. Creating render pipeline...
   ✓ Render pipeline created

10. Creating output texture for rendering...
    ✓ Output texture created: 256x256 pixels

11. Recording and submitting render commands...
    ✓ Render commands submitted

12. Waiting for GPU to complete...
    ✓ Rendering complete!

=== Example Summary ===
This example demonstrated:
  ✓ Creating a texture programmatically (8x8 checkerboard)
  ✓ Configuring a sampler (Repeat + Nearest filtering)
  ✓ Setting up vertex data with UV coordinates
  ✓ Loading and compiling a shader with texture sampling
  ✓ Binding textures and samplers to shaders via bind groups
  ✓ Rendering a textured quad to an offscreen texture

=== Example Complete ===
#### Rotating Cube Example

A 3D rotating cube demonstrating advanced rendering features:

```bash
cargo run --package wgpu_playground_examples --example rotating_cube
```

This example demonstrates:
- 3D cube geometry with 8 vertices and index buffer (36 indices for 12 triangles)
- Uniform buffers for transformation matrices (model-view-projection)
- Depth testing with Depth24Plus format
- Back-face culling
- Animation with rotation over multiple frames
- Perspective projection and camera positioning

**Example output:**
```
=== Rotating 3D Cube Example ===
Using adapter: NVIDIA GeForce RTX 3080
Backend: Vulkan

Cube geometry:
  8 vertices (8 corners)
  36 indices (12 triangles, 6 faces)

✓ Vertex buffer created (192 bytes)
✓ Index buffer created (72 bytes)
✓ Shader loaded and compiled
✓ Uniform buffer created (64 bytes)
✓ Bind group created

✓ Render pipeline created with depth testing

✓ Render target created (800x600)
✓ Depth buffer created

Rendering animation frames:
  Frame 0: rotation = 0.00 radians
  Frame 1: rotation = 1.26 radians
  Frame 2: rotation = 2.51 radians
  Frame 3: rotation = 3.77 radians
  Frame 4: rotation = 5.03 radians

✓ All frames rendered successfully

=== Rotating Cube Example Complete ===

The 3D cube was successfully rendered with:
  • 8 vertices defining cube corners
  • 36 indices defining 12 triangles (6 faces)
  • Uniform buffer with model-view-projection matrix
  • Depth testing enabled (Depth24Plus format)
  • Back-face culling enabled
  • Rotation animation over 5 frames
```

#### Render-to-Texture Example

A multi-pass rendering example demonstrating framebuffer usage:

```bash
cargo run --package wgpu_playground_examples --example render_to_texture
```

This example demonstrates:
- Creating an offscreen texture as a framebuffer (RENDER_ATTACHMENT usage)
- First render pass: Rendering a colorful triangle to the offscreen texture
- Second render pass: Using the rendered texture as input to display on a fullscreen quad
- Multi-pass rendering workflow
- Texture sampling with bind groups
- Proper texture usage flags for both rendering and sampling

**Example output:**
```
=== Render-to-Texture Example ===

This example demonstrates multi-pass rendering:
  Pass 1: Render triangle to offscreen texture
  Pass 2: Display texture on fullscreen quad

Using adapter: NVIDIA GeForce RTX 3080
Backend: Vulkan
✓ GPU device created

=== First Pass Setup (Render Triangle to Texture) ===
✓ Triangle vertex buffer created (60 bytes)
✓ Scene shader loaded and compiled
✓ Scene render pipeline created
✓ Offscreen texture created (512x512)
  - Usage: RENDER_ATTACHMENT | TEXTURE_BINDING

=== Second Pass Setup (Display Texture on Quad) ===
✓ Quad vertex buffer created (96 bytes)
✓ Texture sampler created
✓ Display shader loaded and compiled
✓ Bind group created (texture + sampler)
✓ Display render pipeline created

✓ Final output texture created (800x600)

=== Executing Multi-Pass Rendering ===

Pass 1: Rendering triangle to offscreen texture...
  ✓ Triangle rendered to offscreen texture
Pass 2: Displaying offscreen texture on quad...
  ✓ Offscreen texture sampled and displayed on quad

✓ Render commands submitted to GPU
✓ Rendering complete

=== Render-to-Texture Example Complete ===

This example successfully demonstrated:
  • Creating an offscreen texture as a framebuffer
  • First pass: Rendering a triangle to the offscreen texture
  • Second pass: Using the texture as input to render a textured quad
  • Multi-pass rendering workflow
  • Texture sampling with bind groups

Key concepts:
  - Offscreen texture usage: RENDER_ATTACHMENT | TEXTURE_BINDING
  - First pass StoreOp::Store preserves rendered content
  - Second pass samples the texture via bind group
  - Two separate render pipelines for different passes
```

#### Compute-Render Buffer Sharing Example

A comprehensive example demonstrating buffer sharing between compute and render pipelines:

```bash
cargo run --package wgpu_playground_examples --example compute_render_sharing
```

This example demonstrates:
- Creating a compute shader that processes particle data (position, velocity updates)
- Creating a render pipeline that visualizes the computed particles as colored points
- Sharing a buffer between compute and render pipelines using STORAGE + VERTEX usage flags
- Multiple frames showing the compute shader updating particle positions
- Simple particle physics with boundary collision and circular motion

**Example output:**
```
=== Compute-Render Buffer Sharing Example ===

Using adapter: Your GPU Name
Backend: Vulkan

Created 1024 particles arranged in a circle
  - Each particle has position, velocity, and color
  - Particles will be updated by compute shader
  - Same buffer will be used for vertex data in render pass

✓ Created shared buffer with STORAGE + VERTEX usage
  - Size: 32768 bytes (1024 particles)
  - STORAGE flag: allows compute shader access
  - VERTEX flag: allows render pipeline access

Setting up compute pipeline...
✓ Compute pipeline created
  - Updates particle positions based on velocity
  - Applies boundary collision detection
  - Adds circular motion effect

Setting up render pipeline...
✓ Render pipeline created
  - Draws particles as points (PointList topology)
  - Uses particle position and color from buffer
  - Alpha blending enabled for nice visual effect

Executing compute and render operations...

Frame 0:
  ✓ Compute pass: Updated particle positions
  ✓ Render pass: Drew 1024 particles as points
  ✓ Commands submitted and completed

Frame 1:
  ✓ Compute pass: Updated particle positions
  ✓ Render pass: Drew 1024 particles as points
  ✓ Commands submitted and completed

Frame 2:
  ✓ Compute pass: Updated particle positions
  ✓ Render pass: Drew 1024 particles as points
  ✓ Commands submitted and completed

=== Example Complete ===

This example demonstrated:
  ✓ Creating a shared buffer with STORAGE + VERTEX usage flags
  ✓ Compute shader that processes particle data
  ✓ Render pipeline that visualizes the same data
  ✓ Buffer sharing between compute and render pipelines
  ✓ Multiple frames showing data updates from compute shader

Key Concepts:
  • STORAGE usage: Enables read/write access in compute shaders
  • VERTEX usage: Enables use as vertex buffer in render pipeline
  • Combined usage allows seamless data flow from compute to render
  • Same buffer used without copying, maximizing GPU efficiency
```

Other available examples:
- `adapter_demo` - Enumerate and select GPU adapters
- `texture_mapping` - Texture creation, sampling, and binding
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

- **Dawn** (native + fallback): C++ implementation used by Chromium
  - Google's reference implementation
  - Attempts to build actual Dawn C++ library from source
  - Falls back to wgpu-core when Dawn build unavailable
  - Cross-platform support (Windows D3D12, Linux Vulkan, macOS Metal)
  - **Status**: Native FFI when built, wgpu-core fallback otherwise
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
- Git (to clone Dawn repository)
- CMake 3.16+ (to build Dawn)
- C++ compiler with C++20 support
- Python 3 (for Dawn's dependency scripts)
- Build time: 10-30 minutes (first build only)
- **Note**: If build fails, automatically falls back to wgpu-core

**Install build tools:**
If you want to use actual Dawn C++ library (optional), install:

```bash
# Ubuntu/Debian:
sudo apt-get install git cmake build-essential python3 libvulkan-dev

# macOS (Homebrew):
brew install git cmake python3

# Windows:
# Install Visual Studio with C++ support, CMake, Git, Python 3
```

**Note**: If build tools are not available, the `dawn` feature will automatically use wgpu-core as a compatible fallback. See [docs/BUILDING_DAWN.md](docs/BUILDING_DAWN.md) for details.

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

## Testing

### Running Tests

The project includes comprehensive unit and integration tests that run in both local and CI environments:

```bash
# Run all tests
cargo test --workspace

# Run tests for a specific package
cargo test --package wgpu_playground_core

# Run a specific test
cargo test --package wgpu_playground_core --test buffer_integration_test
```

### Headless GPU Testing

Tests automatically detect headless/CI environments (via `CI` or `WGPU_HEADLESS` environment variables) and use software rendering adapters to enable GPU testing without physical hardware:

- **Local Testing**: Tests use available hardware GPU adapters
- **CI/Headless**: Tests automatically switch to software rendering (Vulkan lavapipe or OpenGL)
- **Fallback Adapter**: In headless mode, `force_fallback_adapter` is enabled for software rendering

**Manually enable headless mode:**

```bash
# Set WGPU_HEADLESS to force software rendering
WGPU_HEADLESS=1 cargo test --workspace

# Or set CI variable
CI=1 cargo test --workspace
```

### Visual Regression Testing

This project includes a visual regression testing framework to catch unintended visual changes in GPU rendering output. The framework:

- Captures rendered GPU textures to PNG images
- Compares captured output with reference images
- Generates diff images highlighting discrepancies
- Supports configurable comparison thresholds

**Running Visual Regression Tests:**

```bash
# Run all visual regression tests
cargo test --package wgpu_playground_core visual_regression

# Generate or update reference images (do this on a system with GPU)
UPDATE_VISUAL_REFERENCES=1 cargo test --package wgpu_playground_core visual_regression
```

**Note:** Visual regression tests work in both local and headless/CI environments thanks to automatic software rendering adapter selection.

For detailed information about the visual regression framework, see [tests/visual_regression/reference/README.md](tests/visual_regression/reference/README.md).

## Performance Benchmarking

This project includes performance benchmarks to track and monitor the performance of critical code paths:

- **Criterion.rs Integration**: High-quality statistical benchmarking with HTML reports
- **Automated CI Runs**: Weekly scheduled benchmarks and on-demand execution
- **Baseline Comparison**: Automatic comparison against main branch to detect regressions
- **Historical Tracking**: Long-term storage of benchmark results for trend analysis

**Running Benchmarks:**

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark suite
cargo bench --bench buffer_operations
cargo bench --bench shader_compilation

# View detailed HTML reports
open target/criterion/report/index.html
```

**CI Integration:**
- Benchmarks run automatically every Monday at 00:00 UTC
- Manual triggers available via GitHub Actions
- Results stored as artifacts for 90 days
- PR comments show performance comparison with main branch

For detailed information about the benchmarking system, see [docs/BENCHMARKING.md](docs/BENCHMARKING.md) and [crates/wgpu_playground_core/benches/README.md](crates/wgpu_playground_core/benches/README.md).

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

### Pull Request Workflow

All pull requests are automatically validated through comprehensive PR checks:

- **Format Check**: Ensures code is properly formatted with `rustfmt`
- **Lint Check**: Code quality validation with Clippy
- **Build Check**: Ensures project builds successfully
- **Test Check**: All tests must pass
- **Security Check**: Dependency security and license compliance
- **Automated Labeling**: PRs are labeled by size and type of changes

See [docs/BRANCH_PROTECTION.md](docs/BRANCH_PROTECTION.md) for:
- Complete PR workflow documentation
- Branch protection configuration
- Status check requirements
- Troubleshooting guide

## Documentation

### User Documentation

- **[Quick Start Guide](docs/QUICK_START.md)** - Get up and running in 5 minutes
- **[User Guide](docs/USER_GUIDE.md)** - Comprehensive guide covering:
  - Installation and setup
  - GUI navigation and usage
  - Creating buffers, textures, and samplers
  - Writing and compiling shaders
  - Step-by-step tutorials for rendering triangles, textured quads, and compute shaders
  - Common workflows and troubleshooting

### API Documentation

API documentation is automatically generated from source code and published to GitHub Pages:

- **Online**: Visit the [GitHub Pages documentation](https://telecos.github.io/wgpu_playground/) for the latest API reference
- **Local**: Generate and view documentation locally:
  ```bash
  cargo doc --workspace --all-features --no-deps --open
  ```

The documentation includes all public APIs for:
- `wgpu_playground_core` - Core WebGPU functionality and rendering primitives
- `wgpu_playground_gui` - GUI application and user interface components
- `wgpu_playground_examples` - Example programs and usage demonstrations

### Design Documents

- **[USER_GUIDE.md](docs/USER_GUIDE.md)** - Comprehensive end-user documentation with tutorials and workflows
- **[QUICK_START.md](docs/QUICK_START.md)** - Quick start guide for new users
- **[VISUAL_REGRESSION_TESTING.md](docs/VISUAL_REGRESSION_TESTING.md)** - Complete guide to the visual regression testing framework, including examples, API reference, and best practices
- **[SHADER_EDITOR.md](docs/SHADER_EDITOR.md)** - Complete guide to the WGSL Shader Editor including usage examples, tips, and API reference
- **[WEBGPU_IMPLEMENTATIONS.md](docs/WEBGPU_IMPLEMENTATIONS.md)** - Guide to WebGPU implementations (wgpu vs Dawn), architecture, and how to switch between them
- **[GUI_FRAMEWORK_EVALUATION.md](GUI_FRAMEWORK_EVALUATION.md)** - Detailed evaluation and rationale for selecting egui as the GUI framework, including comparison with iced and imgui-wgpu
- **[PLAN.md](PLAN.md)** - Complete project roadmap with implementation phases
- **[UI_MOCKUP.md](UI_MOCKUP.md)** - UI design and layout documentation
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Development guidelines and contribution instructions

## License

MIT
