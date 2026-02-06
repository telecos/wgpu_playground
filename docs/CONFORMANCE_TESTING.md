# Backend Conformance Test Suite

This test suite provides micro-tests that exercise specific WebGPU API calls and verify identical behavior between Dawn and wgpu-core implementations.

## Overview

The conformance test suite is designed to:
- Test individual WebGPU operations in isolation
- Compare behavior between different backend implementations (Dawn vs wgpu-rs)
- Report conformance percentage to track cross-backend compatibility
- Identify divergent behaviors between implementations

## Test Categories

The suite covers the following WebGPU API categories:

### 1. Buffer Operations (3 tests)
- **buffer_create_vertex**: Creates a vertex buffer and verifies size
- **buffer_init_data**: Creates a buffer initialized with data
- **buffer_copy_ops**: Tests buffer-to-buffer copy operations

### 2. Texture Operations (2 tests)
- **texture_format_rgba8**: Creates an RGBA8 texture and verifies properties
- **texture_create_view**: Creates a texture view from a texture

### 3. Pipeline Creation (2 tests)
- **render_pipeline_basic**: Creates a basic render pipeline with vertex/fragment shaders
- **compute_pipeline_basic**: Creates a basic compute pipeline

### 4. Draw Calls (1 test)
- **draw_basic**: Executes a basic draw call with render pass

### 5. Compute Dispatch (1 test)
- **dispatch_1d**: Dispatches a compute shader with storage buffer

## Running the Tests

Run the conformance test suite:

```bash
cargo test --package wgpu_playground_core --test backend_conformance_suite -- --nocapture
```

Run specific test categories by name:

```bash
cargo test --package wgpu_playground_core --test backend_conformance_suite buffer -- --nocapture
```

## Test Output

The test suite produces a detailed report including:

- **Total Tests**: Number of micro-tests executed
- **Passing Tests**: Tests that passed on at least one backend
- **Conformance Percentage**: Percentage of tests with identical behavior across backends
- **Divergent Behaviors**: List of tests that behave differently between backends
- **Test Coverage**: Summary of test coverage by API category

### Example Output

```
========================================
Backend Conformance Test Report
========================================
Backends Tested: 2
Total Tests: 9
Passing Tests: 9 (100.0%)
Conformant: 9 (100.0%)

Test Coverage by API Category:
  ✓ Buffer Operations: 3 tests
    - create_vertex, init_data, copy_ops
  ✓ Texture Operations: 2 tests
    - format_rgba8, create_view
  ✓ Pipeline Creation: 2 tests
    - render_pipeline_basic, compute_pipeline_basic
  ✓ Draw Calls: 1 test
    - draw_basic
  ✓ Compute Dispatch: 1 test
    - dispatch_1d
========================================
```

## Adding New Tests

To add a new conformance test:

1. Create a new test function in the appropriate module (`buffer_ops`, `texture_ops`, etc.)
2. Follow the pattern of existing tests:
   - Accept `tracker: &ConformanceTracker` and `backend: &str` parameters
   - Return a `Result<(), String>` wrapped in an async block
   - Record the outcome using `tracker.record()`
3. Add the test call in `conformance_suite_run_all()`

### Example

```rust
pub async fn test_new_feature(tracker: &ConformanceTracker, backend: &str) {
    let test_name = "new_feature";
    
    let result = async {
        let Some((dev, q)) = create_test_device().await else {
            return Err("No device".to_string());
        };

        // Your test code here
        
        Ok(())
    }.await;

    tracker.record(TestOutcome {
        backend_name: backend.to_string(),
        test_name: test_name.to_string(),
        passed: result.is_ok(),
        error_message: result.err(),
    });
}
```

## CI/CD Integration

The test suite gracefully handles environments without GPU support:
- Tests are skipped when no GPU adapter is available
- Reports clearly indicate when tests were skipped
- Exit code is still 0 (success) to avoid breaking CI pipelines

## Future Enhancements

Planned improvements:
- Add support for testing with actual Dawn backend (when available)
- Add more comprehensive texture format tests
- Add tests for advanced pipeline configurations
- Add tests for indirect drawing
- Add tests for multi-sampled render targets
- Add performance benchmarks alongside conformance tests
