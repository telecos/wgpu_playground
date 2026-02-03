# WASM Testing Guide

This document describes how to run and develop WASM-specific tests for the wgpu_playground project.

## Overview

The project includes comprehensive WASM-specific tests that verify:
- Web-sys integration (DOM, Canvas, WebGPU APIs)
- wasm-bindgen exports and JavaScript interop
- Browser-specific functionality
- TypedArray operations for buffer management
- Promise handling and async operations

## Test Files

The WASM tests are located in `crates/wgpu_playground_core/tests/`:

- **`wasm_integration_test.rs`** (17 tests): Tests web-sys APIs, WGPU instance creation, and browser APIs
- **`wasm_exports_test.rs`** (14 tests): Tests wasm-bindgen exports, type conversions, and JavaScript interop
- **`wasm_canvas_test.rs`** (13 tests): Tests HTML Canvas creation, manipulation, and WebGPU context

Total: **44 WASM-specific tests**

## Prerequisites

### Install wasm-pack

```bash
cargo install wasm-pack --version 0.14.0
```

### Install Browser for Testing

You need a browser installed for headless testing:

**Chrome/Chromium** (recommended):
```bash
# Ubuntu/Debian
sudo apt-get install chromium-browser chromium-chromedriver

# macOS
brew install --cask google-chrome
brew install chromedriver
```

**Firefox**:
```bash
# Ubuntu/Debian
sudo apt-get install firefox geckodriver

# macOS
brew install firefox
brew install geckodriver
```

## Running Tests

### Run all WASM tests with Chrome (headless)

```bash
cd crates/wgpu_playground_core
wasm-pack test --headless --chrome
```

### Run all WASM tests with Firefox (headless)

```bash
cd crates/wgpu_playground_core
wasm-pack test --headless --firefox
```

### Run tests in a visible browser window (for debugging)

```bash
cd crates/wgpu_playground_core
wasm-pack test --chrome
```

### Run specific test file

```bash
cd crates/wgpu_playground_core
wasm-pack test --headless --chrome --test wasm_canvas_test
```

### Run tests in Node.js

```bash
cd crates/wgpu_playground_core
wasm-pack test --node
```

Note: Some tests require browser APIs and will be skipped in Node.js.

## Test Configuration

The tests are configured with:

```toml
[target.'cfg(target_arch = "wasm32")'.dev-dependencies]
wasm-bindgen-test = "0.3"
```

Each test file uses:

```rust
#![cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);
```

## Writing New WASM Tests

### Basic Test Structure

```rust
#[wasm_bindgen_test]
fn test_my_feature() {
    // Your test code here
    assert!(true);
}
```

### Async Tests

```rust
#[wasm_bindgen_test]
async fn test_async_feature() {
    let result = some_async_function().await;
    assert!(result.is_ok());
}
```

### Testing WebGPU

```rust
#[wasm_bindgen_test]
async fn test_webgpu() {
    use wgpu::Instance;
    
    let instance = Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        ..Default::default()
    });
    
    // Note: GPU may not be available in all test environments
    if let Ok(adapter) = instance.request_adapter(&Default::default()).await {
        // Test with adapter
    }
}
```

### Testing Canvas

```rust
#[wasm_bindgen_test]
fn test_canvas() {
    use web_sys::{window, HtmlCanvasElement};
    use wasm_bindgen::JsCast;
    
    let window = window().expect("Should have window");
    let document = window.document().expect("Should have document");
    
    let canvas: HtmlCanvasElement = document
        .create_element("canvas")
        .expect("Should create canvas")
        .dyn_into()
        .expect("Should be canvas");
    
    canvas.set_width(800);
    canvas.set_height(600);
    
    assert_eq!(canvas.width(), 800);
}
```

## CI/CD Integration

The WASM tests are automatically run in CI via the `.github/workflows/wasm-builds.yml` workflow:

```yaml
- name: Test wgpu_playground_core WASM
  run: |
    cd crates/wgpu_playground_core
    wasm-pack test --headless --chrome
```

The workflow:
1. Installs Rust with the `wasm32-unknown-unknown` target
2. Installs wasm-pack
3. Installs Chrome for headless testing
4. Runs all WASM tests
5. Fails the build if any tests fail

## Debugging Tests

### Enable console output

Tests can log to the browser console:

```rust
use web_sys::console;

console::log_1(&"Debug message".into());
console::warn_1(&"Warning message".into());
```

### Run in visible browser

Remove the `--headless` flag to see the browser window:

```bash
wasm-pack test --chrome
```

### Use panic hook for better error messages

The project includes `console_error_panic_hook` which provides better stack traces in the browser console when panics occur.

## Common Issues

### Tests pass locally but fail in CI

- Check that all required web-sys features are enabled in Cargo.toml
- Verify that the browser version in CI supports the APIs being tested
- Some WebGPU features may not be available in headless mode

### Tests timeout

- Increase timeout with environment variable: `WASM_BINDGEN_TEST_TIMEOUT=300`
- Check for infinite loops or missing `.await` in async code

### JsCast not found

Add the import at the top of your test file:

```rust
use wasm_bindgen::JsCast;
```

## Resources

- [wasm-bindgen book](https://rustwasm.github.io/wasm-bindgen/)
- [wasm-bindgen-test documentation](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/index.html)
- [web-sys documentation](https://rustwasm.github.io/wasm-bindgen/web-sys/index.html)
- [WebGPU specification](https://www.w3.org/TR/webgpu/)
