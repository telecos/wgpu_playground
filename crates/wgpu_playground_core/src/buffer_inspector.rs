// Buffer inspector utilities for viewing GPU buffer contents

/// Format for displaying buffer data
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataFormat {
    /// Display as hexadecimal bytes
    Hex,
    /// Display as signed 32-bit integers
    Int32,
    /// Display as unsigned 32-bit integers
    Uint32,
    /// Display as 32-bit floating point
    Float32,
}

impl DataFormat {
    /// Get a human-readable name for the format
    pub fn as_str(&self) -> &'static str {
        match self {
            DataFormat::Hex => "Hex",
            DataFormat::Int32 => "Int32",
            DataFormat::Uint32 => "UInt32",
            DataFormat::Float32 => "Float32",
        }
    }

    /// Get all available formats
    pub fn all() -> &'static [DataFormat] {
        &[
            DataFormat::Hex,
            DataFormat::Int32,
            DataFormat::Uint32,
            DataFormat::Float32,
        ]
    }
}

/// Inspector for viewing GPU buffer contents
///
/// This utility allows reading and displaying the contents of GPU buffers
/// in various formats. It handles buffer mapping and provides a UI for
/// viewing the data.
pub struct BufferInspector {
    /// Currently loaded buffer data
    buffer_data: Vec<u8>,
    /// Format for displaying the data
    display_format: DataFormat,
    /// Number of bytes to display per row
    bytes_per_row: usize,
    /// Offset in bytes to start displaying from
    display_offset: usize,
    /// Maximum number of bytes to display
    max_display_bytes: usize,
    /// Whether data is currently being loaded
    is_loading: bool,
    /// Error message if loading failed
    error_message: Option<String>,
}

impl Default for BufferInspector {
    fn default() -> Self {
        Self::new()
    }
}

impl BufferInspector {
    /// Create a new buffer inspector
    pub fn new() -> Self {
        Self {
            buffer_data: Vec::new(),
            display_format: DataFormat::Hex,
            bytes_per_row: 16,
            display_offset: 0,
            max_display_bytes: 4096, // Show up to 4KB by default
            is_loading: false,
            error_message: None,
        }
    }

    /// Set the display format
    pub fn set_format(&mut self, format: DataFormat) {
        self.display_format = format;
    }

    /// Get the current display format
    pub fn format(&self) -> DataFormat {
        self.display_format
    }

    /// Set the number of bytes to display per row
    pub fn set_bytes_per_row(&mut self, bytes: usize) {
        self.bytes_per_row = bytes.max(1);
    }

    /// Get the currently loaded data
    pub fn data(&self) -> &[u8] {
        &self.buffer_data
    }

    /// Clear the loaded data
    pub fn clear(&mut self) {
        self.buffer_data.clear();
        self.error_message = None;
    }

    /// Load buffer data synchronously (for testing or when buffer is already mapped)
    ///
    /// # Arguments
    /// * `data` - The raw buffer data to load
    pub fn load_data(&mut self, data: Vec<u8>) {
        self.buffer_data = data;
        self.error_message = None;
        self.is_loading = false;
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

    /// Format data as hexadecimal
    fn format_hex(&self, data: &[u8]) -> String {
        let mut result = String::new();
        let display_data = &data[self.display_offset.min(data.len())..];
        let display_data = &display_data[..display_data.len().min(self.max_display_bytes)];

        for (i, chunk) in display_data.chunks(self.bytes_per_row).enumerate() {
            let offset = self.display_offset + i * self.bytes_per_row;
            result.push_str(&format!("{:08x}:  ", offset));

            for (j, byte) in chunk.iter().enumerate() {
                result.push_str(&format!("{:02x} ", byte));
                if (j + 1) % 8 == 0 && j + 1 < chunk.len() {
                    result.push(' ');
                }
            }

            // Add ASCII representation
            result.push_str("  |");
            for byte in chunk {
                if byte.is_ascii_graphic() || *byte == b' ' {
                    result.push(*byte as char);
                } else {
                    result.push('.');
                }
            }
            result.push_str("|\n");
        }

        result
    }

    /// Format data as 32-bit integers
    fn format_int32(&self, data: &[u8]) -> String {
        let mut result = String::new();
        let display_data = &data[self.display_offset.min(data.len())..];
        let display_data = &display_data[..display_data.len().min(self.max_display_bytes)];

        for (i, chunk) in display_data.chunks(4).enumerate() {
            if chunk.len() == 4 {
                let value = i32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                let offset = self.display_offset + i * 4;
                result.push_str(&format!("{:08x}: {:11}\n", offset, value));
            }
        }

        result
    }

    /// Format data as 32-bit unsigned integers
    fn format_uint32(&self, data: &[u8]) -> String {
        let mut result = String::new();
        let display_data = &data[self.display_offset.min(data.len())..];
        let display_data = &display_data[..display_data.len().min(self.max_display_bytes)];

        for (i, chunk) in display_data.chunks(4).enumerate() {
            if chunk.len() == 4 {
                let value = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                let offset = self.display_offset + i * 4;
                result.push_str(&format!("{:08x}: {:10}\n", offset, value));
            }
        }

        result
    }

    /// Format data as 32-bit floats
    fn format_float32(&self, data: &[u8]) -> String {
        let mut result = String::new();
        let display_data = &data[self.display_offset.min(data.len())..];
        let display_data = &display_data[..display_data.len().min(self.max_display_bytes)];

        for (i, chunk) in display_data.chunks(4).enumerate() {
            if chunk.len() == 4 {
                let value = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                let offset = self.display_offset + i * 4;
                result.push_str(&format!("{:08x}: {:11.6}\n", offset, value));
            }
        }

        result
    }

    /// Format the buffer data according to the current display format
    pub fn format_data(&self) -> String {
        if self.buffer_data.is_empty() {
            return "No data loaded".to_string();
        }

        match self.display_format {
            DataFormat::Hex => self.format_hex(&self.buffer_data),
            DataFormat::Int32 => self.format_int32(&self.buffer_data),
            DataFormat::Uint32 => self.format_uint32(&self.buffer_data),
            DataFormat::Float32 => self.format_float32(&self.buffer_data),
        }
    }

    /// Render the buffer inspector UI
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Buffer Inspector");
        ui.separator();

        // Format selection
        ui.horizontal(|ui| {
            ui.label("Display format:");
            for format in DataFormat::all() {
                if ui.selectable_label(self.display_format == *format, format.as_str()).clicked() {
                    self.display_format = *format;
                }
            }
        });

        ui.separator();

        // Display statistics
        if !self.buffer_data.is_empty() {
            ui.horizontal(|ui| {
                ui.label(format!("Buffer size: {} bytes", self.buffer_data.len()));
                ui.separator();
                ui.label(format!("Displaying: {}/{} bytes", 
                    self.max_display_bytes.min(self.buffer_data.len()),
                    self.buffer_data.len()));
            });
        }

        // Display error if any
        if let Some(error) = &self.error_message {
            ui.colored_label(egui::Color32::RED, format!("Error: {}", error));
            ui.separator();
        }

        // Display buffer contents
        if self.is_loading {
            ui.spinner();
            ui.label("Loading buffer data...");
        } else if !self.buffer_data.is_empty() {
            egui::ScrollArea::vertical()
                .max_height(400.0)
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.format_data().as_str())
                            .font(egui::TextStyle::Monospace)
                            .code_editor()
                            .desired_width(f32::INFINITY)
                    );
                });
        } else if self.error_message.is_none() {
            ui.label("Select a buffer from the Resource Inspector to view its contents");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_inspector_creation() {
        let inspector = BufferInspector::new();
        assert_eq!(inspector.data().len(), 0);
        assert_eq!(inspector.format(), DataFormat::Hex);
        assert!(!inspector.has_error());
    }

    #[test]
    fn test_load_data() {
        let mut inspector = BufferInspector::new();
        let data = vec![0x01, 0x02, 0x03, 0x04];
        inspector.load_data(data.clone());
        assert_eq!(inspector.data(), &data[..]);
        assert!(!inspector.has_error());
    }

    #[test]
    fn test_format_hex() {
        let mut inspector = BufferInspector::new();
        inspector.load_data(vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]); // "Hello"
        inspector.set_format(DataFormat::Hex);
        let formatted = inspector.format_data();
        assert!(formatted.contains("48 65 6c 6c 6f"));
    }

    #[test]
    fn test_format_int32() {
        let mut inspector = BufferInspector::new();
        // 42 as little-endian i32
        inspector.load_data(vec![42, 0, 0, 0]);
        inspector.set_format(DataFormat::Int32);
        let formatted = inspector.format_data();
        assert!(formatted.contains("42"));
    }

    #[test]
    fn test_format_uint32() {
        let mut inspector = BufferInspector::new();
        // 1000 as little-endian u32
        inspector.load_data(vec![0xe8, 0x03, 0, 0]);
        inspector.set_format(DataFormat::Uint32);
        let formatted = inspector.format_data();
        assert!(formatted.contains("1000"));
    }

    #[test]
    fn test_format_float32() {
        let mut inspector = BufferInspector::new();
        // 3.14159 as little-endian f32
        let bytes = 3.14159f32.to_le_bytes();
        inspector.load_data(bytes.to_vec());
        inspector.set_format(DataFormat::Float32);
        let formatted = inspector.format_data();
        assert!(formatted.contains("3.14"));
    }

    #[test]
    fn test_clear() {
        let mut inspector = BufferInspector::new();
        inspector.load_data(vec![1, 2, 3, 4]);
        assert_eq!(inspector.data().len(), 4);
        inspector.clear();
        assert_eq!(inspector.data().len(), 0);
    }

    #[test]
    fn test_set_error() {
        let mut inspector = BufferInspector::new();
        inspector.set_error("Test error".to_string());
        assert!(inspector.has_error());
        assert_eq!(inspector.error_message(), Some("Test error"));
    }

    #[test]
    fn test_empty_data_format() {
        let inspector = BufferInspector::new();
        let formatted = inspector.format_data();
        assert_eq!(formatted, "No data loaded");
    }
}
