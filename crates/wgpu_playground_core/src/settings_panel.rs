use crate::state::Theme;

/// UI panel for application settings
pub struct SettingsPanel {
    /// Current theme selection
    current_theme: Theme,
}

impl SettingsPanel {
    /// Create a new settings panel with default theme
    pub fn new() -> Self {
        Self {
            current_theme: Theme::default(),
        }
    }

    /// Create a settings panel with a specific theme
    pub fn with_theme(theme: Theme) -> Self {
        Self {
            current_theme: theme,
        }
    }

    /// Get the current theme
    pub fn get_theme(&self) -> Theme {
        self.current_theme
    }

    /// Set the current theme
    pub fn set_theme(&mut self, theme: Theme) {
        self.current_theme = theme;
    }

    /// Render the settings panel UI
    pub fn ui(&mut self, ui: &mut egui::Ui) -> Option<Theme> {
        let mut theme_changed = None;

        ui.heading("⚙️ Settings");
        ui.separator();
        ui.add_space(10.0);

        ui.heading("Theme");
        ui.label("Choose your preferred UI theme:");
        ui.add_space(5.0);

        let previous_theme = self.current_theme;

        egui::ComboBox::from_label("Theme")
            .selected_text(match self.current_theme {
                Theme::Light => "☀️ Light",
                Theme::Dark => "🌙 Dark",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.current_theme, Theme::Light, "☀️ Light");
                ui.selectable_value(&mut self.current_theme, Theme::Dark, "🌙 Dark");
            });

        if self.current_theme != previous_theme {
            theme_changed = Some(self.current_theme);
        }

        ui.add_space(10.0);
        ui.label("Theme changes are applied immediately and saved automatically.");

        theme_changed
    }
}

impl Default for SettingsPanel {
    fn default() -> Self {
        Self::new()
    }
}
