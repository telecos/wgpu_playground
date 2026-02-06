# wgpu_playground

[![CI](https://github.com/telecos/wgpu_playground/workflows/CI/badge.svg)](https://github.com/telecos/wgpu_playground/actions/workflows/ci.yml)
[![Coverage](https://codecov.io/gh/telecos/wgpu_playground/branch/main/graph/badge.svg)](https://codecov.io/gh/telecos/wgpu_playground)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Repository for experimenting WebGPU capabilities in Rust

## Table of Contents

- [🚀 Try the Demo](#-try-the-demo)
- [Quick Start](#quick-start)
- [Contributing](#contributing)
- [Overview](#overview)
- [Features](#features)
- [Supported Platforms](#supported-platforms)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
  - [Native Build](#native-build-desktop-application)
  - [Web Build](#web-build-webassembly)
- [Usage](#usage)
  - [Running the GUI Application](#running-the-gui-application)
  - [Using the Web Demo](#using-the-web-demo)
  - [Running Examples](#running-examples)
- [Example Programs](#example-programs)
- [Project Structure](#project-structure)
- [Documentation](#documentation)
- [Testing](#testing)
- [Troubleshooting](#troubleshooting)
- [License](#license)

## 🚀 Try the Demo

Experience wgpu_playground directly in your browser:

**[Try the WebGPU Demo](https://telecos.github.io/wgpu_playground/demo/)**

*Requires a browser with WebGPU support (Chrome 113+, Edge 113+, or Safari with WebGPU enabled)*

## Quick Start

New to wgpu_playground? Get started quickly:

📖 **[Quick Start Guide](docs/QUICK_START.md)** - Get up and running in 5 minutes

📚 **[User Guide](docs/USER_GUIDE.md)** - Comprehensive guide with tutorials and workflows

## Contributing

We welcome contributions to wgpu_playground! Here's how to get started:

### Quick Contribution Guide

1. **Fork and Clone**: Fork the repository and clone it locally
2. **Create a Branch**: `git checkout -b feature/your-feature-name`
3. **Make Changes**: Implement your feature or fix
4. **Test**: Run `cargo test --workspace` and `cargo clippy`
5. **Format**: Run `cargo fmt --all`
6. **Commit**: Write clear commit messages
7. **Push and PR**: Push your branch and create a pull request

### Development Prerequisites

- Rust toolchain (latest stable)
- Familiarity with WebGPU concepts (optional but helpful)
- GPU with Vulkan/Metal/DirectX 12 support

### Running Tests and Checks

```bash
# Format code
cargo fmt --all

# Run lints
cargo clippy --workspace --all-targets -- -D warnings

# Run tests
cargo test --workspace

# Check security
cargo deny check
cargo audit
```

### Documentation and Resources

For comprehensive development information:

🛠️ **[Developer Guide](DEVELOPER_GUIDE.md)** - Architecture, workflows, and debugging tips

📝 **[Contributing Guidelines](CONTRIBUTING.md)** - Detailed contribution process, coding standards, and PR requirements

📋 **[Project Plan](PLAN.md)** - Roadmap and planned features

### Code of Conduct

- Be respectful and constructive
- Welcome newcomers and help them get started
- Focus on the code and ideas, not personalities
- Assume good intentions

## Overview

This is an interactive tool for experimenting with the wgpu crate's WebGPU API capabilities. It provides a graphical user interface built with egui that allows you to explore and test various WebGPU features including rendering pipelines, compute shaders, and ML inferencing operations.

## Features

- **GPU Adapter Selection**: Choose from available GPU adapters with detailed properties and configure power preferences
- **Device Information**: View detailed information about your GPU, including adapter info, device limits, and supported features
- **Device Configuration**: Configure device features and limits before device creation
- **Buffer Configuration**: Create and configure GPU buffers with custom parameters including size, usage flags, and mapping options
- **Texture Loading & Export**: Load textures from image files (PNG, JPEG) with drag-and-drop support and export textures to PNG format
- **Rendering APIs**: Experiment with render pipelines, shaders, buffers, textures, and advanced rendering techniques
- **Compute/ML APIs**: Test compute pipelines, storage buffers, and machine learning operations
- **Configuration Presets**: Load pre-configured setups for common rendering scenarios (PBR materials, shadow mapping, post-processing)
- **Code Export**: Generate standalone Rust projects from your playground configuration with one click
- **State Persistence**: Save and load your playground configurations to/from JSON files
- **URL Sharing**: Generate shareable links with your configuration encoded in the URL for easy collaboration
- **API Reference**: Browse comprehensive WebGPU API documentation inline with method signatures and code examples

**📊 For a comprehensive overview of WebGPU API feature coverage, see [WebGPU API Coverage](docs/WEBGPU_API_COVERAGE.md)**

## Supported Platforms

wgpu_playground runs on multiple platforms with different backend support:

### Desktop Platforms

| Platform | Native Build | Graphics API | Status | Notes |
|----------|-------------|--------------|--------|-------|
| **Linux** | ✅ Full Support | Vulkan (primary), OpenGL | ✅ Fully Tested | Requires X11 or Wayland |
| **macOS** | ✅ Full Support | Metal | ✅ Fully Tested | macOS 10.13+ required |
| **Windows** | ✅ Full Support | DirectX 12 (primary), Vulkan | ✅ Fully Tested | Windows 10+ recommended |

### Web Platforms

| Platform | Browser | WebGPU Support | Status | Notes |
|----------|---------|----------------|--------|-------|
| **Desktop** | Chrome 113+ | ✅ Enabled by default | ✅ Fully Tested | Recommended browser |
| **Desktop** | Edge 113+ | ✅ Enabled by default | ✅ Fully Tested | Based on Chromium |
| **Desktop** | Safari 17+ | 🟡 Requires manual enable | ✅ Tested | Enable in Experimental Features |
| **Desktop** | Firefox Nightly | 🟡 Experimental | ⚠️ Limited Testing | Enable in about:config |
| **Mobile** | Chrome/Edge (Android) | ✅ Enabled by default | ✅ Touch controls supported | Chrome 113+ required |
| **Mobile** | Safari (iOS 17+) | 🟡 Requires manual enable | ✅ Touch controls supported | Enable in Settings |

### Headless/CI Environments

| Environment | Backend | Support | Notes |
|-------------|---------|---------|-------|
| **Linux CI** | Vulkan (lavapipe) | ✅ Full Support | Software rendering for tests |
| **Linux CI** | OpenGL (Mesa) | ✅ Full Support | Fallback software rendering |
| **GitHub Actions** | All platforms | ✅ Automated | Full CI/CD pipeline |

### Graphics Backend Selection

You can select a specific graphics backend using the `WGPU_BACKEND` environment variable:

```bash
# Use Vulkan (Linux, Windows)
WGPU_BACKEND=vulkan cargo run --release

# Use Metal (macOS)
WGPU_BACKEND=metal cargo run --release

# Use DirectX 12 (Windows)
WGPU_BACKEND=dx12 cargo run --release

# Use OpenGL (fallback, all platforms)
WGPU_BACKEND=gl cargo run --release
```

For more details on backend selection and WebGPU implementations, see [WEBGPU_IMPLEMENTATIONS.md](docs/WEBGPU_IMPLEMENTATIONS.md).

## Sharing and Collaboration

The playground supports multiple ways to save and share your work:

### Save/Load State

Use the file operations in the top menu bar to save and load playground configurations:

- **💾 Save State**: Save your current configuration to a JSON file
- **📂 Load State**: Load a previously saved configuration from a JSON file

Saved configurations include:
- Buffer settings (size, usage flags, labels)
- Texture settings (dimensions, format, usage)
- Sampler settings (filtering, addressing modes)
- Shader source code and labels

### URL Sharing

Generate shareable links that encode your entire playground state:

1. Configure your resources (buffers, textures, shaders, etc.)
2. Click **🔗 Generate Share Link** in the top menu bar
3. The link is automatically copied to your clipboard
4. Share the URL with others - they can open it to see your exact configuration

**Example share URL format:**
```
https://telecos.github.io/wgpu_playground/demo?state=eyJ2ZXJzaW9uIjoiMS4wIi...
```

When someone opens a share URL, the playground automatically loads the encoded state and restores all your settings.

**Note**: URL sharing works best for reasonably-sized configurations. Very large shader code or many resources may result in long URLs.

## User Interface

The application provides an organized, collapsible sidebar navigation with immediate visual feedback:

### Navigation Structure

The sidebar is organized into five main sections:

1. **⚙️ Setup & Configuration**:
   - **Adapter Selection**: Choose and configure GPU adapters with detailed properties
   - **Device Config**: Enable/disable WebGPU features and adjust device limits
   - **Device Info**: View comprehensive GPU adapter information and capabilities

2. **🎨 Rendering & Graphics** (Open by default with auto-running example):
   - **Examples & Preview**: Interactive WebGPU rendering examples with live preview
     - Triangle rendering example (auto-runs on startup)
     - Rotating 3D cube with camera controls
     - Real-time rendering preview displayed prominently
     - Canvas controls (size, clear color, camera position)
     - Source code viewer for each example
   - **WGSL Shader Editor**: Interactive shader editor with syntax highlighting
   - **Render Pipeline**: Configure rendering pipeline settings
   - **Render Pass**: Set up render pass configuration
   - **Draw Commands**: Configure draw command parameters

3. **🧮 Compute & ML**:
   - **Compute Panel**: Tools for compute shader and ML operations
   - **Compute Pipeline**: Configure compute pipeline settings
   - **Compute Dispatch**: Set up compute dispatch parameters

4. **📦 Resources**:
   - **Buffers**: Create and configure GPU buffers with usage flags
   - **Textures**: Texture creation and configuration
   - **Samplers**: Sampler configuration for texture filtering
   - **Bind Groups**: Resource binding configuration
   - **Bind Group Layouts**: Layout configuration for bind groups

5. **🔧 Tools & Debugging**:
   - **Resource Inspector**: Inspect created GPU resources
   - **Command Recording**: Record and inspect command buffer execution
   - **Console**: View GPU errors, warnings, and validation messages
   - **Performance**: Monitor performance metrics

### Key Features

- **Immediate Visual Feedback**: The app opens to the Rendering tab with a triangle example auto-running, showcasing WebGPU capabilities immediately
- **Collapsible Sections**: Reduce visual clutter by grouping related features
- **Prominent Preview**: Rendered output is displayed at the top when running examples
- **Interactive Controls**: Canvas size, clear color, and camera controls for 3D examples
- **Mouse Interaction**: Drag to rotate 3D objects, scroll to zoom

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
   - **Code Export**: Generate standalone Cargo projects from examples:
     - Export Triangle, Cube, or custom shader examples
     - Customize project name and configuration
     - Generates complete buildable Rust project with all dependencies
     - Includes Cargo.toml, main.rs, shaders, and README
     - **Hot Reload**: Automatically reload and update shaders when files change on disk (native platforms only)
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

## Prerequisites

Before building wgpu_playground, ensure you have the following installed:

### System Requirements

#### All Platforms
- **Rust**: Latest stable version (1.75.0 or newer recommended)
  - Install via [rustup](https://rustup.rs/): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  - Verify installation: `rustc --version`

#### Platform-Specific Requirements

**Linux:**
- **Display Server**: X11 or Wayland
- **Graphics API**: Vulkan support (recommended) or OpenGL 4.3+
  - Install Vulkan drivers for your GPU:
    - NVIDIA: `nvidia-vulkan-driver` (Debian/Ubuntu) or `nvidia-utils` (Arch)
    - AMD: `mesa-vulkan-drivers` (Debian/Ubuntu) or `vulkan-radeon` (Arch)
    - Intel: `mesa-vulkan-drivers` (Debian/Ubuntu) or `vulkan-intel` (Arch)
- **Build Tools**: `gcc`, `cmake`, `pkg-config`
  - Debian/Ubuntu: `sudo apt install build-essential cmake pkg-config`
  - Arch: `sudo pacman -S base-devel cmake`

**macOS:**
- **macOS Version**: 10.13 (High Sierra) or newer
- **Graphics API**: Metal (built-in on macOS 10.13+)
- **Xcode Command Line Tools**: `xcode-select --install`

**Windows:**
- **Graphics API**: DirectX 12 (Windows 10+) or Vulkan
- **Visual Studio**: Build Tools for Visual Studio 2019 or newer
  - Download from [Microsoft](https://visualstudio.microsoft.com/downloads/)
  - Install "Desktop development with C++" workload

### GPU Requirements

Your GPU must support one of the following graphics APIs:
- **Vulkan 1.1+** (Linux, Windows, Android)
- **Metal 2+** (macOS, iOS)
- **DirectX 12** (Windows 10+)
- **OpenGL 4.3+** (fallback, limited features)

**Note**: For headless systems or CI environments, software rendering is available using Vulkan lavapipe or OpenGL Mesa.

### Web Browser Requirements (for Web Build)

For running the WebAssembly demo in a browser:
- **Chrome/Edge**: Version 113 or newer (WebGPU enabled by default)
- **Safari**: Technology Preview, or Safari 17+ with WebGPU enabled in Experimental Features (Settings → Safari → Advanced → Experimental Features)
- **Firefox**: Nightly build with WebGPU enabled in `about:config`

## Installation

### Native Build (Desktop Application)

#### 1. Clone the Repository

```bash
git clone https://github.com/telecos/wgpu_playground.git
cd wgpu_playground
```

#### 2. Build the Project

```bash
# Development build (faster compilation, includes debug info)
cargo build

# Release build (optimized, recommended for daily use)
cargo build --release
```

#### 3. Run the Application

```bash
# Run development build
cargo run

# Run release build (recommended)
cargo run --release
```

**Platform-Specific Notes:**

- **Linux**: Ensure you have a display server running (X11 or Wayland). On headless systems, the GUI application won't run, but you can still run tests and examples.
- **macOS**: The application should run natively using Metal backend.
- **Windows**: The application will use DirectX 12 by default. For Vulkan, set `WGPU_BACKEND=vulkan` environment variable.

### Web Build (WebAssembly)

Build and run wgpu_playground in a web browser using WebAssembly.

#### 1. Install wasm-pack

```bash
# Using cargo
cargo install wasm-pack

# Or download from https://rustwasm.github.io/wasm-pack/installer/
```

#### 2. Build the WASM Module

```bash
cd crates/wgpu_playground_core
wasm-pack build --target web --release
```

This creates a `pkg/` directory containing the WebAssembly module and JavaScript bindings.

#### 3. Copy Built Files to Web Directory

```bash
# From the repository root
cp -r crates/wgpu_playground_core/pkg web/
```

#### 4. Serve Locally

```bash
cd web

# Using Python 3
python3 -m http.server 8000

# Or using Node.js http-server
npx http-server -p 8000

# Or using Rust's basic-http-server
cargo install basic-http-server
basic-http-server .
```

#### 5. Open in Browser

Navigate to `http://localhost:8000` in a WebGPU-supported browser.

**Note**: The web build is automatically deployed to GitHub Pages at https://telecos.github.io/wgpu_playground/demo/ on every push to the main branch.

## Usage

### Running the GUI Application

After building the project, you can run the main GUI application:

```bash
# Run with default settings (release mode recommended)
cargo run --release

# Run with debug logging enabled
RUST_LOG=debug cargo run --release

# Run with specific backend (Vulkan example)
WGPU_BACKEND=vulkan cargo run --release
```

The application will open a window with an interactive GUI featuring:
- **GPU adapter selection and configuration**
- **Device information and capabilities viewer**
- **Interactive rendering examples** (triangle, rotating cube, etc.)
- **WGSL shader editor** with syntax highlighting
- **Buffer, texture, and sampler configuration tools**
- **Compute shader experimentation**
- **Code export** to standalone Rust projects

### Using the Web Demo

Visit the live demo at https://telecos.github.io/wgpu_playground/demo/ or run it locally:

1. Build and serve the web version (see [Web Build](#web-build-webassembly) section above)
2. Open your browser to the served URL
3. Interact with the WebGPU demo using touch or mouse controls
4. Share your configurations using the URL sharing feature

### Running Examples

The project includes several standalone command-line examples that demonstrate specific WebGPU features. These examples run independently of the GUI application and are useful for learning and testing.

#### Available Examples

**List all examples:**
```bash
# List example files
ls crates/wgpu_playground_examples/examples/

# Or view in cargo package metadata
cargo tree -p wgpu_playground_examples
```

**Run specific examples:**
```bash
# Basic triangle rendering
cargo run --package wgpu_playground_examples --example triangle

# 3D rotating cube
cargo run --package wgpu_playground_examples --example rotating_cube

# Render to texture
cargo run --package wgpu_playground_examples --example render_to_texture

# Post-processing effects (blur, grayscale, edge detection)
cargo run --package wgpu_playground_examples --example post_processing

# Compute shader example
cargo run --package wgpu_playground_examples --example compute_render_sharing

# Instanced rendering
cargo run --package wgpu_playground_examples --example instanced_rendering

# Multisampling anti-aliasing
cargo run --package wgpu_playground_examples --example multisampling

# Backend selection
cargo run --package wgpu_playground_examples --example backend_selection

# Error handling patterns
cargo run --package wgpu_playground_examples --example error_handling
```

Each example includes detailed output explaining what operations are being performed and their results.

### Common Workflows

#### Creating and Testing Shaders

1. Run the GUI application: `cargo run --release`
2. Navigate to the **Rendering** tab in the sidebar
3. Open the **WGSL Shader Editor**
4. Load an example shader or write your own
5. Compile and view errors in real-time
6. Export your shader as a standalone project

#### Experimenting with Buffers and Textures

1. Navigate to the **Resources** section in the sidebar
2. Configure buffer settings (size, usage flags, labels)
3. Create textures with custom dimensions and formats
4. Configure samplers for texture filtering
5. Bind resources to shaders using bind groups

#### Sharing Your Work

1. Configure your resources and shaders
2. Click **🔗 Generate Share Link** in the menu bar
3. Copy the URL and share it with others
4. Recipients can open the link to see your exact configuration

For detailed tutorials and workflows, see the **[User Guide](docs/USER_GUIDE.md)**.

## Example Programs

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

#### Post-Processing Effects Example

An advanced multi-pass rendering example demonstrating real-world post-processing techniques:

```bash
cargo run --package wgpu_playground_examples --example post_processing
```

This example demonstrates:
- Multiple render-to-texture passes (5 passes total)
- Rendering a scene to an offscreen texture
- Applying sequential post-processing effects: blur, grayscale, edge detection
- Multiple render pipelines with different fragment shaders
- Full-screen quad rendering for post-processing
- Texture sampling in fragment shaders with various algorithms
- Real-world post-processing workflow commonly used in games and graphics applications

**Example output:**
```
=== Post-Processing Effects Example ===

This example demonstrates multiple post-processing effects:
  Pass 1: Render scene to texture
  Pass 2: Apply blur effect
  Pass 3: Apply grayscale effect
  Pass 4: Apply edge detection effect
  Pass 5: Display final result

Using adapter: NVIDIA GeForce RTX 3080
Backend: Vulkan
✓ GPU device created

=== Scene Pass Setup ===
✓ Scene pipeline created
✓ Render textures created

=== Post-Processing Pipelines Setup ===
✓ All post-processing pipelines created

✓ All bind groups created

=== Executing Multi-Pass Post-Processing ===

Pass 1: Rendering scene...
  ✓ Scene rendered
Pass 2: Applying blur effect...
  ✓ Blur applied
Pass 3: Applying grayscale effect...
  ✓ Grayscale applied
Pass 4: Applying edge detection...
  ✓ Edge detection applied
Pass 5: Displaying final result...
  ✓ Final result displayed

✓ All render commands submitted to GPU
✓ Rendering complete

=== Post-Processing Example Complete ===

This example successfully demonstrated:
  • Multiple render-to-texture passes (5 passes total)
  • Scene rendering to offscreen texture
  • Blur post-processing effect
  • Grayscale post-processing effect
  • Edge detection post-processing effect
  • Multiple render pipelines with different shaders
  • Texture sampling in fragment shaders
  • Full-screen quad rendering

Key WebGPU concepts:
  - Render-to-texture workflow (framebuffers)
  - Multiple render pipelines for different effects
  - Bind groups for texture and sampler resources
  - Multi-pass rendering with intermediate textures
```

**Post-Processing Effects Implemented:**
- **Blur**: 3x3 box blur kernel for smoothing
- **Grayscale**: Luminance-based color conversion using standard weights
- **Edge Detection**: Sobel operator for detecting edges in the image

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

#### GPU Particle System Example

A complete GPU particle system demonstrating advanced compute-to-render integration:

```bash
cargo run --package wgpu_playground_examples --example particle_system
```

This example demonstrates:
- Compute shaders for particle physics simulation (position, velocity, lifetime)
- Storage buffers shared between compute and render pipelines
- Instanced rendering to efficiently draw thousands of particles
- Dynamic particle spawning with queue.write_buffer
- Alpha blending and soft circular particle rendering
- Gravity, damping, and boundary collision physics

**Key WebGPU APIs exercised:**
- **Compute pipelines**: GPU-accelerated particle physics with workgroup dispatching
- **Storage buffers**: Shared STORAGE + VERTEX buffer usage for compute-to-render data flow
- **Instanced rendering**: Single draw call for 10,000 particle quads
- **Dynamic updates**: CPU-to-GPU particle spawning per frame

**Example output:**
```
=== GPU Particle System Example ===

This example demonstrates:
  • Compute shader for particle physics simulation
  • Storage buffers shared between compute and render
  • Instanced rendering for efficient particle display
  • Dynamic particle spawning with queue.write_buffer

Using adapter: Your GPU Name
Backend: Vulkan

✓ GPU device created

Initialized 10000 particles (all dead initially)
✓ Created particle buffer
  - Size: 480000 bytes (10000 particles × 48 bytes)
  - Usage: STORAGE (compute access) + VERTEX (render access) + COPY_DST (updates)

✓ Created simulation parameters buffer
  - Delta time: 0.016 seconds
  - Gravity: -0.5
  - Damping: 0.98

Setting up compute pipeline...
✓ Compute pipeline created

Setting up render pipeline...
✓ Render pipeline created
  - Using instanced rendering (one quad per particle)
  - Alpha blending enabled for smooth particles

Running particle simulation...

Frame 0:
  ✓ Spawned 50 new particles
  ✓ Compute pass: Updated particle physics
  ✓ Render pass: Drew 50 active particles (instanced)
  ✓ Frame complete

Frame 2:
  ✓ Spawned 50 new particles
  ✓ Compute pass: Updated particle physics
  ✓ Render pass: Drew 100 active particles (instanced)
  ✓ Frame complete

=== Particle System Example Complete ===

Successfully demonstrated:
  ✓ Compute shader particle physics simulation
    - Position and velocity integration
    - Gravity and damping forces
    - Lifetime management
  ✓ Storage buffer sharing (STORAGE + VERTEX usage)
  ✓ Instanced rendering (10000 instances per frame)
  ✓ Dynamic particle spawning with queue.write_buffer
  ✓ Compute-to-render pipeline integration

Key WebGPU Concepts:
  • Compute pipelines: GPU-accelerated physics simulation
  • Storage buffers: Shared data between compute and render
  • Instanced rendering: Efficient drawing of many objects
  • Dynamic updates: CPU-to-GPU data transfer per frame
```

#### Advanced Compute Blur Example

An advanced compute shader example demonstrating Gaussian blur with workgroup shared memory:

```bash
cargo run --package wgpu_playground_examples --example advanced_compute_blur
```

This example demonstrates:
- `var<workgroup>` shared memory for efficient tile-based processing
- `workgroupBarrier()` synchronization between shader invocations
- Multiple dispatch calls (separable blur: horizontal + vertical passes)
- Storage textures for read/write operations
- Gaussian blur using a 11x11 kernel (radius=5)

**Example output:**
```
=== Advanced Compute Shader Example ===
Gaussian Blur with Shared Memory and Multiple Dispatches

Using adapter: NVIDIA GeForce RTX 3080
Backend: Vulkan

Generating 512x512 test image...
  ✓ Created checkerboard pattern with gradient

Creating textures...
  ✓ Input texture: 512x512 (TEXTURE_BINDING)
  ✓ Uploaded 1048576 bytes of image data
  ✓ Intermediate texture: 512x512 (STORAGE_BINDING + TEXTURE_BINDING)
  ✓ Output texture: 512x512 (STORAGE_BINDING)

Creating horizontal blur compute pipeline...
  ✓ Horizontal blur pipeline created
  ✓ Uses workgroup shared memory for caching
  ✓ Applies Gaussian blur in horizontal direction

Creating vertical blur compute pipeline...
  ✓ Vertical blur pipeline created
  ✓ Uses workgroup shared memory for caching
  ✓ Applies Gaussian blur in vertical direction

  ✓ Created bind groups for horizontal and vertical passes

Executing blur passes...

Pass 1: Horizontal Blur
  - Input: Original image
  - Output: Intermediate texture
  - Workgroups: 32x32 (1024 total)
  ✓ Horizontal blur dispatched

Pass 2: Vertical Blur
  - Input: Intermediate texture
  - Output: Final blurred image
  - Workgroups: 32x32 (1024 total)
  ✓ Vertical blur dispatched

  ✓ Both passes completed successfully

=== Example Complete ===

This example demonstrated:
  ✓ Workgroup shared memory (var<workgroup>) for efficient tile caching
  ✓ workgroupBarrier() for synchronization between threads
  ✓ Multiple dispatch calls (separable blur: horizontal + vertical)
  ✓ Storage textures for read/write operations
  ✓ Gaussian blur using 11x11 kernel

Key WebGPU APIs Exercised:
  • var<workgroup>: Shared memory within a workgroup
  • workgroupBarrier(): Ensures all threads complete before proceeding
  • textureLoad/textureStore: Direct texture access in compute shaders
  • Multiple dispatch: Sequential passes for efficient separable filters

Performance Benefits:
  • Shared memory reduces global memory bandwidth
  • Separable blur: O(n) instead of O(n²) for 2D convolution
  • Workgroup tiling enables efficient texture caching
```

Other available examples:
- `advanced_compute_blur` - Advanced compute shader with workgroup shared memory, barriers, and multi-dispatch
- `particle_system` - GPU particle system with compute shaders and instanced rendering
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

[![Coverage](https://codecov.io/gh/telecos/wgpu_playground/branch/main/graph/badge.svg)](https://codecov.io/gh/telecos/wgpu_playground)

This project uses `cargo-llvm-cov` for code coverage reporting with automated uploads to [Codecov](https://codecov.io/gh/telecos/wgpu_playground).

**Coverage Targets:**
- Overall project coverage: **70%** (with 2% threshold)
- New code (patches): **60%** (with 5% threshold)

See [COVERAGE.md](COVERAGE.md) for details on:

- Running coverage locally
- Coverage thresholds and configuration
- CI integration
- Viewing coverage reports

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
- **[WGSL Shader Guide](docs/WGSL_SHADER_GUIDE.md)** - Complete guide to writing WGSL shaders:
  - Shader structure and anatomy (vertex, fragment, compute)
  - Data types, vectors, matrices, and structs
  - Built-in functions and operations
  - Uniforms, bindings, and resource management
  - Debugging techniques and best practices
  - Common patterns and real-world examples

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

- **[CONFIGURATION_PRESETS.md](CONFIGURATION_PRESETS.md)** - Guide to configuration presets for common rendering scenarios (PBR, shadow mapping, post-processing)
- **[WEBGPU_API_COVERAGE.md](docs/WEBGPU_API_COVERAGE.md)** - Comprehensive mapping of WebGPU API features to playground implementation with implementation status
- **[USER_GUIDE.md](docs/USER_GUIDE.md)** - Comprehensive end-user documentation with tutorials and workflows
- **[QUICK_START.md](docs/QUICK_START.md)** - Quick start guide for new users
- **[WGSL_SHADER_GUIDE.md](docs/WGSL_SHADER_GUIDE.md)** - Complete guide to writing WGSL shaders including structure, built-in functions, and debugging
- **[VISUAL_REGRESSION_TESTING.md](docs/VISUAL_REGRESSION_TESTING.md)** - Complete guide to the visual regression testing framework, including examples, API reference, and best practices
- **[SHADER_EDITOR.md](docs/SHADER_EDITOR.md)** - Complete guide to the WGSL Shader Editor including usage examples, tips, and API reference
- **[WEBGPU_IMPLEMENTATIONS.md](docs/WEBGPU_IMPLEMENTATIONS.md)** - Guide to WebGPU implementations (wgpu vs Dawn), architecture, and how to switch between them
- **[GUI_FRAMEWORK_EVALUATION.md](GUI_FRAMEWORK_EVALUATION.md)** - Detailed evaluation and rationale for selecting egui as the GUI framework, including comparison with iced and imgui-wgpu
- **[PLAN.md](PLAN.md)** - Complete project roadmap with implementation phases
- **[UI_MOCKUP.md](UI_MOCKUP.md)** - UI design and layout documentation
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Development guidelines and contribution instructions

## License

MIT
