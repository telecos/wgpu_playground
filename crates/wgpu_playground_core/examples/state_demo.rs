/// Demonstration of save/load functionality
///
/// This test demonstrates how the playground state can be saved to and loaded from JSON files.
use wgpu_playground_core::state::{
    BufferPanelState, PlaygroundState, SamplerPanelState, ShaderEditorState, TexturePanelState,
};

fn main() {
    println!("=== Playground State Save/Load Demonstration ===\n");

    // 1. Create a playground state with realistic configuration
    println!("1. Creating playground state with sample configuration...");
    let state = PlaygroundState {
        version: "1.0".to_string(),
        theme: wgpu_playground_core::state::Theme::default(),
        buffer_panel: Some(BufferPanelState {
            label: "vertex_buffer".to_string(),
            size: "4096".to_string(),
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
        texture_panel: Some(TexturePanelState {
            label: "render_target".to_string(),
            width: "1024".to_string(),
            height: "768".to_string(),
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
            label: "linear_sampler".to_string(),
            address_mode_u: "Repeat".to_string(),
            address_mode_v: "Repeat".to_string(),
            address_mode_w: "Repeat".to_string(),
            mag_filter: "Linear".to_string(),
            min_filter: "Linear".to_string(),
            mipmap_filter: "Linear".to_string(),
            lod_min_clamp: "0.0".to_string(),
            lod_max_clamp: "32.0".to_string(),
            compare: None,
            max_anisotropy: "16".to_string(),
        }),
        shader_editor: Some(ShaderEditorState {
            source_code: r#"@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 0.5),
        vec2<f32>(-0.5, -0.5),
        vec2<f32>(0.5, -0.5)
    );
    let pos = positions[vertex_index];
    return vec4<f32>(pos, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.5, 0.2, 1.0);
}"#
            .to_string(),
            label: "triangle_shader".to_string(),
            file_path: "".to_string(),
        }),
        render_pipeline_panel: None,
        compute_pipeline_panel: None,
        bind_group_panel: None,
        bind_group_layout_panel: None,
    };
    println!("   ✓ State created with:");
    println!("     - Buffer: vertex_buffer (4096 bytes, VERTEX | COPY_DST)");
    println!("     - Texture: render_target (1024x768, RGBA8Unorm)");
    println!("     - Sampler: linear_sampler (Linear filtering, Repeat addressing)");
    println!("     - Shader: triangle_shader (WGSL vertex + fragment shader)");
    println!();

    // 2. Serialize to JSON
    println!("2. Serializing state to JSON...");
    let json = state.to_json().expect("Failed to serialize state");
    println!("   ✓ Serialized to JSON ({} bytes)", json.len());
    println!();

    // 3. Save to file
    println!("3. Saving state to file...");
    let temp_dir = std::env::temp_dir();
    let save_path = temp_dir.join("demo_playground_state.json");
    state
        .save_to_file(&save_path)
        .expect("Failed to save state");
    println!("   ✓ Saved to: {:?}", save_path);
    println!();

    // 4. Display sample JSON
    println!("4. Sample JSON content:");
    println!("   {}", "-".repeat(60));
    let lines: Vec<&str> = json.lines().take(15).collect();
    for line in lines {
        println!("   {}", line);
    }
    println!("   ... (truncated)");
    println!("   {}", "-".repeat(60));
    println!();

    // 5. Load from file
    println!("5. Loading state from file...");
    let loaded_state = PlaygroundState::load_from_file(&save_path).expect("Failed to load state");
    println!("   ✓ State loaded successfully");
    println!();

    // 6. Verify loaded data
    println!("6. Verifying loaded state...");
    assert_eq!(loaded_state.version, "1.0");
    println!("   ✓ Version: {}", loaded_state.version);

    if let Some(buffer) = &loaded_state.buffer_panel {
        assert_eq!(buffer.label, "vertex_buffer");
        assert_eq!(buffer.size, "4096");
        assert!(buffer.usage_vertex);
        println!(
            "   ✓ Buffer panel: {} ({} bytes)",
            buffer.label, buffer.size
        );
    }

    if let Some(texture) = &loaded_state.texture_panel {
        assert_eq!(texture.label, "render_target");
        assert_eq!(texture.width, "1024");
        assert_eq!(texture.height, "768");
        println!(
            "   ✓ Texture panel: {} ({}x{})",
            texture.label, texture.width, texture.height
        );
    }

    if let Some(sampler) = &loaded_state.sampler_panel {
        assert_eq!(sampler.label, "linear_sampler");
        assert_eq!(sampler.mag_filter, "Linear");
        println!(
            "   ✓ Sampler panel: {} ({})",
            sampler.label, sampler.mag_filter
        );
    }

    if let Some(shader) = &loaded_state.shader_editor {
        assert_eq!(shader.label, "triangle_shader");
        assert!(shader.source_code.contains("@vertex"));
        println!(
            "   ✓ Shader editor: {} ({} bytes)",
            shader.label,
            shader.source_code.len()
        );
    }
    println!();

    // 7. Clean up
    println!("7. Cleaning up...");
    std::fs::remove_file(&save_path).ok();
    println!("   ✓ Test file removed");
    println!();

    println!("=== Demonstration Complete ===");
    println!("\nThe playground state save/load feature is working correctly!");
    println!("Users can now:");
    println!("  • Configure resources (buffers, textures, samplers)");
    println!("  • Write custom shaders");
    println!("  • Save their work to a JSON file");
    println!("  • Load previous configurations");
    println!("\nThis enables workflow preservation and sharing of playground setups.");
}
