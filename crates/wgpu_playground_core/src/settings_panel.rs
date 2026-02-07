use crate::implementation::WebGPUImplementation;
use crate::state::Theme;
use crate::tooltip::TooltipExt;

/// UI panel for application settings
pub struct SettingsPanel {
    /// Current theme selection
    current_theme: Theme,
    /// Currently selected backend
    selected_backend: WebGPUImplementation,
}

impl SettingsPanel {
    /// Create a new settings panel with default theme
    pub fn new() -> Self {
        Self {
            current_theme: Theme::default(),
            selected_backend: WebGPUImplementation::current(),
        }
    }

    /// Create a settings panel with a specific theme
    pub fn with_theme(theme: Theme) -> Self {
        Self {
            current_theme: theme,
            selected_backend: WebGPUImplementation::current(),
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

        // Theme Settings
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

        ui.add_space(20.0);
        ui.separator();
        ui.add_space(10.0);

        // Backend Settings
        ui.heading("🔧 WebGPU Backend");
        ui.label("Select the WebGPU implementation backend:");
        ui.add_space(5.0);

        let current_backend = WebGPUImplementation::current();

        // Display current backend prominently
        ui.horizontal(|ui| {
            ui.label("Current Backend:");
            match current_backend {
                WebGPUImplementation::Wgpu => {
                    ui.colored_label(egui::Color32::from_rgb(100, 150, 255), "wgpu-rs");
                }
                #[cfg(feature = "dawn")]
                WebGPUImplementation::Dawn => {
                    ui.colored_label(egui::Color32::from_rgb(255, 180, 100), "Dawn Native");
                }
            }
        });
        ui.add_space(5.0);

        // Backend selection dropdown
        let selected_text = match self.selected_backend {
            WebGPUImplementation::Wgpu => "🦀 wgpu-rs (Rust implementation)",
            #[cfg(feature = "dawn")]
            WebGPUImplementation::Dawn => "🌅 Dawn Native (C++ implementation)",
        };

        egui::ComboBox::from_label("Select Backend")
            .selected_text(selected_text)
            .show_ui(ui, |ui| {
                // Always show wgpu option
                ui.selectable_value(
                    &mut self.selected_backend,
                    WebGPUImplementation::Wgpu,
                    "🦀 wgpu-rs (Rust implementation)",
                )
                .webgpu_tooltip(
                    "wgpu-rs is a Rust implementation of the WebGPU API. Used by Firefox and Servo, it provides a fully featured and stable WebGPU implementation with excellent support for Vulkan, Metal, DirectX 12, and OpenGL backends.",
                    None
                );

                // Show Dawn option only if available
                #[cfg(feature = "dawn")]
                {
                    ui.selectable_value(
                        &mut self.selected_backend,
                        WebGPUImplementation::Dawn,
                        "🌅 Dawn Native (C++ implementation)",
                    )
                    .webgpu_tooltip(
                        "Dawn is Google's C++ implementation of the WebGPU API. Used by Chromium-based browsers (Chrome, Edge, Opera), it provides native WebGPU support with backends for Vulkan, Metal, and DirectX 12.",
                        None
                    );
                }

                // Show Dawn as disabled if not compiled in
                #[cfg(not(feature = "dawn"))]
                {
                    ui.add_enabled(false, egui::Label::new("🌅 Dawn Native (Not Available)"))
                        .webgpu_tooltip(
                            "Dawn is not available in this build. Compile with --features dawn to enable Google's C++ implementation of the WebGPU API used by Chromium browsers.",
                            None
                        );
                }
            });

        ui.add_space(10.0);

        // Show backend description
        ui.label(self.selected_backend.description());
        ui.add_space(5.0);

        // Show if selection differs from current
        if self.selected_backend != current_backend {
            ui.add_space(5.0);
            ui.colored_label(
                egui::Color32::from_rgb(255, 200, 100),
                "⚠️ Warning: Backend switching requires application restart",
            );
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("To apply this change, set the environment variable:");
                ui.code(format!("WEBGPU_IMPL={}", self.selected_backend.name()));
            });
            ui.label("Then restart the application.");
        }

        ui.add_space(10.0);

        // Show availability status
        if WebGPUImplementation::is_dawn_available() {
            ui.colored_label(
                egui::Color32::from_rgb(100, 200, 100),
                "✓ Dawn support is compiled in",
            );
        } else {
            ui.colored_label(
                egui::Color32::from_rgb(200, 200, 100),
                "ℹ️ Dawn support not available (compile with --features dawn)",
            );
        }

        ui.add_space(10.0);

        // Additional info
        let available_backends = WebGPUImplementation::available_implementations();
        ui.label("💡 Tip: Available backends:");
        for backend in &available_backends {
            ui.horizontal(|ui| {
                if *backend == current_backend {
                    ui.label("  ✓");
                } else {
                    ui.label("  ○");
                }
                ui.label(backend.name());
                ui.label(format!(
                    "({})",
                    if *backend == current_backend {
                        "active"
                    } else {
                        "inactive"
                    }
                ));
            });
        }

        theme_changed
    }
}

impl Default for SettingsPanel {
    fn default() -> Self {
        Self::new()
    }
}
