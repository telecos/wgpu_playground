/// Integration test for state save/load functionality
use wgpu_playground_core::state::{BufferPanelState, PlaygroundState, SamplerPanelState, TexturePanelState};
use std::fs;

#[test]
fn test_save_and_load_state() {
    // Create a playground state with some panel configurations
    let state = PlaygroundState {
        version: "1.0".to_string(),
        buffer_panel: Some(BufferPanelState {
            label: "test_buffer".to_string(),
            size: "2048".to_string(),
            usage_vertex: true,
            usage_index: false,
            usage_uniform: true,
            usage_storage: false,
            usage_indirect: false,
            usage_copy_src: true,
            usage_copy_dst: true,
            usage_map_read: false,
            usage_map_write: false,
            usage_query_resolve: false,
            mapped_at_creation: false,
        }),
        texture_panel: Some(TexturePanelState {
            label: "test_texture".to_string(),
            width: "512".to_string(),
            height: "512".to_string(),
            depth: "1".to_string(),
            mip_levels: "1".to_string(),
            sample_count: "1".to_string(),
            format: "Rgba8Unorm".to_string(),
            dimension: "D2".to_string(),
            usage_copy_src: false,
            usage_copy_dst: true,
            usage_texture_binding: true,
            usage_storage_binding: false,
            usage_render_attachment: true,
        }),
        sampler_panel: Some(SamplerPanelState {
            label: "test_sampler".to_string(),
            address_mode_u: "ClampToEdge".to_string(),
            address_mode_v: "ClampToEdge".to_string(),
            address_mode_w: "ClampToEdge".to_string(),
            mag_filter: "Linear".to_string(),
            min_filter: "Linear".to_string(),
            mipmap_filter: "Linear".to_string(),
            lod_min_clamp: "0.0".to_string(),
            lod_max_clamp: "32.0".to_string(),
            compare: None,
            max_anisotropy: "1".to_string(),
        }),
        shader_editor: None,
        render_pipeline_panel: None,
        compute_pipeline_panel: None,
        bind_group_panel: None,
        bind_group_layout_panel: None,
    };

    // Save state to a temp file
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_playground_state.json");
    
    // Save the state
    state.save_to_file(&test_file).expect("Failed to save state");
    
    // Verify file exists
    assert!(test_file.exists(), "State file was not created");
    
    // Load the state back
    let loaded_state = PlaygroundState::load_from_file(&test_file).expect("Failed to load state");
    
    // Verify the loaded state matches
    assert_eq!(loaded_state.version, "1.0");
    
    // Verify buffer panel
    assert!(loaded_state.buffer_panel.is_some());
    let buffer = loaded_state.buffer_panel.unwrap();
    assert_eq!(buffer.label, "test_buffer");
    assert_eq!(buffer.size, "2048");
    assert!(buffer.usage_vertex);
    assert!(buffer.usage_uniform);
    assert!(buffer.usage_copy_src);
    
    // Verify texture panel
    assert!(loaded_state.texture_panel.is_some());
    let texture = loaded_state.texture_panel.unwrap();
    assert_eq!(texture.label, "test_texture");
    assert_eq!(texture.width, "512");
    assert_eq!(texture.height, "512");
    assert!(texture.usage_texture_binding);
    assert!(texture.usage_render_attachment);
    
    // Verify sampler panel
    assert!(loaded_state.sampler_panel.is_some());
    let sampler = loaded_state.sampler_panel.unwrap();
    assert_eq!(sampler.label, "test_sampler");
    assert_eq!(sampler.mag_filter, "Linear");
    assert_eq!(sampler.min_filter, "Linear");
    
    // Clean up
    fs::remove_file(test_file).ok();
}

#[test]
fn test_json_serialization_format() {
    let state = PlaygroundState {
        version: "1.0".to_string(),
        buffer_panel: Some(BufferPanelState {
            label: "my_buffer".to_string(),
            size: "1024".to_string(),
            usage_vertex: true,
            usage_index: false,
            usage_uniform: false,
            usage_storage: false,
            usage_indirect: false,
            usage_copy_src: false,
            usage_copy_dst: true,
            usage_map_read: false,
            usage_map_write: false,
            usage_query_resolve: false,
            mapped_at_creation: false,
        }),
        texture_panel: None,
        sampler_panel: None,
        shader_editor: None,
        render_pipeline_panel: None,
        compute_pipeline_panel: None,
        bind_group_panel: None,
        bind_group_layout_panel: None,
    };

    // Convert to JSON
    let json = state.to_json().expect("Failed to serialize to JSON");
    
    // Verify JSON contains expected fields
    assert!(json.contains("\"version\""));
    assert!(json.contains("\"1.0\""));
    assert!(json.contains("\"buffer_panel\""));
    assert!(json.contains("\"my_buffer\""));
    assert!(json.contains("\"1024\""));
    assert!(json.contains("\"usage_vertex\": true"));
    assert!(json.contains("\"usage_copy_dst\": true"));
    
    // Verify it can be parsed back
    let parsed = PlaygroundState::from_json(&json).expect("Failed to parse JSON");
    assert_eq!(parsed.version, "1.0");
    assert!(parsed.buffer_panel.is_some());
}

#[test]
fn test_partial_state_loading() {
    // Test loading a state with only some panels configured
    let state = PlaygroundState {
        version: "1.0".to_string(),
        buffer_panel: None,
        texture_panel: Some(TexturePanelState {
            label: "only_texture".to_string(),
            width: "256".to_string(),
            height: "256".to_string(),
            depth: "1".to_string(),
            mip_levels: "1".to_string(),
            sample_count: "1".to_string(),
            format: "Rgba8Unorm".to_string(),
            dimension: "D2".to_string(),
            usage_copy_src: false,
            usage_copy_dst: true,
            usage_texture_binding: true,
            usage_storage_binding: false,
            usage_render_attachment: false,
        }),
        sampler_panel: None,
        shader_editor: None,
        render_pipeline_panel: None,
        compute_pipeline_panel: None,
        bind_group_panel: None,
        bind_group_layout_panel: None,
    };

    let json = state.to_json().expect("Failed to serialize");
    let loaded = PlaygroundState::from_json(&json).expect("Failed to parse");
    
    assert!(loaded.buffer_panel.is_none());
    assert!(loaded.texture_panel.is_some());
    assert!(loaded.sampler_panel.is_none());
    
    let texture = loaded.texture_panel.unwrap();
    assert_eq!(texture.label, "only_texture");
}
