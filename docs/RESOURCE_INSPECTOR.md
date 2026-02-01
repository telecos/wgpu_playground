# Resource Inspector Panel

The Resource Inspector Panel is a powerful debugging and monitoring tool for WGPU resources in the playground.

## Features

### Resource Tracking
The panel tracks all major GPU resources:
- **Buffers**: Vertex, index, uniform, storage buffers with size and usage information
- **Textures**: 1D, 2D, 3D textures with format, dimensions, and memory usage
- **Render Pipelines**: Vertex and fragment shader entry points
- **Compute Pipelines**: Compute shader entry points

### Resource Properties Displayed
For each resource, the panel shows:
- **Label**: Optional user-defined name
- **Type**: Buffer, Texture, Render Pipeline, or Compute Pipeline
- **State**: Active, In Use, or Destroyed
- **Memory Usage**: Calculated based on resource type and configuration
- **Detailed Properties**: Size, format, dimensions, usage flags, etc.

### Filtering and Search
- **Type Filter**: Show only Buffers, Textures, Pipelines, or All
- **Search**: Filter resources by label or type name (case-insensitive)
- **State Filter**: Toggle visibility of destroyed resources

### Memory Tracking
The panel calculates and displays:
- Total memory usage of all resources
- Memory usage of filtered resources
- Per-resource memory usage with human-readable formatting (B, KB, MB, GB)

## Usage

### Accessing the Panel
1. Launch the WGPU Playground GUI application
2. Navigate to "🔍 Resource Inspector" in the sidebar

### Loading Demo Data
Click the "📝 Load Demo Data" button to populate the panel with sample resources:
- 3 Buffers (Vertex, Index, Uniform)
- 2 Textures (Color, Depth)
- 2 Pipelines (Render, Compute)

### Filtering Resources
1. Select a resource type filter (All, Buffers, Textures, Pipelines)
2. Enter search text to filter by label/name
3. Toggle "Show destroyed resources" to include/exclude destroyed items

### Understanding Resource States
- **✓ Active**: Resource is created and ready to use
- **🔄 In Use**: Resource is currently being used in rendering/computation
- **❌ Destroyed**: Resource has been destroyed and is no longer valid

## Implementation Details

### Memory Calculation
- **Buffers**: Size in bytes as specified
- **Textures**: width × height × depth × bytes_per_pixel × sample_count × mip_levels
- **Pipelines**: Fixed overhead (1KB estimate for shader state)

### Usage Flags Displayed
#### Buffer Usage:
- VERTEX, INDEX, UNIFORM, STORAGE
- INDIRECT, COPY_SRC, COPY_DST
- MAP_READ, MAP_WRITE, QUERY_RESOLVE

#### Texture Usage:
- COPY_SRC, COPY_DST
- TEXTURE_BINDING, STORAGE_BINDING
- RENDER_ATTACHMENT

## Code Example

```rust
use wgpu_playground_core::resource_inspector::{
    BufferInfo, ResourceInspectorPanel, ResourceState,
};
use wgpu_playground_core::buffer::BufferUsages;

// Create the panel
let mut inspector = ResourceInspectorPanel::new();

// Track a buffer
inspector.add_buffer(BufferInfo {
    label: Some("Vertex Buffer".to_string()),
    size: 4096,
    usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
    mapped_at_creation: false,
    state: ResourceState::Active,
});

// Track a texture
inspector.add_texture(TextureInfo {
    label: Some("Color Texture".to_string()),
    width: 1024,
    height: 1024,
    depth_or_array_layers: 1,
    dimension: TextureDimension::D2,
    format: TextureFormat::Rgba8Unorm,
    mip_level_count: 1,
    sample_count: 1,
    usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
    state: ResourceState::Active,
});

// Get resource count
println!("Total resources: {}", inspector.resource_count());
```

## Testing

The resource inspector includes comprehensive tests:
- 11 unit tests in `resource_inspector.rs`
- 16 integration tests in `resource_inspector_integration_test.rs`

Run tests with:
```bash
cargo test --package wgpu_playground_core resource_inspector
```

## Future Enhancements

Potential improvements:
- Real-time resource creation/destruction tracking
- Resource dependency graph visualization
- Memory usage trends over time
- Export resource list to CSV/JSON
- Resource validation and leak detection
