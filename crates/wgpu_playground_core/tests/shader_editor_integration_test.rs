/// Integration tests for the shader editor module
use wgpu_playground_core::shader_editor::{CompilationResult, ShaderEditor};

#[cfg(test)]
mod common;

#[test]
fn test_shader_editor_create_and_validate() {
    let mut editor = ShaderEditor::new();

    // Editor should have default shader code
    assert!(!editor.source_code().is_empty());

    // Should be able to validate the default shader
    assert!(editor.validate());
}

#[test]
fn test_shader_editor_set_invalid_shader() {
    let mut editor = ShaderEditor::new();

    // Set invalid shader code (empty)
    editor.set_source_code("".to_string());

    // Validation should fail
    assert!(!editor.validate());

    // Compilation result should be error
    match editor.compilation_result() {
        CompilationResult::Error(_) => {}
        _ => panic!("Expected compilation error for empty shader"),
    }
}

#[test]
fn test_shader_editor_set_valid_shader() {
    let mut editor = ShaderEditor::new();

    // Set valid shader code
    let valid_shader = r#"
        @vertex
        fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
            return vec4<f32>(0.0, 0.0, 0.0, 1.0);
        }
        
        @fragment
        fn fs_main() -> @location(0) vec4<f32> {
            return vec4<f32>(1.0, 1.0, 1.0, 1.0);
        }
    "#;

    editor.set_source_code(valid_shader.to_string());

    // Validation should succeed
    assert!(editor.validate());
}

#[test]
fn test_shader_editor_compile_with_device() {
    // This test requires a GPU device
    pollster::block_on(async {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .await;

        // Skip test if no GPU is available
        let Ok(adapter) = adapter else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let device_result = adapter
            .request_device(&wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: Some("Test Device"),
                memory_hints: Default::default(),
                experimental_features: Default::default(),
                trace: Default::default(),
            })
            .await;

        let Ok((device, _queue)) = device_result else {
            eprintln!("Skipping test: Failed to create device");
            return;
        };

        let mut editor = ShaderEditor::new();

        // Compile the default shader
        editor.compile(&device);

        // Compilation should succeed
        match editor.compilation_result() {
            CompilationResult::Success => {}
            CompilationResult::Error(msg) => {
                panic!("Expected successful compilation, got error: {}", msg)
            }
            CompilationResult::NotCompiled => panic!("Expected compilation to occur"),
        }
    });
}

#[test]
fn test_shader_editor_load_example() {
    let mut editor = ShaderEditor::new();

    // Try to load example shader
    editor.load_from_file("example.wgsl");

    // The file should load successfully
    assert!(!editor.source_code().is_empty());
    assert!(editor.source_code().contains("@vertex"));
    assert!(editor.source_code().contains("@fragment"));
}

#[test]
fn test_shader_editor_load_nonexistent_file() {
    let mut editor = ShaderEditor::new();

    // Try to load a file that doesn't exist
    editor.load_from_file("nonexistent.wgsl");

    // Should have an error in compilation result
    match editor.compilation_result() {
        CompilationResult::Error(msg) => {
            assert!(msg.contains("Failed to load file"));
        }
        _ => panic!("Expected error when loading nonexistent file"),
    }
}

#[test]
fn test_shader_editor_reset_clears_error() {
    let mut editor = ShaderEditor::new();

    // Set invalid code
    editor.set_source_code("".to_string());
    assert!(!editor.validate());

    // Reset should clear the error and restore default code
    editor.set_source_code(ShaderEditor::new().source_code().to_string());

    // Should be valid again
    assert!(editor.validate());
}
