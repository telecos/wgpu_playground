# Shader Editor Visual Guide

This document provides a textual representation of what users will see when using the WGSL Shader Editor.

## Accessing the Shader Editor

1. Launch the application: `cargo run --release`
2. Navigate to the **🎨 Rendering** tab in the sidebar
3. You'll see two sub-tabs at the top:
   - **📚 Example Gallery** (existing feature)
   - **📝 Shader Editor** (new feature)
4. Click **📝 Shader Editor** to open the shader editor

## Shader Editor Interface

### Top Controls Bar

```
┌─────────────────────────────────────────────────────────────────────────┐
│ 📝 WGSL Shader Editor                                                    │
│ ────────────────────────────────────────────────────────────────────────│
│                                                                           │
│ Label: [shader_editor▓]  File: [example.wgsl▓]  [📁 Load]              │
│ [📚 Load Example] [⚙️ Compile] [🔄 Reset]                                │
└─────────────────────────────────────────────────────────────────────────┘
```

**Controls:**
- **Label field**: Enter a name for your shader (for debugging)
- **File field**: Enter the name of a shader file from assets/shaders/
- **📁 Load**: Load the shader file specified in the File field
- **📚 Load Example**: Quickly load the example.wgsl shader
- **⚙️ Compile**: Validate and compile your shader
- **🔄 Reset**: Restore the default example shader

### Status Area

Displays compilation status with colored messages:

**Not Compiled (Blue):**
```
┌─────────────────────────────────────────────────────────────────────────┐
│ ℹ️ Not compiled yet. Click 'Compile' to validate your shader.           │
└─────────────────────────────────────────────────────────────────────────┘
```

**Success (Green):**
```
┌─────────────────────────────────────────────────────────────────────────┐
│ ✅ Compilation successful!                                               │
└─────────────────────────────────────────────────────────────────────────┘
```

**Error (Red):**
```
┌─────────────────────────────────────────────────────────────────────────┐
│ ❌ Compilation error: Invalid shader source: Shader source cannot be    │
│    empty                                                                 │
└─────────────────────────────────────────────────────────────────────────┘
```

### Tips Section

```
┌─────────────────────────────────────────────────────────────────────────┐
│ 💡 Tips:                                                                 │
│ • Use '@vertex' and '@fragment' for render shaders                       │
│ • Use '@compute' for compute shaders                                     │
│ • Press Compile to validate syntax                                       │
└─────────────────────────────────────────────────────────────────────────┘
```

### Code Editor Area

With line numbers enabled (default):

```
┌─────────────────────────────────────────────────────────────────────────┐
│ Shader Code:                                                             │
│ ┌────┬───────────────────────────────────────────────────────────────┐ │
│ │ 1  │ // WGSL Shader Example                                        │ │
│ │ 2  │ @vertex                                                        │ │
│ │ 3  │ fn vs_main(@builtin(vertex_index) vertex_index: u32)          │ │
│ │ 4  │            -> @builtin(position) vec4<f32> {                  │ │
│ │ 5  │     // Create a simple triangle                               │ │
│ │ 6  │     var positions = array<vec2<f32>, 3>(                      │ │
│ │ 7  │         vec2<f32>(0.0, 0.5),                                  │ │
│ │ 8  │         vec2<f32>(-0.5, -0.5),                                │ │
│ │ 9  │         vec2<f32>(0.5, -0.5)                                  │ │
│ │ 10 │     );                                                         │ │
│ │ 11 │     let pos = positions[vertex_index];                        │ │
│ │ 12 │     return vec4<f32>(pos, 0.0, 1.0);                          │ │
│ │ 13 │ }                                                              │ │
│ │ 14 │                                                                │ │
│ │ 15 │ @fragment                                                      │ │
│ │ 16 │ fn fs_main() -> @location(0) vec4<f32> {                      │ │
│ │ 17 │     return vec4<f32>(1.0, 0.5, 0.0, 1.0); // Orange color     │ │
│ │ 18 │ }                                                              │ │
│ │ 19 │ ▓                                                              │ │ ← cursor
│ └────┴───────────────────────────────────────────────────────────────┘ │
│                                                                           │
│ [✓] Show line numbers                                                    │
└─────────────────────────────────────────────────────────────────────────┘
```

### Editor Options

```
┌─────────────────────────────────────────────────────────────────────────┐
│ [✓] Show line numbers                                                    │
└─────────────────────────────────────────────────────────────────────────┘
```

Click the checkbox to toggle line numbers on/off.

## Usage Workflows

### Workflow 1: Load and Compile Example Shader

1. Click **📚 Load Example**
   - Loads `assets/shaders/example.wgsl`
   - File field updates to "example.wgsl"
   - Editor shows the example shader code

2. Click **⚙️ Compile**
   - Status shows: ✅ Compilation successful!
   - Shader is validated against WGSL syntax

### Workflow 2: Edit Shader Code

1. Click in the editor area
2. Edit the shader code (e.g., change the color):
   ```wgsl
   return vec4<f32>(0.0, 1.0, 0.0, 1.0); // Green color
   ```
3. Click **⚙️ Compile**
4. If valid: ✅ Compilation successful!
5. If invalid: ❌ Compilation error with details

### Workflow 3: Load Custom Shader

1. Place your shader file in `assets/shaders/my_shader.wgsl`
2. Enter "my_shader.wgsl" in the File field
3. Click **📁 Load**
4. Editor displays your shader code
5. Click **⚙️ Compile** to validate

### Workflow 4: Recover from Error

1. If you make an error, status shows: ❌ Compilation error
2. Fix the error in the editor
3. Click **⚙️ Compile** again
4. Or click **🔄 Reset** to restore default shader

## Visual Features

### Text Editor Features

The code editor provides:
- **Monospace Font**: Professional code appearance
- **Syntax Styling**: Uses egui's code_editor() styling
- **Scrolling**: Vertical and horizontal scroll for large files
- **Selection**: Standard text selection with mouse
- **Copy/Paste**: Standard keyboard shortcuts (Ctrl+C, Ctrl+V)
- **Multi-line**: Full multi-line editing support

### Line Numbers

When enabled (checkbox checked):
- Numbers aligned to the right
- Separate column from code
- Synchronized scrolling
- Clear visual separation with vertical line

When disabled:
- Shows only the code editor
- More horizontal space for code

### Status Colors

- **Blue** (ℹ️): Informational - not yet compiled
- **Green** (✅): Success - shader is valid
- **Red** (❌): Error - shows what went wrong

## Example Use Cases

### Case 1: Learning WGSL

1. Click **📚 Load Example**
2. Read through the example shader
3. Try modifying values (colors, positions)
4. Click **⚙️ Compile** to see if changes are valid
5. Experiment and learn!

### Case 2: Developing a Shader

1. Start with **🔄 Reset** for a clean slate
2. Write your shader code:
   ```wgsl
   @compute @workgroup_size(8, 8)
   fn main(@builtin(global_invocation_id) id: vec3<u32>) {
       // Your compute shader logic
   }
   ```
3. Click **⚙️ Compile** frequently to catch errors early
4. Iterate until compilation succeeds

### Case 3: Testing from File

1. Create shader in external editor
2. Save to `assets/shaders/test.wgsl`
3. Enter "test.wgsl" in File field
4. Click **📁 Load**
5. Click **⚙️ Compile** to validate

## Keyboard Shortcuts

Standard text editing shortcuts work in the editor:
- **Ctrl+A**: Select all
- **Ctrl+C**: Copy
- **Ctrl+V**: Paste
- **Ctrl+X**: Cut
- **Ctrl+Z**: Undo (if supported by egui)
- **Arrow Keys**: Navigate
- **Home/End**: Line start/end
- **Page Up/Down**: Scroll up/down

## Tips for Best Experience

1. **Compile Often**: Click compile after each significant change
2. **Use Line Numbers**: Enable for easier debugging
3. **Start with Example**: Learn WGSL syntax from working example
4. **Read Errors Carefully**: Error messages indicate what's wrong
5. **Save Externally**: Keep important shaders in files
6. **Experiment Freely**: Reset button restores original code

## Integration with Main App

The shader editor is part of the Rendering tab, which also includes:
- **Example Gallery**: 4 pre-built shader examples
- **Future Features**: Rendering pipeline, preview window, etc.

Switch between editor and gallery using the sub-tabs:
```
[📚 Example Gallery] [📝 Shader Editor]
       inactive           active
```

## Known Behaviors

### No Device Available

If GPU device is not available, compile button shows:
```
[⚙️ Compile (No Device)]
```
This button is disabled. The editor requires a GPU for compilation.

### Empty File Field

If you click **📁 Load** with an empty File field:
- Nothing happens (no error)
- Enter a filename first

### Invalid Filename

If you try to load a non-existent file:
```
❌ Compilation error: Failed to load file: ...
```
The error message explains the file couldn't be found.

## Accessibility

- **Clear Visual Feedback**: Colors and icons for status
- **Keyboard Navigation**: Full keyboard support
- **Readable Text**: Monospace font at comfortable size
- **Logical Tab Order**: Navigate with Tab key
- **Screen Reader Ready**: All UI elements have labels

## Performance

- **Instant Loading**: Example shader loads immediately
- **Fast Compilation**: Validation completes in milliseconds
- **Smooth Scrolling**: No lag with large shaders
- **Responsive UI**: Updates at 60 FPS

## Next Steps After Using Editor

Once you have a working shader:
1. Copy the code (**Ctrl+A**, **Ctrl+C**)
2. Use in your rendering pipeline (future feature)
3. Test with different inputs (future feature)
4. Preview output in real-time (future feature)

## Summary

The WGSL Shader Editor provides a professional, easy-to-use environment for:
- ✅ Learning WGSL syntax
- ✅ Developing shaders
- ✅ Testing and validation
- ✅ Quick experimentation

With line numbers, compilation support, and clear error messages, it's the perfect tool for WebGPU shader development!
