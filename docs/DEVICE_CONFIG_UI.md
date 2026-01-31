# Device Configuration UI

This document describes the new Device Configuration UI panel added to the WebGPU Playground.

## Overview

The Device Configuration tab provides an interactive interface for exploring and configuring WebGPU device features and limits. This panel displays what features and limits are available on your GPU adapter and allows you to experiment with different configurations.

## Location

The Device Configuration panel can be accessed from the navigation sidebar:
- **Tab Name**: "🔧 Device Config"
- **Position**: Between "Adapter Selection" and "Device Info" tabs

## UI Components

### 1. Information Banner

At the top of the panel, an informational message explains the current functionality:
```
ℹ️ Note: This panel shows available features and limits. In the current version,
the device is created at startup with default settings. This UI can be used to
explore what features and limits your adapter supports.
```

### 2. Available Features Section

#### Layout
- Grid layout with 2 columns
- Striped rows for better readability
- Each row contains:
  - Checkbox with feature name
  - Support status indicator

#### Features Displayed
The following WebGPU features are shown:
- Depth Clip Control
- Depth32Float Stencil8
- Timestamp Query
- Pipeline Statistics Query
- Texture Compression BC
- Texture Compression ETC2
- Texture Compression ASTC
- Indirect First Instance
- Shader F16
- RG11B10UFloat Renderable
- BGRA8UnormStorage
- Float32 Filterable
- Shader Primitive Index

#### Behavior
- **Supported features**: Checkbox is enabled, shows "✓ Supported"
- **Unsupported features**: Checkbox is disabled/grayed out, shows "✗ Not supported"
- Clicking an enabled checkbox toggles the feature on/off

### 3. Device Limits Section

#### Layout
- Grid layout with 2 columns
- Striped rows for better readability
- Each row contains:
  - Limit name (left column)
  - Editable drag value (right column)

#### Limits Displayed

**Texture Limits:**
- Max Texture Dimension 1D
- Max Texture Dimension 2D
- Max Texture Dimension 3D
- Max Texture Array Layers

**Bind Group Limits:**
- Max Bind Groups
- Max Bindings Per Bind Group

**Buffer Limits:**
- Max Uniform Buffer Binding Size
- Max Storage Buffer Binding Size
- Max Buffer Size

**Vertex Limits:**
- Max Vertex Buffers
- Max Vertex Attributes

**Compute Limits:**
- Max Compute Workgroup Size X
- Max Compute Workgroup Size Y
- Max Compute Workgroup Size Z
- Max Compute Invocations Per Workgroup
- Max Compute Workgroup Storage Size
- Max Compute Workgroups Per Dimension

#### Behavior
- **Drag Values**: Click and drag left/right to adjust values
- **Value Validation**: All values are automatically clamped between 1 and the adapter's maximum capability
- **Tip Message**: "💡 Tip: Adjust limits as needed. Values are clamped to adapter maximum."

### 4. Action Buttons

Two buttons are provided at the bottom of the limits section:

1. **Reset to Default**
   - Resets all limits to WebGPU default values
   - Useful for starting fresh with standard configuration

2. **Use Maximum Available**
   - Sets all limits to the maximum values supported by your adapter
   - Useful for exploring full GPU capabilities

## Visual Design

- **Heading**: Large, clear section headings with emojis
- **Grid Spacing**: 10px horizontal, 4px vertical
- **Striped Rows**: Alternating background colors for easier reading
- **Color Coding**: 
  - Yellow/orange info banner for the note
  - Green checkmarks for supported features
  - Red X marks for unsupported features
- **Scrollable Content**: Vertical scroll area to accommodate all features and limits

## Use Cases

1. **Educational**: Learn what features your GPU supports
2. **Exploration**: See the range of limits available on your hardware
3. **Planning**: Test different configurations to understand compatibility
4. **Comparison**: Compare capabilities across different adapters

## Future Enhancements

In future versions, this panel could be enhanced to:
- Allow device recreation with the configured features and limits
- Save/load configuration presets
- Export configuration as code snippets
- Provide detailed descriptions for each feature and limit
- Show recommended values for different use cases
