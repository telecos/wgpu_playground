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
  Shows placeholder text describing planned rendering features:
  - Render Pipelines
  - Buffers & Vertex Data
  - Textures & Sampling
  - Render Passes
  - Advanced Rendering

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
7. **Tabbed interface** for organizing features
8. **Placeholder panels** for future rendering and compute features

## Next Steps

Refer to PLAN.md for the detailed implementation roadmap. The next logical steps would be:
1. Implement Issue 1: Basic Triangle Rendering
2. Implement Issue 2: Vertex Buffer Management
3. Continue through the phases as outlined in PLAN.md
