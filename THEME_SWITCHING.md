# Theme Switching Feature

## Overview

The WebGPU Playground now supports dark and light UI themes that can be switched at runtime and are persisted across application restarts.

## Features

- **Two themes available**:
  - 🌙 Dark Theme (default)
  - ☀️ Light Theme

- **Easy theme switching**: Navigate to Settings panel and select your preferred theme
- **Automatic persistence**: Theme preference is automatically saved when changed
- **Loads on startup**: Your theme preference is restored when you restart the application

## How to Use

### Changing the Theme

1. Open the WebGPU Playground application
2. In the left sidebar, navigate to: **🔧 Tools & Debugging → Settings**
3. In the Settings panel, use the "Theme" dropdown to select your preferred theme:
   - ☀️ Light - for a bright, light-colored interface
   - 🌙 Dark - for a dark, low-light interface
4. The theme changes immediately upon selection
5. Your preference is automatically saved to `playground_state.json`

### Theme Persistence

The theme preference is stored in the same state file as other playground settings (`playground_state.json` by default). This means:

- Your theme preference persists across application restarts
- You can use the "Save State" and "Load State" functionality to manage different theme configurations
- The theme is backward compatible - if you load an old state file without a theme field, it defaults to Dark theme

## Implementation Details

### State Structure

The theme is stored in the `PlaygroundState` structure:

```rust
pub struct PlaygroundState {
    pub theme: Theme,
    // ... other fields
}

pub enum Theme {
    Light,
    Dark,
}
```

### API

For developers working with the codebase:

```rust
// Create settings panel
let mut settings = SettingsPanel::new(); // Defaults to Dark theme

// Get current theme
let current_theme = settings.get_theme();

// Set theme
settings.set_theme(Theme::Light);

// Create with specific theme
let settings = SettingsPanel::with_theme(Theme::Light);
```

### Testing

The theme switching feature includes comprehensive tests:

- Unit tests for the settings panel
- Theme serialization/deserialization tests
- Theme persistence tests
- Integration tests for the complete workflow

Run tests with:
```bash
cargo test -p wgpu_playground_core --test settings_panel_test
cargo test -p wgpu_playground_core --test theme_workflow_test
```

## Technical Notes

- The theme uses egui's built-in `Visuals::light()` and `Visuals::dark()` presets
- Theme changes are applied immediately to the egui context
- The theme field in `PlaygroundState` uses `#[serde(default)]` for backward compatibility
- If the theme field is missing from an older state file, it defaults to Dark theme
