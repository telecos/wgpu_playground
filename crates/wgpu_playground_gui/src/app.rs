use wgpu_playground_core::adapter_selection::AdapterSelectionPanel;
use wgpu_playground_core::bind_group_layout_panel::BindGroupLayoutPanel;
use wgpu_playground_core::bind_group_panel::BindGroupPanel;
use wgpu_playground_core::buffer_inspector::BufferInspector;
use wgpu_playground_core::buffer_panel::BufferPanel;
use wgpu_playground_core::command_recording_panel::CommandRecordingPanel;
use wgpu_playground_core::compute::ComputePanel;
use wgpu_playground_core::compute_dispatch_panel::ComputeDispatchPanel;
use wgpu_playground_core::compute_pipeline_panel::ComputePipelinePanel;
use wgpu_playground_core::console::ConsolePanel;
use wgpu_playground_core::device_config::DeviceConfigPanel;
use wgpu_playground_core::device_info::DeviceInfo;
use wgpu_playground_core::draw_command_panel::DrawCommandPanel;
use wgpu_playground_core::model_loader_panel::ModelLoaderPanel;
use wgpu_playground_core::performance_panel::PerformancePanel;
use wgpu_playground_core::pipeline_debugger::PipelineDebugger;
use wgpu_playground_core::render_pass_panel::RenderPassPanel;
use wgpu_playground_core::render_pipeline_panel::RenderPipelinePanel;
use wgpu_playground_core::rendering::RenderingPanel;
use wgpu_playground_core::resource_inspector::ResourceInspectorPanel;
use wgpu_playground_core::sampler_panel::SamplerPanel;
use wgpu_playground_core::settings_panel::SettingsPanel;
use wgpu_playground_core::state::Theme;
use wgpu_playground_core::texture_inspector::TextureInspector;
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
    model_loader_panel: ModelLoaderPanel,
    console_panel: ConsolePanel,
    draw_command_panel: DrawCommandPanel,
    render_pass_panel: RenderPassPanel,
    resource_inspector_panel: ResourceInspectorPanel,
    buffer_inspector: BufferInspector,
    texture_inspector: TextureInspector,
    pipeline_debugger: PipelineDebugger,
    performance_panel: PerformancePanel,
    command_recording_panel: CommandRecordingPanel,
    settings_panel: SettingsPanel,
    selected_tab: Tab,
    // Collapsible section states
    setup_section_open: bool,
    rendering_section_open: bool,
    compute_section_open: bool,
    resources_section_open: bool,
    tools_section_open: bool,
    // State save/load UI fields
    save_load_filename: String,
    save_load_message: Option<String>,
    // URL sharing fields
    share_url: String,
    share_message: Option<String>,
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
    BufferInspector,
    TextureInspector,
    PipelineDebugger,
    Performance,
    CommandRecording,
    Settings,
    ModelLoader,
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
            model_loader_panel: ModelLoaderPanel::new(),
            console_panel,
            draw_command_panel: DrawCommandPanel::new(),
            render_pass_panel: RenderPassPanel::new(),
            resource_inspector_panel: ResourceInspectorPanel::new(),
            buffer_inspector: BufferInspector::new(),
            texture_inspector: TextureInspector::new(),
            pipeline_debugger: PipelineDebugger::new(),
            performance_panel: PerformancePanel::new(),
            command_recording_panel: CommandRecordingPanel::new(),
            settings_panel: SettingsPanel::new(),
            selected_tab: Tab::Rendering, // Start with Rendering tab to show visual example
            // Initialize section states - Rendering open by default
            setup_section_open: false,
            rendering_section_open: true,
            compute_section_open: false,
            resources_section_open: false,
            tools_section_open: false,
            save_load_filename: "playground_state.json".to_string(),
            save_load_message: None,
            share_url: String::new(),
            share_message: None,
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
            ui.horizontal(|ui| {
                ui.heading("🎮 WebGPU Playground");

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // File operations
                    ui.label("File:");

                    if ui.button("💾 Save State").clicked() {
                        let filename = self.save_load_filename.clone();
                        let path = std::path::Path::new(&filename);
                        match self.save_state_to_file(path) {
                            Ok(_) => {
                                self.save_load_message =
                                    Some(format!("✓ State saved to {}", filename));
                            }
                            Err(e) => {
                                self.save_load_message = Some(format!("✗ Failed to save: {}", e));
                            }
                        }
                    }

                    if ui.button("📂 Load State").clicked() {
                        let filename = self.save_load_filename.clone();
                        let path = std::path::Path::new(&filename);
                        match self.load_state_from_file(path) {
                            Ok(_) => {
                                self.save_load_message =
                                    Some(format!("✓ State loaded from {}", filename));
                            }
                            Err(e) => {
                                self.save_load_message = Some(format!("✗ Failed to load: {}", e));
                            }
                        }
                    }

                    ui.add(
                        egui::TextEdit::singleline(&mut self.save_load_filename)
                            .desired_width(200.0)
                            .hint_text("filename.json"),
                    );
                });
            });

            // Show save/load message if any
            if let Some(msg) = &self.save_load_message {
                ui.colored_label(
                    if msg.starts_with("✓") {
                        egui::Color32::GREEN
                    } else {
                        egui::Color32::RED
                    },
                    msg,
                );
            }

            // Second row: Share functionality
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Share:");

                if ui.button("🔗 Generate Share Link").clicked() {
                    // Get current base URL (for native, use localhost; for web, use window.location)
                    let base_url = if cfg!(target_arch = "wasm32") {
                        "https://telecos.github.io/wgpu_playground/demo"
                    } else {
                        "http://localhost:8080"
                    };

                    match self.export_state().to_shareable_url(base_url) {
                        Ok(url) => {
                            self.share_url = url.clone();
                            self.share_message = Some("✓ Share link generated!".to_string());
                            // Copy to clipboard automatically
                            ctx.copy_text(url);
                        }
                        Err(e) => {
                            self.share_message = Some(format!("✗ Failed to generate link: {}", e));
                        }
                    }
                }

                if !self.share_url.is_empty() && ui.button("📋 Copy to Clipboard").clicked() {
                    ctx.copy_text(self.share_url.clone());
                    self.share_message = Some("✓ Copied to clipboard!".to_string());
                }
            });

            // Show share message if any
            if let Some(msg) = &self.share_message {
                ui.colored_label(
                    if msg.starts_with("✓") {
                        egui::Color32::GREEN
                    } else {
                        egui::Color32::RED
                    },
                    msg,
                );
            }

            // Show generated URL in a scrollable text area
            if !self.share_url.is_empty() {
                ui.horizontal(|ui| {
                    ui.label("Share URL:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.share_url)
                            .desired_width(ui.available_width())
                            .interactive(false),
                    );
                });
            }
        });

        // Sidebar on the left
        egui::SidePanel::left("sidebar").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Navigation");
                ui.separator();
                ui.add_space(5.0);

                // Setup Section
                ui.push_id("setup_section", |ui| {
                    let header_response =
                        ui.selectable_label(self.setup_section_open, "⚙️ Setup & Configuration");
                    if header_response.clicked() {
                        self.setup_section_open = !self.setup_section_open;
                    }
                });

                if self.setup_section_open {
                    ui.indent("setup_indent", |ui| {
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::AdapterSelection,
                            "  Adapter Selection",
                        );
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::DeviceConfig,
                            "  Device Config",
                        );
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::DeviceInfo,
                            "  Device Info",
                        );
                    });
                }
                ui.add_space(3.0);

                // Rendering Section
                ui.push_id("rendering_section", |ui| {
                    let header_response =
                        ui.selectable_label(self.rendering_section_open, "🎨 Rendering & Graphics");
                    if header_response.clicked() {
                        self.rendering_section_open = !self.rendering_section_open;
                    }
                });

                if self.rendering_section_open {
                    ui.indent("rendering_indent", |ui| {
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::Rendering,
                            "  Examples & Preview",
                        );
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::RenderPipelineConfig,
                            "  Render Pipeline",
                        );
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::RenderPassConfig,
                            "  Render Pass",
                        );
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::DrawCommand,
                            "  Draw Commands",
                        );
                    });
                }
                ui.add_space(3.0);

                // Compute Section
                ui.push_id("compute_section", |ui| {
                    let header_response =
                        ui.selectable_label(self.compute_section_open, "🧮 Compute & ML");
                    if header_response.clicked() {
                        self.compute_section_open = !self.compute_section_open;
                    }
                });

                if self.compute_section_open {
                    ui.indent("compute_indent", |ui| {
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::Compute,
                            "  Compute Panel",
                        );
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::ComputePipelineConfig,
                            "  Compute Pipeline",
                        );
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::ComputeDispatch,
                            "  Compute Dispatch",
                        );
                    });
                }
                ui.add_space(3.0);

                // Resources Section
                ui.push_id("resources_section", |ui| {
                    let header_response =
                        ui.selectable_label(self.resources_section_open, "📦 Resources");
                    if header_response.clicked() {
                        self.resources_section_open = !self.resources_section_open;
                    }
                });

                if self.resources_section_open {
                    ui.indent("resources_indent", |ui| {
                        ui.selectable_value(&mut self.selected_tab, Tab::BufferConfig, "  Buffers");
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::TextureConfig,
                            "  Textures",
                        );
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::SamplerConfig,
                            "  Samplers",
                        );
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::ModelLoader,
                            "  3D Models",
                        );
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::BindGroupConfig,
                            "  Bind Groups",
                        );
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::BindGroupLayoutConfig,
                            "  Bind Group Layouts",
                        );
                    });
                }
                ui.add_space(3.0);

                // Tools Section
                ui.push_id("tools_section", |ui| {
                    let header_response =
                        ui.selectable_label(self.tools_section_open, "🔧 Tools & Debugging");
                    if header_response.clicked() {
                        self.tools_section_open = !self.tools_section_open;
                    }
                });

                if self.tools_section_open {
                    ui.indent("tools_indent", |ui| {
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::ResourceInspector,
                            "  Resource Inspector",
                        );
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::BufferInspector,
                            "  Buffer Inspector",
                        );
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::TextureInspector,
                            "  Texture Inspector",
                        );
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::PipelineDebugger,
                            "  Pipeline Debugger",
                        );
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::CommandRecording,
                            "  Command Recording",
                        );
                        ui.selectable_value(&mut self.selected_tab, Tab::Console, "  Console");
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::Performance,
                            "  Performance",
                        );
                        ui.selectable_value(&mut self.selected_tab, Tab::Settings, "  Settings");
                    });
                }
            });
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
            Tab::ModelLoader => self.model_loader_panel.show(ui, device),
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
            Tab::BufferInspector => self.buffer_inspector.ui(ui),
            Tab::TextureInspector => self.texture_inspector.ui(ui),
            Tab::PipelineDebugger => self.pipeline_debugger.ui(ui),
            Tab::Performance => self.performance_panel.ui(ui),
            Tab::CommandRecording => self.command_recording_panel.ui(ui),
            Tab::Settings => {
                if let Some(new_theme) = self.settings_panel.ui(ui) {
                    // Apply the theme change
                    Self::apply_theme(ctx, new_theme);
                    // Save the state with the new theme
                    let filename = self.save_load_filename.clone();
                    let path = std::path::Path::new(&filename);
                    if let Err(e) = self.save_state_to_file(path) {
                        log::warn!("Failed to save theme preference: {}", e);
                    }
                }
            }
        });
    }

    /// Apply a theme to the egui context
    pub fn apply_theme(ctx: &egui::Context, theme: Theme) {
        match theme {
            Theme::Light => {
                ctx.set_visuals(egui::Visuals::light());
            }
            Theme::Dark => {
                ctx.set_visuals(egui::Visuals::dark());
            }
        }
    }

    /// Get the current theme
    #[allow(dead_code)]
    pub fn get_current_theme(&self) -> Theme {
        self.settings_panel.get_theme()
    }

    /// Export the current playground state
    pub fn export_state(&self) -> wgpu_playground_core::state::PlaygroundState {
        wgpu_playground_core::state::PlaygroundState {
            version: "1.0".to_string(),
            theme: self.settings_panel.get_theme(),
            buffer_panel: Some(self.buffer_panel.export_state()),
            texture_panel: Some(self.texture_panel.export_state()),
            sampler_panel: Some(self.sampler_panel.export_state()),
            shader_editor: Some(self.rendering_panel.export_shader_editor_state()),
            render_pipeline_panel: None, // TODO: Add when RenderPipelinePanel has export_state
            compute_pipeline_panel: None, // TODO: Add when ComputePipelinePanel has export_state
            bind_group_panel: None,      // TODO: Add when BindGroupPanel has export_state
            bind_group_layout_panel: None, // TODO: Add when BindGroupLayoutPanel has export_state
        }
    }

    /// Import state into the playground
    pub fn import_state(&mut self, state: &wgpu_playground_core::state::PlaygroundState) {
        // Import theme preference
        self.settings_panel.set_theme(state.theme);

        if let Some(buffer_state) = &state.buffer_panel {
            self.buffer_panel.import_state(buffer_state);
        }
        if let Some(texture_state) = &state.texture_panel {
            self.texture_panel.import_state(texture_state);
        }
        if let Some(sampler_state) = &state.sampler_panel {
            self.sampler_panel.import_state(sampler_state);
        }
        if let Some(shader_state) = &state.shader_editor {
            self.rendering_panel
                .import_shader_editor_state(shader_state);
        }
        // TODO: Import other panel states when available
    }

    /// Save the current state to a file
    pub fn save_state_to_file(&self, path: &std::path::Path) -> Result<(), std::io::Error> {
        let state = self.export_state();
        state.save_to_file(path)?;
        log::info!("Playground state saved to {:?}", path);
        Ok(())
    }

    /// Load state from a file
    pub fn load_state_from_file(&mut self, path: &std::path::Path) -> Result<(), std::io::Error> {
        let state = wgpu_playground_core::state::PlaygroundState::load_from_file(path)?;
        self.import_state(&state);
        log::info!("Playground state loaded from {:?}", path);
        Ok(())
    }

    /// Load state from a URL parameter string
    ///
    /// This method parses a URL query string and loads the state if a 'state' parameter is found.
    #[allow(dead_code)]
    pub fn load_state_from_url(&mut self, url: &str) -> Result<(), String> {
        let state = wgpu_playground_core::state::PlaygroundState::from_url(url)?;
        self.import_state(&state);
        log::info!("Playground state loaded from URL");
        Ok(())
    }

    /// Try to load state from URL on startup (for web/WASM builds)
    ///
    /// This checks the browser's URL for a 'state' parameter and loads it if present.
    #[cfg(target_arch = "wasm32")]
    pub fn try_load_from_browser_url(&mut self) {
        if let Some(window) = web_sys::window() {
            if let Ok(location) = window.location().href() {
                if location.contains("?state=") {
                    match self.load_state_from_url(&location) {
                        Ok(_) => {
                            log::info!("Successfully loaded shared state from URL");
                            self.share_message =
                                Some("✓ Loaded shared configuration from URL".to_string());
                        }
                        Err(e) => {
                            log::error!("Failed to load state from URL: {}", e);
                            self.share_message =
                                Some(format!("✗ Failed to load shared state: {}", e));
                        }
                    }
                }
            }
        }
    }

    /// Placeholder for native builds (URL loading not supported)
    #[cfg(not(target_arch = "wasm32"))]
    pub fn try_load_from_browser_url(&mut self) {
        // No-op for native builds - URL state loading only works in browser
    }

    /// Handle dropped image file
    pub fn handle_dropped_image(&mut self, bytes: Vec<u8>) {
        self.texture_panel.load_from_bytes(bytes);
        // Switch to texture tab to show the loaded texture
        self.selected_tab = Tab::TextureConfig;
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
            let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
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
            let Ok(adapter) = adapter else {
                eprintln!("Skipping test: No GPU adapter available");
                return;
            };

            let device_result = adapter
                .request_device(&wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: Some("Test Device"),
                    memory_hints: Default::default(),
                    experimental_features: Default::default(),
                    trace: Default::default(),
                })
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
