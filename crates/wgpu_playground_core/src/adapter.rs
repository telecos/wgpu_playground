use wgpu::{Adapter, Backends, Features, Instance, Limits, PowerPreference, RequestAdapterOptions};

/// Options for requesting a GPU adapter
#[derive(Debug, Clone)]
pub struct AdapterOptions {
    /// Power preference for adapter selection
    pub power_preference: PowerPreference,
    /// Whether to force the use of a fallback/software adapter
    pub force_fallback_adapter: bool,
}

impl Default for AdapterOptions {
    fn default() -> Self {
        Self {
            power_preference: PowerPreference::default(),
            force_fallback_adapter: false,
        }
    }
}

impl AdapterOptions {
    /// Create adapter options with high performance preference
    pub fn high_performance() -> Self {
        Self {
            power_preference: PowerPreference::HighPerformance,
            force_fallback_adapter: false,
        }
    }

    /// Create adapter options with low power preference
    pub fn low_power() -> Self {
        Self {
            power_preference: PowerPreference::LowPower,
            force_fallback_adapter: false,
        }
    }

    /// Create adapter options for fallback/software rendering
    pub fn fallback() -> Self {
        Self {
            power_preference: PowerPreference::default(),
            force_fallback_adapter: true,
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
                write!(f, "No suitable GPU adapter found with the specified options")
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
}

/// Enumerate all available GPU adapters
pub fn enumerate_adapters(backends: Backends) -> Vec<AdapterInfo> {
    let instance = Instance::new(wgpu::InstanceDescriptor {
        backends,
        ..Default::default()
    });

    instance
        .enumerate_adapters(backends)
        .into_iter()
        .map(|adapter| AdapterInfo::from_adapter(&adapter))
        .collect()
}

/// Request a GPU adapter with the specified options
pub async fn request_adapter(
    instance: &Instance,
    options: &AdapterOptions,
    compatible_surface: Option<&wgpu::Surface<'_>>,
) -> Result<Adapter, AdapterError> {
    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: options.power_preference,
            force_fallback_adapter: options.force_fallback_adapter,
            compatible_surface,
        })
        .await
        .ok_or(AdapterError::NoAdapterFound)?;

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
            .with_fallback_adapter(true);
        
        assert_eq!(options.power_preference, PowerPreference::HighPerformance);
        assert!(options.force_fallback_adapter);
    }

    #[test]
    fn test_adapter_error_display() {
        let err = AdapterError::NoAdapterFound;
        let msg = format!("{}", err);
        assert!(msg.contains("No suitable GPU adapter found"));
    }
}
