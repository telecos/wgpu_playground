use serial_test::serial;
use wgpu_playground_core::assets;
use wgpu_playground_core::model_loader::load_model_from_file;

mod common;

#[test]
#[serial]
fn test_load_obj_cube() {
    let cube_path = assets::models_dir().join("cube.obj");

    let result = load_model_from_file(&cube_path);
    assert!(
        result.is_ok(),
        "Failed to load cube.obj: {:?}",
        result.err()
    );

    let model = result.unwrap();

    // Should have at least one mesh
    assert!(
        !model.meshes.is_empty(),
        "Model should have at least one mesh"
    );

    // OBJ files with normals/UVs will have more vertices than the base geometry
    // due to vertex duplication for different normal/UV combinations
    assert!(
        model.vertex_count >= 8,
        "Cube should have at least 8 vertices"
    );

    // Cube should have 36 indices (12 triangles * 3 indices)
    assert_eq!(
        model.index_count, 36,
        "Cube should have 36 indices (12 triangles)"
    );
}

#[test]
#[serial]
fn test_create_buffers_from_model() {
    pollster::block_on(async {
        // Skip test if no GPU device is available (e.g., in CI)
        let device_result = common::create_test_device().await;
        if device_result.is_none() {
            eprintln!("Skipping test: No GPU device available");
            return;
        }

        let (device, _queue) = device_result.unwrap();

        let cube_path = assets::models_dir().join("cube.obj");
        let model = load_model_from_file(&cube_path).expect("Failed to load cube.obj");

        // Test buffer creation
        let result = model.create_buffers(&device);
        assert!(
            result.is_ok(),
            "Failed to create buffers: {:?}",
            result.err()
        );

        let (vertex_buffer, index_buffer) = result.unwrap();

        // Verify buffers were created (non-zero size)
        assert!(
            vertex_buffer.size() > 0,
            "Vertex buffer should have non-zero size"
        );
        assert!(
            index_buffer.size() > 0,
            "Index buffer should have non-zero size"
        );
    });
}

#[test]
#[serial]
fn test_unsupported_format() {
    let fake_path = std::path::PathBuf::from("/tmp/fake.xyz");
    let result = load_model_from_file(&fake_path);

    assert!(result.is_err(), "Should fail for unsupported format");
    // Note: Can't use unwrap_err() without Debug trait on ModelData, so we'll just check is_err()
}

#[test]
#[serial]
fn test_missing_file() {
    let missing_path = assets::models_dir().join("nonexistent.obj");
    let result = load_model_from_file(&missing_path);

    assert!(result.is_err(), "Should fail for missing file");
}
