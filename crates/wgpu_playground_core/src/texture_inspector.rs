use wgpu::TextureFormat;

/// Represents texture data that can be displayed
#[derive(Debug, Clone)]
pub struct TextureData {
    /// Width of the texture
    pub width: u32,
    /// Height of the texture
    pub height: u32,
    /// Texture format
    pub format: TextureFormat,
    /// Raw pixel data (RGBA8)
    pub data: Vec<u8>,
}

/// Inspector for visualizing GPU textures
///
/// This utility allows viewing texture contents by reading data from
/// GPU textures and displaying them as images in the UI.
pub struct TextureInspector {
    /// Currently loaded texture data
    texture_data: Option<TextureData>,
    /// Mip level to display (0 is the base level)
    selected_mip_level: u32,
    /// Array layer to display (for texture arrays)
    selected_array_layer: u32,
    /// Whether to show alpha channel
    show_alpha: bool,
    /// Zoom level for the texture display
    zoom_level: f32,
    /// Whether data is currently being loaded
    is_loading: bool,
    /// Error message if loading failed
    error_message: Option<String>,
}

impl Default for TextureInspector {
    fn default() -> Self {
        Self::new()
    }
}

impl TextureInspector {
    /// Create a new texture inspector
    pub fn new() -> Self {
        Self {
            texture_data: None,
            selected_mip_level: 0,
            selected_array_layer: 0,
            show_alpha: true,
            zoom_level: 1.0,
            is_loading: false,
            error_message: None,
        }
    }

    /// Get the currently loaded texture data
    pub fn texture_data(&self) -> Option<&TextureData> {
        self.texture_data.as_ref()
    }

    /// Clear the loaded texture
    pub fn clear(&mut self) {
        self.texture_data = None;
        self.error_message = None;
        self.selected_mip_level = 0;
        self.selected_array_layer = 0;
    }

    /// Load texture data
    ///
    /// # Arguments
    /// * `data` - The texture data to load
    pub fn load_texture(&mut self, data: TextureData) {
        self.texture_data = Some(data);
        self.error_message = None;
        self.is_loading = false;
        self.selected_mip_level = 0;
        self.selected_array_layer = 0;
    }

    /// Set an error message
    pub fn set_error(&mut self, error: String) {
        self.error_message = Some(error);
        self.is_loading = false;
    }

    /// Check if there's an error
    pub fn has_error(&self) -> bool {
        self.error_message.is_some()
    }

    /// Get the error message if any
    pub fn error_message(&self) -> Option<&str> {
        self.error_message.as_deref()
    }

    /// Set the mip level to display
    pub fn set_mip_level(&mut self, level: u32) {
        self.selected_mip_level = level;
    }

    /// Get the current mip level
    pub fn get_mip_level(&self) -> u32 {
        self.selected_mip_level
    }

    /// Set the array layer to display
    pub fn set_array_layer(&mut self, layer: u32) {
        self.selected_array_layer = layer;
    }

    /// Set zoom level
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom_level = zoom.max(0.1).min(10.0);
    }

    /// Get zoom level
    pub fn get_zoom(&self) -> f32 {
        self.zoom_level
    }

    /// Convert texture data to egui color image
    fn create_color_image(&self, data: &TextureData) -> egui::ColorImage {
        let width = data.width as usize;
        let height = data.height as usize;
        
        let mut pixels = Vec::with_capacity(width * height);
        
        // Convert data to RGBA format
        match data.format {
            TextureFormat::Rgba8Unorm | TextureFormat::Rgba8UnormSrgb => {
                for chunk in data.data.chunks(4) {
                    if chunk.len() == 4 {
                        if self.show_alpha {
                            pixels.push(egui::Color32::from_rgba_premultiplied(
                                chunk[0], chunk[1], chunk[2], chunk[3]
                            ));
                        } else {
                            pixels.push(egui::Color32::from_rgb(chunk[0], chunk[1], chunk[2]));
                        }
                    }
                }
            }
            TextureFormat::Bgra8Unorm | TextureFormat::Bgra8UnormSrgb => {
                for chunk in data.data.chunks(4) {
                    if chunk.len() == 4 {
                        if self.show_alpha {
                            pixels.push(egui::Color32::from_rgba_premultiplied(
                                chunk[2], chunk[1], chunk[0], chunk[3]
                            ));
                        } else {
                            pixels.push(egui::Color32::from_rgb(chunk[2], chunk[1], chunk[0]));
                        }
                    }
                }
            }
            _ => {
                // For unsupported formats, show a placeholder pattern
                for y in 0..height {
                    for x in 0..width {
                        let checker = ((x / 8) + (y / 8)) % 2 == 0;
                        pixels.push(if checker {
                            egui::Color32::GRAY
                        } else {
                            egui::Color32::DARK_GRAY
                        });
                    }
                }
            }
        }
        
        // Ensure we have the right number of pixels
        while pixels.len() < width * height {
            pixels.push(egui::Color32::BLACK);
        }
        
        egui::ColorImage {
            size: [width, height],
            source_size: egui::vec2(width as f32, height as f32),
            pixels,
        }
    }

    /// Render the texture inspector UI
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Texture Inspector");
        ui.separator();

        // Controls
        ui.horizontal(|ui| {
            ui.label("Zoom:");
            if ui.button("-").clicked() {
                self.zoom_level = (self.zoom_level * 0.8).max(0.1);
            }
            ui.label(format!("{:.1}x", self.zoom_level));
            if ui.button("+").clicked() {
                self.zoom_level = (self.zoom_level * 1.25).min(10.0);
            }
            
            ui.separator();
            
            ui.checkbox(&mut self.show_alpha, "Show Alpha");
        });

        ui.separator();

        // Display error if any
        if let Some(error) = &self.error_message {
            ui.colored_label(egui::Color32::RED, format!("Error: {}", error));
            ui.separator();
        }

        // Display texture
        if self.is_loading {
            ui.spinner();
            ui.label("Loading texture data...");
        } else if let Some(texture_data) = &self.texture_data {
            // Display texture info
            ui.horizontal(|ui| {
                ui.label(format!("Size: {}x{}", texture_data.width, texture_data.height));
                ui.separator();
                ui.label(format!("Format: {:?}", texture_data.format));
            });

            ui.separator();

            // Create and display the texture
            let color_image = self.create_color_image(texture_data);
            let texture_handle = ui.ctx().load_texture(
                "texture_preview",
                color_image,
                egui::TextureOptions::default()
            );

            let display_width = texture_data.width as f32 * self.zoom_level;
            let display_height = texture_data.height as f32 * self.zoom_level;

            egui::ScrollArea::both()
                .max_width(ui.available_width())
                .max_height(500.0)
                .show(ui, |ui| {
                    ui.add(
                        egui::Image::new(&texture_handle)
                            .fit_to_exact_size(egui::vec2(display_width, display_height))
                    );
                });
        } else if self.error_message.is_none() {
            ui.label("Select a texture from the Resource Inspector to visualize it");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_texture() -> TextureData {
        // Create a simple 2x2 RGBA texture
        TextureData {
            width: 2,
            height: 2,
            format: TextureFormat::Rgba8Unorm,
            data: vec![
                255, 0, 0, 255,    // Red
                0, 255, 0, 255,    // Green
                0, 0, 255, 255,    // Blue
                255, 255, 0, 255,  // Yellow
            ],
        }
    }

    #[test]
    fn test_texture_inspector_creation() {
        let inspector = TextureInspector::new();
        assert!(inspector.texture_data().is_none());
        assert!(!inspector.has_error());
    }

    #[test]
    fn test_load_texture() {
        let mut inspector = TextureInspector::new();
        let texture = create_test_texture();
        inspector.load_texture(texture);
        assert!(inspector.texture_data().is_some());
        assert!(!inspector.has_error());
    }

    #[test]
    fn test_texture_dimensions() {
        let mut inspector = TextureInspector::new();
        inspector.load_texture(create_test_texture());
        let data = inspector.texture_data().unwrap();
        assert_eq!(data.width, 2);
        assert_eq!(data.height, 2);
    }

    #[test]
    fn test_clear() {
        let mut inspector = TextureInspector::new();
        inspector.load_texture(create_test_texture());
        assert!(inspector.texture_data().is_some());
        inspector.clear();
        assert!(inspector.texture_data().is_none());
    }

    #[test]
    fn test_set_error() {
        let mut inspector = TextureInspector::new();
        inspector.set_error("Test error".to_string());
        assert!(inspector.has_error());
        assert_eq!(inspector.error_message(), Some("Test error"));
    }

    #[test]
    fn test_zoom_level() {
        let mut inspector = TextureInspector::new();
        inspector.set_zoom(2.0);
        assert_eq!(inspector.zoom_level, 2.0);
        
        // Test clamping
        inspector.set_zoom(100.0);
        assert_eq!(inspector.zoom_level, 10.0);
        
        inspector.set_zoom(0.01);
        assert_eq!(inspector.zoom_level, 0.1);
    }

    #[test]
    fn test_mip_level() {
        let mut inspector = TextureInspector::new();
        inspector.set_mip_level(2);
        assert_eq!(inspector.selected_mip_level, 2);
    }

    #[test]
    fn test_array_layer() {
        let mut inspector = TextureInspector::new();
        inspector.set_array_layer(3);
        assert_eq!(inspector.selected_array_layer, 3);
    }

    #[test]
    fn test_create_color_image() {
        let inspector = TextureInspector::new();
        let texture = create_test_texture();
        let color_image = inspector.create_color_image(&texture);
        assert_eq!(color_image.size, [2, 2]);
        assert_eq!(color_image.pixels.len(), 4);
    }
}
