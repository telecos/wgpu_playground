# API Reference Panel

The API Reference Panel is an inline documentation viewer that provides WebGPU API reference documentation directly within the application. It helps developers learn and understand WebGPU APIs without leaving the playground.

## Features

- **Comprehensive Coverage**: Documentation for all major WebGPU objects and methods
- **Category Organization**: APIs organized by WebGPU categories (Device, Buffer, Texture, Pipeline, etc.)
- **Method Details**: Each method includes signature, description, and usage examples
- **Search/Filter**: Quick filtering to find specific APIs or methods
- **Specification Links**: Direct links to official WebGPU specification
- **Code Examples**: Practical code examples for each API method

## Usage

### Accessing the Panel

The API Reference panel can be accessed from the Tools & Debugging section in the sidebar:

1. Open the "🔧 Tools & Debugging" section
2. Click on "📖 API Reference"

### Browsing API Categories

The panel displays a list of WebGPU categories on the left side:

- **Adapter**: Physical GPU adapter selection
- **Device**: Main GPU interface for resource creation
- **Queue**: Command submission and data transfer
- **Buffer**: GPU memory buffers
- **Texture**: Image and render target management
- **Sampler**: Texture sampling configuration
- **Shader Module**: WGSL shader compilation
- **Render Pipeline**: Graphics pipeline state
- **Compute Pipeline**: Compute shader pipeline
- **Bind Group**: Resource binding to shaders
- **Command Encoder**: GPU command recording
- **Render Pass**: Rendering operations
- **Compute Pass**: Compute operations

### Viewing Method Details

Click on a category to view its methods. Each method shows:

1. **Method Name**: The name of the API method
2. **Description**: What the method does
3. **Signature**: Full function signature with parameter types
4. **Example**: Practical usage example

### Searching

Use the search bar at the top to filter methods by name or description:

```
Type: "create_buffer" to find buffer creation methods
Type: "pipeline" to find all pipeline-related methods
Type: "draw" to find drawing operations
```

Click "Clear" to reset the search and category filters.

### Opening Specification Links

Each category includes a direct link to the official WebGPU specification:

- Click the specification URL to open it in your browser
- Specification links point to the W3C WebGPU standard

## API Categories Reference

### Device APIs

Core GPU operations for creating resources:

- `create_buffer` - Create GPU buffers
- `create_texture` - Create textures
- `create_shader_module` - Compile WGSL shaders
- `create_render_pipeline` - Create render pipelines
- `create_compute_pipeline` - Create compute pipelines
- `create_bind_group` - Create resource bindings
- `create_command_encoder` - Create command recorders

### Queue APIs

GPU submission and data transfer:

- `submit` - Submit command buffers to GPU
- `write_buffer` - Write data to buffers
- `write_texture` - Write image data to textures

### Buffer APIs

GPU memory operations:

- `slice` - Create buffer views
- `map_async` - Map buffer for CPU access
- `unmap` - Unmap buffer for GPU use

### Render Pass APIs

Drawing operations:

- `set_pipeline` - Set active render pipeline
- `set_bind_group` - Bind resources
- `set_vertex_buffer` - Bind vertex data
- `set_index_buffer` - Bind index data
- `draw` - Draw primitives
- `draw_indexed` - Draw indexed primitives

### Compute Pass APIs

Compute shader operations:

- `set_pipeline` - Set active compute pipeline
- `set_bind_group` - Bind resources
- `dispatch_workgroups` - Execute compute shader

## Integration with Other Panels

The API Reference panel complements other playground panels:

- **API Coverage Panel**: Tracks which APIs you've used
- **Tutorial Panel**: Guides you through using APIs step-by-step
- **Configuration Panels**: Where you actually configure and use the APIs

## Code Examples

### Buffer Creation Example

```rust
let buffer = device.create_buffer(&BufferDescriptor {
    label: Some("Vertex Buffer"),
    size: 1024,
    usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
    mapped_at_creation: false,
});
```

### Texture Creation Example

```rust
let texture = device.create_texture(&TextureDescriptor {
    size: Extent3d { width: 256, height: 256, depth_or_array_layers: 1 },
    format: TextureFormat::Rgba8UnormSrgb,
    usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
    ..Default::default()
});
```

### Render Pass Example

```rust
let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
    color_attachments: &[...],
    depth_stencil_attachment: Some(...),
    ..Default::default()
});

render_pass.set_pipeline(&pipeline);
render_pass.set_bind_group(0, &bind_group, &[]);
render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
render_pass.draw(0..3, 0..1);
```

## Implementation Details

### Panel Structure

The panel is implemented in `api_reference_panel.rs` with:

- `ApiReferenceCategory`: Enum of WebGPU categories
- `ApiMethod`: Struct containing method documentation
- `ApiReferencePanel`: Main panel struct with UI logic

### Data Organization

Documentation is organized hierarchically:

1. **Categories** - Major WebGPU object types
2. **Methods** - Operations available on each type
3. **Details** - Signature, description, and examples for each method

### UI Layout

The panel uses a two-column layout:

- **Left Column**: Category list with descriptions
- **Right Column**: Method details with expandable sections

### Adding New Documentation

To add documentation for new APIs:

1. Add a new variant to `ApiReferenceCategory` if needed
2. Implement `name()`, `description()`, and `spec_url()` for the category
3. Add methods to the `get_api_methods()` function
4. Each method should include signature, description, and example

## Testing

The panel includes comprehensive tests:

```bash
# Run panel tests
cargo test -p wgpu_playground_core --test api_reference_panel_test

# Run unit tests
cargo test -p wgpu_playground_core --lib api_reference_panel::
```

Test coverage includes:

- Panel creation and initialization
- Category metadata validation
- Specification URL validation
- Method documentation completeness

## Future Enhancements

Potential improvements for the API Reference panel:

- **Interactive Examples**: Click to load examples into configuration panels
- **Copy to Clipboard**: Copy code examples with one click
- **Version Selection**: Show API differences across WebGPU versions
- **Favorites**: Bookmark frequently used APIs
- **Search History**: Remember previous searches
- **Related APIs**: Show related methods and common usage patterns

## See Also

- [API Coverage Tracker](API_COVERAGE_TRACKER.md) - Track which APIs you've used
- [Tutorial System](TUTORIAL_SYSTEM.md) - Learn WebGPU through guided tutorials
- [WebGPU Specification](https://www.w3.org/TR/webgpu/) - Official W3C specification
