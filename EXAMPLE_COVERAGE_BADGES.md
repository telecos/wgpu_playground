# Example Coverage Badges Implementation

## Overview
This document describes the implementation of coverage badges for the examples gallery. The badges display which WebGPU API categories each example demonstrates, helping users select examples based on what they want to learn.

## Implementation Details

### Architecture
The implementation uses a modular approach:

1. **example_metadata.rs** - A new module that maps example IDs to their API coverage categories
2. **rendering.rs** - Updated to display color-coded badges in the gallery UI
3. **Badge coloring** - Each API category has a distinct color for easy visual identification

### API Coverage Mapping
Each example is mapped to the WebGPU API categories it exercises:

- **Triangle Example**: Buffer, Shader, RenderPipeline, RenderPass, CommandEncoder, Queue
- **Cube Example**: Buffer, Shader, RenderPipeline, BindGroup, RenderPass, CommandEncoder, Queue
- **Texture Mapping Example**: Buffer, Texture, Sampler, Shader, RenderPipeline, BindGroup, RenderPass, CommandEncoder, Queue
- **Compute Shader Example**: Buffer, Shader, ComputePipeline, BindGroup, ComputePass, CommandEncoder, Queue

### Badge Color Scheme
Each API category has a unique color:

- **RenderPass** - Crimson (main rendering operations)
- **ComputePass** - Blue Violet (compute operations)
- **Buffer** - Sea Green (data operations)
- **Texture** - Dark Orange (texture operations)
- **Sampler** - Goldenrod (sampler operations)
- **Shader** - Steel Blue (shader operations)
- **RenderPipeline** - Firebrick (render pipeline)
- **ComputePipeline** - Medium Slate Blue (compute pipeline)
- **BindGroup** - Forest Green (binding operations)
- **CommandEncoder** - Dark Olive Green (command operations)
- **Queue** - Dark Goldenrod (queue operations)
- **Other categories** - Dim Gray (default)

## UI Integration
The badges appear in the example gallery's right panel, immediately after the example description and before the "Run Example" button. They are displayed as horizontal wrapped labels with:

- White text on colored backgrounds
- Small font size for compactness
- Proper spacing between badges
- Responsive wrapping for different screen sizes

## Testing
Comprehensive tests ensure:

1. All API categories have distinct colors
2. All existing examples have API coverage metadata
3. The badge color function works correctly for all categories
4. Integration between example_metadata and rendering modules

## Future Enhancements
Potential improvements:

1. Add tooltips to badges explaining what each API category does
2. Make badges clickable to filter examples by API category
3. Add icons/emojis to badges for visual variety
4. Track actual API usage dynamically and update badges in real-time
5. Export badge information for documentation generation

## Files Modified

- `crates/wgpu_playground_core/src/lib.rs` - Added example_metadata module
- `crates/wgpu_playground_core/src/example_metadata.rs` - New file with API coverage mapping
- `crates/wgpu_playground_core/src/rendering.rs` - Added badge display and coloring function
