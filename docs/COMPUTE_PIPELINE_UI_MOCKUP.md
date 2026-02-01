# Compute Pipeline UI Layout

```
┌─────────────────────────────────────────────────────────────────────┐
│  🎮 WebGPU Playground                                               │
├───────────────┬─────────────────────────────────────────────────────┤
│ Navigation    │  ⚙️ Compute Pipeline Configuration                  │
│               │                                                      │
│ ⚙️ Adapter    │  Configure and create compute pipelines for GPU     │
│ 🔧 Device     │  compute operations.                                │
│ 📊 Info       │                                                      │
│ 🎨 Rendering  │  ┌─────────────────────────────────────────────┐   │
│ 📐 Buffer     │  │ Pipeline Properties                         │   │
│ 🎨 Sampler    │  ├─────────────────────────────────────────────┤   │
│ 🖼️  Texture    │  │ Pipeline Label:  [________________]        │   │
│ 🔗 Bind Group │  │ Entry Point:     [main___________]          │   │
│ ⚙️ Compute ◄──┤  └─────────────────────────────────────────────┘   │
│   Pipeline    │                                                      │
│ 🧮 Compute/ML │  ┌─────────────────────────────────────────────┐   │
│               │  │ Shader Module                               │   │
│               │  ├─────────────────────────────────────────────┤   │
│               │  │ Configure the compute shader for this       │   │
│               │  │ pipeline.                                   │   │
│               │  │                                             │   │
│               │  │ Shader Label: [compute_shader_______]      │   │
│               │  │                                             │   │
│               │  │ Shader Source (WGSL):                      │   │
│               │  │ ┌─────────────────────────────────────┐    │   │
│               │  │ │ // Simple compute shader template  │    │   │
│               │  │ │ @compute @workgroup_size(64)       │    │   │
│               │  │ │ fn main(@builtin(global_invocation_│    │   │
│               │  │ │         id) global_id: vec3<u32>) {│    │   │
│               │  │ │     // Add your compute logic here │    │   │
│               │  │ │ }                                  │    │   │
│               │  │ │                                    │    │   │
│               │  │ │                                    │    │   │
│               │  │ │                                    │    │   │
│               │  │ └─────────────────────────────────────┘    │   │
│               │  │                                             │   │
│               │  │ Templates:                                 │   │
│               │  │ [Simple Compute] [Storage Buffer]          │   │
│               │  │ [Matrix Multiply]                          │   │
│               │  └─────────────────────────────────────────────┘   │
│               │                                                      │
│               │  ┌─────────────────────────────────────────────┐   │
│               │  │ Pipeline Layout                             │   │
│               │  ├─────────────────────────────────────────────┤   │
│               │  │ Configure how bind groups are organized in │   │
│               │  │ the pipeline.                               │   │
│               │  │                                             │   │
│               │  │ ☑ Use Auto-Generated Layout                │   │
│               │  └─────────────────────────────────────────────┘   │
│               │                                                      │
│               │  ────────────────────────────────────────────────   │
│               │                                                      │
│               │  [Validate Configuration]   [Create Pipeline]       │
│               │                              (disabled)             │
│               │                                                      │
│               │  ✓ Configuration is valid                           │
│               │                                                      │
│               │  ┌─────────────────────────────────────────────┐   │
│               │  │ ℹ️  Compute Pipeline Information            │   │
│               │  ├─────────────────────────────────────────────┤   │
│               │  │ A compute pipeline consists of:             │   │
│               │  │ • Shader Module: Contains the compute       │   │
│               │  │   shader code (WGSL)                        │   │
│               │  │ • Entry Point: The function name to execute │   │
│               │  │   (e.g., 'main')                            │   │
│               │  │ • Pipeline Layout: Defines bind group       │   │
│               │  │   organization (auto-generated or manual)   │   │
│               │  │                                             │   │
│               │  │ Compute shaders must have:                  │   │
│               │  │ • @compute attribute on the entry point     │   │
│               │  │   function                                  │   │
│               │  │ • @workgroup_size attribute specifying      │   │
│               │  │   execution dimensions                      │   │
│               │  │ • Example: @compute @workgroup_size(64,1,1) │   │
│               │  └─────────────────────────────────────────────┘   │
│               │                                                      │
└───────────────┴─────────────────────────────────────────────────────┘
```

## UI Flow

```
User Action Flow:
─────────────────

1. Select Tab
   └─> Click "⚙️ Compute Pipeline" in sidebar

2. Configure Pipeline
   ├─> Enter pipeline label (optional)
   └─> Enter entry point name

3. Set Up Shader
   ├─> Option A: Click template button
   │   ├─> Simple Compute
   │   ├─> Storage Buffer
   │   └─> Matrix Multiply
   │
   └─> Option B: Write custom WGSL code

4. Configure Layout
   └─> Use auto-generated layout (default)

5. Validate
   ├─> Click "Validate Configuration"
   ├─> Success: Green checkmark message
   └─> Error: Red error message with details

6. Create (Future)
   └─> Click "Create Pipeline" (when enabled)
```

## Error Display Examples

```
❌ Error: Shader compilation error: Expected '@compute' attribute on entry point

❌ Error: Missing entry point: compute pipeline requires an entry point

❌ Error: Entry point name cannot be empty

❌ Error: Missing shader module: compute pipeline requires a shader
```

## Success Display Example

```
✓ Compute pipeline created successfully: 'my_pipeline'

✓ Configuration is valid
```

## Template Previews

### Simple Compute Template
```
┌─────────────────────────────────────────┐
│ // Simple compute shader template      │
│ @compute @workgroup_size(64)           │
│ fn main(@builtin(global_invocation_id) │
│         global_id: vec3<u32>) {        │
│     // Add your compute logic here     │
│ }                                      │
└─────────────────────────────────────────┘
```

### Storage Buffer Template
```
┌─────────────────────────────────────────┐
│ // Compute shader with storage buffer  │
│ @group(0) @binding(0)                  │
│ var<storage, read_write> data:         │
│     array<f32>;                        │
│                                        │
│ @compute @workgroup_size(64)           │
│ fn main(@builtin(global_invocation_id) │
│         global_id: vec3<u32>) {        │
│     let index = global_id.x;           │
│     data[index] = data[index] * 2.0;   │
│ }                                      │
└─────────────────────────────────────────┘
```

### Matrix Multiply Template
```
┌─────────────────────────────────────────┐
│ // Matrix multiplication compute shader│
│ @group(0) @binding(0)                  │
│ var<storage, read> matrix_a:           │
│     array<f32>;                        │
│                                        │
│ @group(0) @binding(1)                  │
│ var<storage, read> matrix_b:           │
│     array<f32>;                        │
│                                        │
│ @group(0) @binding(2)                  │
│ var<storage, read_write> matrix_result:│
│     array<f32>;                        │
│                                        │
│ @group(0) @binding(3)                  │
│ var<uniform> dimensions: vec3<u32>;    │
│                                        │
│ @compute @workgroup_size(8, 8, 1)      │
│ fn main(...) { /* implementation */ }  │
└─────────────────────────────────────────┘
```
