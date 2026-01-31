# Sampler Configuration Panel

The Sampler Configuration Panel is a UI interface for creating and configuring GPU samplers with full control over texture sampling parameters.

## Overview

The panel provides an intuitive interface for configuring all aspects of texture sampling, including:

- **Address Modes**: Control how texture coordinates outside the [0, 1] range are handled
- **Filter Modes**: Configure magnification, minification, and mipmap filtering
- **LOD Clamping**: Set minimum and maximum level-of-detail values
- **Comparison Function**: Optional depth/stencil comparison for shadow mapping
- **Anisotropic Filtering**: Improve texture quality at oblique angles
- **Border Color**: Color used when address mode is ClampToBorder

## UI Layout

### Sampler Properties Section
- **Label**: Text field for optional sampler label/name

### Address Modes Section
Three dropdown selectors for controlling texture wrapping:
- **U (horizontal)**: How to handle horizontal coordinates outside [0, 1]
- **V (vertical)**: How to handle vertical coordinates outside [0, 1]
- **W (depth)**: How to handle depth coordinates outside [0, 1]

Each dropdown offers:
- `ClampToEdge`: Clamp coordinates to edge
- `Repeat`: Wrap coordinates, creating repeating pattern
- `MirrorRepeat`: Mirror coordinates back and forth
- `ClampToBorder`: Use border color outside [0, 1]

### Filter Modes Section
Three dropdown selectors for texture filtering:
- **Magnification (zoom in)**: Filter when pixel is smaller than texel
  - `Nearest`: Use nearest texel value
  - `Linear`: Interpolate between texels
- **Minification (zoom out)**: Filter when pixel is larger than texel
  - `Nearest`: Use nearest texel value
  - `Linear`: Interpolate between texels
- **Mipmap**: Filter between mipmap levels
  - `Nearest`: Use nearest mipmap level
  - `Linear`: Interpolate between mipmap levels

### LOD Clamping Section
Two numeric input fields:
- **Min LOD**: Minimum level of detail (default: 0.0)
- **Max LOD**: Maximum level of detail (default: 32.0)

### Anisotropic Filtering Section
Slider control (1-16):
- Value of 1 disables anisotropic filtering
- Higher values (up to 16) improve texture quality at oblique angles
- Uses more GPU resources at higher values

### Comparison Function Section
Optional depth/stencil comparison for shadow mapping:
- **Enable comparison**: Checkbox to enable/disable
- **Function**: Dropdown with comparison options when enabled:
  - `Never`: Never pass
  - `Less`: Pass if new value < existing value
  - `Equal`: Pass if new value == existing value
  - `LessEqual`: Pass if new value <= existing value
  - `Greater`: Pass if new value > existing value
  - `NotEqual`: Pass if new value != existing value
  - `GreaterEqual`: Pass if new value >= existing value
  - `Always`: Always pass

### Border Color Section
Optional color for ClampToBorder address mode:
- **Enable border color**: Checkbox to enable/disable
- **Color**: Dropdown with options when enabled:
  - `Transparent Black`: RGBA(0, 0, 0, 0)
  - `Opaque Black`: RGBA(0, 0, 0, 1)
  - `Opaque White`: RGBA(1, 1, 1, 1)
  - `Zero`: All zeros

**Note**: The UI displays a warning when ClampToBorder is selected but border color is not enabled.

### Action Buttons
- **🔍 Validate**: Checks configuration for errors
- **✨ Create Sampler**: Creates a GPU sampler with the current configuration
- **🔄 Reset**: Resets all settings to default values

### Messages Area
Displays validation errors in red or success messages in green.

### Configuration Summary
Real-time summary showing all current settings:
- Label
- Address modes (U, V, W)
- Filter modes (mag, min, mipmap)
- LOD clamp range
- Anisotropy level
- Compare function (if enabled)
- Border color (if enabled)

## Default Configuration

The panel initializes with WebGPU default values:
- All address modes: `ClampToEdge`
- All filter modes: `Nearest`
- LOD range: 0.0 to 32.0
- Anisotropy: 1 (disabled)
- Compare function: Disabled
- Border color: Disabled

## Validation Rules

The panel validates the configuration and displays errors for:
1. **Invalid LOD range**: Min LOD must be ≤ Max LOD
2. **Invalid anisotropy**: Must be between 1 and 16
3. **Missing border color**: Border color must be specified when using ClampToBorder address mode

## Common Use Cases

### Texture Repeat Sampler (for tiling)
```
Address modes: All Repeat
Mag filter: Linear
Min filter: Linear
Mipmap filter: Linear
Anisotropy: 16
```

### UI Texture Sampler (clamped, linear)
```
Address modes: All ClampToEdge
Mag filter: Linear
Min filter: Linear
Mipmap filter: Nearest
```

### Pixel Art Sampler (nearest, no mipmaps)
```
Address modes: All ClampToEdge
Mag filter: Nearest
Min filter: Nearest
Mipmap filter: Nearest
Anisotropy: 1
```

### Shadow Map Sampler
```
Address modes: All ClampToEdge
Mag filter: Linear
Min filter: Linear
Compare function: Enabled, LessEqual
```

## Implementation Details

The panel is implemented in `crates/wgpu_playground_core/src/sampler_panel.rs` and provides:

- **Full UI integration** with the egui framework
- **Comprehensive validation** with helpful error messages
- **Complete test coverage** with 20+ unit tests
- **Type-safe API** using strongly-typed enums from the sampler module
- **Real-time updates** to the configuration summary

## Testing

The module includes extensive unit tests covering:
- Panel creation and defaults
- Descriptor updates from UI state
- Validation logic for all edge cases
- Border color and compare function toggling
- Invalid input handling

Run tests with:
```bash
cargo test --package wgpu_playground_core sampler_panel
```
