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
    pub fn current() -> Self {
        #[cfg(feature = "dawn")]
        {
            Self::Dawn
        }
        #[cfg(not(feature = "dawn"))]
        {
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
        let impls = vec![Self::Wgpu];
        #[cfg(feature = "dawn")]
        {
            let mut impls = impls;
            impls.push(Self::Dawn);
            return impls;
        }
        #[cfg(not(feature = "dawn"))]
        impls
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

    #[test]
    fn test_current_implementation() {
        let impl_type = WebGPUImplementation::current();
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
        assert!(url.contains("github.com") || url.contains("googlesource.com"));
    }

    #[test]
    fn test_is_dawn_available() {
        // Dawn is not available by default (feature not enabled)
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
}
