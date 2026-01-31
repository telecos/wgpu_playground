//! Common test utilities for integration tests
//!
//! This module provides shared helper functions and utilities for integration tests,
//! reducing code duplication and ensuring consistent test setup.

use wgpu::{Adapter, Device, Instance, Queue};

/// Creates a test device and queue for integration testing.
///
/// This function attempts to create a wgpu instance, request an adapter,
/// and then request a device with default features and limits.
///
/// # Returns
///
/// Returns `Some((Device, Queue))` if successful, or `None` if no GPU adapter
/// is available (e.g., in headless environments or CI without GPU support).
///
/// # Example
///
/// ```no_run
/// # use wgpu_playground_core::tests::common::create_test_device;
/// # async fn test() {
/// let Some((device, queue)) = create_test_device().await else {
///     eprintln!("Skipping test: No GPU adapter available");
///     return;
/// };
/// // Use device and queue for testing
/// # }
/// ```
pub async fn create_test_device() -> Option<(Device, Queue)> {
    let instance = Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await?;

    adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: Some("Test Device"),
                memory_hints: Default::default(),
            },
            None,
        )
        .await
        .ok()
}

/// Creates a test device with specific features enabled.
///
/// # Arguments
///
/// * `features` - The GPU features to require
///
/// # Returns
///
/// Returns `Some((Device, Queue))` if successful and the adapter supports
/// the requested features, or `None` otherwise.
pub async fn create_test_device_with_features(features: wgpu::Features) -> Option<(Device, Queue)> {
    let instance = Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await?;

    // Check if adapter supports the requested features
    if !adapter.features().contains(features) {
        return None;
    }

    adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                required_features: features,
                required_limits: wgpu::Limits::default(),
                label: Some("Test Device with Features"),
                memory_hints: Default::default(),
            },
            None,
        )
        .await
        .ok()
}

/// Creates a test device with custom limits.
///
/// # Arguments
///
/// * `limits` - The GPU limits to require
///
/// # Returns
///
/// Returns `Some((Device, Queue))` if successful and the adapter supports
/// the requested limits, or `None` otherwise.
pub async fn create_test_device_with_limits(limits: wgpu::Limits) -> Option<(Device, Queue)> {
    let instance = Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await?;

    adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: limits,
                label: Some("Test Device with Limits"),
                memory_hints: Default::default(),
            },
            None,
        )
        .await
        .ok()
}

/// Creates a test instance and adapter for testing adapter capabilities.
///
/// # Returns
///
/// Returns `Some((Instance, Adapter))` if successful, or `None` if no GPU
/// adapter is available.
pub async fn create_test_instance_and_adapter() -> Option<(Instance, Adapter)> {
    let instance = Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await?;

    Some((instance, adapter))
}

/// Creates a simple WGSL shader source for testing.
///
/// Returns a basic vertex and fragment shader that can be used for
/// render pipeline tests.
pub fn create_test_shader_source() -> &'static str {
    r#"
@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    let x = f32(i32(in_vertex_index) - 1);
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
    return vec4<f32>(x, y, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
"#
}

/// Creates a simple compute shader source for testing.
///
/// Returns a basic compute shader that can be used for compute pipeline tests.
pub fn create_test_compute_shader_source() -> &'static str {
    r#"
@group(0) @binding(0)
var<storage, read_write> data: array<u32>;

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    data[global_id.x] = global_id.x;
}
"#
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shader_sources_are_valid() {
        let shader_source = create_test_shader_source();
        assert!(shader_source.contains("vs_main"));
        assert!(shader_source.contains("fs_main"));

        let compute_source = create_test_compute_shader_source();
        assert!(compute_source.contains("@compute"));
        assert!(compute_source.contains("workgroup_size"));
    }

    #[test]
    fn test_create_test_device_returns_some_or_none() {
        // This test ensures the function compiles and can be called
        pollster::block_on(async {
            let result = create_test_device().await;
            match result {
                Some((device, _queue)) => {
                    // Verify device is valid by checking limits
                    let limits = device.limits();
                    // All devices should have at least some max texture dimension
                    assert!(limits.max_texture_dimension_2d > 0);
                }
                None => {
                    // No GPU available - this is fine in some test environments
                    eprintln!("No GPU available for testing");
                }
            }
        });
    }
}
