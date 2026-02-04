use wgpu::{TextureDimension, TextureFormat, TextureUsages};
use image::GenericImageView;

/// UI panel for creating and configuring GPU textures
pub struct TexturePanel {
    /// Label input text
    label_input: String,
    /// Width input text
    width_input: String,
    /// Height input text
    height_input: String,
    /// Depth or array layers input text
    depth_input: String,
    /// Mip level count input text
    mip_levels_input: String,
    /// Sample count input text
    sample_count_input: String,
    /// Selected texture format
    selected_format: TextureFormat,
    /// Selected texture dimension
    selected_dimension: TextureDimension,
    /// Usage flags state
    usage_copy_src: bool,
    usage_copy_dst: bool,
    usage_texture_binding: bool,
    usage_storage_binding: bool,
    usage_render_attachment: bool,
    /// Validation error message
    validation_error: Option<String>,
    /// Success message
    success_message: Option<String>,
    /// Loaded texture data (bytes)
    loaded_texture_data: Option<Vec<u8>>,
    /// Loaded texture dimensions
    loaded_texture_dimensions: Option<(u32, u32)>,
    /// File load message
    file_load_message: Option<String>,
}

impl Default for TexturePanel {
    fn default() -> Self {
        Self::new()
    }
}

impl TexturePanel {
    /// Create a new texture panel with default values
    pub fn new() -> Self {
        Self {
            label_input: String::new(),
            width_input: "256".to_string(),
            height_input: "256".to_string(),
            depth_input: "1".to_string(),
            mip_levels_input: "1".to_string(),
            sample_count_input: "1".to_string(),
            selected_format: TextureFormat::Rgba8Unorm,
            selected_dimension: TextureDimension::D2,
            usage_copy_src: false,
            usage_copy_dst: true,
            usage_texture_binding: true,
            usage_storage_binding: false,
            usage_render_attachment: false,
            validation_error: None,
            success_message: None,
            loaded_texture_data: None,
            loaded_texture_dimensions: None,
            file_load_message: None,
        }
    }

    /// Validate the current configuration
    fn validate(&mut self) -> bool {
        // Parse dimensions
        let width = match self.width_input.parse::<u32>() {
            Ok(w) if w > 0 => w,
            _ => {
                self.validation_error = Some("Width must be a positive number".to_string());
                self.success_message = None;
                return false;
            }
        };

        let height = match self.height_input.parse::<u32>() {
            Ok(h) if h > 0 => h,
            _ => {
                self.validation_error = Some("Height must be a positive number".to_string());
                self.success_message = None;
                return false;
            }
        };

        let depth = match self.depth_input.parse::<u32>() {
            Ok(d) if d > 0 => d,
            _ => {
                self.validation_error =
                    Some("Depth/array layers must be a positive number".to_string());
                self.success_message = None;
                return false;
            }
        };

        let mip_levels = match self.mip_levels_input.parse::<u32>() {
            Ok(m) if m > 0 => m,
            _ => {
                self.validation_error = Some("Mip levels must be a positive number".to_string());
                self.success_message = None;
                return false;
            }
        };

        let sample_count = match self.sample_count_input.parse::<u32>() {
            Ok(s) if [1, 2, 4, 8, 16, 32].contains(&s) => s,
            _ => {
                self.validation_error =
                    Some("Sample count must be 1, 2, 4, 8, 16, or 32".to_string());
                self.success_message = None;
                return false;
            }
        };

        // Build usage flags
        let usage = self.build_usage_flags();
        if usage.is_empty() {
            self.validation_error = Some("At least one usage flag must be selected".to_string());
            self.success_message = None;
            return false;
        }

        // Validate dimension constraints
        if self.selected_dimension == TextureDimension::D1 {
            if height != 1 {
                self.validation_error = Some("1D textures must have height = 1".to_string());
                self.success_message = None;
                return false;
            }
            if depth != 1 {
                self.validation_error =
                    Some("1D textures must have depth/array layers = 1".to_string());
                self.success_message = None;
                return false;
            }
        }

        // Validate mip levels
        let max_dimension = width.max(height);
        let max_mip_levels = (max_dimension as f32).log2().floor() as u32 + 1;
        if mip_levels > max_mip_levels {
            self.validation_error = Some(format!(
                "Mip levels ({}) exceeds maximum ({}) for {}x{} texture",
                mip_levels, max_mip_levels, width, height
            ));
            self.success_message = None;
            return false;
        }

        // Validate multisampling
        if sample_count > 1 {
            if mip_levels > 1 {
                self.validation_error =
                    Some("Multisampled textures cannot have mip levels > 1".to_string());
                self.success_message = None;
                return false;
            }
            if self.selected_dimension != TextureDimension::D2 {
                self.validation_error = Some("Only 2D textures can be multisampled".to_string());
                self.success_message = None;
                return false;
            }
        }

        self.validation_error = None;
        true
    }

    /// Build usage flags from current UI state
    fn build_usage_flags(&self) -> TextureUsages {
        let mut usage = TextureUsages::empty();
        if self.usage_copy_src {
            usage |= TextureUsages::COPY_SRC;
        }
        if self.usage_copy_dst {
            usage |= TextureUsages::COPY_DST;
        }
        if self.usage_texture_binding {
            usage |= TextureUsages::TEXTURE_BINDING;
        }
        if self.usage_storage_binding {
            usage |= TextureUsages::STORAGE_BINDING;
        }
        if self.usage_render_attachment {
            usage |= TextureUsages::RENDER_ATTACHMENT;
        }
        usage
    }

    /// Handle file loading from bytes
    pub fn load_from_bytes(&mut self, bytes: Vec<u8>) {
        // Try to decode the image to get dimensions
        match image::load_from_memory(&bytes) {
            Ok(img) => {
                let dimensions = img.dimensions();
                self.loaded_texture_data = Some(bytes);
                self.loaded_texture_dimensions = Some(dimensions);
                self.width_input = dimensions.0.to_string();
                self.height_input = dimensions.1.to_string();
                self.file_load_message = Some(format!(
                    "✓ Image loaded successfully: {}x{} pixels",
                    dimensions.0, dimensions.1
                ));
                self.validation_error = None;
            }
            Err(e) => {
                self.file_load_message = None;
                self.validation_error = Some(format!("Failed to load image: {}", e));
            }
        }
    }

    /// Clear loaded texture data
    pub fn clear_loaded_texture(&mut self) {
        self.loaded_texture_data = None;
        self.loaded_texture_dimensions = None;
        self.file_load_message = None;
    }

    /// Get loaded texture data
    pub fn get_loaded_texture_data(&self) -> Option<&Vec<u8>> {
        self.loaded_texture_data.as_ref()
    }

    /// Get loaded texture dimensions
    pub fn get_loaded_texture_dimensions(&self) -> Option<(u32, u32)> {
        self.loaded_texture_dimensions
    }

    /// Render the texture configuration UI
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("🖼️ Texture Configuration");
            ui.label("Configure and create GPU textures with custom parameters.");
            ui.add_space(10.0);

            // Texture Properties
            ui.group(|ui| {
                ui.heading("Texture Properties");
                ui.add_space(5.0);

                egui::Grid::new("texture_properties")
                    .num_columns(2)
                    .spacing([10.0, 8.0])
                    .show(ui, |ui| {
                        ui.label("Label:");
                        ui.text_edit_singleline(&mut self.label_input);
                        ui.end_row();

                        ui.label("Width:");
                        ui.text_edit_singleline(&mut self.width_input);
                        ui.end_row();

                        ui.label("Height:");
                        ui.text_edit_singleline(&mut self.height_input);
                        ui.end_row();

                        ui.label("Depth/Array Layers:");
                        ui.text_edit_singleline(&mut self.depth_input);
                        ui.end_row();

                        ui.label("Mip Levels:");
                        ui.text_edit_singleline(&mut self.mip_levels_input);
                        ui.end_row();

                        ui.label("Sample Count:");
                        ui.text_edit_singleline(&mut self.sample_count_input);
                        ui.end_row();
                    });
            });

            ui.add_space(10.0);

            // Texture Dimension
            ui.group(|ui| {
                ui.heading("Texture Dimension");
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.radio_value(&mut self.selected_dimension, TextureDimension::D1, "1D");
                    ui.radio_value(&mut self.selected_dimension, TextureDimension::D2, "2D");
                    ui.radio_value(&mut self.selected_dimension, TextureDimension::D3, "3D");
                });
            });

            ui.add_space(10.0);

            // Texture Format
            ui.group(|ui| {
                ui.heading("Texture Format");
                ui.add_space(5.0);

                egui::ComboBox::from_label("Format")
                    .selected_text(format!("{:?}", self.selected_format))
                    .show_ui(ui, |ui| {
                        ui.label("Color Formats");
                        ui.separator();
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Rgba8Unorm);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Rgba8UnormSrgb);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Bgra8Unorm);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Bgra8UnormSrgb);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Rgba16Float);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Rgba32Float);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Rgb10a2Unorm);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::R8Unorm);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::R8Snorm);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::R8Uint);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::R8Sint);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::R16Uint);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::R16Sint);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::R16Float);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Rg8Unorm);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Rg8Snorm);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Rg8Uint);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Rg8Sint);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Rg16Uint);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Rg16Sint);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Rg16Float);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Rgba16Uint);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Rgba16Sint);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Rgba32Uint);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Rgba32Sint);

                        ui.add_space(5.0);
                        ui.label("Depth/Stencil Formats");
                        ui.separator();
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Depth32Float);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Depth24Plus);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Depth24PlusStencil8);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Stencil8);

                        ui.add_space(5.0);
                        ui.label("Compressed Formats (BC)");
                        ui.separator();
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Bc1RgbaUnorm);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Bc1RgbaUnormSrgb);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Bc2RgbaUnorm);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Bc2RgbaUnormSrgb);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Bc3RgbaUnorm);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Bc3RgbaUnormSrgb);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Bc4RUnorm);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Bc4RSnorm);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Bc5RgUnorm);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Bc5RgSnorm);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Bc6hRgbUfloat);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Bc6hRgbFloat);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Bc7RgbaUnorm);
                        Self::format_option(ui, &mut self.selected_format, TextureFormat::Bc7RgbaUnormSrgb);
                    });
            });

            ui.add_space(10.0);

            // Usage Flags
            ui.group(|ui| {
                ui.heading("Usage Flags");
                ui.label("Select how the texture will be used (multiple flags can be selected):");
                ui.add_space(5.0);

                egui::Grid::new("usage_flags")
                    .num_columns(2)
                    .spacing([10.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        Self::render_usage_checkbox(
                            ui,
                            "COPY_SRC",
                            &mut self.usage_copy_src,
                            "Texture can be used as a copy source",
                        );
                        Self::render_usage_checkbox(
                            ui,
                            "COPY_DST",
                            &mut self.usage_copy_dst,
                            "Texture can be used as a copy destination",
                        );
                        Self::render_usage_checkbox(
                            ui,
                            "TEXTURE_BINDING",
                            &mut self.usage_texture_binding,
                            "Texture can be bound in a shader",
                        );
                        Self::render_usage_checkbox(
                            ui,
                            "STORAGE_BINDING",
                            &mut self.usage_storage_binding,
                            "Texture can be used as a storage texture",
                        );
                        Self::render_usage_checkbox(
                            ui,
                            "RENDER_ATTACHMENT",
                            &mut self.usage_render_attachment,
                            "Texture can be used as a render attachment",
                        );
                    });
            });

            ui.add_space(15.0);

            // File Loading Section
            ui.group(|ui| {
                ui.heading("📁 Load Texture from File");
                ui.label("Load image files (PNG, JPEG) to create textures.");
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        if ui.button("📂 Load Image...").clicked() {
                            // For native platforms, we'll show a message about drag-and-drop
                            self.file_load_message = Some("Drag and drop an image file onto this window, or use file system access in your application.".to_string());
                        }
                    }

                    #[cfg(target_arch = "wasm32")]
                    {
                        if ui.button("📂 Load Image...").clicked() {
                            self.file_load_message = Some("File loading in browser - use drag and drop or file upload in your application.".to_string());
                        }
                    }

                    if self.loaded_texture_data.is_some() {
                        if ui.button("🗑️ Clear Loaded Image").clicked() {
                            self.clear_loaded_texture();
                        }
                    }
                });

                ui.add_space(5.0);

                // Display file load message or loaded texture info
                if let Some(msg) = &self.file_load_message {
                    ui.colored_label(egui::Color32::GREEN, msg);
                }

                if let Some((width, height)) = self.loaded_texture_dimensions {
                    ui.label(format!("📐 Loaded image: {} x {} pixels", width, height));
                    ui.label("Image dimensions have been applied to Width and Height fields.");
                }

                ui.add_space(5.0);
                ui.label("💡 Tip: Drag and drop image files onto the application window to load them.");
            });

            ui.add_space(15.0);

            // Validation and Creation
            ui.horizontal(|ui| {
                if ui.button("🔍 Validate").clicked() && self.validate() {
                    self.success_message = Some("✓ Configuration is valid".to_string());
                }

                if ui.button("✨ Create Texture").clicked() && self.validate() {
                    self.success_message = Some(
                        "✓ Configuration is valid. In a full implementation, the texture would be created here."
                            .to_string(),
                    );
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

                ui.monospace(format!(
                    "Label: {}",
                    if self.label_input.is_empty() {
                        "<none>"
                    } else {
                        &self.label_input
                    }
                ));
                ui.monospace(format!("Dimension: {:?}", self.selected_dimension));
                ui.monospace(format!(
                    "Size: {}x{}x{}",
                    self.width_input, self.height_input, self.depth_input
                ));
                ui.monospace(format!("Format: {:?}", self.selected_format));
                ui.monospace(format!("Mip Levels: {}", self.mip_levels_input));
                ui.monospace(format!("Sample Count: {}", self.sample_count_input));

                ui.add_space(5.0);
                ui.label("Usage flags:");
                let usage = self.build_usage_flags();
                if usage.is_empty() {
                    ui.monospace("  (none)");
                } else {
                    if usage.contains(TextureUsages::COPY_SRC) {
                        ui.monospace("  • COPY_SRC");
                    }
                    if usage.contains(TextureUsages::COPY_DST) {
                        ui.monospace("  • COPY_DST");
                    }
                    if usage.contains(TextureUsages::TEXTURE_BINDING) {
                        ui.monospace("  • TEXTURE_BINDING");
                    }
                    if usage.contains(TextureUsages::STORAGE_BINDING) {
                        ui.monospace("  • STORAGE_BINDING");
                    }
                    if usage.contains(TextureUsages::RENDER_ATTACHMENT) {
                        ui.monospace("  • RENDER_ATTACHMENT");
                    }
                }
            });
        });
    }

    fn format_option(ui: &mut egui::Ui, current: &mut TextureFormat, format: TextureFormat) {
        ui.selectable_value(current, format, format!("{:?}", format));
    }

    fn render_usage_checkbox(ui: &mut egui::Ui, label: &str, value: &mut bool, tooltip: &str) {
        ui.checkbox(value, label).on_hover_text(tooltip);
        ui.end_row();
    }

    /// Export the current state to a serializable format
    pub fn export_state(&self) -> crate::state::TexturePanelState {
        crate::state::TexturePanelState {
            label: self.label_input.clone(),
            width: self.width_input.clone(),
            height: self.height_input.clone(),
            depth: self.depth_input.clone(),
            mip_levels: self.mip_levels_input.clone(),
            sample_count: self.sample_count_input.clone(),
            format: format!("{:?}", self.selected_format),
            dimension: format!("{:?}", self.selected_dimension),
            usage_copy_src: self.usage_copy_src,
            usage_copy_dst: self.usage_copy_dst,
            usage_texture_binding: self.usage_texture_binding,
            usage_storage_binding: self.usage_storage_binding,
            usage_render_attachment: self.usage_render_attachment,
        }
    }

    /// Import state from a serializable format
    ///
    /// Note: Format and dimension enum values are stored as strings but are not parsed back
    /// to avoid complexity. The panel will retain default values for these fields.
    /// Future enhancement could add enum parsing support.
    pub fn import_state(&mut self, state: &crate::state::TexturePanelState) {
        self.label_input = state.label.clone();
        self.width_input = state.width.clone();
        self.height_input = state.height.clone();
        self.depth_input = state.depth.clone();
        self.mip_levels_input = state.mip_levels.clone();
        self.sample_count_input = state.sample_count.clone();
        self.usage_copy_src = state.usage_copy_src;
        self.usage_copy_dst = state.usage_copy_dst;
        self.usage_texture_binding = state.usage_texture_binding;
        self.usage_storage_binding = state.usage_storage_binding;
        self.usage_render_attachment = state.usage_render_attachment;

        // TODO: Parse format and dimension from strings
        // For now, these remain at their default values
        // The string values are preserved in the saved state for reference

        self.validation_error = None;
        self.success_message = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_panel_creation() {
        let panel = TexturePanel::new();
        assert_eq!(panel.width_input, "256");
        assert_eq!(panel.height_input, "256");
        assert_eq!(panel.depth_input, "1");
        assert_eq!(panel.mip_levels_input, "1");
        assert_eq!(panel.sample_count_input, "1");
        assert_eq!(panel.label_input, "");
        assert_eq!(panel.selected_format, TextureFormat::Rgba8Unorm);
        assert_eq!(panel.selected_dimension, TextureDimension::D2);
        assert!(panel.usage_copy_dst);
        assert!(panel.usage_texture_binding);
        assert!(!panel.usage_copy_src);
    }

    #[test]
    fn test_texture_panel_default() {
        let panel = TexturePanel::default();
        assert_eq!(panel.width_input, "256");
        assert_eq!(panel.selected_format, TextureFormat::Rgba8Unorm);
    }

    #[test]
    fn test_validate_success() {
        let mut panel = TexturePanel::new();
        panel.width_input = "512".to_string();
        panel.height_input = "512".to_string();
        panel.usage_texture_binding = true;

        let valid = panel.validate();
        assert!(valid);
        assert!(panel.validation_error.is_none());
    }

    #[test]
    fn test_validate_zero_width() {
        let mut panel = TexturePanel::new();
        panel.width_input = "0".to_string();

        let valid = panel.validate();
        assert!(!valid);
        assert!(panel.validation_error.is_some());
        assert!(panel
            .validation_error
            .as_ref()
            .unwrap()
            .contains("positive number"));
    }

    #[test]
    fn test_validate_invalid_sample_count() {
        let mut panel = TexturePanel::new();
        panel.sample_count_input = "3".to_string();

        let valid = panel.validate();
        assert!(!valid);
        assert!(panel.validation_error.is_some());
        assert!(panel
            .validation_error
            .as_ref()
            .unwrap()
            .contains("1, 2, 4, 8, 16, or 32"));
    }

    #[test]
    fn test_validate_no_usage() {
        let mut panel = TexturePanel::new();
        panel.usage_copy_dst = false;
        panel.usage_texture_binding = false;

        let valid = panel.validate();
        assert!(!valid);
        assert!(panel.validation_error.is_some());
        assert!(panel
            .validation_error
            .as_ref()
            .unwrap()
            .contains("At least one usage flag"));
    }

    #[test]
    fn test_validate_1d_dimension_constraints() {
        let mut panel = TexturePanel::new();
        panel.selected_dimension = TextureDimension::D1;
        panel.height_input = "2".to_string();

        let valid = panel.validate();
        assert!(!valid);
        assert!(panel.validation_error.is_some());
        assert!(panel
            .validation_error
            .as_ref()
            .unwrap()
            .contains("1D textures must have height = 1"));
    }

    #[test]
    fn test_validate_mip_levels_exceed_max() {
        let mut panel = TexturePanel::new();
        panel.width_input = "256".to_string();
        panel.height_input = "256".to_string();
        panel.mip_levels_input = "20".to_string(); // Max for 256x256 is 9

        let valid = panel.validate();
        assert!(!valid);
        assert!(panel.validation_error.is_some());
        assert!(panel
            .validation_error
            .as_ref()
            .unwrap()
            .contains("exceeds maximum"));
    }

    #[test]
    fn test_validate_multisampling_with_mips() {
        let mut panel = TexturePanel::new();
        panel.sample_count_input = "4".to_string();
        panel.mip_levels_input = "2".to_string();

        let valid = panel.validate();
        assert!(!valid);
        assert!(panel.validation_error.is_some());
        assert!(panel
            .validation_error
            .as_ref()
            .unwrap()
            .contains("Multisampled textures cannot have mip levels > 1"));
    }

    #[test]
    fn test_validate_multisampling_non_2d() {
        let mut panel = TexturePanel::new();
        panel.selected_dimension = TextureDimension::D3;
        panel.sample_count_input = "4".to_string();

        let valid = panel.validate();
        assert!(!valid);
        assert!(panel.validation_error.is_some());
        assert!(panel
            .validation_error
            .as_ref()
            .unwrap()
            .contains("Only 2D textures can be multisampled"));
    }

    #[test]
    fn test_build_usage_flags() {
        let mut panel = TexturePanel::new();
        panel.usage_copy_src = true;
        panel.usage_copy_dst = true;
        panel.usage_texture_binding = true;
        panel.usage_storage_binding = true;
        panel.usage_render_attachment = true;

        let usage = panel.build_usage_flags();
        assert!(usage.contains(TextureUsages::COPY_SRC));
        assert!(usage.contains(TextureUsages::COPY_DST));
        assert!(usage.contains(TextureUsages::TEXTURE_BINDING));
        assert!(usage.contains(TextureUsages::STORAGE_BINDING));
        assert!(usage.contains(TextureUsages::RENDER_ATTACHMENT));
    }

    #[test]
    fn test_format_selection() {
        let mut panel = TexturePanel::new();
        panel.selected_format = TextureFormat::Rgba16Float;
        assert_eq!(panel.selected_format, TextureFormat::Rgba16Float);

        panel.selected_format = TextureFormat::Depth32Float;
        assert_eq!(panel.selected_format, TextureFormat::Depth32Float);

        panel.selected_format = TextureFormat::Bc1RgbaUnorm;
        assert_eq!(panel.selected_format, TextureFormat::Bc1RgbaUnorm);
    }

    #[test]
    fn test_dimension_selection() {
        let mut panel = TexturePanel::new();
        panel.selected_dimension = TextureDimension::D1;
        assert_eq!(panel.selected_dimension, TextureDimension::D1);

        panel.selected_dimension = TextureDimension::D3;
        assert_eq!(panel.selected_dimension, TextureDimension::D3);
    }
}
