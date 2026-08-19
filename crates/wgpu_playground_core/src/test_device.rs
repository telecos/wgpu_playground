//! Shared GPU device helper for in-crate unit tests.
//!
//! Mirrors the helper used by the integration tests in `tests/common`, so unit
//! tests that exercise private GPU code paths can skip gracefully when no
//! adapter is available.

use wgpu::{Device, Queue};

/// Creates a device and queue for unit tests.
///
/// Returns `None` when no GPU adapter is available (for example on machines
/// without software rendering), allowing callers to skip the test.
pub(crate) async fn create_test_device() -> Option<(Device, Queue)> {
    let is_headless = std::env::var("CI").is_ok() || std::env::var("WGPU_HEADLESS").is_ok();

    let backends = if is_headless {
        wgpu::Backends::VULKAN | wgpu::Backends::GL
    } else {
        wgpu::Backends::all()
    };

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends,
        ..wgpu::InstanceDescriptor::new_without_display_handle()
    });

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: is_headless,
            compatible_surface: None,
            apply_limit_buckets: false,
        })
        .await
        .ok()?;

    adapter
        .request_device(&wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            label: Some("Unit Test Device"),
            memory_hints: Default::default(),
            experimental_features: Default::default(),
            trace: Default::default(),
        })
        .await
        .ok()
}
