//! Learning Path Visualization Panel
//!
//! Interactive visualization showing the recommended learning path through WebGPU concepts
//! with progress tracking.

use crate::learning_path::{
    get_learning_path, LearningNode, LearningProgress, NodeCategory, NodeDifficulty,
};
use egui::{Color32, Pos2, Rect, RichText, ScrollArea, Stroke, Ui, Vec2};
use std::collections::HashMap;

pub struct LearningPathPanel {
    nodes: Vec<LearningNode>,
    progress: LearningProgress,
    /// Filter by category
    selected_category: Option<NodeCategory>,
    /// Show only available nodes (prerequisites met)
    show_only_available: bool,
    /// Layout cache for node positions
    node_positions: HashMap<String, Pos2>,
}

impl Default for LearningPathPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl LearningPathPanel {
    pub fn new() -> Self {
        Self {
            nodes: get_learning_path(),
            progress: LearningProgress::new(),
            selected_category: None,
            show_only_available: false,
            node_positions: HashMap::new(),
        }
    }

    /// Update progress from tutorial state
    pub fn update_from_tutorial_state(&mut self, completed_tutorials: &[String]) {
        for tutorial_id in completed_tutorials {
            self.progress.complete_tutorial(tutorial_id.clone());
        }
    }

    /// Mark an example as tried
    pub fn mark_example_tried(&mut self, example_id: &str) {
        self.progress.try_example(example_id.to_string());
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.heading("🗺️ WebGPU Learning Path");
        ui.add_space(5.0);

        ui.label("Navigate your journey through WebGPU concepts. Follow the recommended path or explore freely!");
        ui.add_space(10.0);

        // Controls and filters
        self.render_controls(ui);
        ui.add_space(10.0);

        // Progress summary
        self.render_progress_summary(ui);
        ui.add_space(10.0);

        ui.separator();
        ui.add_space(5.0);

        // Main visualization
        ScrollArea::both().show(ui, |ui| {
            self.render_visualization(ui);
        });
    }

    fn render_controls(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Filter by category:");

            if ui
                .selectable_label(self.selected_category.is_none(), "All")
                .clicked()
            {
                self.selected_category = None;
            }

            for category in [
                NodeCategory::Foundation,
                NodeCategory::Resources,
                NodeCategory::Shaders,
                NodeCategory::Rendering,
                NodeCategory::Compute,
                NodeCategory::Advanced,
            ] {
                let is_selected = self.selected_category == Some(category);
                let label = RichText::new(category.name()).color(category.color());

                if ui.selectable_label(is_selected, label).clicked() {
                    self.selected_category = Some(category);
                }
            }
        });

        ui.horizontal(|ui| {
            ui.checkbox(&mut self.show_only_available, "Show only available");
            ui.label("(prerequisites met)");
        });
    }

    fn render_progress_summary(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let total_nodes = self.nodes.len();
            let completed = self
                .nodes
                .iter()
                .filter(|n| self.progress.is_node_completed(n))
                .count();
            let in_progress = self
                .nodes
                .iter()
                .filter(|n| self.progress.is_node_in_progress(n))
                .count();

            ui.label(RichText::new(format!("✓ Completed: {}", completed)).color(Color32::GREEN));
            ui.label(
                RichText::new(format!("◐ In Progress: {}", in_progress)).color(Color32::YELLOW),
            );
            ui.label(
                RichText::new(format!("○ Remaining: {}", total_nodes - completed - in_progress))
                    .color(Color32::GRAY),
            );

            if total_nodes > 0 {
                let percentage = (completed as f32 / total_nodes as f32) * 100.0;
                ui.label(
                    RichText::new(format!("({:.0}% complete)", percentage))
                        .color(Color32::LIGHT_BLUE),
                );
            }
        });
    }

    fn render_visualization(&mut self, ui: &mut Ui) {
        let width = 1200.0;
        let height = 1000.0;

        // Clone filtered nodes to avoid borrow conflicts
        let all_nodes = self.nodes.clone();
        let filtered_nodes: Vec<LearningNode> = all_nodes
            .iter()
            .filter(|n| {
                if let Some(cat) = self.selected_category {
                    if n.category != cat {
                        return false;
                    }
                }
                if self.show_only_available {
                    return self.progress.are_prerequisites_met(n, &self.nodes);
                }
                true
            })
            .cloned()
            .collect();

        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            let (response, painter) =
                ui.allocate_painter(Vec2::new(width, height), egui::Sense::hover());
            let rect = response.rect;

            // Draw background
            painter.rect_filled(rect, 0.0, Color32::from_rgb(25, 25, 30));

            if filtered_nodes.is_empty() {
                painter.text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "No nodes match the current filters",
                    egui::FontId::proportional(16.0),
                    Color32::GRAY,
                );
                return;
            }

            // Create references for layout calculation
            let filtered_refs: Vec<&LearningNode> = filtered_nodes.iter().collect();

            // Calculate layout
            self.calculate_layout(&filtered_refs, rect);

            // Draw connections first (so they appear behind nodes)
            self.draw_connections(&painter, &filtered_refs);

            // Draw nodes
            self.draw_nodes(&painter, &filtered_refs, &response);

            // Draw legend
            self.draw_legend(&painter, rect);
        });
    }

    fn calculate_layout(&mut self, nodes: &[&LearningNode], rect: Rect) {
        self.node_positions.clear();

        // Group nodes by difficulty level for layout
        let mut beginner_nodes = Vec::new();
        let mut intermediate_nodes = Vec::new();
        let mut advanced_nodes = Vec::new();

        for node in nodes {
            match node.difficulty {
                NodeDifficulty::Beginner => beginner_nodes.push(*node),
                NodeDifficulty::Intermediate => intermediate_nodes.push(*node),
                NodeDifficulty::Advanced => advanced_nodes.push(*node),
            }
        }

        let margin = 50.0;
        let column_width = (rect.width() - margin * 2.0) / 3.0;

        // Layout nodes in columns by difficulty
        self.layout_column(&beginner_nodes, rect.left() + margin, rect.top() + 80.0, column_width);
        self.layout_column(
            &intermediate_nodes,
            rect.left() + margin + column_width,
            rect.top() + 80.0,
            column_width,
        );
        self.layout_column(
            &advanced_nodes,
            rect.left() + margin + column_width * 2.0,
            rect.top() + 80.0,
            column_width,
        );
    }

    fn layout_column(&mut self, nodes: &[&LearningNode], x: f32, start_y: f32, width: f32) {
        let node_height = 100.0;
        let spacing = 20.0;
        let center_x = x + width / 2.0;

        for (i, node) in nodes.iter().enumerate() {
            let y = start_y + (i as f32) * (node_height + spacing);
            let pos = Pos2::new(center_x, y + node_height / 2.0);
            self.node_positions.insert(node.id.clone(), pos);
        }
    }

    fn draw_connections(&self, painter: &egui::Painter, nodes: &[&LearningNode]) {
        for node in nodes {
            let Some(node_pos) = self.node_positions.get(&node.id) else {
                continue;
            };

            for prereq_id in &node.prerequisites {
                let Some(prereq_pos) = self.node_positions.get(prereq_id) else {
                    continue;
                };

                // Draw arrow from prerequisite to current node
                let color = if self.progress.is_node_completed(node) {
                    Color32::from_rgba_unmultiplied(100, 255, 100, 100)
                } else if self.progress.are_prerequisites_met(node, &self.nodes) {
                    Color32::from_rgba_unmultiplied(255, 255, 100, 100)
                } else {
                    Color32::from_rgba_unmultiplied(100, 100, 100, 50)
                };

                painter.line_segment(
                    [*prereq_pos, *node_pos],
                    Stroke::new(2.0, color),
                );

                // Draw arrowhead
                self.draw_arrow_head(painter, *prereq_pos, *node_pos, color);
            }
        }
    }

    fn draw_arrow_head(
        &self,
        painter: &egui::Painter,
        from: Pos2,
        to: Pos2,
        color: Color32,
    ) {
        let arrow_size = 8.0;
        let dir = (to - from).normalized();
        let perp = Vec2::new(-dir.y, dir.x);

        // Position arrowhead slightly before the target node
        let arrow_pos = to - dir * 60.0;

        let points = [
            arrow_pos,
            arrow_pos - dir * arrow_size + perp * arrow_size * 0.5,
            arrow_pos - dir * arrow_size - perp * arrow_size * 0.5,
        ];

        painter.add(egui::Shape::convex_polygon(
            points.to_vec(),
            color,
            Stroke::NONE,
        ));
    }

    fn draw_nodes(
        &self,
        painter: &egui::Painter,
        nodes: &[&LearningNode],
        response: &egui::Response,
    ) {
        let node_width = 150.0;
        let node_height = 100.0;

        for node in nodes {
            let Some(center) = self.node_positions.get(&node.id) else {
                continue;
            };

            let rect = Rect::from_center_size(*center, Vec2::new(node_width, node_height));

            // Determine node state and color
            let (bg_color, border_color, border_width) =
                if self.progress.is_node_completed(node) {
                    (
                        node.category.color().linear_multiply(0.4),
                        Color32::GREEN,
                        3.0,
                    )
                } else if self.progress.is_node_in_progress(node) {
                    (
                        node.category.color().linear_multiply(0.3),
                        Color32::YELLOW,
                        2.5,
                    )
                } else if self.progress.are_prerequisites_met(node, &self.nodes) {
                    (
                        node.category.color().linear_multiply(0.2),
                        node.category.color(),
                        2.0,
                    )
                } else {
                    (Color32::from_rgb(40, 40, 45), Color32::DARK_GRAY, 1.5)
                };

            // Draw node box
            painter.rect_filled(rect, 8.0, bg_color);
            painter.rect_stroke(
                rect,
                8.0,
                Stroke::new(border_width, border_color),
                egui::epaint::StrokeKind::Outside,
            );

            // Draw status icon
            let icon_pos = Pos2::new(rect.left() + 15.0, rect.top() + 15.0);
            let icon = if self.progress.is_node_completed(node) {
                "✓"
            } else if self.progress.is_node_in_progress(node) {
                "◐"
            } else if self.progress.are_prerequisites_met(node, &self.nodes) {
                "○"
            } else {
                "🔒"
            };
            painter.text(
                icon_pos,
                egui::Align2::LEFT_TOP,
                icon,
                egui::FontId::proportional(18.0),
                Color32::WHITE,
            );

            // Draw node name
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                &node.name,
                egui::FontId::proportional(14.0),
                Color32::WHITE,
            );

            // Draw difficulty indicator
            let diff_text = match node.difficulty {
                NodeDifficulty::Beginner => "Beginner",
                NodeDifficulty::Intermediate => "Intermediate",
                NodeDifficulty::Advanced => "Advanced",
            };
            painter.text(
                Pos2::new(rect.center().x, rect.bottom() - 12.0),
                egui::Align2::CENTER_CENTER,
                diff_text,
                egui::FontId::proportional(10.0),
                Color32::LIGHT_GRAY,
            );

            // Show tooltip on hover
            if let Some(hover_pos) = response.hover_pos() {
                if rect.contains(hover_pos) {
                    self.show_node_tooltip(painter, node, rect);
                }
            }
        }
    }

    fn show_node_tooltip(&self, painter: &egui::Painter, node: &LearningNode, node_rect: Rect) {
        let tooltip_width = 300.0;
        let tooltip_height = 150.0;
        let tooltip_pos = Pos2::new(node_rect.right() + 10.0, node_rect.top());
        let tooltip_rect =
            Rect::from_min_size(tooltip_pos, Vec2::new(tooltip_width, tooltip_height));

        // Draw tooltip background
        painter.rect_filled(tooltip_rect, 5.0, Color32::from_rgb(45, 45, 50));
        painter.rect_stroke(
            tooltip_rect,
            5.0,
            Stroke::new(1.5, Color32::from_rgb(100, 100, 100)),
            egui::epaint::StrokeKind::Outside,
        );

        let mut y_offset = 15.0;

        // Title
        painter.text(
            Pos2::new(tooltip_rect.left() + 10.0, tooltip_rect.top() + y_offset),
            egui::Align2::LEFT_TOP,
            &node.name,
            egui::FontId::proportional(16.0),
            Color32::WHITE,
        );
        y_offset += 25.0;

        // Description
        painter.text(
            Pos2::new(tooltip_rect.left() + 10.0, tooltip_rect.top() + y_offset),
            egui::Align2::LEFT_TOP,
            &node.description,
            egui::FontId::proportional(12.0),
            Color32::LIGHT_GRAY,
        );
        y_offset += 30.0;

        // Associated tutorials
        if !node.tutorials.is_empty() {
            let tutorials_text = format!("📚 Tutorials: {}", node.tutorials.join(", "));
            painter.text(
                Pos2::new(tooltip_rect.left() + 10.0, tooltip_rect.top() + y_offset),
                egui::Align2::LEFT_TOP,
                tutorials_text,
                egui::FontId::proportional(11.0),
                Color32::from_rgb(150, 200, 255),
            );
            y_offset += 20.0;
        }

        // Associated examples
        if !node.examples.is_empty() {
            let examples_text = format!("💡 Examples: {}", node.examples.join(", "));
            painter.text(
                Pos2::new(tooltip_rect.left() + 10.0, tooltip_rect.top() + y_offset),
                egui::Align2::LEFT_TOP,
                examples_text,
                egui::FontId::proportional(11.0),
                Color32::from_rgb(255, 200, 150),
            );
        }
    }

    fn draw_legend(&self, painter: &egui::Painter, rect: Rect) {
        let legend_x = rect.left() + 20.0;
        let legend_y = rect.top() + 20.0;
        let item_height = 20.0;

        let legend_items = [
            ("✓ Completed", Color32::GREEN),
            ("◐ In Progress", Color32::YELLOW),
            ("○ Available", Color32::LIGHT_BLUE),
            ("🔒 Locked", Color32::DARK_GRAY),
        ];

        for (i, (text, color)) in legend_items.iter().enumerate() {
            let y = legend_y + (i as f32) * item_height;
            painter.text(
                Pos2::new(legend_x, y),
                egui::Align2::LEFT_TOP,
                *text,
                egui::FontId::proportional(12.0),
                *color,
            );
        }
    }

    /// Get reference to progress for saving state
    pub fn progress(&self) -> &LearningProgress {
        &self.progress
    }

    /// Set progress from loaded state
    pub fn set_progress(&mut self, progress: LearningProgress) {
        self.progress = progress;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panel_creation() {
        let panel = LearningPathPanel::new();
        assert!(!panel.nodes.is_empty());
        assert!(panel.progress.completed_tutorials.is_empty());
    }

    #[test]
    fn test_update_from_tutorial_state() {
        let mut panel = LearningPathPanel::new();
        let tutorials = vec!["hello_triangle".to_string()];

        panel.update_from_tutorial_state(&tutorials);
        assert!(panel.progress.completed_tutorials.contains("hello_triangle"));
    }

    #[test]
    fn test_mark_example_tried() {
        let mut panel = LearningPathPanel::new();
        panel.mark_example_tried("triangle");
        assert!(panel.progress.tried_examples.contains("triangle"));
    }

    #[test]
    fn test_category_filter() {
        let mut panel = LearningPathPanel::new();
        panel.selected_category = Some(NodeCategory::Foundation);
        // Just test that the filter can be set without errors
        assert_eq!(panel.selected_category, Some(NodeCategory::Foundation));
    }

    #[test]
    fn test_progress_persistence() {
        let mut panel = LearningPathPanel::new();
        panel.mark_example_tried("triangle");

        // Get progress and create new panel with it
        let progress = panel.progress().clone();
        let mut new_panel = LearningPathPanel::new();
        new_panel.set_progress(progress);

        assert!(new_panel.progress.tried_examples.contains("triangle"));
    }
}
