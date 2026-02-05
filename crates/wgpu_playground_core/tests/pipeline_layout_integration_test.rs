mod common;

use common::create_test_device;
use std::num::NonZeroU64;
use wgpu::ShaderStages;
use wgpu_playground_core::bind_group::{
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
};
use wgpu_playground_core::pipeline_layout::{PipelineLayoutDescriptor, PushConstantRange};

#[test]
fn test_empty_pipeline_layout() {
    let descriptor = PipelineLayoutDescriptor::new(Some("empty_pipeline_layout"));
    assert!(descriptor.validate().is_ok());
    assert_eq!(descriptor.bind_group_layouts().len(), 0);
    assert_eq!(descriptor.push_constant_ranges().len(), 0);
}

#[test]
fn test_pipeline_layout_with_push_constants() {
    let range = PushConstantRange::new(ShaderStages::VERTEX, 0, 64);
    let descriptor =
        PipelineLayoutDescriptor::new(Some("push_constant_layout")).with_push_constant_range(range);

    assert!(descriptor.validate().is_ok());
    assert_eq!(descriptor.push_constant_ranges().len(), 1);
}

#[test]
fn test_pipeline_layout_with_multiple_push_constant_ranges() {
    // Non-overlapping ranges for different stages
    let vertex_range = PushConstantRange::new(ShaderStages::VERTEX, 0, 64);
    let fragment_range = PushConstantRange::new(ShaderStages::FRAGMENT, 64, 128);

    let descriptor = PipelineLayoutDescriptor::new(Some("multi_push_constant_layout"))
        .with_push_constant_range(vertex_range)
        .with_push_constant_range(fragment_range);

    assert!(descriptor.validate().is_ok());
    assert_eq!(descriptor.push_constant_ranges().len(), 2);
}

#[test]
fn test_pipeline_layout_vertex_fragment_shared_push_constants() {
    // Same range shared across vertex and fragment stages
    let range = PushConstantRange::new(ShaderStages::VERTEX | ShaderStages::FRAGMENT, 0, 128);
    let descriptor = PipelineLayoutDescriptor::new(Some("shared_push_constant_layout"))
        .with_push_constant_range(range);

    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_pipeline_layout_compute_push_constants() {
    let range = PushConstantRange::new(ShaderStages::COMPUTE, 0, 256);
    let descriptor = PipelineLayoutDescriptor::new(Some("compute_push_constant_layout"))
        .with_push_constant_range(range);

    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_pipeline_layout_all_stages_push_constants() {
    let range = PushConstantRange::new(
        ShaderStages::VERTEX | ShaderStages::FRAGMENT | ShaderStages::COMPUTE,
        0,
        64,
    );
    let descriptor = PipelineLayoutDescriptor::new(Some("all_stages_push_constant_layout"))
        .with_push_constant_range(range);

    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_pipeline_layout_multiple_non_overlapping_ranges() {
    let ranges = vec![
        PushConstantRange::new(ShaderStages::VERTEX, 0, 32),
        PushConstantRange::new(ShaderStages::VERTEX, 32, 64),
        PushConstantRange::new(ShaderStages::VERTEX, 64, 96),
    ];

    let descriptor = PipelineLayoutDescriptor::new(Some("sequential_ranges_layout"))
        .with_push_constant_ranges(&ranges);

    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_pipeline_layout_ranges_different_stages_same_offset() {
    // Same offset range but for different stages - should be allowed
    let ranges = vec![
        PushConstantRange::new(ShaderStages::VERTEX, 0, 64),
        PushConstantRange::new(ShaderStages::FRAGMENT, 0, 64),
        PushConstantRange::new(ShaderStages::COMPUTE, 0, 64),
    ];

    let descriptor = PipelineLayoutDescriptor::new(Some("per_stage_ranges_layout"))
        .with_push_constant_ranges(&ranges);

    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_pipeline_layout_with_single_bind_group() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a bind group layout
        let bind_group_layout_descriptor = BindGroupLayoutDescriptor::new(Some("test_bind_group"))
            .with_entry(BindGroupLayoutEntry::new(
                0,
                ShaderStages::VERTEX,
                BindingType::UniformBuffer {
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU64::new(64),
                },
            ));

        let bind_group_layout = bind_group_layout_descriptor
            .create_layout(&device)
            .expect("Failed to create bind group layout");

        // Create pipeline layout with the bind group layout
        let pipeline_layout_descriptor =
            PipelineLayoutDescriptor::new(Some("test_pipeline_layout"))
                .with_bind_group_layout(&bind_group_layout);

        assert!(pipeline_layout_descriptor.validate().is_ok());

        let pipeline_layout = pipeline_layout_descriptor
            .create_layout(&device)
            .expect("Failed to create pipeline layout");

        // Verify the layout was created (we can't inspect much without unsafe code)
        drop(pipeline_layout);
    });
}

#[test]
fn test_pipeline_layout_with_multiple_bind_groups() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create multiple bind group layouts
        let layout1_descriptor = BindGroupLayoutDescriptor::new(Some("bind_group_0")).with_entry(
            BindGroupLayoutEntry::new(
                0,
                ShaderStages::VERTEX,
                BindingType::UniformBuffer {
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU64::new(64),
                },
            ),
        );

        let layout2_descriptor = BindGroupLayoutDescriptor::new(Some("bind_group_1")).with_entry(
            BindGroupLayoutEntry::new(
                0,
                ShaderStages::FRAGMENT,
                BindingType::UniformBuffer {
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU64::new(16),
                },
            ),
        );

        let bind_group_layout1 = layout1_descriptor
            .create_layout(&device)
            .expect("Failed to create bind group layout 1");

        let bind_group_layout2 = layout2_descriptor
            .create_layout(&device)
            .expect("Failed to create bind group layout 2");

        // Create pipeline layout with multiple bind group layouts
        let pipeline_layout_descriptor =
            PipelineLayoutDescriptor::new(Some("multi_bind_group_pipeline_layout"))
                .with_bind_group_layouts(&[&bind_group_layout1, &bind_group_layout2]);

        assert!(pipeline_layout_descriptor.validate().is_ok());
        assert_eq!(pipeline_layout_descriptor.bind_group_layouts().len(), 2);

        let pipeline_layout = pipeline_layout_descriptor
            .create_layout(&device)
            .expect("Failed to create pipeline layout");

        drop(pipeline_layout);
    });
}

#[test]
#[cfg_attr(
    all(target_os = "linux", target_env = "gnu"),
    ignore = "Requires PUSH_CONSTANTS feature not available in CI"
)]
fn test_pipeline_layout_with_bind_groups_and_push_constants() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a bind group layout
        let bind_group_layout_descriptor = BindGroupLayoutDescriptor::new(Some("test_bind_group"))
            .with_entry(BindGroupLayoutEntry::new(
                0,
                ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                BindingType::UniformBuffer {
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU64::new(64),
                },
            ));

        let bind_group_layout = bind_group_layout_descriptor
            .create_layout(&device)
            .expect("Failed to create bind group layout");

        // Create push constant range
        let push_constant_range = PushConstantRange::new(ShaderStages::VERTEX, 0, 64);

        // Create pipeline layout with both bind group and push constants
        let pipeline_layout_descriptor =
            PipelineLayoutDescriptor::new(Some("combined_pipeline_layout"))
                .with_bind_group_layout(&bind_group_layout)
                .with_push_constant_range(push_constant_range);

        assert!(pipeline_layout_descriptor.validate().is_ok());

        let pipeline_layout = pipeline_layout_descriptor
            .create_layout(&device)
            .expect("Failed to create pipeline layout");

        drop(pipeline_layout);
    });
}

#[test]
#[cfg_attr(
    all(target_os = "linux", target_env = "gnu"),
    ignore = "Requires PUSH_CONSTANTS feature not available in CI"
)]
fn test_complex_pipeline_layout() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create multiple bind group layouts representing a realistic rendering scenario

        // Bind group 0: Global uniforms (camera, time, etc.)
        let global_layout = BindGroupLayoutDescriptor::new(Some("global_uniforms"))
            .with_entry(BindGroupLayoutEntry::new(
                0,
                ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                BindingType::UniformBuffer {
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU64::new(128),
                },
            ))
            .create_layout(&device)
            .expect("Failed to create global layout");

        // Bind group 1: Material properties
        let material_layout = BindGroupLayoutDescriptor::new(Some("material_properties"))
            .with_entry(BindGroupLayoutEntry::new(
                0,
                ShaderStages::FRAGMENT,
                BindingType::UniformBuffer {
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU64::new(64),
                },
            ))
            .create_layout(&device)
            .expect("Failed to create material layout");

        // Push constants for per-draw data
        let push_ranges = vec![
            PushConstantRange::new(ShaderStages::VERTEX, 0, 64),
            PushConstantRange::new(ShaderStages::FRAGMENT, 64, 128),
        ];

        // Create comprehensive pipeline layout
        let pipeline_layout_descriptor =
            PipelineLayoutDescriptor::new(Some("complex_rendering_layout"))
                .with_bind_group_layouts(&[&global_layout, &material_layout])
                .with_push_constant_ranges(&push_ranges);

        assert!(pipeline_layout_descriptor.validate().is_ok());
        assert_eq!(pipeline_layout_descriptor.bind_group_layouts().len(), 2);
        assert_eq!(pipeline_layout_descriptor.push_constant_ranges().len(), 2);

        let pipeline_layout = pipeline_layout_descriptor
            .create_layout(&device)
            .expect("Failed to create complex pipeline layout");

        drop(pipeline_layout);
    });
}

#[test]
#[cfg_attr(
    all(target_os = "linux", target_env = "gnu"),
    ignore = "Requires PUSH_CONSTANTS feature not available in CI"
)]
fn test_compute_pipeline_layout() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Compute shader typically uses storage buffers
        let compute_layout = BindGroupLayoutDescriptor::new(Some("compute_buffers"))
            .with_entry(BindGroupLayoutEntry::new(
                0,
                ShaderStages::COMPUTE,
                BindingType::StorageBuffer {
                    has_dynamic_offset: false,
                    min_binding_size: None,
                    read_only: true,
                },
            ))
            .with_entry(BindGroupLayoutEntry::new(
                1,
                ShaderStages::COMPUTE,
                BindingType::StorageBuffer {
                    has_dynamic_offset: false,
                    min_binding_size: None,
                    read_only: false,
                },
            ))
            .create_layout(&device)
            .expect("Failed to create compute layout");

        // Push constants for compute parameters
        let push_range = PushConstantRange::new(ShaderStages::COMPUTE, 0, 16);

        let pipeline_layout_descriptor =
            PipelineLayoutDescriptor::new(Some("compute_pipeline_layout"))
                .with_bind_group_layout(&compute_layout)
                .with_push_constant_range(push_range);

        assert!(pipeline_layout_descriptor.validate().is_ok());

        let pipeline_layout = pipeline_layout_descriptor
            .create_layout(&device)
            .expect("Failed to create compute pipeline layout");

        drop(pipeline_layout);
    });
}

#[test]
fn test_pipeline_layout_label() {
    let descriptor = PipelineLayoutDescriptor::new(Some("my_pipeline_layout"));
    assert_eq!(descriptor.label(), Some("my_pipeline_layout"));

    let descriptor_no_label = PipelineLayoutDescriptor::new(None);
    assert_eq!(descriptor_no_label.label(), None);
}

#[test]
fn test_pipeline_layout_default() {
    let descriptor = PipelineLayoutDescriptor::default();
    assert_eq!(descriptor.label(), None);
    assert_eq!(descriptor.bind_group_layouts().len(), 0);
    assert_eq!(descriptor.push_constant_ranges().len(), 0);
}

#[test]
fn test_push_constant_range_size() {
    let range = PushConstantRange::new(ShaderStages::VERTEX, 0, 64);
    assert_eq!(range.size(), 64);

    let range2 = PushConstantRange::new(ShaderStages::FRAGMENT, 32, 96);
    assert_eq!(range2.size(), 64);

    let range3 = PushConstantRange::new(ShaderStages::COMPUTE, 0, 256);
    assert_eq!(range3.size(), 256);
}
