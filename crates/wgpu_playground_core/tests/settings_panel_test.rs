/// Tests for the settings panel and theme persistence
use wgpu_playground_core::settings_panel::SettingsPanel;
use wgpu_playground_core::state::{PlaygroundState, Theme};

#[test]
fn test_settings_panel_creation() {
    let panel = SettingsPanel::new();
    assert_eq!(panel.get_theme(), Theme::Dark); // Default is dark
}

#[test]
fn test_settings_panel_with_theme() {
    let panel = SettingsPanel::with_theme(Theme::Light);
    assert_eq!(panel.get_theme(), Theme::Light);
}

#[test]
fn test_settings_panel_set_theme() {
    let mut panel = SettingsPanel::new();
    assert_eq!(panel.get_theme(), Theme::Dark);
    
    panel.set_theme(Theme::Light);
    assert_eq!(panel.get_theme(), Theme::Light);
    
    panel.set_theme(Theme::Dark);
    assert_eq!(panel.get_theme(), Theme::Dark);
}

#[test]
fn test_theme_serialization() {
    let state = PlaygroundState {
        theme: Theme::Light,
        ..Default::default()
    };
    
    // Serialize to JSON
    let json = state.to_json().expect("Failed to serialize");
    assert!(json.contains("\"theme\""));
    assert!(json.contains("\"Light\""));
    
    // Deserialize back
    let deserialized: PlaygroundState = serde_json::from_str(&json)
        .expect("Failed to deserialize");
    assert_eq!(deserialized.theme, Theme::Light);
}

#[test]
fn test_theme_persistence() {
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_theme_state.json");
    
    // Create state with light theme
    let state = PlaygroundState {
        theme: Theme::Light,
        ..Default::default()
    };
    
    // Save to file
    state.save_to_file(&test_file).expect("Failed to save");
    
    // Load from file
    let loaded_state = PlaygroundState::load_from_file(&test_file)
        .expect("Failed to load");
    
    assert_eq!(loaded_state.theme, Theme::Light);
    
    // Clean up
    std::fs::remove_file(&test_file).ok();
}

#[test]
fn test_default_theme_on_missing_field() {
    // Test backward compatibility - if theme field is missing, it should default
    let json = r#"{
        "version": "1.0",
        "buffer_panel": null,
        "texture_panel": null,
        "sampler_panel": null,
        "shader_editor": null,
        "render_pipeline_panel": null,
        "compute_pipeline_panel": null,
        "bind_group_panel": null,
        "bind_group_layout_panel": null
    }"#;
    
    let state: PlaygroundState = serde_json::from_str(json)
        .expect("Failed to deserialize");
    
    // Should default to Dark theme
    assert_eq!(state.theme, Theme::Dark);
}
