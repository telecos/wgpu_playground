# wgpu_playground User Guide

Welcome to the wgpu_playground user guide! This comprehensive guide will help you get started with the application and master its features for experimenting with WebGPU in Rust.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Understanding the Interface](#understanding-the-interface)
3. [Core Features](#core-features)
4. [Working with Resources](#working-with-resources)
5. [Pipelines and Rendering](#pipelines-and-rendering)
6. [Compute Shaders](#compute-shaders)
7. [Troubleshooting](#troubleshooting)

## Getting Started

### Prerequisites

- **Rust** (latest stable version) - Install from [rustup.rs](https://rustup.rs/)
- **A GPU with WebGPU support**:
  - Vulkan (Windows, Linux, Android)
  - Metal (macOS, iOS)
  - DirectX 12 (Windows 10+)

### Installation

```bash
# Clone the repository
git clone https://github.com/telecos/wgpu_playground.git
cd wgpu_playground

# Build in release mode for better performance
cargo build --release

# Run the application
cargo run --release
```

### First Launch

On first launch, the application will:
1. Enumerate available GPU adapters
2. Select the most suitable adapter
3. Create a WebGPU device
4. Open the main GUI window

### Choosing Backend

Force a specific graphics API backend:

```bash
# Use Vulkan (recommended for Windows/Linux)
WGPU_BACKEND=vulkan cargo run --release

# Use Metal (macOS/iOS)
WGPU_BACKEND=metal cargo run --release

# Use DirectX 12 (Windows)
WGPU_BACKEND=dx12 cargo run --release
```

## Understanding the Interface

### Main Window Layout

- **Top Menu Bar**: Quick access to tabs and settings
- **Tab Bar**: Navigate between different feature areas
- **Main Panel**: The active tab's content
- **Console Panel** (bottom): GPU errors, warnings, and messages
- **Status Bar**: Active WebGPU implementation and backend

### Available Tabs

| Tab | Description |
|-----|-------------|
| **Adapter Selection** | GPU adapter enumeration and selection |
| **Device Config** | Configure device features and limits |
| **Device Info** | View GPU capabilities |
| **Buffer Config** | Create and configure GPU buffers |
| **Texture Config** | Create textures with various formats |
| **Sampler Config** | Configure texture samplers |
| **Bind Group Layout** | Define resource binding layouts |
| **Bind Group** | Bind resources to shaders |
| **Render Pipeline** | Configure render pipeline state |
| **Compute Pipeline** | Configure compute pipelines |
| **Render Pass** | Set up render pass configuration |
| **Compute & ML** | Interactive compute shader examples |
| **Rendering** | Rendering examples gallery |
| **Shader Editor** | Write and compile WGSL shaders |
| **Resource Inspector** | Inspect active GPU resources |
| **Performance** | View performance metrics |
| **API Coverage** | Track which WebGPU APIs you've used |
| **Tutorial** | Interactive learning guides |

## Core Features

### API Coverage Tracking

The **API Coverage** tab shows which WebGPU APIs you've exercised:
- Percentage coverage per category
- Links to official documentation
- Navigation to relevant panels

### Shader Editor

Write WGSL shaders with:
- Syntax highlighting
- Real-time validation
- Compile to check for errors
- Example shaders to start from

### Console

The console panel shows:
- GPU validation errors
- Shader compilation errors
- Performance warnings
- Debug messages

## Working with Resources

### Creating Buffers

1. Go to **Buffer Config** tab
2. Set buffer size (in bytes)
3. Select usage flags (VERTEX, INDEX, UNIFORM, STORAGE, etc.)
4. Click **Create Buffer**

### Creating Textures

1. Go to **Texture Config** tab
2. Set dimensions (width, height, depth)
3. Choose format (RGBA8, BGRA8, etc.)
4. Select usage flags
5. Click **Create Texture**

### Creating Samplers

1. Go to **Sampler Config** tab
2. Configure filtering (nearest, linear)
3. Set address modes (repeat, clamp, mirror)
4. Configure mipmap settings
5. Click **Create Sampler**

### Bind Groups

1. First create a **Bind Group Layout** defining slots
2. Then create a **Bind Group** binding resources to those slots
3. Reference the bind group in your pipeline

## Pipelines and Rendering

### Render Pipeline

1. Go to **Render Pipeline** tab
2. Select vertex and fragment shaders
3. Configure vertex buffer layout
4. Set primitive topology (triangles, lines, points)
5. Configure depth/stencil state
6. Set blend modes for color targets
7. Click **Create Pipeline**

### Render Pass

1. Go to **Render Pass** tab
2. Configure color attachments (clear color, load/store ops)
3. Set up depth/stencil attachment if needed
4. Execute rendering commands

### Example Gallery

The **Rendering** tab provides pre-built examples:
- **Triangle**: Basic triangle rendering
- **Rotating Cube**: 3D with depth testing
- **Texture Mapping**: Using textures and samplers
- **Instancing**: GPU instancing
- **Multisampling**: MSAA anti-aliasing

## Compute Shaders

### Compute & ML Panel

The **Compute & ML** tab provides interactive compute shader examples:

| Example | Description |
|---------|-------------|
| **Array Double** | Double each element in an array |
| **Vector Add** | Add two vectors element-wise |
| **Matrix Multiply** | Matrix multiplication (4x4) |
| **Grayscale** | Convert RGBA to grayscale |
| **Reduction** | Sum all elements (parallel reduction) |
| **Prefix Sum** | Compute running sum |

#### Running Compute Examples

1. Select an example from the dropdown
2. Edit input data values
3. Click **Run (CPU Simulation)** to test logic
4. Click **Run on GPU** to execute on the GPU
5. View output data and execution time

### Compute Pipeline

1. Go to **Compute Pipeline** tab
2. Select a compute shader module
3. Set entry point function name
4. Configure workgroup size
5. Click **Create Pipeline**

## Troubleshooting

### Common Issues

#### "No GPU adapters found"
- Ensure your GPU drivers are up to date
- Try forcing a different backend with `WGPU_BACKEND=vulkan`

#### Shader compilation errors
- Check the console panel for detailed error messages
- Validate WGSL syntax in the shader editor
- Ensure entry point names match

#### Black screen
- Check that all resources are properly bound
- Verify vertex buffer layout matches shader inputs
- Check render pass configuration

#### Poor performance
- Use release builds: `cargo run --release`
- Check the Performance panel for bottlenecks
- Reduce texture sizes or buffer counts

### Debug Logging

Enable detailed logging:

```bash
RUST_LOG=debug cargo run
RUST_LOG=wgpu=debug cargo run  # wgpu-specific logs
```

### Reporting Issues

When reporting issues, include:
1. GPU model and driver version
2. Operating system
3. Console output
4. Steps to reproduce
