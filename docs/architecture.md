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

## System Architecture

### High-Level Architecture

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
│  ┌────────────────────────────────────────────────────────┐    │
│  │              Supporting Systems                        │    │
│  │  • Assets (Shaders, Textures, Models)                 │    │
│  │  • Error Handling & Validation                        │    │
│  │  • Visual Regression Testing                          │    │
│  │  • Performance Metrics                                │    │
│  │  • Example Gallery                                    │    │
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
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      GPU Hardware                               │
└─────────────────────────────────────────────────────────────────┘
```

### Workspace Structure

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

### 1. wgpu_playground_core

The core crate provides all WebGPU functionality and UI panels. It's organized into several categories:

#### Panel Modules (UI Components)
These modules provide egui-based UI panels for interactive configuration:

- **`adapter_selection.rs`**: GPU adapter enumeration and selection interface
- **`device_config.rs`**: Device features and limits configuration
- **`device_info.rs`**: Display GPU adapter information and capabilities
- **`rendering.rs`**: Rendering pipeline experimentation panel with example gallery
- **`compute.rs`**: Compute shader and ML operations panel
- **`buffer_panel.rs`**: GPU buffer creation and configuration
- **`texture_panel.rs`**: Texture creation with format/dimension options
- **`sampler_panel.rs`**: Sampler configuration (filtering, addressing)
- **`bind_group_panel.rs`**: Bind group resource binding interface
- **`bind_group_layout_panel.rs`**: Bind group layout configuration
- **`render_pipeline_panel.rs`**: Render pipeline state configuration
- **`compute_pipeline_panel.rs`**: Compute pipeline setup
- **`render_pass_panel.rs`**: Render pass configuration
- **`draw_command_panel.rs`**: Draw command builder
- **`compute_dispatch_panel.rs`**: Compute dispatch configuration
- **`command_recording_panel.rs`**: Command buffer recording interface
- **`resource_inspector.rs`**: Runtime resource inspection and debugging
- **`console.rs`**: GPU error/warning console output
- **`performance_panel.rs`**: Performance metrics and profiling

#### Core WebGPU Abstractions
Low-level wrappers around WebGPU concepts:

- **`adapter.rs`**: GPU adapter request and backend selection
- **`buffer.rs`**: GPU buffer creation and management
- **`texture.rs`**: Texture creation with all formats/dimensions
- **`sampler.rs`**: Sampler objects for texture filtering
- **`shader.rs`**: WGSL shader compilation and validation
- **`shader_editor.rs`**: Interactive shader editor with syntax highlighting
- **`render_pipeline.rs`**: Graphics pipeline configuration
- **`bind_group.rs`**: Resource binding for shaders
- **`pipeline_layout.rs`**: Pipeline layout creation
- **`command_encoder.rs`**: Command recording
- **`render_pass_encoder.rs`**: Render pass command recording
- **`compute_pass_encoder.rs`**: Compute pass command recording
- **`render_bundle_encoder.rs`**: Reusable render command bundles
- **`queue.rs`**: Command submission and buffer writes
- **`surface.rs`**: Window surface management
- **`query_set.rs`**: GPU query sets for profiling

#### Supporting Systems

- **`assets.rs`**: Asset loading (shaders, textures, models)
- **`error.rs`**: Error handling and device error callbacks
- **`examples.rs`**: Example gallery and categorization
- **`implementation.rs`**: WebGPU implementation selection (wgpu vs Dawn)
- **`dawn_wrapper.rs`**: Dawn C++ FFI bindings (when `dawn` feature enabled)
- **`performance_metrics.rs`**: Performance tracking and statistics
- **`visual_regression/`**: Visual regression testing framework

### 2. wgpu_playground_gui

The GUI application crate manages the window and event loop:

- **`main.rs`**: Application entry point, window creation, event loop
- **`app.rs`**: Main application state and tab management

Key responsibilities:
- Window creation and management (winit)
- Event loop handling
- Surface configuration
- egui integration (egui-wgpu)
- Render loop orchestration
- Tab-based interface management

### 3. wgpu_playground_examples

Standalone example programs demonstrating WebGPU features:

- **`triangle.rs`**: Basic triangle rendering
- **`rotating_cube.rs`**: 3D rendering with depth testing and uniforms
- **`texture_mapping.rs`**: Texture creation and sampling
- **`render_to_texture.rs`**: Multi-pass rendering to offscreen targets
- **`compute_render_sharing.rs`**: Buffer sharing between compute/render
- **`instanced_rendering.rs`**: GPU instancing
- **`multisampling.rs`**: MSAA anti-aliasing
- **`backend_selection.rs`**: Backend enumeration
- **`compute_pass.rs`**: Compute shader execution
- **`shader_loading.rs`**: Shader compilation
- **`error_handling.rs`**: Error handling patterns

## Data Flow

### Application Initialization

```
┌──────────────┐
│ main.rs      │  1. Initialize logging
│              │  2. Create event loop
└──────┬───────┘
       │
       ▼
┌──────────────────┐
│ Event Loop       │  3. Create window
│                  │  4. Request GPU adapter
└──────┬───────────┘
       │
       ▼
┌──────────────────┐
│ AppState::new()  │  5. Create device & queue
│                  │  6. Configure surface
│                  │  7. Initialize egui renderer
│                  │  8. Create PlaygroundApp
└──────┬───────────┘
       │
       ▼
┌──────────────────┐
│ PlaygroundApp    │  9. Initialize all panels
│                  │  10. Set up error handlers
│                  │  11. Load example gallery
└──────────────────┘
```

### Render Loop

```
┌──────────────────┐
│ Event Loop       │
└────────┬─────────┘
         │
         ▼
┌──────────────────────────────────────┐
│ WindowEvent::RedrawRequested         │
└────────┬─────────────────────────────┘
         │
         ▼
┌──────────────────────────────────────┐
│ AppState::render()                   │
├──────────────────────────────────────┤
│ 1. Get surface texture               │
│ 2. Create command encoder            │
│ 3. Clear screen (render pass)        │
└────────┬─────────────────────────────┘
         │
         ▼
┌──────────────────────────────────────┐
│ egui Context::run()                  │
├──────────────────────────────────────┤
│ 4. Process egui input                │
│ 5. Call PlaygroundApp::ui()          │
│    ├─ Render tab bar                 │
│    ├─ Route to selected panel        │
│    └─ Panel renders UI & executes    │
│       GPU operations                 │
└────────┬─────────────────────────────┘
         │
         ▼
┌──────────────────────────────────────┐
│ egui Rendering                       │
├──────────────────────────────────────┤
│ 6. Tessellate egui shapes            │
│ 7. Update egui textures              │
│ 8. Update egui buffers               │
│ 9. Render egui (render pass)         │
└────────┬─────────────────────────────┘
         │
         ▼
┌──────────────────────────────────────┐
│ Submit & Present                     │
├──────────────────────────────────────┤
│ 10. Submit command buffer to queue  │
│ 11. Present surface texture          │
└──────────────────────────────────────┘
```

### WebGPU Resource Creation Flow

Example: Creating and using a render pipeline

```
┌─────────────────────┐
│ User Action in UI   │
│ (RenderingPanel)    │
└─────────┬───────────┘
          │
          ▼
┌─────────────────────────────────┐
│ Load Shader                     │
│ (shader_editor.rs)              │
│ ├─ Read WGSL from file/editor   │
│ ├─ Validate syntax              │
│ └─ Create ShaderModule          │
└─────────┬───────────────────────┘
          │
          ▼
┌─────────────────────────────────┐
│ Configure Pipeline              │
│ (render_pipeline_panel.rs)     │
│ ├─ Set vertex layout            │
│ ├─ Configure primitive state    │
│ ├─ Set depth/stencil            │
│ ├─ Configure fragment state     │
│ └─ Set blend modes              │
└─────────┬───────────────────────┘
          │
          ▼
┌─────────────────────────────────┐
│ Create GPU Resources            │
│ (wgpu Device API)               │
│ ├─ Create vertex buffer         │
│ ├─ Create uniform buffer        │
│ ├─ Create bind group layout     │
│ ├─ Create bind group            │
│ ├─ Create pipeline layout       │
│ └─ Create render pipeline       │
└─────────┬───────────────────────┘
          │
          ▼
┌─────────────────────────────────┐
│ Record Commands                 │
│ (command_encoder.rs)            │
│ ├─ Create command encoder       │
│ ├─ Begin render pass            │
│ ├─ Set pipeline                 │
│ ├─ Set bind groups              │
│ ├─ Set vertex buffer            │
│ ├─ Issue draw call              │
│ ├─ End render pass              │
│ └─ Finish encoding              │
└─────────┬───────────────────────┘
          │
          ▼
┌─────────────────────────────────┐
│ Submit to Queue                 │
│ (queue.rs)                      │
│ └─ Submit command buffer        │
└─────────┬───────────────────────┘
          │
          ▼
┌─────────────────────────────────┐
│ GPU Execution                   │
│ └─ Asynchronous execution       │
└─────────────────────────────────┘
```

## Key Design Decisions

### 1. Workspace Architecture

**Decision**: Use a multi-crate workspace structure

**Rationale**:
- **Separation of concerns**: Core logic, GUI, and examples are independent
- **Reusability**: Core crate can be used as a library
- **Build optimization**: Examples can be built independently
- **Testing**: Easier to test core logic without GUI dependencies

**Trade-offs**:
- Slightly more complex project structure
- Need to manage dependencies between crates
- Benefits outweigh complexity for a project of this size

### 2. GUI Framework Selection: egui

**Decision**: Use egui for the graphical interface

**Rationale**:
- **Immediate mode**: Simpler state management, natural control flow
- **WebGPU integration**: Excellent egui-wgpu backend support
- **Performance**: Minimal overhead, suitable for real-time applications
- **Rich widgets**: Built-in support for complex UI elements
- **Active development**: Well-maintained with strong community

**Alternatives considered**:
- **iced**: Elm-inspired architecture, but more complex state management
- **imgui-rs**: C++ FFI overhead, less "Rusty" API

See [GUI_FRAMEWORK_EVALUATION.md](../GUI_FRAMEWORK_EVALUATION.md) (in repository root) for detailed comparison.

### 3. WebGPU Implementation: wgpu (with optional Dawn)

**Decision**: Use wgpu as default with optional Dawn support

**Rationale**:
- **wgpu benefits**:
  - Pure Rust, no C++ build dependencies
  - Fast compilation, native Rust tooling
  - Used by Firefox, production-ready
  - Excellent documentation and community
- **Dawn as optional**:
  - Allows comparison with Chromium's implementation
  - Feature flag enables it only when needed
  - Provides educational value
  - Validates cross-implementation compatibility

**Implementation**:
- Runtime selection via `WEBGPU_IMPL` environment variable
- Compile-time inclusion via `--features dawn`
- Falls back to wgpu-core if Dawn build fails

See [WEBGPU_IMPLEMENTATIONS.md](WEBGPU_IMPLEMENTATIONS.md) for details.

### 4. Error Handling Strategy

**Decision**: Comprehensive error handling with user-friendly feedback

**Rationale**:
- **Device error callbacks**: Capture GPU errors asynchronously
- **Validation messages**: Display in dedicated console panel
- **Graceful degradation**: Continue operation when possible
- **Educational value**: Users learn from error messages

**Implementation**:
```rust
// In error.rs
pub fn setup_device_error_handling(device: &Device) {
    device.on_uncaptured_error(Box::new(|error| {
        log::error!("Uncaptured device error: {:?}", error);
    }));
}
```

### 5. Asset Management

**Decision**: File-based assets with relative path resolution

**Rationale**:
- **Simplicity**: Easy to add/modify shaders and textures
- **External editing**: Use specialized editors for assets
- **Version control friendly**: Text-based WGSL shaders
- **Hot reloading potential**: Can watch for file changes

**Structure**:
```
assets/
├── shaders/     # WGSL shader source files
├── textures/    # PNG/JPG texture files
└── models/      # 3D model files (future)
```

### 6. Panel-Based Architecture

**Decision**: Each UI section is a self-contained panel module

**Rationale**:
- **Modularity**: Panels are independent, testable units
- **Maintainability**: Easy to add/remove features
- **State encapsulation**: Each panel manages its own state
- **Parallel development**: Multiple developers can work on different panels

**Pattern**:
```rust
pub struct SomePanel {
    // Panel-specific state
    config: SomeConfig,
    validation_errors: Vec<String>,
}

impl SomePanel {
    pub fn new() -> Self { /* ... */ }
    
    pub fn ui(&mut self, ui: &mut egui::Ui, device: &Device, queue: &Queue) {
        // Render UI and handle interactions
    }
}
```

### 7. Visual Regression Testing

**Decision**: Implement GPU-based visual regression testing

**Rationale**:
- **Visual verification**: Catch rendering changes automatically
- **CI integration**: Automated testing in pipeline
- **Reference images**: Golden images for comparison
- **Diff visualization**: Highlight pixel differences

**Implementation**:
- Render to offscreen textures
- Capture to PNG images
- Pixel-wise comparison with tolerance
- Generate diff images on failure

See [VISUAL_REGRESSION_TESTING.md](VISUAL_REGRESSION_TESTING.md) for details.

### 8. Example-Driven Development

**Decision**: Maintain comprehensive standalone examples

**Rationale**:
- **Learning resource**: Each example demonstrates specific features
- **Testing**: Validates that APIs work end-to-end
- **Documentation**: Runnable code is better than text
- **Debugging**: Isolated examples are easier to debug

**Example categories**:
- **Basic rendering**: Triangle, textured quad, rotating cube
- **Advanced rendering**: Multi-pass, instancing, MSAA
- **Compute**: Compute shaders, buffer sharing
- **System**: Backend selection, error handling, shader loading

### 9. Shader Editor Integration

**Decision**: Inline WGSL shader editor with syntax highlighting

**Rationale**:
- **Rapid iteration**: Edit and test shaders in the app
- **Learning tool**: Immediate feedback on shader changes
- **Syntax highlighting**: Improves readability
- **Error reporting**: Display compilation errors inline

**Features**:
- Keyword/type highlighting
- Line numbers
- File loading from assets
- Real-time compilation
- Error display

See [SHADER_EDITOR.md](SHADER_EDITOR.md) for details.

### 10. Performance Monitoring

**Decision**: Built-in performance metrics and profiling

**Rationale**:
- **Optimization**: Identify bottlenecks during development
- **Education**: Understand GPU performance characteristics
- **Debugging**: Performance regressions are bugs too
- **Real-time feedback**: See impact of changes immediately

**Metrics tracked**:
- Frame time and FPS
- GPU queue operations
- Memory usage
- Command buffer size
- Draw call counts

## Component Interactions

### Adapter Selection Flow

```
┌────────────────────┐
│ AdapterSelection   │  User selects adapter/backend
│ Panel              │  preferences
└─────────┬──────────┘
          │
          ▼
┌────────────────────┐
│ adapter.rs         │  Request adapter with options
│                    │  (backend, power preference)
└─────────┬──────────┘
          │
          ▼
┌────────────────────┐
│ wgpu Instance      │  Enumerate adapters
│                    │  Match criteria
└─────────┬──────────┘
          │
          ▼
┌────────────────────┐
│ Return Adapter     │  Adapter info displayed
│                    │  in DeviceInfo panel
└────────────────────┘
```

### Shader Compilation Flow

```
┌────────────────────┐
│ ShaderEditor       │  User edits WGSL code
│ Panel              │  or loads from file
└─────────┬──────────┘
          │
          ▼
┌────────────────────┐
│ shader.rs          │  Parse and validate WGSL
│ ShaderModule       │  
└─────────┬──────────┘
          │
          ▼
┌────────────────────┐
│ wgpu Device        │  Create shader module
│                    │  Compile to IR
└─────────┬──────────┘
          │
          ├─Success──▶ ShaderModule ready for pipeline
          │
          └─Error────▶ Display in ShaderEditor with
                      line numbers and messages
```

### Resource Binding Flow

```
┌────────────────────┐
│ BufferPanel        │  Create buffers
│ TexturePanel       │  Create textures
│ SamplerPanel       │  Create samplers
└─────────┬──────────┘
          │
          ▼
┌────────────────────┐
│ BindGroupLayout    │  Define binding slots
│ Panel              │  (binding numbers, types,
│                    │   visibility)
└─────────┬──────────┘
          │
          ▼
┌────────────────────┐
│ BindGroupPanel     │  Bind resources to slots
│                    │  (match layout)
└─────────┬──────────┘
          │
          ▼
┌────────────────────┐
│ RenderPipeline     │  Use bind group layout
│ Panel              │  in pipeline creation
└─────────┬──────────┘
          │
          ▼
┌────────────────────┐
│ RenderPass         │  Set bind group during
│ Recording          │  command recording
└────────────────────┘
```

## Extension Points

The architecture is designed to be extensible in several ways:

### 1. Adding New Panels

To add a new UI panel:

1. Create a new module in `wgpu_playground_core/src/`
2. Implement panel struct with state
3. Add `ui()` method taking egui context
4. Add panel to `PlaygroundApp` in `app.rs`
5. Add tab enum variant and routing

### 2. Adding New Examples

To add a new example:

1. Create new file in `wgpu_playground_examples/examples/`
2. Implement using wgpu APIs
3. Add to example gallery in `examples.rs`
4. Document in README.md

### 3. Supporting New Backends

The adapter system supports any wgpu backend:

- Vulkan, Metal, DirectX 12, OpenGL are built-in
- WebGPU backend for WASM targets
- Backend selection via environment variable

### 4. Alternative WebGPU Implementations

The implementation layer is abstracted:

- Currently supports wgpu and Dawn
- Can add more via `implementation.rs`
- Feature flags for compile-time selection
- Runtime selection via environment variable

## Testing Strategy

### Unit Tests

- Located in each module (`#[cfg(test)]`)
- Test individual functions and structs
- Mock GPU resources where appropriate

### Integration Tests

- Located in `tests/` directory
- Test complete workflows
- Validate API interactions

### Visual Regression Tests

- Located in `wgpu_playground_core/src/visual_regression/`
- Render to textures and compare with references
- Automated in CI pipeline

### Example Tests

- Each example has associated tests
- Validate that examples run without errors
- Skip GPU-requiring tests in CI without hardware

## Performance Considerations

### Efficient Rendering

- **Immediate mode GUI**: egui minimizes retained state
- **Command batching**: Group GPU operations when possible
- **Resource caching**: Reuse pipelines and bind groups
- **Lazy initialization**: Create resources on demand

### Memory Management

- **Explicit buffer management**: Users control buffer lifecycle
- **Texture cleanup**: Automatic via Drop trait
- **Resource tracking**: Console shows active resources

### Async GPU Operations

- **Non-blocking submission**: Commands execute asynchronously
- **Queue management**: Proper synchronization for readback
- **Error handling**: Async errors captured via callbacks

## Future Architecture Improvements

### Planned Enhancements

1. **State Management**: Consider a more formal state management pattern
2. **Plugin System**: Allow third-party extensions
3. **Script Integration**: Lua/Python bindings for automation
4. **Network Protocol**: Remote GPU access for testing
5. **Recording/Playback**: Capture and replay command sequences

### Scalability

The architecture is designed to scale with additional features:

- **Modular panels**: Easy to add new functionality
- **Workspace structure**: Supports additional crates
- **Asset pipeline**: Can be extended for more formats
- **Backend abstraction**: Supports new GPU APIs

## Conclusion

The wgpu_playground architecture provides a solid foundation for WebGPU experimentation while maintaining:

- **Clarity**: Clean separation of concerns
- **Extensibility**: Easy to add new features
- **Performance**: Efficient GPU resource usage
- **Educational value**: Code serves as learning material
- **Maintainability**: Well-organized, documented codebase

The design prioritizes developer experience and learning, making it an effective tool for both exploring WebGPU capabilities and understanding GPU programming concepts.
