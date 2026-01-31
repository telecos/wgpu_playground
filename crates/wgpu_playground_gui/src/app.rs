use wgpu_playground_core::compute::ComputePanel;
use wgpu_playground_core::device_info::DeviceInfo;
use wgpu_playground_core::rendering::RenderingPanel;

pub struct PlaygroundApp {
    device_info: DeviceInfo,
    rendering_panel: RenderingPanel,
    compute_panel: ComputePanel,
    selected_tab: Tab,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tab {
    DeviceInfo,
    Rendering,
    Compute,
}

impl PlaygroundApp {
    pub fn new(adapter: &wgpu::Adapter, device: &wgpu::Device) -> Self {
        Self {
            device_info: DeviceInfo::new(adapter, device),
            rendering_panel: RenderingPanel::new(),
            compute_panel: ComputePanel::new(),
            selected_tab: Tab::DeviceInfo,
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        // Menu bar at the top
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.heading("🎮 WebGPU Playground");
        });

        // Sidebar on the left
        egui::SidePanel::left("sidebar").show(ctx, |ui| {
            ui.heading("Navigation");
            ui.separator();

            ui.selectable_value(&mut self.selected_tab, Tab::DeviceInfo, "📊 Device Info");
            ui.selectable_value(&mut self.selected_tab, Tab::Rendering, "🎨 Rendering");
            ui.selectable_value(&mut self.selected_tab, Tab::Compute, "🧮 Compute/ML");
        });

        // Main canvas area
        egui::CentralPanel::default().show(ctx, |ui| match self.selected_tab {
            Tab::DeviceInfo => self.device_info.ui(ui),
            Tab::Rendering => self.rendering_panel.ui(ui),
            Tab::Compute => self.compute_panel.ui(ui),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_enum_values() {
        // Test that Tab enum has the expected variants
        let tab1 = Tab::DeviceInfo;
        let tab2 = Tab::Rendering;
        let tab3 = Tab::Compute;

        // Test equality
        assert_eq!(tab1, Tab::DeviceInfo);
        assert_eq!(tab2, Tab::Rendering);
        assert_eq!(tab3, Tab::Compute);

        // Test inequality
        assert_ne!(tab1, tab2);
        assert_ne!(tab2, tab3);
        assert_ne!(tab3, tab1);
    }

    #[test]
    fn test_tab_clone() {
        let tab = Tab::DeviceInfo;
        let cloned = tab;
        assert_eq!(tab, cloned);
    }

    #[test]
    fn test_tab_copy() {
        let tab = Tab::Rendering;
        let copied = tab;
        // Verify both can be used independently
        assert_eq!(tab, Tab::Rendering);
        assert_eq!(copied, Tab::Rendering);
    }

    #[test]
    fn test_playground_app_creation() {
        // This test verifies that the app can be created with a GPU adapter/device
        // We use pollster to block on async GPU initialization
        pollster::block_on(async {
            let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
                backends: wgpu::Backends::all(),
                ..Default::default()
            });

            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    force_fallback_adapter: false,
                    compatible_surface: None,
                })
                .await;

            // Skip test if no GPU is available
            let Some(adapter) = adapter else {
                eprintln!("Skipping test: No GPU adapter available");
                return;
            };

            let device_result = adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        required_features: wgpu::Features::empty(),
                        required_limits: wgpu::Limits::default(),
                        label: Some("Test Device"),
                        memory_hints: Default::default(),
                    },
                    None,
                )
                .await;

            let Ok((device, _queue)) = device_result else {
                eprintln!("Skipping test: Failed to create device");
                return;
            };

            // Test that we can create a PlaygroundApp
            let _app = PlaygroundApp::new(&adapter, &device);
            // If we get here without panicking, the test passes
        });
    }
}
