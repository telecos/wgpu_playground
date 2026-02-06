# API Reference Panel UI Mockup

This document describes the visual layout and user interface of the API Reference Panel.

## Panel Layout

```
╔═══════════════════════════════════════════════════════════════════════╗
║ 📖 WebGPU API Reference                                               ║
╠═══════════════════════════════════════════════════════════════════════╣
║                                                                         ║
║ Browse WebGPU API documentation and examples. Click on categories     ║
║ to view methods and usage examples.                                    ║
║                                                                         ║
║ ┌─────────────────────────────────────────────────────────────────┐  ║
║ │ 🔍 Search: [________________]  [Clear]                           │  ║
║ └─────────────────────────────────────────────────────────────────┘  ║
║                                                                         ║
║ ────────────────────────────────────────────────────────────────────  ║
║                                                                         ║
║ ┌──────────────────┬──────────────────────────────────────────────┐  ║
║ │ Categories       │ Category Details                              │  ║
║ │                  │                                               │  ║
║ │ Adapter          │ Device                                        │  ║
║ │ ▶ Device         │ Main interface for GPU operations. Creates   │  ║
║ │ Queue            │ resources and command encoders.               │  ║
║ │ Buffer           │                                               │  ║
║ │ Texture          │ 📄 Specification:                             │  ║
║ │ Sampler          │ https://www.w3.org/TR/webgpu/#gpu-device     │  ║
║ │ Shader Module    │                                               │  ║
║ │ Render Pipeline  │ ──────────────────────────────────────────── │  ║
║ │ Compute Pipeline │                                               │  ║
║ │ Bind Group       │ Methods                                       │  ║
║ │ Command Encoder  │                                               │  ║
║ │ Render Pass      │ ▼ create_buffer                               │  ║
║ │ Compute Pass     │   Creates a GPU buffer with the specified    │  ║
║ │                  │   size and usage flags.                       │  ║
║ │                  │                                               │  ║
║ │                  │   Signature:                                  │  ║
║ │                  │   fn create_buffer(                           │  ║
║ │                  │     &self,                                    │  ║
║ │                  │     descriptor: &BufferDescriptor             │  ║
║ │                  │   ) -> Buffer                                 │  ║
║ │                  │                                               │  ║
║ │                  │   Example:                                    │  ║
║ │                  │   let buffer = device.create_buffer(          │  ║
║ │                  │     &BufferDescriptor {                       │  ║
║ │                  │       label: Some("Vertex Buffer"),           │  ║
║ │                  │       size: 1024,                             │  ║
║ │                  │       usage: BufferUsages::VERTEX |           │  ║
║ │                  │                BufferUsages::COPY_DST,        │  ║
║ │                  │       mapped_at_creation: false,              │  ║
║ │                  │     }                                         │  ║
║ │                  │   );                                          │  ║
║ │                  │                                               │  ║
║ │                  │ ▶ create_texture                              │  ║
║ │                  │ ▶ create_shader_module                        │  ║
║ │                  │ ▶ create_render_pipeline                      │  ║
║ │                  │ ▶ create_compute_pipeline                     │  ║
║ │                  │ ▶ create_command_encoder                      │  ║
║ │                  │ ▶ create_bind_group                           │  ║
║ └──────────────────┴──────────────────────────────────────────────┘  ║
╚═══════════════════════════════════════════════════════════════════════╝
```

## Key UI Elements

### Header
- **Icon**: 📖 (book emoji) - indicates documentation/reference
- **Title**: "WebGPU API Reference" in heading font
- **Description**: Brief explanation of panel purpose

### Search Bar
- **Icon**: 🔍 (magnifying glass emoji)
- **Input Field**: Single-line text input for filtering
- **Clear Button**: Resets search and category selection

### Two-Column Layout

#### Left Column: Category List
- Width: 200px fixed
- **Scrollable**: Vertical scroll for all categories
- **Items**: Selectable category names
- **Hover**: Shows category description as tooltip
- **Selection**: Highlighted background for selected category

#### Right Column: Category Details
- **Dynamic width**: Fills remaining space
- **Scrollable**: Vertical scroll for long content

##### Category Header
- **Category Name**: Large heading
- **Description**: Brief explanation
- **Specification Link**: Clickable URL to W3C spec

##### Methods Section
- **Heading**: "Methods" in subheading font
- **Method Items**: Collapsible headers for each method

##### Method Details (when expanded)
- **Description**: What the method does
- **Signature**: Code-formatted function signature
- **Example**: Code-formatted usage example

## Color Scheme

The panel uses the application's theme colors:

### Dark Theme (Default)
- **Background**: Dark gray (#1e1e1e)
- **Text**: Light gray/white (#e0e0e0)
- **Code**: Light blue (#c8c8ff) for signatures, light green (#c8ffc8) for examples
- **Links**: Blue (#6495ed)
- **Selected**: Darker blue background (#2d4f7c)
- **Hover**: Subtle highlight (#2a2a2a)

### Light Theme
- **Background**: Light gray (#f5f5f5)
- **Text**: Dark gray (#2e2e2e)
- **Code**: Dark blue (#4040c0) for signatures, dark green (#40c040) for examples
- **Links**: Blue (#1e90ff)
- **Selected**: Light blue background (#d0e0f0)
- **Hover**: Subtle highlight (#e8e8e8)

## Interaction Patterns

### Category Selection
1. User clicks on a category name in the left column
2. Category becomes highlighted/selected
3. Right column updates to show category details
4. Previous selection is deselected

### Method Expansion
1. User clicks on a method header (▶ icon)
2. Method expands to show full details
3. Icon changes to ▼ (down arrow)
4. Click again to collapse

### Search/Filter
1. User types in search field
2. Results update in real-time
3. Only matching methods are shown
4. Clear button removes all filters

### Specification Links
1. User clicks on specification URL
2. Opens in new browser tab/window
3. On WASM: uses `window.open()`
4. On native: uses `webbrowser::open()`

## Accessibility

- **Keyboard Navigation**: Tab through categories and methods
- **Screen Readers**: Proper ARIA labels for all interactive elements
- **High Contrast**: Uses theme colors with sufficient contrast
- **Focus Indicators**: Clear visual focus states

## Responsive Design

The panel adapts to different window sizes:

- **Wide Windows** (>1200px): Full two-column layout
- **Medium Windows** (800-1200px): Narrower category list (150px)
- **Narrow Windows** (<800px): Could switch to stacked layout (future enhancement)

## Integration Points

### Access from Sidebar
```
🔧 Tools & Debugging
├── Tutorials
├── Configuration Presets
├── Resource Inspector
├── ...
├── API Coverage
└── 📖 API Reference  ← New entry
```

### Menu Item Details
- **Icon**: 📖 (matches panel icon)
- **Label**: "API Reference"
- **Tooltip**: "Browse WebGPU API documentation and examples"
- **Position**: In Tools & Debugging section, after API Coverage

## Future Enhancements

Potential UI improvements:

1. **Copy Button**: One-click copy for code examples
2. **Related APIs**: Show related methods in sidebar
3. **Breadcrumbs**: Navigation trail for deep exploration
4. **Favorites**: Star icon to bookmark frequently used APIs
5. **Recent**: Show recently viewed APIs
6. **Interactive Examples**: Click to load example into config panels
7. **Syntax Highlighting**: Full code syntax highlighting for examples
8. **Dark/Light Code Theme**: Separate code themes independent of UI theme
