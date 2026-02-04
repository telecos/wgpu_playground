use std::fs;
/// Integration test for state save/load functionality
use wgpu_playground_core::state::{
    BufferPanelState, PlaygroundState, SamplerPanelState, TexturePanelState,
};

#[test]
fn test_save_and_load_state() {
    // Create a playground state with some panel configurations
    let state = PlaygroundState {
        version: "1.0".to_string(),
        theme: wgpu_playground_core::state::Theme::default(),
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
    state
        .save_to_file(&test_file)
        .expect("Failed to save state");

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
        theme: wgpu_playground_core::state::Theme::default(),
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
        theme: wgpu_playground_core::state::Theme::default(),
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

#[test]
fn test_url_encoding_integration() {
    // Create a complex state
    let state = PlaygroundState {
        version: "1.0".to_string(),
        buffer_panel: Some(BufferPanelState {
            label: "shared_buffer".to_string(),
            size: "4096".to_string(),
            usage_vertex: true,
            usage_index: true,
            usage_uniform: false,
            usage_storage: true,
            usage_indirect: false,
            usage_copy_src: true,
            usage_copy_dst: true,
            usage_map_read: false,
            usage_map_write: false,
            usage_query_resolve: false,
            mapped_at_creation: false,
        }),
        texture_panel: Some(TexturePanelState {
            label: "shared_texture".to_string(),
            width: "1024".to_string(),
            height: "768".to_string(),
            depth: "1".to_string(),
            mip_levels: "4".to_string(),
            sample_count: "1".to_string(),
            format: "Rgba8Unorm".to_string(),
            dimension: "D2".to_string(),
            usage_copy_src: false,
            usage_copy_dst: true,
            usage_texture_binding: true,
            usage_storage_binding: false,
            usage_render_attachment: true,
        }),
        sampler_panel: None,
        shader_editor: None,
        render_pipeline_panel: None,
        compute_pipeline_panel: None,
        bind_group_panel: None,
        bind_group_layout_panel: None,
    };

    // Test URL encoding and decoding
    let encoded = state.to_url_encoded().expect("Failed to encode state");
    assert!(!encoded.is_empty());
    assert!(
        encoded
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_'),
        "Encoded string should be URL-safe"
    );

    // Decode and verify
    let decoded = PlaygroundState::from_url_encoded(&encoded).expect("Failed to decode state");
    assert_eq!(decoded.version, state.version);
    assert!(decoded.buffer_panel.is_some());
    assert!(decoded.texture_panel.is_some());

    let buffer = decoded.buffer_panel.unwrap();
    assert_eq!(buffer.label, "shared_buffer");
    assert_eq!(buffer.size, "4096");
    assert!(buffer.usage_vertex);
    assert!(buffer.usage_storage);

    let texture = decoded.texture_panel.unwrap();
    assert_eq!(texture.label, "shared_texture");
    assert_eq!(texture.width, "1024");
    assert_eq!(texture.height, "768");
}

#[test]
fn test_shareable_url_generation_integration() {
    let state = PlaygroundState {
        version: "1.0".to_string(),
        buffer_panel: Some(BufferPanelState {
            label: "url_buffer".to_string(),
            size: "2048".to_string(),
            usage_vertex: true,
            usage_index: false,
            usage_uniform: true,
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

    // Test shareable URL generation
    let base_url = "https://example.com/playground";
    let url = state
        .to_shareable_url(base_url)
        .expect("Failed to generate shareable URL");

    assert!(url.starts_with(base_url));
    assert!(url.contains("?state="));

    // Extract state from URL and verify
    let decoded = PlaygroundState::from_url(&url).expect("Failed to extract state from URL");
    assert_eq!(decoded.version, "1.0");
    assert!(decoded.buffer_panel.is_some());

    let buffer = decoded.buffer_panel.unwrap();
    assert_eq!(buffer.label, "url_buffer");
    assert_eq!(buffer.size, "2048");
    assert!(buffer.usage_vertex);
    assert!(buffer.usage_uniform);
}

#[test]
fn test_url_with_complex_shader_code() {
    use wgpu_playground_core::state::ShaderEditorState;

    // Test with a realistic shader code that contains special characters
    let shader_code = r#"
@group(0) @binding(0)
var<uniform> transform: mat4x4<f32>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = transform * vec4<f32>(input.position, 1.0);
    output.color = input.color;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return input.color;
}
"#;

    let state = PlaygroundState {
        version: "1.0".to_string(),
        buffer_panel: None,
        texture_panel: None,
        sampler_panel: None,
        shader_editor: Some(ShaderEditorState {
            source_code: shader_code.to_string(),
            label: "transform_shader".to_string(),
            file_path: "transform.wgsl".to_string(),
        }),
        render_pipeline_panel: None,
        compute_pipeline_panel: None,
        bind_group_panel: None,
        bind_group_layout_panel: None,
    };

    // Encode to URL
    let encoded = state
        .to_url_encoded()
        .expect("Failed to encode shader state");
    assert!(!encoded.is_empty());

    // Decode and verify shader code is preserved
    let decoded = PlaygroundState::from_url_encoded(&encoded).expect("Failed to decode");
    assert!(decoded.shader_editor.is_some());

    let shader = decoded.shader_editor.unwrap();
    assert_eq!(shader.source_code, shader_code);
    assert_eq!(shader.label, "transform_shader");
    assert_eq!(shader.file_path, "transform.wgsl");
}

#[test]
fn test_url_parameter_extraction() {
    // Test extracting state from various URL formats
    let state = PlaygroundState {
        version: "1.0".to_string(),
        buffer_panel: Some(BufferPanelState {
            label: "test".to_string(),
            size: "1024".to_string(),
            usage_vertex: true,
            usage_index: false,
            usage_uniform: false,
            usage_storage: false,
            usage_indirect: false,
            usage_copy_src: false,
            usage_copy_dst: false,
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

    let encoded = state.to_url_encoded().unwrap();

    // Test different URL formats
    let test_cases = vec![
        format!("https://example.com?state={}", encoded),
        format!("https://example.com/path?state={}", encoded),
        format!("https://example.com?foo=bar&state={}", encoded),
        format!("https://example.com?state={}&foo=bar", encoded),
        format!("http://localhost:8080?state={}", encoded),
    ];

    for url in test_cases {
        let decoded = PlaygroundState::from_url(&url)
            .unwrap_or_else(|e| panic!("Failed to decode URL '{}': {}", url, e));
        assert_eq!(decoded.version, "1.0");
        assert!(decoded.buffer_panel.is_some());
    }
}
