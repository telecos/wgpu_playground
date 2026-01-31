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
