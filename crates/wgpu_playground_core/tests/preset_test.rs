/// Tests for configuration presets
use wgpu_playground_core::preset::{get_all_presets, PresetCategory};

#[test]
fn test_get_all_presets() {
    let presets = get_all_presets();

    // Should have at least the 3 core presets
    assert!(
        presets.len() >= 3,
        "Expected at least 3 presets, got {}",
        presets.len()
    );

    // Check that we have unique IDs
    let mut ids = std::collections::HashSet::new();
    for preset in &presets {
        assert!(ids.insert(preset.id), "Duplicate preset ID: {}", preset.id);
    }
}

#[test]
fn test_pbr_material_preset() {
    let presets = get_all_presets();
    let pbr = presets.iter().find(|p| p.id == "pbr_material");

    assert!(pbr.is_some(), "PBR material preset not found");
    let pbr = pbr.unwrap();

    assert_eq!(pbr.name, "PBR Material");
    assert_eq!(pbr.category, PresetCategory::Material);
    assert!(!pbr.description.is_empty());

    // Check that it has the required configurations
    assert!(
        pbr.state.shader_editor.is_some(),
        "PBR preset should have shader"
    );
    assert!(
        pbr.state.buffer_panel.is_some(),
        "PBR preset should have buffer"
    );
    assert!(
        pbr.state.render_pipeline_panel.is_some(),
        "PBR preset should have render pipeline"
    );

    // Check shader contains PBR-related code
    let shader = pbr.state.shader_editor.as_ref().unwrap();
    assert!(
        shader.source_code.contains("PBR"),
        "Shader should mention PBR"
    );
    assert!(
        shader.source_code.contains("Cook-Torrance") || shader.source_code.contains("BRDF"),
        "Shader should mention Cook-Torrance or BRDF"
    );
}

#[test]
fn test_shadow_mapping_preset() {
    let presets = get_all_presets();
    let shadow = presets.iter().find(|p| p.id == "shadow_mapping");

    assert!(shadow.is_some(), "Shadow mapping preset not found");
    let shadow = shadow.unwrap();

    assert_eq!(shadow.name, "Shadow Mapping");
    assert_eq!(shadow.category, PresetCategory::Lighting);
    assert!(!shadow.description.is_empty());

    // Check that it has the required configurations
    assert!(
        shadow.state.shader_editor.is_some(),
        "Shadow preset should have shader"
    );
    assert!(
        shadow.state.texture_panel.is_some(),
        "Shadow preset should have texture for shadow map"
    );
    assert!(
        shadow.state.sampler_panel.is_some(),
        "Shadow preset should have sampler"
    );
    assert!(
        shadow.state.buffer_panel.is_some(),
        "Shadow preset should have buffer"
    );

    // Check texture is depth texture
    let texture = shadow.state.texture_panel.as_ref().unwrap();
    assert!(
        texture.format.contains("Depth"),
        "Shadow map should be a depth texture"
    );

    // Check sampler has comparison
    let sampler = shadow.state.sampler_panel.as_ref().unwrap();
    assert!(
        sampler.compare.is_some(),
        "Shadow sampler should have comparison enabled"
    );
}

#[test]
fn test_post_processing_preset() {
    let presets = get_all_presets();
    let post_process = presets.iter().find(|p| p.id == "post_processing");

    assert!(post_process.is_some(), "Post-processing preset not found");
    let post_process = post_process.unwrap();

    assert_eq!(post_process.name, "Post-Processing Effects");
    assert_eq!(post_process.category, PresetCategory::PostProcessing);
    assert!(!post_process.description.is_empty());

    // Check that it has the required configurations
    assert!(
        post_process.state.shader_editor.is_some(),
        "Post-processing preset should have shader"
    );
    assert!(
        post_process.state.texture_panel.is_some(),
        "Post-processing preset should have texture"
    );
    assert!(
        post_process.state.sampler_panel.is_some(),
        "Post-processing preset should have sampler"
    );
    assert!(
        post_process.state.buffer_panel.is_some(),
        "Post-processing preset should have buffer"
    );

    // Check shader contains post-processing effects
    let shader = post_process.state.shader_editor.as_ref().unwrap();
    assert!(
        shader.source_code.contains("vignette")
            || shader.source_code.contains("bloom")
            || shader.source_code.contains("chromatic"),
        "Shader should mention post-processing effects"
    );
}

#[test]
fn test_preset_categories() {
    let presets = get_all_presets();

    // Check that each category has at least one preset
    let categories = [
        PresetCategory::Material,
        PresetCategory::Lighting,
        PresetCategory::PostProcessing,
    ];

    for category in &categories {
        let count = presets.iter().filter(|p| &p.category == category).count();
        assert!(
            count > 0,
            "Category {:?} should have at least one preset",
            category
        );
    }
}

#[test]
fn test_preset_state_serialization() {
    let presets = get_all_presets();

    for preset in presets {
        // Test that the state can be serialized and deserialized
        let json = serde_json::to_string(&preset.state)
            .unwrap_or_else(|_| panic!("Failed to serialize preset: {}", preset.id));

        assert!(
            !json.is_empty(),
            "Serialized JSON should not be empty for preset: {}",
            preset.id
        );

        let _deserialized: wgpu_playground_core::state::PlaygroundState =
            serde_json::from_str(&json)
                .unwrap_or_else(|_| panic!("Failed to deserialize preset: {}", preset.id));
    }
}

#[test]
fn test_preset_tags() {
    let presets = get_all_presets();

    for preset in presets {
        // Each preset should have at least one tag
        assert!(
            !preset.tags.is_empty(),
            "Preset {} should have at least one tag",
            preset.id
        );

        // Tags should not be empty strings
        for tag in preset.tags {
            assert!(!tag.is_empty(), "Preset {} has an empty tag", preset.id);
        }
    }
}

#[test]
fn test_preset_metadata_completeness() {
    let presets = get_all_presets();

    for preset in presets {
        // ID should not be empty
        assert!(!preset.id.is_empty(), "Preset has empty ID");

        // Name should not be empty
        assert!(
            !preset.name.is_empty(),
            "Preset {} has empty name",
            preset.id
        );

        // Description should not be empty
        assert!(
            !preset.description.is_empty(),
            "Preset {} has empty description",
            preset.id
        );

        // Description should be reasonably detailed (at least 50 chars)
        assert!(
            preset.description.len() >= 50,
            "Preset {} has a short description ({})",
            preset.id,
            preset.description.len()
        );
    }
}
