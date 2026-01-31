use wgpu::{Adapter, Backends, Features, Instance, Limits, PowerPreference, RequestAdapterOptions};

/// Options for requesting a GPU adapter
///
/// Note: The `backends` field should be used when creating the Instance via
/// `create_instance_with_options()` or `create_instance()`. It does not affect
/// `request_adapter()` - the Instance must be created with the desired backends first.
#[derive(Debug, Clone)]
pub struct AdapterOptions {
    /// Power preference for adapter selection
    pub power_preference: PowerPreference,
    /// Whether to force the use of a fallback/software adapter
    pub force_fallback_adapter: bool,
    /// Backend(s) to use when creating the Instance (Vulkan, Metal, DX12, etc.)
    /// Use this with create_instance_with_options() to create an Instance with specific backends.
    pub backends: Backends,
}

impl Default for AdapterOptions {
    fn default() -> Self {
        Self {
            power_preference: PowerPreference::default(),
            force_fallback_adapter: false,
            backends: Backends::all(),
        }
    }
}

impl AdapterOptions {
    /// Create adapter options with high performance preference
    pub fn high_performance() -> Self {
        Self {
            power_preference: PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            backends: Backends::all(),
        }
    }

    /// Create adapter options with low power preference
    pub fn low_power() -> Self {
        Self {
            power_preference: PowerPreference::LowPower,
            force_fallback_adapter: false,
            backends: Backends::all(),
        }
    }

    /// Create adapter options for fallback/software rendering
    pub fn fallback() -> Self {
        Self {
            power_preference: PowerPreference::default(),
            force_fallback_adapter: true,
            backends: Backends::all(),
        }
    }

    /// Set power preference
    pub fn with_power_preference(mut self, power_preference: PowerPreference) -> Self {
        self.power_preference = power_preference;
        self
    }

    /// Set whether to force fallback adapter
    pub fn with_fallback_adapter(mut self, force_fallback: bool) -> Self {
        self.force_fallback_adapter = force_fallback;
        self
    }

    /// Set which backends to use
    pub fn with_backends(mut self, backends: Backends) -> Self {
        self.backends = backends;
        self
    }

    /// Create adapter options with a specific backend
    pub fn with_backend(backend: Backends) -> Self {
        Self {
            power_preference: PowerPreference::default(),
            force_fallback_adapter: false,
            backends: backend,
        }
    }
}

/// Error types for adapter operations
#[derive(Debug)]
pub enum AdapterError {
    /// No suitable adapter was found
    NoAdapterFound,
}

impl std::fmt::Display for AdapterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdapterError::NoAdapterFound => {
                write!(
                    f,
                    "No suitable GPU adapter found with the specified options"
                )
            }
        }
    }
}

impl std::error::Error for AdapterError {}

/// Information about a GPU adapter
#[derive(Debug, Clone)]
pub struct AdapterInfo {
    /// Adapter name
    pub name: String,
    /// Vendor ID
    pub vendor: u32,
    /// Device ID
    pub device: u32,
    /// Device type (Discrete GPU, Integrated GPU, Virtual GPU, CPU, or Unknown)
    pub device_type: wgpu::DeviceType,
    /// Driver name
    pub driver: String,
    /// Driver information
    pub driver_info: String,
    /// Backend used (Vulkan, Metal, DX12, etc.)
    pub backend: wgpu::Backend,
}

impl From<wgpu::AdapterInfo> for AdapterInfo {
    fn from(info: wgpu::AdapterInfo) -> Self {
        Self {
            name: info.name,
            vendor: info.vendor,
            device: info.device,
            device_type: info.device_type,
            driver: info.driver,
            driver_info: info.driver_info,
            backend: info.backend,
        }
    }
}

impl AdapterInfo {
    /// Get adapter information from a wgpu Adapter
    pub fn from_adapter(adapter: &Adapter) -> Self {
        adapter.get_info().into()
    }

    /// Format adapter info as a human-readable string
    pub fn format(&self) -> String {
        format!(
            "Name: {}\nVendor: {}\nDevice: {}\nDevice Type: {:?}\nDriver: {}\nDriver Info: {}\nBackend: {:?}",
            self.name, self.vendor, self.device, self.device_type, self.driver, self.driver_info, self.backend
        )
    }

    /// Get backend name as a string
    pub fn backend_name(&self) -> &'static str {
        backend_to_str(&self.backend)
    }
}

/// Convert a Backend to a human-readable string
pub fn backend_to_str(backend: &wgpu::Backend) -> &'static str {
    match backend {
        wgpu::Backend::Empty => "Empty",
        wgpu::Backend::Vulkan => "Vulkan",
        wgpu::Backend::Metal => "Metal",
        wgpu::Backend::Dx12 => "DirectX 12",
        wgpu::Backend::Gl => "OpenGL",
        wgpu::Backend::BrowserWebGpu => "Browser WebGPU",
    }
}

/// Parse backend names from string to Backends flags
///
/// Accepts multiple common variations for each backend:
/// - "vulkan" or "vk" -> Vulkan
/// - "metal" or "mtl" -> Metal  
/// - "dx12", "d3d12", "directx12", "directx" -> DirectX 12
/// - "gl" or "opengl" -> OpenGL
/// - "webgpu" or "browser" -> Browser WebGPU
/// - "primary" -> PRIMARY backends (platform defaults)
/// - "all" -> All available backends
pub fn parse_backends(name: &str) -> Option<Backends> {
    match name.to_lowercase().as_str() {
        "vulkan" | "vk" => Some(Backends::VULKAN),
        "metal" | "mtl" => Some(Backends::METAL),
        "dx12" | "d3d12" | "directx12" | "directx" => Some(Backends::DX12),
        "gl" | "opengl" => Some(Backends::GL),
        "webgpu" | "browser" => Some(Backends::BROWSER_WEBGPU),
        "primary" => Some(Backends::PRIMARY),
        "all" => Some(Backends::all()),
        _ => None,
    }
}

/// Deprecated: Use parse_backends instead
#[deprecated(since = "0.1.0", note = "Use parse_backends instead for clarity")]
pub fn parse_backend(name: &str) -> Option<Backends> {
    parse_backends(name)
}

/// Get list of available backend names for display purposes
///
/// Returns human-readable display names. To parse backend strings, use parse_backends().
/// Accepted input strings: "vulkan", "metal", "dx12", "gl", "primary", "all"
pub fn available_backends() -> Vec<&'static str> {
    vec![
        "Vulkan",
        "Metal",
        "DirectX 12",
        "OpenGL",
        "Browser WebGPU",
        "Primary",
        "All",
    ]
}

/// Get list of accepted backend input strings for parsing
pub fn backend_input_options() -> Vec<&'static str> {
    vec!["vulkan", "metal", "dx12", "gl", "webgpu", "primary", "all"]
}

/// Create a wgpu Instance with the specified backends
pub fn create_instance(backends: Backends) -> Instance {
    log::debug!("Creating wgpu Instance with backends: {:?}", backends);
    let instance = Instance::new(wgpu::InstanceDescriptor {
        backends,
        ..Default::default()
    });
    log::trace!("Instance created successfully");
    instance
}

/// Create a wgpu Instance with backends from AdapterOptions
pub fn create_instance_with_options(options: &AdapterOptions) -> Instance {
    create_instance(options.backends)
}

/// Enumerate all available GPU adapters
#[cfg(not(target_arch = "wasm32"))]
pub fn enumerate_adapters(backends: Backends) -> Vec<AdapterInfo> {
    let instance = create_instance(backends);

    instance
        .enumerate_adapters(backends)
        .into_iter()
        .map(|adapter| AdapterInfo::from_adapter(&adapter))
        .collect()
}

/// Enumerate all available GPU adapters (WASM stub)
/// Note: enumerate_adapters is not available on WASM. Use request_adapter instead.
#[cfg(target_arch = "wasm32")]
pub fn enumerate_adapters(_backends: Backends) -> Vec<AdapterInfo> {
    vec![]
}

/// Request a GPU adapter with the specified options
pub async fn request_adapter(
    instance: &Instance,
    options: &AdapterOptions,
    compatible_surface: Option<&wgpu::Surface<'_>>,
) -> Result<Adapter, AdapterError> {
    log::debug!(
        "Requesting GPU adapter: power_preference={:?}, backends={:?}",
        options.power_preference,
        options.backends
    );

    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: options.power_preference,
            force_fallback_adapter: options.force_fallback_adapter,
            compatible_surface,
        })
        .await
        .ok_or_else(|| {
            log::error!("No suitable GPU adapter found");
            AdapterError::NoAdapterFound
        })?;

    let info = adapter.get_info();
    log::info!(
        "Adapter selected: {} (Backend: {:?}, Device Type: {:?})",
        info.name,
        info.backend,
        info.device_type
    );

    Ok(adapter)
}

/// Get the limits of an adapter
pub fn get_adapter_limits(adapter: &Adapter) -> Limits {
    adapter.limits()
}

/// Get the features supported by an adapter
pub fn get_adapter_features(adapter: &Adapter) -> Features {
    adapter.features()
}

/// Check if an adapter supports a specific feature
pub fn adapter_supports_feature(adapter: &Adapter, feature: Features) -> bool {
    adapter.features().contains(feature)
}

/// Format adapter features as a human-readable string
pub fn format_adapter_features(features: &Features) -> String {
    format!("{:?}", features)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adapter_options_default() {
        let options = AdapterOptions::default();
        assert!(!options.force_fallback_adapter);
    }

    #[test]
    fn test_adapter_options_high_performance() {
        let options = AdapterOptions::high_performance();
        assert_eq!(options.power_preference, PowerPreference::HighPerformance);
        assert!(!options.force_fallback_adapter);
    }

    #[test]
    fn test_adapter_options_low_power() {
        let options = AdapterOptions::low_power();
        assert_eq!(options.power_preference, PowerPreference::LowPower);
        assert!(!options.force_fallback_adapter);
    }

    #[test]
    fn test_adapter_options_fallback() {
        let options = AdapterOptions::fallback();
        assert!(options.force_fallback_adapter);
    }

    #[test]
    fn test_adapter_options_builder() {
        let options = AdapterOptions::default()
            .with_power_preference(PowerPreference::HighPerformance)
            .with_fallback_adapter(true)
            .with_backends(Backends::VULKAN);

        assert_eq!(options.power_preference, PowerPreference::HighPerformance);
        assert!(options.force_fallback_adapter);
        assert_eq!(options.backends, Backends::VULKAN);
    }

    #[test]
    fn test_adapter_options_with_backend() {
        let options = AdapterOptions::with_backend(Backends::METAL);
        assert_eq!(options.backends, Backends::METAL);
    }

    #[test]
    #[allow(deprecated)]
    fn test_parse_backend() {
        assert_eq!(parse_backend("vulkan"), Some(Backends::VULKAN));
        assert_eq!(parse_backend("vk"), Some(Backends::VULKAN));
        assert_eq!(parse_backend("metal"), Some(Backends::METAL));
        assert_eq!(parse_backend("mtl"), Some(Backends::METAL));
        assert_eq!(parse_backend("dx12"), Some(Backends::DX12));
        assert_eq!(parse_backend("d3d12"), Some(Backends::DX12));
        assert_eq!(parse_backend("gl"), Some(Backends::GL));
        assert_eq!(parse_backend("opengl"), Some(Backends::GL));
        assert_eq!(parse_backend("primary"), Some(Backends::PRIMARY));
        assert_eq!(parse_backend("all"), Some(Backends::all()));
        assert_eq!(parse_backend("invalid"), None);
    }

    #[test]
    fn test_parse_backends() {
        assert_eq!(parse_backends("vulkan"), Some(Backends::VULKAN));
        assert_eq!(parse_backends("vk"), Some(Backends::VULKAN));
        assert_eq!(parse_backends("metal"), Some(Backends::METAL));
        assert_eq!(parse_backends("mtl"), Some(Backends::METAL));
        assert_eq!(parse_backends("dx12"), Some(Backends::DX12));
        assert_eq!(parse_backends("d3d12"), Some(Backends::DX12));
        assert_eq!(parse_backends("gl"), Some(Backends::GL));
        assert_eq!(parse_backends("opengl"), Some(Backends::GL));
        assert_eq!(parse_backends("primary"), Some(Backends::PRIMARY));
        assert_eq!(parse_backends("all"), Some(Backends::all()));
        assert_eq!(parse_backends("invalid"), None);
    }

    #[test]
    fn test_available_backends() {
        let backends = available_backends();
        assert!(!backends.is_empty());
        assert!(backends.contains(&"Vulkan"));
        assert!(backends.contains(&"Metal"));
        assert!(backends.contains(&"DirectX 12"));
        assert!(backends.contains(&"OpenGL"));
    }

    #[test]
    fn test_backend_input_options() {
        let options = backend_input_options();
        assert!(!options.is_empty());
        assert!(options.contains(&"vulkan"));
        assert!(options.contains(&"metal"));
        assert!(options.contains(&"dx12"));
        assert!(options.contains(&"gl"));
        assert!(options.contains(&"all"));
    }

    #[test]
    fn test_create_instance() {
        let instance = create_instance(Backends::all());
        // Instance creation should succeed (no panic)
        drop(instance);
    }

    #[test]
    fn test_create_instance_with_options() {
        let options = AdapterOptions::default().with_backends(Backends::VULKAN);
        let instance = create_instance_with_options(&options);
        // Instance creation should succeed (no panic)
        drop(instance);
    }

    #[test]
    fn test_backend_to_str() {
        assert_eq!(backend_to_str(&wgpu::Backend::Vulkan), "Vulkan");
        assert_eq!(backend_to_str(&wgpu::Backend::Metal), "Metal");
        assert_eq!(backend_to_str(&wgpu::Backend::Dx12), "DirectX 12");
        assert_eq!(backend_to_str(&wgpu::Backend::Gl), "OpenGL");
        assert_eq!(
            backend_to_str(&wgpu::Backend::BrowserWebGpu),
            "Browser WebGPU"
        );
    }

    #[test]
    fn test_adapter_error_display() {
        let err = AdapterError::NoAdapterFound;
        let msg = format!("{}", err);
        assert!(msg.contains("No suitable GPU adapter found"));
    }
}
