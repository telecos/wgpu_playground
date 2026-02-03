# wgpu_playground Quick Start Guide

Get up and running with wgpu_playground in 5 minutes!

## Installation

### Prerequisites

- **Rust** (install from [rustup.rs](https://rustup.rs/))
- **GPU** with Vulkan, Metal, or DirectX 12 support

### Build and Run

```bash
# Clone the repository
git clone https://github.com/telecos/wgpu_playground.git
cd wgpu_playground

# Build and run
cargo run --release
```

**Linux users**: Install Vulkan first:
```bash
sudo apt-get install libvulkan-dev vulkan-tools
```

**Windows users with AMD GPU**: Use Vulkan backend to avoid errors:
```bash
WGPU_BACKEND=vulkan cargo run --release
```

## First Steps

### 1. Select Your GPU

When the application launches:
1. You'll see the **Adapter Selection** tab
2. Your GPU should be automatically selected
3. If you have multiple GPUs, click on your preferred one
4. Click **Create Device**

### 2. View GPU Information

1. Click the **Device Info** tab
2. See your GPU's:
   - Name and vendor
   - Maximum texture sizes
   - Supported features
   - Device limits

### 3. Try Your First Shader

1. Click the **Rendering** tab
2. Expand **Example Gallery**
3. Click on "Basic Triangle"
4. Click **Load Example**
5. Click **Compile Shader**
6. ✓ You should see "Shader compiled successfully!"

### 4. Create a Buffer

1. Click the **Buffer Config** tab
2. Enter:
   - **Size**: `1024`
   - **Usage**: Check ☑ **VERTEX** and ☑ **COPY_DST**
   - **Label**: `My First Buffer`
3. Click **Create Buffer**
4. Check the **Console** tab (bottom) for confirmation

### 5. Create a Texture

1. Click the **Texture Config** tab
2. Configure:
   - **Width**: `512`
   - **Height**: `512`
   - **Format**: `Rgba8Unorm`
   - **Usage**: Check ☑ **TEXTURE_BINDING** and ☑ **COPY_DST**
3. Click **Create Texture**

### 6. Inspect Your Resources

1. Click the **Resource Inspector** tab
2. View all buffers and textures you've created
3. Click on a buffer to see its details
4. Click **Read Buffer** to view its contents

## Common Tasks

### Load and Edit a Shader

```
Rendering Tab → Example Gallery → Select Example → Load Example → Edit → Compile
```

### Create a Storage Buffer for Compute

```
Buffer Config Tab → Size: 4096 → Usage: ☑ STORAGE ☑ COPY_DST → Create Buffer
```

### Configure Texture Sampler

```
Sampler Config Tab → Set Address Mode (Repeat) → Set Filters (Linear) → Create Sampler
```

### Check for Errors

```
Console Tab (bottom of window) → Review error messages in red
```

## Tab Overview

| Tab | Purpose |
|-----|---------|
| **Adapter Selection** | Choose your GPU |
| **Device Config** | Configure GPU features and limits |
| **Device Info** | View GPU capabilities |
| **Rendering** | Shader editor and examples |
| **Buffer Config** | Create GPU buffers |
| **Texture Config** | Create textures |
| **Sampler Config** | Configure texture samplers |
| **Bind Group Config** | Create resource bindings |
| **Console** | View errors and messages |
| **Resource Inspector** | Inspect created resources |
| **Performance** | Monitor GPU performance |

## Troubleshooting

### Application won't start
- **Update GPU drivers** to latest version
- **Try Vulkan backend**: `WGPU_BACKEND=vulkan cargo run --release`
- **Verify GPU support**: Check GPU supports Vulkan/Metal/DX12

### Shader won't compile
- **Check error message** in Console tab for line number
- **Load an example** and modify it incrementally
- **Verify syntax**: WGSL is case-sensitive

### Buffer/Texture creation fails
- **Check Console** for specific error
- **Verify size** is > 0
- **Check usage flags** - at least one must be set
- **Review limits** in Device Info tab

### Windows DirectX 12 errors
- **Use Vulkan**: `WGPU_BACKEND=vulkan cargo run --release`
- This avoids known DirectX 12 backend issues

## Next Steps

Ready to dive deeper? Check out:

- **[USER_GUIDE.md](USER_GUIDE.md)** - Comprehensive user guide with tutorials
- **[SHADER_EDITOR.md](SHADER_EDITOR.md)** - Complete shader editor guide
- **[Examples](../crates/wgpu_playground_examples/)** - Standalone example programs

## Examples to Try

Run these standalone examples to learn more:

```bash
# Render a colorful triangle
cargo run --package wgpu_playground_examples --example triangle

# Texture mapping demo
cargo run --package wgpu_playground_examples --bin texture_mapping

# 3D rotating cube
cargo run --package wgpu_playground_examples --example rotating_cube

# Multi-pass rendering
cargo run --package wgpu_playground_examples --example render_to_texture

# Compute shader demo
cargo run --package wgpu_playground_examples --example compute_render_sharing
```

## Getting Help

- **Check Console tab** - Most errors are explained there
- **Read error messages** - They usually tell you what's wrong
- **Try examples** - Load working examples and modify them
- **Open an issue** - [GitHub Issues](https://github.com/telecos/wgpu_playground/issues)

Happy experimenting! 🎉
