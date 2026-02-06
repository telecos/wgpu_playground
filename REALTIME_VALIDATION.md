# Real-time Shader Validation Implementation

## Overview
This implementation adds real-time WGSL shader validation to the shader editor, providing immediate feedback as users type.

## Key Features

### 1. Real-time Validation
- Automatically validates shader code on every change
- Uses naga parser for accurate WGSL validation
- Can be toggled on/off via UI checkbox

### 2. Inline Error Markers
- Error markers (❌) appear next to line numbers where errors are detected
- Line numbers with errors are clearly highlighted
- Clean visual separation between error-free and error lines

### 3. Error Display Panel
- Dedicated error panel below the editor
- Shows count of validation errors
- Lists each error with line number and detailed message
- Scrollable for multiple errors
- Color-coded for visibility (red for errors, green for success)

### 4. Smart Validation
- Skips validation for empty shaders
- Optimized to avoid unnecessary revalidation
- Persists validation state across file loads

## UI Components

### Error Markers in Line Numbers
```
   1  // Valid line
❌ 5  invalid syntax
   6  // Valid line
```

### Validation Status Display
- ✅ No validation errors (green) - when shader is valid
- ⚠️ N validation error(s) (red) - when errors are present

### Toggle Control
- ⚡ Real-time Validation: ON/OFF checkbox
- Allows users to disable validation if desired

## Technical Implementation

### ValidationError Structure
```rust
pub struct ValidationError {
    pub message: String,      // Detailed error message
    pub line: usize,          // Line number (1-indexed)
    pub column: Option<usize>, // Optional column number
}
```

### Validation Flow
1. User types in the editor
2. `set_source_code()` is called with new content
3. If real-time validation is enabled, `realtime_validate()` is triggered
4. naga parser validates the WGSL code
5. Errors are extracted and stored in `validation_errors` vector
6. UI displays error markers and messages

### Integration Points
- `ShaderEditor::new()` - Initializes with validation enabled
- `ShaderEditor::set_source_code()` - Triggers validation on content change
- `ShaderEditor::load_from_file()` - Validates newly loaded files
- `ShaderEditor::ui()` - Renders error markers and validation status

## Example Usage

### Creating a Shader Editor with Validation
```rust
let mut editor = ShaderEditor::new();
// Validation is enabled by default

// Set shader code (triggers validation)
editor.set_source_code(shader_code);

// Check for errors
if !editor.validation_errors().is_empty() {
    for error in editor.validation_errors() {
        println!("Line {}: {}", error.line, error.message);
    }
}
```

### Toggling Validation
```rust
// Disable validation
editor.realtime_validation_enabled = false;

// Re-enable validation
editor.realtime_validation_enabled = true;
```

## Testing

### Unit Tests
- `test_realtime_validation_valid_shader()` - Validates correct WGSL
- `test_realtime_validation_invalid_shader()` - Detects syntax errors
- `test_realtime_validation_empty_shader()` - Handles empty input
- `test_realtime_validation_toggle()` - Tests enable/disable feature
- `test_validation_error_structure()` - Validates error data structure

### Example Test
Run the validation test example:
```bash
cargo run --package wgpu_playground_examples --example test_validation
```

## Benefits

1. **Immediate Feedback** - Errors are caught as you type
2. **Better UX** - No need to click "Compile" to see errors
3. **Educational** - Helps users learn WGSL syntax through instant feedback
4. **Productivity** - Faster iteration cycles
5. **Accuracy** - Uses same naga parser as wgpu for consistent validation

## Dependencies

Added `naga = "27.0"` to parse and validate WGSL shaders.
