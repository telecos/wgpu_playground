# wgpu_playground User Guide

Welcome to the wgpu_playground user guide! This comprehensive guide will help you get started with the application and master its features for experimenting with WebGPU in Rust.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Understanding the Interface](#understanding-the-interface)
3. [Common Workflows](#common-workflows)
4. [Step-by-Step Tutorials](#step-by-step-tutorials)
5. [Advanced Features](#advanced-features)
6. [Troubleshooting](#troubleshooting)

## Getting Started

### Installation and Setup

#### Prerequisites

Before running wgpu_playground, ensure you have:

- **Rust** (latest stable version) - Install from [rustup.rs](https://rustup.rs/)
- **A GPU with WebGPU support** - Your GPU must support one of:
  - Vulkan (Windows, Linux, Android, macOS via MoltenVK)
  - Metal (macOS, iOS)
  - DirectX 12 (Windows 10+)
- **Display/Window System** - X11 or Wayland on Linux

#### Building the Application

```bash
# Clone the repository (if not already done)
git clone https://github.com/telecos/wgpu_playground.git
cd wgpu_playground

# Build in release mode for better performance
cargo build --release

# Run the application
cargo run --release
```

#### First Launch

On first launch, the application will:

1. Enumerate available GPU adapters on your system
2. Select the most suitable adapter (typically your discrete GPU if available)
3. Create a WebGPU device with default features and limits
4. Open the main GUI window with the Adapter Selection tab active

### Choosing Your WebGPU Implementation

wgpu_playground supports two WebGPU implementations:

- **wgpu** (default) - Pure Rust implementation, fast and cross-platform
- **Dawn** (optional) - Google's C++ implementation used in Chromium

To use the default wgpu implementation:
```bash
cargo run --release
```

To use Dawn (requires build tools: CMake, C++ compiler):
```bash
cargo build --release --features dawn
cargo run --release --features dawn
```

### Selecting Graphics Backend

You can force a specific graphics API backend:

```bash
# Use Vulkan (recommended for Windows/Linux)
WGPU_BACKEND=vulkan cargo run --release

# Use Metal (macOS/iOS)
WGPU_BACKEND=metal cargo run --release

# Use DirectX 12 (Windows)
WGPU_BACKEND=dx12 cargo run --release
```

## Understanding the Interface

### Main Window Layout

The wgpu_playground interface consists of:

- **Top Menu Bar** - Quick access to tabs and settings
- **Tab Bar** - Navigate between different feature areas
- **Main Panel** - The active tab's content
- **Console Panel** (bottom) - Shows GPU errors, warnings, and messages
- **Status Bar** (bottom) - Shows active WebGPU implementation and backend

### Available Tabs

The application provides 19 specialized tabs organized by functionality:

#### GPU Setup Tabs

1. **Adapter Selection** - Choose and configure GPU adapters
2. **Device Config** - Configure device features and limits
3. **Device Info** - View detailed GPU information

#### Resource Configuration Tabs

4. **Buffer Config** - Create and configure GPU buffers
5. **Texture Config** - Create and configure textures
6. **Sampler Config** - Configure texture samplers
7. **Bind Group Config** - Create bind groups for shader resources
8. **Bind Group Layout Config** - Define bind group layouts

#### Pipeline Configuration Tabs

9. **Render Pipeline Config** - Configure graphics rendering pipelines
10. **Compute Pipeline Config** - Configure compute shader pipelines

#### Rendering & Compute Tabs

11. **Rendering** - Interactive shader editor and example gallery
12. **Compute** - Compute shader and ML operations
13. **Compute Dispatch** - Dispatch compute workgroups

#### Command Recording Tabs

14. **Draw Command** - Configure draw calls
15. **Render Pass Config** - Configure render passes
16. **Command Recording** - Record and manage command buffers

#### Monitoring Tabs

17. **Console** - View GPU messages and errors
18. **Resource Inspector** - Inspect created GPU resources
19. **Performance** - Monitor GPU performance metrics

### Navigation Tips

- **Tab Selection**: Click tabs in the top bar or use keyboard shortcuts
- **Scrolling**: Use mouse wheel or scrollbars for long panels
- **Expanding Sections**: Click section headers with ▶ icons to expand/collapse
- **Tooltips**: Hover over UI elements for helpful tooltips

## Common Workflows

### Workflow 1: Selecting a GPU Adapter

**Objective**: Choose the best GPU for your experiments

**Steps**:

1. Launch the application (it starts on the **Adapter Selection** tab)
2. View the list of available GPU adapters in the left panel
3. Each adapter shows:
   - Name (e.g., "NVIDIA GeForce RTX 3080")
   - Vendor (e.g., "NVIDIA Corporation")
   - Device Type (DiscreteGpu, IntegratedGpu, VirtualGpu, Cpu)
   - Backend (Vulkan, Metal, DX12, OpenGL)
4. Click on an adapter to select it
5. Review the adapter's detailed properties in the right panel
6. Configure power preference if needed:
   - **None** - No preference
   - **Low Power** - Prefer integrated GPU for battery life
   - **High Performance** - Prefer discrete GPU for best performance
7. Click **Create Device** to initialize the GPU with selected adapter

**Tips**:
- Discrete GPUs (dedicated graphics cards) offer better performance
- Integrated GPUs (built into CPU) use less power
- Vulkan backend is recommended for Windows/Linux
- Check the Console tab for any adapter-related warnings

### Workflow 2: Viewing GPU Information

**Objective**: Understand your GPU's capabilities and limits

**Steps**:

1. Navigate to the **Device Info** tab
2. Review the **Adapter Information** section:
   - GPU name and vendor
   - Device type and backend
   - WebGPU implementation (wgpu or Dawn)
3. Scroll to **Device Limits** to see:
   - Maximum texture dimensions (e.g., 16384x16384)
   - Maximum buffer size
   - Maximum bind groups
   - Maximum workgroup sizes for compute shaders
   - And many more limits...
4. Check **Supported Features** for enabled WebGPU features:
   - Texture compression formats (BC, ETC2, ASTC)
   - Advanced shader features
   - Depth formats
   - And more...

**Use Cases**:
- Verify GPU supports required features before writing shaders
- Check limits when designing large textures or buffers
- Compare different adapters' capabilities
- Debug feature availability issues

### Workflow 3: Creating a GPU Buffer

**Objective**: Create a buffer for vertex data, uniforms, or storage

**Steps**:

1. Navigate to the **Buffer Config** tab
2. Configure buffer properties:
   
   **a. Set Buffer Size**:
   - Enter size in bytes (e.g., `1024` for 1KB)
   - The UI validates size is > 0
   
   **b. Select Usage Flags** (check all that apply):
   - ☑ **MAP_READ** - Allow CPU to read buffer contents
   - ☑ **MAP_WRITE** - Allow CPU to write buffer contents
   - ☑ **COPY_SRC** - Buffer can be copied from
   - ☑ **COPY_DST** - Buffer can be copied to
   - ☑ **INDEX** - Use as index buffer
   - ☑ **VERTEX** - Use as vertex buffer
   - ☑ **UNIFORM** - Use as uniform buffer
   - ☑ **STORAGE** - Use as storage buffer (read/write in shaders)
   - ☑ **INDIRECT** - Use for indirect draw/dispatch commands
   - ☑ **QUERY_RESOLVE** - Use for query results
   
   **c. Optional Settings**:
   - **Label**: Give the buffer a descriptive name (e.g., "Vertex Buffer")
   - **Mapped at Creation**: Check to map the buffer immediately for writing initial data

3. Review the **Configuration Summary** panel showing your settings
4. Click **Create Buffer** to create the GPU buffer
5. Check the **Console** tab for creation confirmation or errors
6. View the created buffer in the **Resource Inspector** tab

**Common Buffer Configurations**:

- **Vertex Buffer**: `VERTEX | COPY_DST`, size = vertex count × vertex size
- **Index Buffer**: `INDEX | COPY_DST`, size = index count × 4 bytes
- **Uniform Buffer**: `UNIFORM | COPY_DST`, size = struct size (aligned to 256 bytes)
- **Storage Buffer**: `STORAGE | COPY_DST`, size = data size
- **Staging Buffer**: `MAP_WRITE | COPY_SRC`, size = data size to upload

### Workflow 4: Working with Shaders

**Objective**: Write, load, and compile WGSL shaders

**Steps**:

1. Navigate to the **Rendering** tab
2. The Shader Editor is displayed with two sections:

#### Loading Example Shaders

3. Click **Example Gallery** section to expand
4. Browse available shader examples:
   - Basic Triangle
   - Textured Quad
   - 3D Cube
   - Compute Particles
   - And more...
5. Click an example to see its description and source code
6. Click **Load Example** to load it into the editor

#### Editing Shaders

7. The **WGSL Shader Editor** section shows:
   - Line numbers for easy navigation
   - Syntax highlighting for WGSL keywords, types, and functions
   - Current file name (if loaded from file)
8. Edit the shader code directly in the text area
9. Features of the editor:
   - **Syntax Highlighting**: Keywords, types, built-in functions are colored
   - **Line Numbers**: Click to select a line
   - **Load from File**: Click **Browse Shaders** to load from `assets/shaders/`
   - **Real-time Editing**: Changes are immediately visible

#### Compiling Shaders

10. Click **Compile Shader** to validate your shader
11. Compilation results appear below:
    - ✓ **Success**: "Shader compiled successfully!"
    - ✗ **Error**: Detailed error messages with line numbers

**Shader Writing Tips**:

- Start with an example shader and modify it
- Check the Console tab for detailed compilation errors
- Use `//` for single-line comments
- WGSL is case-sensitive
- Common entry points: `@vertex`, `@fragment`, `@compute`
- Refer to [WGSL Specification](https://www.w3.org/TR/WGSL/) for syntax details

### Workflow 5: Configuring a Texture

**Objective**: Create a texture for rendering or sampling

**Steps**:

1. Navigate to the **Texture Config** tab
2. Configure texture properties:

   **a. Texture Dimensions**:
   - **Width**: Texture width in pixels (e.g., 512)
   - **Height**: Texture height in pixels (e.g., 512)
   - **Depth/Array Layers**: Number of layers for 2D array or 3D textures (usually 1)
   
   **b. Texture Format**:
   - Select from dropdown (e.g., `Rgba8Unorm` for standard RGBA textures)
   - Common formats:
     - **Rgba8Unorm** - 8-bit RGBA, normalized to [0, 1]
     - **Rgba16Float** - 16-bit RGBA, floating-point
     - **Depth24Plus** - 24-bit depth buffer
     - **Bgra8Unorm** - 8-bit BGRA (common for swap chains)
   
   **c. Texture Dimension Type**:
   - **D1** - 1D texture (width only)
   - **D2** - 2D texture (width × height)
   - **D3** - 3D texture (width × height × depth)
   
   **d. Mip Levels**:
   - Number of mipmap levels (1 = no mipmaps)
   - Mipmaps improve texture quality at different distances
   
   **e. Sample Count**:
   - Usually 1 (no multisampling)
   - Set to 4, 8, or 16 for MSAA (MultiSample Anti-Aliasing)
   
   **f. Usage Flags** (check all that apply):
   - ☑ **TEXTURE_BINDING** - Allow binding to shaders for sampling
   - ☑ **COPY_DST** - Allow copying data to texture
   - ☑ **COPY_SRC** - Allow copying data from texture
   - ☑ **RENDER_ATTACHMENT** - Use as render target
   
   **g. Optional Settings**:
   - **Label**: Descriptive name (e.g., "Albedo Texture")

3. Review the **Configuration Summary**
4. Click **Create Texture** to create the texture
5. Check **Console** for confirmation or errors

**Common Texture Configurations**:

- **Color Render Target**: `RENDER_ATTACHMENT | TEXTURE_BINDING`, Rgba8Unorm, sample_count=1
- **Depth Buffer**: `RENDER_ATTACHMENT`, Depth24Plus, sample_count=1
- **Sampled Texture**: `TEXTURE_BINDING | COPY_DST`, Rgba8Unorm, with mipmaps
- **Storage Texture**: `STORAGE_BINDING | COPY_DST`, Rgba8Unorm or Rgba32Float

### Workflow 6: Creating a Sampler

**Objective**: Configure how textures are sampled in shaders

**Steps**:

1. Navigate to the **Sampler Config** tab
2. Configure sampler properties:

   **a. Address Modes** (how texture coordinates outside [0, 1] are handled):
   - **Address Mode U** (horizontal):
     - **Repeat** - Tile the texture (wrap around)
     - **MirrorRepeat** - Mirror and tile
     - **ClampToEdge** - Clamp to edge pixels
     - **ClampToBorder** - Clamp to border color
   - **Address Mode V** (vertical): Same options as U
   - **Address Mode W** (depth): Same options as U
   
   **b. Filter Modes**:
   - **Mag Filter** (magnification - texture is larger than screen pixels):
     - **Nearest** - Sharp, pixelated look
     - **Linear** - Smooth, blurred look
   - **Min Filter** (minification - texture is smaller than screen pixels):
     - **Nearest** - Sharp, aliased
     - **Linear** - Smooth, anti-aliased
   - **Mipmap Filter**:
     - **Nearest** - Sharp transitions between mip levels
     - **Linear** - Smooth transitions between mip levels
   
   **c. LOD (Level of Detail)**:
   - **LOD Min Clamp**: Minimum mipmap level (e.g., 0.0)
   - **LOD Max Clamp**: Maximum mipmap level (e.g., 32.0)
   
   **d. Comparison Function** (for depth textures):
   - **Never**, **Less**, **Equal**, **LessEqual**, **Greater**, **NotEqual**, **GreaterEqual**, **Always**
   
   **e. Anisotropic Clamping**:
   - Value from 1 to 16 (1 = disabled, 16 = maximum quality)
   - Higher values improve texture quality at oblique angles
   
   **f. Border Color** (if using ClampToBorder):
   - **TransparentBlack** - (0, 0, 0, 0)
   - **OpaqueBlack** - (0, 0, 0, 1)
   - **OpaqueWhite** - (1, 1, 1, 1)
   
   **g. Optional Settings**:
   - **Label**: Descriptive name (e.g., "Linear Repeat Sampler")

3. Review the **Configuration Summary**
4. Click **Create Sampler**
5. Check **Console** for confirmation

**Common Sampler Configurations**:

- **Pixel Art**: Nearest/Nearest/Nearest, Repeat, no anisotropy
- **Smooth Textures**: Linear/Linear/Linear, Repeat, anisotropy 8-16
- **UI Textures**: Linear/Linear, ClampToEdge, no anisotropy
- **Shadow Maps**: Linear/Linear, ClampToEdge, comparison function (Less or LessEqual)

## Step-by-Step Tutorials

### Tutorial 1: Rendering Your First Triangle

This tutorial walks you through creating a simple colored triangle from scratch.

#### Step 1: Verify GPU Setup

1. Launch wgpu_playground
2. On the **Adapter Selection** tab, verify a GPU adapter is selected
3. If needed, click **Create Device** to initialize the GPU
4. Check the **Console** tab - you should see "WebGPU Playground console initialized"

#### Step 2: Create a Vertex Buffer

1. Navigate to the **Buffer Config** tab
2. Configure the buffer:
   - **Size**: `60` (3 vertices × 20 bytes per vertex)
   - **Usage Flags**: Check ☑ **VERTEX** and ☑ **COPY_DST**
   - **Label**: `Triangle Vertex Buffer`
   - **Mapped at Creation**: Leave unchecked
3. Click **Create Buffer**
4. Verify in **Console**: "Buffer created: Triangle Vertex Buffer"

**Note**: Each vertex has position (8 bytes: 2 floats) + color (12 bytes: 3 floats) = 20 bytes

#### Step 3: Load a Triangle Shader

1. Navigate to the **Rendering** tab
2. In the **Example Gallery** section, find "Basic Triangle"
3. Click **Load Example**
4. Review the loaded shader code in the **WGSL Shader Editor**:

```wgsl
// Vertex shader
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(in.position, 0.0, 1.0);
    out.color = in.color;
    return out;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
```

5. Click **Compile Shader**
6. Verify in output: "Shader compiled successfully!"

#### Step 4: Configure Render Pipeline

1. Navigate to the **Render Pipeline Config** tab
2. Configure pipeline:
   - **Vertex Shader Entry Point**: `vs_main`
   - **Fragment Shader Entry Point**: `fs_main`
   - **Primitive Topology**: `TriangleList`
   - **Color Target Format**: `Bgra8Unorm` (or your swap chain format)
3. Set vertex buffer layout:
   - **Attribute 0**: `Float32x2` at offset 0 (position)
   - **Attribute 1**: `Float32x3` at offset 8 (color)
   - **Stride**: 20 bytes
4. Click **Create Pipeline**
5. Verify in **Console**: "Render pipeline created"

#### Step 5: Create Render Pass

1. Navigate to the **Render Pass Config** tab
2. Configure render pass:
   - **Color Attachment 0**: Select your render target texture
   - **Load Op**: `Clear`
   - **Clear Color**: (0.1, 0.1, 0.1, 1.0) - dark gray
   - **Store Op**: `Store`
3. Click **Create Render Pass**

#### Step 6: Record Draw Commands

1. Navigate to the **Draw Command** tab
2. Configure draw call:
   - **Vertex Buffer**: Select "Triangle Vertex Buffer"
   - **Vertex Count**: `3`
   - **Instance Count**: `1`
   - **First Vertex**: `0`
   - **First Instance**: `0`
3. Click **Record Draw**

#### Step 7: Execute and View

1. Navigate to the **Command Recording** tab
2. Review the recorded commands
3. Click **Submit Commands** to execute on GPU
4. Check **Performance** tab to see execution time
5. Check **Console** for "Render commands submitted successfully"

**Congratulations!** You've rendered your first triangle in wgpu_playground!

### Tutorial 2: Creating a Textured Quad

This tutorial demonstrates texture mapping on a 2D quad.

#### Step 1: Create a Texture

1. Navigate to the **Texture Config** tab
2. Configure texture:
   - **Width**: `256`
   - **Height**: `256`
   - **Format**: `Rgba8Unorm`
   - **Dimension**: `D2`
   - **Mip Levels**: `1`
   - **Usage Flags**: ☑ **TEXTURE_BINDING** and ☑ **COPY_DST**
   - **Label**: `Quad Texture`
3. Click **Create Texture**

#### Step 2: Create a Sampler

1. Navigate to the **Sampler Config** tab
2. Configure sampler:
   - **Address Mode U/V/W**: `Repeat`
   - **Mag Filter**: `Linear`
   - **Min Filter**: `Linear`
   - **Mipmap Filter**: `Linear`
   - **Label**: `Linear Sampler`
3. Click **Create Sampler**

#### Step 3: Create Vertex Buffer with UV Coordinates

1. Navigate to the **Buffer Config** tab
2. Configure buffer:
   - **Size**: `96` (6 vertices × 16 bytes per vertex)
   - **Usage Flags**: ☑ **VERTEX** and ☑ **COPY_DST**
   - **Label**: `Quad Vertex Buffer`
3. Click **Create Buffer**

**Note**: Each vertex has position (8 bytes: 2 floats) + UV coords (8 bytes: 2 floats) = 16 bytes

#### Step 4: Load Textured Quad Shader

1. Navigate to the **Rendering** tab
2. Load "Textured Quad" example
3. Review the shader (includes texture sampling)
4. Click **Compile Shader**

#### Step 5: Create Bind Group

1. Navigate to the **Bind Group Layout Config** tab
2. Create layout with 2 bindings:
   - **Binding 0**: Sampler, fragment stage
   - **Binding 1**: Texture, fragment stage
3. Click **Create Layout**
4. Navigate to **Bind Group Config** tab
5. Create bind group:
   - **Layout**: Select the layout created above
   - **Binding 0**: Select "Linear Sampler"
   - **Binding 1**: Select "Quad Texture"
6. Click **Create Bind Group**

#### Step 6: Configure and Execute Pipeline

1. Follow steps similar to Tutorial 1 for render pipeline
2. Ensure pipeline uses the bind group for texture access
3. Record draw commands (6 vertices for 2 triangles forming a quad)
4. Submit commands and view results

**Result**: A textured quad with the texture sampled across its surface!

### Tutorial 3: Running a Compute Shader

This tutorial shows how to use compute shaders for GPU computation.

#### Step 1: Create Storage Buffers

1. Navigate to the **Buffer Config** tab
2. Create input buffer:
   - **Size**: `1024` (256 floats × 4 bytes)
   - **Usage Flags**: ☑ **STORAGE** and ☑ **COPY_DST**
   - **Label**: `Compute Input`
3. Create output buffer:
   - **Size**: `1024`
   - **Usage Flags**: ☑ **STORAGE** and ☑ **COPY_SRC**
   - **Label**: `Compute Output`

#### Step 2: Write Compute Shader

1. Navigate to the **Rendering** tab (shader editor)
2. Load or write a compute shader:

```wgsl
@group(0) @binding(0) var<storage, read> input: array<f32>;
@group(0) @binding(1) var<storage, read_write> output: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if (index < arrayLength(&input)) {
        output[index] = input[index] * 2.0;
    }
}
```

3. Click **Compile Shader**

#### Step 3: Configure Compute Pipeline

1. Navigate to the **Compute Pipeline Config** tab
2. Configure pipeline:
   - **Entry Point**: `main`
   - **Shader Module**: Select compiled shader
3. Create bind group layout and bind group (similar to Tutorial 2)
4. Click **Create Pipeline**

#### Step 4: Dispatch Compute Workgroups

1. Navigate to the **Compute Dispatch** tab
2. Configure dispatch:
   - **Workgroup Count X**: `4` (256 items / 64 workgroup size)
   - **Workgroup Count Y**: `1`
   - **Workgroup Count Z**: `1`
3. Click **Dispatch**

#### Step 5: Read Results

1. Navigate to the **Resource Inspector** tab
2. Select "Compute Output" buffer
3. Click **Read Buffer** to copy data from GPU to CPU
4. View the results in the inspector

**Result**: The compute shader doubled all input values and stored them in the output buffer!

## Advanced Features

### Resource Inspector

The **Resource Inspector** tab provides detailed views of all GPU resources you've created:

**Features**:
- List all buffers, textures, samplers, bind groups, and pipelines
- View resource properties (size, format, usage flags, labels)
- Inspect buffer contents (read data from GPU to CPU)
- View texture data and dimensions
- Track resource lifetime and usage

**How to Use**:
1. Navigate to **Resource Inspector** tab
2. Select resource type (Buffers, Textures, Samplers, etc.)
3. Click a resource to view details
4. For buffers: Click **Read Buffer** to view contents
5. For textures: Click **Download Texture** to save to file

### Performance Monitoring

The **Performance** tab tracks GPU performance metrics:

**Metrics Displayed**:
- Frame time (milliseconds per frame)
- FPS (frames per second)
- GPU queue submission time
- Command encoding time
- Pipeline compilation time
- Buffer/texture creation time

**How to Use**:
1. Navigate to **Performance** tab
2. Start your rendering/compute operations
3. Observe real-time performance graphs
4. Identify bottlenecks and optimization opportunities

**Tips**:
- Look for spikes in frame time indicating performance issues
- Long compilation times may indicate shader complexity
- High queue submission time may indicate CPU bottleneck

### Command Recording

The **Command Recording** tab lets you record complex command sequences:

**Features**:
- Record multiple render passes
- Record compute dispatches
- Combine rendering and compute in one sequence
- Save and replay command sequences
- Debug command execution order

**How to Use**:
1. Navigate to **Command Recording** tab
2. Click **Start Recording**
3. Perform operations (draw calls, compute dispatches, etc.)
4. Click **Stop Recording**
5. Review recorded commands in the list
6. Click **Replay Commands** to execute
7. Click **Save Commands** to save for later

### Console and Error Handling

The **Console** tab is essential for debugging:

**Message Types**:
- **INFO** (blue) - Informational messages
- **WARNING** (yellow) - Non-critical issues
- **ERROR** (red) - Critical errors

**Common Messages**:
- "Buffer created: [name]" - Buffer creation success
- "Shader compiled successfully" - Shader compilation success
- "Validation Error: ..." - WebGPU validation errors
- "Device Lost: ..." - GPU device errors

**Tips**:
- Always check Console after operations
- Validation errors indicate incorrect API usage
- Device lost errors may require restarting the application

## Troubleshooting

### Application Won't Start

**Problem**: Application crashes or won't launch

**Solutions**:
1. **Verify GPU support**: Ensure your GPU supports Vulkan, Metal, or DirectX 12
2. **Update GPU drivers**: Install latest drivers from manufacturer
3. **Check display system**: On Linux, ensure X11 or Wayland is running
4. **Run with specific backend**:
   ```bash
   WGPU_BACKEND=vulkan cargo run --release
   ```
5. **Check build**:
   ```bash
   cargo clean
   cargo build --release
   ```

### DirectX 12 Errors on Windows

**Problem**: Console shows "INVALID_SUBRESOURCE_STATE" errors

**Solution**: Use Vulkan backend instead:
```bash
WGPU_BACKEND=vulkan cargo run --release
```

Or set environment variable permanently in Windows:
```cmd
setx WGPU_BACKEND vulkan
```

This is a known wgpu DirectX 12 backend issue. Vulkan works better on Windows.

### Shader Compilation Errors

**Problem**: Shader won't compile

**Solutions**:
1. **Check syntax**: Review error message for line number and syntax error
2. **Verify entry points**: Ensure `@vertex`, `@fragment`, or `@compute` are used correctly
3. **Check types**: WGSL is strict about types (f32, i32, u32, vec2, vec3, etc.)
4. **Validate bindings**: Ensure `@group(0) @binding(0)` numbers match bind group
5. **Review examples**: Load a working example and modify incrementally

**Common Errors**:
- "Expected ';'" - Missing semicolon
- "Unknown identifier" - Typo in variable/function name
- "Type mismatch" - Wrong type used (e.g., vec2 vs vec3)
- "Invalid location" - Vertex input location doesn't match buffer layout

### Buffer Creation Fails

**Problem**: Buffer creation error in Console

**Solutions**:
1. **Check size**: Ensure size > 0
2. **Verify usage flags**: At least one usage flag must be set
3. **Check limits**: Navigate to **Device Info** tab and verify buffer size doesn't exceed `max_buffer_size`
4. **Valid flag combinations**:
   - MAP_READ requires COPY_DST
   - MAP_WRITE requires COPY_SRC
   - VERTEX and INDEX cannot be used together

### Texture Creation Fails

**Problem**: Texture creation error in Console

**Solutions**:
1. **Check dimensions**: Ensure width, height > 0
2. **Verify format**: Check format is supported (see **Device Info** > **Supported Features**)
3. **Check limits**: Ensure dimensions don't exceed `max_texture_dimension_2d` (see **Device Info**)
4. **Valid usage combinations**:
   - RENDER_ATTACHMENT requires color or depth format
   - STORAGE_BINDING requires specific formats (Rgba8Unorm, Rgba32Float, etc.)

### Performance Issues

**Problem**: Application is slow or laggy

**Solutions**:
1. **Build in release mode**: Always use `cargo run --release`
2. **Check GPU selection**: Navigate to **Adapter Selection** and choose discrete GPU
3. **Monitor Performance tab**: Identify bottlenecks
4. **Reduce texture sizes**: Large textures (4K+) can slow down rendering
5. **Optimize shaders**: Complex shaders increase compilation and execution time
6. **Limit resource creation**: Avoid creating resources every frame

### "Device Lost" Errors

**Problem**: Console shows "Device lost" or "GPU timeout"

**Causes**:
- GPU driver crash
- Infinite loop in shader
- Too much GPU work in one command buffer
- Out of GPU memory

**Solutions**:
1. **Restart application**: Device lost requires re-initialization
2. **Check shader loops**: Ensure loops have proper termination
3. **Reduce workload**: Split large compute dispatches into smaller chunks
4. **Update GPU drivers**: Outdated drivers can cause stability issues
5. **Check GPU memory**: Reduce texture sizes or buffer counts

### Linux-Specific Issues

**Problem**: Application won't run on Linux

**Solutions**:
1. **Install Vulkan**: 
   ```bash
   # Ubuntu/Debian
   sudo apt-get install libvulkan-dev vulkan-tools
   
   # Check Vulkan support
   vulkaninfo
   ```
2. **Verify display**:
   ```bash
   echo $DISPLAY  # Should output :0 or similar
   echo $WAYLAND_DISPLAY  # Check if Wayland is running
   ```
3. **Run with Vulkan**:
   ```bash
   WGPU_BACKEND=vulkan cargo run --release
   ```

## Additional Resources

### Documentation

- **[README.md](../README.md)** - Project overview and features
- **[SHADER_EDITOR.md](SHADER_EDITOR.md)** - Complete shader editor guide
- **[WEBGPU_IMPLEMENTATIONS.md](WEBGPU_IMPLEMENTATIONS.md)** - WebGPU implementation details
- **[API Documentation](https://telecos.github.io/wgpu_playground/)** - Auto-generated API docs

### Examples

Run standalone examples to learn specific features:

```bash
# Basic triangle
cargo run --package wgpu_playground_examples --example triangle

# Texture mapping
cargo run --package wgpu_playground_examples --bin texture_mapping

# 3D rotating cube
cargo run --package wgpu_playground_examples --example rotating_cube

# Multi-pass rendering
cargo run --package wgpu_playground_examples --example render_to_texture

# Compute-render sharing
cargo run --package wgpu_playground_examples --example compute_render_sharing
```

### External Resources

- **[WebGPU Specification](https://www.w3.org/TR/webgpu/)** - Official WebGPU standard
- **[WGSL Specification](https://www.w3.org/TR/WGSL/)** - WebGPU Shading Language
- **[wgpu Repository](https://github.com/gfx-rs/wgpu)** - wgpu implementation
- **[WebGPU Samples](https://webgpu.github.io/webgpu-samples/)** - Official WebGPU examples
- **[Learn wgpu](https://sotrh.github.io/learn-wgpu/)** - Comprehensive wgpu tutorial

## Getting Help

If you encounter issues not covered in this guide:

1. **Check Console tab** - Error messages provide valuable debugging info
2. **Review examples** - Load and study working examples
3. **Check Device Info** - Verify GPU capabilities and limits
4. **Search issues** - Check [GitHub issues](https://github.com/telecos/wgpu_playground/issues)
5. **Ask for help** - Open a new issue with:
   - Operating system and version
   - GPU model and driver version
   - Steps to reproduce the issue
   - Console output and error messages

Happy experimenting with WebGPU! 🚀
