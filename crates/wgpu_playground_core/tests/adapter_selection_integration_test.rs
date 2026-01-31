use wgpu_playground_core::adapter_selection::AdapterSelectionPanel;
use wgpu::PowerPreference;

// Helper function to create a test adapter
async fn create_test_adapter() -> Option<wgpu::Adapter> {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await
}

#[test]
fn test_adapter_selection_panel_creation() {
    pollster::block_on(async {
        let Some(adapter) = create_test_adapter().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let panel = AdapterSelectionPanel::new(&adapter);
        
        // Verify panel was created successfully
        assert!(panel.selected_adapter().is_some());
        
        // Power preference should be the default value
        assert_eq!(panel.power_preference(), PowerPreference::default());
    });
}

#[test]
fn test_adapter_enumeration() {
    pollster::block_on(async {
        let Some(adapter) = create_test_adapter().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let panel = AdapterSelectionPanel::new(&adapter);
        
        // There should be at least one adapter if we got here
        assert!(panel.selected_adapter().is_some());
        
        let selected = panel.selected_adapter().unwrap();
        
        // Verify adapter info contains expected fields
        assert!(!selected.name.is_empty(), "Adapter name should not be empty");
        // Device type should be one of the valid types
        assert!(matches!(
            selected.device_type,
            wgpu::DeviceType::DiscreteGpu
                | wgpu::DeviceType::IntegratedGpu
                | wgpu::DeviceType::VirtualGpu
                | wgpu::DeviceType::Cpu
                | wgpu::DeviceType::Other
        ));
    });
}

#[test]
fn test_power_preference_values() {
    pollster::block_on(async {
        let Some(adapter) = create_test_adapter().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let panel = AdapterSelectionPanel::new(&adapter);
        
        // Test that power preference can be retrieved
        let pref = panel.power_preference();
        
        // Should be one of the valid power preference values
        assert!(matches!(
            pref,
            PowerPreference::None | PowerPreference::LowPower | PowerPreference::HighPerformance
        ));
    });
}
