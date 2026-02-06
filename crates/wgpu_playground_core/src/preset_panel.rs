/// UI panel for configuration presets
///
/// This panel allows users to browse and load preset configurations
/// for common rendering scenarios.
use crate::preset::{get_all_presets, ConfigPreset, PresetCategory};
use crate::state::PlaygroundState;
use egui::{Color32, RichText};

/// Panel for browsing and loading configuration presets
pub struct PresetPanel {
    /// Currently selected preset index
    selected_preset: Option<usize>,
    /// Category filter
    category_filter: Option<PresetCategory>,
    /// Search query
    search_query: String,
    /// Message to display (e.g., "Preset loaded successfully")
    message: Option<String>,
}

impl PresetPanel {
    /// Create a new preset panel
    pub fn new() -> Self {
        Self {
            selected_preset: None,
            category_filter: None,
            search_query: String::new(),
            message: None,
        }
    }

    /// Render the preset panel UI
    pub fn ui(&mut self, ui: &mut egui::Ui) -> Option<PlaygroundState> {
        let mut state_to_load = None;

        ui.heading("Configuration Presets");
        ui.add_space(10.0);

        ui.label("Browse and load preset configurations for common rendering scenarios.");
        ui.add_space(10.0);

        // Show message if any
        if let Some(ref msg) = self.message {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("ℹ")
                        .color(Color32::from_rgb(70, 130, 180))
                        .size(16.0),
                );
                ui.label(msg);
            });
            ui.add_space(5.0);
        }

        ui.separator();

        // Filter controls
        ui.horizontal(|ui| {
            ui.label("Category:");
            ui.radio_value(&mut self.category_filter, None, "All");
            ui.radio_value(
                &mut self.category_filter,
                Some(PresetCategory::Material),
                "Material",
            );
            ui.radio_value(
                &mut self.category_filter,
                Some(PresetCategory::Lighting),
                "Lighting",
            );
            ui.radio_value(
                &mut self.category_filter,
                Some(PresetCategory::PostProcessing),
                "Post-Processing",
            );
            ui.radio_value(
                &mut self.category_filter,
                Some(PresetCategory::Rendering),
                "Rendering",
            );
        });

        ui.horizontal(|ui| {
            ui.label("Search:");
            ui.text_edit_singleline(&mut self.search_query);
            if ui.button("Clear").clicked() {
                self.search_query.clear();
            }
        });

        ui.add_space(10.0);
        ui.separator();

        // Get and filter presets
        let all_presets = get_all_presets();
        let filtered_presets: Vec<(usize, &ConfigPreset)> = all_presets
            .iter()
            .enumerate()
            .filter(|(_, preset)| {
                // Apply category filter
                if let Some(category) = self.category_filter {
                    if preset.category != category {
                        return false;
                    }
                }

                // Apply search filter
                if !self.search_query.is_empty() {
                    let query_lower = self.search_query.to_lowercase();
                    let matches_name = preset.name.to_lowercase().contains(&query_lower);
                    let matches_description =
                        preset.description.to_lowercase().contains(&query_lower);
                    let matches_tags = preset
                        .tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&query_lower));

                    if !matches_name && !matches_description && !matches_tags {
                        return false;
                    }
                }

                true
            })
            .collect();

        if filtered_presets.is_empty() {
            ui.label("No presets match the current filters.");
            return None;
        }

        // Display presets in a scrollable area
        egui::ScrollArea::vertical()
            .max_height(500.0)
            .show(ui, |ui| {
                for (original_idx, preset) in filtered_presets {
                    let is_selected = self.selected_preset == Some(original_idx);

                    ui.group(|ui| {
                        ui.set_min_width(ui.available_width());

                        // Preset header
                        ui.horizontal(|ui| {
                            // Selection checkbox
                            let mut selected = is_selected;
                            if ui.checkbox(&mut selected, "").changed() {
                                self.selected_preset =
                                    if selected { Some(original_idx) } else { None };
                            }

                            // Category badge
                            let (badge_text, badge_color) = match preset.category {
                                PresetCategory::Material => {
                                    ("Material", Color32::from_rgb(70, 130, 180))
                                }
                                PresetCategory::Lighting => {
                                    ("Lighting", Color32::from_rgb(255, 215, 0))
                                }
                                PresetCategory::PostProcessing => {
                                    ("Post-Processing", Color32::from_rgb(138, 43, 226))
                                }
                                PresetCategory::Rendering => {
                                    ("Rendering", Color32::from_rgb(220, 20, 60))
                                }
                            };

                            ui.label(
                                RichText::new(badge_text)
                                    .color(Color32::WHITE)
                                    .background_color(badge_color)
                                    .size(11.0),
                            );

                            ui.heading(preset.name);
                        });

                        ui.add_space(5.0);

                        // Description
                        ui.label(preset.description);

                        ui.add_space(5.0);

                        // Tags
                        ui.horizontal_wrapped(|ui| {
                            ui.label(RichText::new("Tags:").color(Color32::GRAY));
                            for tag in preset.tags {
                                ui.label(
                                    RichText::new(format!("#{}", tag))
                                        .color(Color32::from_rgb(100, 149, 237))
                                        .size(11.0),
                                );
                            }
                        });

                        ui.add_space(5.0);

                        // Action buttons
                        ui.horizontal(|ui| {
                            if ui.button("Load Preset").clicked() {
                                state_to_load = Some(preset.state.clone());
                                self.message = Some(format!("Loaded preset: {}", preset.name));
                            }

                            if ui.button("View Details").clicked() {
                                self.selected_preset = Some(original_idx);
                            }
                        });

                        // Show details if selected
                        if is_selected {
                            ui.add_space(10.0);
                            ui.separator();
                            ui.label(RichText::new("Preset Configuration Details").strong());
                            ui.add_space(5.0);

                            // Show what's configured in the preset
                            ui.horizontal_wrapped(|ui| {
                                ui.label("Includes:");

                                let mut components = Vec::new();
                                if preset.state.shader_editor.is_some() {
                                    components.push("Shader");
                                }
                                if preset.state.buffer_panel.is_some() {
                                    components.push("Buffer");
                                }
                                if preset.state.texture_panel.is_some() {
                                    components.push("Texture");
                                }
                                if preset.state.sampler_panel.is_some() {
                                    components.push("Sampler");
                                }
                                if preset.state.render_pipeline_panel.is_some() {
                                    components.push("Render Pipeline");
                                }
                                if preset.state.compute_pipeline_panel.is_some() {
                                    components.push("Compute Pipeline");
                                }

                                for component in components {
                                    ui.label(
                                        RichText::new(component)
                                            .color(Color32::from_rgb(50, 150, 100)),
                                    );
                                    ui.label("•");
                                }
                            });

                            ui.add_space(5.0);

                            // Show shader preview if available
                            if let Some(ref shader) = preset.state.shader_editor {
                                ui.collapsing("Shader Preview", |ui| {
                                    ui.add_space(5.0);
                                    egui::ScrollArea::vertical()
                                        .max_height(300.0)
                                        .show(ui, |ui| {
                                            ui.code(&shader.source_code);
                                        });
                                });
                            }
                        }
                    });

                    ui.add_space(10.0);
                }
            });

        ui.add_space(10.0);
        ui.separator();

        // Help section
        ui.collapsing("Help", |ui| {
            ui.add_space(5.0);
            ui.label(RichText::new("How to use presets:").strong());
            ui.add_space(5.0);
            ui.label("1. Browse available presets and read their descriptions");
            ui.label("2. Use category filters and search to find specific presets");
            ui.label("3. Click 'View Details' to see what's included in a preset");
            ui.label("4. Click 'Load Preset' to apply the configuration");
            ui.label("5. After loading, visit the relevant panels to customize the configuration");
            ui.add_space(5.0);
            ui.label(RichText::new("Note:").color(Color32::from_rgb(255, 140, 0)));
            ui.label("Loading a preset will update shader, buffer, texture, sampler, and pipeline configurations.");
            ui.label("You can further customize any loaded preset to match your specific needs.");
        });

        state_to_load
    }
}

impl Default for PresetPanel {
    fn default() -> Self {
        Self::new()
    }
}
