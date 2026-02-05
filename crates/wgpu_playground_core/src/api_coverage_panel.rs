//! UI panel for displaying and managing API coverage tracking

use crate::api_coverage::{ApiCategory, ApiCoverageTracker, CoverageData};
use egui::{CollapsingHeader, Color32, RichText, ScrollArea, Ui};

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
    pub fn show(&mut self, ctx: &egui::Context, tracker: &ApiCoverageTracker) {
        let mut is_open = self.is_open;
        egui::Window::new("📊 API Coverage Tracker")
            .open(&mut is_open)
            .default_width(500.0)
            .default_height(600.0)
            .show(ctx, |ui| {
                self.ui(ui, tracker);
            });
        self.is_open = is_open;
    }

    /// Render the panel contents
    pub fn ui(&mut self, ui: &mut Ui, tracker: &ApiCoverageTracker) {
        let snapshot = tracker.snapshot();

        // Header with overall statistics
        self.render_header(ui, &snapshot, tracker);

        ui.separator();

        // Controls
        self.render_controls(ui, tracker);

        ui.separator();

        // Category breakdown
        self.render_category_breakdown(ui, &snapshot);

        ui.separator();

        // Detailed API call list
        self.render_api_call_list(ui, &snapshot);
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

    fn render_category_breakdown(&mut self, ui: &mut Ui, snapshot: &CoverageData) {
        CollapsingHeader::new("📂 Categories")
            .default_open(true)
            .show(ui, |ui| {
                ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                    for category in ApiCategory::all() {
                        let count = snapshot.category_count(category);
                        if count > 0 || self.selected_category == Some(category) {
                            ui.horizontal(|ui| {
                                let is_selected = self.selected_category == Some(category);
                                let response = ui.selectable_label(
                                    is_selected,
                                    format!("{}: {}", category.name(), count),
                                );

                                if response.clicked() {
                                    self.selected_category =
                                        if is_selected { None } else { Some(category) };
                                }

                                // Progress bar
                                let bar_width = (count as f32 / 10.0).min(1.0);
                                ui.add(
                                    egui::ProgressBar::new(bar_width)
                                        .desired_width(100.0)
                                        .show_percentage(),
                                );
                            });
                        }
                    }
                });
            });
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
}
