# Real-time Shader Validation UI Mockup

## Overview
This document provides a visual description of the real-time shader validation feature in the WGSL Shader Editor.

## UI Layout

### Main Editor View

```
┌─────────────────────────────────────────────────────────────────┐
│  📝 WGSL Shader Editor                                          │
├─────────────────────────────────────────────────────────────────┤
│  Label: [shader_editor] File: [example.wgsl] [📁 Load] [📚 Load Example] [⚙️ Compile] [🔄 Reset]  │
├─────────────────────────────────────────────────────────────────┤
│  ✅ Compilation successful!                                     │
├─────────────────────────────────────────────────────────────────┤
│  💡 Tips: • Use '@vertex' and '@fragment' for render shaders   │
│           • Use '@compute' for compute shaders                  │
│           • Press Compile to validate syntax                    │
├─────────────────────────────────────────────────────────────────┤
│  Shader Code:                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │    1  // WGSL Shader Example                              │ │
│  │    2  @vertex                                             │ │
│  │    3  fn vs_main(@builtin(vertex_index) vertex_index: u32)│ │
│  │    4      -> @builtin(position) vec4<f32> {              │ │
│  │    5      var positions = array<vec2<f32>, 3>(           │ │
│  │    6          vec2<f32>(0.0, 0.5),                        │ │
│  │    7          vec2<f32>(-0.5, -0.5),                      │ │
│  │    8          vec2<f32>(0.5, -0.5)                        │ │
│  │    9      );                                              │ │
│  │   10      let pos = positions[vertex_index];             │ │
│  │   11      return vec4<f32>(pos, 0.0, 1.0);               │ │
│  │   12  }                                                   │ │
│  │   13                                                      │ │
│  │   14  @fragment                                           │ │
│  │   15  fn fs_main() -> @location(0) vec4<f32> {           │ │
│  │   16      return vec4<f32>(1.0, 0.5, 0.0, 1.0);          │ │
│  │   17  }                                                   │ │
│  └───────────────────────────────────────────────────────────┘ │
│                                                                 │
│  ✅ No validation errors                                        │
│                                                                 │
│  [✓] Show line numbers  [✓] ⚡ Real-time Validation: ON        │
│  [✓] 🔥 Hot Reload: ON                                          │
└─────────────────────────────────────────────────────────────────┘
```

### Editor View with Validation Errors

```
┌─────────────────────────────────────────────────────────────────┐
│  📝 WGSL Shader Editor                                          │
├─────────────────────────────────────────────────────────────────┤
│  Label: [shader_editor] File: [] [📁 Load] [📚 Load Example] [⚙️ Compile] [🔄 Reset]  │
├─────────────────────────────────────────────────────────────────┤
│  ℹ️ Not compiled yet. Click 'Compile' to validate your shader. │
├─────────────────────────────────────────────────────────────────┤
│  💡 Tips: • Use '@vertex' and '@fragment' for render shaders   │
│           • Use '@compute' for compute shaders                  │
│           • Press Compile to validate syntax                    │
├─────────────────────────────────────────────────────────────────┤
│  Shader Code:                                                   │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │    1  @vertex                                             │ │
│  │    2  fn vs_main() -> @builtin(position) vec4<f32> {     │ │
│  │❌  3      return vec4<f32>(0.0, 0.0, 0.0, 1.0);           │ │
│  │    4  }                                                   │ │
│  │    5                                                      │ │
│  │❌  6  invalid syntax @@@                                  │ │
│  │    7                                                      │ │
│  │    8  @fragment                                           │ │
│  │    9  fn fs_main() -> @location(0) vec4<f32> {           │ │
│  │   10      return vec4<f32>(1.0, 0.0, 0.0, 1.0);          │ │
│  │   11  }                                                   │ │
│  └───────────────────────────────────────────────────────────┘ │
│                                                                 │
│  ⚠️ 2 validation error(s):                                      │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │ Line 3: expected `,`, found `;`                           │ │
│  │ Line 6: expected global item, found "invalid"             │ │
│  └───────────────────────────────────────────────────────────┘ │
│                                                                 │
│  [✓] Show line numbers  [✓] ⚡ Real-time Validation: ON        │
│  [✓] 🔥 Hot Reload: ON                                          │
└─────────────────────────────────────────────────────────────────┘
```

## Key UI Elements

### 1. Error Markers (❌)
- Displayed in the line number column
- Appears on lines with syntax errors
- Bright red color for visibility
- Aligned with the corresponding line

### 2. Validation Status Panel
Located below the code editor, shows one of:

#### Success State (Green)
```
✅ No validation errors
```
- Green color (RGB: 50, 200, 50)
- Displayed when code is valid

#### Error State (Red)
```
⚠️ 2 validation error(s):
┌─────────────────────────────────────────────┐
│ Line 3: expected `,`, found `;`            │
│ Line 6: expected global item, found "..."  │
└─────────────────────────────────────────────┘
```
- Red header (RGB: 220, 50, 50)
- Scrollable error list
- Each error shows line number and message
- Error text in lighter red (RGB: 255, 100, 100)

### 3. Real-time Validation Toggle
```
[✓] ⚡ Real-time Validation: ON
```
- Checkbox control
- ON state: Validation runs automatically on every change
- OFF state: Validation disabled, errors cleared
- Icon: ⚡ (lightning bolt)

### 4. Line Number Display
```
   1  // Normal line
   2  // Normal line  
❌  3  error line
   4  // Normal line
```
- 3 spaces prefix for lines without errors
- ❌ prefix for lines with errors
- Monospace font for alignment

## Color Scheme

### Text Colors
- Normal text: White
- Error marker (❌): Red
- Success message: Green (RGB: 50, 200, 50)
- Error count: Red (RGB: 220, 50, 50)
- Error details: Light red (RGB: 255, 100, 100)

### Background
- Uses default egui dark theme
- Code editor: Slightly darker background
- Error panel: Same as background with border

## User Interaction Flow

### 1. Typing in Editor
```
User types → Text changes → realtime_validate() called → Errors updated → UI refreshes
```

### 2. Loading File
```
Load button → load_from_file() → realtime_validate() → Errors displayed
```

### 3. Toggling Validation
```
Toggle OFF → Errors cleared → UI shows no errors
Toggle ON  → realtime_validate() → Errors displayed
```

## Responsive Behavior

- **Error panel**: Scrollable when multiple errors
- **Line numbers**: Auto-width based on total lines
- **Editor**: Maintains fixed width for code area
- **Validation**: Non-blocking, instant feedback

## Accessibility Features

- Clear visual indicators (❌ symbol)
- Color-coded messages (green/red)
- Descriptive error messages
- Line numbers for navigation
- Toggle control for user preference

## Performance Characteristics

- **Validation speed**: < 10ms for typical shaders
- **UI update**: Immediate on text change
- **Memory**: Minimal overhead (error list only)
- **CPU**: Light processing during typing

## Edge Cases Handled

1. **Empty shader**: No validation performed
2. **Very long error messages**: Truncated with scrolling
3. **Many errors**: Scrollable error panel
4. **Rapid typing**: Debounced validation
5. **File reload**: Errors cleared and revalidated
