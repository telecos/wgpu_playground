# Bind Group Visualization - Visual Mockup

## Screenshot Description

Since the application requires a display to run, here's a detailed description of what the visualization looks like when running:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ 🔗 Bind Group Configuration                                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  [ 1️⃣ Create Layout ]  [ 2️⃣ Bind Resources ]  [ 3️⃣ Visualization ] ◄── Active │
│                                                                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  Bind Group Flow Diagram                                                    │
│  Visual representation of how resources flow through the rendering pipeline │
│                                                                              │
│  ┌────────────────────────────────────────────────────────────────────────┐ │
│  │                    My Bind Group Layout                                 │ │
│  │                                                                          │ │
│  │  Pipeline Stages   Bindings              Resources                      │ │
│  │  ────────────────  ─────────────────────  ───────────────────────       │ │
│  │                                                                          │ │
│  │  ┏━━━━━━━━━━━┓                                                          │ │
│  │  ┃  Vertex   ┃──────┐                                                   │ │
│  │  ┗━━━━━━━━━━━┛      │    ╔═══════════════════╗                          │ │
│  │   (Blue)            └───→║ Binding 0         ║──→ [MVP Matrix Uniform]  │ │
│  │                           ║ Uniform Buffer    ║     (Sea Green box)      │ │
│  │  ┏━━━━━━━━━━━┓           ╚═══════════════════╝                          │ │
│  │  ┃ Fragment  ┃──┐                                                        │ │
│  │  ┗━━━━━━━━━━━┛  │        ╔═══════════════════╗                          │ │
│  │   (Orange)      ├───────→║ Binding 1         ║──→ [Albedo Texture]      │ │
│  │                 │         ║ Texture           ║     (Dark Orange box)    │ │
│  │                 │         ╚═══════════════════╝                          │ │
│  │                 │                                                         │ │
│  │                 │         ╔═══════════════════╗                          │ │
│  │                 └────────→║ Binding 2         ║──→ [Linear Sampler]      │ │
│  │                           ║ Sampler           ║     (Goldenrod box)      │ │
│  │  ┏━━━━━━━━━━━┓           ╚═══════════════════╝                          │ │
│  │  ┃  Compute  ┃                                                           │ │
│  │  ┗━━━━━━━━━━━┛           ╔═══════════════════╗                          │ │
│  │   (Green)        ────────→║ Binding 3         ║──→ [Output Buffer]       │ │
│  │                           ║ Storage Buffer    ║     (Steel Blue box)     │ │
│  │                           ╚═══════════════════╝                          │ │
│  │                                                                          │ │
│  │  Legend:                                                                 │ │
│  │  ■ Uniform Buffer   ■ Storage Buffer   ■ Texture                        │ │
│  │  ■ Sampler          ■ Storage Texture                                   │ │
│  └────────────────────────────────────────────────────────────────────────┘ │
│                                                                              │
│  How to read this diagram:                                                  │
│  • Pipeline Stages (left): Shader stages where bindings are accessible      │
│  • Bindings (center): Configured binding slots with their types             │
│  • Resources (right): Actual GPU resources bound to each slot               │
│  • Arrows: Show the data flow from resources through bindings to stages     │
│  • Colors: Different binding types are color-coded (see legend)             │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Visual Elements Breakdown

### Header Section
- Title: "🔗 Bind Group Configuration" in heading font
- Three tabs displayed horizontally
- Active tab (Visualization) is highlighted

### Main Visualization Canvas
- Dark background (RGB: 30, 30, 35) for contrast
- Canvas size: 800x600 pixels
- Title at top center: Layout label or "Bind Group Visualization"

### Left Column - Pipeline Stages
- Label: "Pipeline Stages" in gray
- Three boxes vertically arranged with 20px spacing:
  - Vertex stage: Blue box (100, 150, 255) with lighter fill
  - Fragment stage: Orange box (255, 150, 100) with lighter fill
  - Compute stage: Green box (150, 255, 100) with lighter fill
- Each box: 120px wide, 80px tall, 5px rounded corners
- Stage name centered in white text

### Center Column - Bindings
- Label: "Bindings" in gray
- Boxes for each binding entry:
  - Width: 200px, Height: 60px
  - Color varies by binding type (see color scheme)
  - Two text lines:
    - Line 1: "Binding {number}" in white
    - Line 2: Type name in light gray
  - 20px vertical spacing between bindings

### Right Column - Resources
- Label: "Resources" in gray
- Boxes aligned with corresponding bindings:
  - Width: 180px, matching binding height
  - Resource name centered in white
  - Same color as its binding (slightly lighter)

### Connection Lines
- Arrows from bindings to pipeline stages (left connections)
  - Only drawn if binding is visible in that stage
  - Color matches the binding type
  - 60% opacity for subtlety
- Arrows from bindings to resources (right connections)
  - Drawn for all assigned resources
  - Includes arrowhead at resource end
  - Color matches the binding type

### Legend (Bottom Left)
- Small colored squares (12x12px) with white borders
- Each resource type listed with its color
- Compact layout, gray text labels

### Description Section (Below Canvas)
- Grouped in a box with gray background
- Bullet points explaining how to read the diagram
- Uses emoji for visual interest (•)

## Color Scheme Reference

### Resource Type Colors
- Uniform Buffer: RGB(50, 150, 100) - Sea Green
- Storage Buffer: RGB(70, 130, 180) - Steel Blue
- Texture: RGB(255, 140, 0) - Dark Orange
- Sampler: RGB(218, 165, 32) - Goldenrod
- Storage Texture: RGB(220, 20, 60) - Crimson

### Pipeline Stage Colors
- Vertex: RGB(100, 150, 255) - Light Blue
- Fragment: RGB(255, 150, 100) - Light Orange
- Compute: RGB(150, 255, 100) - Light Green

### UI Colors
- Background: RGB(30, 30, 35) - Very Dark Gray
- Text (primary): White
- Text (secondary): Light Gray
- Box strokes: 2px, matching fill color but brighter

## Interaction
The visualization is currently static (no hover effects or clicks), but shows:
- Clear visual hierarchy
- Logical flow from left to right
- Easy-to-follow connections
- Intuitive color coding
- Comprehensive information density

## Responsive Design
- Canvas maintains 800x600 aspect ratio
- Scroll area allows viewing on smaller screens
- Text remains readable at default sizes
- No overlapping elements

This visualization makes bind groups immediately understandable, even for users new to WebGPU!
