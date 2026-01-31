# WebGPU Playground UI Mockup

Since the application requires a display to run, here's a textual description of the user interface:

```
┌─────────────────────────────────────────────────────────────────────┐
│ 🎮 WebGPU Playground                                                │
├─────────────────────────────────────────────────────────────────────┤
│ [⚙️ Adapter Selection] [📊 Device Info] [🎨 Rendering] [📐 Buffer Config] [🧮 Compute/ML] │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│ When "Adapter Selection" tab is selected:                           │
│                                                                      │
│  🎮 GPU Adapter Selection                                           │
│  ─────────────────────────                                          │
│  Backend Filter                                                      │
│  [All] [Primary] [Vulkan] [Metal] [DX12] [OpenGL]                  │
│                                                                      │
│  Power Preference                                                    │
│  [None] [Low Power] [High Performance]                              │
│                                                                      │
│  Available Adapters                                                  │
│  Found 2 adapter(s)                                                  │
│                                                                      │
│  ┌─────────────────────────────────────────────┐                   │
│  │ ● NVIDIA GeForce RTX 3080                   │ (selected)         │
│  │   Backend: Vulkan                            │                   │
│  │   Device Type: DiscreteGpu                   │                   │
│  │   Vendor ID: 0x10DE                          │                   │
│  │   Device ID: 0x2206                          │                   │
│  │   Driver: NVIDIA 525.60.11                   │                   │
│  └─────────────────────────────────────────────┘                   │
│                                                                      │
│  ┌─────────────────────────────────────────────┐                   │
│  │ Intel(R) UHD Graphics 630                    │                   │
│  └─────────────────────────────────────────────┘                   │
│                                                                      │
│  ℹ️ Information                                                     │
│  ⚠️ Note: Changing the adapter requires restarting the application.│
│  Set the WGPU_BACKEND environment variable and restart:             │
│  WGPU_BACKEND=vulkan cargo run --release                            │
│                                                                      │
│ When "Device Info" tab is selected:                                 │
│                                                                      │
│  Adapter Information                                                 │
│  ─────────────────                                                   │
│  Name: [GPU Name]                                                    │
│  Vendor: [Vendor ID]                                                 │
│  Device: [Device ID]                                                 │
│  Device Type: DiscreteGpu/IntegratedGpu/VirtualGpu/Cpu              │
│  Driver: [Driver Name]                                               │
│  Driver Info: [Driver Version]                                       │
│  Backend: Vulkan/Metal/Dx12/Gl                                       │
│                                                                      │
│  Device Limits                                                       │
│  ─────────────                                                       │
│  Max Texture Dimension 1D: [value]                                   │
│  Max Texture Dimension 2D: [value]                                   │
│  Max Texture Dimension 3D: [value]                                   │
│  Max Texture Array Layers: [value]                                   │
│  ... (all other limits listed)                                       │
│                                                                      │
│  Device Features                                                     │
│  ───────────────                                                     │
│  Features: [Detailed feature flags]                                  │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘

When "Rendering" tab is selected:
  Shows two sub-tabs: "📚 Example Gallery" and "📝 Shader Editor"

  Example Gallery Sub-tab:
  ┌─────────────────────────────────────────────────────────────────────┐
  │ 🎨 Example Gallery                                                  │
  │ Browse and explore WebGPU examples with descriptions and source code│
  │                                                                      │
  │ Filter by category:                                                  │
  │ [All] [Rendering] [Compute]                                         │
  │                                                                      │
  │ Found 4 example(s):                                                  │
  │                                                                      │
  │ ┌─────────────────────────────────────────────┐                    │
  │ │ ● 🎨 Basic Triangle (Rendering)              │ (selected)         │
  │ │   Description: Renders a simple colored...   │                   │
  │ │   [Hide Source Code]                         │                   │
  │ │   Source Code:                                │                   │
  │ │   // Triangle Rendering Example              │                   │
  │ │   struct VertexInput { ... }                 │                   │
  │ │   ...                                         │                   │
  │ │   [📋 Copy Source Code]                       │                   │
  │ └─────────────────────────────────────────────┘                    │
  │                                                                      │
  │ ○ 🎨 Rotating Cube (Rendering)                                      │
  │ ○ 🎨 Texture Mapping (Rendering)                                    │
  │ ○ 🧮 Compute Shader (Compute)                                       │
  └─────────────────────────────────────────────────────────────────────┘

  Shader Editor Sub-tab:
  ┌─────────────────────────────────────────────────────────────────────┐
  │ 📝 WGSL Shader Editor                                                │
  │ ─────────────────────                                                │
  │ Label: [shader_editor]  File: [example.wgsl]                        │
  │ [📁 Load] [📚 Load Example] [⚙️ Compile] [🔄 Reset]                 │
  │                                                                      │
  │ ℹ️ Not compiled yet. Click 'Compile' to validate your shader.       │
  │                                                                      │
  │ 💡 Tips:                                                             │
  │ • Use '@vertex' and '@fragment' for render shaders                   │
  │ • Use '@compute' for compute shaders                                 │
  │ • Press Compile to validate syntax                                   │
  │                                                                      │
  │ Shader Code:                                                         │
  │ ┌────────────────────────────────────────────────────────────┐     │
  │ │ 1  │ // WGSL Shader Example                                │     │
  │ │ 2  │ @vertex                                                │     │
  │ │ 3  │ fn vs_main(@builtin(vertex_index) vertex_index: u32)  │     │
  │ │ 4  │            -> @builtin(position) vec4<f32> {           │     │
  │ │ 5  │     var positions = array<vec2<f32>, 3>(               │     │
  │ │ 6  │         vec2<f32>(0.0, 0.5),                           │     │
  │ │ 7  │         vec2<f32>(-0.5, -0.5),                         │     │
  │ │ 8  │         vec2<f32>(0.5, -0.5)                           │     │
  │ │ 9  │     );                                                  │     │
  │ │ 10 │     let pos = positions[vertex_index];                 │     │
  │ │ 11 │     return vec4<f32>(pos, 0.0, 1.0);                   │     │
  │ │ 12 │ }                                                       │     │
  │ │ 13 │                                                         │     │
  │ │ 14 │ @fragment                                               │     │
  │ │ 15 │ fn fs_main() -> @location(0) vec4<f32> {               │     │
  │ │ 16 │     return vec4<f32>(1.0, 0.5, 0.0, 1.0);             │     │
  │ │ 17 │ }                                                       │     │
  │ └────────────────────────────────────────────────────────────┘     │
  │                                                                      │
  │ [✓] Show line numbers                                               │
  └─────────────────────────────────────────────────────────────────────┘


When "Buffer Config" tab is selected:
  Shows buffer configuration interface:
  
  ┌─────────────────────────────────────────────────────────────────┐
  │ 📐 Buffer Configuration                                         │
  │ Configure and create GPU buffers with custom parameters.        │
  │                                                                  │
  │ Buffer Properties                                                │
  │ ──────────────────                                               │
  │ Label:    [text input field]                                     │
  │ Size (bytes): [256]                                              │
  │                                                                  │
  │ Usage Flags                                                      │
  │ ────────────                                                     │
  │ Select how the buffer will be used (multiple flags can be        │
  │ selected):                                                        │
  │                                                                  │
  │ [ ] VERTEX         Buffer can be used as a vertex buffer        │
  │ [ ] INDEX          Buffer can be used as an index buffer        │
  │ [ ] UNIFORM        Buffer can be used as a uniform buffer       │
  │ [ ] STORAGE        Buffer can be used as a storage buffer       │
  │ [ ] INDIRECT       Buffer can be used for indirect draw commands│
  │ [ ] COPY_SRC       Buffer can be used as a copy source          │
  │ [✓] COPY_DST       Buffer can be used as a copy destination     │
  │ [ ] MAP_READ       Buffer can be mapped for reading             │
  │ [ ] MAP_WRITE      Buffer can be mapped for writing             │
  │ [ ] QUERY_RESOLVE  Buffer can be used to resolve query results  │
  │                                                                  │
  │ 💡 Note: MAP_READ and MAP_WRITE cannot be used together         │
  │                                                                  │
  │ Additional Options                                               │
  │ ───────────────────                                              │
  │ [ ] Mapped at creation                                           │
  │     Whether the buffer should be mapped immediately after        │
  │     creation                                                     │
  │                                                                  │
  │ [🔍 Validate] [✨ Create Buffer] [🔄 Reset]                      │
  │                                                                  │
  │ Configuration Summary                                            │
  │ ──────────────────────                                           │
  │ Label: <none>                                                    │
  │ Size: 256 bytes                                                  │
  │ Mapped at creation: false                                        │
  │                                                                  │
  │ Usage flags:                                                     │
  │   • COPY_DST                                                     │
  │                                                                  │
  └─────────────────────────────────────────────────────────────────┘

When "Compute/ML" tab is selected:
  Shows placeholder text describing planned compute features:
  - Compute Pipelines
  - Storage Buffers
  - Compute Operations
  - ML Inferencing Use Cases
  - Example Workloads
  - Advanced Compute
```

## Current State

The application currently provides:
1. **Functional window and UI framework** using egui
2. **WebGPU initialization** with wgpu
3. **Adapter selection panel** for choosing GPU adapters with detailed properties and power preferences
4. **Device information display** showing all GPU capabilities
5. **Device configuration panel** for setting device features and limits
6. **Buffer configuration panel** for creating GPU buffers with custom parameters:
   - Size configuration with validation
   - All buffer usage flags as checkboxes (VERTEX, INDEX, UNIFORM, STORAGE, INDIRECT, COPY_SRC, COPY_DST, MAP_READ, MAP_WRITE, QUERY_RESOLVE)
   - Label input for debugging
   - Mapped-at-creation option
   - Real-time validation with error messages
   - Configuration summary display
7. **Rendering panel** with two sub-tabs:
   - **Example Gallery**: Browse 4 WebGPU shader examples (triangle, cube, texture mapping, compute shader) with descriptions and source code viewing
   - **WGSL Shader Editor**: Interactive shader editor with:
     - Syntax highlighting (structure in place for future enhancement)
     - Line numbers display
     - File loading from assets/shaders directory
     - Inline editing
     - Shader compilation with wgpu
     - Error reporting
     - Example shader loading
8. **Tabbed interface** for organizing features
9. **Placeholder panels** for future compute features

## Next Steps

Refer to PLAN.md for the detailed implementation roadmap. The next logical steps would be:
1. Implement Issue 1: Basic Triangle Rendering
2. Implement Issue 2: Vertex Buffer Management
3. Continue through the phases as outlined in PLAN.md
