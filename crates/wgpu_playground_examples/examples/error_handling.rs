use pollster::FutureExt;
use wgpu_playground_core::adapter::{create_instance, request_adapter, AdapterOptions};
use wgpu_playground_core::buffer::{BufferDescriptor, BufferUsages};
use wgpu_playground_core::error::{setup_device_error_handling, ErrorFilter, ErrorScope};
use wgpu_playground_core::shader::ShaderModule;

/// This example demonstrates comprehensive error handling in wgpu_playground_core
///
/// It shows:
/// 1. Setting up device-level error callbacks
/// 2. Using error scopes to catch GPU errors
/// 3. Handling validation errors
/// 4. Proper logging configuration
fn main() {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // Create instance and request adapter
    let instance = create_instance(wgpu::Backends::PRIMARY);
    let adapter_options = AdapterOptions::default();

    let adapter = request_adapter(&instance, &adapter_options, None)
        .block_on()
        .expect("Failed to find adapter");

    // Request device
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: Some("Error Handling Example Device"),
                memory_hints: Default::default(),
            },
            None,
        )
        .block_on()
        .expect("Failed to create device");

    // Set up comprehensive error handling
    // This configures callbacks for device loss and uncaptured errors
    setup_device_error_handling(&device);

    println!("=== Error Handling Examples ===\n");

    // Example 1: Validation errors are caught automatically
    println!("1. Testing automatic error logging for invalid buffer:");
    let invalid_buffer = BufferDescriptor::new(Some("invalid"), 0, BufferUsages::UNIFORM);
    match invalid_buffer.validate() {
        Ok(_) => println!("   Buffer is valid"),
        Err(e) => println!("   ✓ Caught validation error: {}", e),
    }
    println!();

    // Example 2: Using error scopes to catch GPU errors
    println!("2. Testing error scope for validation errors:");
    ErrorScope::push(&device, ErrorFilter::Validation);

    // This would cause a validation error if actually executed on GPU
    // (simulated here for demonstration)
    let _valid_buffer = BufferDescriptor::new(
        Some("test_buffer"),
        256,
        BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    )
    .create_buffer(&device)
    .expect("Failed to create buffer");

    // Pop the error scope and check for errors
    let error = ErrorScope::pop(&device).block_on();
    match error {
        Some(e) => println!("   ✓ Error scope caught: {}", e),
        None => println!("   ✓ No errors in this scope"),
    }
    println!();

    // Example 3: Testing shader validation
    println!("3. Testing shader error handling:");

    // Empty shader source - should fail validation
    let empty_shader_result = ShaderModule::from_source("", Some("empty"));
    match empty_shader_result {
        Ok(_) => println!("   Shader created (unexpected)"),
        Err(e) => println!("   ✓ Caught shader error: {}", e),
    }

    // Valid shader source
    let _valid_shader = ShaderModule::from_source(
        r#"
        @vertex
        fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
            return vec4<f32>(0.0, 0.0, 0.0, 1.0);
        }
        "#,
        Some("valid_shader"),
    )
    .expect("Failed to create shader");
    println!("   ✓ Valid shader created successfully");
    println!();

    // Example 4: Buffer mapping with error handling
    println!("4. Testing buffer operations:");

    // Create a mappable buffer
    use wgpu_playground_core::buffer::BufferOps;
    let map_buffer = BufferDescriptor::new(
        Some("map_test"),
        256,
        BufferUsages::MAP_READ | BufferUsages::COPY_DST,
    )
    .create_buffer(&device)
    .expect("Failed to create mappable buffer");

    // Write some data to the buffer
    use wgpu_playground_core::queue::QueueOps;
    let queue_ops = QueueOps::new(&queue);
    let data = vec![0u8; 256];
    queue_ops.write_buffer(&map_buffer, 0, &data);

    // Map the buffer for reading
    BufferOps::map_read(&map_buffer)
        .block_on()
        .expect("Failed to map buffer");
    println!("   ✓ Buffer mapped successfully");

    // Read the data
    let view = BufferOps::get_mapped_range(&map_buffer);
    println!("   ✓ Read {} bytes from buffer", view.len());
    drop(view);

    // Unmap the buffer
    BufferOps::unmap(&map_buffer);
    println!("   ✓ Buffer unmapped");
    println!();

    // Example 5: Error scope for out-of-memory errors
    println!("5. Testing error scope for out-of-memory errors:");
    ErrorScope::push(&device, ErrorFilter::OutOfMemory);

    // Create a reasonably sized buffer (won't actually cause OOM)
    let _large_buffer = BufferDescriptor::new(
        Some("large_buffer"),
        1024 * 1024, // 1MB
        BufferUsages::STORAGE | BufferUsages::COPY_DST,
    )
    .create_buffer(&device)
    .expect("Failed to create buffer");

    let oom_error = ErrorScope::pop(&device).block_on();
    match oom_error {
        Some(e) => println!("   ✗ Out of memory: {}", e),
        None => println!("   ✓ No out-of-memory errors"),
    }
    println!();

    println!("=== All error handling examples completed successfully! ===");
}
