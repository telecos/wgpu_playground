use crate::adapter::{enumerate_adapters, AdapterInfo};
use crate::implementation::WebGPUImplementation;
use wgpu::{Backends, PowerPreference};

/// UI panel for selecting GPU adapters and configuring power preferences
pub struct AdapterSelectionPanel {
    /// List of available adapters
    available_adapters: Vec<AdapterInfo>,
    /// Currently selected adapter index
    selected_adapter_index: usize,
    /// Power preference setting
    power_preference: PowerPreference,
    /// Selected backends for enumeration
    selected_backends: Backends,
}

impl AdapterSelectionPanel {
    /// Create a new adapter selection panel with the current adapter
    pub fn new(current_adapter: &wgpu::Adapter) -> Self {
        let backends = Backends::all();
        let available_adapters = enumerate_adapters(backends);
        let current_info = current_adapter.get_info();

        // Find the index of the current adapter
        let selected_adapter_index = available_adapters
            .iter()
            .position(|info| {
                info.name == current_info.name
                    && info.backend == current_info.backend
                    && info.device == current_info.device
            })
            .unwrap_or(0);

        Self {
            available_adapters,
            selected_adapter_index,
            // PowerPreference::default() is PowerPreference::None
            power_preference: PowerPreference::default(),
            selected_backends: backends,
        }
    }

    /// Get the currently selected power preference
    pub fn power_preference(&self) -> PowerPreference {
        self.power_preference
    }

    /// Get the currently selected adapter info
    pub fn selected_adapter(&self) -> Option<&AdapterInfo> {
        self.available_adapters.get(self.selected_adapter_index)
    }

    /// Refresh the list of available adapters
    fn refresh_adapters(&mut self) {
        self.available_adapters = enumerate_adapters(self.selected_backends);
        // Clamp selected index to valid range
        if self.selected_adapter_index >= self.available_adapters.len() {
            self.selected_adapter_index = self.available_adapters.len().saturating_sub(1);
        }
    }

    /// Render the UI panel
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("🎮 GPU Adapter Selection");
            ui.separator();
            ui.add_space(10.0);

            // Backend selection
            ui.heading("Backend Filter");
            ui.horizontal(|ui| {
                let mut changed = false;

                if ui
                    .selectable_label(self.selected_backends == Backends::all(), "All")
                    .clicked()
                {
                    self.selected_backends = Backends::all();
                    changed = true;
                }
                if ui
                    .selectable_label(self.selected_backends == Backends::PRIMARY, "Primary")
                    .clicked()
                {
                    self.selected_backends = Backends::PRIMARY;
                    changed = true;
                }
                if ui
                    .selectable_label(self.selected_backends == Backends::VULKAN, "Vulkan")
                    .clicked()
                {
                    self.selected_backends = Backends::VULKAN;
                    changed = true;
                }
                if ui
                    .selectable_label(self.selected_backends == Backends::METAL, "Metal")
                    .clicked()
                {
                    self.selected_backends = Backends::METAL;
                    changed = true;
                }
                if ui
                    .selectable_label(self.selected_backends == Backends::DX12, "DX12")
                    .clicked()
                {
                    self.selected_backends = Backends::DX12;
                    changed = true;
                }
                if ui
                    .selectable_label(self.selected_backends == Backends::GL, "OpenGL")
                    .clicked()
                {
                    self.selected_backends = Backends::GL;
                    changed = true;
                }

                if changed {
                    self.refresh_adapters();
                }
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            // Power preference selection
            ui.heading("Power Preference");
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.power_preference, PowerPreference::None, "None");
                ui.selectable_value(
                    &mut self.power_preference,
                    PowerPreference::LowPower,
                    "Low Power",
                );
                ui.selectable_value(
                    &mut self.power_preference,
                    PowerPreference::HighPerformance,
                    "High Performance",
                );
            });

            ui.add_space(5.0);
            ui.label("💡 Power preference hints the system about your performance needs:");
            ui.label("   • None: No preference (default)");
            ui.label("   • Low Power: Prefer energy efficiency (integrated GPU)");
            ui.label("   • High Performance: Prefer maximum performance (discrete GPU)");

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(10.0);

            // Available adapters list
            ui.heading("Available Adapters");

            if self.available_adapters.is_empty() {
                ui.label("⚠️ No adapters found with the selected backend filter.");
            } else {
                ui.label(format!(
                    "Found {} adapter(s)",
                    self.available_adapters.len()
                ));
                ui.add_space(10.0);

                for (idx, adapter_info) in self.available_adapters.iter().enumerate() {
                    let is_selected = idx == self.selected_adapter_index;

                    ui.group(|ui| {
                        if ui
                            .selectable_label(is_selected, &adapter_info.name)
                            .clicked()
                        {
                            self.selected_adapter_index = idx;
                        }

                        if is_selected {
                            ui.add_space(5.0);

                            ui.horizontal(|ui| {
                                ui.label("Backend:");
                                ui.strong(adapter_info.backend_name());
                            });

                            ui.horizontal(|ui| {
                                ui.label("Device Type:");
                                ui.strong(format!("{:?}", adapter_info.device_type));
                            });

                            ui.horizontal(|ui| {
                                ui.label("Vendor ID:");
                                ui.strong(format!("0x{:04X}", adapter_info.vendor));
                            });

                            ui.horizontal(|ui| {
                                ui.label("Device ID:");
                                ui.strong(format!("0x{:04X}", adapter_info.device));
                            });

                            if !adapter_info.driver.is_empty() {
                                ui.horizontal(|ui| {
                                    ui.label("Driver:");
                                    ui.strong(&adapter_info.driver);
                                });
                            }

                            if !adapter_info.driver_info.is_empty() {
                                ui.horizontal(|ui| {
                                    ui.label("Driver Info:");
                                    ui.strong(&adapter_info.driver_info);
                                });
                            }
                        }
                    });

                    ui.add_space(5.0);
                }
            }

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(10.0);

            // WebGPU Implementation information
            ui.heading("🔧 WebGPU Implementation");
            let current_impl = WebGPUImplementation::current();
            ui.horizontal(|ui| {
                ui.label("Active Implementation:");
                ui.strong(current_impl.name());
            });
            ui.label(current_impl.description());
            ui.add_space(5.0);

            // Show implementation status
            let status = current_impl.status_message();
            if current_impl.is_native() {
                ui.colored_label(egui::Color32::from_rgb(100, 200, 100), status);
            } else {
                ui.colored_label(egui::Color32::from_rgb(255, 165, 0), status);
            }
            ui.add_space(5.0);

            // List available implementations
            ui.label("Available implementations:");
            for impl_type in WebGPUImplementation::available_implementations() {
                ui.horizontal(|ui| {
                    if impl_type == current_impl {
                        ui.label("  ✓");
                    } else {
                        ui.label("  ○");
                    }
                    ui.label(impl_type.name());
                    ui.label(format!(
                        "({})",
                        if impl_type == current_impl {
                            "active"
                        } else {
                            "inactive"
                        }
                    ));
                });
            }

            if !WebGPUImplementation::is_dawn_available() {
                ui.add_space(5.0);
                ui.label("ℹ️ To enable Dawn support, compile with: cargo build --features dawn");
            }

            ui.add_space(5.0);
            ui.label("💡 Tip: Set WEBGPU_IMPL environment variable to switch implementations.");
            ui.label(format!(
                "   Available: {}",
                WebGPUImplementation::available_implementations_list()
            ));

            ui.add_space(20.0);
            ui.separator();
            ui.add_space(10.0);

            // Information section
            ui.heading("ℹ️ Information");
            ui.label("⚠️ Note: Changing the adapter requires restarting the application.");
            ui.label("Set the WGPU_BACKEND environment variable and restart:");
            ui.monospace("WGPU_BACKEND=vulkan cargo run --release");
            ui.add_space(5.0);
            ui.label("The selected power preference will be used when requesting the adapter.");
        });
    }
}
