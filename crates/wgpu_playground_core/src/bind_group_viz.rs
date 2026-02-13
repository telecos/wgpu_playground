/// Bind Group Visualization
///
/// Visual diagram showing bind group layouts and their connections to resources
/// and pipeline stages. Helps users understand resource flow through the pipeline.
use crate::bind_group_panel::{BindGroupLayoutEntryConfig, BindingTypeConfig, ShaderStagesConfig};
use egui::{Color32, Pos2, Rect, Stroke, Vec2};

/// Layout parameters for visualization positioning
struct LayoutParams {
    pipeline_x: f32,
    binding_x: f32,
    resource_x: f32,
    start_y: f32,
}

/// Visualizer for bind group layouts
pub struct BindGroupVisualizer {
    /// Preview canvas size
    pub width: f32,
    pub height: f32,
}

impl Default for BindGroupVisualizer {
    fn default() -> Self {
        Self::new()
    }
}

impl BindGroupVisualizer {
    /// Create a new bind group visualizer
    pub fn new() -> Self {
        Self {
            width: 800.0,
            height: 600.0,
        }
    }

    /// Render the bind group visualization
    pub fn render(
        &self,
        ui: &mut egui::Ui,
        layout_entries: &[BindGroupLayoutEntryConfig],
        layout_label: &str,
        binding_assignments: &[(u32, String)], // (binding, resource_name)
    ) {
        if layout_entries.is_empty() {
            ui.heading("No Bind Group Layout");
            ui.label("Create a bind group layout to see the visualization.");
            return;
        }

        // Create a frame for the visualization
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            let (response, painter) =
                ui.allocate_painter(Vec2::new(self.width, self.height), egui::Sense::hover());
            let rect = response.rect;

            // Draw background
            painter.rect_filled(rect, 0.0, Color32::from_rgb(30, 30, 35));

            // Draw title
            let title = if layout_label.is_empty() {
                "Bind Group Visualization"
            } else {
                layout_label
            };
            self.draw_title(&painter, rect, title);

            // Calculate layout
            let layout_params = LayoutParams {
                pipeline_x: rect.left() + 50.0,
                binding_x: rect.center().x - 100.0,
                resource_x: rect.right() - 200.0,
                start_y: rect.top() + 100.0,
            };

            // Draw pipeline stages section
            self.draw_pipeline_stages(&painter, rect, layout_params.pipeline_x);

            // Draw bindings section with connections
            self.draw_bindings(
                &painter,
                layout_entries,
                &layout_params,
                binding_assignments,
            );

            // Draw legend
            self.draw_legend(&painter, rect);
        });
    }

    /// Draw title
    fn draw_title(&self, painter: &egui::Painter, rect: Rect, title: &str) {
        let title_pos = Pos2::new(rect.center().x, rect.top() + 25.0);
        painter.text(
            title_pos,
            egui::Align2::CENTER_CENTER,
            title,
            egui::FontId::proportional(20.0),
            Color32::WHITE,
        );
    }

    /// Draw pipeline stages column
    fn draw_pipeline_stages(&self, painter: &egui::Painter, rect: Rect, x: f32) {
        let stages = [
            ("Vertex", Color32::from_rgb(100, 150, 255)),
            ("Fragment", Color32::from_rgb(255, 150, 100)),
            ("Compute", Color32::from_rgb(150, 255, 100)),
        ];

        let stage_height = 80.0;
        let start_y = rect.top() + 100.0;

        // Draw label
        painter.text(
            Pos2::new(x, start_y - 30.0),
            egui::Align2::LEFT_CENTER,
            "Pipeline Stages",
            egui::FontId::proportional(14.0),
            Color32::LIGHT_GRAY,
        );

        for (i, (stage_name, color)) in stages.iter().enumerate() {
            let y = start_y + (i as f32) * (stage_height + 20.0);
            let stage_rect = Rect::from_min_size(Pos2::new(x, y), Vec2::new(120.0, stage_height));

            // Draw stage box
            painter.rect_filled(stage_rect, 5.0, color.linear_multiply(0.3));
            painter.rect_stroke(
                stage_rect,
                5.0,
                Stroke::new(2.0, *color),
                egui::epaint::StrokeKind::Outside,
            );

            // Draw stage name
            painter.text(
                stage_rect.center(),
                egui::Align2::CENTER_CENTER,
                *stage_name,
                egui::FontId::proportional(16.0),
                Color32::WHITE,
            );
        }
    }

    /// Draw bindings with connections to pipeline stages and resources
    fn draw_bindings(
        &self,
        painter: &egui::Painter,
        layout_entries: &[BindGroupLayoutEntryConfig],
        layout_params: &LayoutParams,
        binding_assignments: &[(u32, String)],
    ) {
        let binding_height = 60.0;
        let binding_spacing = 20.0;
        let start_y = layout_params.start_y;

        // Draw bindings label
        painter.text(
            Pos2::new(layout_params.binding_x, start_y - 30.0),
            egui::Align2::LEFT_CENTER,
            "Bindings",
            egui::FontId::proportional(14.0),
            Color32::LIGHT_GRAY,
        );

        // Draw resources label
        painter.text(
            Pos2::new(layout_params.resource_x, start_y - 30.0),
            egui::Align2::LEFT_CENTER,
            "Resources",
            egui::FontId::proportional(14.0),
            Color32::LIGHT_GRAY,
        );

        for (i, entry) in layout_entries.iter().enumerate() {
            let y = start_y + (i as f32) * (binding_height + binding_spacing);
            let binding_rect = Rect::from_min_size(
                Pos2::new(layout_params.binding_x, y),
                Vec2::new(200.0, binding_height),
            );

            // Get color based on binding type
            let color = self.get_binding_type_color(&entry.binding_type);

            // Draw binding box
            painter.rect_filled(binding_rect, 5.0, color.linear_multiply(0.2));
            painter.rect_stroke(
                binding_rect,
                5.0,
                Stroke::new(2.0, color),
                egui::epaint::StrokeKind::Outside,
            );

            // Draw binding number and type
            let binding_text = format!("Binding {}", entry.binding);
            let type_text = entry.binding_type.name();

            painter.text(
                Pos2::new(binding_rect.left() + 10.0, binding_rect.top() + 15.0),
                egui::Align2::LEFT_CENTER,
                binding_text,
                egui::FontId::proportional(14.0),
                Color32::WHITE,
            );

            painter.text(
                Pos2::new(binding_rect.left() + 10.0, binding_rect.top() + 35.0),
                egui::Align2::LEFT_CENTER,
                type_text,
                egui::FontId::proportional(12.0),
                Color32::LIGHT_GRAY,
            );

            // Draw connections to pipeline stages
            self.draw_pipeline_connections(
                painter,
                &entry.visibility,
                Pos2::new(binding_rect.left(), binding_rect.center().y),
                layout_params.pipeline_x,
                start_y,
                color,
            );

            // Draw connections to resources
            if let Some((_, resource_name)) = binding_assignments
                .iter()
                .find(|(b, _)| *b == entry.binding)
            {
                self.draw_resource_connection(
                    painter,
                    Pos2::new(binding_rect.right(), binding_rect.center().y),
                    layout_params.resource_x,
                    y,
                    binding_height,
                    resource_name,
                    color,
                );
            }
        }
    }

    /// Draw connections from binding to pipeline stages
    fn draw_pipeline_connections(
        &self,
        painter: &egui::Painter,
        visibility: &ShaderStagesConfig,
        binding_pos: Pos2,
        pipeline_x: f32,
        start_y: f32,
        color: Color32,
    ) {
        let stage_height = 80.0;
        let stage_spacing = 20.0;
        let stage_width = 120.0;

        if visibility.vertex {
            let target = Pos2::new(pipeline_x + stage_width, start_y + stage_height / 2.0);
            self.draw_connection_line(painter, binding_pos, target, color, "Vertex");
        }

        if visibility.fragment {
            let target = Pos2::new(
                pipeline_x + stage_width,
                start_y + stage_height + stage_spacing + stage_height / 2.0,
            );
            self.draw_connection_line(painter, binding_pos, target, color, "Fragment");
        }

        if visibility.compute {
            let target = Pos2::new(
                pipeline_x + stage_width,
                start_y + 2.0 * (stage_height + stage_spacing) + stage_height / 2.0,
            );
            self.draw_connection_line(painter, binding_pos, target, color, "Compute");
        }
    }

    /// Draw connection from binding to resource
    #[allow(clippy::too_many_arguments)]
    fn draw_resource_connection(
        &self,
        painter: &egui::Painter,
        binding_pos: Pos2,
        resource_x: f32,
        y: f32,
        height: f32,
        resource_name: &str,
        color: Color32,
    ) {
        // Draw resource box
        let resource_rect = Rect::from_min_size(Pos2::new(resource_x, y), Vec2::new(180.0, height));
        painter.rect_filled(resource_rect, 5.0, color.linear_multiply(0.15));
        painter.rect_stroke(
            resource_rect,
            5.0,
            Stroke::new(1.5, color),
            egui::epaint::StrokeKind::Outside,
        );

        // Draw resource name
        painter.text(
            resource_rect.center(),
            egui::Align2::CENTER_CENTER,
            resource_name,
            egui::FontId::proportional(13.0),
            Color32::WHITE,
        );

        // Draw connection line
        let target = Pos2::new(resource_rect.left(), resource_rect.center().y);
        self.draw_connection_line(painter, binding_pos, target, color, "");
    }

    /// Draw a connection line between two points
    fn draw_connection_line(
        &self,
        painter: &egui::Painter,
        from: Pos2,
        to: Pos2,
        color: Color32,
        label: &str,
    ) {
        // Simple line (can be upgraded to curved later)
        painter.line_segment([from, to], Stroke::new(2.0, color.linear_multiply(0.6)));

        // Draw arrow head at target
        let arrow_size = 8.0;
        let direction = (to - from).normalized();
        let perpendicular = Vec2::new(-direction.y, direction.x);

        let arrow_tip = to;
        let arrow_left = to - direction * arrow_size + perpendicular * arrow_size * 0.5;
        let arrow_right = to - direction * arrow_size - perpendicular * arrow_size * 0.5;

        painter.add(egui::Shape::convex_polygon(
            vec![arrow_tip, arrow_left, arrow_right],
            color.linear_multiply(0.6),
            Stroke::NONE,
        ));

        // Add label at midpoint of line if provided and non-empty
        if !label.is_empty() {
            let midpoint = (from + to.to_vec2()) * 0.5;
            let label_color = Color32::from_gray(200);
            painter.text(
                midpoint,
                egui::Align2::CENTER_CENTER,
                label,
                egui::FontId::proportional(10.0),
                label_color,
            );
        }
    }

    /// Get color for binding type
    pub fn get_binding_type_color(&self, binding_type: &BindingTypeConfig) -> Color32 {
        match binding_type {
            BindingTypeConfig::UniformBuffer => Color32::from_rgb(50, 150, 100), // Sea Green
            BindingTypeConfig::StorageBuffer { .. } => Color32::from_rgb(70, 130, 180), // Steel Blue
            BindingTypeConfig::Texture => Color32::from_rgb(255, 140, 0), // Dark Orange
            BindingTypeConfig::Sampler => Color32::from_rgb(218, 165, 32), // Goldenrod
            BindingTypeConfig::StorageTexture => Color32::from_rgb(220, 20, 60), // Crimson
        }
    }

    /// Draw legend
    fn draw_legend(&self, painter: &egui::Painter, rect: Rect) {
        let legend_x = rect.left() + 20.0;
        let legend_y = rect.bottom() - 120.0;

        painter.text(
            Pos2::new(legend_x, legend_y),
            egui::Align2::LEFT_CENTER,
            "Legend:",
            egui::FontId::proportional(12.0),
            Color32::LIGHT_GRAY,
        );

        let binding_types = [
            ("Uniform Buffer", BindingTypeConfig::UniformBuffer),
            (
                "Storage Buffer",
                BindingTypeConfig::StorageBuffer { read_only: true },
            ),
            ("Texture", BindingTypeConfig::Texture),
            ("Sampler", BindingTypeConfig::Sampler),
            ("Storage Texture", BindingTypeConfig::StorageTexture),
        ];

        for (i, (name, binding_type)) in binding_types.iter().enumerate() {
            let y = legend_y + 20.0 + (i as f32) * 18.0;
            let color = self.get_binding_type_color(binding_type);

            // Draw color box
            let box_rect = Rect::from_min_size(Pos2::new(legend_x, y - 5.0), Vec2::new(12.0, 12.0));
            painter.rect_filled(box_rect, 2.0, color);
            painter.rect_stroke(
                box_rect,
                2.0,
                Stroke::new(1.0, Color32::WHITE),
                egui::epaint::StrokeKind::Outside,
            );

            // Draw label
            painter.text(
                Pos2::new(legend_x + 18.0, y),
                egui::Align2::LEFT_CENTER,
                *name,
                egui::FontId::proportional(11.0),
                Color32::LIGHT_GRAY,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visualizer_creation() {
        let viz = BindGroupVisualizer::new();
        assert_eq!(viz.width, 800.0);
        assert_eq!(viz.height, 600.0);
    }

    #[test]
    fn test_binding_type_colors() {
        let viz = BindGroupVisualizer::new();

        let uniform_color = viz.get_binding_type_color(&BindingTypeConfig::UniformBuffer);
        assert_ne!(uniform_color, Color32::TRANSPARENT);

        let texture_color = viz.get_binding_type_color(&BindingTypeConfig::Texture);
        assert_ne!(texture_color, Color32::TRANSPARENT);

        // Different types should have different colors
        assert_ne!(uniform_color, texture_color);
    }
}
