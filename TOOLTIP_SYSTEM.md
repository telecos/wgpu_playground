# Tooltip System with API Links

## Overview

The WebGPU Playground now includes a comprehensive tooltip system that enhances all UI controls with rich tooltips explaining WebGPU concepts and linking directly to relevant sections of the WebGPU specification.

## Implementation

### Core Module: `tooltip.rs`

The tooltip system is implemented in `crates/wgpu_playground_core/src/tooltip.rs` and provides:

1. **`TooltipExt` trait** - Extends `egui::Response` with `.webgpu_tooltip()` method
2. **`TooltipInfo` struct** - Encapsulates description and spec anchor
3. **Organized tooltip modules** - 14 modules covering all WebGPU concepts

### Tooltip Modules

The tooltip system is organized into the following modules:

- **`buffer_usage`** - Buffer usage flags (VERTEX, INDEX, UNIFORM, STORAGE, etc.)
- **`texture_usage`** - Texture usage flags (COPY_SRC, COPY_DST, TEXTURE_BINDING, etc.)
- **`primitive_topology`** - Primitive topologies (POINT_LIST, LINE_LIST, TRIANGLE_LIST, etc.)
- **`cull_mode`** - Face culling modes (NONE, FRONT, BACK)
- **`front_face`** - Front face winding orders (CCW, CW)
- **`compare_function`** - Comparison functions for depth/stencil testing
- **`blend_factor`** - Blend factors for color blending
- **`blend_operation`** - Blend operations (ADD, SUBTRACT, MIN, MAX)
- **`address_mode`** - Texture addressing modes (CLAMP_TO_EDGE, REPEAT, etc.)
- **`filter_mode`** - Texture filtering modes (NEAREST, LINEAR)
- **`stencil_operation`** - Stencil operations (KEEP, ZERO, REPLACE, etc.)
- **`property`** - General properties (sizes, flags, counts)
- **`shader_visibility`** - Shader stage visibility (VERTEX, FRAGMENT, COMPUTE)
- **`compute`** - Compute-specific properties (workgroup counts, entry points)
- **`load_store_op`** - Render pass load/store operations
- **`draw`** - Draw command parameters
- **`sampler`** - Sampler-specific properties (LOD, anisotropy, border color)

## Usage

### Adding Tooltips to UI Controls

There are two main patterns for adding tooltips:

#### Pattern 1: Direct `.webgpu_tooltip()`

```rust
use crate::tooltip::TooltipExt;

ui.label("Label:")
    .webgpu_tooltip(
        "Optional label for debugging and identification",
        Some("#dom-gpuobjectbase-label")
    );
```

#### Pattern 2: Using Tooltip Constants with `.apply()`

```rust
use crate::tooltip::{buffer_usage, property, TooltipExt};

// For properties
property::BUFFER_SIZE.apply(ui.label("Size (bytes):"));

// For checkboxes
buffer_usage::VERTEX.apply(ui.checkbox(&mut value, "VERTEX"));
```

### Enum Value Tooltips

For enums, create helper methods that map values to tooltips:

```rust
fn topology_tooltip(topology: &PrimitiveTopology) -> &'static TooltipInfo {
    match topology {
        PrimitiveTopology::PointList => &primitive_topology::POINT_LIST,
        PrimitiveTopology::LineList => &primitive_topology::LINE_LIST,
        PrimitiveTopology::TriangleList => &primitive_topology::TRIANGLE_LIST,
        // ...
    }
}

// Usage
Self::topology_tooltip(&topology).apply(
    ui.selectable_label(is_selected, format!("{:?}", topology))
);
```

## Enhanced Panels

The following panels have been enhanced with tooltips:

1. **Buffer Panel** - All buffer usage flags and properties
2. **Texture Panel** - Texture usage flags, dimensions, mip levels, sample count
3. **Render Pipeline Panel** - Topology, culling, depth/stencil, blending, multisampling
4. **Compute Pipeline Panel** - Entry point, pipeline layout
5. **Sampler Panel** - Address modes, filters, LOD, anisotropy, border color
6. **Bind Group Layout Panel** - Shader visibility, binding configurations
7. **Bind Group Panel** - Shader visibility
8. **Render Pass Panel** - Load/store operations, attachment configurations
9. **Draw Command Panel** - Vertex count, instance count, first vertex/instance
10. **Compute Dispatch Panel** - Workgroup dimensions
11. **Settings Panel** - Backend selection

## Tooltip Features

Each tooltip provides:

1. **Human-readable description** - Clear explanation of the WebGPU concept
2. **Clickable spec link** - Direct link to relevant WebGPU specification section
3. **Context-aware content** - Tooltips adapt to selected values (e.g., different tooltip for each blend factor)

### Example Tooltip Display

When hovering over a buffer usage checkbox:
```
Buffer can be used as a uniform buffer. Contains read-only data accessible 
to shaders, typically for global parameters like transformation matrices.

📄 WebGPU Spec (clickable)
```

## Testing

The tooltip system includes comprehensive test coverage in `tests/tooltip_test.rs`:

- 21 tests covering all tooltip modules
- Tests verify descriptions are non-empty
- Tests verify spec anchors are present
- All tests pass

Run tests with:
```bash
cargo test --package wgpu_playground_core --test tooltip_test
```

## Adding New Tooltips

To add tooltips for new UI controls:

1. **Determine the appropriate module** in `tooltip.rs` or create a new one
2. **Add tooltip constant**:
   ```rust
   pub const MY_PROPERTY: TooltipInfo = TooltipInfo::new(
       "Description of the property...",
       Some("#dom-gpuproperty-name"),
   );
   ```
3. **Import and use in panel**:
   ```rust
   use crate::tooltip::{my_module, TooltipExt};
   
   my_module::MY_PROPERTY.apply(ui.label("My Property:"));
   ```
4. **Add test** in `tooltip_test.rs`:
   ```rust
   #[test]
   fn test_my_module_tooltips() {
       assert!(!my_module::MY_PROPERTY.description.is_empty());
       assert!(my_module::MY_PROPERTY.spec_anchor.is_some());
   }
   ```

## WebGPU Specification Links

All spec links point to the official WebGPU specification at:
`https://www.w3.org/TR/webgpu/`

Anchors follow the WebGPU spec naming convention, e.g.:
- `#dom-gpubufferusage-vertex` - Buffer usage flags
- `#dom-gpuprimitivetopology-triangle-list` - Primitive topology
- `#dom-gpucomparefunction-less` - Compare functions

## Benefits

The tooltip system provides several benefits:

1. **Learn WebGPU in-app** - Users can understand concepts without leaving the UI
2. **Quick reference** - Immediate access to WebGPU specification
3. **Reduced learning curve** - Tooltips explain complex concepts in simple terms
4. **Consistent UX** - Uniform tooltip pattern across all panels
5. **Maintainable** - Centralized tooltip definitions make updates easy

## Future Enhancements

Potential future improvements:

1. Add more detailed examples in tooltips
2. Include performance tips for specific settings
3. Add tooltips for more advanced panels (if any)
4. Support for localization/internationalization
5. Add visual diagrams in tooltips for complex concepts

## Maintenance

When updating the tooltip system:

1. Keep descriptions concise and clear (aim for 1-2 sentences)
2. Always include spec anchors when available
3. Test tooltips in the UI to ensure they display correctly
4. Update tests when adding new tooltip constants
5. Run `cargo fmt` to maintain code formatting
6. Verify all tests pass with `cargo test`
