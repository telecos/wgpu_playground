//! UI panel for interactive guided tutorials

use crate::tutorial::{
    get_all_tutorials, Difficulty, HighlightTarget, StepAction, Tutorial, TutorialState,
    TutorialStep, ValidationCheck,
};
use egui::{Color32, RichText, ScrollArea, Ui};

pub struct TutorialPanel {
    tutorials: Vec<Tutorial>,
    state: TutorialState,
    show_completed: bool,
}

impl Default for TutorialPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl TutorialPanel {
    pub fn new() -> Self {
        Self {
            tutorials: get_all_tutorials(),
            state: TutorialState::default(),
            show_completed: false,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.heading("🎓 Guided Tutorials");
        ui.add_space(5.0);

        ui.label("Learn WebGPU step-by-step with interactive tutorials that guide you through creating rendering examples.");
        ui.add_space(10.0);

        // Show tutorial selection or active tutorial
        if let Some(tutorial_idx) = self.state.current_tutorial {
            self.render_active_tutorial(ui, tutorial_idx);
        } else {
            self.render_tutorial_list(ui);
        }
    }

    fn render_tutorial_list(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Available Tutorials");
            ui.add_space(10.0);
            ui.checkbox(&mut self.show_completed, "Show completed");
        });

        ui.separator();
        ui.add_space(5.0);

        ScrollArea::vertical().show(ui, |ui| {
            for (idx, tutorial) in self.tutorials.iter().enumerate() {
                let is_completed = self.state.is_tutorial_completed(&tutorial.id);

                // Skip completed tutorials if filter is active
                if is_completed && !self.show_completed {
                    continue;
                }

                ui.group(|ui| {
                    ui.set_min_width(ui.available_width());

                    // Tutorial header with difficulty badge
                    ui.horizontal(|ui| {
                        // Completion checkmark
                        if is_completed {
                            ui.label(RichText::new("✓").color(Color32::GREEN).size(20.0));
                        } else {
                            ui.label(RichText::new("○").color(Color32::GRAY).size(20.0));
                        }

                        ui.heading(&tutorial.title);
                        ui.add_space(5.0);

                        // Difficulty badge
                        let (diff_text, diff_color) = match tutorial.difficulty {
                            Difficulty::Beginner => ("Beginner", Color32::from_rgb(100, 200, 100)),
                            Difficulty::Intermediate => {
                                ("Intermediate", Color32::from_rgb(255, 180, 100))
                            }
                            Difficulty::Advanced => ("Advanced", Color32::from_rgb(255, 100, 100)),
                        };
                        ui.label(RichText::new(diff_text).color(diff_color));
                    });

                    ui.add_space(3.0);
                    ui.label(&tutorial.description);
                    ui.add_space(3.0);

                    ui.horizontal(|ui| {
                        ui.label(format!("{} steps", tutorial.steps.len()));
                        ui.add_space(10.0);

                        if is_completed {
                            if ui.button("🔄 Restart Tutorial").clicked() {
                                self.state.current_tutorial = Some(idx);
                                self.state.current_step = 0;
                                self.state.visited_panels.clear();
                            }
                        } else if ui.button("▶ Start Tutorial").clicked() {
                            self.state.current_tutorial = Some(idx);
                            self.state.current_step = 0;
                            self.state.visited_panels.clear();
                        }
                    });
                });

                ui.add_space(8.0);
            }
        });
    }

    fn render_active_tutorial(&mut self, ui: &mut Ui, tutorial_idx: usize) {
        // Extract needed data from tutorial to avoid borrow checker issues
        let tutorial = &self.tutorials[tutorial_idx];
        let tutorial_title = tutorial.title.clone();
        let tutorial_steps_len = tutorial.steps.len();
        let tutorial_id = tutorial.id.clone();
        let current_step_idx = self.state.current_step;

        // Tutorial header
        ui.horizontal(|ui| {
            if ui.button("← Back to Tutorials").clicked() {
                self.state.current_tutorial = None;
                self.state.current_step = 0;
                self.state.visited_panels.clear();
            }

            ui.separator();
            ui.heading(&tutorial_title);
        });

        ui.separator();
        ui.add_space(5.0);

        // Progress bar
        let progress = current_step_idx as f32 / tutorial_steps_len as f32;
        ui.horizontal(|ui| {
            ui.label(format!(
                "Progress: Step {} of {}",
                current_step_idx + 1,
                tutorial_steps_len
            ));
            ui.add_space(5.0);
            let progress_bar = egui::ProgressBar::new(progress)
                .text(format!("{:.0}%", progress * 100.0))
                .desired_width(200.0);
            ui.add(progress_bar);
        });

        ui.add_space(10.0);

        // Current step content
        if current_step_idx < tutorial_steps_len {
            let step = self.tutorials[tutorial_idx].steps[current_step_idx].clone();
            self.render_tutorial_step(ui, &step, tutorial_idx);
        } else {
            // Tutorial completed
            self.render_tutorial_completion(ui, &tutorial_id);
        }
    }

    fn render_tutorial_step(&mut self, ui: &mut Ui, step: &TutorialStep, tutorial_idx: usize) {
        ScrollArea::vertical().show(ui, |ui| {
            // Step title
            ui.heading(&step.title);
            ui.add_space(10.0);

            // Step description
            ui.group(|ui| {
                ui.set_min_width(ui.available_width() - 20.0);
                ui.label(RichText::new("📋 Task").strong().size(16.0));
                ui.add_space(5.0);
                ui.label(&step.description);
            });

            ui.add_space(10.0);

            // Explanation section
            ui.group(|ui| {
                ui.set_min_width(ui.available_width() - 20.0);
                ui.label(RichText::new("💡 Explanation").strong().size(16.0));
                ui.add_space(5.0);
                ui.label(&step.explanation);
            });

            ui.add_space(10.0);

            // Highlight panel info
            if let Some(panel) = step.highlight_panel {
                ui.group(|ui| {
                    ui.set_min_width(ui.available_width() - 20.0);
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("🎯 Focus:").strong().color(Color32::YELLOW));
                        ui.label(format!("{:?} panel", panel));
                    });
                });
                ui.add_space(5.0);
            }

            // Action hint
            let action_hint = match &step.action {
                StepAction::ReadAndUnderstand => {
                    "Read the information above to understand the concept"
                }
                StepAction::NavigateToPanel(_) => "Navigate to the specified panel",
                StepAction::ConfigureShader { .. } => "Configure the shader code",
                StepAction::CreateBuffer { .. } => "Create a buffer with the specified type",
                StepAction::CreateTexture => "Create a texture resource",
                StepAction::CreateBindGroup => "Create a bind group for shader resources",
                StepAction::ConfigurePipeline => "Configure the pipeline settings",
                StepAction::ExecuteRender => "Execute the render command",
                StepAction::ExecuteCompute => "Dispatch the compute operation",
            };

            ui.group(|ui| {
                ui.set_min_width(ui.available_width() - 20.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new("✅ Action:").strong().color(Color32::LIGHT_BLUE));
                    ui.label(action_hint);
                });
            });

            ui.add_space(10.0);

            // Validation hint
            if let Some(validation) = &step.validation {
                ui.group(|ui| {
                    ui.set_min_width(ui.available_width() - 20.0);
                    ui.label(RichText::new("💭 Hint").strong().color(Color32::LIGHT_GREEN));
                    ui.add_space(3.0);
                    ui.label(&validation.hint);
                });
                ui.add_space(10.0);
            }

            // Navigation buttons
            ui.separator();
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                // Previous button
                if self.state.current_step > 0 && ui.button("← Previous Step").clicked() {
                    self.state.current_step -= 1;
                }

                ui.add_space(10.0);

                // Next button (with validation check)
                let can_proceed = self.check_step_validation(step);
                let next_button = if can_proceed {
                    ui.button("Next Step →")
                } else {
                    ui.add_enabled(false, egui::Button::new("Next Step →"))
                };

                if next_button.clicked() {
                    self.state.current_step += 1;

                    // Check if tutorial is complete
                    let tutorial = &self.tutorials[tutorial_idx];
                    if self.state.current_step >= tutorial.steps.len() {
                        self.state.complete_current_tutorial(tutorial.id.clone());
                    }
                }

                if !can_proceed {
                    ui.label(
                        RichText::new("Complete the step action to continue")
                            .color(Color32::YELLOW),
                    );
                }
            });
        });
    }

    fn render_tutorial_completion(&mut self, ui: &mut Ui, tutorial_id: &str) {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.heading(RichText::new("🎉 Congratulations!").size(24.0).color(Color32::GREEN));
            ui.add_space(20.0);
            
            let tutorial_title = match tutorial_id {
                "hello_triangle" => "Hello Triangle",
                "adding_textures" => "Adding Textures",
                "3d_with_depth" => "3D with Depth",
                "gpu_compute" => "GPU Compute",
                _ => "Tutorial",
            };
            
            ui.label(RichText::new(format!("You've completed the '{}' tutorial!", tutorial_title)).size(18.0));
            ui.add_space(30.0);

            ui.label("You've learned:");
            ui.add_space(10.0);

            // Summary of key concepts
            match tutorial_id {
                "hello_triangle" => {
                    ui.label("• Vertex buffers for geometry data");
                    ui.label("• Vertex and fragment shaders");
                    ui.label("• Render passes and pipelines");
                    ui.label("• Basic draw commands");
                }
                "adding_textures" => {
                    ui.label("• Loading and creating textures");
                    ui.label("• Texture sampling and filtering");
                    ui.label("• UV coordinate mapping");
                    ui.label("• Bind groups for resources");
                }
                "3d_with_depth" => {
                    ui.label("• Transformation matrices (MVP)");
                    ui.label("• Depth testing and depth buffers");
                    ui.label("• 3D coordinate systems");
                    ui.label("• Uniform buffers");
                }
                "gpu_compute" => {
                    ui.label("• Compute shaders and pipelines");
                    ui.label("• Storage buffers for large data");
                    ui.label("• Workgroups and parallel execution");
                    ui.label("• Dispatch commands");
                }
                _ => {}
            }

            ui.add_space(30.0);

            if ui.button("← Back to Tutorial List").clicked() {
                self.state.current_tutorial = None;
                self.state.current_step = 0;
                self.state.visited_panels.clear();
            }
        });
    }

    fn check_step_validation(&self, step: &TutorialStep) -> bool {
        if let Some(validation) = &step.validation {
            match &validation.check_type {
                ValidationCheck::ManualConfirm => true, // Always allow manual confirmation
                ValidationCheck::PanelVisited(panel) => {
                    self.state.visited_panels.contains(panel)
                }
                ValidationCheck::ShaderCompiled => {
                    // For now, allow manual progression
                    // This could be integrated with actual shader compilation state
                    true
                }
                ValidationCheck::BufferCreated => true,
                ValidationCheck::TextureCreated => true,
                ValidationCheck::BindGroupCreated => true,
                ValidationCheck::RenderExecuted => true,
            }
        } else {
            true
        }
    }

    /// Mark a panel as visited (called from app when user navigates)
    pub fn mark_panel_visited(&mut self, panel: HighlightTarget) {
        self.state.mark_panel_visited(panel);
    }

    /// Get current highlighted panel if tutorial is active
    pub fn get_current_highlight(&self) -> Option<HighlightTarget> {
        if let Some(tutorial_idx) = self.state.current_tutorial {
            let tutorial = &self.tutorials[tutorial_idx];
            if self.state.current_step < tutorial.steps.len() {
                return tutorial.steps[self.state.current_step].highlight_panel;
            }
        }
        None
    }

    /// Check if a specific panel should be highlighted
    pub fn should_highlight_panel(&self, panel: HighlightTarget) -> bool {
        self.get_current_highlight() == Some(panel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tutorial_panel_creation() {
        let panel = TutorialPanel::new();
        assert_eq!(panel.tutorials.len(), 4);
        assert!(panel.state.current_tutorial.is_none());
    }

    #[test]
    fn test_panel_visited_tracking() {
        let mut panel = TutorialPanel::new();
        panel.mark_panel_visited(HighlightTarget::BufferConfig);
        assert!(panel.state.visited_panels.contains(&HighlightTarget::BufferConfig));
    }

    #[test]
    fn test_highlight_tracking() {
        let panel = TutorialPanel::new();
        assert!(panel.get_current_highlight().is_none());
        assert!(!panel.should_highlight_panel(HighlightTarget::BufferConfig));
    }

    #[test]
    fn test_step_validation_manual_confirm() {
        let panel = TutorialPanel::new();
        let step = TutorialStep {
            title: "Test".to_string(),
            description: "Test".to_string(),
            explanation: "Test".to_string(),
            highlight_panel: None,
            action: StepAction::ReadAndUnderstand,
            validation: Some(crate::tutorial::StepValidation {
                check_type: ValidationCheck::ManualConfirm,
                hint: "Test".to_string(),
            }),
        };
        assert!(panel.check_step_validation(&step));
    }
}
