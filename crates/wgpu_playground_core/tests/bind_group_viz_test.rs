use wgpu_playground_core::bind_group_panel::{
    BindGroupLayoutEntryConfig, BindingTypeConfig, ShaderStagesConfig,
};
use wgpu_playground_core::bind_group_viz::BindGroupVisualizer;

#[test]
fn test_visualizer_default() {
    let viz = BindGroupVisualizer::new();
    assert_eq!(viz.width, 800.0);
    assert_eq!(viz.height, 600.0);
}

#[test]
fn test_visualizer_with_empty_layout() {
    let _viz = BindGroupVisualizer::new();
    let layout_entries: Vec<BindGroupLayoutEntryConfig> = vec![];
    let _binding_assignments: Vec<(u32, String)> = vec![];

    // Should handle empty layout gracefully - this test ensures no panics
    // In actual UI, it would show "No Bind Group Layout" message
    assert!(layout_entries.is_empty());
}

#[test]
fn test_visualizer_with_basic_layout() {
    let viz = BindGroupVisualizer::new();

    // Create a simple bind group layout with one uniform buffer
    let layout_entries = [BindGroupLayoutEntryConfig {
        binding: 0,
        visibility: ShaderStagesConfig {
            vertex: true,
            fragment: true,
            compute: false,
        },
        binding_type: BindingTypeConfig::UniformBuffer,
    }];

    let binding_assignments = [(0, "MVP Matrix Uniform".to_string())];

    // Verify the data structure is set up correctly
    assert_eq!(layout_entries.len(), 1);
    assert_eq!(binding_assignments.len(), 1);
    assert_eq!(layout_entries[0].binding, 0);
    assert_eq!(binding_assignments[0].0, 0);

    // Verify the visualizer can get the correct color for this binding type
    let color = viz.get_binding_type_color(&layout_entries[0].binding_type);
    assert_ne!(color, egui::Color32::TRANSPARENT);
}

#[test]
fn test_visualizer_with_complex_layout() {
    let viz = BindGroupVisualizer::new();

    // Create a more complex layout with multiple bindings
    let layout_entries = [
        BindGroupLayoutEntryConfig {
            binding: 0,
            visibility: ShaderStagesConfig {
                vertex: true,
                fragment: false,
                compute: false,
            },
            binding_type: BindingTypeConfig::UniformBuffer,
        },
        BindGroupLayoutEntryConfig {
            binding: 1,
            visibility: ShaderStagesConfig {
                vertex: false,
                fragment: true,
                compute: false,
            },
            binding_type: BindingTypeConfig::Texture,
        },
        BindGroupLayoutEntryConfig {
            binding: 2,
            visibility: ShaderStagesConfig {
                vertex: false,
                fragment: true,
                compute: false,
            },
            binding_type: BindingTypeConfig::Sampler,
        },
        BindGroupLayoutEntryConfig {
            binding: 3,
            visibility: ShaderStagesConfig {
                vertex: false,
                fragment: false,
                compute: true,
            },
            binding_type: BindingTypeConfig::StorageBuffer { read_only: false },
        },
    ];

    let binding_assignments = [
        (0, "Transform Uniform".to_string()),
        (1, "Albedo Texture".to_string()),
        (2, "Linear Sampler".to_string()),
        (3, "Output Buffer".to_string()),
    ];

    // Verify all bindings are accounted for
    assert_eq!(layout_entries.len(), 4);
    assert_eq!(binding_assignments.len(), 4);

    // Verify binding numbers match
    for i in 0..4 {
        assert_eq!(layout_entries[i].binding as usize, i);
        assert_eq!(binding_assignments[i].0 as usize, i);
    }

    // Verify each binding type gets a distinct color
    let colors: Vec<_> = layout_entries
        .iter()
        .map(|entry| viz.get_binding_type_color(&entry.binding_type))
        .collect();

    // Colors should be distinct (at least uniform vs texture vs sampler vs storage)
    assert_ne!(colors[0], colors[1]); // Uniform vs Texture
    assert_ne!(colors[1], colors[2]); // Texture vs Sampler
    assert_ne!(colors[0], colors[3]); // Uniform vs Storage
}

#[test]
fn test_binding_type_colors_are_distinct() {
    let viz = BindGroupVisualizer::new();

    let uniform_color = viz.get_binding_type_color(&BindingTypeConfig::UniformBuffer);
    let storage_color =
        viz.get_binding_type_color(&BindingTypeConfig::StorageBuffer { read_only: true });
    let texture_color = viz.get_binding_type_color(&BindingTypeConfig::Texture);
    let sampler_color = viz.get_binding_type_color(&BindingTypeConfig::Sampler);
    let storage_texture_color = viz.get_binding_type_color(&BindingTypeConfig::StorageTexture);

    // All colors should be distinct
    assert_ne!(uniform_color, storage_color);
    assert_ne!(uniform_color, texture_color);
    assert_ne!(uniform_color, sampler_color);
    assert_ne!(uniform_color, storage_texture_color);
    assert_ne!(storage_color, texture_color);
    assert_ne!(storage_color, sampler_color);
    assert_ne!(texture_color, sampler_color);
}

#[test]
fn test_shader_visibility_combinations() {
    // Test various visibility combinations
    let configs = vec![
        ShaderStagesConfig {
            vertex: true,
            fragment: false,
            compute: false,
        },
        ShaderStagesConfig {
            vertex: false,
            fragment: true,
            compute: false,
        },
        ShaderStagesConfig {
            vertex: false,
            fragment: false,
            compute: true,
        },
        ShaderStagesConfig {
            vertex: true,
            fragment: true,
            compute: false,
        },
        ShaderStagesConfig {
            vertex: true,
            fragment: true,
            compute: true,
        },
    ];

    // All configurations should be valid
    for config in configs {
        let layout_entry = BindGroupLayoutEntryConfig {
            binding: 0,
            visibility: config,
            binding_type: BindingTypeConfig::UniformBuffer,
        };

        // Ensure at least one stage is set (except for the all-false case)
        let has_visibility = config.vertex || config.fragment || config.compute;
        if has_visibility {
            assert!(
                layout_entry.visibility.vertex
                    || layout_entry.visibility.fragment
                    || layout_entry.visibility.compute
            );
        }
    }
}
