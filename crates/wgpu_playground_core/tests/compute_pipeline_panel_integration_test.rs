mod common;

use common::create_test_device;
use wgpu_playground_core::compute_pipeline_panel::ComputePipelinePanel;

#[test]
fn test_compute_pipeline_panel_initialization() {
    let panel = ComputePipelinePanel::new();

    // Verify initial state
    assert_eq!(panel.label_input, "");
    assert_eq!(panel.entry_point_input, "main");
    assert_eq!(panel.shader_label, "compute_shader");
    assert!(!panel.shader_source.is_empty());
    assert!(panel.use_auto_layout);
}

#[test]
fn test_compute_pipeline_panel_with_custom_configuration() {
    let mut panel = ComputePipelinePanel::new();

    // Configure the panel
    panel.label_input = "test_pipeline".to_string();
    panel.entry_point_input = "compute_main".to_string();
    panel.shader_label = "my_shader".to_string();
    panel.shader_source = r#"
@compute @workgroup_size(8, 8, 1)
fn compute_main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // Test compute shader
}
"#
    .to_string();

    // Validate should succeed
    assert!(panel.validate());
}

#[test]
fn test_compute_pipeline_panel_validation_errors() {
    let mut panel = ComputePipelinePanel::new();

    // Test with empty entry point
    panel.entry_point_input = "".to_string();
    assert!(!panel.validate());

    // Test with empty shader source
    panel.entry_point_input = "main".to_string();
    panel.shader_source = "".to_string();
    panel.cached_shader = None;
    assert!(!panel.validate());
}

#[test]
fn test_compute_pipeline_panel_creates_valid_pipeline() {
    let device_queue = pollster::block_on(create_test_device());
    if device_queue.is_none() {
        eprintln!("Skipping test: could not create wgpu device");
        return;
    }

    let (device, _queue) = device_queue.unwrap();

    let mut panel = ComputePipelinePanel::new();
    panel.label_input = "integration_test_pipeline".to_string();

    // Create pipeline should succeed with valid configuration
    let pipeline = panel.create_pipeline(&device);
    assert!(pipeline.is_some());
    assert!(panel.validation_error.is_none());
    assert!(panel.success_message.is_some());
}

#[test]
fn test_compute_pipeline_panel_with_storage_buffer_template() {
    let device_queue = pollster::block_on(create_test_device());
    if device_queue.is_none() {
        eprintln!("Skipping test: could not create wgpu device");
        return;
    }

    let (device, _queue) = device_queue.unwrap();

    let mut panel = ComputePipelinePanel::new();
    panel.shader_source = ComputePipelinePanel::storage_buffer_shader();
    panel.cached_shader = None;

    // Should validate successfully
    assert!(panel.validate());

    // Should create pipeline successfully
    let pipeline = panel.create_pipeline(&device);
    assert!(pipeline.is_some());
}

#[test]
fn test_compute_pipeline_panel_with_matrix_multiply_template() {
    let device_queue = pollster::block_on(create_test_device());
    if device_queue.is_none() {
        eprintln!("Skipping test: could not create wgpu device");
        return;
    }

    let (device, _queue) = device_queue.unwrap();

    let mut panel = ComputePipelinePanel::new();
    panel.shader_source = ComputePipelinePanel::matrix_multiply_shader();
    panel.cached_shader = None;

    // Should validate successfully
    assert!(panel.validate());

    // Should create pipeline successfully
    let pipeline = panel.create_pipeline(&device);
    assert!(pipeline.is_some());
}

#[test]
fn test_compute_pipeline_panel_shader_caching() {
    let mut panel = ComputePipelinePanel::new();

    // First validation should create cached shader
    assert!(panel.validate());
    assert!(panel.cached_shader.is_some());

    let first_source = panel.shader_source.clone();

    // Validating again with same source should use cache
    assert!(panel.validate());
    assert!(panel.cached_shader.is_some());

    // Changing shader source should invalidate cache
    panel.shader_source = r#"
@compute @workgroup_size(1)
fn main() {
    // Different shader
}
"#
    .to_string();

    assert!(panel.validate());
    assert!(panel.cached_shader.is_some());

    // Verify the cached shader has the new source
    assert_ne!(
        panel.cached_shader.as_ref().unwrap().source(),
        first_source
    );
}

#[test]
fn test_compute_pipeline_panel_error_messages() {
    let mut panel = ComputePipelinePanel::new();

    // Empty entry point should produce error
    panel.entry_point_input = "".to_string();
    assert!(!panel.validate());
    assert!(panel.validation_error.is_some());
    assert!(panel
        .validation_error
        .as_ref()
        .unwrap()
        .contains("entry point"));

    // Empty shader should produce error
    panel.entry_point_input = "main".to_string();
    panel.shader_source = "".to_string();
    panel.cached_shader = None;
    assert!(!panel.validate());
    assert!(panel.validation_error.is_some());
}
