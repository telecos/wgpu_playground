# Texture Configuration Panel

## Overview

The Texture Configuration Panel is a comprehensive UI interface for creating and configuring GPU textures in the wgpu_playground application. It provides an intuitive way to experiment with texture parameters and understand WebGPU texture creation.

## Location

The panel is accessible via the "🖼️ Texture Config" tab in the main application navigation.

## Features

### 1. Texture Properties
- **Label**: Optional text label for debugging purposes
- **Width**: Texture width in pixels with validation
- **Height**: Texture height in pixels with validation
- **Depth/Array Layers**: Depth for 3D textures or array layers for 2D/cube textures
- **Mip Levels**: Number of mipmap levels (must be >= 1)
- **Sample Count**: Sample count for multisampling (1, 2, 4, 8, 16, or 32)

### 2. Texture Dimension (Radio Buttons)
Three texture dimension options:
- **1D**: One-dimensional texture (requires height = 1, depth = 1)
- **2D**: Two-dimensional texture (default)
- **3D**: Three-dimensional texture

### 3. Texture Format (Dropdown)
All 43 WebGPU-compliant texture formats organized into three categories:

#### Color Formats (25 formats)
| Format | Description |
|--------|-------------|
| Rgba8Unorm | 8-bit RGBA normalized |
| Rgba8UnormSrgb | 8-bit RGBA normalized sRGB |
| Bgra8Unorm | 8-bit BGRA normalized |
| Bgra8UnormSrgb | 8-bit BGRA normalized sRGB |
| Rgba16Float | 16-bit RGBA floating point |
| Rgba32Float | 32-bit RGBA floating point |
| Rgb10a2Unorm | 10-bit RGB + 2-bit alpha normalized |
| R8Unorm | 8-bit single-channel normalized |
| R8Snorm | 8-bit single-channel signed normalized |
| R8Uint | 8-bit single-channel unsigned integer |
| R8Sint | 8-bit single-channel signed integer |
| R16Uint | 16-bit single-channel unsigned integer |
| R16Sint | 16-bit single-channel signed integer |
| R16Float | 16-bit single-channel floating point |
| Rg8Unorm | 8-bit dual-channel normalized |
| Rg8Snorm | 8-bit dual-channel signed normalized |
| Rg8Uint | 8-bit dual-channel unsigned integer |
| Rg8Sint | 8-bit dual-channel signed integer |
| Rg16Uint | 16-bit dual-channel unsigned integer |
| Rg16Sint | 16-bit dual-channel signed integer |
| Rg16Float | 16-bit dual-channel floating point |
| Rgba16Uint | 16-bit RGBA unsigned integer |
| Rgba16Sint | 16-bit RGBA signed integer |
| Rgba32Uint | 32-bit RGBA unsigned integer |
| Rgba32Sint | 32-bit RGBA signed integer |

#### Depth/Stencil Formats (4 formats)
| Format | Description |
|--------|-------------|
| Depth32Float | 32-bit floating point depth |
| Depth24Plus | At least 24-bit depth |
| Depth24PlusStencil8 | 24-bit depth + 8-bit stencil |
| Stencil8 | 8-bit stencil only |

#### Compressed Formats (BC) (14 formats)
| Format | Description |
|--------|-------------|
| Bc1RgbaUnorm | BC1 RGBA normalized |
| Bc1RgbaUnormSrgb | BC1 RGBA normalized sRGB |
| Bc2RgbaUnorm | BC2 RGBA normalized |
| Bc2RgbaUnormSrgb | BC2 RGBA normalized sRGB |
| Bc3RgbaUnorm | BC3 RGBA normalized |
| Bc3RgbaUnormSrgb | BC3 RGBA normalized sRGB |
| Bc4RUnorm | BC4 single-channel normalized |
| Bc4RSnorm | BC4 single-channel signed normalized |
| Bc5RgUnorm | BC5 dual-channel normalized |
| Bc5RgSnorm | BC5 dual-channel signed normalized |
| Bc6hRgbUfloat | BC6H RGB unsigned float |
| Bc6hRgbFloat | BC6H RGB signed float |
| Bc7RgbaUnorm | BC7 RGBA normalized |
| Bc7RgbaUnormSrgb | BC7 RGBA normalized sRGB |

### 4. Usage Flags (Checkboxes)
All 5 WebGPU texture usage flags are available as checkboxes with descriptions:

| Flag | Description |
|------|-------------|
| COPY_SRC | Texture can be used as a copy source |
| COPY_DST | Texture can be used as a copy destination |
| TEXTURE_BINDING | Texture can be bound in a shader |
| STORAGE_BINDING | Texture can be used as a storage texture |
| RENDER_ATTACHMENT | Texture can be used as a render attachment |

### 5. Validation
Real-time validation ensures texture configurations are valid:

- **Dimension validation**: All dimensions must be positive numbers
- **Usage validation**: At least one usage flag must be selected
- **1D texture constraints**: 1D textures must have height = 1 and depth = 1
- **Mip level validation**: Mip levels cannot exceed the maximum for the texture size
- **Sample count validation**: Must be 1, 2, 4, 8, 16, or 32
- **Multisampling constraints**: 
  - Multisampled textures cannot have mip levels > 1
  - Only 2D textures can be multisampled

Validation errors are displayed with descriptive messages in red.

### 6. Configuration Summary
A live summary panel displays the current configuration including:
- Label (or "<none>" if not set)
- Texture dimension (1D, 2D, or 3D)
- Size (width x height x depth/layers)
- Selected texture format
- Mip levels
- Sample count
- List of selected usage flags

### 7. Texture Loading from Files (NEW)
Load textures from image files with support for drag-and-drop:
- **📂 Load Image**: Load PNG, JPEG, and other image formats
- **🗑️ Clear Loaded Image**: Remove the currently loaded texture data
- **Drag & Drop Support**: Drag image files directly onto the application window
- **Automatic Dimension Detection**: Image dimensions are automatically applied to Width/Height fields
- **Format Support**: PNG, JPEG, and other formats supported by the image crate

#### Supported Image Formats
- PNG (Portable Network Graphics)
- JPEG/JPG (Joint Photographic Experts Group)
- Additional formats supported by the `image` crate

#### Usage
1. Click **📂 Load Image** button or drag-and-drop an image file onto the application window
2. The image will be decoded and its dimensions automatically populated in the Width and Height fields
3. The loaded texture data is stored and ready for texture creation
4. Use **🗑️ Clear Loaded Image** to remove the loaded data

### 8. Texture Export (NEW)
Export textures to image files:
- **Export to PNG**: Save texture data as PNG format
- **Async Operation**: Uses GPU buffer mapping for efficient data retrieval
- **Programmatic API**: Available via `export_texture_to_bytes()` function

### 9. Actions
Three action buttons are provided:
- **🔍 Validate**: Check if the current configuration is valid
- **✨ Create Texture**: Validate and create the texture (shows success message)
- **🔄 Reset**: Reset all fields to default values

## Technical Implementation

### Module Structure
- **File**: `crates/wgpu_playground_core/src/texture_panel.rs`
- **Lines of Code**: ~570 lines
- **Tests**: 13 comprehensive unit tests

### Key Components

```rust
pub struct TexturePanel {
    label_input: String,
    width_input: String,
    height_input: String,
    depth_input: String,
    mip_levels_input: String,
    sample_count_input: String,
    selected_format: TextureFormat,
    selected_dimension: TextureDimension,
    usage_copy_src: bool,
    usage_copy_dst: bool,
    usage_texture_binding: bool,
    usage_storage_binding: bool,
    usage_render_attachment: bool,
    validation_error: Option<String>,
    success_message: Option<String>,
    loaded_texture_data: Option<Vec<u8>>,        // NEW
    loaded_texture_dimensions: Option<(u32, u32)>, // NEW
    file_load_message: Option<String>,           // NEW
}
```

### New Functions for Texture Loading/Export

```rust
// Load texture from image bytes
pub fn load_texture_from_bytes(
    device: &Device,
    queue: &wgpu::Queue,
    bytes: &[u8],
    label: Option<&str>,
) -> Result<(Texture, u32, u32), String>

// Export texture to PNG bytes
pub async fn export_texture_to_bytes(
    device: &Device,
    queue: &wgpu::Queue,
    texture: &Texture,
    width: u32,
    height: u32,
) -> Result<Vec<u8>, String>
```

### Integration
The panel is integrated into the main application through:
1. Import in `lib.rs`: `pub mod texture_panel;`
2. Import in `app.rs`: `use wgpu_playground_core::texture_panel::TexturePanel;`
3. Field in `PlaygroundApp`: `texture_panel: TexturePanel`
4. New tab enum variant: `Tab::TextureConfig`
5. Navigation sidebar entry: `"🖼️ Texture Config"`

## Usage Example

1. Navigate to the "🖼️ Texture Config" tab
2. Set texture dimensions (e.g., 512x512x1)
3. Select a texture format from the dropdown (e.g., Rgba8Unorm)
4. Choose texture dimension (1D, 2D, or 3D)
5. Set mip levels (e.g., 9 for a 512x512 texture)
6. Set sample count (e.g., 1 for no multisampling)
7. Select desired usage flags (e.g., TEXTURE_BINDING and COPY_DST)
8. Optionally set a label (e.g., "color_texture")
9. Click "🔍 Validate" to check the configuration
10. Click "✨ Create Texture" to create the texture (in full implementation)

## Default Configuration

When the panel is first opened or reset:
- **Label**: Empty
- **Width**: 256 pixels
- **Height**: 256 pixels
- **Depth/Array Layers**: 1
- **Mip Levels**: 1
- **Sample Count**: 1
- **Dimension**: 2D
- **Format**: Rgba8Unorm
- **Usage Flags**: COPY_DST and TEXTURE_BINDING (selected)

## Validation Rules

The panel enforces WebGPU texture creation rules:

1. **All dimensions must be positive**: Width, height, depth, mip levels, and sample count must be > 0
2. **Valid sample count**: Must be one of: 1, 2, 4, 8, 16, or 32
3. **At least one usage flag**: Textures must have a purpose
4. **1D texture constraints**: 1D textures require height = 1 and depth = 1
5. **Mip level maximum**: Cannot exceed log2(max(width, height)) + 1
6. **Multisampling constraints**:
   - Multisampled textures (sample count > 1) cannot have mip levels > 1
   - Only 2D textures can be multisampled

## Test Coverage

The module includes 13 unit tests covering:
- Panel creation and default values
- Validation success scenarios
- Zero/invalid dimension validation
- Invalid sample count validation
- No usage flags validation
- 1D texture dimension constraints
- Mip level maximum validation
- Multisampling with mip levels validation
- Multisampling on non-2D textures validation
- Usage flag building
- Format selection
- Dimension selection

All tests pass successfully.

## Comparison with Buffer Panel

The Texture Configuration Panel follows the same design patterns as the Buffer Configuration Panel, but is more complex due to:
- More configuration options (dimensions, formats, mip levels, sample count)
- More complex validation rules (dimension constraints, mip level calculations, multisampling rules)
- Larger format dropdown (43 formats vs. 10 usage flags)
- Additional dimension selection (1D, 2D, 3D)

## Future Enhancements

Potential improvements for future iterations:
- Integration with actual device to create textures in GPU memory
- Texture list/management UI
- Texture preview with mipmap visualization
- Texture data upload/download
- Format compatibility checking
- Performance metrics and recommendations
- View format configuration
- Cube texture and texture array support indicators
