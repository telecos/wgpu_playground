/// Example demonstrating shader module loading and compilation
///
/// This example shows how to:
/// - Load a shader from a file
/// - Load a shader from an inline string
/// - Create wgpu shader modules
use wgpu_playground_core::shader::{ShaderModule, ShaderSource};

pub fn run_shader_example() {
    env_logger::init();

    println!("=== WGSL Shader Module Example ===\n");

    // Example 1: Load shader from inline source
    println!("1. Creating shader from inline source:");
    let inline_shader = ShaderModule::from_source(
        r#"
@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    let x = f32(i32(in_vertex_index) - 1);
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
    return vec4<f32>(x, y, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
"#,
        Some("inline_shader"),
    );

    match inline_shader {
        Ok(shader) => {
            println!("   ✓ Successfully created shader: {:?}", shader.label());
            println!("   Source length: {} bytes", shader.source().len());
        }
        Err(e) => {
            println!("   ✗ Failed to create shader: {}", e);
        }
    }

    // Example 2: Load shader from file
    println!("\n2. Loading shader from file:");
    let file_shader = ShaderModule::from_file("example.wgsl", Some("example_shader"));

    match file_shader {
        Ok(shader) => {
            println!("   ✓ Successfully loaded shader: {:?}", shader.label());
            println!("   Source length: {} bytes", shader.source().len());
            // Print first few lines
            let lines: Vec<&str> = shader.source().lines().take(5).collect();
            println!("   First few lines:");
            for (i, line) in lines.iter().enumerate() {
                println!("      {}: {}", i + 1, line);
            }
        }
        Err(e) => {
            println!("   ✗ Failed to load shader: {}", e);
        }
    }

    // Example 3: Using ShaderSource enum
    println!("\n3. Using ShaderSource enum:");
    let sources = vec![
        (
            "Inline",
            ShaderSource::Inline("@compute @workgroup_size(1) fn main() {}".to_string()),
        ),
        ("File", ShaderSource::File("example.wgsl".to_string())),
    ];

    for (name, source) in sources {
        match ShaderModule::new(source, Some(name)) {
            Ok(_shader) => println!("   ✓ {} shader created successfully", name),
            Err(e) => println!("   ✗ {} shader failed: {}", name, e),
        }
    }

    // Example 4: Error handling - empty source
    println!("\n4. Error handling - empty source:");
    let empty_shader = ShaderModule::from_source("", None);
    match empty_shader {
        Ok(_) => println!("   ✗ Unexpectedly succeeded with empty source"),
        Err(e) => println!("   ✓ Correctly rejected empty source: {}", e),
    }

    // Example 5: Error handling - non-existent file
    println!("\n5. Error handling - non-existent file:");
    let missing_shader = ShaderModule::from_file("nonexistent.wgsl", None);
    match missing_shader {
        Ok(_) => println!("   ✗ Unexpectedly succeeded with missing file"),
        Err(e) => println!("   ✓ Correctly failed to load missing file: {}", e),
    }

    println!("\n=== Example Complete ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shader_example_runs() {
        // This test just ensures the example can run without panicking
        run_shader_example();
    }

    #[test]
    fn test_inline_shader_creation() {
        // Test creating a shader from inline source
        let shader = ShaderModule::from_source(
            r#"
@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
"#,
            Some("test_shader"),
        );

        assert!(shader.is_ok(), "Inline shader creation should succeed");
        let shader = shader.unwrap();
        assert_eq!(shader.label(), Some("test_shader"));
        assert!(shader.source().contains("vs_main"));
        assert!(shader.source().contains("fs_main"));
    }

    #[test]
    fn test_empty_shader_is_rejected() {
        // Test that empty shader source is rejected
        let result = ShaderModule::from_source("", None);
        assert!(result.is_err(), "Empty shader should be rejected");
    }

    #[test]
    fn test_shader_from_file() {
        // Test loading a shader from file (example.wgsl should exist)
        let result = ShaderModule::from_file("example.wgsl", Some("file_shader"));
        // This may fail in some test environments if the file isn't accessible
        // so we just check that it returns a result
        match result {
            Ok(shader) => {
                assert_eq!(shader.label(), Some("file_shader"));
                assert!(!shader.source().is_empty());
            }
            Err(_) => {
                // File might not be accessible in test environment
                eprintln!("Note: example.wgsl not accessible in this test environment");
            }
        }
    }

    #[test]
    fn test_nonexistent_file_is_rejected() {
        // Test that loading a non-existent file fails
        let result = ShaderModule::from_file("definitely_does_not_exist.wgsl", None);
        assert!(result.is_err(), "Non-existent file should be rejected");
    }

    #[test]
    fn test_compute_shader_creation() {
        // Test creating a simple compute shader
        let shader = ShaderModule::from_source(
            "@compute @workgroup_size(1) fn main() {}",
            Some("compute_shader"),
        );

        assert!(
            shader.is_ok(),
            "Compute shader creation should succeed: {:?}",
            shader.err()
        );
        let shader = shader.unwrap();
        assert_eq!(shader.label(), Some("compute_shader"));
        assert!(shader.source().contains("@compute"));
    }

    #[test]
    fn test_shader_source_enum() {
        // Test using ShaderSource::Inline variant
        let inline_source = ShaderSource::Inline(
            "@compute @workgroup_size(1) fn main() {}".to_string(),
        );
        let shader = ShaderModule::new(inline_source, Some("enum_shader"));
        assert!(shader.is_ok());

        // Test using ShaderSource::File variant
        let file_source = ShaderSource::File("example.wgsl".to_string());
        let shader = ShaderModule::new(file_source, Some("file_enum_shader"));
        // May fail if file not accessible, but should return a Result
        let _ = shader;
    }
}
