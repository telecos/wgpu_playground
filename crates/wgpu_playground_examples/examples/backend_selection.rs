use wgpu::Backends;
/// Example demonstrating backend selection and enumeration
///
/// This example shows how to:
/// 1. Enumerate available GPU adapters for different backends
/// 2. Select a specific backend at runtime
/// 3. Create an instance with a specific backend configuration
///
/// Run with different backends:
/// ```bash
/// WGPU_BACKEND=vulkan cargo run --example backend_selection
/// WGPU_BACKEND=metal cargo run --example backend_selection
/// WGPU_BACKEND=dx12 cargo run --example backend_selection
/// ```
use wgpu_playground_core::adapter;

fn main() {
    env_logger::init();

    println!("=== WebGPU Backend Selection Example ===\n");

    // Check for environment variable
    let backend_choice = std::env::var("WGPU_BACKEND").unwrap_or_else(|_| "all".to_string());
    println!("Backend selection: {}\n", backend_choice);

    // Parse the backend
    let backends = adapter::parse_backends(&backend_choice).unwrap_or(Backends::all());
    println!("Using backends: {:?}\n", backends);

    // Enumerate all available adapters for the selected backends
    println!("--- Available Adapters ---");
    let adapters = adapter::enumerate_adapters(backends);

    if adapters.is_empty() {
        println!("❌ No adapters found for the selected backend(s)!");
        println!("\nTry running with a different backend:");
        println!("  WGPU_BACKEND=vulkan cargo run --example backend_selection");
        println!("  WGPU_BACKEND=metal cargo run --example backend_selection");
        println!("  WGPU_BACKEND=dx12 cargo run --example backend_selection");
        println!("  WGPU_BACKEND=all cargo run --example backend_selection");
        return;
    }

    for (i, adapter_info) in adapters.iter().enumerate() {
        println!("\n📱 Adapter #{}: {}", i + 1, adapter_info.name);
        println!("   Backend: {}", adapter_info.backend_name());
        println!("   Device Type: {:?}", adapter_info.device_type);
        println!("   Vendor ID: {}", adapter_info.vendor);
        println!("   Device ID: {}", adapter_info.device);
        println!("   Driver: {}", adapter_info.driver);
        if !adapter_info.driver_info.is_empty() {
            println!("   Driver Info: {}", adapter_info.driver_info);
        }
    }

    println!("\n--- Backend Comparison ---");
    println!("Available backend options:");
    for backend_name in adapter::available_backends() {
        println!("  • {}", backend_name);
    }

    println!("\n✨ Example completed successfully!");
    println!("\nTo test with a specific backend, run:");
    println!("  WGPU_BACKEND=<backend> cargo run --example backend_selection");
}
