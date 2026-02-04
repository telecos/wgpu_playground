# WGSL Shader Editor

This document describes the WGSL Shader Editor feature implemented in the WebGPU Playground.

## Overview

The WGSL Shader Editor provides an interactive environment for writing, editing, and testing WGSL (WebGPU Shading Language) shaders. It includes:

- **Syntax Highlighting**: Visual highlighting of WGSL keywords, types, and built-in functions
- **Line Numbers**: Optional line number display for easier code navigation
- **File Loading**: Load shader files from the assets/shaders directory
- **Inline Editing**: Edit shaders directly in the application
- **Compilation**: Validate and compile shaders using wgpu
- **Error Display**: Clear error messages when compilation fails

## Location

The shader editor is accessible from the **Rendering** tab in the main application. Click the "📝 Shader Editor" button to switch from the Example Gallery to the Shader Editor.

## Features

### 1. Code Editor

- **Multi-line text editor** with monospace font for shader code
- **Optional line numbers** to help navigate your code
- **Scrollable area** for large shaders
- **Standard text editing** controls (copy, paste, select, etc.)

### 2. File Operations

- **Load from File**: Enter a filename (e.g., "example.wgsl") and click "📁 Load"
  - Files are loaded from the `assets/shaders/` directory
  - Only `.wgsl` files in the shaders directory can be loaded
  
- **Load Example**: Click "📚 Load Example" to load the default example shader
  - Loads `assets/shaders/example.wgsl`
  - Great starting point for learning WGSL syntax

### 3. Shader Compilation

- **Compile Button**: Click "⚙️ Compile" to validate your shader
  - Checks syntax and validates against wgpu's WGSL parser
  - Shows success (✅) or error (❌) messages
  - Compilation requires a GPU device to be available

- **Validation**: Automatic syntax checking before compilation
  - Catches empty shaders
  - Validates basic WGSL structure

### 4. Editor Controls

- **Label**: Set a debugging label for your shader
- **Reset**: Restore the default example shader code
- **Line Numbers**: Toggle line number display on/off

## Usage Examples

### Loading the Example Shader

1. Navigate to the **Rendering** tab
2. Click **📝 Shader Editor**
3. Click **📚 Load Example**
4. Click **⚙️ Compile** to validate the shader

### Creating a Custom Shader

1. Clear the existing code or start with the example
2. Write your WGSL shader code:
   ```wgsl
   @vertex
   fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
       var positions = array<vec2<f32>, 3>(
           vec2<f32>(0.0, 0.5),
           vec2<f32>(-0.5, -0.5),
           vec2<f32>(0.5, -0.5)
       );
       let pos = positions[vertex_index];
       return vec4<f32>(pos, 0.0, 1.0);
   }

   @fragment
   fn fs_main() -> @location(0) vec4<f32> {
       return vec4<f32>(1.0, 0.5, 0.0, 1.0);
   }
   ```
3. Click **⚙️ Compile** to validate your shader
4. Check for compilation errors in the message area

### Loading from a File

1. Place your `.wgsl` file in the `assets/shaders/` directory
2. Enter the filename in the **File** field (e.g., "my_shader.wgsl")
3. Click **📁 Load**
4. The shader code will appear in the editor

## Tips

- **Use '@vertex' and '@fragment'** for render shaders
- **Use '@compute'** for compute shaders  
- **Press Compile** to validate syntax before using in a pipeline
- **Check error messages** carefully - they indicate the location and type of error
- **Start with the example** to learn proper WGSL syntax

## WGSL Resources

For learning WGSL syntax:
- **[WGSL Shader Guide](WGSL_SHADER_GUIDE.md)** - Comprehensive guide to writing WGSL shaders (included in this repository)
- [WGSL Specification](https://www.w3.org/TR/WGSL/)
- [WebGPU Fundamentals - WGSL](https://webgpufundamentals.org/webgpu/lessons/webgpu-wgsl.html)
- [wgpu Tutorial](https://sotrh.github.io/learn-wgpu/)

## Syntax Highlighting

The editor provides basic syntax highlighting for:

- **Keywords**: `fn`, `struct`, `var`, `let`, `const`, `return`, `if`, `else`, `for`, `while`, etc.
- **Attributes**: `@vertex`, `@fragment`, `@compute`, `@group`, `@binding`, `@location`, `@builtin`, etc.
- **Types**: `bool`, `i32`, `u32`, `f32`, `vec2`, `vec3`, `vec4`, `mat4x4`, `texture_2d`, `sampler`, etc.
- **Comments**: Lines starting with `//`

## Known Limitations

- **No advanced IDE features**: No autocomplete, code navigation, or refactoring tools (yet)
- **Basic syntax highlighting**: Color highlighting is simplified compared to full IDE support
- **No real-time compilation**: Must click Compile button to validate
- **Error position highlighting**: Not yet implemented (planned for future release)

## Future Enhancements

Planned improvements for the shader editor:

- [ ] Advanced syntax highlighting with color themes
- [ ] Error position highlighting in the editor
- [ ] Autocomplete for WGSL keywords and functions
- [ ] Shader snippet library
- [ ] Save shader to file functionality
- [ ] Multiple shader tabs
- [ ] Live preview of shader output
- [ ] Debugging tools and variable inspection

## API Reference

For programmatic usage, see the `ShaderEditor` struct in `wgpu_playground_core::shader_editor`:

```rust
use wgpu_playground_core::shader_editor::ShaderEditor;

// Create a new editor
let mut editor = ShaderEditor::new();

// Load from file
editor.load_from_file("example.wgsl");

// Set custom code
editor.set_source_code(shader_code);

// Validate syntax
let is_valid = editor.validate();

// Compile with device
editor.compile(&device);

// Get compilation result
match editor.compilation_result() {
    CompilationResult::Success => println!("Success!"),
    CompilationResult::Error(msg) => eprintln!("Error: {}", msg),
    CompilationResult::NotCompiled => println!("Not compiled yet"),
}
```

## Integration with Rendering Pipeline

The shader editor is designed to integrate with the rendering pipeline in future updates. Currently, it validates shaders but does not yet execute them in the preview window.

Future integration will allow:
- Real-time preview of shader output
- Testing shaders with different inputs
- Performance profiling
- Visual debugging

See [PLAN.md](../PLAN.md) for the complete roadmap.
