use wgpu_playground_core::implementation::WebGPUImplementation;

#[test]
fn test_implementation_available() {
    let implementations = WebGPUImplementation::available_implementations();
    assert!(!implementations.is_empty());
    assert!(implementations.contains(&WebGPUImplementation::Wgpu));
}

#[test]
fn test_implementation_current() {
    let current = WebGPUImplementation::current();
    assert!(WebGPUImplementation::available_implementations().contains(&current));
}

#[test]
fn test_implementation_properties() {
    let wgpu = WebGPUImplementation::Wgpu;
    assert_eq!(wgpu.name(), "wgpu");
    assert!(wgpu.description().contains("wgpu-rs"));
    assert!(wgpu.url().contains("github.com"));
    assert!(wgpu.is_native());
    assert!(wgpu.status_message().contains("Native"));
}

#[test]
#[cfg(feature = "dawn")]
fn test_dawn_properties() {
    let dawn = WebGPUImplementation::Dawn;
    assert_eq!(dawn.name(), "Dawn");
    assert!(dawn.description().contains("Dawn"));
    assert!(dawn.url().contains("dawn.googlesource.com"));
    assert!(dawn.is_native()); // Dawn implementation is fully integrated
    let status = dawn.status_message();
    // Status depends on whether Dawn C++ library was built
    assert!(
        status.contains("Dawn") || status.contains("fallback"),
        "Expected Dawn status message, got: {}",
        status
    );
}

#[test]
fn test_available_implementations_list() {
    let list = WebGPUImplementation::available_implementations_list();
    assert!(list.contains("wgpu"));
}

#[test]
#[cfg(feature = "dawn")]
fn test_dawn_feature_enabled() {
    assert!(WebGPUImplementation::is_dawn_available());
    let implementations = WebGPUImplementation::available_implementations();
    assert_eq!(implementations.len(), 2);
    assert!(implementations.contains(&WebGPUImplementation::Dawn));
}

#[test]
#[cfg(not(feature = "dawn"))]
fn test_dawn_feature_disabled() {
    assert!(!WebGPUImplementation::is_dawn_available());
    let implementations = WebGPUImplementation::available_implementations();
    assert_eq!(implementations.len(), 1);
    assert_eq!(implementations[0], WebGPUImplementation::Wgpu);
}
