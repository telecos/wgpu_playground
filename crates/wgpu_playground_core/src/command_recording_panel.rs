/// UI panel for recording and playing back GPU commands
use std::time::SystemTime;

/// Type of GPU command
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandType {
    /// Render pass command
    RenderPass,
    /// Compute pass command
    ComputePass,
    /// Buffer copy command
    BufferCopy,
    /// Texture copy command
    TextureCopy,
    /// Clear buffer command
    ClearBuffer,
}

impl CommandType {
    /// Get display name for command type
    pub fn as_str(&self) -> &'static str {
        match self {
            CommandType::RenderPass => "Render Pass",
            CommandType::ComputePass => "Compute Pass",
            CommandType::BufferCopy => "Buffer Copy",
            CommandType::TextureCopy => "Texture Copy",
            CommandType::ClearBuffer => "Clear Buffer",
        }
    }

    /// Get emoji icon for command type
    pub fn icon(&self) -> &'static str {
        match self {
            CommandType::RenderPass => "🎨",
            CommandType::ComputePass => "🧮",
            CommandType::BufferCopy => "📋",
            CommandType::TextureCopy => "🖼️",
            CommandType::ClearBuffer => "🧹",
        }
    }
}

/// A recorded GPU command with metadata
#[derive(Debug, Clone)]
pub struct CommandRecord {
    /// Command ID (unique identifier)
    pub id: usize,
    /// Command type
    pub command_type: CommandType,
    /// Command label/name
    pub label: String,
    /// Command description
    pub description: String,
    /// Timestamp when command was recorded
    pub timestamp: SystemTime,
    /// Duration in microseconds (0 if not measured)
    pub duration_us: u64,
}

impl CommandRecord {
    /// Create a new command record
    pub fn new(id: usize, command_type: CommandType, label: impl Into<String>) -> Self {
        Self {
            id,
            command_type,
            label: label.into(),
            description: String::new(),
            timestamp: SystemTime::now(),
            duration_us: 0,
        }
    }

    /// Create with description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Set duration
    pub fn with_duration(mut self, duration_us: u64) -> Self {
        self.duration_us = duration_us;
        self
    }

    /// Format timestamp for display
    pub fn format_timestamp(&self) -> String {
        let duration = self
            .timestamp
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default();
        let millis = duration.as_millis();
        format!("{} ms", millis % 100000)
    }

    /// Format duration for display
    pub fn format_duration(&self) -> String {
        if self.duration_us == 0 {
            "-".to_string()
        } else if self.duration_us < 1000 {
            format!("{} μs", self.duration_us)
        } else if self.duration_us < 1_000_000 {
            format!("{:.2} ms", self.duration_us as f64 / 1000.0)
        } else {
            format!("{:.2} s", self.duration_us as f64 / 1_000_000.0)
        }
    }
}

/// Command recording and playback panel
pub struct CommandRecordingPanel {
    /// Recorded commands
    commands: Vec<CommandRecord>,
    /// Selected command index for inspection
    selected_command: Option<usize>,
    /// Whether recording is enabled
    is_recording: bool,
    /// Next command ID
    next_id: usize,
    /// Timeline zoom level (pixels per millisecond)
    timeline_zoom: f32,
    /// Export format selection
    export_format: ExportFormat,
}

/// Export format for command recordings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExportFormat {
    Json,
    Text,
}

impl Default for CommandRecordingPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandRecordingPanel {
    /// Create a new command recording panel
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            selected_command: None,
            is_recording: false,
            next_id: 1,
            timeline_zoom: 1.0,
            export_format: ExportFormat::Json,
        }
    }

    /// Add a sample command for demonstration
    fn add_sample_command(&mut self, command_type: CommandType, label: &str, description: &str) {
        let record = CommandRecord::new(self.next_id, command_type, label)
            .with_description(description)
            .with_duration((self.next_id as u64) * 100); // Mock duration

        self.next_id += 1;
        self.commands.push(record);
    }

    /// Clear all recorded commands
    fn clear_commands(&mut self) {
        self.commands.clear();
        self.selected_command = None;
        self.next_id = 1;
    }

    /// Export commands as JSON
    fn export_as_json(&self) -> String {
        let mut json = String::from("[\n");
        for (i, cmd) in self.commands.iter().enumerate() {
            json.push_str(&format!(
                r#"  {{
    "id": {},
    "type": "{}",
    "label": "{}",
    "description": "{}",
    "duration_us": {}
  }}{}"#,
                cmd.id,
                cmd.command_type.as_str(),
                cmd.label,
                cmd.description,
                cmd.duration_us,
                if i < self.commands.len() - 1 { "," } else { "" }
            ));
            json.push('\n');
        }
        json.push_str("]\n");
        json
    }

    /// Export commands as text
    fn export_as_text(&self) -> String {
        let mut text = String::from("GPU Command Recording\n");
        text.push_str("=====================\n\n");

        for cmd in &self.commands {
            text.push_str(&format!(
                "{} {} (ID: {})\n",
                cmd.command_type.icon(),
                cmd.label,
                cmd.id
            ));
            text.push_str(&format!("  Type: {}\n", cmd.command_type.as_str()));
            if !cmd.description.is_empty() {
                text.push_str(&format!("  Description: {}\n", cmd.description));
            }
            text.push_str(&format!("  Duration: {}\n", cmd.format_duration()));
            text.push_str(&format!("  Timestamp: {}\n", cmd.format_timestamp()));
            text.push('\n');
        }

        text
    }

    /// Render the UI
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("📹 Command Recording & Playback");
            ui.separator();
            ui.label("Record, inspect, replay, and export GPU command sequences.");
            ui.add_space(10.0);

            // Control buttons
            ui.group(|ui| {
                ui.label(egui::RichText::new("Recording Controls").strong());
                ui.separator();
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    // Recording toggle
                    let recording_text = if self.is_recording {
                        "⏸️ Stop Recording"
                    } else {
                        "⏺️ Start Recording"
                    };

                    if ui.button(recording_text).clicked() {
                        self.is_recording = !self.is_recording;
                        if self.is_recording {
                            // Add a marker command when recording starts
                            self.add_sample_command(
                                CommandType::RenderPass,
                                "Recording Started",
                                "Begin command recording session",
                            );
                        }
                    }

                    // Clear button
                    if ui.button("🗑️ Clear All").clicked() {
                        self.clear_commands();
                    }

                    // Add sample data for demonstration
                    if ui.button("➕ Add Sample Data").clicked() {
                        self.add_sample_command(
                            CommandType::RenderPass,
                            "Main Render",
                            "Primary rendering pass with geometry",
                        );
                        self.add_sample_command(
                            CommandType::BufferCopy,
                            "Update Uniforms",
                            "Copy uniform data to GPU buffer",
                        );
                        self.add_sample_command(
                            CommandType::ComputePass,
                            "Physics Update",
                            "Compute shader for particle physics",
                        );
                        self.add_sample_command(
                            CommandType::TextureCopy,
                            "Copy Framebuffer",
                            "Copy rendered output to texture",
                        );
                    }

                    ui.label(format!("Recording: {}", if self.is_recording { "🔴 ON" } else { "⚪ OFF" }));
                });
            });

            ui.add_space(10.0);

            // Command list
            ui.group(|ui| {
                ui.label(egui::RichText::new("Recorded Commands").strong());
                ui.separator();
                ui.add_space(5.0);

                if self.commands.is_empty() {
                    ui.label(egui::RichText::new("No commands recorded").weak().italics());
                    ui.label("Click 'Add Sample Data' to see example commands");
                } else {
                    ui.label(format!("Total: {} commands", self.commands.len()));
                    ui.add_space(5.0);

                    // Commands table
                    egui::ScrollArea::vertical()
                        .max_height(200.0)
                        .show(ui, |ui| {
                            egui::Grid::new("commands_grid")
                                .num_columns(5)
                                .spacing([10.0, 4.0])
                                .striped(true)
                                .show(ui, |ui| {
                                    // Header
                                    ui.label(egui::RichText::new("ID").strong());
                                    ui.label(egui::RichText::new("Type").strong());
                                    ui.label(egui::RichText::new("Label").strong());
                                    ui.label(egui::RichText::new("Duration").strong());
                                    ui.label(egui::RichText::new("Actions").strong());
                                    ui.end_row();

                                    // Command rows
                                    for (idx, cmd) in self.commands.iter().enumerate() {
                                        ui.label(cmd.id.to_string());
                                        ui.label(format!("{} {}", cmd.command_type.icon(), cmd.command_type.as_str()));
                                        ui.label(&cmd.label);
                                        ui.label(cmd.format_duration());

                                        let is_selected = self.selected_command == Some(idx);
                                        if ui.selectable_label(is_selected, "🔍 Inspect").clicked() {
                                            self.selected_command = Some(idx);
                                        }

                                        ui.end_row();
                                    }
                                });
                        });
                }
            });

            ui.add_space(10.0);

            // Timeline visualization
            ui.group(|ui| {
                ui.label(egui::RichText::new("Timeline View").strong());
                ui.separator();
                ui.add_space(5.0);

                if !self.commands.is_empty() {
                    // Timeline controls
                    ui.horizontal(|ui| {
                        ui.label("Zoom:");
                        ui.add(egui::Slider::new(&mut self.timeline_zoom, 0.1..=5.0).text("scale"));
                    });

                    ui.add_space(5.0);

                    // Simple timeline visualization
                    let available_width = ui.available_width();
                    let timeline_height = 60.0;
                    let (response, painter) = ui.allocate_painter(
                        egui::vec2(available_width, timeline_height),
                        egui::Sense::hover(),
                    );

                    let rect = response.rect;

                    // Background
                    painter.rect_filled(rect, 0.0, egui::Color32::from_gray(30));

                    // Draw command blocks
                    let total_duration: u64 = self.commands.iter().map(|c| c.duration_us).sum();
                    if total_duration > 0 {
                        let mut offset = 0u64;
                        for (idx, cmd) in self.commands.iter().enumerate() {
                            let start_x = rect.min.x + (offset as f32 / total_duration as f32) * rect.width();
                            let width = (cmd.duration_us as f32 / total_duration as f32) * rect.width();

                            let color = match cmd.command_type {
                                CommandType::RenderPass => egui::Color32::from_rgb(100, 150, 255),
                                CommandType::ComputePass => egui::Color32::from_rgb(255, 150, 100),
                                CommandType::BufferCopy => egui::Color32::from_rgb(150, 255, 150),
                                CommandType::TextureCopy => egui::Color32::from_rgb(255, 200, 100),
                                CommandType::ClearBuffer => egui::Color32::from_rgb(200, 200, 200),
                            };

                            let is_selected = self.selected_command == Some(idx);
                            let bar_color = if is_selected {
                                egui::Color32::WHITE
                            } else {
                                color
                            };

                            painter.rect_filled(
                                egui::Rect::from_min_size(
                                    egui::pos2(start_x, rect.min.y + 10.0),
                                    egui::vec2(width.max(2.0), rect.height() - 20.0),
                                ),
                                2.0,
                                bar_color,
                            );

                            offset += cmd.duration_us;
                        }
                    }
                } else {
                    ui.label(egui::RichText::new("No commands to display").weak().italics());
                }
            });

            ui.add_space(10.0);

            // Command inspector
            if let Some(idx) = self.selected_command {
                if let Some(cmd) = self.commands.get(idx) {
                    ui.group(|ui| {
                        ui.label(egui::RichText::new("Command Inspector").strong());
                        ui.separator();
                        ui.add_space(5.0);

                        egui::Grid::new("inspector_grid")
                            .num_columns(2)
                            .spacing([10.0, 8.0])
                            .show(ui, |ui| {
                                ui.label(egui::RichText::new("ID:").strong());
                                ui.label(cmd.id.to_string());
                                ui.end_row();

                                ui.label(egui::RichText::new("Type:").strong());
                                ui.label(format!("{} {}", cmd.command_type.icon(), cmd.command_type.as_str()));
                                ui.end_row();

                                ui.label(egui::RichText::new("Label:").strong());
                                ui.label(&cmd.label);
                                ui.end_row();

                                ui.label(egui::RichText::new("Description:").strong());
                                ui.label(if cmd.description.is_empty() {
                                    "No description"
                                } else {
                                    &cmd.description
                                });
                                ui.end_row();

                                ui.label(egui::RichText::new("Duration:").strong());
                                ui.label(cmd.format_duration());
                                ui.end_row();

                                ui.label(egui::RichText::new("Timestamp:").strong());
                                ui.label(cmd.format_timestamp());
                                ui.end_row();
                            });

                        ui.add_space(10.0);

                        // Command buffer contents (mock)
                        ui.label(egui::RichText::new("Command Buffer Contents:").strong());
                        ui.separator();
                        ui.label(egui::RichText::new(format!(
                            "// Command buffer for: {}\n// Type: {}\n// This would show the actual GPU commands",
                            cmd.label,
                            cmd.command_type.as_str()
                        )).code());
                    });
                } else {
                    self.selected_command = None;
                }
            }

            ui.add_space(10.0);

            // Playback controls
            ui.group(|ui| {
                ui.label(egui::RichText::new("Playback Controls").strong());
                ui.separator();
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    if ui.button("▶️ Replay All").clicked() {
                        // Stub: Would replay all commands
                        log::info!("Replay all commands (stub)");
                    }

                    if ui.button("▶️ Replay Selected").clicked() && self.selected_command.is_some() {
                        // Stub: Would replay selected command
                        if let Some(idx) = self.selected_command {
                            log::info!("Replay command {} (stub)", idx);
                        }
                    }

                    ui.label(egui::RichText::new("(Playback is currently a stub)").weak().italics());
                });
            });

            ui.add_space(10.0);

            // Export section
            ui.group(|ui| {
                ui.label(egui::RichText::new("Export Commands").strong());
                ui.separator();
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.label("Format:");
                    ui.selectable_value(&mut self.export_format, ExportFormat::Json, "JSON");
                    ui.selectable_value(&mut self.export_format, ExportFormat::Text, "Text");
                });

                ui.add_space(5.0);

                if ui.button("📥 Export").clicked() {
                    let exported = match self.export_format {
                        ExportFormat::Json => self.export_as_json(),
                        ExportFormat::Text => self.export_as_text(),
                    };
                    log::info!("Exported commands:\n{}", exported);
                }

                if !self.commands.is_empty() {
                    ui.add_space(5.0);
                    ui.label(egui::RichText::new("Preview:").weak());

                    let preview = match self.export_format {
                        ExportFormat::Json => self.export_as_json(),
                        ExportFormat::Text => self.export_as_text(),
                    };

                    let preview_lines: Vec<&str> = preview.lines().take(10).collect();
                    let preview_text = preview_lines.join("\n");
                    let preview_with_ellipsis = if preview.lines().count() > 10 {
                        format!("{}\n...", preview_text)
                    } else {
                        preview_text
                    };

                    egui::ScrollArea::vertical()
                        .max_height(150.0)
                        .show(ui, |ui| {
                            ui.label(egui::RichText::new(preview_with_ellipsis).code());
                        });
                }
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_recording_panel_new() {
        let panel = CommandRecordingPanel::new();
        assert!(panel.commands.is_empty());
        assert_eq!(panel.selected_command, None);
        assert!(!panel.is_recording);
        assert_eq!(panel.next_id, 1);
    }

    #[test]
    fn test_command_recording_panel_default() {
        let panel = CommandRecordingPanel::default();
        assert!(panel.commands.is_empty());
    }

    #[test]
    fn test_add_sample_command() {
        let mut panel = CommandRecordingPanel::new();
        panel.add_sample_command(CommandType::RenderPass, "Test", "Description");

        assert_eq!(panel.commands.len(), 1);
        assert_eq!(panel.commands[0].label, "Test");
        assert_eq!(panel.commands[0].command_type, CommandType::RenderPass);
        assert_eq!(panel.next_id, 2);
    }

    #[test]
    fn test_clear_commands() {
        let mut panel = CommandRecordingPanel::new();
        panel.add_sample_command(CommandType::RenderPass, "Test", "Description");
        panel.selected_command = Some(0);

        panel.clear_commands();

        assert!(panel.commands.is_empty());
        assert_eq!(panel.selected_command, None);
        assert_eq!(panel.next_id, 1);
    }

    #[test]
    fn test_command_record_new() {
        let record = CommandRecord::new(1, CommandType::ComputePass, "Test Command");
        assert_eq!(record.id, 1);
        assert_eq!(record.command_type, CommandType::ComputePass);
        assert_eq!(record.label, "Test Command");
        assert!(record.description.is_empty());
        assert_eq!(record.duration_us, 0);
    }

    #[test]
    fn test_command_record_with_description() {
        let record = CommandRecord::new(1, CommandType::BufferCopy, "Test")
            .with_description("Test description");

        assert_eq!(record.description, "Test description");
    }

    #[test]
    fn test_command_record_with_duration() {
        let record = CommandRecord::new(1, CommandType::RenderPass, "Test").with_duration(1500);

        assert_eq!(record.duration_us, 1500);
    }

    #[test]
    fn test_command_type_as_str() {
        assert_eq!(CommandType::RenderPass.as_str(), "Render Pass");
        assert_eq!(CommandType::ComputePass.as_str(), "Compute Pass");
        assert_eq!(CommandType::BufferCopy.as_str(), "Buffer Copy");
        assert_eq!(CommandType::TextureCopy.as_str(), "Texture Copy");
        assert_eq!(CommandType::ClearBuffer.as_str(), "Clear Buffer");
    }

    #[test]
    fn test_command_type_icon() {
        assert_eq!(CommandType::RenderPass.icon(), "🎨");
        assert_eq!(CommandType::ComputePass.icon(), "🧮");
        assert_eq!(CommandType::BufferCopy.icon(), "📋");
        assert_eq!(CommandType::TextureCopy.icon(), "🖼️");
        assert_eq!(CommandType::ClearBuffer.icon(), "🧹");
    }

    #[test]
    fn test_format_duration() {
        let record = CommandRecord::new(1, CommandType::RenderPass, "Test");
        assert_eq!(record.format_duration(), "-");

        let record = record.with_duration(500);
        assert_eq!(record.format_duration(), "500 μs");

        let record = CommandRecord::new(2, CommandType::RenderPass, "Test").with_duration(1500);
        assert_eq!(record.format_duration(), "1.50 ms");

        let record =
            CommandRecord::new(3, CommandType::RenderPass, "Test").with_duration(2_000_000);
        assert_eq!(record.format_duration(), "2.00 s");
    }

    #[test]
    fn test_export_as_json() {
        let mut panel = CommandRecordingPanel::new();
        panel.add_sample_command(CommandType::RenderPass, "Test1", "Desc1");
        panel.add_sample_command(CommandType::ComputePass, "Test2", "Desc2");

        let json = panel.export_as_json();
        assert!(json.contains("Test1"));
        assert!(json.contains("Test2"));
        assert!(json.contains("Render Pass"));
        assert!(json.contains("Compute Pass"));
    }

    #[test]
    fn test_export_as_text() {
        let mut panel = CommandRecordingPanel::new();
        panel.add_sample_command(CommandType::BufferCopy, "Copy", "Copy data");

        let text = panel.export_as_text();
        assert!(text.contains("GPU Command Recording"));
        assert!(text.contains("Copy"));
        assert!(text.contains("Copy data"));
        assert!(text.contains("Buffer Copy"));
    }

    #[test]
    fn test_export_format_enum() {
        let json = ExportFormat::Json;
        let text = ExportFormat::Text;

        assert_eq!(json, ExportFormat::Json);
        assert_eq!(text, ExportFormat::Text);
        assert_ne!(json, text);
    }
}
