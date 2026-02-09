//! UI panel for displaying and managing API coverage tracking

use crate::api_coverage::{ApiCategory, ApiCoverageTracker, CoverageData};
use egui::{CollapsingHeader, Color32, RichText, ScrollArea, Ui};
use std::collections::HashMap;

/// Navigation request from the API Coverage panel
/// Used to navigate to appropriate panels when user clicks "Try" buttons
#[derive(Debug, Clone, PartialEq)]
pub enum NavigationRequest {
    /// Navigate to buffer configuration panel
    BufferConfig,
    /// Navigate to texture configuration panel
    TextureConfig,
    /// Navigate to sampler configuration panel
    SamplerConfig,
    /// Navigate to shader/render pipeline panel
    RenderPipelineConfig,
    /// Navigate to compute pipeline panel
    ComputePipelineConfig,
    /// Navigate to bind group panel
    BindGroupConfig,
    /// Navigate to rendering examples panel
    RenderingExamples,
    /// Navigate to compute panel
    ComputePanel,
    /// Navigate to draw commands panel
    DrawCommandPanel,
    /// Navigate to render pass panel
    RenderPassConfig,
    /// Navigate to compute dispatch panel
    ComputeDispatchConfig,
}

impl NavigationRequest {
    /// Get a human-readable description of the navigation target
    pub fn description(&self) -> &'static str {
        match self {
            NavigationRequest::BufferConfig => {
                "Buffer Configuration panel - create and configure GPU buffers"
            }
            NavigationRequest::TextureConfig => {
                "Texture Configuration panel - load and configure textures"
            }
            NavigationRequest::SamplerConfig => {
                "Sampler Configuration panel - configure texture sampling"
            }
            NavigationRequest::RenderPipelineConfig => {
                "Render Pipeline panel - configure graphics pipelines and shaders"
            }
            NavigationRequest::ComputePipelineConfig => {
                "Compute Pipeline panel - configure compute pipelines"
            }
            NavigationRequest::BindGroupConfig => "Bind Group panel - create resource bind groups",
            NavigationRequest::RenderingExamples => {
                "Rendering Examples - see GPU rendering in action"
            }
            NavigationRequest::ComputePanel => "Compute panel - GPU compute operations",
            NavigationRequest::DrawCommandPanel => "Draw Commands panel - configure draw calls",
            NavigationRequest::RenderPassConfig => "Render Pass panel - configure render passes",
            NavigationRequest::ComputeDispatchConfig => {
                "Compute Dispatch panel - dispatch compute workgroups"
            }
        }
    }
}

/// Map an API category and method to a navigation target
fn get_navigation_for_api(category: ApiCategory, method: &str) -> Option<NavigationRequest> {
    match category {
        ApiCategory::Buffer => Some(NavigationRequest::BufferConfig),
        ApiCategory::Texture => Some(NavigationRequest::TextureConfig),
        ApiCategory::Sampler => Some(NavigationRequest::SamplerConfig),
        ApiCategory::Shader => Some(NavigationRequest::RenderPipelineConfig),
        ApiCategory::RenderPipeline => Some(NavigationRequest::RenderPipelineConfig),
        ApiCategory::ComputePipeline => Some(NavigationRequest::ComputePipelineConfig),
        ApiCategory::BindGroup | ApiCategory::PipelineLayout => {
            Some(NavigationRequest::BindGroupConfig)
        }
        ApiCategory::RenderPass => {
            // Specific render pass methods map to different panels
            match method {
                "draw" | "draw_indexed" | "draw_indirect" | "draw_indexed_indirect" => {
                    Some(NavigationRequest::DrawCommandPanel)
                }
                "begin_render_pass" | "end_pass" => Some(NavigationRequest::RenderPassConfig),
                _ => Some(NavigationRequest::RenderingExamples),
            }
        }
        ApiCategory::ComputePass => match method {
            "dispatch_workgroups" | "dispatch_workgroups_indirect" => {
                Some(NavigationRequest::ComputeDispatchConfig)
            }
            _ => Some(NavigationRequest::ComputePanel),
        },
        ApiCategory::CommandEncoder => Some(NavigationRequest::RenderingExamples),
        ApiCategory::Device | ApiCategory::Queue => Some(NavigationRequest::RenderingExamples),
        ApiCategory::RenderBundle | ApiCategory::QuerySet => None, // No direct panel for these yet
    }
}

/// Expected WebGPU APIs organized by category
/// Only includes APIs that can be exercised through the playground UI
fn get_expected_apis() -> HashMap<ApiCategory, Vec<&'static str>> {
    let mut apis = HashMap::new();

    // Note: We don't track Device category separately since those methods
    // are tracked under their respective resource categories

    apis.insert(
        ApiCategory::Queue,
        vec!["submit", "write_buffer", "write_texture"],
    );

    apis.insert(ApiCategory::Buffer, vec!["create_buffer"]);

    apis.insert(ApiCategory::Texture, vec!["create_texture", "create_view"]);

    apis.insert(ApiCategory::Sampler, vec!["create_sampler"]);

    apis.insert(ApiCategory::Shader, vec!["create_shader_module"]);

    apis.insert(ApiCategory::RenderPipeline, vec!["create_render_pipeline"]);

    apis.insert(
        ApiCategory::ComputePipeline,
        vec!["create_compute_pipeline"],
    );

    apis.insert(
        ApiCategory::BindGroup,
        vec!["create_bind_group", "create_bind_group_layout"],
    );

    apis.insert(ApiCategory::PipelineLayout, vec!["create_pipeline_layout"]);

    apis.insert(
        ApiCategory::RenderPass,
        vec![
            "begin_render_pass",
            "set_pipeline",
            "set_bind_group",
            "set_vertex_buffer",
            "set_index_buffer",
            "draw",
            "draw_indexed",
        ],
    );

    apis.insert(
        ApiCategory::ComputePass,
        vec![
            "begin_compute_pass",
            "set_pipeline",
            "set_bind_group",
            "dispatch_workgroups",
        ],
    );

    apis.insert(
        ApiCategory::CommandEncoder,
        vec![
            "create_command_encoder",
            "begin_render_pass",
            "begin_compute_pass",
        ],
    );

    // Advanced features not yet implemented in playground:
    // - RenderBundle (for optimized rendering)
    // - QuerySet (for GPU timing/occlusion queries)
    // - Buffer mapping (map_read, map_write, unmap)
    // - Resource destruction (destroy)
    // - Indirect drawing (draw_indirect, draw_indexed_indirect)
    // - Advanced render pass features (set_viewport, set_scissor_rect, etc.)
    // - Copy operations (copy_buffer_to_buffer, etc.)

    apis
}

/// Get documentation URL for a WebGPU API category
fn get_documentation_url(category: ApiCategory) -> &'static str {
    match category {
        ApiCategory::Device => "https://www.w3.org/TR/webgpu/#gpu-device",
        ApiCategory::Queue => "https://www.w3.org/TR/webgpu/#gpu-queue",
        ApiCategory::Buffer => "https://www.w3.org/TR/webgpu/#gpu-buffer",
        ApiCategory::Texture => "https://www.w3.org/TR/webgpu/#gpu-texture",
        ApiCategory::Sampler => "https://www.w3.org/TR/webgpu/#gpu-sampler",
        ApiCategory::Shader => "https://www.w3.org/TR/webgpu/#gpu-shadermodule",
        ApiCategory::RenderPipeline => "https://www.w3.org/TR/webgpu/#gpu-renderpipeline",
        ApiCategory::ComputePipeline => "https://www.w3.org/TR/webgpu/#gpu-computepipeline",
        ApiCategory::BindGroup => "https://www.w3.org/TR/webgpu/#gpu-bindgroup",
        ApiCategory::PipelineLayout => "https://www.w3.org/TR/webgpu/#gpu-pipelinelayout",
        ApiCategory::RenderPass => "https://www.w3.org/TR/webgpu/#render-passes",
        ApiCategory::ComputePass => "https://www.w3.org/TR/webgpu/#compute-passes",
        ApiCategory::CommandEncoder => "https://www.w3.org/TR/webgpu/#gpu-commandencoder",
        ApiCategory::RenderBundle => "https://www.w3.org/TR/webgpu/#gpu-renderbundle",
        ApiCategory::QuerySet => "https://www.w3.org/TR/webgpu/#gpu-queryset",
    }
}

/// Panel for displaying API coverage information
pub struct ApiCoveragePanel {
    /// Whether the panel is open
    is_open: bool,
    /// Filter text for searching API calls
    filter_text: String,
    /// Selected category for filtering (None = show all)
    selected_category: Option<ApiCategory>,
    /// Show percentage view
    show_percentage: bool,
    /// Expanded categories (to show detailed API list)
    expanded_categories: HashMap<ApiCategory, bool>,
}

impl Default for ApiCoveragePanel {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiCoveragePanel {
    /// Create a new API coverage panel
    pub fn new() -> Self {
        Self {
            is_open: false,
            filter_text: String::new(),
            selected_category: None,
            show_percentage: true,
            expanded_categories: HashMap::new(),
        }
    }

    /// Toggle panel visibility
    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    /// Set panel visibility
    pub fn set_open(&mut self, open: bool) {
        self.is_open = open;
    }

    /// Check if panel is open
    pub fn is_open(&self) -> bool {
        self.is_open
    }

    /// Show the panel UI
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        tracker: &ApiCoverageTracker,
    ) -> Option<NavigationRequest> {
        let mut is_open = self.is_open;
        let mut nav_request = None;
        egui::Window::new("📊 API Coverage Tracker")
            .open(&mut is_open)
            .default_width(500.0)
            .default_height(600.0)
            .show(ctx, |ui| {
                nav_request = self.ui(ui, tracker);
            });
        self.is_open = is_open;
        nav_request
    }

    /// Render the panel contents
    /// Returns a NavigationRequest if the user clicks a "Try" button
    pub fn ui(&mut self, ui: &mut Ui, tracker: &ApiCoverageTracker) -> Option<NavigationRequest> {
        let snapshot = tracker.snapshot();

        // Header with overall statistics
        self.render_header(ui, &snapshot, tracker);

        ui.separator();

        // Controls
        self.render_controls(ui, tracker);

        ui.separator();

        // Category breakdown - may return navigation request
        let nav_request = self.render_category_breakdown(ui, &snapshot);

        ui.separator();

        // Detailed API call list
        self.render_api_call_list(ui, &snapshot);

        nav_request
    }

    fn render_header(
        &mut self,
        ui: &mut Ui,
        snapshot: &CoverageData,
        tracker: &ApiCoverageTracker,
    ) {
        ui.vertical_centered(|ui| {
            ui.heading("API Coverage Statistics");
            ui.add_space(4.0);

            let call_count = snapshot.call_count();
            let coverage_pct = snapshot.coverage_percentage();

            ui.label(
                RichText::new(format!("{} API calls tracked", call_count))
                    .size(18.0)
                    .strong(),
            );

            if self.show_percentage {
                let color = if coverage_pct < 30.0 {
                    Color32::from_rgb(255, 100, 100) // Red for low coverage
                } else if coverage_pct < 70.0 {
                    Color32::from_rgb(255, 200, 100) // Yellow for medium coverage
                } else {
                    Color32::from_rgb(100, 255, 100) // Green for high coverage
                };

                ui.colored_label(
                    color,
                    RichText::new(format!("~{:.1}% coverage", coverage_pct)).size(16.0),
                );
            }

            // Session info
            if let Some(session_name) = &snapshot.session_name {
                ui.label(format!("Session: {}", session_name));
            }
            if let Some(start_time) = &snapshot.start_time {
                ui.label(format!("Started: {}", start_time));
            }

            // Tracking status
            let status_text = if tracker.is_enabled() {
                RichText::new("● Tracking Active").color(Color32::from_rgb(100, 255, 100))
            } else {
                RichText::new("○ Tracking Paused").color(Color32::from_rgb(200, 200, 200))
            };
            ui.label(status_text);
        });
    }

    fn render_controls(&mut self, ui: &mut Ui, tracker: &ApiCoverageTracker) {
        ui.horizontal(|ui| {
            // Enable/Disable tracking
            if tracker.is_enabled() {
                if ui.button("⏸ Pause Tracking").clicked() {
                    tracker.disable();
                }
            } else if ui.button("▶ Resume Tracking").clicked() {
                tracker.enable();
            }

            // Reset button
            if ui.button("🔄 Reset").clicked() {
                tracker.reset();
            }

            // Export button
            if ui.button("💾 Export JSON").clicked() {
                if let Ok(json) = tracker.to_json() {
                    log::info!("Exported coverage data:\n{}", json);
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        // On native, could save to file
                        if let Err(e) = std::fs::write("api_coverage.json", json) {
                            log::error!("Failed to save coverage data: {}", e);
                        } else {
                            log::info!("Coverage data saved to api_coverage.json");
                        }
                    }
                }
            }

            // Toggle percentage view
            ui.checkbox(&mut self.show_percentage, "Show %");
        });

        // Filter controls
        ui.horizontal(|ui| {
            ui.label("🔍 Filter:");
            ui.text_edit_singleline(&mut self.filter_text);

            if ui.button("Clear").clicked() {
                self.filter_text.clear();
                self.selected_category = None;
            }
        });
    }

    fn render_category_breakdown(
        &mut self,
        ui: &mut Ui,
        snapshot: &CoverageData,
    ) -> Option<NavigationRequest> {
        let mut nav_request: Option<NavigationRequest> = None;

        CollapsingHeader::new("📂 Category Coverage")
            .default_open(true)
            .show(ui, |ui| {
                ScrollArea::vertical().max_height(400.0).show(ui, |ui| {
                    let expected_apis = get_expected_apis();

                    for category in ApiCategory::all() {
                        let expected = expected_apis.get(&category).map_or(0, |v| v.len());
                        let covered_count = snapshot.category_count(category);
                        let coverage_pct = if expected > 0 {
                            (covered_count as f32 / expected as f32) * 100.0
                        } else {
                            0.0
                        };

                        // Skip empty categories unless selected
                        if covered_count == 0
                            && expected == 0
                            && self.selected_category != Some(category)
                        {
                            continue;
                        }

                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                // Category name and selection
                                let is_selected = self.selected_category == Some(category);
                                let response = ui.selectable_label(
                                    is_selected,
                                    RichText::new(category.name()).strong(),
                                );

                                if response.clicked() {
                                    self.selected_category =
                                        if is_selected { None } else { Some(category) };
                                }

                                // Coverage stats
                                ui.label(format!("{}/{} APIs", covered_count, expected));

                                // Progress bar with color coding
                                let progress = if expected > 0 {
                                    covered_count as f32 / expected as f32
                                } else {
                                    0.0
                                };

                                let bar_color = if progress >= 0.8 {
                                    Color32::from_rgb(100, 255, 100) // Green
                                } else if progress >= 0.5 {
                                    Color32::from_rgb(255, 200, 100) // Yellow
                                } else if progress > 0.0 {
                                    Color32::from_rgb(255, 150, 100) // Orange
                                } else {
                                    Color32::from_rgb(200, 200, 200) // Gray
                                };

                                let mut progress_bar =
                                    egui::ProgressBar::new(progress).desired_width(150.0);

                                if self.show_percentage {
                                    progress_bar =
                                        progress_bar.text(format!("{:.0}%", coverage_pct));
                                }

                                ui.add(progress_bar.fill(bar_color));

                                // Documentation link
                                if ui
                                    .button("📖 Docs")
                                    .on_hover_text("Open WebGPU specification")
                                    .clicked()
                                {
                                    let url = get_documentation_url(category);
                                    #[cfg(not(target_arch = "wasm32"))]
                                    {
                                        if let Err(e) = webbrowser::open(url) {
                                            log::error!("Failed to open browser: {}", e);
                                        }
                                    }
                                    #[cfg(target_arch = "wasm32")]
                                    {
                                        match web_sys::window() {
                                            Some(w) => {
                                                if let Err(e) = w.open_with_url(url) {
                                                    log::error!(
                                                        "Failed to open documentation: {:?}",
                                                        e
                                                    );
                                                }
                                            }
                                            None => {
                                                log::error!("Failed to get window object");
                                            }
                                        }
                                    }
                                }

                                // Expand/collapse button
                                let is_expanded =
                                    *self.expanded_categories.get(&category).unwrap_or(&false);
                                if ui.button(if is_expanded { "▼" } else { "▶" }).clicked() {
                                    self.expanded_categories.insert(category, !is_expanded);
                                }
                            });

                            // Show detailed API list if expanded
                            if *self.expanded_categories.get(&category).unwrap_or(&false) {
                                ui.indent("api_list", |ui| {
                                    if let Some(expected_api_list) = expected_apis.get(&category) {
                                        let covered_apis: Vec<_> = snapshot
                                            .calls_by_category(category)
                                            .into_iter()
                                            .map(|call| call.method.as_str())
                                            .collect();

                                        for api_name in expected_api_list {
                                            let is_covered = covered_apis.contains(api_name);
                                            ui.horizontal(|ui| {
                                                if is_covered {
                                                    ui.label(
                                                        RichText::new("✓").color(
                                                            Color32::from_rgb(100, 255, 100),
                                                        ),
                                                    );
                                                } else {
                                                    ui.label(
                                                        RichText::new("○").color(
                                                            Color32::from_rgb(200, 200, 200),
                                                        ),
                                                    );
                                                }

                                                let text_color = if is_covered {
                                                    Color32::WHITE
                                                } else {
                                                    Color32::GRAY
                                                };

                                                ui.label(
                                                    RichText::new(*api_name).color(text_color),
                                                );

                                                // "Try this API" button for uncovered APIs
                                                if !is_covered {
                                                    if let Some(target) = get_navigation_for_api(category, api_name) {
                                                        if ui
                                                            .button("Try")
                                                            .on_hover_text(target.description())
                                                            .clicked()
                                                        {
                                                            log::info!(
                                                                "Navigating to try API: {} -> {} (target: {:?})",
                                                                category.name(),
                                                                api_name,
                                                                target
                                                            );
                                                            nav_request = Some(target);
                                                        }
                                                    }
                                                }
                                            });
                                        }
                                    }
                                });
                            }
                        });

                        ui.add_space(4.0);
                    }
                });
            });

        nav_request
    }

    fn render_api_call_list(&mut self, ui: &mut Ui, snapshot: &CoverageData) {
        CollapsingHeader::new("📋 API Calls")
            .default_open(true)
            .show(ui, |ui| {
                ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        let mut calls: Vec<_> = snapshot.calls.iter().collect();
                        calls.sort_by(|a, b| {
                            a.category
                                .name()
                                .cmp(b.category.name())
                                .then(a.method.cmp(&b.method))
                        });

                        let filter_lower = self.filter_text.to_lowercase();
                        let mut displayed_count = 0;

                        for call in calls {
                            // Apply category filter
                            if let Some(selected) = self.selected_category {
                                if call.category != selected {
                                    continue;
                                }
                            }

                            // Apply text filter
                            if !filter_lower.is_empty() {
                                let call_text = format!(
                                    "{} {}",
                                    call.category.name().to_lowercase(),
                                    call.method.to_lowercase()
                                );
                                if !call_text.contains(&filter_lower) {
                                    continue;
                                }
                            }

                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new(call.category.name())
                                        .color(Color32::from_rgb(150, 150, 255))
                                        .strong(),
                                );
                                ui.label("→");
                                ui.label(&call.method);
                            });

                            displayed_count += 1;
                        }

                        if displayed_count == 0 {
                            ui.label(
                                RichText::new("No API calls match the current filter")
                                    .color(Color32::GRAY)
                                    .italics(),
                            );
                        }
                    });
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panel_creation() {
        let panel = ApiCoveragePanel::new();
        assert!(!panel.is_open());
        assert_eq!(panel.filter_text, "");
        assert_eq!(panel.selected_category, None);
        assert!(panel.show_percentage);
        assert!(panel.expanded_categories.is_empty());
    }

    #[test]
    fn test_panel_toggle() {
        let mut panel = ApiCoveragePanel::new();
        assert!(!panel.is_open());

        panel.toggle();
        assert!(panel.is_open());

        panel.toggle();
        assert!(!panel.is_open());
    }

    #[test]
    fn test_panel_set_open() {
        let mut panel = ApiCoveragePanel::new();

        panel.set_open(true);
        assert!(panel.is_open());

        panel.set_open(false);
        assert!(!panel.is_open());
    }

    #[test]
    fn test_expected_apis() {
        let apis = get_expected_apis();

        // Verify key categories have some APIs defined
        assert!(apis.contains_key(&ApiCategory::Buffer));
        assert!(apis.contains_key(&ApiCategory::RenderPass));
        assert!(apis.contains_key(&ApiCategory::ComputePass));

        // Verify specific APIs are present
        let buffer_apis = apis.get(&ApiCategory::Buffer).unwrap();
        assert!(buffer_apis.contains(&"create_buffer"));

        let render_pass_apis = apis.get(&ApiCategory::RenderPass).unwrap();
        assert!(render_pass_apis.contains(&"begin_render_pass"));
    }

    #[test]
    fn test_documentation_urls() {
        // Verify all categories have documentation URLs
        for category in ApiCategory::all() {
            let url = get_documentation_url(category);
            assert!(url.starts_with("https://"));
            assert!(url.contains("webgpu"));
        }
    }
}
