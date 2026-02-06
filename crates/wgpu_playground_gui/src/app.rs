use wgpu_playground_core::adapter_selection::AdapterSelectionPanel;
use wgpu_playground_core::api_coverage::ApiCoverageTracker;
use wgpu_playground_core::api_coverage_panel::ApiCoveragePanel;
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
    api_coverage_panel: ApiCoveragePanel,
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
    ApiCoverage,
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
            api_coverage_panel: ApiCoveragePanel::new(),
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

        // Keyboard shortcuts for accessibility
        ctx.input(|i| {
            // Ctrl+S or Cmd+S: Save state
            if i.modifiers.command && i.key_pressed(egui::Key::S) {
                let filename = self.save_load_filename.clone();
                let path = std::path::Path::new(&filename);
                if let Err(e) = self.save_state_to_file(path) {
                    log::error!("Failed to save state: {}", e);
                } else {
                    log::info!("State saved to {}", filename);
                }
            }

            // Ctrl+O or Cmd+O: Load state
            if i.modifiers.command && i.key_pressed(egui::Key::O) {
                let filename = self.save_load_filename.clone();
                let path = std::path::Path::new(&filename);
                if let Err(e) = self.load_state_from_file(path) {
                    log::error!("Failed to load state: {}", e);
                } else {
                    log::info!("State loaded from {}", filename);
                }
            }

            // Ctrl+1-9: Quick navigation to tabs
            if i.modifiers.command {
                if i.key_pressed(egui::Key::Num1) {
                    self.selected_tab = Tab::Rendering;
                } else if i.key_pressed(egui::Key::Num2) {
                    self.selected_tab = Tab::Compute;
                } else if i.key_pressed(egui::Key::Num3) {
                    self.selected_tab = Tab::BufferConfig;
                } else if i.key_pressed(egui::Key::Num4) {
                    self.selected_tab = Tab::TextureConfig;
                } else if i.key_pressed(egui::Key::Num5) {
                    self.selected_tab = Tab::Console;
                } else if i.key_pressed(egui::Key::Num6) {
                    self.selected_tab = Tab::Settings;
                }
            }
        });

        // Menu bar at the top
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🎮 WebGPU Playground");

                // Add backend indicator
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                let current_backend =
                    wgpu_playground_core::implementation::WebGPUImplementation::current();
                let (backend_label, backend_color) = match current_backend {
                    wgpu_playground_core::implementation::WebGPUImplementation::Wgpu => {
                        ("🦀 wgpu-rs", egui::Color32::from_rgb(100, 150, 255))
                    }
                    #[cfg(feature = "dawn")]
                    wgpu_playground_core::implementation::WebGPUImplementation::Dawn => {
                        ("🌅 Dawn Native", egui::Color32::from_rgb(255, 180, 100))
                    }
                };

                ui.label("Backend:");
                ui.colored_label(backend_color, backend_label)
                    .on_hover_text(current_backend.description());

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // File operations
                    ui.label("File:");

                    if ui
                        .button("💾 Save State")
                        .on_hover_text("Save current playground state to a file (Ctrl+S)")
                        .clicked()
                    {
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

                    if ui
                        .button("📂 Load State")
                        .on_hover_text("Load playground state from a file (Ctrl+O)")
                        .clicked()
                    {
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
                    )
                    .on_hover_text("Enter filename for save/load operations");
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

                if ui
                    .button("🔗 Generate Share Link")
                    .on_hover_text(
                        "Generate a shareable URL with your current playground configuration",
                    )
                    .clicked()
                {
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

                if !self.share_url.is_empty()
                    && ui
                        .button("📋 Copy to Clipboard")
                        .on_hover_text("Copy the generated share link to clipboard")
                        .clicked()
                {
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

                // Platform-appropriate shortcut hint
                let shortcut_modifier = if cfg!(target_os = "macos") {
                    "Cmd"
                } else {
                    "Ctrl"
                };

                ui.label(format!("Quick navigation: {shortcut_modifier}+1-6"))
                    .on_hover_text(format!(
                        "Keyboard shortcuts:\n{0}+1: Rendering\n{0}+2: Compute\n{0}+3: Buffers\n{0}+4: Textures\n{0}+5: Console\n{0}+6: Settings",
                        shortcut_modifier
                    ));
                ui.separator();
                ui.add_space(5.0);

                // Setup Section
                ui.push_id("setup_section", |ui| {
                    let header_response =
                        ui.selectable_label(self.setup_section_open, "⚙️ Setup & Configuration")
                            .on_hover_text("Configure GPU adapter and device settings");
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
                        ).on_hover_text("Choose and configure GPU adapters");
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::DeviceConfig,
                            "  Device Config",
                        ).on_hover_text("Enable/disable WebGPU features and adjust device limits");
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::DeviceInfo,
                            "  Device Info",
                        ).on_hover_text("View comprehensive GPU adapter information");
                    });
                }
                ui.add_space(3.0);

                // Rendering Section
                ui.push_id("rendering_section", |ui| {
                    let header_response =
                        ui.selectable_label(self.rendering_section_open, "🎨 Rendering & Graphics")
                            .on_hover_text("Graphics rendering examples and pipeline configuration (Ctrl+1)");
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
                        ).on_hover_text("View rendering examples and live preview");
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::RenderPipelineConfig,
                            "  Render Pipeline",
                        ).on_hover_text("Configure render pipeline and shaders");
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::RenderPassConfig,
                            "  Render Pass",
                        ).on_hover_text("Configure render pass settings");
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::DrawCommand,
                            "  Draw Commands",
                        ).on_hover_text("Configure and execute draw commands");
                    });
                }
                ui.add_space(3.0);

                // Compute Section
                ui.push_id("compute_section", |ui| {
                    let header_response =
                        ui.selectable_label(self.compute_section_open, "🧮 Compute & ML")
                            .on_hover_text("Compute shader operations and ML inferencing (Ctrl+2)");
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
                        ).on_hover_text("Explore compute shader operations");
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::ComputePipelineConfig,
                            "  Compute Pipeline",
                        ).on_hover_text("Configure compute pipeline and shaders");
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::ComputeDispatch,
                            "  Compute Dispatch",
                        ).on_hover_text("Configure and dispatch compute operations");
                    });
                }
                ui.add_space(3.0);

                // Resources Section
                ui.push_id("resources_section", |ui| {
                    let header_response =
                        ui.selectable_label(self.resources_section_open, "📦 Resources")
                            .on_hover_text("Manage GPU resources (buffers, textures, samplers)");
                    if header_response.clicked() {
                        self.resources_section_open = !self.resources_section_open;
                    }
                });

                if self.resources_section_open {
                    ui.indent("resources_indent", |ui| {
                        ui.selectable_value(&mut self.selected_tab, Tab::BufferConfig, "  Buffers")
                            .on_hover_text("Create and configure GPU buffers (Ctrl+3)");
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::TextureConfig,
                            "  Textures",
                        ).on_hover_text("Load and configure textures from images (Ctrl+4)");
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::SamplerConfig,
                            "  Samplers",
                        ).on_hover_text("Configure texture sampling modes");
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::ModelLoader,
                            "  3D Models",
                        ).on_hover_text("Load and view 3D models");
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::BindGroupConfig,
                            "  Bind Groups",
                        ).on_hover_text("Create bind groups for shader resources");
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::BindGroupLayoutConfig,
                            "  Bind Group Layouts",
                        ).on_hover_text("Define bind group layouts");
                    });
                }
                ui.add_space(3.0);

                // Tools Section
                ui.push_id("tools_section", |ui| {
                    let header_response =
                        ui.selectable_label(self.tools_section_open, "🔧 Tools & Debugging")
                            .on_hover_text("Debugging tools, console, and performance monitoring");
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
                        ).on_hover_text("Inspect all GPU resources");
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::BufferInspector,
                            "  Buffer Inspector",
                        ).on_hover_text("View and inspect buffer contents");
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::TextureInspector,
                            "  Texture Inspector",
                        ).on_hover_text("View and inspect texture data");
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::PipelineDebugger,
                            "  Pipeline Debugger",
                        ).on_hover_text("Debug render and compute pipelines");
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::CommandRecording,
                            "  Command Recording",
                        ).on_hover_text("Record and replay GPU commands");
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::ApiCoverage,
                            "  API Coverage",
                        ).on_hover_text("Track and visualize WebGPU API usage");
                        ui.selectable_value(&mut self.selected_tab, Tab::Console, "  Console")
                            .on_hover_text("View GPU errors and validation messages (Ctrl+5)");
                        ui.selectable_value(
                            &mut self.selected_tab,
                            Tab::Performance,
                            "  Performance",
                        ).on_hover_text("Monitor GPU performance metrics");
                        ui.selectable_value(&mut self.selected_tab, Tab::Settings, "  Settings")
                            .on_hover_text("Application settings and preferences (Ctrl+6)");
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
            Tab::BufferConfig => {
                self.buffer_panel
                    .ui_with_preview(ui, Some(device), Some(queue), Some(renderer))
            }
            Tab::SamplerConfig => self.sampler_panel.ui(ui),
            Tab::TextureConfig => {
                self.texture_panel
                    .ui_with_preview(ui, Some(device), Some(queue), Some(renderer))
            }
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
            Tab::ApiCoverage => {
                let tracker = ApiCoverageTracker::global();
                self.api_coverage_panel.ui(ui, tracker);
            }
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
            api_coverage: None,          // API coverage is tracked globally, not exported per-state
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
