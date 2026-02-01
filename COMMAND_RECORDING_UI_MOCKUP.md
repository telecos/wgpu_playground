# Command Recording Panel - UI Mockup

This document provides a visual description of the Command Recording and Playback Panel UI.

## Panel Layout

```
┌─────────────────────────────────────────────────────────────────┐
│ 📹 Command Recording & Playback                                 │
├─────────────────────────────────────────────────────────────────┤
│ Record, inspect, replay, and export GPU command sequences.     │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────┐   │
│ │ Recording Controls                                       │   │
│ ├─────────────────────────────────────────────────────────┤   │
│ │                                                           │   │
│ │ [⏺️ Start Recording] [🗑️ Clear All] [➕ Add Sample Data] │   │
│ │                                      Recording: 🔴 ON     │   │
│ └─────────────────────────────────────────────────────────┘   │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────┐   │
│ │ Recorded Commands                                        │   │
│ ├─────────────────────────────────────────────────────────┤   │
│ │ Total: 4 commands                                        │   │
│ │                                                           │   │
│ │ ┌───────────────────────────────────────────────────┐   │   │
│ │ │ ID │ Type          │ Label         │ Duration  │ Act│   │   │
│ │ ├───────────────────────────────────────────────────┤   │   │
│ │ │ 1  │ 🎨 Render Pass│ Main Render   │ 100 μs   │🔍 │   │   │
│ │ │ 2  │ 📋 Buffer Copy│ Update Uniform│ 200 μs   │🔍 │   │   │
│ │ │ 3  │ 🧮 Compute Pass│Physics Update│ 300 μs   │🔍 │   │   │
│ │ │ 4  │ 🖼️ Texture Copy│Copy Framebuff│ 400 μs   │🔍 │   │   │
│ │ └───────────────────────────────────────────────────┘   │   │
│ └─────────────────────────────────────────────────────────┘   │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────┐   │
│ │ Timeline View                                            │   │
│ ├─────────────────────────────────────────────────────────┤   │
│ │ Zoom: [────■─────] scale                                 │   │
│ │                                                           │   │
│ │ ┌─────────────────────────────────────────────────────┐ │   │
│ │ │ ████ ████████ ████████████ ████████████████         │ │   │
│ │ │ Blue Orange  Blue       Yellow                        │ │   │
│ │ │ 🎨   🧮       📋          🖼️                           │ │   │
│ │ └─────────────────────────────────────────────────────┘ │   │
│ │                                                           │   │
│ │ (Command blocks sized by duration, color-coded by type)  │   │
│ └─────────────────────────────────────────────────────────┘   │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────┐   │
│ │ Command Inspector                                        │   │
│ ├─────────────────────────────────────────────────────────┤   │
│ │ ID:              3                                       │   │
│ │ Type:            🧮 Compute Pass                         │   │
│ │ Label:           Physics Update                          │   │
│ │ Description:     Compute shader for particle physics     │   │
│ │ Duration:        300 μs                                  │   │
│ │ Timestamp:       45231 ms                                │   │
│ │                                                           │   │
│ │ Command Buffer Contents:                                 │   │
│ │ ┌─────────────────────────────────────────────────────┐ │   │
│ │ │// Command buffer for: Physics Update                │ │   │
│ │ │// Type: Compute Pass                                 │ │   │
│ │ │// This would show the actual GPU commands            │ │   │
│ │ └─────────────────────────────────────────────────────┘ │   │
│ └─────────────────────────────────────────────────────────┘   │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────┐   │
│ │ Playback Controls                                        │   │
│ ├─────────────────────────────────────────────────────────┤   │
│ │ [▶️ Replay All] [▶️ Replay Selected]                     │   │
│ │ (Playback is currently a stub)                           │   │
│ └─────────────────────────────────────────────────────────┘   │
│                                                                 │
│ ┌─────────────────────────────────────────────────────────┐   │
│ │ Export Commands                                          │   │
│ ├─────────────────────────────────────────────────────────┤   │
│ │ Format: (•) JSON  ( ) Text                              │   │
│ │                                                           │   │
│ │ [📥 Export]                                              │   │
│ │                                                           │   │
│ │ Preview:                                                 │   │
│ │ ┌─────────────────────────────────────────────────────┐ │   │
│ │ │[                                                     │ │   │
│ │ │  {                                                   │ │   │
│ │ │    "id": 1,                                          │ │   │
│ │ │    "type": "Render Pass",                            │ │   │
│ │ │    "label": "Main Render",                           │ │   │
│ │ │    "description": "Primary rendering pass...",       │ │   │
│ │ │    "duration_us": 100                                │ │   │
│ │ │  },                                                  │ │   │
│ │ │  ...                                                 │ │   │
│ │ └─────────────────────────────────────────────────────┘ │   │
│ └─────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

## UI Components Description

### 1. Header
- **Title**: "📹 Command Recording & Playback" with recording icon
- **Subtitle**: Brief description of panel functionality

### 2. Recording Controls Section
- **Grouped box** with light background
- **Toggle Button**: Changes between "⏺️ Start Recording" and "⏸️ Stop Recording"
- **Clear All Button**: Red trash icon, removes all recorded commands
- **Add Sample Data Button**: Plus icon, populates example commands for demonstration
- **Status Indicator**: Shows "Recording: 🔴 ON" (red circle) or "Recording: ⚪ OFF" (white circle)

### 3. Recorded Commands Section
- **Header**: Shows total command count
- **Scrollable Table** with striped rows for better readability
  - **Column 1 (ID)**: Unique command identifier
  - **Column 2 (Type)**: Icon + type name (e.g., "🎨 Render Pass")
  - **Column 3 (Label)**: User-friendly command name
  - **Column 4 (Duration)**: Formatted time (μs, ms, or s)
  - **Column 5 (Actions)**: "🔍 Inspect" button to select command for detailed view
- **Empty State**: When no commands, shows helpful message

### 4. Timeline View Section
- **Zoom Control**: Slider from 0.1x to 5x scale
- **Timeline Canvas**: 60px height graphical display
  - Background: Dark gray (#1E1E1E)
  - **Command Blocks**: Horizontal bars
    - Width: Proportional to command duration
    - Height: 40px (with 10px margins top/bottom)
    - Rounded corners (2px radius)
    - Colors:
      - 🎨 Render Pass: Blue (#6496FF)
      - 🧮 Compute Pass: Orange (#FF9664)
      - 📋 Buffer Copy: Green (#96FF96)
      - 🖼️ Texture Copy: Yellow (#FFC864)
      - 🧹 Clear Buffer: Gray (#C8C8C8)
    - Selected command: White (#FFFFFF)
  - Commands arranged sequentially left to right

### 5. Command Inspector Section
- **Conditional Display**: Only shows when a command is selected
- **Grid Layout**: Key-value pairs in two columns
  - **ID**: Command identifier
  - **Type**: Icon + full type name
  - **Label**: Command label
  - **Description**: Detailed description (or "No description")
  - **Duration**: Formatted duration
  - **Timestamp**: Relative timestamp in milliseconds
- **Command Buffer Contents**: Code block showing placeholder GPU commands
  - Gray background
  - Monospace font
  - Future: Would show actual wgpu command buffer details

### 6. Playback Controls Section
- **Replay All Button**: Play icon, triggers replay of all commands
- **Replay Selected Button**: Play icon, replays currently selected command
  - Disabled when no command selected
- **Status Note**: Italic gray text indicating stub implementation

### 7. Export Commands Section
- **Format Selector**: Radio buttons
  - JSON: Structured data format
  - Text: Human-readable report
- **Export Button**: Download icon, triggers export action
- **Live Preview**: Scrollable text area
  - Shows first 10 lines of export
  - Monospace font for code/data
  - Maximum height: 150px
  - "..." ellipsis if content exceeds preview
  - Updates dynamically when format changes

## Color Scheme

### Command Type Colors
- 🎨 **Render Pass**: Blue - `rgb(100, 150, 255)` - #6496FF
- 🧮 **Compute Pass**: Orange - `rgb(255, 150, 100)` - #FF9664
- 📋 **Buffer Copy**: Green - `rgb(150, 255, 150)` - #96FF96
- 🖼️ **Texture Copy**: Yellow - `rgb(255, 200, 100)` - #FFC864
- 🧹 **Clear Buffer**: Gray - `rgb(200, 200, 200)` - #C8C8C8

### UI Elements
- **Success/Active**: Green - `rgb(0, 255, 0)` - #00FF00
- **Error**: Red - `rgb(255, 0, 0)` - #FF0000
- **Selected**: White - `rgb(255, 255, 255)` - #FFFFFF
- **Background (Timeline)**: Dark Gray - `rgb(30, 30, 30)` - #1E1E1E
- **Weak Text**: Gray italic

## Typography

- **Headings**: Bold, larger font
- **Labels**: Regular weight
- **Code/Monospace**: Command buffer contents, export preview
- **Weak/Helper Text**: Italic, lighter gray color
- **Strong Text**: Bold weight for emphasis

## Interactions

### Buttons
- **Hover**: Slightly lighter background
- **Click**: Visual feedback with slight press effect
- Standard egui button styling

### Table Rows
- **Hover**: Highlight on mouse over
- **Striped**: Alternating row colors for readability
- **Selectable**: Inspector button changes color when command selected

### Timeline
- **Hover**: Could show tooltip with command details (future enhancement)
- **Click**: Select command (future enhancement)
- **Zoom Slider**: Real-time timeline scaling

### Scrollable Areas
- **Command List**: Vertical scroll when > 8 commands
- **Timeline**: Horizontal scroll when zoomed in (future)
- **Export Preview**: Vertical scroll for long exports

## Responsive Behavior

- **Minimum Width**: 800px recommended
- **Scrollable Sections**: Adapt to available height
- **Grid Layouts**: Fixed 2-column for inspector, dynamic for command list

## Accessibility

- **Icons**: Paired with text labels
- **Color Coding**: Also differentiated by type name text
- **Clear Labels**: All controls clearly labeled
- **Keyboard Navigation**: Standard egui keyboard support

## Future UI Enhancements

1. **Drag & Drop**: Reorder commands in list
2. **Multi-Select**: Select multiple commands for batch operations
3. **Filter Panel**: Filter by type, duration, date
4. **Search Bar**: Search commands by label or description
5. **Timeline Interactions**: Click timeline blocks to select
6. **Zoom Pan**: Pan timeline when zoomed
7. **Tooltips**: Hover over timeline blocks for details
8. **Performance Graph**: Show timing visualization
9. **Comparison View**: Side-by-side timeline comparison
10. **Dark/Light Themes**: Theme selector

This UI provides a comprehensive interface for GPU command recording and analysis, following modern UI/UX principles with clear visual hierarchy, intuitive controls, and informative feedback.
