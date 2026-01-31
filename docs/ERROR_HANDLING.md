# Error Handling in wgpu_playground_core

This document describes the comprehensive error handling system implemented in wgpu_playground_core.

## Overview

The error handling system provides:
- **Centralized error types** for different categories of GPU errors
- **Error scopes** for capturing errors during specific operations
- **Error callbacks** for handling uncaptured errors
- **Device lost callbacks** for handling GPU device loss
- **Comprehensive logging** throughout the API

## Error Types

The library defines several error categories:

### ErrorType Enum

```rust
pub enum ErrorType {
    Validation,    // API usage violations
    OutOfMemory,   // GPU memory exhaustion
    Internal,      // Driver/hardware issues
    DeviceLost,    // GPU device loss or reset
}
```

### Module-Specific Errors

Each module defines its own error types:
- `BufferError` - Buffer creation and mapping errors
- `ShaderError` - Shader loading and compilation errors
- `RenderPipelineError` - Pipeline creation and validation errors
- `AdapterError` - Adapter selection errors
- `QueueError` - Queue operation errors
- `CommandEncoderError` - Command encoding errors

## Error Scopes

Error scopes allow you to catch specific types of GPU errors during a sequence of operations.

### Basic Usage

```rust
use wgpu_playground_core::error::{ErrorScope, ErrorFilter};

// Push an error scope
ErrorScope::push(&device, ErrorFilter::Validation);

// Perform GPU operations that might generate errors
// ...

// Pop the scope and check for errors
if let Some(error) = ErrorScope::pop(&device).await {
    eprintln!("Caught error: {}", error);
}
```

### Error Filters

Three types of error filters are available:
- `ErrorFilter::Validation` - Captures validation errors
- `ErrorFilter::OutOfMemory` - Captures out-of-memory errors
- `ErrorFilter::Internal` - Captures internal errors

## Device-Level Error Handling

### Setup

Use `setup_device_error_handling()` to configure automatic error handling:

```rust
use wgpu_playground_core::error::setup_device_error_handling;

// After creating the device
setup_device_error_handling(&device);
```

This sets up:
- Uncaptured error callback (logs all errors not caught by error scopes)
- Device lost callback (logs device loss events)

### Custom Error Handlers

For more control, use the `ErrorHandler` type:

```rust
use wgpu_playground_core::error::ErrorHandler;

let mut handler = ErrorHandler::new();

// Register custom error callback
handler.on_error(|error| {
    eprintln!("Custom handler caught: {}", error);
    // Handle error...
});

// Create wgpu callback
let callback = handler.create_wgpu_callback();
device.on_uncaptured_error(callback);
```

## Validation

Most operations provide validation before they interact with the GPU:

### Buffer Validation

```rust
use wgpu_playground_core::buffer::{BufferDescriptor, BufferUsages};

let descriptor = BufferDescriptor::new(
    Some("my_buffer"),
    256,
    BufferUsages::UNIFORM | BufferUsages::COPY_DST
);

// Validate before creating
match descriptor.validate() {
    Ok(_) => {
        let buffer = descriptor.create_buffer(&device)?;
        // Use buffer...
    }
    Err(e) => {
        eprintln!("Invalid buffer configuration: {}", e);
    }
}
```

### Shader Validation

```rust
use wgpu_playground_core::shader::ShaderModule;

// Empty source is automatically rejected
match ShaderModule::from_source("", Some("test")) {
    Ok(_) => println!("Shader created"),
    Err(e) => println!("Shader error: {}", e),
}
```

## Logging

The library uses the `log` crate for structured logging. Configure logging levels:

```rust
env_logger::Builder::from_env(
    env_logger::Env::default().default_filter_or("info")
).init();
```

### Log Levels

- `ERROR` - Critical errors (device loss, uncaptured errors)
- `WARN` - Warnings (error scope captures)
- `INFO` - Important events (adapter selection, pipeline creation)
- `DEBUG` - Detailed operation info (buffer creation, shader loading)
- `TRACE` - Very detailed info (internal operations)

## Common Patterns

### Creating a Buffer with Error Handling

```rust
use wgpu_playground_core::buffer::{BufferDescriptor, BufferUsages};

let descriptor = BufferDescriptor::new(
    Some("vertex_buffer"),
    1024,
    BufferUsages::VERTEX | BufferUsages::COPY_DST
);

match descriptor.create_buffer(&device) {
    Ok(buffer) => {
        // Use buffer
    }
    Err(e) => {
        log::error!("Failed to create buffer: {}", e);
        // Handle error
    }
}
```

### Loading a Shader with Error Handling

```rust
use wgpu_playground_core::shader::ShaderModule;

match ShaderModule::from_file("shader.wgsl", Some("my_shader")) {
    Ok(shader) => {
        let module = shader.create_module(&device);
        // Use module
    }
    Err(e) => {
        log::error!("Failed to load shader: {}", e);
        // Handle error
    }
}
```

### Using Error Scopes for Validation

```rust
use wgpu_playground_core::error::{ErrorScope, ErrorFilter};

// Push validation error scope
ErrorScope::push(&device, ErrorFilter::Validation);

// Create potentially invalid resources
let buffer = descriptor.create_buffer(&device)?;
let shader = shader_module.create_module(&device);

// Check for validation errors
if let Some(error) = ErrorScope::pop(&device).await {
    log::error!("Validation error detected: {}", error);
    // Handle error
}
```

## Example

See `examples/error_handling.rs` for a comprehensive example demonstrating:
- Device error handling setup
- Error scope usage
- Validation error handling
- Buffer mapping with error handling
- Shader error handling

Run the example:
```bash
cargo run --example error_handling
```

## Best Practices

1. **Always validate** before creating GPU resources when possible
2. **Use error scopes** for critical sections of code
3. **Set up device callbacks** early in your application
4. **Log errors** at appropriate levels
5. **Handle device loss** gracefully by recreating resources
6. **Check return values** from all fallible operations
7. **Use descriptive labels** for all GPU resources to aid debugging

## Testing

The library includes comprehensive error handling tests in:
- `tests/error_handling_test.rs` - Validation and error type tests
- Unit tests in each module for error conditions

Run tests:
```bash
cargo test -p wgpu_playground_core
```

## Future Improvements

Planned enhancements:
- Automatic error recovery mechanisms
- Error reporting aggregation
- Performance impact monitoring
- WebAssembly-specific error handling
