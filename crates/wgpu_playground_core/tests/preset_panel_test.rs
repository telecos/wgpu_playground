/// Integration test for preset panel

use wgpu_playground_core::preset_panel::PresetPanel;

#[test]
fn test_preset_panel_creation() {
    let panel = PresetPanel::new();
    
    // Panel should be created successfully
    assert!(std::mem::size_of_val(&panel) > 0);
}

#[test]
fn test_preset_panel_default() {
    let panel = PresetPanel::default();
    
    // Default trait should work
    assert!(std::mem::size_of_val(&panel) > 0);
}
