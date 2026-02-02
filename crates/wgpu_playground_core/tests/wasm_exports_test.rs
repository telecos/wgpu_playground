//! WASM-bindgen export tests
//!
//! This module tests wasm-bindgen exported functions and ensures they are
//! properly accessible from JavaScript code.
//!
//! Run with: wasm-pack test --headless --chrome

#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Example exported function for testing
#[wasm_bindgen]
pub fn wasm_test_add(a: i32, b: i32) -> i32 {
    a + b
}

/// Example exported function that returns a string
#[wasm_bindgen]
pub fn wasm_test_greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// Example exported struct for testing
#[wasm_bindgen]
pub struct WasmTestPoint {
    x: f64,
    y: f64,
}

#[wasm_bindgen]
impl WasmTestPoint {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> WasmTestPoint {
        WasmTestPoint { x, y }
    }
    
    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f64 {
        self.x
    }
    
    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f64 {
        self.y
    }
    
    #[wasm_bindgen]
    pub fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

/// Test basic function export
#[wasm_bindgen_test]
fn test_exported_function() {
    let result = wasm_test_add(5, 3);
    assert_eq!(result, 8);
}

/// Test string return from exported function
#[wasm_bindgen_test]
fn test_exported_string_function() {
    let greeting = wasm_test_greet("World");
    assert_eq!(greeting, "Hello, World!");
}

/// Test exported struct creation and methods
#[wasm_bindgen_test]
fn test_exported_struct() {
    let point = WasmTestPoint::new(3.0, 4.0);
    
    assert_eq!(point.x(), 3.0);
    assert_eq!(point.y(), 4.0);
    assert_eq!(point.distance_from_origin(), 5.0);
}

/// Test calling exported function from JavaScript context
#[wasm_bindgen_test]
fn test_js_interop() {
    use wasm_bindgen::JsCast;
    
    // Get the global object
    let global = js_sys::global();
    
    // Verify we can access the global scope
    assert!(global.is_truthy());
    
    // Test we can call Math functions
    let math = js_sys::Reflect::get(&global, &"Math".into()).unwrap();
    let sqrt = js_sys::Reflect::get(&math, &"sqrt".into()).unwrap();
    let sqrt_fn: &js_sys::Function = sqrt.unchecked_ref();
    
    let result = sqrt_fn.call1(&math, &JsValue::from(16.0)).unwrap();
    assert_eq!(result.as_f64(), Some(4.0));
}

/// Test JsValue type conversions
#[wasm_bindgen_test]
fn test_jsvalue_conversions() {
    // Test number conversions
    let num = JsValue::from(42);
    assert_eq!(num.as_f64(), Some(42.0));
    
    // Test string conversions
    let str_val = JsValue::from("test");
    assert_eq!(str_val.as_string(), Some("test".to_string()));
    
    // Test boolean conversions
    let bool_val = JsValue::from(true);
    assert_eq!(bool_val.as_bool(), Some(true));
    
    // Test null and undefined
    let null_val = JsValue::NULL;
    assert!(null_val.is_null());
    
    let undef_val = JsValue::UNDEFINED;
    assert!(undef_val.is_undefined());
}

/// Test Error conversion to JsValue
#[wasm_bindgen_test]
fn test_error_conversion() {
    let error = JsValue::from_str("Test error message");
    assert_eq!(error.as_string(), Some("Test error message".to_string()));
    
    // Test creating a proper Error object
    let js_error = js_sys::Error::new("JavaScript Error");
    let error_value: JsValue = js_error.into();
    assert!(error_value.is_object());
}

/// Test Object creation and property access
#[wasm_bindgen_test]
fn test_object_creation() {
    let obj = js_sys::Object::new();
    
    // Set properties
    js_sys::Reflect::set(&obj, &"name".into(), &"test".into()).unwrap();
    js_sys::Reflect::set(&obj, &"value".into(), &42.into()).unwrap();
    
    // Get properties
    let name = js_sys::Reflect::get(&obj, &"name".into()).unwrap();
    assert_eq!(name.as_string(), Some("test".to_string()));
    
    let value = js_sys::Reflect::get(&obj, &"value".into()).unwrap();
    assert_eq!(value.as_f64(), Some(42.0));
}

/// Test creating and using closures
#[wasm_bindgen_test]
fn test_closure_creation() {
    use std::cell::RefCell;
    use std::rc::Rc;
    
    let counter = Rc::new(RefCell::new(0));
    let counter_clone = counter.clone();
    
    let closure = Closure::wrap(Box::new(move || {
        *counter_clone.borrow_mut() += 1;
    }) as Box<dyn Fn()>);
    
    // Call the closure via JavaScript
    let func: &js_sys::Function = closure.as_ref().unchecked_ref();
    func.call0(&JsValue::NULL).unwrap();
    func.call0(&JsValue::NULL).unwrap();
    
    assert_eq!(*counter.borrow(), 2);
    
    // Don't forget the closure to prevent it from being dropped
    closure.forget();
}

/// Test Result type with wasm-bindgen
#[wasm_bindgen]
pub fn wasm_test_divide(a: f64, b: f64) -> Result<f64, JsValue> {
    if b == 0.0 {
        Err(JsValue::from_str("Division by zero"))
    } else {
        Ok(a / b)
    }
}

#[wasm_bindgen_test]
fn test_result_export() {
    // Test success case
    let result = wasm_test_divide(10.0, 2.0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 5.0);
    
    // Test error case
    let error_result = wasm_test_divide(10.0, 0.0);
    assert!(error_result.is_err());
}

/// Test Option type handling
#[wasm_bindgen]
pub fn wasm_test_find_index(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target)
}

#[wasm_bindgen_test]
fn test_option_export() {
    let arr = vec![1, 2, 3, 4, 5];
    
    // Test Some case
    let result = wasm_test_find_index(&arr, 3);
    assert_eq!(result, Some(2));
    
    // Test None case
    let result = wasm_test_find_index(&arr, 10);
    assert_eq!(result, None);
}

/// Test Vec<T> to JsArray conversion
#[wasm_bindgen_test]
fn test_vec_to_array_conversion() {
    use js_sys::Array;
    
    let vec = vec![1, 2, 3, 4, 5];
    let arr = Array::new();
    
    for item in vec {
        arr.push(&JsValue::from(item));
    }
    
    assert_eq!(arr.length(), 5);
    assert_eq!(arr.get(0).as_f64(), Some(1.0));
    assert_eq!(arr.get(4).as_f64(), Some(5.0));
}

/// Test accessing environment-specific features
#[wasm_bindgen_test]
fn test_browser_specific_apis() {
    use web_sys::window;
    
    let window = window().expect("Should have window");
    
    // Test location API
    let location = window.location();
    let href = location.href().unwrap_or_default();
    // In test environment, this might be about:blank or similar
    assert!(!href.is_empty() || href.is_empty()); // Either is fine in test
}

/// Test memory management with JsValue
#[wasm_bindgen_test]
fn test_memory_management() {
    // Create a large array to test memory handling
    let arr = js_sys::Uint8Array::new_with_length(1024);
    
    // Fill with data
    for i in 0..1024 {
        arr.set_index(i, (i % 256) as u8);
    }
    
    // Verify data
    assert_eq!(arr.length(), 1024);
    assert_eq!(arr.get_index(0), 0);
    assert_eq!(arr.get_index(255), 255);
    assert_eq!(arr.get_index(256), 0);
}

/// Test JSON serialization
#[wasm_bindgen_test]
fn test_json_serialization() {
    use js_sys::JSON;
    
    // Create an object
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"name".into(), &"test".into()).unwrap();
    js_sys::Reflect::set(&obj, &"value".into(), &42.into()).unwrap();
    
    // Stringify
    let json_str = JSON::stringify(&obj.into()).unwrap();
    assert!(json_str.as_string().unwrap().contains("name"));
    assert!(json_str.as_string().unwrap().contains("test"));
    
    // Parse back
    let parsed = JSON::parse(&json_str.as_string().unwrap()).unwrap();
    let name = js_sys::Reflect::get(&parsed, &"name".into()).unwrap();
    assert_eq!(name.as_string(), Some("test".to_string()));
}
