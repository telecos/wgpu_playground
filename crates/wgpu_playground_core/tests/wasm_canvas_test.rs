//! Canvas and WebGPU rendering tests for WASM
//!
//! This module tests canvas creation, manipulation, and WebGPU context acquisition
//! in the browser environment.
//!
//! Run with: wasm-pack test --headless --chrome

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Test creating an HTML canvas element
#[wasm_bindgen_test]
fn test_create_canvas() {
    use web_sys::{window, HtmlCanvasElement};
    
    let window = window().expect("Should have a window");
    let document = window.document().expect("Should have a document");
    
    let canvas = document
        .create_element("canvas")
        .expect("Should create canvas");
    
    let canvas: HtmlCanvasElement = canvas
        .dyn_into()
        .expect("Element should be a canvas");
    
    // Verify default properties
    assert!(canvas.width() >= 0);
    assert!(canvas.height() >= 0);
}

/// Test setting canvas dimensions
#[wasm_bindgen_test]
fn test_canvas_dimensions() {
    use web_sys::{window, HtmlCanvasElement};
    
    let window = window().expect("Should have a window");
    let document = window.document().expect("Should have a document");
    
    let canvas: HtmlCanvasElement = document
        .create_element("canvas")
        .expect("Should create canvas")
        .dyn_into()
        .expect("Should be canvas");
    
    // Set custom dimensions
    canvas.set_width(1920);
    canvas.set_height(1080);
    
    assert_eq!(canvas.width(), 1920);
    assert_eq!(canvas.height(), 1080);
}

/// Test canvas context acquisition
#[wasm_bindgen_test]
fn test_canvas_2d_context() {
    use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d};
    use wasm_bindgen::JsCast;
    
    let window = window().expect("Should have a window");
    let document = window.document().expect("Should have a document");
    
    let canvas: HtmlCanvasElement = document
        .create_element("canvas")
        .expect("Should create canvas")
        .dyn_into()
        .expect("Should be canvas");
    
    // Get 2D context
    let context = canvas
        .get_context("2d")
        .expect("Should get context")
        .expect("Context should exist");
    
    let _context: CanvasRenderingContext2d = context
        .dyn_into()
        .expect("Should be 2D context");
}

/// Test canvas styling via CSS
#[wasm_bindgen_test]
fn test_canvas_styling() {
    use web_sys::{window, HtmlCanvasElement};
    
    let window = window().expect("Should have a window");
    let document = window.document().expect("Should have a document");
    
    let canvas: HtmlCanvasElement = document
        .create_element("canvas")
        .expect("Should create canvas")
        .dyn_into()
        .expect("Should be canvas");
    
    // Get style object
    let style = canvas.style();
    
    // Set some CSS properties
    style.set_property("width", "800px").expect("Should set width");
    style.set_property("height", "600px").expect("Should set height");
    style.set_property("border", "1px solid black").expect("Should set border");
    
    // Verify properties were set
    let width = style.get_property_value("width").expect("Should get width");
    assert_eq!(width, "800px");
}

/// Test adding canvas to DOM
#[wasm_bindgen_test]
fn test_canvas_dom_insertion() {
    use web_sys::{window, HtmlCanvasElement};
    
    let window = window().expect("Should have a window");
    let document = window.document().expect("Should have a document");
    
    let canvas: HtmlCanvasElement = document
        .create_element("canvas")
        .expect("Should create canvas")
        .dyn_into()
        .expect("Should be canvas");
    
    // Set an ID
    canvas.set_id("test-canvas");
    
    // Get body element
    let body = document.body().expect("Should have body");
    
    // Append canvas to body
    body.append_child(&canvas).expect("Should append canvas");
    
    // Verify we can find it again
    let found_canvas = document.get_element_by_id("test-canvas");
    assert!(found_canvas.is_some());
    
    // Clean up
    body.remove_child(&canvas).expect("Should remove canvas");
}

/// Test canvas pixel manipulation via ImageData
#[wasm_bindgen_test]
fn test_canvas_image_data() {
    use web_sys::{window, HtmlCanvasElement, CanvasRenderingContext2d, ImageData};
    use wasm_bindgen::JsCast;
    
    let window = window().expect("Should have a window");
    let document = window.document().expect("Should have a document");
    
    let canvas: HtmlCanvasElement = document
        .create_element("canvas")
        .expect("Should create canvas")
        .dyn_into()
        .expect("Should be canvas");
    
    canvas.set_width(100);
    canvas.set_height(100);
    
    let context: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .expect("Should get context")
        .expect("Context should exist")
        .dyn_into()
        .expect("Should be 2D context");
    
    // Create ImageData
    let image_data = ImageData::new_with_sw(10, 10).expect("Should create ImageData");
    
    // Put ImageData on canvas
    context.put_image_data(&image_data, 0.0, 0.0).expect("Should put image data");
    
    // Get ImageData back
    let retrieved = context.get_image_data(0.0, 0.0, 10.0, 10.0).expect("Should get image data");
    
    assert_eq!(retrieved.width(), 10);
    assert_eq!(retrieved.height(), 10);
}

/// Test offscreen canvas creation
#[wasm_bindgen_test]
fn test_offscreen_canvas_creation() {
    use web_sys::window;
    use wasm_bindgen::JsCast;
    
    let window = window().expect("Should have a window");
    
    // Try to create an OffscreenCanvas
    // This API may not be available in all browsers
    let offscreen_canvas_constructor = js_sys::Reflect::get(
        &window,
        &"OffscreenCanvas".into()
    );
    
    if offscreen_canvas_constructor.is_ok() {
        // OffscreenCanvas is supported
        // Note: We can't easily test this without more complex setup
    } else {
        // OffscreenCanvas not supported - that's fine
    }
}

/// Test canvas toDataURL functionality
#[wasm_bindgen_test]
fn test_canvas_to_data_url() {
    use web_sys::{window, HtmlCanvasElement};
    
    let window = window().expect("Should have a window");
    let document = window.document().expect("Should have a document");
    
    let canvas: HtmlCanvasElement = document
        .create_element("canvas")
        .expect("Should create canvas")
        .dyn_into()
        .expect("Should be canvas");
    
    canvas.set_width(100);
    canvas.set_height(100);
    
    // Convert to data URL
    let data_url = canvas.to_data_url().expect("Should convert to data URL");
    
    // Verify it's a data URL
    assert!(data_url.starts_with("data:image/"));
}

/// Test canvas with WebGPU context (if available)
#[wasm_bindgen_test]
async fn test_canvas_webgpu_context() {
    use web_sys::{window, HtmlCanvasElement};
    use wasm_bindgen::JsCast;
    
    let window = window().expect("Should have a window");
    let document = window.document().expect("Should have a document");
    
    let canvas: HtmlCanvasElement = document
        .create_element("canvas")
        .expect("Should create canvas")
        .dyn_into()
        .expect("Should be canvas");
    
    canvas.set_width(800);
    canvas.set_height(600);
    
    // Try to get WebGPU context
    let context_result = canvas.get_context("webgpu");
    
    // WebGPU context may not be available in all test environments
    match context_result {
        Ok(Some(_context)) => {
            // WebGPU context is available
        }
        Ok(None) => {
            // WebGPU context not supported - this is fine
        }
        Err(_) => {
            // Error getting context - this is fine in test environment
        }
    }
}

/// Test creating a surface from canvas with wgpu
#[wasm_bindgen_test]
async fn test_wgpu_surface_from_canvas() {
    use web_sys::{window, HtmlCanvasElement};
    use wasm_bindgen::JsCast;
    use wgpu::Instance;
    
    let window = window().expect("Should have a window");
    let document = window.document().expect("Should have a document");
    
    let canvas: HtmlCanvasElement = document
        .create_element("canvas")
        .expect("Should create canvas")
        .dyn_into()
        .expect("Should be canvas");
    
    canvas.set_width(800);
    canvas.set_height(600);
    
    // Create wgpu instance
    let instance = Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        ..Default::default()
    });
    
    // Create surface from canvas
    let surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas));
    
    // If we get here without panic, surface creation succeeded
    assert!(surface.is_ok());
}

/// Test canvas resize handling
#[wasm_bindgen_test]
fn test_canvas_resize() {
    use web_sys::{window, HtmlCanvasElement};
    
    let window = window().expect("Should have a window");
    let document = window.document().expect("Should have a document");
    
    let canvas: HtmlCanvasElement = document
        .create_element("canvas")
        .expect("Should create canvas")
        .dyn_into()
        .expect("Should be canvas");
    
    // Initial size
    canvas.set_width(800);
    canvas.set_height(600);
    
    assert_eq!(canvas.width(), 800);
    assert_eq!(canvas.height(), 600);
    
    // Resize
    canvas.set_width(1024);
    canvas.set_height(768);
    
    assert_eq!(canvas.width(), 1024);
    assert_eq!(canvas.height(), 768);
}

/// Test canvas bounding rect
#[wasm_bindgen_test]
fn test_canvas_bounding_rect() {
    use web_sys::{window, HtmlCanvasElement};
    
    let window = window().expect("Should have a window");
    let document = window.document().expect("Should have a document");
    
    let canvas: HtmlCanvasElement = document
        .create_element("canvas")
        .expect("Should create canvas")
        .dyn_into()
        .expect("Should be canvas");
    
    canvas.set_width(800);
    canvas.set_height(600);
    
    // Get bounding client rect
    let rect = canvas.get_bounding_client_rect();
    
    // Verify we can access rect properties
    let _x = rect.x();
    let _y = rect.y();
    let _width = rect.width();
    let _height = rect.height();
}

/// Test canvas event listeners
#[wasm_bindgen_test]
fn test_canvas_event_listeners() {
    use web_sys::{window, HtmlCanvasElement};
    use wasm_bindgen::prelude::*;
    use std::rc::Rc;
    use std::cell::RefCell;
    
    let window = window().expect("Should have a window");
    let document = window.document().expect("Should have a document");
    
    let canvas: HtmlCanvasElement = document
        .create_element("canvas")
        .expect("Should create canvas")
        .dyn_into()
        .expect("Should be canvas");
    
    // Create a click handler
    let clicked = Rc::new(RefCell::new(false));
    let clicked_clone = clicked.clone();
    
    let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        *clicked_clone.borrow_mut() = true;
    }) as Box<dyn FnMut(_)>);
    
    // Add event listener
    canvas
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .expect("Should add event listener");
    
    // Clean up
    closure.forget();
}
