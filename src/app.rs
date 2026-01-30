use crate::compute::ComputePanel;
use crate::device_info::DeviceInfo;
use crate::rendering::RenderingPanel;

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
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.heading("🎮 WebGPU Playground");
            ui.separator();
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.selected_tab, Tab::DeviceInfo, "📊 Device Info");
                ui.selectable_value(&mut self.selected_tab, Tab::Rendering, "🎨 Rendering");
                ui.selectable_value(&mut self.selected_tab, Tab::Compute, "🧮 Compute/ML");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| match self.selected_tab {
            Tab::DeviceInfo => self.device_info.ui(ui),
            Tab::Rendering => self.rendering_panel.ui(ui),
            Tab::Compute => self.compute_panel.ui(ui),
        });
    }
}
