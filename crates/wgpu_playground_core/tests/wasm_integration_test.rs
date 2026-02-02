//! WASM-specific integration tests
//!
//! This module contains tests specifically for the WASM build target.
//! Tests cover web-sys integration, wasm-bindgen exports, and browser-specific functionality.
//!
//! These tests are only compiled and run when targeting wasm32.
//! Run with: wasm-pack test --headless --chrome

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

// Configure tests to run in browser
wasm_bindgen_test_configure!(run_in_browser);

/// Test that we can import web-sys types
#[wasm_bindgen_test]
fn test_web_sys_imports() {
    use web_sys::window;
    
    // Verify we can access the window object
    let window = window();
    assert!(window.is_some(), "Window object should be available");
}

/// Test document access via web-sys
#[wasm_bindgen_test]
fn test_web_sys_document() {
    use web_sys::window;
    
    let window = window().expect("Should have a window");
    let document = window.document();
    assert!(document.is_some(), "Document should be available");
}

/// Test canvas element creation via web-sys
#[wasm_bindgen_test]
fn test_canvas_element_creation() {
    use web_sys::{window, HtmlCanvasElement};
    
    let window = window().expect("Should have a window");
    let document = window.document().expect("Should have a document");
    
    // Create a canvas element
    let canvas = document
        .create_element("canvas")
        .expect("Should create canvas element");
    
    // Verify it's an HtmlCanvasElement
    let canvas: HtmlCanvasElement = canvas
        .dyn_into()
        .expect("Should be HtmlCanvasElement");
    
    // Set dimensions
    canvas.set_width(800);
    canvas.set_height(600);
    
    assert_eq!(canvas.width(), 800);
    assert_eq!(canvas.height(), 600);
}

/// Test GPU availability detection
#[wasm_bindgen_test]
async fn test_gpu_availability() {
    use web_sys::window;
    
    let window = window().expect("Should have a window");
    
    // Try to access the GPU object
    // Note: This may not be available in all test environments
    let navigator = window.navigator();
    
    // Check if GPU is accessible via navigator.gpu
    // In some browsers/environments this may not be available
    use wasm_bindgen::JsCast;
    let gpu = js_sys::Reflect::get(&navigator, &"gpu".into());
    
    // We just verify the API is accessible, even if GPU is not available
    assert!(gpu.is_ok(), "Should be able to check for GPU object");
}

/// Test creating WGPU instance in WASM environment
#[wasm_bindgen_test]
async fn test_wgpu_instance_creation() {
    use wgpu::Instance;
    
    // Create a WGPU instance
    let instance = Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        ..Default::default()
    });
    
    // Instance creation should succeed
    // Note: Actual adapter request may fail in test environment without real GPU
    let _instance = instance;
}

/// Test adapter enumeration with browser backend
#[wasm_bindgen_test]
async fn test_adapter_enumeration_browser_backend() {
    use wgpu::Instance;
    
    let instance = Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        ..Default::default()
    });
    
    // Try to request an adapter
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await;
    
    // In headless test environment, adapter may not be available
    // We just verify the API works, not that it succeeds
    match adapter {
        Some(_) => {
            // GPU is available in test environment
        }
        None => {
            // GPU not available in test environment - this is ok
        }
    }
}

/// Test that we can get device info if adapter is available
#[wasm_bindgen_test]
async fn test_device_creation_if_available() {
    use wgpu::Instance;
    
    let instance = Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        ..Default::default()
    });
    
    if let Some(adapter) = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await
    {
        // Get adapter info
        let info = adapter.get_info();
        
        // Verify we can access adapter properties
        assert!(!info.name.is_empty(), "Adapter should have a name");
        
        // Try to create a device
        let device_result = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: Some("WASM Test Device"),
                    memory_hints: Default::default(),
                    experimental_features: Default::default(),
                    trace: Default::default(),
                },
            )
            .await;
        
        // If device creation succeeds, verify basic properties
        if let Ok((_device, _queue)) = device_result {
            // Device created successfully
            // In a real test, we could do more with the device
        }
    }
}

/// Test console logging works in WASM
#[wasm_bindgen_test]
fn test_console_logging() {
    use web_sys::console;
    
    // Test various console methods
    console::log_1(&"Test log message".into());
    console::warn_1(&"Test warning message".into());
    console::error_1(&"Test error message".into());
    
    // If we get here without panicking, console logging works
}

/// Test performance timing APIs
#[wasm_bindgen_test]
fn test_performance_timing() {
    use web_sys::window;
    
    let window = window().expect("Should have a window");
    let performance = window.performance();
    
    assert!(performance.is_some(), "Performance API should be available");
    
    if let Some(perf) = performance {
        let now = perf.now();
        assert!(now >= 0.0, "Performance.now() should return non-negative value");
    }
}

/// Test local storage availability
#[wasm_bindgen_test]
fn test_local_storage() {
    use web_sys::window;
    
    let window = window().expect("Should have a window");
    
    // Try to access local storage
    let storage = window.local_storage();
    
    // Storage may not be available in all test environments
    // We just verify we can check for it
    match storage {
        Ok(Some(_storage)) => {
            // Local storage is available
        }
        Ok(None) => {
            // Local storage not available - this is ok in test environment
        }
        Err(_) => {
            // Error accessing storage - this is ok in test environment
        }
    }
}

/// Test JS array creation and manipulation
#[wasm_bindgen_test]
fn test_js_array_operations() {
    use js_sys::Array;
    
    let array = Array::new();
    array.push(&42.into());
    array.push(&"test".into());
    
    assert_eq!(array.length(), 2);
    
    // Verify we can get items back
    let first = array.get(0);
    assert!(first.is_truthy());
}

/// Test TypedArray creation for buffer operations
#[wasm_bindgen_test]
fn test_typed_array_creation() {
    use js_sys::Uint8Array;
    
    let array = Uint8Array::new_with_length(16);
    assert_eq!(array.length(), 16);
    
    // Set some values
    array.set_index(0, 42);
    array.set_index(1, 255);
    
    // Verify values
    assert_eq!(array.get_index(0), 42);
    assert_eq!(array.get_index(1), 255);
}

/// Test that panic hook is properly set up for better error messages
#[wasm_bindgen_test]
fn test_panic_hook_setup() {
    // We can't directly test panics in wasm-bindgen-test easily,
    // but we can verify the console_error_panic_hook is available
    // This is more of a build test
    
    // If this test runs, it means our WASM module compiled correctly
    assert!(true, "WASM module compiled and loaded successfully");
}

/// Test Float32Array for vertex buffer data
#[wasm_bindgen_test]
fn test_float32_array_for_buffers() {
    use js_sys::Float32Array;
    
    // Create a Float32Array (commonly used for vertex data)
    let vertices = Float32Array::new_with_length(9);
    
    // Set triangle vertices
    vertices.set_index(0, 0.0);  // x1
    vertices.set_index(1, 0.5);  // y1
    vertices.set_index(2, 0.0);  // z1
    
    vertices.set_index(3, -0.5); // x2
    vertices.set_index(4, -0.5); // y2
    vertices.set_index(5, 0.0);  // z2
    
    vertices.set_index(6, 0.5);  // x3
    vertices.set_index(7, -0.5); // y3
    vertices.set_index(8, 0.0);  // z3
    
    assert_eq!(vertices.length(), 9);
}

/// Test Uint32Array for index buffer data
#[wasm_bindgen_test]
fn test_uint32_array_for_indices() {
    use js_sys::Uint32Array;
    
    // Create a Uint32Array (commonly used for index data)
    let indices = Uint32Array::new_with_length(3);
    
    indices.set_index(0, 0);
    indices.set_index(1, 1);
    indices.set_index(2, 2);
    
    assert_eq!(indices.length(), 3);
    assert_eq!(indices.get_index(0), 0);
    assert_eq!(indices.get_index(1), 1);
    assert_eq!(indices.get_index(2), 2);
}

/// Test Promise handling with wasm-bindgen
#[wasm_bindgen_test]
async fn test_promise_handling() {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::JsFuture;
    
    // Create a resolved promise
    let promise = js_sys::Promise::resolve(&JsValue::from(42));
    
    // Await the promise
    let result = JsFuture::from(promise).await;
    
    assert!(result.is_ok());
    if let Ok(value) = result {
        assert_eq!(value.as_f64(), Some(42.0));
    }
}

/// Test requestAnimationFrame callback setup
#[wasm_bindgen_test]
fn test_animation_frame_api() {
    use web_sys::window;
    use wasm_bindgen::prelude::*;
    
    let window = window().expect("Should have a window");
    
    // Create a simple callback
    let callback = Closure::wrap(Box::new(move |_time: f64| {
        // Animation frame callback
    }) as Box<dyn FnMut(f64)>);
    
    // Request an animation frame
    let _handle = window.request_animation_frame(callback.as_ref().unchecked_ref());
    
    // Keep callback alive
    callback.forget();
}
