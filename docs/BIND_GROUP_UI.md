# Bind Group UI Documentation

## Overview

The Bind Group UI provides an interface for creating bind groups by selecting layouts and binding resources. This UI is accessible from the "🔗 Bind Group Config" tab in the WebGPU Playground.

## Features

### 1. Create Layout Mode

In this mode, users can:

- **Define Bind Group Layout**:
  - Set an optional label for the layout
  - Add binding entries with different types:
    - Uniform Buffer
    - Storage Buffer (Read-Only)
    - Storage Buffer (Read-Write)
    - Texture
    - Sampler
    - Storage Texture

- **Configure Binding Entries**:
  - Each entry has a unique binding number (auto-incremented)
  - Configure shader stage visibility:
    - Vertex stage
    - Fragment stage
    - Compute stage
  - Remove unwanted entries

- **Validate Layout**:
  - Ensures at least one binding entry exists
  - Verifies each binding is visible in at least one shader stage
  - Checks for duplicate binding numbers

### 2. Bind Resources Mode

In this mode, users can:

- **View Layout Summary**:
  - See the layout name
  - View number of bindings

- **Assign Resources to Bindings**:
  - For each binding slot, the UI displays:
    - Binding number
    - Expected resource type
    - Current assignment status
  
- **Select Resources**:
  - **For Buffer bindings**: Choose from available buffers showing:
    - Buffer name
    - Size in bytes
    - Usage flags
  
  - **For Texture bindings**: Choose from available textures showing:
    - Texture name
    - Format
    - Dimensions
  
  - **For Sampler bindings**: Choose from available samplers showing:
    - Sampler name
    - Filter mode

- **Validate Bindings**:
  - Ensures all binding slots have assigned resources
  - Validates resource types match the layout expectations

## UI Layout

```
┌──────────────────────────────────────────────────────────┐
│ 🔗 Bind Group Configuration                             │
│ Create bind group layouts and assign resources          │
├──────────────────────────────────────────────────────────┤
│ [1️⃣ Create Layout] [2️⃣ Bind Resources]                 │
├──────────────────────────────────────────────────────────┤
│                                                          │
│ Mode-specific content (see below)                       │
│                                                          │
├──────────────────────────────────────────────────────────┤
│ Validation messages / Success messages                  │
└──────────────────────────────────────────────────────────┘
```

### Create Layout Mode View

```
┌──────────────────────────────────────────────────────────┐
│ Bind Group Layout                                        │
│ Label: [text input                                    ]  │
└──────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────┐
│ Binding Entries                                          │
│ Define the layout slots for resources:                  │
│                                                          │
│ Binding | Type             | Vertex | Fragment | Compute│
│ ───────┼──────────────────┼────────┼──────────┼─────────│
│ 0      | Uniform Buffer   |   ☑    |    ☑     |    ☐   │🗑│
│ 1      | Texture          |   ☐    |    ☑     |    ☐   │🗑│
│ 2      | Sampler          |   ☐    |    ☑     |    ☐   │🗑│
│                                                          │
│ Add new binding:                                         │
│ [Uniform Buffer] [Storage Buffer (RO)] [Storage Buffer]  │
│ [Texture] [Sampler] [Storage Texture]                    │
└──────────────────────────────────────────────────────────┘

[🔍 Validate Layout] [🔄 Reset]
```

### Bind Resources Mode View

```
┌──────────────────────────────────────────────────────────┐
│ Bind Group                                               │
│ Label: [text input                                    ]  │
└──────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────┐
│ Layout Summary                                           │
│ Layout: my_layout                                        │
│ Bindings: 3                                              │
└──────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────┐
│ Resource Assignments                                     │
│ Assign resources to each binding slot:                  │
│                                                          │
│ ┌────────────────────────────────────────────────────┐  │
│ │ Binding 0: (Uniform Buffer)                        │  │
│ │ Assigned: Uniform Buffer 0                         │  │
│ │                                                     │  │
│ │ Available Buffers:                                 │  │
│ │ [Uniform Buffer 0]                                 │  │
│ │   256 bytes, UNIFORM | COPY_DST                    │  │
│ │ [Storage Buffer 1]                                 │  │
│ │   1024 bytes, STORAGE | COPY_SRC                   │  │
│ │ [Vertex Buffer]                                    │  │
│ │   512 bytes, VERTEX | COPY_DST                     │  │
│ └────────────────────────────────────────────────────┘  │
│                                                          │
│ ┌────────────────────────────────────────────────────┐  │
│ │ Binding 1: (Texture)                               │  │
│ │ ⚠ Not assigned                                     │  │
│ │                                                     │  │
│ │ Available Textures:                                │  │
│ │ [Color Texture]                                    │  │
│ │   Rgba8Unorm, 256x256                              │  │
│ │ [Depth Texture]                                    │  │
│ │   Depth32Float, 512x512                            │  │
│ └────────────────────────────────────────────────────┘  │
│                                                          │
│ ┌────────────────────────────────────────────────────┐  │
│ │ Binding 2: (Sampler)                               │  │
│ │ ⚠ Not assigned                                     │  │
│ │                                                     │  │
│ │ Available Samplers:                                │  │
│ │ [Linear Sampler]                                   │  │
│ │   Filter: Linear                                   │  │
│ │ [Nearest Sampler]                                  │  │
│ │   Filter: Nearest                                  │  │
│ └────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────┘

[🔍 Validate Bindings] [✨ Create Bind Group]
```

## Workflow

1. **Create a Bind Group Layout**:
   - Switch to "Create Layout" mode
   - Add binding entries for your resources
   - Configure shader stage visibility for each binding
   - Validate the layout

2. **Bind Resources**:
   - Switch to "Bind Resources" mode
   - Review the layout summary
   - For each binding slot, select an appropriate resource
   - Validate that all bindings are assigned

3. **Create Bind Group**:
   - Click "Create Bind Group" to finalize
   - The system validates and would create the actual bind group (in a full implementation with GPU access)

## Mock Resources

The UI includes mock resources for demonstration:

### Buffers
- **Uniform Buffer 0**: 256 bytes, UNIFORM | COPY_DST
- **Storage Buffer 1**: 1024 bytes, STORAGE | COPY_SRC
- **Vertex Buffer**: 512 bytes, VERTEX | COPY_DST

### Textures
- **Color Texture**: Rgba8Unorm, 256x256
- **Depth Texture**: Depth32Float, 512x512

### Samplers
- **Linear Sampler**: Linear filter mode
- **Nearest Sampler**: Nearest filter mode

## Implementation Details

The BindGroupPanel implementation includes:

- **Type-safe binding type configuration**: Ensures correct resource types for each binding
- **Shader stage management**: Fine-grained control over where bindings are visible
- **Validation**: Comprehensive validation of both layouts and resource assignments
- **User-friendly interface**: Clear visual feedback and easy resource selection
- **Mock data**: Pre-populated with example resources for testing and demonstration

## Testing

The implementation includes comprehensive unit tests covering:

- Panel creation and initialization
- Adding and removing binding entries
- Layout validation
- Resource assignment
- Shader stage configuration
- UI mode switching

All tests pass successfully, ensuring the UI logic is correct and reliable.
