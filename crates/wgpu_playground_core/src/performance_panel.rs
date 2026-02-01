/// Performance metrics panel UI
use crate::performance_metrics::PerformanceMetrics;

/// Performance panel for displaying FPS, frame times, and profiling data
pub struct PerformancePanel {
    /// Performance metrics tracker
    metrics: PerformanceMetrics,
    /// Show frame time graph
    show_frame_time_graph: bool,
    /// Show FPS graph
    show_fps_graph: bool,
    /// Auto-reset counters each frame
    auto_reset_counters: bool,
    /// Graph height
    graph_height: f32,
}

impl Default for PerformancePanel {
    fn default() -> Self {
        Self::new()
    }
}

impl PerformancePanel {
    /// Create a new performance panel
    pub fn new() -> Self {
        Self {
            metrics: PerformanceMetrics::new(),
            show_frame_time_graph: true,
            show_fps_graph: true,
            auto_reset_counters: false,
            graph_height: 100.0,
        }
    }

    /// Get mutable reference to metrics for external updates
    pub fn metrics_mut(&mut self) -> &mut PerformanceMetrics {
        &mut self.metrics
    }

    /// Get reference to metrics for external reads
    pub fn metrics(&self) -> &PerformanceMetrics {
        &self.metrics
    }

    /// Update metrics (call this each frame)
    pub fn update(&mut self) {
        // End the previous frame measurement
        self.metrics.end_frame();

        // Reset counters if auto-reset is enabled
        if self.auto_reset_counters {
            self.metrics.set_command_buffer_count(0);
            self.metrics.set_draw_call_count(0);
            self.metrics.set_compute_dispatch_count(0);
        }

        // Start the next frame measurement
        self.metrics.start_frame();
    }

    /// Render the performance panel UI
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("📊 Performance Metrics");
        ui.separator();
        ui.label("Real-time performance monitoring and profiling data.");
        ui.add_space(10.0);

        // Control buttons
        ui.horizontal(|ui| {
            if ui.button("🔄 Reset Peaks").clicked() {
                self.metrics.reset_peaks();
            }

            if ui.button("🗑 Reset All").clicked() {
                self.metrics.reset();
            }

            let pause_text = if self.metrics.is_paused() {
                "▶️ Resume"
            } else {
                "⏸ Pause"
            };

            if ui.button(pause_text).clicked() {
                self.metrics.set_paused(!self.metrics.is_paused());
            }

            ui.checkbox(&mut self.auto_reset_counters, "Auto-reset counters");
        });

        ui.add_space(10.0);
        ui.separator();

        // Performance statistics in a grid
        ui.heading("Performance Statistics");
        ui.add_space(5.0);

        egui::Grid::new("performance_stats")
            .num_columns(2)
            .spacing([40.0, 8.0])
            .striped(true)
            .show(ui, |ui| {
                // FPS metrics
                ui.label("Current FPS:");
                ui.label(format!("{:.1} fps", self.metrics.fps()));
                ui.end_row();

                ui.label("1% Low FPS:");
                ui.label(format!("{:.1} fps", self.metrics.fps_1_percent_low()));
                ui.end_row();

                ui.label("0.1% Low FPS:");
                ui.label(format!("{:.1} fps", self.metrics.fps_0_1_percent_low()));
                ui.end_row();

                ui.separator();
                ui.separator();
                ui.end_row();

                // Frame time metrics
                ui.label("Current Frame Time:");
                ui.label(format!("{:.2} ms", self.metrics.frame_time_ms()));
                ui.end_row();

                ui.label("Average Frame Time:");
                ui.label(format!("{:.2} ms", self.metrics.average_frame_time_ms()));
                ui.end_row();

                ui.label("Peak Frame Time:");
                ui.label(format!("{:.2} ms", self.metrics.peak_frame_time_ms()));
                ui.end_row();

                ui.separator();
                ui.separator();
                ui.end_row();

                // GPU memory
                ui.label("GPU Memory Usage:");
                ui.label(format!("{:.1} MB", self.metrics.gpu_memory_mb()));
                ui.end_row();

                ui.separator();
                ui.separator();
                ui.end_row();

                // Command buffer statistics
                ui.label("Command Buffers:");
                ui.label(format!("{}", self.metrics.command_buffer_count()));
                ui.end_row();

                ui.label("Draw Calls:");
                ui.label(format!("{}", self.metrics.draw_call_count()));
                ui.end_row();

                ui.label("Compute Dispatches:");
                ui.label(format!("{}", self.metrics.compute_dispatch_count()));
                ui.end_row();
            });

        ui.add_space(15.0);
        ui.separator();

        // Graph controls
        ui.heading("Performance Graphs");
        ui.add_space(5.0);

        ui.horizontal(|ui| {
            ui.checkbox(&mut self.show_frame_time_graph, "Show Frame Time Graph");
            ui.checkbox(&mut self.show_fps_graph, "Show FPS Graph");
        });

        ui.add_space(5.0);

        // Frame time graph
        if self.show_frame_time_graph {
            self.render_frame_time_graph(ui);
            ui.add_space(10.0);
        }

        // FPS graph
        if self.show_fps_graph {
            self.render_fps_graph(ui);
            ui.add_space(10.0);
        }

        // Performance tips
        ui.separator();
        ui.heading("Performance Tips");
        ui.add_space(5.0);

        ui.label("• Target: 60 FPS (16.67ms per frame) or 120 FPS (8.33ms per frame)");
        ui.label("• Frame times above 16.67ms indicate performance issues");
        ui.label("• Monitor 1% and 0.1% low FPS for frame consistency");
        ui.label("• Reduce draw calls and command buffers for better performance");
        ui.label("• Use GPU profiling tools for detailed analysis");
    }

    /// Render frame time graph
    fn render_frame_time_graph(&self, ui: &mut egui::Ui) {
        use egui_plot::{Line, Plot, PlotPoints};

        ui.label("Frame Time (ms)");

        let frame_times = self.metrics.frame_time_history();
        if frame_times.is_empty() {
            ui.label("No data available yet");
            return;
        }

        let points: PlotPoints = frame_times
            .iter()
            .enumerate()
            .map(|(i, &ft)| [i as f64, ft as f64])
            .collect();

        let line = Line::new(points).name("Frame Time");

        Plot::new("frame_time_plot")
            .height(self.graph_height)
            .show_axes([true, true])
            .show_grid([true, true])
            .allow_zoom(false)
            .allow_drag(false)
            .show(ui, |plot_ui| {
                plot_ui.line(line);

                // Add reference lines for common frame time targets
                let num_samples = frame_times.len();
                let line_60fps = Line::new(vec![
                    [0.0, 16.67],
                    [num_samples as f64, 16.67],
                ])
                .name("60 FPS target")
                .color(egui::Color32::GREEN);

                let line_30fps = Line::new(vec![
                    [0.0, 33.33],
                    [num_samples as f64, 33.33],
                ])
                .name("30 FPS target")
                .color(egui::Color32::YELLOW);

                plot_ui.line(line_60fps);
                plot_ui.line(line_30fps);
            });
    }

    /// Render FPS graph
    fn render_fps_graph(&self, ui: &mut egui::Ui) {
        use egui_plot::{Line, Plot, PlotPoints};

        ui.label("FPS");

        let frame_times = self.metrics.frame_time_history();
        if frame_times.is_empty() {
            ui.label("No data available yet");
            return;
        }

        // Convert frame times to FPS
        let points: PlotPoints = frame_times
            .iter()
            .enumerate()
            .map(|(i, &ft)| {
                let fps = if ft > 0.0 { 1000.0 / ft } else { 0.0 };
                [i as f64, fps as f64]
            })
            .collect();

        let line = Line::new(points).name("FPS");

        Plot::new("fps_plot")
            .height(self.graph_height)
            .show_axes([true, true])
            .show_grid([true, true])
            .allow_zoom(false)
            .allow_drag(false)
            .show(ui, |plot_ui| {
                plot_ui.line(line);

                // Add reference lines for common FPS targets
                let num_samples = frame_times.len();
                let line_60fps = Line::new(vec![
                    [0.0, 60.0],
                    [num_samples as f64, 60.0],
                ])
                .name("60 FPS")
                .color(egui::Color32::GREEN);

                let line_30fps = Line::new(vec![
                    [0.0, 30.0],
                    [num_samples as f64, 30.0],
                ])
                .name("30 FPS")
                .color(egui::Color32::YELLOW);

                plot_ui.line(line_60fps);
                plot_ui.line(line_30fps);
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_panel_creation() {
        let panel = PerformancePanel::new();
        assert!(panel.show_frame_time_graph);
        assert!(panel.show_fps_graph);
        assert!(!panel.auto_reset_counters);
    }

    #[test]
    fn test_metrics_access() {
        let mut panel = PerformancePanel::new();

        // Test mutable access
        panel.metrics_mut().set_command_buffer_count(10);
        assert_eq!(panel.metrics().command_buffer_count(), 10);

        // Test immutable access
        let metrics = panel.metrics();
        assert_eq!(metrics.command_buffer_count(), 10);
    }

    #[test]
    fn test_update_without_auto_reset() {
        let mut panel = PerformancePanel::new();
        panel.auto_reset_counters = false;

        panel.metrics_mut().set_command_buffer_count(5);
        panel.metrics_mut().set_draw_call_count(10);

        panel.update();

        // Counters should not be reset
        assert_eq!(panel.metrics().command_buffer_count(), 5);
        assert_eq!(panel.metrics().draw_call_count(), 10);
    }

    #[test]
    fn test_update_with_auto_reset() {
        let mut panel = PerformancePanel::new();
        panel.auto_reset_counters = true;

        panel.metrics_mut().set_command_buffer_count(5);
        panel.metrics_mut().set_draw_call_count(10);

        panel.update();

        // Counters should be reset
        assert_eq!(panel.metrics().command_buffer_count(), 0);
        assert_eq!(panel.metrics().draw_call_count(), 0);
    }

    #[test]
    fn test_default_trait() {
        let panel = PerformancePanel::default();
        assert!(panel.show_frame_time_graph);
        assert!(panel.show_fps_graph);
    }
}
