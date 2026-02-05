# Coverage Badges UI Mockup

This document shows a visual representation of how the coverage badges appear in the examples gallery.

## Example Gallery Layout

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        WebGPU Playground - Examples Gallery                 │
├──────────────────────┬──────────────────────────────────────────────────────┤
│  Examples            │  Controls                                            │
├──────────────────────┼──────────────────────────────────────────────────────┤
│                      │                                                      │
│  🎨 Basic Triangle  │  Description:                                        │
│  🎨 Rotating Cube    │  Renders a simple colored triangle using vertex     │
│  🎨 Texture Mapping  │  buffers and a basic shader. This is the classic    │
│  🧮 Compute Shader   │  "Hello World" of graphics programming...           │
│                      │                                                      │
│                      │  WebGPU APIs Covered:                                │
│                      │  ┌────────┐ ┌────────┐ ┌──────────────┐            │
│                      │  │ Buffer │ │ Shader │ │ Render Pass  │            │
│                      │  └────────┘ └────────┘ └──────────────┘            │
│                      │  ┌────────────────┐ ┌────────────────┐             │
│                      │  │ Render Pipeline│ │ Command Encoder│             │
│                      │  └────────────────┘ └────────────────┘             │
│                      │  ┌───────┐                                          │
│                      │  │ Queue │                                          │
│                      │  └───────┘                                          │
│                      │                                                      │
│                      │  ▶ Run Example                                       │
│                      │                                                      │
└──────────────────────┴──────────────────────────────────────────────────────┘
```

## Badge Color Scheme

The badges use distinct colors to represent different API categories:

### Basic Triangle Example
- **Buffer** (Sea Green) - Data operations
- **Shader** (Steel Blue) - Shader operations
- **Render Pipeline** (Firebrick) - Rendering pipeline setup
- **Render Pass** (Crimson) - Main rendering operations
- **Command Encoder** (Dark Olive Green) - Command encoding
- **Queue** (Dark Goldenrod) - Queue submission

### Rotating Cube Example  
- **Buffer** (Sea Green)
- **Shader** (Steel Blue)
- **Render Pipeline** (Firebrick)
- **Bind Group** (Forest Green) - Resource binding
- **Render Pass** (Crimson)
- **Command Encoder** (Dark Olive Green)
- **Queue** (Dark Goldenrod)

### Texture Mapping Example
- **Buffer** (Sea Green)
- **Texture** (Dark Orange) - Texture operations
- **Sampler** (Goldenrod) - Sampler configuration
- **Shader** (Steel Blue)
- **Render Pipeline** (Firebrick)
- **Bind Group** (Forest Green)
- **Render Pass** (Crimson)
- **Command Encoder** (Dark Olive Green)
- **Queue** (Dark Goldenrod)

### Compute Shader Example
- **Buffer** (Sea Green)
- **Shader** (Steel Blue)
- **Compute Pipeline** (Medium Slate Blue) - Compute pipeline setup
- **Bind Group** (Forest Green)
- **Compute Pass** (Blue Violet) - Compute operations
- **Command Encoder** (Dark Olive Green)
- **Queue** (Dark Goldenrod)

## Badge Design

Each badge has:
- **White text** on a colored background
- **Small font size** for compactness
- **Padding** (space before/after text)
- **Proper spacing** between adjacent badges
- **Responsive wrapping** for different screen sizes

## User Experience

When a user selects an example from the list:

1. The example's description is displayed
2. Coverage badges immediately follow, showing all API categories used
3. Badges are visually scannable with distinct colors
4. Users can quickly identify which WebGPU features an example demonstrates
5. This helps users select examples based on their learning goals

## Benefits

- **Quick Identification**: Color-coding allows instant recognition of API categories
- **Educational Value**: Shows learners what APIs they'll encounter
- **Navigation Aid**: Helps users find examples covering specific topics
- **Visual Appeal**: Adds color and structure to the gallery interface
