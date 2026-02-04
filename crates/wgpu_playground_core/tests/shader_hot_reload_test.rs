use wgpu_playground_core::shader::ShaderModule;
use wgpu_playground_core::shader_watcher::ShaderWatcher;
use serial_test::serial;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

// Helper function to get a path to a test shader file
fn get_test_shader_path() -> PathBuf {
    let assets_dir = wgpu_playground_core::assets::shaders_dir();
    assets_dir.join("test_hot_reload.wgsl")
}

// Helper function to create a test shader file
fn create_test_shader(content: &str) -> PathBuf {
    let path = get_test_shader_path();
    let mut file = fs::File::create(&path).expect("Failed to create test shader file");
    file.write_all(content.as_bytes())
        .expect("Failed to write test shader");
    file.sync_all().expect("Failed to sync file");
    path
}

// Helper function to update a test shader file
fn update_test_shader(content: &str) {
    let path = get_test_shader_path();
    let mut file = fs::File::create(&path).expect("Failed to update test shader file");
    file.write_all(content.as_bytes())
        .expect("Failed to write test shader");
    file.sync_all().expect("Failed to sync file");
    
    // Give the filesystem and watcher a moment to detect the change
    thread::sleep(Duration::from_millis(100));
}

// Helper function to clean up test shader
fn cleanup_test_shader() {
    let path = get_test_shader_path();
    if path.exists() {
        let _ = fs::remove_file(path);
    }
}

#[test]
#[serial]
#[cfg(not(target_arch = "wasm32"))]
fn test_shader_watcher_detects_changes() {
    // Create a test shader file
    let initial_content = r#"
@vertex
fn vs_main() -> @builtin(position) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}
"#;
    
    create_test_shader(initial_content);
    
    // Create watcher
    let watcher = ShaderWatcher::new().expect("Failed to create shader watcher");
    
    // Wait a moment for watcher to initialize
    thread::sleep(Duration::from_millis(200));
    
    // Modify the shader file
    let modified_content = r#"
@vertex
fn vs_main() -> @builtin(position) vec4<f32> {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}
"#;
    update_test_shader(modified_content);
    
    // Wait for the watcher to detect the change
    thread::sleep(Duration::from_millis(500));
    
    // Poll for changes
    let events = watcher.poll_all();
    
    // Clean up
    cleanup_test_shader();
    
    // Verify that we got a change event
    let found_change = events.iter().any(|e| e.filename == "test_hot_reload.wgsl");
    assert!(
        found_change,
        "Expected to detect change to test_hot_reload.wgsl, but got events: {:?}",
        events
    );
}

#[test]
#[serial]
#[cfg(not(target_arch = "wasm32"))]
fn test_shader_module_reload() {
    // Create a test shader file with no leading newline
    let initial_content = "@fragment\nfn fs_main() -> @location(0) vec4<f32> {\n    return vec4<f32>(1.0, 0.0, 0.0, 1.0);\n}";
    
    create_test_shader(initial_content);
    
    // Create shader module from file
    let mut shader = ShaderModule::from_file("test_hot_reload.wgsl", Some("test_shader"))
        .expect("Failed to create shader module");
    
    // Verify initial content (just check for the vec4 literal)
    let initial_source = shader.source();
    assert!(
        initial_source.contains("vec4<f32>(1.0, 0.0, 0.0, 1.0)"),
        "Initial shader should contain red color, got: {:?}",
        initial_source
    );
    
    // Modify the file - green color instead of red
    let modified_content = "@fragment\nfn fs_main() -> @location(0) vec4<f32> {\n    return vec4<f32>(0.0, 1.0, 0.0, 1.0);\n}";
    update_test_shader(modified_content);
    
    // Reload the shader
    let reload_result = shader.reload().expect("Failed to reload shader");
    
    // Clean up
    cleanup_test_shader();
    
    // Verify reload happened and content changed
    assert!(reload_result, "Expected shader to be reloaded");
    let reloaded_source = shader.source();
    assert!(
        reloaded_source.contains("vec4<f32>(0.0, 1.0, 0.0, 1.0)"),
        "Expected shader content to change after reload to green, got: {:?}",
        reloaded_source
    );
}

#[test]
fn test_shader_module_reload_inline_shader() {
    // Create an inline shader
    let mut shader = ShaderModule::from_source(
        "@vertex fn main() -> @builtin(position) vec4<f32> { return vec4<f32>(0.0); }",
        Some("inline_shader"),
    )
    .expect("Failed to create inline shader");
    
    // Try to reload (should be no-op for inline shaders)
    let reload_result = shader.reload().expect("Reload should succeed for inline shaders");
    
    // Verify no reload happened
    assert!(!reload_result, "Expected no reload for inline shader");
}

#[test]
#[serial]
#[cfg(not(target_arch = "wasm32"))]
fn test_shader_module_reload_with_same_content() {
    // Create a test shader file
    let content = r#"
@compute @workgroup_size(1)
fn cs_main() {
    // Do nothing
}
"#;
    
    create_test_shader(content);
    
    // Create shader module from file
    let mut shader = ShaderModule::from_file("test_hot_reload.wgsl", Some("test_shader"))
        .expect("Failed to create shader module");
    
    // Reload without changing the file
    let reload_result = shader.reload().expect("Failed to reload shader");
    
    // Clean up
    cleanup_test_shader();
    
    // Verify no reload happened since content is the same
    assert!(!reload_result, "Expected no reload when content is unchanged");
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn test_shader_watcher_multiple_files() {
    // Create two test shader files
    let shader1_path = wgpu_playground_core::assets::shaders_dir().join("test_shader1.wgsl");
    let shader2_path = wgpu_playground_core::assets::shaders_dir().join("test_shader2.wgsl");
    
    let content = "@vertex fn main() -> @builtin(position) vec4<f32> { return vec4<f32>(0.0); }";
    
    fs::write(&shader1_path, content).expect("Failed to create test shader 1");
    fs::write(&shader2_path, content).expect("Failed to create test shader 2");
    
    // Create watcher
    let watcher = ShaderWatcher::new().expect("Failed to create shader watcher");
    
    // Wait for watcher to initialize
    thread::sleep(Duration::from_millis(200));
    
    // Modify both files
    let modified_content =
        "@vertex fn main() -> @builtin(position) vec4<f32> { return vec4<f32>(1.0); }";
    
    fs::write(&shader1_path, modified_content).expect("Failed to update shader 1");
    thread::sleep(Duration::from_millis(100));
    fs::write(&shader2_path, modified_content).expect("Failed to update shader 2");
    
    // Wait for the watcher to detect the changes
    thread::sleep(Duration::from_millis(500));
    
    // Poll for changes
    let events = watcher.poll_all();
    
    // Clean up
    let _ = fs::remove_file(shader1_path);
    let _ = fs::remove_file(shader2_path);
    
    // Verify that we got change events for both files
    let shader1_changed = events.iter().any(|e| e.filename == "test_shader1.wgsl");
    let shader2_changed = events.iter().any(|e| e.filename == "test_shader2.wgsl");
    
    assert!(
        shader1_changed || shader2_changed,
        "Expected to detect changes to at least one test shader, got events: {:?}",
        events
    );
}
