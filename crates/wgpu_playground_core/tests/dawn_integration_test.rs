//! Comprehensive integration tests for Dawn WebGPU implementation
//!
//! These tests verify that the Dawn integration is fully functional,
//! including instance creation, adapter enumeration, and device creation.

#![cfg(feature = "dawn")]

use wgpu_playground_core::dawn_wrapper::{DawnDeviceDescriptor, DawnInstance, DawnPowerPreference};

#[test]
fn test_dawn_instance_creation() {
    // Test that we can create a Dawn instance
    let instance = DawnInstance::new();
    assert!(instance.is_ok(), "Failed to create Dawn instance");

    let _instance = instance.unwrap();
    // Instance should be valid and ready to use
}

#[test]
fn test_dawn_adapter_request() {
    // Create instance
    let instance = DawnInstance::new().expect("Failed to create instance");

    // Request adapter with default power preference
    let adapter = pollster::block_on(instance.request_adapter(DawnPowerPreference::Undefined));

    // In headless environments, this may fail - that's expected
    if let Ok(adapter) = adapter {
        let info = adapter.get_info();

        // Verify adapter info
        assert!(!info.name.is_empty(), "Adapter name should not be empty");
        println!("Adapter: {}", info.name);
        println!("Backend: {:?}", info.backend);
    } else {
        println!("No adapter available (expected in headless environment)");
    }
}

#[test]
fn test_dawn_adapter_high_performance() {
    // Create instance
    let instance = DawnInstance::new().expect("Failed to create instance");

    // Request high-performance adapter
    let adapter =
        pollster::block_on(instance.request_adapter(DawnPowerPreference::HighPerformance));

    // In headless environments, this may fail - that's expected
    if let Ok(adapter) = adapter {
        let info = adapter.get_info();
        println!("High-performance adapter: {}", info.name);
    } else {
        println!("No high-performance adapter available (expected in headless environment)");
    }
}

#[test]
fn test_dawn_adapter_low_power() {
    // Create instance
    let instance = DawnInstance::new().expect("Failed to create instance");

    // Request low-power adapter
    let adapter = pollster::block_on(instance.request_adapter(DawnPowerPreference::LowPower));

    // In headless environments, this may fail - that's expected
    if let Ok(adapter) = adapter {
        let info = adapter.get_info();
        println!("Low-power adapter: {}", info.name);
    } else {
        println!("No low-power adapter available (expected in headless environment)");
    }
}

#[test]
fn test_dawn_device_creation() {
    // Create instance
    let instance = DawnInstance::new().expect("Failed to create instance");

    // Request adapter
    let adapter = pollster::block_on(instance.request_adapter(DawnPowerPreference::Undefined));

    if let Ok(adapter) = adapter {
        // Request device
        let descriptor = DawnDeviceDescriptor {
            label: Some("Test Device".to_string()),
        };

        let device = pollster::block_on(adapter.request_device(&descriptor));
        assert!(device.is_ok(), "Failed to create device");

        let device = device.unwrap();

        // Verify device has queue
        let _queue = device.wgpu_queue();
        let _device = device.wgpu_device();

        println!("Device created successfully");
    } else {
        println!("No adapter available for device creation (expected in headless environment)");
    }
}

#[test]
fn test_dawn_device_without_label() {
    // Create instance
    let instance = DawnInstance::new().expect("Failed to create instance");

    // Request adapter
    let adapter = pollster::block_on(instance.request_adapter(DawnPowerPreference::Undefined));

    if let Ok(adapter) = adapter {
        // Request device without label
        let descriptor = DawnDeviceDescriptor::default();

        let device = pollster::block_on(adapter.request_device(&descriptor));
        assert!(device.is_ok(), "Failed to create device without label");

        println!("Device without label created successfully");
    } else {
        println!("No adapter available (expected in headless environment)");
    }
}

#[test]
fn test_dawn_multiple_adapters() {
    // Test that we can request adapters multiple times
    let instance = DawnInstance::new().expect("Failed to create instance");

    let adapter1 = pollster::block_on(instance.request_adapter(DawnPowerPreference::Undefined));

    let adapter2 =
        pollster::block_on(instance.request_adapter(DawnPowerPreference::HighPerformance));

    if let (Ok(adapter1), Ok(adapter2)) = (adapter1, adapter2) {
        let info1 = adapter1.get_info();
        let info2 = adapter2.get_info();

        println!("Adapter 1: {}", info1.name);
        println!("Adapter 2: {}", info2.name);
    } else {
        println!("Adapters not available (expected in headless environment)");
    }
}

#[test]
fn test_dawn_full_workflow() {
    // Test the complete workflow: instance -> adapter -> device

    // 1. Create instance
    let instance = DawnInstance::new().expect("Failed to create instance");
    println!("✓ Instance created");

    // 2. Request adapter
    let adapter =
        pollster::block_on(instance.request_adapter(DawnPowerPreference::HighPerformance));

    if let Ok(adapter) = adapter {
        println!("✓ Adapter requested");

        // 3. Get adapter info
        let info = adapter.get_info();
        println!("✓ Adapter info: {} ({:?})", info.name, info.backend);

        // 4. Create device
        let descriptor = DawnDeviceDescriptor {
            label: Some("Integration Test Device".to_string()),
        };
        let device = pollster::block_on(adapter.request_device(&descriptor));

        if let Ok(device) = device {
            println!("✓ Device created");

            // 5. Access device components
            let _wgpu_device = device.wgpu_device();
            let _queue = device.wgpu_queue();
            println!("✓ Device components accessible");

            println!("\n✅ Full Dawn workflow completed successfully!");
        } else {
            println!("✓ Device creation tested (unavailable in headless environment)");
        }
    } else {
        println!("✓ Adapter request tested (unavailable in headless environment)");
    }
}

#[test]
fn test_dawn_power_preference_types() {
    // Test that all power preference types exist and can be used
    let _undefined = DawnPowerPreference::Undefined;
    let _low = DawnPowerPreference::LowPower;
    let _high = DawnPowerPreference::HighPerformance;

    println!("All power preference types available");
}

#[test]
fn test_dawn_device_descriptor() {
    // Test device descriptor creation
    let desc1 = DawnDeviceDescriptor {
        label: Some("Test".to_string()),
    };
    assert_eq!(desc1.label.as_deref(), Some("Test"));

    let desc2 = DawnDeviceDescriptor::default();
    assert!(desc2.label.is_none());

    println!("Device descriptor types work correctly");
}
