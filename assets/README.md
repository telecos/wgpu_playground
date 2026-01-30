# Assets Directory

This directory contains static assets used by the wgpu_playground application.

## Directory Structure

```
assets/
├── shaders/      # WGSL shader files
├── textures/     # Texture assets (PNG, JPG, etc.)
└── models/       # 3D model files (OBJ, GLTF, etc.)
```

## Shaders

Place WGSL (WebGPU Shading Language) shader files in the `shaders/` directory.

**Example:**
- `example.wgsl` - A simple vertex and fragment shader example

## Textures

Place texture image files in the `textures/` directory. Supported formats typically include:
- PNG
- JPG/JPEG
- BMP
- TGA
- DDS

## Models

Place 3D model files in the `models/` directory. Supported formats may include:
- OBJ
- GLTF/GLB
- FBX (depending on implementation)

## Asset Loading

Assets can be loaded using the `wgpu_playground_core::assets` module:

```rust
use wgpu_playground_core::assets;

// Load a shader
let shader_code = assets::load_shader("example.wgsl")?;

// Load a texture
let texture_data = assets::load_texture("my_texture.png")?;

// Load a model
let model_data = assets::load_model("my_model.obj")?;
```

### Native vs Web Builds

The asset loading system automatically handles path resolution for both native and web builds:

- **Native builds**: Assets are loaded from the filesystem relative to the workspace root
- **Web builds**: Assets are served from the `/assets` path and loaded via the Fetch API

## Adding New Assets

1. Place your asset file in the appropriate subdirectory
2. Use the asset loading functions from the `assets` module
3. For shaders, ensure they use valid WGSL syntax
4. For textures and models, ensure they are in a supported format
