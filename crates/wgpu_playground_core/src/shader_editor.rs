/// WGSL Shader Editor with syntax highlighting, line numbers, and compilation support
use crate::shader::ShaderModule;
use crate::shader_watcher::ShaderWatcher;

/// Represents a shader compilation result
#[derive(Debug, Clone, Default)]
pub enum CompilationResult {
    /// Shader compiled successfully
    Success,
    /// Shader compilation failed with error message
    Error(String),
    /// No compilation attempted yet
    #[default]
    NotCompiled,
}

/// WGSL Shader Editor component
pub struct ShaderEditor {
    /// Current shader source code
    source_code: String,
    /// Label for the shader
    label: String,
    /// Compilation result
    compilation_result: CompilationResult,
    /// File path for loading/saving
    file_path: String,
    /// Whether to show line numbers
    show_line_numbers: bool,
    /// Shader watcher for hot reload (optional)
    shader_watcher: Option<ShaderWatcher>,
    /// Whether hot reload is enabled
    hot_reload_enabled: bool,
}

impl Default for ShaderEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl ShaderEditor {
    /// Create a new shader editor with default example code
    pub fn new() -> Self {
        let shader_watcher = match ShaderWatcher::new() {
            Ok(watcher) => {
                log::info!("Shader hot reload initialized successfully");
                Some(watcher)
            }
            Err(e) => {
                log::warn!("Failed to initialize shader watcher: {}", e);
                None
            }
        };

        Self {
            source_code: Self::default_shader_code(),
            label: "shader_editor".to_string(),
            compilation_result: CompilationResult::NotCompiled,
            file_path: String::new(),
            show_line_numbers: true,
            shader_watcher,
            hot_reload_enabled: true,
        }
    }

    /// Get default example shader code
    fn default_shader_code() -> String {
        r#"// WGSL Shader Example
@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    // Create a simple triangle
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 0.5),
        vec2<f32>(-0.5, -0.5),
        vec2<f32>(0.5, -0.5)
    );
    let pos = positions[vertex_index];
    return vec4<f32>(pos, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.5, 0.0, 1.0); // Orange color
}
"#
        .to_string()
    }

    /// Load shader from file
    pub fn load_from_file(&mut self, filename: &str) {
        self.file_path = filename.to_string();
        match crate::assets::load_shader(filename) {
            Ok(code) => {
                self.source_code = code;
                self.compilation_result = CompilationResult::NotCompiled;
            }
            Err(e) => {
                self.compilation_result =
                    CompilationResult::Error(format!("Failed to load file: {}", e));
            }
        }
    }

    /// Get the current source code
    pub fn source_code(&self) -> &str {
        &self.source_code
    }

    /// Set the source code
    pub fn set_source_code(&mut self, code: String) {
        self.source_code = code;
        self.compilation_result = CompilationResult::NotCompiled;
    }

    /// Get the compilation result
    pub fn compilation_result(&self) -> &CompilationResult {
        &self.compilation_result
    }

    /// Compile the current shader
    pub fn compile(&mut self, device: &wgpu::Device) {
        // Try to create a shader module
        match ShaderModule::from_source(&self.source_code, Some(&self.label)) {
            Ok(shader) => {
                // Attempt to compile with wgpu
                // Note: wgpu's create_module does validation internally
                let _module = shader.create_module(device);
                self.compilation_result = CompilationResult::Success;
            }
            Err(e) => {
                self.compilation_result = CompilationResult::Error(format!("{}", e));
            }
        }
    }

    /// Validate shader syntax (compilation without creating module)
    pub fn validate(&mut self) -> bool {
        match ShaderModule::from_source(&self.source_code, Some(&self.label)) {
            Ok(_) => {
                // Basic validation passed
                true
            }
            Err(e) => {
                self.compilation_result = CompilationResult::Error(format!("{}", e));
                false
            }
        }
    }

    /// Render the shader editor UI
    pub fn ui(&mut self, ui: &mut egui::Ui, device: Option<&wgpu::Device>) {
        // Check for shader file changes if hot reload is enabled
        if self.hot_reload_enabled && !self.file_path.is_empty() {
            if let Some(watcher) = &self.shader_watcher {
                for event in watcher.poll_all() {
                    // Check if the changed file matches our current file
                    // Compare just the filename, as file_path may just be a filename or a full path
                    if event.filename == self.file_path
                        || std::path::Path::new(&self.file_path)
                            .file_name()
                            .and_then(|n| n.to_str())
                            == Some(&event.filename)
                    {
                        log::info!(
                            "Hot reload: Shader file '{}' changed, reloading...",
                            event.filename
                        );
                        let path = self.file_path.clone();
                        self.load_from_file(&path);
                        ui.ctx().request_repaint(); // Request UI repaint
                    }
                }
            }
        }

        ui.heading("📝 WGSL Shader Editor");
        ui.separator();

        // Controls row
        ui.horizontal(|ui| {
            // Label input
            ui.label("Label:");
            ui.text_edit_singleline(&mut self.label);

            ui.separator();

            // File path input
            ui.label("File:");
            ui.text_edit_singleline(&mut self.file_path);

            if ui.button("📁 Load").clicked() && !self.file_path.is_empty() {
                let path = self.file_path.clone();
                self.load_from_file(&path);
            }

            // Load example button
            if ui.button("📚 Load Example").clicked() {
                self.load_from_file("example.wgsl");
                self.file_path = "example.wgsl".to_string();
            }

            ui.separator();

            // Compile button
            if let Some(dev) = device {
                if ui.button("⚙️ Compile").clicked() {
                    self.compile(dev);
                }
            } else {
                ui.add_enabled(false, egui::Button::new("⚙️ Compile (No Device)"));
            }

            // Reset button
            if ui.button("🔄 Reset").clicked() {
                self.source_code = Self::default_shader_code();
                self.compilation_result = CompilationResult::NotCompiled;
            }
        });

        ui.add_space(10.0);

        // Compilation result display
        match &self.compilation_result {
            CompilationResult::Success => {
                ui.colored_label(egui::Color32::GREEN, "✅ Compilation successful!");
            }
            CompilationResult::Error(msg) => {
                ui.colored_label(egui::Color32::RED, format!("❌ Compilation error: {}", msg));
            }
            CompilationResult::NotCompiled => {
                ui.label("ℹ️ Not compiled yet. Click 'Compile' to validate your shader.");
            }
        }

        ui.add_space(10.0);
        ui.separator();

        // Tips and info
        ui.horizontal(|ui| {
            ui.label("💡 Tips:");
            ui.label("• Use '@vertex' and '@fragment' for render shaders");
            ui.label("• Use '@compute' for compute shaders");
            ui.label("• Press Compile to validate syntax");
        });

        ui.add_space(10.0);
        ui.separator();

        // Shader code editor
        ui.label("Shader Code:");

        egui::ScrollArea::vertical()
            .max_height(500.0)
            .show(ui, |ui| {
                if self.show_line_numbers {
                    self.render_with_line_numbers(ui);
                } else {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.source_code)
                            .code_editor()
                            .desired_width(f32::INFINITY)
                            .desired_rows(20),
                    );
                }
            });

        ui.add_space(10.0);

        // Options
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.show_line_numbers, "Show line numbers");

            // Hot reload toggle
            if self.shader_watcher.is_some() {
                ui.separator();
                let hot_reload_label = if self.hot_reload_enabled {
                    "🔥 Hot Reload: ON"
                } else {
                    "🔥 Hot Reload: OFF"
                };
                if ui
                    .checkbox(&mut self.hot_reload_enabled, hot_reload_label)
                    .changed()
                {
                    if self.hot_reload_enabled {
                        log::info!("Hot reload enabled");
                    } else {
                        log::info!("Hot reload disabled");
                    }
                }
            }
        });
    }

    /// Render editor with line numbers
    fn render_with_line_numbers(&mut self, ui: &mut egui::Ui) {
        // Split into lines
        let lines: Vec<&str> = self.source_code.lines().collect();
        let line_count = lines.len();
        let line_number_width = (line_count.to_string().len() as f32) * 8.0 + 10.0;

        ui.horizontal(|ui| {
            // Line numbers column
            ui.vertical(|ui| {
                ui.set_width(line_number_width);
                ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);

                let mut line_numbers_text = String::new();
                for i in 1..=line_count {
                    line_numbers_text.push_str(&format!("{}\n", i));
                }

                ui.add(
                    egui::TextEdit::multiline(&mut line_numbers_text)
                        .code_editor()
                        .interactive(false)
                        .desired_width(line_number_width),
                );
            });

            ui.separator();

            // Code editor column
            ui.add(
                egui::TextEdit::multiline(&mut self.source_code)
                    .code_editor()
                    .desired_width(f32::INFINITY)
                    .desired_rows(20),
            );
        });
    }

    /// Apply syntax highlighting to the code (basic implementation)
    ///
    /// NOTE: This method is currently unused but prepared for future enhanced
    /// syntax highlighting feature. It will be integrated when we implement
    /// colored text rendering in the editor.
    ///
    /// TODO(future): Integrate this into the editor's rendering to display
    /// colored syntax highlighting instead of plain monospace text.
    #[allow(dead_code)]
    fn highlight_wgsl(&self, text: &str) -> Vec<(String, egui::Color32)> {
        // WGSL keywords
        let keywords = [
            "fn",
            "struct",
            "var",
            "let",
            "const",
            "return",
            "if",
            "else",
            "for",
            "while",
            "break",
            "continue",
            "switch",
            "case",
            "default",
            "loop",
            "continuing",
            "@vertex",
            "@fragment",
            "@compute",
            "@group",
            "@binding",
            "@location",
            "@builtin",
            "@workgroup_size",
            "@stage",
            "@align",
            "@size",
            "@interpolate",
        ];

        // WGSL types
        let types = [
            "bool",
            "i32",
            "u32",
            "f32",
            "f16",
            "vec2",
            "vec3",
            "vec4",
            "mat2x2",
            "mat2x3",
            "mat2x4",
            "mat3x2",
            "mat3x3",
            "mat3x4",
            "mat4x2",
            "mat4x3",
            "mat4x4",
            "array",
            "ptr",
            "atomic",
            "texture_1d",
            "texture_2d",
            "texture_2d_array",
            "texture_3d",
            "texture_cube",
            "texture_cube_array",
            "texture_multisampled_2d",
            "texture_depth_2d",
            "texture_depth_2d_array",
            "texture_depth_cube",
            "texture_depth_cube_array",
            "texture_depth_multisampled_2d",
            "texture_storage_1d",
            "texture_storage_2d",
            "texture_storage_2d_array",
            "texture_storage_3d",
            "sampler",
            "sampler_comparison",
        ];

        let mut result = Vec::new();
        let mut current_word = String::new();
        let mut in_comment = false;

        for ch in text.chars() {
            if ch == '/' && !in_comment {
                in_comment = true;
                if !current_word.is_empty() {
                    let color = if keywords.contains(&current_word.as_str()) {
                        egui::Color32::from_rgb(200, 100, 200) // Purple for keywords
                    } else if types.contains(&current_word.as_str()) {
                        egui::Color32::from_rgb(100, 200, 200) // Cyan for types
                    } else {
                        egui::Color32::WHITE
                    };
                    result.push((current_word.clone(), color));
                    current_word.clear();
                }
                current_word.push(ch);
            } else if ch == '\n' && in_comment {
                result.push((current_word.clone(), egui::Color32::GRAY));
                current_word.clear();
                in_comment = false;
                current_word.push(ch);
            } else if ch.is_whitespace()
                || ch == '('
                || ch == ')'
                || ch == '{'
                || ch == '}'
                || ch == ';'
                || ch == ','
            {
                if !current_word.is_empty() {
                    let color = if in_comment {
                        egui::Color32::GRAY
                    } else if keywords.contains(&current_word.as_str()) {
                        egui::Color32::from_rgb(200, 100, 200)
                    } else if types.contains(&current_word.as_str()) {
                        egui::Color32::from_rgb(100, 200, 200)
                    } else {
                        egui::Color32::WHITE
                    };
                    result.push((current_word.clone(), color));
                    current_word.clear();
                }
                current_word.push(ch);
                result.push((current_word.clone(), egui::Color32::WHITE));
                current_word.clear();
            } else {
                current_word.push(ch);
            }
        }

        if !current_word.is_empty() {
            let color = if in_comment {
                egui::Color32::GRAY
            } else if keywords.contains(&current_word.as_str()) {
                egui::Color32::from_rgb(200, 100, 200)
            } else if types.contains(&current_word.as_str()) {
                egui::Color32::from_rgb(100, 200, 200)
            } else {
                egui::Color32::WHITE
            };
            result.push((current_word, color));
        }

        result
    }

    /// Export the current state to a serializable format
    pub fn export_state(&self) -> crate::state::ShaderEditorState {
        crate::state::ShaderEditorState {
            source_code: self.source_code.clone(),
            label: self.label.clone(),
            file_path: self.file_path.clone(),
        }
    }

    /// Import state from a serializable format
    pub fn import_state(&mut self, state: &crate::state::ShaderEditorState) {
        self.source_code = state.source_code.clone();
        self.label = state.label.clone();
        self.file_path = state.file_path.clone();
        self.compilation_result = CompilationResult::NotCompiled;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shader_editor_new() {
        let editor = ShaderEditor::new();
        assert!(!editor.source_code.is_empty());
        assert_eq!(editor.label, "shader_editor");
        assert!(matches!(
            editor.compilation_result,
            CompilationResult::NotCompiled
        ));
        assert!(editor.show_line_numbers);
    }

    #[test]
    fn test_shader_editor_default() {
        let editor = ShaderEditor::default();
        assert!(!editor.source_code.is_empty());
    }

    #[test]
    fn test_set_source_code() {
        let mut editor = ShaderEditor::new();
        let test_code = "@vertex fn test() {}";
        editor.set_source_code(test_code.to_string());
        assert_eq!(editor.source_code(), test_code);
        assert!(matches!(
            editor.compilation_result,
            CompilationResult::NotCompiled
        ));
    }

    #[test]
    fn test_validate_valid_shader() {
        let mut editor = ShaderEditor::new();
        editor.set_source_code(
            "@vertex fn vs_main() -> @builtin(position) vec4<f32> { return vec4<f32>(0.0); }"
                .to_string(),
        );
        assert!(editor.validate());
    }

    #[test]
    fn test_validate_invalid_shader() {
        let mut editor = ShaderEditor::new();
        editor.set_source_code("".to_string());
        assert!(!editor.validate());
        assert!(matches!(
            editor.compilation_result,
            CompilationResult::Error(_)
        ));
    }

    #[test]
    fn test_default_shader_code() {
        let code = ShaderEditor::default_shader_code();
        assert!(code.contains("@vertex"));
        assert!(code.contains("@fragment"));
        assert!(code.contains("vs_main"));
        assert!(code.contains("fs_main"));
    }

    #[test]
    fn test_compilation_result_default() {
        let result = CompilationResult::default();
        assert!(matches!(result, CompilationResult::NotCompiled));
    }

    #[test]
    fn test_highlight_wgsl_keywords() {
        let editor = ShaderEditor::new();
        let highlighted = editor.highlight_wgsl("fn main() {}");

        // Should have "fn", " ", "main", "(", ")", " ", "{", "}"
        assert!(!highlighted.is_empty());

        // "fn" should be highlighted as a keyword
        let fn_token = highlighted.iter().find(|(text, _)| text == "fn");
        assert!(fn_token.is_some());
    }

    #[test]
    fn test_highlight_wgsl_types() {
        let editor = ShaderEditor::new();
        let highlighted = editor.highlight_wgsl("var x: vec4<f32>;");

        // Check that we get some tokens back
        assert!(!highlighted.is_empty());

        // The tokens should include our code
        let all_text: String = highlighted.iter().map(|(text, _)| text.as_str()).collect();
        assert!(all_text.contains("vec4"));
        assert!(all_text.contains("f32"));
    }
}
