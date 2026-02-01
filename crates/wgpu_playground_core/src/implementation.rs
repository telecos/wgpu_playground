/// WebGPU implementation type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebGPUImplementation {
    /// wgpu-rs implementation (Rust-based, used by Firefox)
    Wgpu,
    /// Dawn implementation (C++-based, used by Chromium)
    #[cfg(feature = "dawn")]
    Dawn,
}

impl WebGPUImplementation {
    /// Get the current implementation being used
    ///
    /// This checks the WEBGPU_IMPL environment variable first, then falls back to
    /// the compile-time default based on feature flags.
    ///
    /// # Environment Variable
    /// Set `WEBGPU_IMPL=dawn` or `WEBGPU_IMPL=wgpu` to override the default.
    /// The environment variable is only respected if the requested implementation
    /// was compiled in via feature flags.
    pub fn current() -> Self {
        // Check environment variable for runtime override
        if let Ok(impl_str) = std::env::var("WEBGPU_IMPL") {
            let impl_str = impl_str.to_lowercase();
            match impl_str.as_str() {
                "dawn" => {
                    #[cfg(feature = "dawn")]
                    {
                        log::info!("WEBGPU_IMPL=dawn: Using Dawn implementation");
                        return Self::Dawn;
                    }
                    #[cfg(not(feature = "dawn"))]
                    {
                        log::warn!(
                            "WEBGPU_IMPL=dawn requested but Dawn feature not compiled in. \
                            Compile with --features dawn to enable. Falling back to wgpu."
                        );
                    }
                }
                "wgpu" => {
                    log::info!("WEBGPU_IMPL=wgpu: Using wgpu implementation");
                    return Self::Wgpu;
                }
                _ => {
                    log::warn!(
                        "Unknown WEBGPU_IMPL value: '{}'. Valid values are 'dawn' or 'wgpu'. \
                        Using default.",
                        impl_str
                    );
                }
            }
        }

        // Fall back to compile-time default
        #[cfg(feature = "dawn")]
        {
            log::info!("Using Dawn implementation (compile-time default)");
            Self::Dawn
        }
        #[cfg(not(feature = "dawn"))]
        {
            log::info!("Using wgpu implementation (compile-time default)");
            Self::Wgpu
        }
    }

    /// Get the implementation name as a string
    pub fn name(&self) -> &'static str {
        match self {
            Self::Wgpu => "wgpu",
            #[cfg(feature = "dawn")]
            Self::Dawn => "Dawn",
        }
    }

    /// Get the implementation description
    pub fn description(&self) -> &'static str {
        match self {
            Self::Wgpu => "wgpu-rs (Rust implementation, used by Firefox)",
            #[cfg(feature = "dawn")]
            Self::Dawn => "Dawn (C++ implementation, used by Chromium)",
        }
    }

    /// Get the implementation website/repository URL
    pub fn url(&self) -> &'static str {
        match self {
            Self::Wgpu => "https://github.com/gfx-rs/wgpu",
            #[cfg(feature = "dawn")]
            Self::Dawn => "https://dawn.googlesource.com/dawn",
        }
    }

    /// Check if Dawn is available (compiled in)
    pub fn is_dawn_available() -> bool {
        cfg!(feature = "dawn")
    }

    /// Get all available implementations
    pub fn available_implementations() -> Vec<Self> {
        #[cfg(feature = "dawn")]
        {
            vec![Self::Wgpu, Self::Dawn]
        }
        #[cfg(not(feature = "dawn"))]
        {
            vec![Self::Wgpu]
        }
    }

    /// Check if this implementation is fully integrated or a placeholder
    ///
    /// Returns true if the implementation has native integration.
    /// Returns false if it's a placeholder that uses wgpu underneath.
    pub fn is_native(&self) -> bool {
        match self {
            Self::Wgpu => true,
            #[cfg(feature = "dawn")]
            Self::Dawn => false, // Dawn build infrastructure exists, but runtime integration pending
        }
    }

    /// Get a status message about the implementation
    pub fn status_message(&self) -> &'static str {
        match self {
            Self::Wgpu => "Native wgpu implementation",
            #[cfg(feature = "dawn")]
            Self::Dawn => "Placeholder: Build infrastructure ready, runtime integration pending",
        }
    }

    /// Get a comma-separated list of available implementation names
    pub fn available_implementations_list() -> String {
        Self::available_implementations()
            .iter()
            .map(|impl_type| impl_type.name())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl std::fmt::Display for WebGPUImplementation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_current_implementation() {
        // Clear any environment variable first
        std::env::remove_var("WEBGPU_IMPL");
        let impl_type = WebGPUImplementation::current();
        #[cfg(feature = "dawn")]
        assert_eq!(impl_type, WebGPUImplementation::Dawn);
        #[cfg(not(feature = "dawn"))]
        assert_eq!(impl_type, WebGPUImplementation::Wgpu);
    }

    #[test]
    fn test_implementation_name() {
        assert_eq!(WebGPUImplementation::Wgpu.name(), "wgpu");
    }

    #[test]
    fn test_implementation_description() {
        let desc = WebGPUImplementation::Wgpu.description();
        assert!(desc.contains("wgpu-rs"));
        assert!(desc.contains("Firefox"));
    }

    #[test]
    fn test_implementation_url() {
        let url = WebGPUImplementation::Wgpu.url();
        assert!(url.contains("github.com"));
        assert!(url.contains("gfx-rs/wgpu"));
    }

    #[test]
    fn test_is_dawn_available() {
        #[cfg(feature = "dawn")]
        assert!(WebGPUImplementation::is_dawn_available());
        #[cfg(not(feature = "dawn"))]
        assert!(!WebGPUImplementation::is_dawn_available());
    }

    #[test]
    fn test_available_implementations() {
        let impls = WebGPUImplementation::available_implementations();
        assert!(!impls.is_empty());
        assert!(impls.contains(&WebGPUImplementation::Wgpu));
    }

    #[test]
    fn test_display_implementation() {
        let impl_type = WebGPUImplementation::Wgpu;
        assert_eq!(format!("{}", impl_type), "wgpu");
    }

    #[test]
    fn test_is_native() {
        assert!(WebGPUImplementation::Wgpu.is_native());
        #[cfg(feature = "dawn")]
        assert!(!WebGPUImplementation::Dawn.is_native());
    }

    #[test]
    fn test_status_message() {
        let status = WebGPUImplementation::Wgpu.status_message();
        assert!(status.contains("Native"));
        #[cfg(feature = "dawn")]
        {
            let status = WebGPUImplementation::Dawn.status_message();
            assert!(status.contains("Placeholder"));
        }
    }

    #[test]
    #[serial]
    fn test_environment_variable_override() {
        // Test wgpu selection
        std::env::set_var("WEBGPU_IMPL", "wgpu");
        let impl_type = WebGPUImplementation::current();
        assert_eq!(impl_type, WebGPUImplementation::Wgpu);
        std::env::remove_var("WEBGPU_IMPL");
    }

    #[test]
    #[serial]
    #[cfg(feature = "dawn")]
    fn test_environment_variable_dawn() {
        std::env::set_var("WEBGPU_IMPL", "dawn");
        let impl_type = WebGPUImplementation::current();
        assert_eq!(impl_type, WebGPUImplementation::Dawn);
        std::env::remove_var("WEBGPU_IMPL");
    }

    #[test]
    #[serial]
    fn test_environment_variable_invalid() {
        std::env::set_var("WEBGPU_IMPL", "invalid");
        let impl_type = WebGPUImplementation::current();
        // Should fall back to default
        #[cfg(feature = "dawn")]
        assert_eq!(impl_type, WebGPUImplementation::Dawn);
        #[cfg(not(feature = "dawn"))]
        assert_eq!(impl_type, WebGPUImplementation::Wgpu);
        std::env::remove_var("WEBGPU_IMPL");
    }

    #[test]
    fn test_available_implementations_contains_wgpu() {
        let impls = WebGPUImplementation::available_implementations();
        assert!(impls.contains(&WebGPUImplementation::Wgpu));
    }

    #[test]
    #[cfg(feature = "dawn")]
    fn test_available_implementations_contains_dawn() {
        let impls = WebGPUImplementation::available_implementations();
        assert!(impls.contains(&WebGPUImplementation::Dawn));
        assert_eq!(impls.len(), 2);
    }

    #[test]
    fn test_available_implementations_list() {
        let list = WebGPUImplementation::available_implementations_list();
        assert!(list.contains("wgpu"));
        #[cfg(feature = "dawn")]
        {
            assert!(list.contains("Dawn"));
            assert!(list.contains(", "));
        }
        #[cfg(not(feature = "dawn"))]
        {
            assert_eq!(list, "wgpu");
        }
    }
}
