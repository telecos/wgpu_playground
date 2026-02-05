# Backend Switch UI Implementation

This document describes the UI changes implemented for backend switching between Dawn Native and wgpu-rs.

## Features Implemented

### 1. Backend Indicator in Top Menu Bar

**Location**: Top menu bar, next to the "WebGPU Playground" heading

**Appearance**:
- Label: "Backend:"
- Current backend name displayed with color coding:
  - 🦀 wgpu-rs (blue color: RGB 100, 150, 255)
  - 🌅 Dawn Native (orange color: RGB 255, 180, 100)
- Hover tooltip shows the backend description

**Example Display**:
```
🎮 WebGPU Playground | Backend: 🦀 wgpu-rs | File: [💾 Save State] [📂 Load State] ...
```

### 2. Backend Selector in Settings Panel

**Location**: Settings panel (accessible from the Tools section in the sidebar)

**UI Components**:

#### Current Backend Display
- Prominently shows the active backend with color coding
- Format: "Current Backend: wgpu-rs" (in blue) or "Current Backend: Dawn Native" (in orange)

#### Backend Selection Dropdown
- ComboBox labeled "Select Backend"
- Options:
  1. 🦀 wgpu-rs (Rust implementation)
     - Always available
     - Tooltip: "Used by Firefox, fully featured and stable"
  
  2. 🌅 Dawn Native (C++ implementation)
     - Available when compiled with `--features dawn`
     - Tooltip: "Used by Chromium browsers"
     - When not available: Shows as "🌅 Dawn Native (Not Available)" (disabled)
     - Disabled tooltip: "Compile with --features dawn to enable"

#### Backend Description
- Shows detailed description of the selected backend
- Examples:
  - wgpu: "wgpu-rs (Rust implementation, used by Firefox)"
  - Dawn: "Dawn (C++ implementation, used by Chromium)"

#### Change Warning
- Displayed when selected backend differs from current backend
- Orange warning: "⚠️ Warning: Backend switching requires application restart"
- Instructions: "To apply this change, set the environment variable: WEBGPU_IMPL=<backend>"
- "Then restart the application."

#### Availability Status
- Green checkmark: "✓ Dawn support is compiled in" (when Dawn feature is enabled)
- Yellow info: "ℹ️ Dawn support not available (compile with --features dawn)" (when Dawn not compiled)

#### Available Backends List
- Shows all available backends with status indicators
- Format:
  ```
  💡 Tip: Available backends:
    ✓ wgpu (active)
    ○ Dawn (inactive)
  ```

## User Workflow

### Viewing Current Backend

1. **Quick Check**: Look at the top menu bar to see the current backend indicator
2. **Detailed Info**: Navigate to Settings panel for full backend information

### Switching Backend

1. Open Settings panel
2. Scroll to "🔧 WebGPU Backend" section
3. Use the "Select Backend" dropdown to choose desired backend
4. If Dawn is not available, it will appear as disabled with instructions to compile with `--features dawn`
5. When selection changes, a warning appears with environment variable instructions
6. Set the environment variable: `WEBGPU_IMPL=dawn` or `WEBGPU_IMPL=wgpu`
7. Restart the application

### Graceful Handling of Unavailable Dawn

When Dawn is not compiled in:
- The dropdown shows "🌅 Dawn Native (Not Available)" as a disabled option
- Hovering shows: "Compile with --features dawn to enable"
- The availability status shows: "ℹ️ Dawn support not available (compile with --features dawn)"
- The backend list only shows "wgpu" as available

## Technical Implementation

### Files Modified

1. **crates/wgpu_playground_core/src/settings_panel.rs**
   - Added `selected_backend` field to track user's backend selection
   - Extended `ui()` method to render backend selection UI
   - Uses conditional compilation (`#[cfg(feature = "dawn")]`) to handle Dawn availability

2. **crates/wgpu_playground_gui/src/app.rs**
   - Added backend indicator to top menu bar
   - Uses `WebGPUImplementation::current()` to get active backend
   - Color-codes the display based on backend type

### Color Scheme

- **wgpu-rs**: Blue (RGB 100, 150, 255) - representing Rust
- **Dawn Native**: Orange (RGB 255, 180, 100) - representing native C++
- **Active/Success**: Green (RGB 100, 200, 100)
- **Warning**: Orange (RGB 255, 200, 100)
- **Info**: Yellow (RGB 200, 200, 100)

## Benefits

1. **Clear Visibility**: Users can instantly see which backend is active from the menu bar
2. **Easy Discovery**: Backend selection is in the Settings panel where users expect configuration options
3. **Safe Switching**: Clear warnings about restart requirements prevent confusion
4. **Graceful Degradation**: When Dawn is not available, UI clearly indicates this without errors
5. **Consistent Design**: Follows existing UI patterns and color schemes in the application
6. **Educational**: Descriptions and tooltips help users understand the differences between backends

## Future Enhancements

Possible future improvements:
- Add a "Restart with Backend" button that sets the environment variable and restarts the app
- Show backend-specific features or capabilities
- Add performance comparison information
- Support runtime backend switching without restart (would require significant architectural changes)
