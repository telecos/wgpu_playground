# Configuration Templates/Presets

## Overview

The Configuration Presets feature provides pre-configured settings for common rendering scenarios, allowing users to quickly set up complex rendering pipelines without manual configuration. This feature is designed to help both beginners learn WebGPU concepts and experienced users quickly prototype rendering techniques.

## Features

### Available Presets

1. **PBR Material** (Physically Based Rendering)
   - Category: Material
   - Demonstrates realistic material rendering using Cook-Torrance BRDF
   - Includes:
     - PBR shader with metallic-roughness workflow
     - Material uniform buffer (albedo, metallic, roughness, AO)
     - Proper depth testing and blending configuration
   - Use cases: Realistic 3D object rendering, game engines, product visualization

2. **Shadow Mapping**
   - Category: Lighting
   - Two-pass shadow rendering with PCF (Percentage Closer Filtering)
   - Includes:
     - Depth texture (2048x2048 shadow map)
     - Comparison sampler with depth testing
     - Light space transformation uniforms
     - Shader with shadow calculation and PCF filtering
   - Use cases: Dynamic shadows, realistic lighting, outdoor scenes

3. **Post-Processing Effects**
   - Category: Post-Processing
   - Full-screen quad with multiple image effects
   - Includes:
     - HDR input texture (Rgba16Float)
     - Linear sampler for smooth filtering
     - Effect parameters uniform buffer
     - Shader with vignette, chromatic aberration, bloom, and tone mapping
   - Use cases: Film effects, game post-processing, image enhancement

### Preset Capabilities

Each preset includes:
- **Complete Shader Code**: Production-ready WGSL shaders with detailed comments
- **Buffer Configurations**: Properly sized uniform buffers with correct usage flags
- **Texture Settings**: Appropriate formats, dimensions, and usage flags
- **Sampler Configuration**: Filtering modes and address modes for the specific use case
- **Pipeline State**: Render pipeline configuration including depth testing, blending, and topology
- **Documentation**: Descriptions explaining the technique and its applications

## User Interface

### Accessing Presets

1. Navigate to **Tools & Debugging** section in the sidebar
2. Click on **Configuration Presets**
3. The preset panel will display all available presets

### Preset Panel Features

- **Category Filtering**: Filter presets by Material, Lighting, Post-Processing, or Rendering
- **Search Functionality**: Search presets by name, description, or tags
- **Preset Cards**: Each preset displays:
  - Category badge with color coding
  - Name and description
  - Tags for easy discovery
  - Action buttons (Load Preset, View Details)
- **Expandable Details**: Click "View Details" to see:
  - List of included components (shader, buffers, textures, etc.)
  - Shader code preview with syntax highlighting
- **One-Click Loading**: Click "Load Preset" to apply the configuration

### Color Coding

- **Material** (Steel Blue): Material and shading techniques
- **Lighting** (Gold): Lighting and shadow rendering
- **Post-Processing** (Blue Violet): Image effects and filters
- **Rendering** (Crimson): General rendering techniques

## Usage

### Loading a Preset

1. Open the **Configuration Presets** panel
2. Browse or search for a preset
3. Click **Load Preset** on the desired configuration
4. The preset configuration will be applied to:
   - Shader editor (if included)
   - Buffer panel (if included)
   - Texture panel (if included)
   - Sampler panel (if included)
   - Render pipeline panel (if included)

### Customizing a Preset

After loading a preset:

1. Navigate to the relevant configuration panels
2. Modify settings as needed
3. The preset serves as a starting point for your custom configuration
4. Save your modified state using the Save/Load feature

### Example Workflow

**Setting up shadow mapping:**

1. Load the "Shadow Mapping" preset
2. Navigate to the Texture panel to review the shadow map configuration
3. Navigate to the Shader editor to see the shadow mapping implementation
4. Customize shadow map resolution or PCF kernel size as needed
5. Test with your scene geometry

## Technical Details

### Preset Structure

Each preset is defined by:

```rust
pub struct ConfigPreset {
    pub id: &'static str,           // Unique identifier
    pub name: &'static str,          // Display name
    pub category: PresetCategory,    // Category
    pub description: &'static str,   // Detailed description
    pub tags: &'static [&'static str], // Search tags
    pub state: PlaygroundState,      // Configuration state
}
```

### State Serialization

Presets use the same `PlaygroundState` structure as the save/load system, ensuring:
- Consistency with manual configurations
- Ability to share preset-based configurations via URL
- Export preset modifications for later use

### Adding New Presets

To add a new preset, edit `crates/wgpu_playground_core/src/preset.rs`:

1. Create a function that returns `ConfigPreset`
2. Define the `PlaygroundState` with appropriate panel configurations
3. Add the preset to `get_all_presets()` function

Example:

```rust
fn create_my_preset() -> ConfigPreset {
    let mut state = PlaygroundState::new();
    
    state.shader_editor = Some(ShaderEditorState {
        source_code: "...".to_string(),
        label: "my_shader".to_string(),
        file_path: String::new(),
    });
    
    // Configure other components...
    
    ConfigPreset::new(
        "my_preset",
        "My Preset Name",
        PresetCategory::Rendering,
        "Description of my preset...",
        &["tag1", "tag2"],
        state,
    )
}
```

## Best Practices

1. **Start with Presets**: Use presets as learning tools to understand WebGPU concepts
2. **Customize Gradually**: Load a preset and modify one aspect at a time
3. **Save Your Work**: After customizing a preset, save your configuration
4. **Combine Techniques**: Load multiple presets in sequence to learn different techniques
5. **Read the Shaders**: Study the included shader code to understand the implementation

## Educational Value

Presets are designed to be educational:

- **Well-Commented Shaders**: Each shader includes extensive comments explaining the algorithm
- **Standard Practices**: Configurations follow WebGPU best practices
- **Progressive Complexity**: Presets range from basic to advanced techniques
- **Real-World Applicable**: Techniques used in production graphics applications

## Troubleshooting

### Preset Not Loading

- Ensure you're clicking "Load Preset" not just "View Details"
- Check the console for any error messages
- Try loading a different preset to isolate the issue

### Configuration Conflicts

- Loading a preset replaces the current configuration for that panel
- To preserve existing work, save your state before loading a preset
- You can manually copy specific settings you want to keep

### Performance Issues

- Shadow mapping preset uses a 2048x2048 texture (adjust if needed)
- Post-processing effects run per-frame (suitable for real-time rendering)
- PBR shaders are optimized but may be heavy for complex scenes

## Future Enhancements

Potential additions to the preset system:

- More presets (deferred rendering, SSAO, particles, etc.)
- Preset categories for compute shaders
- User-created preset sharing
- Preset preview thumbnails
- Preset compatibility checking
- Animated preset demonstrations

## Related Features

- **Save/Load System**: Export preset-based configurations
- **URL Sharing**: Share configurations derived from presets
- **Tutorial System**: Presets complement tutorials for learning
- **Examples**: View executable examples that use similar techniques

## API Reference

### Core Functions

- `get_all_presets()`: Returns all available presets
- `PresetPanel::new()`: Create a new preset panel instance
- `PresetPanel::ui(&mut self, ui: &mut egui::Ui)`: Render the preset panel UI

### Categories

```rust
pub enum PresetCategory {
    Material,
    Lighting,
    PostProcessing,
    Rendering,
}
```

## Testing

Presets are thoroughly tested:

- Unit tests for preset structure and metadata
- Serialization tests for state compatibility
- Integration tests for panel functionality
- All tests can be run with: `cargo test preset`

## Conclusion

The Configuration Presets feature accelerates the learning curve for WebGPU and provides a solid foundation for building complex rendering applications. Whether you're a beginner exploring graphics programming or an experienced developer prototyping new techniques, presets offer a quick way to get started with production-quality configurations.
