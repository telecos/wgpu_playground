/// Integration test to verify theme switching workflow
use wgpu_playground_core::settings_panel::SettingsPanel;
use wgpu_playground_core::state::{PlaygroundState, Theme};

#[test]
fn test_theme_switching_workflow() {
    // Simulate the workflow of changing theme and saving

    // Step 1: Create a new settings panel (default is Dark)
    let mut settings = SettingsPanel::new();
    assert_eq!(settings.get_theme(), Theme::Dark);

    // Step 2: User changes theme to Light
    settings.set_theme(Theme::Light);
    assert_eq!(settings.get_theme(), Theme::Light);

    // Step 3: Export state with theme preference
    let state = PlaygroundState {
        theme: settings.get_theme(),
        ..Default::default()
    };

    // Step 4: Save to file
    let temp_dir = std::env::temp_dir();
    let state_file = temp_dir.join("test_theme_workflow.json");
    state
        .save_to_file(&state_file)
        .expect("Failed to save state");

    // Step 5: Simulate app restart - load state from file
    let loaded_state = PlaygroundState::load_from_file(&state_file).expect("Failed to load state");

    // Step 6: Create new settings panel with loaded theme
    let restored_settings = SettingsPanel::with_theme(loaded_state.theme);

    // Verify theme was persisted correctly
    assert_eq!(restored_settings.get_theme(), Theme::Light);

    // Clean up
    std::fs::remove_file(&state_file).ok();
}

#[test]
fn test_theme_switching_back_and_forth() {
    // Test switching theme multiple times
    let mut settings = SettingsPanel::new();

    // Start with Dark (default)
    assert_eq!(settings.get_theme(), Theme::Dark);

    // Switch to Light
    settings.set_theme(Theme::Light);
    assert_eq!(settings.get_theme(), Theme::Light);

    // Switch back to Dark
    settings.set_theme(Theme::Dark);
    assert_eq!(settings.get_theme(), Theme::Dark);

    // Switch to Light again
    settings.set_theme(Theme::Light);
    assert_eq!(settings.get_theme(), Theme::Light);
}

#[test]
fn test_theme_independence_from_other_state() {
    // Verify theme can be saved/loaded independently of other panel states
    let temp_dir = std::env::temp_dir();
    let state_file = temp_dir.join("test_theme_independence.json");

    // Save state with Light theme but no other panels
    let state = PlaygroundState {
        version: "1.0".to_string(),
        theme: Theme::Light,
        buffer_panel: None,
        texture_panel: None,
        sampler_panel: None,
        shader_editor: None,
        render_pipeline_panel: None,
        compute_pipeline_panel: None,
        bind_group_panel: None,
        bind_group_layout_panel: None,
        api_coverage: None,
        tutorial_state: None,
        learning_progress: None,
    };

    state.save_to_file(&state_file).expect("Failed to save");

    // Load and verify
    let loaded = PlaygroundState::load_from_file(&state_file).expect("Failed to load");

    assert_eq!(loaded.theme, Theme::Light);
    assert!(loaded.buffer_panel.is_none());
    assert!(loaded.texture_panel.is_none());

    // Clean up
    std::fs::remove_file(&state_file).ok();
}
