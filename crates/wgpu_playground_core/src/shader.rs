use std::fmt;

/// Shader source can come from either a file or an inline string
#[derive(Debug, Clone)]
pub enum ShaderSource {
    /// Load shader from a file path
    File(String),
    /// Use inline WGSL source code
    Inline(String),
}

/// Errors that can occur during shader module operations
#[derive(Debug)]
pub enum ShaderError {
    /// Failed to load shader file
    LoadError(std::io::Error),
    /// Invalid shader source
    InvalidSource(String),
}

impl fmt::Display for ShaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShaderError::LoadError(err) => write!(f, "Failed to load shader: {}", err),
            ShaderError::InvalidSource(msg) => write!(f, "Invalid shader source: {}", msg),
        }
    }
}

impl std::error::Error for ShaderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ShaderError::LoadError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ShaderError {
    fn from(err: std::io::Error) -> Self {
        ShaderError::LoadError(err)
    }
}

/// Represents a WGSL shader module with its source code
#[derive(Debug, Clone)]
pub struct ShaderModule {
    /// The WGSL source code
    source: String,
    /// Optional label for debugging
    label: Option<String>,
    /// Track the original source type for reloading
    source_type: ShaderSource,
}

impl ShaderModule {
    /// Create a new shader module from a source
    ///
    /// # Arguments
    /// * `source` - The shader source (file or inline)
    /// * `label` - Optional label for debugging
    ///
    /// # Returns
    /// A Result containing the ShaderModule or a ShaderError
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::shader::{ShaderModule, ShaderSource};
    ///
    /// // From inline source
    /// let shader = ShaderModule::new(
    ///     ShaderSource::Inline("@vertex fn main() {}".to_string()),
    ///     Some("my_shader")
    /// ).unwrap();
    ///
    /// // From file
    /// let shader = ShaderModule::new(
    ///     ShaderSource::File("example.wgsl".to_string()),
    ///     Some("example_shader")
    /// );
    /// ```
    pub fn new(source: ShaderSource, label: Option<&str>) -> Result<Self, ShaderError> {
        log::debug!("Loading shader: label={:?}", label);
        let source_code = match &source {
            ShaderSource::Inline(code) => {
                if code.trim().is_empty() {
                    log::error!("Shader source is empty");
                    return Err(ShaderError::InvalidSource(
                        "Shader source cannot be empty".to_string(),
                    ));
                }
                log::trace!("Using inline shader source ({} bytes)", code.len());
                code.clone()
            }
            ShaderSource::File(filename) => {
                log::debug!("Loading shader from file: {}", filename);
                match crate::assets::load_shader(filename) {
                    Ok(code) => {
                        log::trace!("Loaded shader from file ({} bytes)", code.len());
                        code
                    }
                    Err(e) => {
                        log::error!("Failed to load shader from file '{}': {}", filename, e);
                        return Err(e.into());
                    }
                }
            }
        };

        Ok(Self {
            source: source_code,
            label: label.map(String::from),
            source_type: source,
        })
    }

    /// Create a shader module from inline WGSL source
    ///
    /// # Arguments
    /// * `source` - The WGSL source code
    /// * `label` - Optional label for debugging
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::shader::ShaderModule;
    ///
    /// let shader = ShaderModule::from_source(
    ///     "@vertex fn main() -> @builtin(position) vec4<f32> { return vec4<f32>(0.0); }",
    ///     Some("vertex_shader")
    /// ).unwrap();
    /// ```
    pub fn from_source(source: &str, label: Option<&str>) -> Result<Self, ShaderError> {
        Self::new(ShaderSource::Inline(source.to_string()), label)
    }

    /// Create a shader module from a file
    ///
    /// # Arguments
    /// * `filename` - The name of the shader file (e.g., "example.wgsl")
    /// * `label` - Optional label for debugging
    ///
    /// # Examples
    /// ```no_run
    /// use wgpu_playground_core::shader::ShaderModule;
    ///
    /// let shader = ShaderModule::from_file("example.wgsl", Some("example")).unwrap();
    /// ```
    pub fn from_file(filename: &str, label: Option<&str>) -> Result<Self, ShaderError> {
        Self::new(ShaderSource::File(filename.to_string()), label)
    }

    /// Get the shader source code
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Get the shader label
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Create a wgpu shader module from this shader
    ///
    /// This method compiles and validates the shader. If the shader contains
    /// compilation errors, wgpu will panic in debug mode or log errors in release mode.
    ///
    /// # Arguments
    /// * `device` - The wgpu device to create the module on
    ///
    /// # Returns
    /// A wgpu::ShaderModule ready for use in pipelines
    ///
    /// # Examples
    /// ```no_run
    /// use wgpu_playground_core::shader::ShaderModule;
    /// # async fn example(device: &wgpu::Device) {
    /// let shader = ShaderModule::from_source(
    ///     "@vertex fn main() -> @builtin(position) vec4<f32> { return vec4<f32>(0.0); }",
    ///     Some("my_shader")
    /// ).unwrap();
    ///
    /// let wgpu_module = shader.create_module(device);
    /// # }
    /// ```
    pub fn create_module(&self, device: &wgpu::Device) -> wgpu::ShaderModule {
        log::debug!("Creating shader module: label={:?}", self.label);
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: self.label.as_deref(),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(&self.source)),
        });
        log::trace!("Shader module created successfully");
        module
    }

    /// Reload the shader source from its original source
    ///
    /// For file-based shaders, this reloads from disk.
    /// For inline shaders, this is a no-op.
    ///
    /// # Returns
    /// Ok(true) if the shader was reloaded, Ok(false) if no reload was needed,
    /// or Err if reloading failed
    ///
    /// # Examples
    /// ```no_run
    /// use wgpu_playground_core::shader::ShaderModule;
    ///
    /// let mut shader = ShaderModule::from_file("example.wgsl", Some("example")).unwrap();
    /// // ... shader file is modified ...
    /// shader.reload().unwrap();
    /// ```
    pub fn reload(&mut self) -> Result<bool, ShaderError> {
        match &self.source_type {
            ShaderSource::File(filename) => {
                log::info!("Reloading shader from file: {}", filename);
                match crate::assets::load_shader(filename) {
                    Ok(new_source) => {
                        if new_source.trim().is_empty() {
                            log::error!("Reloaded shader source is empty");
                            return Err(ShaderError::InvalidSource(
                                "Shader source cannot be empty".to_string(),
                            ));
                        }

                        if new_source != self.source {
                            log::info!(
                                "Shader source changed, updating ({} bytes)",
                                new_source.len()
                            );
                            self.source = new_source;
                            Ok(true)
                        } else {
                            log::debug!("Shader source unchanged");
                            Ok(false)
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to reload shader from file '{}': {}", filename, e);
                        Err(e.into())
                    }
                }
            }
            ShaderSource::Inline(_) => {
                log::debug!("Inline shader, no reload needed");
                Ok(false)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shader_from_inline_source() {
        let source = "@vertex fn main() -> @builtin(position) vec4<f32> { return vec4<f32>(0.0); }";
        let shader = ShaderModule::from_source(source, Some("test_shader"));

        assert!(shader.is_ok());
        let shader = shader.unwrap();
        assert_eq!(shader.source(), source);
        assert_eq!(shader.label(), Some("test_shader"));
    }

    #[test]
    fn test_shader_from_inline_source_no_label() {
        let source = "@fragment fn main() -> @location(0) vec4<f32> { return vec4<f32>(1.0); }";
        let shader = ShaderModule::from_source(source, None);

        assert!(shader.is_ok());
        let shader = shader.unwrap();
        assert_eq!(shader.source(), source);
        assert_eq!(shader.label(), None);
    }

    #[test]
    fn test_shader_from_empty_source() {
        let shader = ShaderModule::from_source("", Some("empty"));
        assert!(shader.is_err());

        let shader = ShaderModule::from_source("   ", Some("whitespace"));
        assert!(shader.is_err());
    }

    #[test]
    fn test_shader_source_enum() {
        let inline = ShaderSource::Inline("test code".to_string());
        let file = ShaderSource::File("test.wgsl".to_string());

        match inline {
            ShaderSource::Inline(code) => assert_eq!(code, "test code"),
            _ => panic!("Expected inline source"),
        }

        match file {
            ShaderSource::File(name) => assert_eq!(name, "test.wgsl"),
            _ => panic!("Expected file source"),
        }
    }

    #[test]
    fn test_shader_error_display() {
        let err = ShaderError::InvalidSource("test error".to_string());
        assert_eq!(err.to_string(), "Invalid shader source: test error");

        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err = ShaderError::LoadError(io_err);
        assert!(err.to_string().contains("Failed to load shader"));
    }

    #[test]
    fn test_shader_new_with_source_enum() {
        let inline = ShaderSource::Inline("@vertex fn main() {}".to_string());
        let shader = ShaderModule::new(inline, Some("test"));
        assert!(shader.is_ok());
    }

    // Note: File loading tests would require the actual file to exist
    // Integration test in a separate file can test this with the example.wgsl file
}
