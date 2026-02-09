# System Architecture and Design

## Overview

**wgpu_playground** is an interactive WebGPU experimentation tool built in Rust. It provides a comprehensive graphical interface for exploring and testing all aspects of the WebGPU API, from basic buffer creation to advanced rendering pipelines and compute shaders.

### Core Purpose

- **Educational**: Learn WebGPU concepts through interactive experimentation
- **Development Tool**: Rapidly prototype and test WebGPU features
- **Reference Implementation**: Demonstrate best practices for WebGPU usage in Rust

### Technology Stack

- **Core Graphics API**: [wgpu](https://github.com/gfx-rs/wgpu) - Pure Rust WebGPU implementation
- **GUI Framework**: [egui](https://github.com/emilk/egui) - Immediate mode GUI library
- **Window Management**: [winit](https://github.com/rust-windowing/winit) - Cross-platform windowing
- **Language**: Rust (2021 Edition)
- **Optional Backend**: [Dawn](https://dawn.googlesource.com/dawn) - Chromium's C++ WebGPU implementation

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         User Interface                          │
│                      (egui-based GUI)                           │
├─────────────────────────────────────────────────────────────────┤
│                     wgpu_playground_gui                         │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │ Window Mgmt  │  │  Event Loop  │  │  Rendering   │         │
│  │   (winit)    │  │  (winit)     │  │  (egui-wgpu) │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    wgpu_playground_core                         │
│  ┌────────────────────────────────────────────────────────┐    │
│  │              Panel Modules (UI Components)             │    │
│  │  • AdapterSelection  • DeviceConfig  • DeviceInfo     │    │
│  │  • RenderingPanel    • ComputePanel  • BufferPanel    │    │
│  │  • TexturePanel      • SamplerPanel  • BindGroupPanel │    │
│  │  • RenderPipeline    • ComputePipeline               │    │
│  │  • RenderPass        • CommandRecording              │    │
│  │  • ResourceInspector • Console       • Performance    │    │
│  └────────────────────────────────────────────────────────┘    │
│  ┌────────────────────────────────────────────────────────┐    │
│  │           Core WebGPU Abstractions                     │    │
│  │  • Adapter      • Device       • Queue                │    │
│  │  • Buffer       • Texture      • Sampler              │    │
│  │  • Shader       • Pipeline     • BindGroup            │    │
│  │  • CommandEncoder • RenderPass • ComputePass          │    │
│  └────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                   WebGPU Implementation Layer                   │
│  ┌──────────────────────────┐  ┌──────────────────────────┐   │
│  │    wgpu (Default)        │  │   Dawn (Optional)        │   │
│  │  Rust implementation     │  │  C++ implementation      │   │
│  │  Used by Firefox         │  │  Used by Chromium        │   │
│  └──────────────────────────┘  └──────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Graphics API Backends                        │
│      Vulkan  │  Metal  │  DirectX 12  │  OpenGL  │  WebGPU     │
└─────────────────────────────────────────────────────────────────┘
```

## Workspace Structure

The project uses a Cargo workspace with three main crates:

```
wgpu_playground/
├── crates/
│   ├── wgpu_playground_core/     # Core WebGPU functionality
│   ├── wgpu_playground_gui/      # GUI application
│   └── wgpu_playground_examples/ # Standalone examples
├── assets/                        # Static resources
│   ├── shaders/                   # WGSL shader files
│   ├── textures/                  # Texture assets
│   └── models/                    # 3D model files
├── docs/                          # Documentation
└── tests/                         # Integration tests
```

## Module Structure

### wgpu_playground_core

The core crate provides all WebGPU functionality and UI panels:

#### Panel Modules (UI Components)
- **`adapter_selection.rs`**: GPU adapter enumeration and selection
- **`device_config.rs`**: Device features and limits configuration
- **`rendering.rs`**: Rendering pipeline experimentation with example gallery
- **`compute.rs`**: Compute shader and ML operations panel
- **`buffer_panel.rs`**: GPU buffer creation and configuration
- **`texture_panel.rs`**: Texture creation with format/dimension options
- **`sampler_panel.rs`**: Sampler configuration (filtering, addressing)
- **`bind_group_panel.rs`**: Bind group resource binding interface
- **`render_pipeline_panel.rs`**: Render pipeline state configuration
- **`compute_pipeline_panel.rs`**: Compute pipeline setup
- **`render_pass_panel.rs`**: Render pass configuration
- **`command_recording_panel.rs`**: Command buffer recording interface
- **`resource_inspector.rs`**: Runtime resource inspection
- **`console.rs`**: GPU error/warning output
- **`performance_panel.rs`**: Performance metrics

#### Core WebGPU Abstractions
- **`adapter.rs`**: GPU adapter request and backend selection
- **`buffer.rs`**: GPU buffer creation and management
- **`texture.rs`**: Texture creation with all formats/dimensions
- **`sampler.rs`**: Sampler objects for texture filtering
- **`shader.rs`**: WGSL shader compilation and validation
- **`render_pipeline.rs`**: Graphics pipeline configuration
- **`bind_group.rs`**: Resource binding for shaders
- **`command_encoder.rs`**: Command recording

### wgpu_playground_gui

The GUI application crate manages the window and event loop:
- **`main.rs`**: Application entry point, window creation
- **`app.rs`**: Main application state and tab management

### wgpu_playground_examples

Standalone example programs:
- **`triangle.rs`**: Basic triangle rendering
- **`rotating_cube.rs`**: 3D rendering with depth testing
- **`texture_mapping.rs`**: Texture creation and sampling
- **`compute_pass.rs`**: Compute shader execution

## Data Flow

### Application Initialization

1. Initialize logging
2. Create event loop and window
3. Request GPU adapter
4. Create device and queue
5. Configure surface
6. Initialize egui renderer
7. Create PlaygroundApp with all panels

### Render Loop

1. Get surface texture
2. Create command encoder
3. Clear screen (render pass)
4. Run egui context
5. Route to selected panel
6. Tessellate and render egui
7. Submit command buffer
8. Present surface texture

## Key Design Decisions

### 1. Workspace Architecture

**Decision**: Multi-crate workspace structure

**Benefits**:
- Separation of concerns (core logic, GUI, examples)
- Core crate can be used as a library
- Independent testing

### 2. GUI Framework: egui

**Decision**: Use egui for the graphical interface

**Benefits**:
- Immediate mode = simpler state management
- Excellent WebGPU integration (egui-wgpu)
- Rich built-in widgets
- Minimal overhead

### 3. WebGPU Implementation

**Decision**: wgpu as default with optional Dawn support

**Benefits**:
- Pure Rust, no C++ build dependencies
- Fast compilation
- Optional Dawn for cross-implementation comparison

## API Coverage Tracking

The application tracks which WebGPU APIs have been exercised:

- **ApiCoverageTracker**: Global singleton for recording API usage
- **Categories**: Adapter, Device, Buffer, Texture, Sampler, Shader, BindGroup, Pipeline, RenderPass, ComputePass, CommandEncoder, Queue
- **UI Panel**: Shows coverage percentage and links to documentation

## Extension Points

### Adding New Panels

1. Create new module in `wgpu_playground_core/src/`
2. Implement panel struct with `ui()` method
3. Add to `PlaygroundApp` in `app.rs`
4. Add tab routing

### Adding New Examples

1. Create file in `wgpu_playground_examples/examples/`
2. Implement using wgpu APIs
3. Add to example gallery
4. Document in README

## Performance Considerations

- **Immediate mode GUI**: egui minimizes retained state
- **Command batching**: Group GPU operations when possible
- **Resource caching**: Reuse pipelines and bind groups
- **Lazy initialization**: Create resources on demand
