use wgpu_playground_core::adapter_selection::AdapterSelectionPanel;
use wgpu_playground_core::bind_group_layout_panel::BindGroupLayoutPanel;
use wgpu_playground_core::bind_group_panel::BindGroupPanel;
use wgpu_playground_core::buffer_panel::BufferPanel;
use wgpu_playground_core::compute::ComputePanel;
use wgpu_playground_core::compute_dispatch_panel::ComputeDispatchPanel;
use wgpu_playground_core::compute_pipeline_panel::ComputePipelinePanel;
use wgpu_playground_core::console::ConsolePanel;
use wgpu_playground_core::device_config::DeviceConfigPanel;
use wgpu_playground_core::device_info::DeviceInfo;
use wgpu_playground_core::draw_command_panel::DrawCommandPanel;
use wgpu_playground_core::performance_panel::PerformancePanel;
use wgpu_playground_core::render_pass_panel::RenderPassPanel;
use wgpu_playground_core::render_pipeline_panel::RenderPipelinePanel;
use wgpu_playground_core::rendering::RenderingPanel;
use wgpu_playground_core::resource_inspector::ResourceInspectorPanel;
use wgpu_playground_core::sampler_panel::SamplerPanel;
use wgpu_playground_core::texture_panel::TexturePanel;

pub struct PlaygroundApp {
    device_info: DeviceInfo,
    device_config: DeviceConfigPanel,
    adapter_selection: AdapterSelectionPanel,
    rendering_panel: RenderingPanel,
    compute_panel: ComputePanel,
    compute_pipeline_panel: ComputePipelinePanel,
    compute_dispatch_panel: ComputeDispatchPanel,
    buffer_panel: BufferPanel,
    sampler_panel: SamplerPanel,
    texture_panel: TexturePanel,
    bind_group_panel: BindGroupPanel,
    bind_group_layout_panel: BindGroupLayoutPanel,
    render_pipeline_panel: RenderPipelinePanel,
    console_panel: ConsolePanel,
    draw_command_panel: DrawCommandPanel,
    render_pass_panel: RenderPassPanel,
    resource_inspector_panel: ResourceInspectorPanel,
    performance_panel: PerformancePanel,
    selected_tab: Tab,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tab {
    AdapterSelection,
    DeviceConfig,
    DeviceInfo,
    Rendering,
    BufferConfig,
    SamplerConfig,
    TextureConfig,
    BindGroupConfig,
    BindGroupLayoutConfig,
    ComputePipelineConfig,
    RenderPipelineConfig,
    DrawCommand,
    RenderPassConfig,
    ComputeDispatch,
    Compute,
    Console,
    ResourceInspector,
    Performance,
}

impl PlaygroundApp {
    pub fn new(adapter: &wgpu::Adapter, device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let mut console_panel = ConsolePanel::new();
        // Add a welcome message to the console
        console_panel.info("WebGPU Playground console initialized");
        console_panel.info("GPU errors, warnings, and validation messages will appear here");

        Self {
            device_info: DeviceInfo::new(adapter, device),
            device_config: DeviceConfigPanel::new(adapter),
            adapter_selection: AdapterSelectionPanel::new(adapter),
            rendering_panel: RenderingPanel::new(device, queue),
            compute_panel: ComputePanel::new(),
            compute_pipeline_panel: ComputePipelinePanel::new(),
            compute_dispatch_panel: ComputeDispatchPanel::new(),
            buffer_panel: BufferPanel::new(),
            sampler_panel: SamplerPanel::new(),
            texture_panel: TexturePanel::new(),
            bind_group_panel: BindGroupPanel::new(),
            bind_group_layout_panel: BindGroupLayoutPanel::new(),
            render_pipeline_panel: RenderPipelinePanel::new(),
            console_panel,
            draw_command_panel: DrawCommandPanel::new(),
            render_pass_panel: RenderPassPanel::new(),
            resource_inspector_panel: ResourceInspectorPanel::new(),
            performance_panel: PerformancePanel::new(),
            selected_tab: Tab::AdapterSelection,
        }
    }

    pub fn ui(
        &mut self,
        ctx: &egui::Context,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        renderer: &mut egui_wgpu::Renderer,
    ) {
        // Update performance metrics each frame
        self.performance_panel.update();

        // Menu bar at the top
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.heading("🎮 WebGPU Playground");
        });

        // Sidebar on the left
        egui::SidePanel::left("sidebar").show(ctx, |ui| {
            ui.heading("Navigation");
            ui.separator();

            ui.selectable_value(
                &mut self.selected_tab,
                Tab::AdapterSelection,
                "⚙️ Adapter Selection",
            );
            ui.selectable_value(
                &mut self.selected_tab,
                Tab::DeviceConfig,
                "🔧 Device Config",
            );
            ui.selectable_value(&mut self.selected_tab, Tab::DeviceInfo, "📊 Device Info");
            ui.selectable_value(&mut self.selected_tab, Tab::Rendering, "🎨 Rendering");
            ui.selectable_value(
                &mut self.selected_tab,
                Tab::BufferConfig,
                "📐 Buffer Config",
            );
            ui.selectable_value(
                &mut self.selected_tab,
                Tab::SamplerConfig,
                "🎨 Sampler Config",
            );
            ui.selectable_value(
                &mut self.selected_tab,
                Tab::TextureConfig,
                "🖼️ Texture Config",
            );
            ui.selectable_value(
                &mut self.selected_tab,
                Tab::BindGroupConfig,
                "🔗 Bind Group Config",
            );
            ui.selectable_value(
                &mut self.selected_tab,
                Tab::BindGroupLayoutConfig,
                "🔗 Bind Group Layout",
            );
            ui.selectable_value(
                &mut self.selected_tab,
                Tab::ComputePipelineConfig,
                "⚙️ Compute Pipeline",
            );
            ui.selectable_value(
                &mut self.selected_tab,
                Tab::RenderPipelineConfig,
                "⚡ Render Pipeline",
            );
            ui.selectable_value(&mut self.selected_tab, Tab::DrawCommand, "📊 Draw Command");
            ui.selectable_value(
                &mut self.selected_tab,
                Tab::RenderPassConfig,
                "🎬 Render Pass",
            );
            ui.selectable_value(
                &mut self.selected_tab,
                Tab::ComputeDispatch,
                "🚀 Compute Dispatch",
            );
            ui.selectable_value(&mut self.selected_tab, Tab::Compute, "🧮 Compute/ML");
            ui.selectable_value(&mut self.selected_tab, Tab::Console, "🖥️ Console");
            ui.selectable_value(
                &mut self.selected_tab,
                Tab::ResourceInspector,
                "🔍 Resource Inspector",
            );
            ui.selectable_value(
                &mut self.selected_tab,
                Tab::Performance,
                "📊 Performance",
            );
        });

        // Main canvas area
        egui::CentralPanel::default().show(ctx, |ui| match self.selected_tab {
            Tab::AdapterSelection => self.adapter_selection.ui(ui),
            Tab::DeviceConfig => self.device_config.ui(ui),
            Tab::DeviceInfo => self.device_info.ui(ui),
            Tab::Rendering => self.rendering_panel.ui(ui, device, queue, renderer),
            Tab::BufferConfig => self.buffer_panel.ui(ui),
            Tab::SamplerConfig => self.sampler_panel.ui(ui),
            Tab::TextureConfig => self.texture_panel.ui(ui),
            Tab::BindGroupConfig => self.bind_group_panel.ui(ui),
            Tab::BindGroupLayoutConfig => self.bind_group_layout_panel.ui(ui),
            Tab::ComputePipelineConfig => self.compute_pipeline_panel.ui(ui),
            Tab::RenderPipelineConfig => self.render_pipeline_panel.ui(ui),
            Tab::DrawCommand => self.draw_command_panel.ui(ui),
            Tab::RenderPassConfig => self.render_pass_panel.ui(ui),
            Tab::ComputeDispatch => self.compute_dispatch_panel.ui(ui),
            Tab::Compute => self.compute_panel.ui(ui),
            Tab::Console => self.console_panel.ui(ui),
            Tab::ResourceInspector => self.resource_inspector_panel.ui(ui),
            Tab::Performance => self.performance_panel.ui(ui),
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
    fn test_tab_copy_trait() {
        // Test that Tab implements Copy trait
        let tab = Tab::Rendering;
        let copied = tab;
        // Both can be used independently after copy
        assert_eq!(tab, Tab::Rendering);
        assert_eq!(copied, Tab::Rendering);
        // Demonstrate independent use
        let _ = (tab, copied);
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
            let _app = PlaygroundApp::new(&adapter, &device, &_queue);
            // If we get here without panicking, the test passes
        });
    }
}
