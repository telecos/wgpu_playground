# Device Configuration UI - Visual Layout

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ 🎮 WebGPU Playground                                                        │
├──────────────────┬──────────────────────────────────────────────────────────┤
│ Navigation       │ ⚙️ Device Configuration                                  │
│                  │                                                           │
│ ⚙️ Adapter        │ Configure features and limits for device creation.      │
│   Selection      │                                                           │
│                  │ ⚠️  Note: This panel shows available features and limits │
│ 🔧 Device Config │ In the current version, the device is created at startup │
│                  │ with default settings. This UI can be used to explore    │
│ 📊 Device Info   │ what features and limits your adapter supports.          │
│                  │                                                           │
│ 🎨 Rendering     │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ │
│                  │ Available Features                                        │
│ 🧮 Compute/ML    │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ │
│                  │ Enable or disable WebGPU features:                        │
│                  │                                                           │
│                  │ ┌────────────────────────────────────────────────────┐   │
│                  │ │ Feature                        │ Support Status    │   │
│                  │ ├────────────────────────────────┼──────────────────┤   │
│                  │ │ ☑ Depth Clip Control           │ ✓ Supported      │   │
│                  │ │ ☑ Depth32Float Stencil8        │ ✓ Supported      │   │
│                  │ │ ☑ Timestamp Query              │ ✓ Supported      │   │
│                  │ │ ☐ Pipeline Statistics Query    │ ✗ Not supported  │   │
│                  │ │ ☑ Texture Compression BC       │ ✓ Supported      │   │
│                  │ │ ☐ Texture Compression ETC2     │ ✗ Not supported  │   │
│                  │ │ ☐ Texture Compression ASTC     │ ✗ Not supported  │   │
│                  │ │ ☑ Indirect First Instance      │ ✓ Supported      │   │
│                  │ │ ☐ Shader F16                   │ ✗ Not supported  │   │
│                  │ │ ☑ RG11B10UFloat Renderable     │ ✓ Supported      │   │
│                  │ │ ☑ BGRA8UnormStorage            │ ✓ Supported      │   │
│                  │ │ ☑ Float32 Filterable           │ ✓ Supported      │   │
│                  │ │ ☑ Shader Primitive Index       │ ✓ Supported      │   │
│                  │ └────────────────────────────────────────────────────┘   │
│                  │                                                           │
│                  │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ │
│                  │ Device Limits                                             │
│                  │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ │
│                  │ Adjust device limits (values are clamped to adapter max):│
│                  │                                                           │
│                  │ 💡 Tip: Adjust limits as needed. Values are clamped to   │
│                  │ adapter maximum.                                          │
│                  │                                                           │
│                  │ ┌────────────────────────────────────────────────────┐   │
│                  │ │ Limit                          │ Value             │   │
│                  │ ├────────────────────────────────┼──────────────────┤   │
│                  │ │ Max Texture Dimension 1D       │ [8192        ▼]  │   │
│                  │ │ Max Texture Dimension 2D       │ [8192        ▼]  │   │
│                  │ │ Max Texture Dimension 3D       │ [2048        ▼]  │   │
│                  │ │ Max Texture Array Layers       │ [256         ▼]  │   │
│                  │ │ Max Bind Groups                │ [4           ▼]  │   │
│                  │ │ Max Bindings Per Bind Group    │ [1000        ▼]  │   │
│                  │ │ Max Uniform Buffer Binding Size│ [65536       ▼]  │   │
│                  │ │ Max Storage Buffer Binding Size│ [134217728   ▼]  │   │
│                  │ │ Max Buffer Size                │ [268435456   ▼]  │   │
│                  │ │ Max Vertex Buffers             │ [8           ▼]  │   │
│                  │ │ Max Vertex Attributes          │ [16          ▼]  │   │
│                  │ │ Max Compute Workgroup Size X   │ [256         ▼]  │   │
│                  │ │ Max Compute Workgroup Size Y   │ [256         ▼]  │   │
│                  │ │ Max Compute Workgroup Size Z   │ [64          ▼]  │   │
│                  │ │ Max Compute Invocations/WG     │ [256         ▼]  │   │
│                  │ │ Max Compute Workgroup Storage  │ [16384       ▼]  │   │
│                  │ │ Max Compute Workgroups/Dim     │ [65535       ▼]  │   │
│                  │ └────────────────────────────────────────────────────┘   │
│                  │                                                           │
│                  │ [ Reset to Default ] [ Use Maximum Available ]            │
│                  │                                                           │
└──────────────────┴──────────────────────────────────────────────────────────┘

Legend:
  ☑ = Enabled checkbox (supported feature)
  ☐ = Disabled/grayed checkbox (unsupported feature)
  [Value ▼] = Drag value control (click and drag to adjust)
  ━━━ = Section separator
```

## Interaction Guide

### Features Section
- **Supported features** (with ✓):
  - Checkboxes are enabled and clickable
  - Click to toggle feature on/off
  - Checked = feature will be requested when device is created
  
- **Unsupported features** (with ✗):
  - Checkboxes are disabled/grayed out
  - Cannot be toggled
  - Adapter does not support this feature

### Limits Section
- **Drag Values**:
  - Click on value and drag left/right to adjust
  - Or click to type a new value directly
  - Values automatically clamped between 1 and adapter maximum
  
- **Action Buttons**:
  - **Reset to Default**: Sets all limits to WebGPU defaults
  - **Use Maximum Available**: Sets all limits to adapter's maximum capabilities

### Visual Design
- Striped grid rows for easier reading
- Clear section headers with emojis
- Color-coded status indicators
- Professional, clean layout
- Scrollable content area for all features/limits
