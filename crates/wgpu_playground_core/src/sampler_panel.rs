use crate::sampler::{
    AddressMode, CompareFunction, FilterMode, MipmapFilterMode, SamplerDescriptor,
};

/// UI panel for creating and configuring GPU samplers
pub struct SamplerPanel {
    /// Current sampler descriptor being configured
    descriptor: SamplerDescriptor,
    /// Label input text
    label_input: String,
    /// Address mode for U coordinate
    address_mode_u: AddressMode,
    /// Address mode for V coordinate
    address_mode_v: AddressMode,
    /// Address mode for W coordinate
    address_mode_w: AddressMode,
    /// Magnification filter mode
    mag_filter: FilterMode,
    /// Minification filter mode
    min_filter: FilterMode,
    /// Mipmap filter mode
    mipmap_filter: MipmapFilterMode,
    /// LOD min clamp input text
    lod_min_input: String,
    /// LOD max clamp input text
    lod_max_input: String,
    /// Whether to enable comparison function
    enable_compare: bool,
    /// Comparison function
    compare_function: CompareFunction,
    /// Anisotropy level (1-16)
    anisotropy: u16,
    /// Whether to enable border color
    enable_border_color: bool,
    /// Border color selection
    border_color: BorderColorChoice,
    /// Validation error message
    validation_error: Option<String>,
    /// Success message
    success_message: Option<String>,
}

/// Border color options for UI selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BorderColorChoice {
    TransparentBlack,
    OpaqueBlack,
    OpaqueWhite,
    Zero,
}

impl BorderColorChoice {
    fn to_wgpu(self) -> wgpu::SamplerBorderColor {
        match self {
            BorderColorChoice::TransparentBlack => wgpu::SamplerBorderColor::TransparentBlack,
            BorderColorChoice::OpaqueBlack => wgpu::SamplerBorderColor::OpaqueBlack,
            BorderColorChoice::OpaqueWhite => wgpu::SamplerBorderColor::OpaqueWhite,
            BorderColorChoice::Zero => wgpu::SamplerBorderColor::Zero,
        }
    }

    fn all() -> Vec<Self> {
        vec![
            BorderColorChoice::TransparentBlack,
            BorderColorChoice::OpaqueBlack,
            BorderColorChoice::OpaqueWhite,
            BorderColorChoice::Zero,
        ]
    }

    fn name(&self) -> &'static str {
        match self {
            BorderColorChoice::TransparentBlack => "Transparent Black",
            BorderColorChoice::OpaqueBlack => "Opaque Black",
            BorderColorChoice::OpaqueWhite => "Opaque White",
            BorderColorChoice::Zero => "Zero",
        }
    }
}

impl Default for SamplerPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl SamplerPanel {
    /// Create a new sampler panel with default values
    pub fn new() -> Self {
        Self {
            descriptor: SamplerDescriptor::default(),
            label_input: String::new(),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: MipmapFilterMode::Nearest,
            lod_min_input: "0.0".to_string(),
            lod_max_input: "32.0".to_string(),
            enable_compare: false,
            compare_function: CompareFunction::Less,
            anisotropy: 1,
            enable_border_color: false,
            border_color: BorderColorChoice::TransparentBlack,
            validation_error: None,
            success_message: None,
        }
    }

    /// Update the internal descriptor based on current UI state
    fn update_descriptor(&mut self) {
        // Parse LOD values
        let lod_min = self.lod_min_input.parse::<f32>().unwrap_or(0.0);
        let lod_max = self.lod_max_input.parse::<f32>().unwrap_or(32.0);

        // Create descriptor
        let label = if self.label_input.is_empty() {
            None
        } else {
            Some(self.label_input.as_str())
        };

        let mut descriptor = SamplerDescriptor::new(label)
            .with_address_mode_u(self.address_mode_u)
            .with_address_mode_v(self.address_mode_v)
            .with_address_mode_w(self.address_mode_w)
            .with_mag_filter(self.mag_filter)
            .with_min_filter(self.min_filter)
            .with_mipmap_filter(self.mipmap_filter)
            .with_lod_clamp(lod_min, lod_max)
            .with_anisotropy(self.anisotropy);

        // Add optional compare function
        if self.enable_compare {
            descriptor = descriptor.with_compare(self.compare_function);
        }

        // Add optional border color
        if self.enable_border_color {
            descriptor = descriptor.with_border_color(self.border_color.to_wgpu());
        }

        self.descriptor = descriptor;
    }

    /// Validate the current configuration
    fn validate(&mut self) -> bool {
        self.update_descriptor();

        match self.descriptor.validate() {
            Ok(_) => {
                self.validation_error = None;
                true
            }
            Err(e) => {
                self.validation_error = Some(e.to_string());
                self.success_message = None;
                false
            }
        }
    }

    /// Create a sampler with the current configuration
    /// Returns a sampler that can be used with the GPU
    pub fn create_sampler(&mut self, device: &wgpu::Device) -> Option<wgpu::Sampler> {
        if !self.validate() {
            return None;
        }

        match self.descriptor.create_sampler(device) {
            Ok(sampler) => {
                self.success_message = Some(format!(
                    "✓ Sampler created successfully: {}",
                    self.descriptor.label().unwrap_or("<unnamed>")
                ));
                self.validation_error = None;
                Some(sampler)
            }
            Err(e) => {
                self.validation_error = Some(format!("Failed to create sampler: {}", e));
                self.success_message = None;
                None
            }
        }
    }

    /// Render the sampler configuration UI
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("🎨 Sampler Configuration");
            ui.label("Configure and create GPU samplers for texture sampling.");
            ui.add_space(10.0);

            // Sampler Label
            ui.group(|ui| {
                ui.heading("Sampler Properties");
                ui.add_space(5.0);

                egui::Grid::new("sampler_properties")
                    .num_columns(2)
                    .spacing([10.0, 8.0])
                    .show(ui, |ui| {
                        ui.label("Label:");
                        ui.text_edit_singleline(&mut self.label_input);
                        ui.end_row();
                    });
            });

            ui.add_space(10.0);

            // Address Modes
            ui.group(|ui| {
                ui.heading("Address Modes");
                ui.label("Control how texture coordinates outside [0, 1] are handled:");
                ui.add_space(5.0);

                egui::Grid::new("address_modes")
                    .num_columns(2)
                    .spacing([10.0, 8.0])
                    .show(ui, |ui| {
                        ui.label("U (horizontal):");
                        Self::render_address_mode_combo(ui, &mut self.address_mode_u, "address_u");
                        ui.end_row();

                        ui.label("V (vertical):");
                        Self::render_address_mode_combo(ui, &mut self.address_mode_v, "address_v");
                        ui.end_row();

                        ui.label("W (depth):");
                        Self::render_address_mode_combo(ui, &mut self.address_mode_w, "address_w");
                        ui.end_row();
                    });
            });

            ui.add_space(10.0);

            // Filter Modes
            ui.group(|ui| {
                ui.heading("Filter Modes");
                ui.label("Control how textures are sampled and filtered:");
                ui.add_space(5.0);

                egui::Grid::new("filter_modes")
                    .num_columns(2)
                    .spacing([10.0, 8.0])
                    .show(ui, |ui| {
                        ui.label("Magnification (zoom in):")
                            .on_hover_text("Filter when pixel is smaller than texel");
                        Self::render_filter_mode_combo(ui, &mut self.mag_filter, "mag_filter");
                        ui.end_row();

                        ui.label("Minification (zoom out):")
                            .on_hover_text("Filter when pixel is larger than texel");
                        Self::render_filter_mode_combo(ui, &mut self.min_filter, "min_filter");
                        ui.end_row();

                        ui.label("Mipmap:")
                            .on_hover_text("Filter between mipmap levels");
                        Self::render_mipmap_filter_combo(ui, &mut self.mipmap_filter, "mipmap_filter");
                        ui.end_row();
                    });
            });

            ui.add_space(10.0);

            // LOD Clamping
            ui.group(|ui| {
                ui.heading("LOD Clamping");
                ui.label("Limit the level of detail range:");
                ui.add_space(5.0);

                egui::Grid::new("lod_clamp")
                    .num_columns(2)
                    .spacing([10.0, 8.0])
                    .show(ui, |ui| {
                        ui.label("Min LOD:");
                        ui.text_edit_singleline(&mut self.lod_min_input);
                        ui.end_row();

                        ui.label("Max LOD:");
                        ui.text_edit_singleline(&mut self.lod_max_input);
                        ui.end_row();
                    });
            });

            ui.add_space(10.0);

            // Anisotropic Filtering
            ui.group(|ui| {
                ui.heading("Anisotropic Filtering");
                ui.label("Improve texture quality at oblique angles (1 = disabled, 16 = maximum quality):");
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.add(egui::Slider::new(&mut self.anisotropy, 1..=16).text("Level"));
                });
            });

            ui.add_space(10.0);

            // Comparison Function
            ui.group(|ui| {
                ui.heading("Comparison Function");
                ui.label("Optional depth/stencil comparison for shadow mapping:");
                ui.add_space(5.0);

                ui.checkbox(&mut self.enable_compare, "Enable comparison");

                if self.enable_compare {
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.label("Function:");
                        Self::render_compare_function_combo(ui, &mut self.compare_function, "compare_func");
                    });
                }
            });

            ui.add_space(10.0);

            // Border Color
            ui.group(|ui| {
                ui.heading("Border Color");
                ui.label("Color used when address mode is ClampToBorder:");
                ui.add_space(5.0);

                ui.checkbox(&mut self.enable_border_color, "Enable border color");

                if self.enable_border_color {
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.label("Color:");
                        Self::render_border_color_combo(ui, &mut self.border_color, "border_color");
                    });
                }

                // Auto-enable border color if using ClampToBorder
                if (self.address_mode_u == AddressMode::ClampToBorder
                    || self.address_mode_v == AddressMode::ClampToBorder
                    || self.address_mode_w == AddressMode::ClampToBorder)
                    && !self.enable_border_color
                {
                    ui.add_space(5.0);
                    ui.colored_label(
                        egui::Color32::from_rgb(200, 200, 100),
                        "⚠ Border color should be enabled when using ClampToBorder"
                    );
                }
            });

            ui.add_space(15.0);

            // Validation and Creation
            ui.horizontal(|ui| {
                if ui.button("🔍 Validate").clicked() {
                    self.validate();
                }

                if ui.button("✨ Create Sampler").clicked() {
                    // Note: In the actual implementation, we would need a device reference
                    // For now, we just validate
                    if self.validate() {
                        self.success_message = Some(
                            "✓ Configuration is valid. In a full implementation, the sampler would be created here."
                                .to_string(),
                        );
                    }
                }

                if ui.button("🔄 Reset").clicked() {
                    *self = Self::new();
                }
            });

            ui.add_space(10.0);

            // Display validation errors or success messages
            if let Some(error) = &self.validation_error {
                ui.colored_label(egui::Color32::RED, format!("❌ {}", error));
            }

            if let Some(success) = &self.success_message {
                ui.colored_label(egui::Color32::GREEN, success);
            }

            ui.add_space(15.0);

            // Current Configuration Summary
            ui.group(|ui| {
                ui.heading("Configuration Summary");
                ui.add_space(5.0);

                self.update_descriptor();

                ui.monospace(format!(
                    "Label: {}",
                    self.descriptor.label().unwrap_or("<none>")
                ));
                ui.monospace(format!("Address U: {:?}", self.descriptor.address_mode_u()));
                ui.monospace(format!("Address V: {:?}", self.descriptor.address_mode_v()));
                ui.monospace(format!("Address W: {:?}", self.descriptor.address_mode_w()));
                ui.monospace(format!("Mag Filter: {:?}", self.descriptor.mag_filter()));
                ui.monospace(format!("Min Filter: {:?}", self.descriptor.min_filter()));
                ui.monospace(format!("Mipmap Filter: {:?}", self.descriptor.mipmap_filter()));
                ui.monospace(format!("LOD Clamp: {:.1} - {:.1}", 
                    self.descriptor.lod_min_clamp(),
                    self.descriptor.lod_max_clamp()
                ));
                ui.monospace(format!("Anisotropy: {}", self.descriptor.anisotropy_clamp()));

                if let Some(compare) = self.descriptor.compare() {
                    ui.monospace(format!("Compare: {:?}", compare));
                } else {
                    ui.monospace("Compare: None");
                }

                if let Some(border) = self.descriptor.border_color() {
                    ui.monospace(format!("Border Color: {:?}", border));
                } else {
                    ui.monospace("Border Color: None");
                }
            });
        });
    }

    fn render_address_mode_combo(ui: &mut egui::Ui, current: &mut AddressMode, id: &str) {
        egui::ComboBox::from_id_salt(id)
            .selected_text(format!("{:?}", current))
            .show_ui(ui, |ui| {
                ui.selectable_value(current, AddressMode::ClampToEdge, "ClampToEdge")
                    .on_hover_text("Clamp coordinates to edge");
                ui.selectable_value(current, AddressMode::Repeat, "Repeat")
                    .on_hover_text("Wrap coordinates, creating repeating pattern");
                ui.selectable_value(current, AddressMode::MirrorRepeat, "MirrorRepeat")
                    .on_hover_text("Mirror coordinates back and forth");
                ui.selectable_value(current, AddressMode::ClampToBorder, "ClampToBorder")
                    .on_hover_text("Use border color outside [0, 1]");
            });
    }

    fn render_filter_mode_combo(ui: &mut egui::Ui, current: &mut FilterMode, id: &str) {
        egui::ComboBox::from_id_salt(id)
            .selected_text(format!("{:?}", current))
            .show_ui(ui, |ui| {
                ui.selectable_value(current, FilterMode::Nearest, "Nearest")
                    .on_hover_text("Use nearest texel value");
                ui.selectable_value(current, FilterMode::Linear, "Linear")
                    .on_hover_text("Interpolate between texels");
            });
    }

    fn render_mipmap_filter_combo(ui: &mut egui::Ui, current: &mut MipmapFilterMode, id: &str) {
        egui::ComboBox::from_id_salt(id)
            .selected_text(format!("{:?}", current))
            .show_ui(ui, |ui| {
                ui.selectable_value(current, MipmapFilterMode::Nearest, "Nearest")
                    .on_hover_text("Use nearest mipmap level");
                ui.selectable_value(current, MipmapFilterMode::Linear, "Linear")
                    .on_hover_text("Interpolate between mipmap levels");
            });
    }

    fn render_compare_function_combo(ui: &mut egui::Ui, current: &mut CompareFunction, id: &str) {
        egui::ComboBox::from_id_salt(id)
            .selected_text(format!("{:?}", current))
            .show_ui(ui, |ui| {
                ui.selectable_value(current, CompareFunction::Never, "Never");
                ui.selectable_value(current, CompareFunction::Less, "Less");
                ui.selectable_value(current, CompareFunction::Equal, "Equal");
                ui.selectable_value(current, CompareFunction::LessEqual, "LessEqual");
                ui.selectable_value(current, CompareFunction::Greater, "Greater");
                ui.selectable_value(current, CompareFunction::NotEqual, "NotEqual");
                ui.selectable_value(current, CompareFunction::GreaterEqual, "GreaterEqual");
                ui.selectable_value(current, CompareFunction::Always, "Always");
            });
    }

    fn render_border_color_combo(ui: &mut egui::Ui, current: &mut BorderColorChoice, id: &str) {
        egui::ComboBox::from_id_salt(id)
            .selected_text(current.name())
            .show_ui(ui, |ui| {
                for color in BorderColorChoice::all() {
                    ui.selectable_value(current, color, color.name());
                }
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sampler_panel_creation() {
        let panel = SamplerPanel::new();
        assert_eq!(panel.label_input, "");
        assert_eq!(panel.address_mode_u, AddressMode::ClampToEdge);
        assert_eq!(panel.address_mode_v, AddressMode::ClampToEdge);
        assert_eq!(panel.address_mode_w, AddressMode::ClampToEdge);
        assert_eq!(panel.mag_filter, FilterMode::Nearest);
        assert_eq!(panel.min_filter, FilterMode::Nearest);
        assert_eq!(panel.mipmap_filter, MipmapFilterMode::Nearest);
        assert_eq!(panel.lod_min_input, "0.0");
        assert_eq!(panel.lod_max_input, "32.0");
        assert!(!panel.enable_compare);
        assert_eq!(panel.anisotropy, 1);
        assert!(!panel.enable_border_color);
    }

    #[test]
    fn test_sampler_panel_default() {
        let panel = SamplerPanel::default();
        assert_eq!(panel.lod_min_input, "0.0");
        assert_eq!(panel.lod_max_input, "32.0");
    }

    #[test]
    fn test_update_descriptor() {
        let mut panel = SamplerPanel::new();
        panel.label_input = "test_sampler".to_string();
        panel.address_mode_u = AddressMode::Repeat;
        panel.mag_filter = FilterMode::Linear;
        panel.min_filter = FilterMode::Linear;
        panel.anisotropy = 16;

        panel.update_descriptor();

        assert_eq!(panel.descriptor.label(), Some("test_sampler"));
        assert_eq!(panel.descriptor.address_mode_u(), AddressMode::Repeat);
        assert_eq!(panel.descriptor.mag_filter(), FilterMode::Linear);
        assert_eq!(panel.descriptor.min_filter(), FilterMode::Linear);
        assert_eq!(panel.descriptor.anisotropy_clamp(), 16);
    }

    #[test]
    fn test_validate_success() {
        let mut panel = SamplerPanel::new();
        panel.label_input = "valid_sampler".to_string();

        let valid = panel.validate();
        assert!(valid);
        assert!(panel.validation_error.is_none());
    }

    #[test]
    fn test_validate_invalid_lod() {
        let mut panel = SamplerPanel::new();
        panel.lod_min_input = "10.0".to_string();
        panel.lod_max_input = "5.0".to_string();

        let valid = panel.validate();
        assert!(!valid);
        assert!(panel.validation_error.is_some());
        assert!(panel
            .validation_error
            .as_ref()
            .unwrap()
            .contains("lod_min_clamp"));
    }

    #[test]
    fn test_validate_invalid_anisotropy() {
        let mut panel = SamplerPanel::new();
        panel.anisotropy = 0; // Invalid: must be 1-16

        let valid = panel.validate();
        assert!(!valid);
        assert!(panel.validation_error.is_some());
    }

    #[test]
    fn test_validate_clamp_to_border_without_color() {
        let mut panel = SamplerPanel::new();
        panel.address_mode_u = AddressMode::ClampToBorder;
        panel.enable_border_color = false;

        let valid = panel.validate();
        assert!(!valid);
        assert!(panel.validation_error.is_some());
        assert!(panel
            .validation_error
            .as_ref()
            .unwrap()
            .contains("border_color"));
    }

    #[test]
    fn test_validate_clamp_to_border_with_color() {
        let mut panel = SamplerPanel::new();
        panel.address_mode_u = AddressMode::ClampToBorder;
        panel.enable_border_color = true;
        panel.border_color = BorderColorChoice::OpaqueBlack;

        let valid = panel.validate();
        assert!(valid);
        assert!(panel.validation_error.is_none());
    }

    #[test]
    fn test_compare_function_enabled() {
        let mut panel = SamplerPanel::new();
        panel.enable_compare = true;
        panel.compare_function = CompareFunction::LessEqual;

        panel.update_descriptor();

        assert_eq!(panel.descriptor.compare(), Some(CompareFunction::LessEqual));
    }

    #[test]
    fn test_compare_function_disabled() {
        let mut panel = SamplerPanel::new();
        panel.enable_compare = false;

        panel.update_descriptor();

        assert_eq!(panel.descriptor.compare(), None);
    }

    #[test]
    fn test_border_color_enabled() {
        let mut panel = SamplerPanel::new();
        panel.enable_border_color = true;
        panel.border_color = BorderColorChoice::OpaqueWhite;

        panel.update_descriptor();

        assert_eq!(
            panel.descriptor.border_color(),
            Some(wgpu::SamplerBorderColor::OpaqueWhite)
        );
    }

    #[test]
    fn test_border_color_disabled() {
        let mut panel = SamplerPanel::new();
        panel.enable_border_color = false;

        panel.update_descriptor();

        assert_eq!(panel.descriptor.border_color(), None);
    }

    #[test]
    fn test_empty_label() {
        let mut panel = SamplerPanel::new();
        panel.label_input = "".to_string();

        panel.update_descriptor();

        assert_eq!(panel.descriptor.label(), None);
    }

    #[test]
    fn test_invalid_lod_input() {
        let mut panel = SamplerPanel::new();
        panel.lod_min_input = "not_a_number".to_string();
        panel.lod_max_input = "also_not_a_number".to_string();

        panel.update_descriptor();

        // Should default to 0.0 and 32.0
        assert_eq!(panel.descriptor.lod_min_clamp(), 0.0);
        assert_eq!(panel.descriptor.lod_max_clamp(), 32.0);
    }

    #[test]
    fn test_all_address_modes() {
        let mut panel = SamplerPanel::new();

        panel.address_mode_u = AddressMode::Repeat;
        panel.address_mode_v = AddressMode::MirrorRepeat;
        panel.address_mode_w = AddressMode::ClampToEdge;

        panel.update_descriptor();

        assert_eq!(panel.descriptor.address_mode_u(), AddressMode::Repeat);
        assert_eq!(panel.descriptor.address_mode_v(), AddressMode::MirrorRepeat);
        assert_eq!(panel.descriptor.address_mode_w(), AddressMode::ClampToEdge);
    }

    #[test]
    fn test_all_filter_modes() {
        let mut panel = SamplerPanel::new();

        panel.mag_filter = FilterMode::Linear;
        panel.min_filter = FilterMode::Nearest;
        panel.mipmap_filter = MipmapFilterMode::Linear;

        panel.update_descriptor();

        assert_eq!(panel.descriptor.mag_filter(), FilterMode::Linear);
        assert_eq!(panel.descriptor.min_filter(), FilterMode::Nearest);
        assert_eq!(panel.descriptor.mipmap_filter(), MipmapFilterMode::Linear);
    }

    #[test]
    fn test_anisotropy_range() {
        let mut panel = SamplerPanel::new();

        panel.anisotropy = 8;
        panel.update_descriptor();
        assert_eq!(panel.descriptor.anisotropy_clamp(), 8);

        panel.anisotropy = 16;
        panel.update_descriptor();
        assert_eq!(panel.descriptor.anisotropy_clamp(), 16);

        panel.anisotropy = 1;
        panel.update_descriptor();
        assert_eq!(panel.descriptor.anisotropy_clamp(), 1);
    }

    #[test]
    fn test_border_color_choice_conversion() {
        assert_eq!(
            BorderColorChoice::TransparentBlack.to_wgpu(),
            wgpu::SamplerBorderColor::TransparentBlack
        );
        assert_eq!(
            BorderColorChoice::OpaqueBlack.to_wgpu(),
            wgpu::SamplerBorderColor::OpaqueBlack
        );
        assert_eq!(
            BorderColorChoice::OpaqueWhite.to_wgpu(),
            wgpu::SamplerBorderColor::OpaqueWhite
        );
        assert_eq!(
            BorderColorChoice::Zero.to_wgpu(),
            wgpu::SamplerBorderColor::Zero
        );
    }

    #[test]
    fn test_border_color_choice_names() {
        assert_eq!(
            BorderColorChoice::TransparentBlack.name(),
            "Transparent Black"
        );
        assert_eq!(BorderColorChoice::OpaqueBlack.name(), "Opaque Black");
        assert_eq!(BorderColorChoice::OpaqueWhite.name(), "Opaque White");
        assert_eq!(BorderColorChoice::Zero.name(), "Zero");
    }

    #[test]
    fn test_border_color_choice_all() {
        let all_colors = BorderColorChoice::all();
        assert_eq!(all_colors.len(), 4);
        assert!(all_colors.contains(&BorderColorChoice::TransparentBlack));
        assert!(all_colors.contains(&BorderColorChoice::OpaqueBlack));
        assert!(all_colors.contains(&BorderColorChoice::OpaqueWhite));
        assert!(all_colors.contains(&BorderColorChoice::Zero));
    }
}
