# Render Pass Configuration UI

## Overview

The Render Pass Configuration UI provides a comprehensive interface for configuring WebGPU render passes with color attachments, depth-stencil attachments, and timestamp writes. This panel is accessible through the "🎬 Render Pass" tab in the navigation sidebar.

## UI Layout

```
┌─────────────────────────────────────────────────────────────────┐
│ 🎬 Render Pass Configuration                                    │
│                                                                  │
│ Configure render pass with color attachments, depth-stencil,    │
│ and timestamp writes.                                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Render Pass Properties                                       │ │
│ │                                                              │ │
│ │ Label: [___________________]                                │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                  │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ 🎨 Color Attachment                                          │ │
│ │                                                              │ │
│ │ ☑ Enable color attachment                                   │ │
│ │                                                              │ │
│ │ Load Operation:       [Clear ▼]                             │ │
│ │ Clear Color:                                                 │ │
│ │   R: [====|=========] 0.00                                  │ │
│ │   G: [====|=========] 0.00                                  │ │
│ │   B: [====|=========] 0.00                                  │ │
│ │   A: [================|==] 1.00                             │ │
│ │   Preview: ████████                                         │ │
│ │ Store Operation:      [Store ▼]                             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                  │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ 📏 Depth-Stencil Attachment                                  │ │
│ │                                                              │ │
│ │ ☐ Enable depth-stencil attachment                          │ │
│ │                                                              │ │
│ │ (When enabled, shows:)                                       │ │
│ │   Depth Configuration:                                       │ │
│ │     Load Operation:    [Clear ▼]                            │ │
│ │     Clear Depth:       [================|==] 1.00           │ │
│ │     Store Operation:   [Store ▼]                            │ │
│ │                                                              │ │
│ │   Stencil Configuration:                                     │ │
│ │     Load Operation:    [Clear ▼]                            │ │
│ │     Clear Stencil:     [====|===============] 0             │ │
│ │     Store Operation:   [Store ▼]                            │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                  │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ ⏱️ Timestamp Writes                                          │ │
│ │                                                              │ │
│ │ ☐ Enable timestamp writes                                   │ │
│ │                                                              │ │
│ │ (When enabled, shows:)                                       │ │
│ │   Timestamp writes allow you to measure GPU performance by  │ │
│ │   writing timestamps to a query set.                        │ │
│ │                                                              │ │
│ │   Beginning Index: [0___]                                   │ │
│ │   End Index:       [1___]                                   │ │
│ │                                                              │ │
│ │   💡 Note: Requires a QuerySet of type Timestamp and the    │ │
│ │   TIMESTAMP_QUERY feature                                    │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                  │
│ [🔍 Validate]  [🔄 Reset]  [📋 Preset: Black Clear]           │
│ [📋 Preset: With Depth]                                        │
│                                                                  │
│ ✓ Configuration is valid                                        │
│                                                                  │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Configuration Summary                                        │ │
│ │                                                              │ │
│ │ Label: <none>                                                │ │
│ │                                                              │ │
│ │ Color Attachment:                                            │ │
│ │   Load: Clear                                                │ │
│ │   Clear Color: (0.00, 0.00, 0.00, 1.00)                     │ │
│ │   Store: Store                                               │ │
│ │                                                              │ │
│ │ Depth-Stencil Attachment:                                    │ │
│ │   (disabled)                                                 │ │
│ │                                                              │ │
│ │ Timestamp Writes:                                            │ │
│ │   (disabled)                                                 │ │
│ └─────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

## Features

### 1. Render Pass Properties

Configure basic render pass properties:
- **Label**: Optional label for debugging purposes

### 2. Color Attachment Configuration

Control how color output is handled in the render pass:

#### Enable/Disable
- Checkbox to enable or disable the color attachment
- At least one attachment (color or depth-stencil) must be enabled

#### Load Operation
Two options for what happens at the start of the render pass:
- **Clear**: Clear the attachment with a specified color
- **Load**: Load the existing contents of the attachment

#### Clear Color (when Load Operation is Clear)
Configure the RGBA color to clear the attachment to:
- **R (Red)**: Slider from 0.0 to 1.0
- **G (Green)**: Slider from 0.0 to 1.0
- **B (Blue)**: Slider from 0.0 to 1.0
- **A (Alpha)**: Slider from 0.0 to 1.0
- **Preview**: Visual preview of the configured color

#### Store Operation
Two options for what happens at the end of the render pass:
- **Store**: Store the contents of the attachment
- **Discard**: Discard the contents of the attachment

### 3. Depth-Stencil Attachment Configuration

Configure depth and stencil testing for the render pass:

#### Enable/Disable
- Checkbox to enable or disable the depth-stencil attachment
- Can be enabled independently of color attachment

#### Depth Configuration
- **Load Operation**: Clear or Load
- **Clear Depth**: Slider from 0.0 to 1.0 (when Clear is selected)
- **Store Operation**: Store or Discard

#### Stencil Configuration
- **Load Operation**: Clear or Load
- **Clear Stencil**: Slider from 0 to 255 (when Clear is selected)
- **Store Operation**: Store or Discard

### 4. Timestamp Writes Configuration

Enable GPU performance measurement:

#### Enable/Disable
- Checkbox to enable or disable timestamp writes
- Provides information about GPU performance

#### Configuration (when enabled)
- **Beginning Index**: Query set index for the beginning timestamp
- **End Index**: Query set index for the end timestamp

**Note**: Requires:
- A QuerySet of type Timestamp
- The TIMESTAMP_QUERY GPU feature enabled

### 5. Actions

Four action buttons are available:

#### Validate
- Validates the current configuration
- Shows success or error messages
- Checks:
  - At least one attachment is enabled
  - Clear color values are in valid range (0.0-1.0)
  - Clear depth value is in valid range (0.0-1.0)
  - Timestamp indices are valid numbers

#### Reset
- Resets all configuration to default values
- Enables color attachment with black clear color
- Disables depth-stencil and timestamp writes

#### Preset: Black Clear
- Sets up color attachment with black clear color
- Quick setup for common use case

#### Preset: With Depth
- Sets up both color and depth-stencil attachments
- Color: Clear to black
- Depth: Clear to 1.0

### 6. Configuration Summary

Real-time display of current configuration including:
- Label
- Color attachment settings (load/store ops, clear color)
- Depth-stencil attachment settings (or "(disabled)")
- Timestamp writes settings (or "(disabled)")

## Validation Rules

The panel validates configurations according to these rules:

1. **At least one attachment**: Either color or depth-stencil must be enabled
2. **Clear color range**: All RGBA values must be between 0.0 and 1.0
3. **Clear depth range**: Depth value must be between 0.0 and 1.0
4. **Timestamp indices**: Must be valid unsigned integers

## Usage

### Basic Color Rendering

1. Enable color attachment (enabled by default)
2. Select "Clear" for load operation
3. Set desired clear color using sliders
4. Click "Validate" to verify configuration

### Depth-Tested Rendering

1. Enable both color and depth-stencil attachments
2. Configure color attachment as desired
3. Set depth load operation to "Clear"
4. Set clear depth to 1.0
5. Click "Preset: With Depth" for quick setup

### Performance Measurement

1. Configure color and/or depth-stencil as needed
2. Enable timestamp writes
3. Set beginning and end indices
4. Ensure TIMESTAMP_QUERY feature is enabled on device
5. Create a QuerySet of type Timestamp before using

## Technical Implementation

The panel is implemented in `render_pass_panel.rs` and includes:

### Core Features
- Comprehensive state management for all render pass parameters
- Validation and error handling
- Success/error message display
- Integration with wgpu render pass descriptors
- Real-time configuration preview

### Testing
16 comprehensive unit tests covering:
- Panel creation and defaults
- Load/store operation choices
- Color clear value configuration
- Depth and stencil operations
- Validation for various scenarios:
  - No attachments (error)
  - Color only (valid)
  - Depth only (valid)
  - Invalid clear color values
  - Invalid depth values
  - Invalid timestamp indices
  - Valid timestamp configuration
- Store operation conversion

All tests pass successfully, ensuring reliable functionality.

## Code Examples

### Getting Color Load Operation

```rust
let panel = RenderPassPanel::new();
let load_op = panel.get_color_load_op(); // Returns LoadOp<Color>
```

### Getting Configured Clear Color

```rust
let panel = RenderPassPanel::new();
let color = panel.get_color_clear(); // Returns Color
```

### Validating Configuration

```rust
let mut panel = RenderPassPanel::new();
if panel.validate() {
    println!("Configuration is valid");
} else {
    println!("Error: {:?}", panel.validation_error);
}
```

## Integration

The render pass panel is integrated into the main application:
- Added to `PlaygroundApp` struct
- Accessible via "🎬 Render Pass" tab in navigation
- Follows the same pattern as other configuration panels

## Future Enhancements

Potential improvements for future versions:
- Multiple color attachments support
- Occlusion query set configuration
- Preset configurations for common scenarios
- Advanced stencil operations configuration
- Visual diagram of render pass flow
