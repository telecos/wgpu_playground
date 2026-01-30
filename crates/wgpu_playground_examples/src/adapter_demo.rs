use wgpu_playground_core::adapter::{
    enumerate_adapters, format_adapter_features, get_adapter_features, request_adapter,
    AdapterOptions,
};

#[tokio::main]
async fn main() {
    env_logger::init();
    
    println!("=== GPU Adapter Enumeration Demo ===\n");
    
    // Enumerate all available adapters
    println!("Available GPU adapters:");
    let adapters = enumerate_adapters(wgpu::Backends::all());
    
    if adapters.is_empty() {
        println!("No GPU adapters found!");
        return;
    }
    
    for (i, adapter_info) in adapters.iter().enumerate() {
        println!("\nAdapter {}:", i + 1);
        println!("{}", adapter_info.format());
        println!("{}", "-".repeat(80));
    }
    
    // Request adapter with different options
    println!("\n=== Requesting Adapter ===\n");
    
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });
    
    // Try with default options
    println!("Requesting adapter with default options...");
    let options = AdapterOptions::default();
    match request_adapter(&instance, &options, None).await {
        Ok(adapter) => {
            let info = adapter.get_info();
            println!("✓ Successfully requested adapter: {}", info.name);
            println!("  Backend: {:?}", info.backend);
            println!("  Device Type: {:?}", info.device_type);
            
            // Get features
            let features = get_adapter_features(&adapter);
            println!("\n  Supported features:");
            println!("  {}", format_adapter_features(&features));
        }
        Err(e) => {
            println!("✗ Failed to request adapter: {}", e);
        }
    }
    
    // Try with high performance preference
    println!("\n\nRequesting adapter with high performance preference...");
    let options = AdapterOptions::high_performance();
    match request_adapter(&instance, &options, None).await {
        Ok(adapter) => {
            let info = adapter.get_info();
            println!("✓ Successfully requested adapter: {}", info.name);
            println!("  Backend: {:?}", info.backend);
            println!("  Device Type: {:?}", info.device_type);
        }
        Err(e) => {
            println!("✗ Failed to request adapter: {}", e);
        }
    }
    
    println!("\n=== Demo Complete ===");
}
