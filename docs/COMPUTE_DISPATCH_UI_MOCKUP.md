# Compute Dispatch UI Layout

```
┌─────────────────────────────────────────────────────────────────────┐
│  🎮 WebGPU Playground                                               │
├───────────────┬─────────────────────────────────────────────────────┤
│ Navigation    │  🧮 Compute Dispatch Configuration                  │
│               │                                                      │
│ ⚙️ Adapter    │  Configure and preview compute dispatch parameters  │
│ 🔧 Device     │  for GPU compute operations.                        │
│ 📊 Info       │                                                      │
│ 🎨 Rendering  │  ┌─────────────────────────────────────────────┐   │
│ 📐 Buffer     │  │ Dispatch Type Selection                     │   │
│ 🎨 Sampler    │  ├─────────────────────────────────────────────┤   │
│ 🖼️  Texture    │  │ [Direct] [Indirect]                        │   │
│ 🔗 Bind Group │  │                                             │   │
│ ⚙️ Compute    │  │ Dispatch with explicit workgroup counts    │   │
│   Pipeline    │  │ for X, Y, Z dimensions                      │   │
│ ⚡ Render     │  └─────────────────────────────────────────────┘   │
│   Pipeline    │                                                      │
│ 📊 Draw       │  ┌─────────────────────────────────────────────┐   │
│   Command     │  │ Workgroup Counts                            │   │
│ 🎬 Render     │  ├─────────────────────────────────────────────┤   │
│   Pass        │  │                                             │   │
│ 🚀 Compute ◄──┤  │ Workgroups X:  [1_________________]        │   │
│   Dispatch    │  │ Workgroups Y:  [1_________________]        │   │
│ 🧮 Compute/ML │  │ Workgroups Z:  [1_________________]        │   │
│ 🖥️ Console    │  │                                             │   │
│               │  │ Note: Total invocations = workgroups *      │   │
│               │  │       workgroup_size (from shader)          │   │
│               │  └─────────────────────────────────────────────┘   │
│               │                                                      │
│               │  ┌─────────────────────────────────────────────┐   │
│               │  │ Actions                                     │   │
│               │  ├─────────────────────────────────────────────┤   │
│               │  │ [✓ Validate]  [🔄 Reset]                   │   │
│               │  └─────────────────────────────────────────────┘   │
│               │                                                      │
│               │  ✓ Parameters are valid!                            │
│               │                                                      │
│               │  ┌─────────────────────────────────────────────┐   │
│               │  │ Command Summary                             │   │
│               │  ├─────────────────────────────────────────────┤   │
│               │  │ Generated Dispatch Call:                    │   │
│               │  │ dispatch_workgroups(1, 1, 1)                │   │
│               │  └─────────────────────────────────────────────┘   │
│               │                                                      │
│               │  ┌─────────────────────────────────────────────┐   │
│               │  │ ℹ️  Information                              │   │
│               │  ├─────────────────────────────────────────────┤   │
│               │  │ • Workgroups X: Number of workgroups in     │   │
│               │  │   the X dimension                           │   │
│               │  │ • Workgroups Y: Number of workgroups in     │   │
│               │  │   the Y dimension                           │   │
│               │  │ • Workgroups Z: Number of workgroups in     │   │
│               │  │   the Z dimension                           │   │
│               │  │                                             │   │
│               │  │ Each workgroup executes the compute shader  │   │
│               │  │ with the workgroup size specified in the    │   │
│               │  │ shader's @workgroup_size attribute.         │   │
│               │  │                                             │   │
│               │  │ Example: @workgroup_size(64, 1, 1)          │   │
│               │  └─────────────────────────────────────────────┘   │
│               │                                                      │
└───────────────┴─────────────────────────────────────────────────────┘
```

## Indirect Dispatch Mode

```
┌─────────────────────────────────────────────────────────────────────┐
│  🎮 WebGPU Playground                                               │
├───────────────┬─────────────────────────────────────────────────────┤
│ Navigation    │  🧮 Compute Dispatch Configuration                  │
│               │                                                      │
│ ...           │  Configure and preview compute dispatch parameters  │
│               │  for GPU compute operations.                        │
│               │                                                      │
│               │  ┌─────────────────────────────────────────────┐   │
│               │  │ Dispatch Type Selection                     │   │
│               │  ├─────────────────────────────────────────────┤   │
│               │  │ [Direct] [Indirect]                         │   │
│               │  │                                             │   │
│               │  │ Dispatch with workgroup counts stored in a  │   │
│               │  │ GPU buffer                                  │   │
│               │  └─────────────────────────────────────────────┘   │
│               │                                                      │
│               │  ┌─────────────────────────────────────────────┐   │
│               │  │ Indirect Dispatch Parameters                │   │
│               │  ├─────────────────────────────────────────────┤   │
│               │  │                                             │   │
│               │  │ Indirect Buffer:       [▼ Select buffer...] │   │
│               │  │                        └─ Buffer 0 (placeholder)│
│               │  │                           Buffer 1 (placeholder)│
│               │  │                                             │   │
│               │  │ Indirect Offset:       [0_____________]     │   │
│               │  │ (bytes)                                     │   │
│               │  │                                             │   │
│               │  │ Note: Buffer must have INDIRECT usage flag  │   │
│               │  │       and contain 3 u32 values              │   │
│               │  └─────────────────────────────────────────────┘   │
│               │                                                      │
│               │  ┌─────────────────────────────────────────────┐   │
│               │  │ Actions                                     │   │
│               │  ├─────────────────────────────────────────────┤   │
│               │  │ [✓ Validate]  [🔄 Reset]                   │   │
│               │  └─────────────────────────────────────────────┘   │
│               │                                                      │
│               │  ✓ Parameters are valid!                            │
│               │                                                      │
│               │  ┌─────────────────────────────────────────────┐   │
│               │  │ Command Summary                             │   │
│               │  ├─────────────────────────────────────────────┤   │
│               │  │ Generated Dispatch Call:                    │   │
│               │  │ dispatch_workgroups_indirect(buffer_0,      │   │
│               │  │                              offset: 0)     │   │
│               │  └─────────────────────────────────────────────┘   │
│               │                                                      │
│               │  ┌─────────────────────────────────────────────┐   │
│               │  │ ℹ️  Information                              │   │
│               │  ├─────────────────────────────────────────────┤   │
│               │  │ • Indirect Buffer: Buffer containing        │   │
│               │  │   dispatch parameters                       │   │
│               │  │ • Indirect Offset: Byte offset in the       │   │
│               │  │   indirect buffer                           │   │
│               │  │                                             │   │
│               │  │ The indirect buffer must contain three      │   │
│               │  │ u32 values:                                 │   │
│               │  │   - workgroups_x (u32)                      │   │
│               │  │   - workgroups_y (u32)                      │   │
│               │  │   - workgroups_z (u32)                      │   │
│               │  │                                             │   │
│               │  │ The buffer must have the INDIRECT usage     │   │
│               │  │ flag set.                                   │   │
│               │  └─────────────────────────────────────────────┘   │
│               │                                                      │
└───────────────┴─────────────────────────────────────────────────────┘
```

## UI Flow

```
User Action Flow:
─────────────────

1. Select Tab
   └─> Click "🚀 Compute Dispatch" in sidebar

2. Choose Dispatch Type
   ├─> Option A: Direct Dispatch
   │   ├─> Enter workgroups X count
   │   ├─> Enter workgroups Y count
   │   └─> Enter workgroups Z count
   │
   └─> Option B: Indirect Dispatch
       ├─> Select an indirect buffer from dropdown
       └─> Enter indirect offset (bytes)

3. Validate Configuration
   ├─> Click "Validate"
   ├─> Success: Green checkmark message
   └─> Error: Red error message with details

4. View Command Summary
   └─> See generated dispatch call preview

5. Reset (if needed)
   └─> Click "Reset" to restore defaults
```

## Error Display Examples

```
❌ Error: Workgroups X must be a valid number

❌ Error: Workgroups Y must be greater than 0

❌ Error: Workgroups Z must be greater than 0

❌ Error: Indirect offset must be a valid number

❌ Error: Please select an indirect buffer for indirect dispatch
```

## Success Display Example

```
✓ Parameters are valid!
```

## Command Summary Examples

### Direct Dispatch
```
┌─────────────────────────────────────────┐
│ Generated Dispatch Call:                │
│ dispatch_workgroups(64, 1, 1)           │
└─────────────────────────────────────────┘
```

### Indirect Dispatch
```
┌─────────────────────────────────────────┐
│ Generated Dispatch Call:                │
│ dispatch_workgroups_indirect(buffer_0,  │
│                              offset: 0) │
└─────────────────────────────────────────┘
```

## Features

### Direct Dispatch
- **Workgroup Count Inputs**: Three text fields for X, Y, Z dimensions
- **Input Validation**: Ensures all values are valid positive numbers
- **Command Preview**: Shows the generated dispatch call
- **Tooltips**: Helpful hover text explaining each parameter
- **Information Panel**: Explains workgroup concepts and relationship with shader workgroup_size

### Indirect Dispatch
- **Buffer Selection**: Dropdown to select indirect buffer
- **Offset Input**: Text field for byte offset into the buffer
- **Buffer Validation**: Ensures a buffer is selected
- **Information Panel**: Explains indirect buffer format requirements (3 u32 values)
- **Usage Hints**: Reminds users that buffers need INDIRECT usage flag

### Common Features
- **Type Toggle**: Switch between Direct and Indirect dispatch modes
- **Validation**: Real-time parameter validation with clear error messages
- **Reset Button**: Quick way to restore default values
- **Command Summary**: Preview of the generated dispatch call
- **Context-Sensitive Help**: Information panel updates based on selected dispatch type

## Integration Notes

The Compute Dispatch panel integrates seamlessly with:
- **Compute Pipeline Configuration**: Use pipelines configured in the Compute Pipeline panel
- **Buffer Configuration**: Select buffers created in the Buffer Config panel for indirect dispatch
- **Compute/ML Panel**: Execute dispatch commands for compute operations

## Future Enhancements

Potential future improvements:
1. Integration with actual buffer list from Buffer Config panel
2. Live execution of dispatch commands
3. Performance metrics and profiling
4. Workgroup size calculator (based on shader configuration)
5. Visual representation of workgroup grid
6. GPU timing queries for dispatch operations
7. Multiple dispatch batching
