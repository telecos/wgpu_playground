use crate::render_pass_encoder::{Color, LoadOp, StoreOp};

/// UI panel for configuring render passes
pub struct RenderPassPanel {
    /// Label input text
    label_input: String,
    
    // Color attachment settings
    /// Enable color attachment
    enable_color_attachment: bool,
    /// Color attachment load operation
    color_load_op: LoadOpChoice,
    /// Clear color for color attachment
    clear_color_r: f32,
    clear_color_g: f32,
    clear_color_b: f32,
    clear_color_a: f32,
    /// Color attachment store operation
    color_store_op: StoreOpChoice,
    
    // Depth-stencil attachment settings
    /// Enable depth-stencil attachment
    enable_depth_stencil: bool,
    /// Depth load operation
    depth_load_op: LoadOpChoice,
    /// Clear depth value
    clear_depth: f32,
    /// Depth store operation
    depth_store_op: StoreOpChoice,
    /// Stencil load operation
    stencil_load_op: LoadOpChoice,
    /// Clear stencil value
    clear_stencil: u32,
    /// Stencil store operation
    stencil_store_op: StoreOpChoice,
    
    // Timestamp writes settings
    /// Enable timestamp writes
    enable_timestamp_writes: bool,
    /// Beginning of pass index input
    timestamp_beginning_index_input: String,
    /// End of pass index input
    timestamp_end_index_input: String,
    
    /// Validation error message
    validation_error: Option<String>,
    /// Success message
    success_message: Option<String>,
}

/// Load operation choice for UI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LoadOpChoice {
    Clear,
    Load,
}

impl LoadOpChoice {
    fn all() -> Vec<Self> {
        vec![LoadOpChoice::Clear, LoadOpChoice::Load]
    }
    
    fn name(&self) -> &'static str {
        match self {
            LoadOpChoice::Clear => "Clear",
            LoadOpChoice::Load => "Load",
        }
    }
}

/// Store operation choice for UI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StoreOpChoice {
    Store,
    Discard,
}

impl StoreOpChoice {
    fn all() -> Vec<Self> {
        vec![StoreOpChoice::Store, StoreOpChoice::Discard]
    }
    
    fn name(&self) -> &'static str {
        match self {
            StoreOpChoice::Store => "Store",
            StoreOpChoice::Discard => "Discard",
        }
    }
    
    fn to_store_op(&self) -> StoreOp {
        match self {
            StoreOpChoice::Store => StoreOp::Store,
            StoreOpChoice::Discard => StoreOp::Discard,
        }
    }
}

impl Default for RenderPassPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderPassPanel {
    /// Create a new render pass panel with default values
    pub fn new() -> Self {
        Self {
            label_input: String::new(),
            enable_color_attachment: true,
            color_load_op: LoadOpChoice::Clear,
            clear_color_r: 0.0,
            clear_color_g: 0.0,
            clear_color_b: 0.0,
            clear_color_a: 1.0,
            color_store_op: StoreOpChoice::Store,
            enable_depth_stencil: false,
            depth_load_op: LoadOpChoice::Clear,
            clear_depth: 1.0,
            depth_store_op: StoreOpChoice::Store,
            stencil_load_op: LoadOpChoice::Clear,
            clear_stencil: 0,
            stencil_store_op: StoreOpChoice::Store,
            enable_timestamp_writes: false,
            timestamp_beginning_index_input: "0".to_string(),
            timestamp_end_index_input: "1".to_string(),
            validation_error: None,
            success_message: None,
        }
    }
    
    /// Get the configured color clear value
    pub fn get_color_clear(&self) -> Color {
        Color::new(
            self.clear_color_r as f64,
            self.clear_color_g as f64,
            self.clear_color_b as f64,
            self.clear_color_a as f64,
        )
    }
    
    /// Get the color load operation
    pub fn get_color_load_op(&self) -> LoadOp<Color> {
        match self.color_load_op {
            LoadOpChoice::Clear => LoadOp::Clear(self.get_color_clear()),
            LoadOpChoice::Load => LoadOp::Load,
        }
    }
    
    /// Get the color store operation
    pub fn get_color_store_op(&self) -> StoreOp {
        self.color_store_op.to_store_op()
    }
    
    /// Get the depth load operation
    pub fn get_depth_load_op(&self) -> LoadOp<f32> {
        match self.depth_load_op {
            LoadOpChoice::Clear => LoadOp::Clear(self.clear_depth),
            LoadOpChoice::Load => LoadOp::Load,
        }
    }
    
    /// Get the depth store operation
    pub fn get_depth_store_op(&self) -> StoreOp {
        self.depth_store_op.to_store_op()
    }
    
    /// Get the stencil load operation
    pub fn get_stencil_load_op(&self) -> LoadOp<u32> {
        match self.stencil_load_op {
            LoadOpChoice::Clear => LoadOp::Clear(self.clear_stencil),
            LoadOpChoice::Load => LoadOp::Load,
        }
    }
    
    /// Get the stencil store operation
    pub fn get_stencil_store_op(&self) -> StoreOp {
        self.stencil_store_op.to_store_op()
    }
    
    /// Validate the current configuration
    fn validate(&mut self) -> bool {
        // Check that at least one attachment is enabled
        if !self.enable_color_attachment && !self.enable_depth_stencil {
            self.validation_error = Some(
                "At least one attachment (color or depth-stencil) must be enabled".to_string()
            );
            self.success_message = None;
            return false;
        }
        
        // Validate clear color values (0.0 to 1.0)
        if self.color_load_op == LoadOpChoice::Clear {
            if self.clear_color_r < 0.0 || self.clear_color_r > 1.0
                || self.clear_color_g < 0.0 || self.clear_color_g > 1.0
                || self.clear_color_b < 0.0 || self.clear_color_b > 1.0
                || self.clear_color_a < 0.0 || self.clear_color_a > 1.0
            {
                self.validation_error = Some(
                    "Clear color values must be between 0.0 and 1.0".to_string()
                );
                self.success_message = None;
                return false;
            }
        }
        
        // Validate depth clear value (0.0 to 1.0)
        if self.enable_depth_stencil && self.depth_load_op == LoadOpChoice::Clear {
            if self.clear_depth < 0.0 || self.clear_depth > 1.0 {
                self.validation_error = Some(
                    "Clear depth value must be between 0.0 and 1.0".to_string()
                );
                self.success_message = None;
                return false;
            }
        }
        
        // Validate timestamp indices if enabled
        if self.enable_timestamp_writes {
            let beginning_index = self.timestamp_beginning_index_input.parse::<u32>();
            let end_index = self.timestamp_end_index_input.parse::<u32>();
            
            if beginning_index.is_err() {
                self.validation_error = Some(
                    "Timestamp beginning index must be a valid number".to_string()
                );
                self.success_message = None;
                return false;
            }
            
            if end_index.is_err() {
                self.validation_error = Some(
                    "Timestamp end index must be a valid number".to_string()
                );
                self.success_message = None;
                return false;
            }
        }
        
        self.validation_error = None;
        true
    }
    
    /// Render a load operation combo box
    fn render_load_op_combo(ui: &mut egui::Ui, load_op: &mut LoadOpChoice, id: &str) {
        egui::ComboBox::from_id_salt(id)
            .selected_text(load_op.name())
            .show_ui(ui, |ui| {
                for op in LoadOpChoice::all() {
                    ui.selectable_value(load_op, op, op.name());
                }
            });
    }
    
    /// Render a store operation combo box
    fn render_store_op_combo(ui: &mut egui::Ui, store_op: &mut StoreOpChoice, id: &str) {
        egui::ComboBox::from_id_salt(id)
            .selected_text(store_op.name())
            .show_ui(ui, |ui| {
                for op in StoreOpChoice::all() {
                    ui.selectable_value(store_op, op, op.name());
                }
            });
    }
    
    /// Render the render pass configuration UI
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("🎬 Render Pass Configuration");
            ui.label("Configure render pass with color attachments, depth-stencil, and timestamp writes.");
            ui.add_space(10.0);
            
            // Render Pass Label
            ui.group(|ui| {
                ui.heading("Render Pass Properties");
                ui.add_space(5.0);
                
                egui::Grid::new("render_pass_properties")
                    .num_columns(2)
                    .spacing([10.0, 8.0])
                    .show(ui, |ui| {
                        ui.label("Label:");
                        ui.text_edit_singleline(&mut self.label_input);
                        ui.end_row();
                    });
            });
            
            ui.add_space(10.0);
            
            // Color Attachment Configuration
            ui.group(|ui| {
                ui.heading("🎨 Color Attachment");
                ui.add_space(5.0);
                
                ui.checkbox(&mut self.enable_color_attachment, "Enable color attachment");
                
                if self.enable_color_attachment {
                    ui.add_space(5.0);
                    
                    egui::Grid::new("color_attachment")
                        .num_columns(2)
                        .spacing([10.0, 8.0])
                        .show(ui, |ui| {
                            ui.label("Load Operation:")
                                .on_hover_text("What to do at the start of the render pass");
                            Self::render_load_op_combo(ui, &mut self.color_load_op, "color_load");
                            ui.end_row();
                            
                            if self.color_load_op == LoadOpChoice::Clear {
                                ui.label("Clear Color:");
                                ui.vertical(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.label("R:");
                                        ui.add(egui::Slider::new(&mut self.clear_color_r, 0.0..=1.0)
                                            .fixed_decimals(2));
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label("G:");
                                        ui.add(egui::Slider::new(&mut self.clear_color_g, 0.0..=1.0)
                                            .fixed_decimals(2));
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label("B:");
                                        ui.add(egui::Slider::new(&mut self.clear_color_b, 0.0..=1.0)
                                            .fixed_decimals(2));
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label("A:");
                                        ui.add(egui::Slider::new(&mut self.clear_color_a, 0.0..=1.0)
                                            .fixed_decimals(2));
                                    });
                                    
                                    // Color preview
                                    ui.add_space(5.0);
                                    ui.horizontal(|ui| {
                                        ui.label("Preview:");
                                        let color = egui::Color32::from_rgba_unmultiplied(
                                            (self.clear_color_r * 255.0) as u8,
                                            (self.clear_color_g * 255.0) as u8,
                                            (self.clear_color_b * 255.0) as u8,
                                            (self.clear_color_a * 255.0) as u8,
                                        );
                                        ui.colored_label(color, "████████");
                                    });
                                });
                                ui.end_row();
                            }
                            
                            ui.label("Store Operation:")
                                .on_hover_text("What to do at the end of the render pass");
                            Self::render_store_op_combo(ui, &mut self.color_store_op, "color_store");
                            ui.end_row();
                        });
                }
            });
            
            ui.add_space(10.0);
            
            // Depth-Stencil Attachment Configuration
            ui.group(|ui| {
                ui.heading("📏 Depth-Stencil Attachment");
                ui.add_space(5.0);
                
                ui.checkbox(&mut self.enable_depth_stencil, "Enable depth-stencil attachment");
                
                if self.enable_depth_stencil {
                    ui.add_space(5.0);
                    
                    ui.label("Depth Configuration:");
                    egui::Grid::new("depth_config")
                        .num_columns(2)
                        .spacing([10.0, 8.0])
                        .show(ui, |ui| {
                            ui.label("Load Operation:");
                            Self::render_load_op_combo(ui, &mut self.depth_load_op, "depth_load");
                            ui.end_row();
                            
                            if self.depth_load_op == LoadOpChoice::Clear {
                                ui.label("Clear Depth:");
                                ui.add(egui::Slider::new(&mut self.clear_depth, 0.0..=1.0)
                                    .fixed_decimals(2));
                                ui.end_row();
                            }
                            
                            ui.label("Store Operation:");
                            Self::render_store_op_combo(ui, &mut self.depth_store_op, "depth_store");
                            ui.end_row();
                        });
                    
                    ui.add_space(5.0);
                    ui.label("Stencil Configuration:");
                    egui::Grid::new("stencil_config")
                        .num_columns(2)
                        .spacing([10.0, 8.0])
                        .show(ui, |ui| {
                            ui.label("Load Operation:");
                            Self::render_load_op_combo(ui, &mut self.stencil_load_op, "stencil_load");
                            ui.end_row();
                            
                            if self.stencil_load_op == LoadOpChoice::Clear {
                                ui.label("Clear Stencil:");
                                ui.add(egui::Slider::new(&mut self.clear_stencil, 0..=255));
                                ui.end_row();
                            }
                            
                            ui.label("Store Operation:");
                            Self::render_store_op_combo(ui, &mut self.stencil_store_op, "stencil_store");
                            ui.end_row();
                        });
                }
            });
            
            ui.add_space(10.0);
            
            // Timestamp Writes Configuration
            ui.group(|ui| {
                ui.heading("⏱️ Timestamp Writes");
                ui.add_space(5.0);
                
                ui.checkbox(&mut self.enable_timestamp_writes, "Enable timestamp writes")
                    .on_hover_text("Capture GPU timestamps at the beginning and end of the render pass");
                
                if self.enable_timestamp_writes {
                    ui.add_space(5.0);
                    ui.label("Timestamp writes allow you to measure GPU performance by writing timestamps to a query set.");
                    ui.add_space(5.0);
                    
                    egui::Grid::new("timestamp_config")
                        .num_columns(2)
                        .spacing([10.0, 8.0])
                        .show(ui, |ui| {
                            ui.label("Beginning Index:")
                                .on_hover_text("Query set index for the beginning timestamp");
                            ui.text_edit_singleline(&mut self.timestamp_beginning_index_input);
                            ui.end_row();
                            
                            ui.label("End Index:")
                                .on_hover_text("Query set index for the end timestamp");
                            ui.text_edit_singleline(&mut self.timestamp_end_index_input);
                            ui.end_row();
                        });
                    
                    ui.add_space(5.0);
                    ui.colored_label(
                        egui::Color32::from_rgb(200, 200, 100),
                        "💡 Note: Requires a QuerySet of type Timestamp and the TIMESTAMP_QUERY feature"
                    );
                }
            });
            
            ui.add_space(15.0);
            
            // Validation and Actions
            ui.horizontal(|ui| {
                if ui.button("🔍 Validate").clicked() {
                    if self.validate() {
                        self.success_message = Some("✓ Configuration is valid".to_string());
                    }
                }
                
                if ui.button("🔄 Reset").clicked() {
                    *self = Self::new();
                }
                
                if ui.button("📋 Preset: Black Clear").clicked() {
                    self.enable_color_attachment = true;
                    self.color_load_op = LoadOpChoice::Clear;
                    self.clear_color_r = 0.0;
                    self.clear_color_g = 0.0;
                    self.clear_color_b = 0.0;
                    self.clear_color_a = 1.0;
                    self.color_store_op = StoreOpChoice::Store;
                }
                
                if ui.button("📋 Preset: With Depth").clicked() {
                    self.enable_color_attachment = true;
                    self.enable_depth_stencil = true;
                    self.depth_load_op = LoadOpChoice::Clear;
                    self.clear_depth = 1.0;
                    self.depth_store_op = StoreOpChoice::Store;
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
                    if self.label_input.is_empty() { "<none>" } else { &self.label_input }
                ));
                
                ui.add_space(5.0);
                ui.label("Color Attachment:");
                if self.enable_color_attachment {
                    ui.monospace(format!("  Load: {}", self.color_load_op.name()));
                    if self.color_load_op == LoadOpChoice::Clear {
                        ui.monospace(format!(
                            "  Clear Color: ({:.2}, {:.2}, {:.2}, {:.2})",
                            self.clear_color_r, self.clear_color_g,
                            self.clear_color_b, self.clear_color_a
                        ));
                    }
                    ui.monospace(format!("  Store: {}", self.color_store_op.name()));
                } else {
                    ui.monospace("  (disabled)");
                }
                
                ui.add_space(5.0);
                ui.label("Depth-Stencil Attachment:");
                if self.enable_depth_stencil {
                    ui.monospace(format!("  Depth Load: {}", self.depth_load_op.name()));
                    if self.depth_load_op == LoadOpChoice::Clear {
                        ui.monospace(format!("  Clear Depth: {:.2}", self.clear_depth));
                    }
                    ui.monospace(format!("  Depth Store: {}", self.depth_store_op.name()));
                    ui.monospace(format!("  Stencil Load: {}", self.stencil_load_op.name()));
                    if self.stencil_load_op == LoadOpChoice::Clear {
                        ui.monospace(format!("  Clear Stencil: {}", self.clear_stencil));
                    }
                    ui.monospace(format!("  Stencil Store: {}", self.stencil_store_op.name()));
                } else {
                    ui.monospace("  (disabled)");
                }
                
                ui.add_space(5.0);
                ui.label("Timestamp Writes:");
                if self.enable_timestamp_writes {
                    ui.monospace(format!("  Beginning Index: {}", self.timestamp_beginning_index_input));
                    ui.monospace(format!("  End Index: {}", self.timestamp_end_index_input));
                } else {
                    ui.monospace("  (disabled)");
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
        let panel = RenderPassPanel::new();
        assert_eq!(panel.label_input, "");
        assert!(panel.enable_color_attachment);
        assert!(!panel.enable_depth_stencil);
        assert!(!panel.enable_timestamp_writes);
    }
    
    #[test]
    fn test_load_op_choice() {
        assert_eq!(LoadOpChoice::Clear.name(), "Clear");
        assert_eq!(LoadOpChoice::Load.name(), "Load");
        assert_eq!(LoadOpChoice::all().len(), 2);
    }
    
    #[test]
    fn test_store_op_choice() {
        assert_eq!(StoreOpChoice::Store.name(), "Store");
        assert_eq!(StoreOpChoice::Discard.name(), "Discard");
        assert_eq!(StoreOpChoice::all().len(), 2);
    }
    
    #[test]
    fn test_color_clear_value() {
        let mut panel = RenderPassPanel::new();
        panel.clear_color_r = 0.5;
        panel.clear_color_g = 0.25;
        panel.clear_color_b = 0.75;
        panel.clear_color_a = 1.0;
        
        let color = panel.get_color_clear();
        assert_eq!(color.r, 0.5);
        assert_eq!(color.g, 0.25);
        assert_eq!(color.b, 0.75);
        assert_eq!(color.a, 1.0);
    }
    
    #[test]
    fn test_color_load_op() {
        let mut panel = RenderPassPanel::new();
        panel.color_load_op = LoadOpChoice::Clear;
        
        let load_op = panel.get_color_load_op();
        match load_op {
            LoadOp::Clear(_) => {},
            LoadOp::Load => panic!("Expected Clear, got Load"),
        }
        
        panel.color_load_op = LoadOpChoice::Load;
        let load_op = panel.get_color_load_op();
        match load_op {
            LoadOp::Clear(_) => panic!("Expected Load, got Clear"),
            LoadOp::Load => {},
        }
    }
    
    #[test]
    fn test_depth_load_op() {
        let mut panel = RenderPassPanel::new();
        panel.depth_load_op = LoadOpChoice::Clear;
        panel.clear_depth = 0.5;
        
        let load_op = panel.get_depth_load_op();
        match load_op {
            LoadOp::Clear(val) => assert_eq!(val, 0.5),
            LoadOp::Load => panic!("Expected Clear, got Load"),
        }
    }
    
    #[test]
    fn test_validation_no_attachments() {
        let mut panel = RenderPassPanel::new();
        panel.enable_color_attachment = false;
        panel.enable_depth_stencil = false;
        
        assert!(!panel.validate());
        assert!(panel.validation_error.is_some());
    }
    
    #[test]
    fn test_validation_color_only() {
        let mut panel = RenderPassPanel::new();
        panel.enable_color_attachment = true;
        panel.enable_depth_stencil = false;
        
        assert!(panel.validate());
        assert!(panel.validation_error.is_none());
    }
    
    #[test]
    fn test_validation_depth_only() {
        let mut panel = RenderPassPanel::new();
        panel.enable_color_attachment = false;
        panel.enable_depth_stencil = true;
        
        assert!(panel.validate());
        assert!(panel.validation_error.is_none());
    }
    
    #[test]
    fn test_validation_invalid_clear_color() {
        let mut panel = RenderPassPanel::new();
        panel.color_load_op = LoadOpChoice::Clear;
        panel.clear_color_r = 1.5; // Invalid
        
        assert!(!panel.validate());
        assert!(panel.validation_error.is_some());
    }
    
    #[test]
    fn test_validation_invalid_depth() {
        let mut panel = RenderPassPanel::new();
        panel.enable_depth_stencil = true;
        panel.depth_load_op = LoadOpChoice::Clear;
        panel.clear_depth = 1.5; // Invalid
        
        assert!(!panel.validate());
        assert!(panel.validation_error.is_some());
    }
    
    #[test]
    fn test_validation_invalid_timestamp_index() {
        let mut panel = RenderPassPanel::new();
        panel.enable_timestamp_writes = true;
        panel.timestamp_beginning_index_input = "not_a_number".to_string();
        
        assert!(!panel.validate());
        assert!(panel.validation_error.is_some());
    }
    
    #[test]
    fn test_validation_valid_timestamp() {
        let mut panel = RenderPassPanel::new();
        panel.enable_timestamp_writes = true;
        panel.timestamp_beginning_index_input = "0".to_string();
        panel.timestamp_end_index_input = "1".to_string();
        
        assert!(panel.validate());
        assert!(panel.validation_error.is_none());
    }
    
    #[test]
    fn test_store_op_conversion() {
        assert_eq!(StoreOpChoice::Store.to_store_op(), StoreOp::Store);
        assert_eq!(StoreOpChoice::Discard.to_store_op(), StoreOp::Discard);
    }
    
    #[test]
    fn test_stencil_ops() {
        let mut panel = RenderPassPanel::new();
        panel.stencil_load_op = LoadOpChoice::Clear;
        panel.clear_stencil = 42;
        
        let load_op = panel.get_stencil_load_op();
        match load_op {
            LoadOp::Clear(val) => assert_eq!(val, 42),
            LoadOp::Load => panic!("Expected Clear, got Load"),
        }
        
        assert_eq!(panel.get_stencil_store_op(), StoreOp::Store);
    }
    
    #[test]
    fn test_default_values() {
        let panel = RenderPassPanel::new();
        assert_eq!(panel.clear_color_r, 0.0);
        assert_eq!(panel.clear_color_g, 0.0);
        assert_eq!(panel.clear_color_b, 0.0);
        assert_eq!(panel.clear_color_a, 1.0);
        assert_eq!(panel.clear_depth, 1.0);
        assert_eq!(panel.clear_stencil, 0);
        assert_eq!(panel.color_load_op, LoadOpChoice::Clear);
        assert_eq!(panel.color_store_op, StoreOpChoice::Store);
    }
}
