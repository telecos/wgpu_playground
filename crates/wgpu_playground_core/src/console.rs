/// Console for displaying WebGPU errors, warnings, and validation messages
use crate::error::{Error, ErrorType};
use std::time::SystemTime;

/// Severity level for console messages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// Informational message
    Info,
    /// Warning message
    Warning,
    /// Error message
    Error,
}

impl Severity {
    /// Get display name for severity
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Info => "Info",
            Severity::Warning => "Warning",
            Severity::Error => "Error",
        }
    }

    /// Get emoji icon for severity
    pub fn icon(&self) -> &'static str {
        match self {
            Severity::Info => "ℹ️",
            Severity::Warning => "⚠️",
            Severity::Error => "❌",
        }
    }
}

/// A console message with timestamp, severity, and details
#[derive(Debug, Clone)]
pub struct ConsoleMessage {
    /// Message timestamp
    pub timestamp: SystemTime,
    /// Message severity level
    pub severity: Severity,
    /// Message content
    pub message: String,
    /// Optional stack trace or additional details
    pub details: Option<String>,
}

impl ConsoleMessage {
    /// Create a new console message
    pub fn new(severity: Severity, message: impl Into<String>) -> Self {
        Self {
            timestamp: SystemTime::now(),
            severity,
            message: message.into(),
            details: None,
        }
    }

    /// Create a new message with details
    pub fn with_details(
        severity: Severity,
        message: impl Into<String>,
        details: impl Into<String>,
    ) -> Self {
        Self {
            timestamp: SystemTime::now(),
            severity,
            message: message.into(),
            details: Some(details.into()),
        }
    }

    /// Create an info message
    pub fn info(message: impl Into<String>) -> Self {
        Self::new(Severity::Info, message)
    }

    /// Create a warning message
    pub fn warning(message: impl Into<String>) -> Self {
        Self::new(Severity::Warning, message)
    }

    /// Create an error message
    pub fn error(message: impl Into<String>) -> Self {
        Self::new(Severity::Error, message)
    }

    /// Get formatted timestamp
    pub fn format_timestamp(&self) -> String {
        let duration = self
            .timestamp
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default();
        let secs = duration.as_secs();
        let millis = duration.subsec_millis();

        // Format as HH:MM:SS.mmm (where mmm is milliseconds)
        let hours = (secs / 3600) % 24;
        let minutes = (secs / 60) % 60;
        let seconds = secs % 60;

        format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis)
    }
}

/// Convert from our Error type to ConsoleMessage
impl From<Error> for ConsoleMessage {
    fn from(error: Error) -> Self {
        let severity = match error.error_type {
            ErrorType::Validation => Severity::Error,
            ErrorType::OutOfMemory => Severity::Error,
            ErrorType::Internal => Severity::Error,
            ErrorType::DeviceLost => Severity::Error,
        };

        ConsoleMessage::with_details(
            severity,
            format!("{}: {}", error.error_type, error.message),
            format!("Error Type: {}", error.error_type),
        )
    }
}

/// Console panel for displaying and filtering messages
pub struct ConsolePanel {
    /// All console messages
    messages: Vec<ConsoleMessage>,
    /// Filter by severity - None means show all
    severity_filter: Option<Severity>,
    /// Maximum number of messages to keep
    max_messages: usize,
    /// Selected message index for details view
    selected_message: Option<usize>,
}

/// Filtered message for display in the UI
struct FilteredMessage {
    index: usize,
    timestamp: String,
    severity: Severity,
    message: String,
    is_selected: bool,
}

impl Default for ConsolePanel {
    fn default() -> Self {
        Self::new()
    }
}

impl ConsolePanel {
    /// Create a new console panel
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            severity_filter: None,
            max_messages: 1000,
            selected_message: None,
        }
    }

    /// Add a message to the console
    pub fn add_message(&mut self, message: ConsoleMessage) {
        self.messages.push(message);

        // Keep only the most recent messages to prevent unbounded growth
        // Using VecDeque would be more efficient, but Vec is simpler for this use case
        if self.messages.len() > self.max_messages {
            self.messages
                .drain(0..self.messages.len() - self.max_messages);

            // Adjust selected message index if needed
            if let Some(idx) = self.selected_message {
                if idx >= self.messages.len() {
                    self.selected_message = None;
                }
            }
        }
    }

    /// Add an info message
    pub fn info(&mut self, message: impl Into<String>) {
        self.add_message(ConsoleMessage::info(message));
    }

    /// Add a warning message
    pub fn warning(&mut self, message: impl Into<String>) {
        self.add_message(ConsoleMessage::warning(message));
    }

    /// Add an error message
    pub fn error(&mut self, message: impl Into<String>) {
        self.add_message(ConsoleMessage::error(message));
    }

    /// Add an error from our Error type
    pub fn add_error(&mut self, error: Error) {
        self.add_message(ConsoleMessage::from(error));
    }

    /// Clear all messages
    pub fn clear(&mut self) {
        self.messages.clear();
        self.selected_message = None;
    }

    /// Set severity filter
    pub fn set_filter(&mut self, filter: Option<Severity>) {
        self.severity_filter = filter;
        self.selected_message = None;
    }

    /// Get filtered messages
    fn filtered_messages(&self) -> Vec<(usize, &ConsoleMessage)> {
        self.messages
            .iter()
            .enumerate()
            .filter(|(_, msg)| {
                self.severity_filter.is_none() || Some(msg.severity) == self.severity_filter
            })
            .collect()
    }

    /// Count messages by severity
    fn count_by_severity(&self, severity: Severity) -> usize {
        self.messages
            .iter()
            .filter(|msg| msg.severity == severity)
            .count()
    }

    /// Add demo messages for testing
    fn add_demo_messages(&mut self) {
        self.info("WebGPU device initialized successfully");
        self.info("Shader compilation started");

        self.warning("Buffer alignment may not be optimal for this usage pattern");
        self.warning("Texture format conversion may impact performance");

        self.add_message(ConsoleMessage::with_details(
            Severity::Error,
            "Validation Error: Invalid buffer usage flags",
            "Buffer created with incompatible usage flags: VERTEX | MAP_WRITE.\n\
             MAP_WRITE is not compatible with VERTEX usage.\n\
             \n\
             Stack trace:\n\
             at create_buffer (buffer.rs:145)\n\
             at setup_vertex_buffer (rendering.rs:89)\n\
             at initialize_pipeline (rendering.rs:34)",
        ));

        self.add_message(ConsoleMessage::with_details(
            Severity::Error,
            "Shader Compilation Error: Type mismatch",
            "Error in fragment shader at line 42:\n\
             Expected 'vec4<f32>' but got 'vec3<f32>'\n\
             \n\
             fn fragment_main() -> vec4<f32> {\n\
                 return vec3(1.0, 0.0, 0.0); // Error: vec3 instead of vec4\n\
             }\n\
             \n\
             Suggestion: Add alpha component:\n\
             return vec4(1.0, 0.0, 0.0, 1.0);",
        ));

        self.warning("Device limit exceeded: Max texture dimension (8192) may be exceeded");
        self.info("Render pass completed in 2.3ms");
    }

    /// Render the console UI
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("🖥️ Error and Warning Console");
        ui.separator();
        ui.label("WebGPU errors, warnings, and validation messages.");
        ui.add_space(10.0);

        // Filter and clear controls
        ui.horizontal(|ui| {
            ui.label("Filter:");

            let all_count = self.messages.len();
            if ui
                .selectable_label(
                    self.severity_filter.is_none(),
                    format!("All ({})", all_count),
                )
                .clicked()
            {
                self.set_filter(None);
            }

            let error_count = self.count_by_severity(Severity::Error);
            if ui
                .selectable_label(
                    self.severity_filter == Some(Severity::Error),
                    format!("{} Errors ({})", Severity::Error.icon(), error_count),
                )
                .clicked()
            {
                self.set_filter(Some(Severity::Error));
            }

            let warning_count = self.count_by_severity(Severity::Warning);
            if ui
                .selectable_label(
                    self.severity_filter == Some(Severity::Warning),
                    format!("{} Warnings ({})", Severity::Warning.icon(), warning_count),
                )
                .clicked()
            {
                self.set_filter(Some(Severity::Warning));
            }

            let info_count = self.count_by_severity(Severity::Info);
            if ui
                .selectable_label(
                    self.severity_filter == Some(Severity::Info),
                    format!("{} Info ({})", Severity::Info.icon(), info_count),
                )
                .clicked()
            {
                self.set_filter(Some(Severity::Info));
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("🗑 Clear").clicked() {
                    self.clear();
                }

                if ui.button("🧪 Add Demo Messages").clicked() {
                    self.add_demo_messages();
                }
            });
        });

        ui.add_space(5.0);
        ui.separator();

        // Message list - collect into a temporary structure to avoid borrow checker issues
        let filtered: Vec<FilteredMessage> = self
            .filtered_messages()
            .iter()
            .map(|(idx, msg)| {
                let is_selected = self.selected_message == Some(*idx);
                FilteredMessage {
                    index: *idx,
                    timestamp: msg.format_timestamp(),
                    severity: msg.severity,
                    message: msg.message.clone(),
                    is_selected,
                }
            })
            .collect();

        if filtered.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.label("No messages to display");
                ui.label("Console messages will appear here");
            });
        } else {
            // Split view: message list on top, details on bottom
            ui.horizontal(|ui| {
                // Message list - display in reverse chronological order (newest first)
                egui::ScrollArea::vertical()
                    .id_salt("message_list")
                    .max_height(ui.available_height() - 150.0)
                    .show(ui, |ui| {
                        for msg in filtered.iter().rev() {
                            let response = ui.selectable_label(
                                msg.is_selected,
                                format!(
                                    "[{}] {} {} {}",
                                    msg.timestamp,
                                    msg.severity.icon(),
                                    msg.severity.as_str(),
                                    msg.message
                                ),
                            );

                            if response.clicked() {
                                self.selected_message = Some(msg.index);
                            }
                        }
                    });
            });

            // Details view
            if let Some(idx) = self.selected_message {
                if let Some(msg) = self.messages.get(idx) {
                    ui.separator();
                    ui.heading("Message Details");

                    egui::ScrollArea::vertical()
                        .id_salt("message_details")
                        .max_height(150.0)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label("Timestamp:");
                                ui.label(msg.format_timestamp());
                            });

                            ui.horizontal(|ui| {
                                ui.label("Severity:");
                                ui.label(format!(
                                    "{} {}",
                                    msg.severity.icon(),
                                    msg.severity.as_str()
                                ));
                            });

                            ui.separator();
                            ui.label("Message:");
                            ui.label(&msg.message);

                            if let Some(details) = &msg.details {
                                ui.separator();
                                ui.label("Details:");
                                ui.label(details);
                            }
                        });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severity_display() {
        assert_eq!(Severity::Info.as_str(), "Info");
        assert_eq!(Severity::Warning.as_str(), "Warning");
        assert_eq!(Severity::Error.as_str(), "Error");
    }

    #[test]
    fn test_severity_icon() {
        assert_eq!(Severity::Info.icon(), "ℹ️");
        assert_eq!(Severity::Warning.icon(), "⚠️");
        assert_eq!(Severity::Error.icon(), "❌");
    }

    #[test]
    fn test_console_message_creation() {
        let msg = ConsoleMessage::info("Test message");
        assert_eq!(msg.severity, Severity::Info);
        assert_eq!(msg.message, "Test message");
        assert!(msg.details.is_none());
    }

    #[test]
    fn test_console_message_with_details() {
        let msg = ConsoleMessage::with_details(Severity::Error, "Error", "Stack trace");
        assert_eq!(msg.severity, Severity::Error);
        assert_eq!(msg.message, "Error");
        assert_eq!(msg.details.as_deref(), Some("Stack trace"));
    }

    #[test]
    fn test_console_panel_creation() {
        let panel = ConsolePanel::new();
        assert_eq!(panel.messages.len(), 0);
        assert!(panel.severity_filter.is_none());
    }

    #[test]
    fn test_add_message() {
        let mut panel = ConsolePanel::new();
        panel.info("Info message");
        panel.warning("Warning message");
        panel.error("Error message");

        assert_eq!(panel.messages.len(), 3);
        assert_eq!(panel.messages[0].severity, Severity::Info);
        assert_eq!(panel.messages[1].severity, Severity::Warning);
        assert_eq!(panel.messages[2].severity, Severity::Error);
    }

    #[test]
    fn test_clear_messages() {
        let mut panel = ConsolePanel::new();
        panel.info("Test");
        panel.error("Error");
        assert_eq!(panel.messages.len(), 2);

        panel.clear();
        assert_eq!(panel.messages.len(), 0);
        assert!(panel.selected_message.is_none());
    }

    #[test]
    fn test_filtering() {
        let mut panel = ConsolePanel::new();
        panel.info("Info 1");
        panel.warning("Warning 1");
        panel.error("Error 1");
        panel.info("Info 2");

        // No filter - all messages
        let filtered = panel.filtered_messages();
        assert_eq!(filtered.len(), 4);

        // Filter by error
        panel.set_filter(Some(Severity::Error));
        let filtered = panel.filtered_messages();
        assert_eq!(filtered.len(), 1);

        // Filter by info
        panel.set_filter(Some(Severity::Info));
        let filtered = panel.filtered_messages();
        assert_eq!(filtered.len(), 2);

        // Back to no filter
        panel.set_filter(None);
        let filtered = panel.filtered_messages();
        assert_eq!(filtered.len(), 4);
    }

    #[test]
    fn test_count_by_severity() {
        let mut panel = ConsolePanel::new();
        panel.info("Info 1");
        panel.info("Info 2");
        panel.warning("Warning 1");
        panel.error("Error 1");

        assert_eq!(panel.count_by_severity(Severity::Info), 2);
        assert_eq!(panel.count_by_severity(Severity::Warning), 1);
        assert_eq!(panel.count_by_severity(Severity::Error), 1);
    }

    #[test]
    fn test_max_messages() {
        let mut panel = ConsolePanel::new();
        panel.max_messages = 5;

        // Add more than max
        for i in 0..10 {
            panel.info(format!("Message {}", i));
        }

        // Should keep only the last 5
        assert_eq!(panel.messages.len(), 5);
        assert_eq!(panel.messages[0].message, "Message 5");
        assert_eq!(panel.messages[4].message, "Message 9");
    }

    #[test]
    fn test_from_error() {
        let error = Error::validation("Test validation error");
        let msg = ConsoleMessage::from(error);

        assert_eq!(msg.severity, Severity::Error);
        assert!(msg.message.contains("Validation"));
        assert!(msg.message.contains("Test validation error"));
        assert!(msg.details.is_some());
    }

    #[test]
    fn test_add_error() {
        let mut panel = ConsolePanel::new();
        let error = Error::out_of_memory("OOM test");

        panel.add_error(error);
        assert_eq!(panel.messages.len(), 1);
        assert_eq!(panel.messages[0].severity, Severity::Error);
    }

    // GUI Interaction Tests - Simulating User Workflows

    #[test]
    fn test_gui_interaction_typical_user_workflow() {
        let mut panel = ConsolePanel::new();
        
        // User starts app, sees initial messages
        panel.info("WebGPU initialized");
        panel.info("Adapter found: NVIDIA GeForce RTX");
        assert_eq!(panel.messages.len(), 2);
        
        // User makes a mistake, sees error
        panel.error("Shader compilation failed");
        assert_eq!(panel.messages.len(), 3);
        assert_eq!(panel.count_by_severity(Severity::Error), 1);
        
        // User toggles filter to show only errors
        panel.set_filter(Some(Severity::Error));
        let filtered = panel.filtered_messages();
        assert_eq!(filtered.len(), 1);
        
        // User fixes error, sees success message
        panel.info("Shader compiled successfully");
        
        // User clears filter to see all messages
        panel.set_filter(None);
        let filtered = panel.filtered_messages();
        assert_eq!(filtered.len(), 4);
        
        // User clears console
        panel.clear();
        assert_eq!(panel.messages.len(), 0);
    }

    #[test]
    fn test_gui_interaction_filter_cycling() {
        let mut panel = ConsolePanel::new();
        
        // Add different message types
        panel.info("Info message 1");
        panel.warning("Warning message 1");
        panel.error("Error message 1");
        panel.info("Info message 2");
        panel.warning("Warning message 2");
        
        // User cycles through filters
        
        // Show all
        panel.set_filter(None);
        assert_eq!(panel.filtered_messages().len(), 5);
        
        // Show only info
        panel.set_filter(Some(Severity::Info));
        assert_eq!(panel.filtered_messages().len(), 2);
        
        // Show only warnings
        panel.set_filter(Some(Severity::Warning));
        assert_eq!(panel.filtered_messages().len(), 2);
        
        // Show only errors
        panel.set_filter(Some(Severity::Error));
        assert_eq!(panel.filtered_messages().len(), 1);
        
        // Back to all
        panel.set_filter(None);
        assert_eq!(panel.filtered_messages().len(), 5);
    }

    #[test]
    fn test_gui_interaction_message_selection() {
        let mut panel = ConsolePanel::new();
        
        panel.info("Message 1");
        panel.warning("Message 2");
        panel.error("Message 3");
        
        // Verify messages are stored in order
        assert_eq!(panel.messages.len(), 3);
        assert_eq!(panel.messages[0].severity, Severity::Info);
        assert_eq!(panel.messages[1].severity, Severity::Warning);
        assert_eq!(panel.messages[2].severity, Severity::Error);
    }

    #[test]
    fn test_gui_interaction_message_overflow() {
        let mut panel = ConsolePanel::new();
        panel.max_messages = 3;
        
        // User performs actions that generate many messages
        panel.info("Message 1");
        panel.info("Message 2");
        panel.info("Message 3");
        
        // Panel should have 3 messages
        assert_eq!(panel.messages.len(), 3);
        
        // Add another message, oldest should be removed
        panel.info("Message 4");
        assert_eq!(panel.messages.len(), 3);
        assert_eq!(panel.messages[0].message, "Message 2");
        assert_eq!(panel.messages[2].message, "Message 4");
    }

    #[test]
    fn test_gui_interaction_clear_with_filter() {
        let mut panel = ConsolePanel::new();
        
        // Add messages
        panel.info("Info");
        panel.warning("Warning");
        panel.error("Error");
        
        // User sets filter
        panel.set_filter(Some(Severity::Error));
        assert_eq!(panel.filtered_messages().len(), 1);
        
        // User clears console
        panel.clear();
        
        // Filter should still be set but no messages
        assert_eq!(panel.messages.len(), 0);
        assert_eq!(panel.filtered_messages().len(), 0);
        
        // Add new error
        panel.error("New error");
        
        // Should be filtered correctly
        assert_eq!(panel.filtered_messages().len(), 1);
    }

    #[test]
    fn test_gui_interaction_multiple_errors_workflow() {
        let mut panel = ConsolePanel::new();
        
        // Simulate a user encountering multiple errors
        panel.error("Validation error in shader");
        panel.error("Buffer size too small");
        panel.warning("Performance warning: Large texture");
        panel.error("Pipeline creation failed");
        
        // User filters to see only errors
        panel.set_filter(Some(Severity::Error));
        let errors = panel.filtered_messages();
        assert_eq!(errors.len(), 3);
        
        // Verify all are errors
        for (_, error) in &errors {
            assert_eq!(error.severity, Severity::Error);
        }
        
        // User clears errors to start fresh
        panel.clear();
        assert_eq!(panel.messages.len(), 0);
    }
}
