# GPU Debugging Utilities

This document describes the GPU debugging utilities available in the WGPU Playground.

## Overview

The playground provides three debugging tools accessible from the **Tools & Debugging** section:

1. **Buffer Inspector** - View GPU buffer contents
2. **Texture Inspector** - Visualize GPU textures  
3. **Pipeline Debugger** - Debug render and compute pipelines

## Buffer Inspector

The Buffer Inspector allows you to view the raw contents of GPU buffers in multiple formats.

### Features

- **Multiple Display Formats:**
  - **Hex**: Hexadecimal byte view with ASCII representation
  - **Int32**: View data as signed 32-bit integers
  - **UInt32**: View data as unsigned 32-bit integers
  - **Float32**: View data as 32-bit floating-point numbers

- **Hex View Format:**
  ```
  00000000:  48 65 6c 6c 6f 20 57 6f  72 6c 64 00             |Hello World.|
  ```

- **Numeric View Format:**
  ```
  00000000:         42
  00000004:       1000
  00000008:         -1
  ```

### Usage

1. Navigate to **Tools & Debugging → Buffer Inspector**
2. Select a buffer from the Resource Inspector (future enhancement)
3. Choose your preferred display format
4. View the buffer contents with offset addresses

### Technical Details

- Displays up to 4KB of data by default
- Shows 16 bytes per row in hex mode
- Automatically handles different data alignments
- Error reporting for failed buffer reads

## Texture Inspector

The Texture Inspector enables visualization of GPU textures.

### Features

- **Texture Visualization:**
  - Displays textures as images in the UI
  - Real-time rendering of texture data
  - Supports multiple texture formats (RGBA8, BGRA8, etc.)

- **Zoom Controls:**
  - Zoom range: 0.1x to 10x
  - Quick zoom in/out buttons
  - Current zoom level display

- **Display Options:**
  - Toggle alpha channel visibility
  - Mip level selection (for mipmapped textures)
  - Array layer selection (for texture arrays)

- **Format Support:**
  - RGBA8Unorm / RGBA8UnormSrgb
  - BGRA8Unorm / BGRA8UnormSrgb
  - Fallback checker pattern for unsupported formats

### Usage

1. Navigate to **Tools & Debugging → Texture Inspector**
2. Select a texture from the Resource Inspector (future enhancement)
3. Use zoom controls to adjust view
4. Toggle "Show Alpha" to visualize transparency
5. Select mip level or array layer if applicable

### Technical Details

- Maximum display size: 500px viewport with scrolling
- Color space handling for sRGB formats
- Efficient image conversion from GPU data
- Handles special cases (multisampled, 3D textures)

## Pipeline Debugger

The Pipeline Debugger helps debug render and compute pipelines.

### Features

- **Pipeline Configuration Display:**
  - Pipeline label
  - Primitive topology (for render pipelines)
  - Color target count
  - Depth/stencil state
  - Blending state
  - Sample count

- **Shader Source Viewing:**
  - Tabbed interface for multiple shaders
  - Syntax highlighting
  - Shows vertex, fragment, and compute shaders
  - Displays entry point names

- **Validation Messages:**
  - Categorized by severity (Error, Warning, Info)
  - Color-coded icons (❌ ⚠️ ℹ️)
  - Message count summary
  - Toggle to show/hide informational messages
  - Scrollable message list

### Usage

1. Navigate to **Tools & Debugging → Pipeline Debugger**
2. Select a pipeline from the Resource Inspector (future enhancement)
3. View pipeline configuration in the Configuration section
4. Switch between shader tabs to view source code
5. Review validation messages for errors and warnings

### Validation Severity Levels

- **Error** (Red ❌): Critical issues that prevent pipeline execution
- **Warning** (Yellow ⚠️): Potential issues that may cause unexpected behavior
- **Info** (Blue ℹ️): Informational messages and suggestions

### Technical Details

- Read-only shader source display
- Maintains shader selection state
- Efficient validation message filtering
- Handles pipelines with/without fragment shaders

## Integration

All three debugging utilities are integrated into the main GUI:

1. **Access Point**: Tools & Debugging section in the sidebar
2. **Tab Navigation**: Click on any inspector to switch views
3. **Resource Selection**: (Future) Click on resources in Resource Inspector to load into debugging tools

## Future Enhancements

Planned improvements for the debugging utilities:

1. **Buffer Inspector:**
   - Direct GPU buffer reading via mapping
   - Custom data type interpretation
   - Diff view between buffer states
   - Export buffer contents to file

2. **Texture Inspector:**
   - Histogram display
   - Pixel value inspection on hover
   - Comparison view for multiple textures
   - Export texture as image file

3. **Pipeline Debugger:**
   - Shader compilation error highlighting
   - Performance metrics integration
   - Pipeline state history
   - Breakpoint support (if available)

4. **Integration:**
   - Automatic resource binding from Resource Inspector
   - Real-time updates as resources change
   - Search and filter capabilities
   - Bookmarking frequently inspected resources

## Examples

### Inspecting Vertex Buffer Data

```rust
// Create a buffer inspector
let mut inspector = BufferInspector::new();

// Load buffer data (e.g., vertex positions)
let vertex_data = vec![
    0.0f32, 0.5f32, 0.0f32,  // Vertex 1
    -0.5f32, -0.5f32, 0.0f32, // Vertex 2
    0.5f32, -0.5f32, 0.0f32,  // Vertex 3
];
inspector.load_data(bytemuck::cast_slice(&vertex_data).to_vec());
inspector.set_format(DataFormat::Float32);

// View in UI
inspector.ui(&mut ui);
```

### Visualizing a Texture

```rust
// Create a texture inspector
let mut inspector = TextureInspector::new();

// Load texture data
let texture_data = TextureData {
    width: 256,
    height: 256,
    format: TextureFormat::Rgba8Unorm,
    data: /* GPU texture data */,
};
inspector.load_texture(texture_data);
inspector.set_zoom(2.0);

// View in UI
inspector.ui(&mut ui);
```

### Debugging a Pipeline

```rust
// Create a pipeline debugger
let mut debugger = PipelineDebugger::new();

// Load pipeline info
let debug_info = PipelineDebugInfo {
    config: PipelineConfig {
        label: Some("Main Render Pipeline".to_string()),
        topology: Some("TriangleList".to_string()),
        has_depth_stencil: true,
        color_target_count: 1,
        has_blending: false,
        sample_count: 1,
    },
    shaders: vec![/* shader info */],
    validation_messages: vec![/* validation messages */],
};
debugger.load_pipeline(debug_info);

// View in UI
debugger.ui(&mut ui);
```

## Troubleshooting

### Buffer Inspector Shows "No data loaded"

- Ensure a buffer is selected from the Resource Inspector
- Check that the buffer has MAP_READ usage flag
- Verify the buffer is not currently in use by GPU

### Texture Inspector Shows Checker Pattern

- The texture format may not be supported yet
- Check if the texture data was successfully read from GPU
- Verify texture usage includes COPY_SRC flag

### Pipeline Debugger Shows No Validation Messages

- This is normal for valid pipelines
- Validation messages only appear when there are issues
- Try loading a different pipeline to see messages

## Performance Considerations

- **Buffer Inspector**: Reading large buffers may take time; data is limited to 4KB by default
- **Texture Inspector**: Large textures are rendered at full resolution; use zoom to view details
- **Pipeline Debugger**: Shader source display is efficient for typical shader sizes

## Security

All debugging utilities operate in read-only mode:

- No modification of GPU resources
- No execution of arbitrary code
- Safe memory access patterns
- Input validation for all data formats
