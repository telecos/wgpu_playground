//! Dawn WebGPU implementation wrapper
//!
//! This module provides WebGPU functionality through Dawn's API style.
//! Dawn is Google's C++ implementation of WebGPU, used by Chromium browsers.
//!
//! # Implementation Strategy
//!
//! This module attempts to build and link against actual Dawn C++ libraries when possible.
//! When Dawn cannot be built (e.g., network unavailable, missing build tools), it falls
//! back to using wgpu-core to provide equivalent WebGPU functionality.
//!
//! ## Building Dawn (Preferred)
//!
//! When the `dawn` feature is enabled, the build script attempts to:
//! 1. Clone the Dawn repository from <https://dawn.googlesource.com/dawn>
//! 2. Build Dawn using CMake with all dependencies
//! 3. Link the Rust code against Dawn's C API
//!
//! Requirements:
//! - Git (to clone Dawn repository)
//! - CMake 3.16+ (to configure and build)
//! - C++ compiler with C++20 support
//! - Python 3 (for Dawn's dependency scripts)
//! - Network access to dawn.googlesource.com
//!
//! ## Fallback Implementation
//!
//! If Dawn cannot be built, this module uses wgpu-core as a compatible backend.
//! This provides full WebGPU functionality while maintaining Dawn's API style.
//!
//! # Platform Support
//!
//! - **Windows**: D3D12 backend (primary)
//! - **Linux**: Vulkan backend
//! - **macOS**: Metal backend
//!
//! # Safety
//!
//! When using actual Dawn, this module wraps unsafe FFI calls to Dawn's C API.
//! When using the fallback, it uses safe wgpu-core APIs.

// FFI type definitions for Dawn's webgpu.h API
// These match the actual Dawn C API for compatibility
#[cfg(feature = "dawn")]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(dead_code)]
mod ffi {
    use std::os::raw::c_void;

    // Opaque types - these are handles to Dawn objects
    pub type WGPUInstance = *mut c_void;
    pub type WGPUAdapter = *mut c_void;
    pub type WGPUDevice = *mut c_void;

    // Enums matching Dawn's webgpu.h
    pub type WGPUPowerPreference = u32;
    pub const WGPUPowerPreference_Undefined: WGPUPowerPreference = 0;
    pub const WGPUPowerPreference_LowPower: WGPUPowerPreference = 1;
    pub const WGPUPowerPreference_HighPerformance: WGPUPowerPreference = 2;

    pub type WGPUBackendType = u32;
    pub const WGPUBackendType_Undefined: WGPUBackendType = 0;
    pub const WGPUBackendType_Null: WGPUBackendType = 1;
    pub const WGPUBackendType_D3D12: WGPUBackendType = 2;
    pub const WGPUBackendType_Metal: WGPUBackendType = 3;
    pub const WGPUBackendType_Vulkan: WGPUBackendType = 4;
    pub const WGPUBackendType_OpenGL: WGPUBackendType = 5;

    // Status codes for adapter request
    pub type WGPURequestAdapterStatus = u32;
    pub const WGPURequestAdapterStatus_Success: WGPURequestAdapterStatus = 0;
    pub const WGPURequestAdapterStatus_Unavailable: WGPURequestAdapterStatus = 1;
    pub const WGPURequestAdapterStatus_Error: WGPURequestAdapterStatus = 2;
    pub const WGPURequestAdapterStatus_Unknown: WGPURequestAdapterStatus = 3;

    // Status codes for device request
    pub type WGPURequestDeviceStatus = u32;
    pub const WGPURequestDeviceStatus_Success: WGPURequestDeviceStatus = 0;
    pub const WGPURequestDeviceStatus_Error: WGPURequestDeviceStatus = 1;
    pub const WGPURequestDeviceStatus_Unknown: WGPURequestDeviceStatus = 2;

    // String view for C strings
    #[repr(C)]
    pub struct WGPUStringView {
        pub data: *const u8,
        pub length: usize,
    }

    // Structs matching Dawn's C API
    #[repr(C)]
    pub struct WGPUChainedStruct {
        pub next: *const WGPUChainedStruct,
        pub s_type: u32,
    }

    #[repr(C)]
    pub struct WGPUInstanceDescriptor {
        pub next_in_chain: *const WGPUChainedStruct,
    }

    #[repr(C)]
    pub struct WGPURequestAdapterOptions {
        pub next_in_chain: *const WGPUChainedStruct,
        pub compatible_surface: *mut c_void,
        pub power_preference: WGPUPowerPreference,
        pub backend_type: WGPUBackendType,
        pub force_fallback_adapter: u32,
    }

    #[repr(C)]
    pub struct WGPUDeviceDescriptor {
        pub next_in_chain: *const WGPUChainedStruct,
        pub label: *const u8,
    }

    // Callback function types
    pub type WGPURequestAdapterCallback = unsafe extern "C" fn(
        status: WGPURequestAdapterStatus,
        adapter: WGPUAdapter,
        message: WGPUStringView,
        userdata1: *mut c_void,
        userdata2: *mut c_void,
    );

    pub type WGPURequestDeviceCallback = unsafe extern "C" fn(
        status: WGPURequestDeviceStatus,
        device: WGPUDevice,
        message: WGPUStringView,
        userdata1: *mut c_void,
        userdata2: *mut c_void,
    );

    // Dawn FFI functions
    // These will be linked from libdawn if successfully built
    // If Dawn build fails, these won't be available and we use the fallback
    #[cfg(dawn_enabled)]
    extern "C" {
        pub fn wgpuCreateInstance(descriptor: *const WGPUInstanceDescriptor) -> WGPUInstance;
        pub fn wgpuInstanceRelease(instance: WGPUInstance);
        pub fn wgpuAdapterRelease(adapter: WGPUAdapter);
        pub fn wgpuDeviceRelease(device: WGPUDevice);
        pub fn wgpuInstanceRequestAdapter(
            instance: WGPUInstance,
            options: *const WGPURequestAdapterOptions,
            callback: WGPURequestAdapterCallback,
            userdata1: *mut c_void,
            userdata2: *mut c_void,
        );
        pub fn wgpuAdapterRequestDevice(
            adapter: WGPUAdapter,
            descriptor: *const WGPUDeviceDescriptor,
            callback: WGPURequestDeviceCallback,
            userdata1: *mut c_void,
            userdata2: *mut c_void,
        );
    }
}

// Constants for async callback polling
#[cfg(all(feature = "dawn", dawn_enabled))]
const CALLBACK_MAX_WAIT_MS: u64 = 5000;
#[cfg(all(feature = "dawn", dawn_enabled))]
const CALLBACK_POLL_INTERVAL_MS: u64 = 10;

// Helper function to extract message from WGPUStringView
#[cfg(all(feature = "dawn", dawn_enabled))]
unsafe fn extract_message_from_string_view(message: ffi::WGPUStringView) -> String {
    if !message.data.is_null() && message.length > 0 {
        let slice = std::slice::from_raw_parts(message.data, message.length);
        String::from_utf8_lossy(slice).to_string()
    } else {
        String::new()
    }
}

/// Dawn instance wrapper
///
/// Manages the lifecycle of a Dawn WebGPU instance.
///
/// This uses actual Dawn C++ library when available (built via build.rs),
/// falling back to wgpu-core for full WebGPU functionality when Dawn build fails.
#[cfg(feature = "dawn")]
pub struct DawnInstance {
    /// The underlying implementation - either Dawn FFI handle or wgpu instance
    inner: DawnInstanceInner,
}

#[cfg(feature = "dawn")]
enum DawnInstanceInner {
    /// Using actual Dawn C++ library via FFI
    #[cfg(dawn_enabled)]
    NativeDawn(ffi::WGPUInstance),
    /// Using wgpu-core as fallback
    WgpuFallback(wgpu::Instance),
}

#[cfg(feature = "dawn")]
impl DawnInstance {
    /// Create a new Dawn instance
    ///
    /// Attempts to use actual Dawn C++ library if available,
    /// falls back to wgpu-core otherwise.
    pub fn new() -> Result<Self, DawnError> {
        // Try native Dawn first if it was successfully built
        #[cfg(dawn_enabled)]
        {
            log::info!("Attempting to create Dawn instance using native C++ library");
            unsafe {
                let descriptor = ffi::WGPUInstanceDescriptor {
                    next_in_chain: std::ptr::null(),
                };
                let instance = ffi::wgpuCreateInstance(&descriptor);
                if !instance.is_null() {
                    log::info!("Successfully created native Dawn instance");
                    return Ok(Self {
                        inner: DawnInstanceInner::NativeDawn(instance),
                    });
                } else {
                    log::warn!("Native Dawn instance creation returned null, falling back to wgpu");
                }
            }
        }

        // Fallback to wgpu-core (or only option if Dawn not built)
        #[cfg(not(dawn_enabled))]
        log::info!("Dawn C++ library not available - using wgpu-core fallback");
        #[cfg(dawn_enabled)]
        log::info!("Falling back to wgpu-core");

        log::info!("Creating Dawn-compatible WebGPU instance using wgpu-core backend");

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        log::info!("Dawn-compatible instance created successfully");
        Ok(Self {
            inner: DawnInstanceInner::WgpuFallback(instance),
        })
    }

    /// Request a Dawn adapter
    ///
    /// # Arguments
    ///
    /// * `power_preference` - Power preference for adapter selection
    ///
    /// # Returns
    ///
    /// Returns a DawnAdapter if successful, or an error if no suitable adapter is found.
    pub async fn request_adapter(
        &self,
        power_preference: DawnPowerPreference,
    ) -> Result<DawnAdapter, DawnError> {
        match &self.inner {
            #[cfg(dawn_enabled)]
            DawnInstanceInner::NativeDawn(instance) => {
                use std::sync::{Arc, Mutex};

                // Callback result container
                #[derive(Default)]
                struct CallbackResult {
                    adapter: Option<ffi::WGPUAdapter>,
                    status: ffi::WGPURequestAdapterStatus,
                    message: String,
                }

                let result = Arc::new(Mutex::new(None::<CallbackResult>));
                let result_clone = Arc::clone(&result);

                // C callback function
                unsafe extern "C" fn adapter_callback(
                    status: ffi::WGPURequestAdapterStatus,
                    adapter: ffi::WGPUAdapter,
                    message: ffi::WGPUStringView,
                    userdata1: *mut std::os::raw::c_void,
                    _userdata2: *mut std::os::raw::c_void,
                ) {
                    let result_ptr = userdata1 as *const Mutex<Option<CallbackResult>>;
                    if !result_ptr.is_null() {
                        let result_arc = Arc::from_raw(result_ptr);

                        // Extract message string
                        let msg = extract_message_from_string_view(message);

                        if let Ok(mut guard) = result_arc.lock() {
                            *guard = Some(CallbackResult {
                                adapter: if adapter.is_null() {
                                    None
                                } else {
                                    Some(adapter)
                                },
                                status,
                                message: msg,
                            });
                        }

                        // Don't drop the Arc, we need it after the callback
                        std::mem::forget(result_arc);
                    }
                }

                // Prepare request options
                let options = ffi::WGPURequestAdapterOptions {
                    next_in_chain: std::ptr::null(),
                    compatible_surface: std::ptr::null_mut(),
                    power_preference: power_preference.to_wgpu(),
                    backend_type: ffi::WGPUBackendType_Undefined,
                    force_fallback_adapter: 0,
                };

                // Make the async request
                unsafe {
                    let userdata = Arc::into_raw(result_clone) as *mut std::os::raw::c_void;
                    ffi::wgpuInstanceRequestAdapter(
                        *instance,
                        &options,
                        adapter_callback,
                        userdata,
                        std::ptr::null_mut(),
                    );
                }

                // Poll for completion (Dawn callbacks are usually immediate or very fast)
                // In a real async implementation, this would integrate with an event loop
                let start = std::time::Instant::now();
                let max_wait = std::time::Duration::from_millis(CALLBACK_MAX_WAIT_MS);

                loop {
                    if let Ok(guard) = result.lock() {
                        if guard.is_some() {
                            break;
                        }
                    }

                    if start.elapsed() > max_wait {
                        return Err(DawnError::NoAdapterFound);
                    }

                    // Small sleep to avoid busy waiting
                    std::thread::sleep(std::time::Duration::from_millis(CALLBACK_POLL_INTERVAL_MS));
                }

                // Extract result
                let callback_result = result
                    .lock()
                    .unwrap()
                    .take()
                    .ok_or(DawnError::NoAdapterFound)?;

                // Map status to error
                match callback_result.status {
                    ffi::WGPURequestAdapterStatus_Success => {
                        if let Some(adapter) = callback_result.adapter {
                            log::info!("Native Dawn adapter request succeeded");
                            Ok(DawnAdapter {
                                inner: DawnAdapterInner::NativeDawn(adapter),
                            })
                        } else {
                            Err(DawnError::NoAdapterFound)
                        }
                    }
                    ffi::WGPURequestAdapterStatus_Unavailable => {
                        log::warn!("Dawn adapter unavailable");
                        Err(DawnError::AdapterUnavailable)
                    }
                    ffi::WGPURequestAdapterStatus_Error => {
                        log::error!("Dawn adapter error: {}", callback_result.message);
                        Err(DawnError::AdapterError(callback_result.message))
                    }
                    _ => {
                        log::error!("Unknown Dawn adapter status");
                        Err(DawnError::Unknown)
                    }
                }
            }
            DawnInstanceInner::WgpuFallback(instance) => {
                log::info!(
                    "Requesting adapter with power preference: {:?}",
                    power_preference
                );

                let wgpu_power_pref = match power_preference {
                    DawnPowerPreference::LowPower => wgpu::PowerPreference::LowPower,
                    DawnPowerPreference::HighPerformance => wgpu::PowerPreference::HighPerformance,
                    DawnPowerPreference::Undefined => wgpu::PowerPreference::default(),
                };

                let adapter = instance
                    .request_adapter(&wgpu::RequestAdapterOptions {
                        power_preference: wgpu_power_pref,
                        compatible_surface: None,
                        force_fallback_adapter: false,
                    })
                    .await
                    .map_err(|_| DawnError::NoAdapterFound)?;

                log::info!("Adapter found successfully");
                Ok(DawnAdapter {
                    inner: DawnAdapterInner::WgpuFallback(adapter),
                })
            }
        }
    }

    /// Get a reference to the underlying wgpu instance (when using fallback)
    ///
    /// Returns None if using native Dawn
    pub fn wgpu_instance(&self) -> Option<&wgpu::Instance> {
        match &self.inner {
            #[cfg(dawn_enabled)]
            DawnInstanceInner::NativeDawn(_) => None,
            DawnInstanceInner::WgpuFallback(instance) => Some(instance),
        }
    }

    /// Check if using native Dawn implementation
    pub fn is_native_dawn(&self) -> bool {
        match &self.inner {
            #[cfg(dawn_enabled)]
            DawnInstanceInner::NativeDawn(_) => true,
            DawnInstanceInner::WgpuFallback(_) => false,
        }
    }
}

#[cfg(feature = "dawn")]
impl Drop for DawnInstance {
    fn drop(&mut self) {
        #[cfg(dawn_enabled)]
        if let DawnInstanceInner::NativeDawn(instance) = &self.inner {
            unsafe {
                ffi::wgpuInstanceRelease(*instance);
            }
        }
    }
}

/// Dawn adapter wrapper
///
/// Represents a GPU adapter obtained from Dawn.
#[cfg(feature = "dawn")]
pub struct DawnAdapter {
    inner: DawnAdapterInner,
}

#[cfg(feature = "dawn")]
enum DawnAdapterInner {
    #[cfg(dawn_enabled)]
    NativeDawn(ffi::WGPUAdapter),
    WgpuFallback(wgpu::Adapter),
}

#[cfg(feature = "dawn")]
impl DawnAdapter {
    /// Get adapter information
    pub fn get_info(&self) -> DawnAdapterInfo {
        match &self.inner {
            #[cfg(dawn_enabled)]
            DawnAdapterInner::NativeDawn(_) => {
                // NOTE: Currently returns placeholder values because Dawn FFI bindings
                // don't expose adapter info query methods. To implement proper info:
                // 1. Add FFI binding for wgpuAdapterGetProperties in dawn_ffi module
                // 2. Define WGPUAdapterProperties C struct mapping
                // 3. Call wgpuAdapterGetProperties and convert to DawnAdapterInfo
                // See: https://github.com/webgpu-native/webgpu-headers for API reference
                DawnAdapterInfo {
                    name: "Dawn Native Adapter".to_string(),
                    vendor: 0,
                    device: 0,
                    backend: DawnBackend::Vulkan,
                }
            }
            DawnAdapterInner::WgpuFallback(adapter) => {
                let info = adapter.get_info();
                let backend = match info.backend {
                    wgpu::Backend::Vulkan => DawnBackend::Vulkan,
                    wgpu::Backend::Metal => DawnBackend::Metal,
                    wgpu::Backend::Dx12 => DawnBackend::D3D12,
                    wgpu::Backend::Gl => DawnBackend::OpenGL,
                    _ => DawnBackend::Null,
                };

                DawnAdapterInfo {
                    name: info.name,
                    vendor: info.vendor,
                    device: info.device,
                    backend,
                }
            }
        }
    }

    /// Request a device from this adapter
    ///
    /// # Arguments
    ///
    /// * `descriptor` - Device descriptor with required features and limits
    ///
    /// # Returns
    ///
    /// Returns a DawnDevice if successful, or an error if device creation fails.
    pub async fn request_device(
        &self,
        descriptor: &DawnDeviceDescriptor,
    ) -> Result<DawnDevice, DawnError> {
        match &self.inner {
            #[cfg(dawn_enabled)]
            DawnAdapterInner::NativeDawn(adapter) => {
                use std::sync::{Arc, Mutex};

                // Callback result container
                #[derive(Default)]
                struct CallbackResult {
                    device: Option<ffi::WGPUDevice>,
                    status: ffi::WGPURequestDeviceStatus,
                    message: String,
                }

                let result = Arc::new(Mutex::new(None::<CallbackResult>));
                let result_clone = Arc::clone(&result);

                // C callback function
                unsafe extern "C" fn device_callback(
                    status: ffi::WGPURequestDeviceStatus,
                    device: ffi::WGPUDevice,
                    message: ffi::WGPUStringView,
                    userdata1: *mut std::os::raw::c_void,
                    _userdata2: *mut std::os::raw::c_void,
                ) {
                    let result_ptr = userdata1 as *const Mutex<Option<CallbackResult>>;
                    if !result_ptr.is_null() {
                        let result_arc = Arc::from_raw(result_ptr);

                        // Extract message string
                        let msg = extract_message_from_string_view(message);

                        if let Ok(mut guard) = result_arc.lock() {
                            *guard = Some(CallbackResult {
                                device: if device.is_null() { None } else { Some(device) },
                                status,
                                message: msg,
                            });
                        }

                        // Don't drop the Arc, we need it after the callback
                        std::mem::forget(result_arc);
                    }
                }

                // Prepare device descriptor
                let label_cstring;
                let label_ptr = if let Some(label) = &descriptor.label {
                    label_cstring = std::ffi::CString::new(label.as_str())
                        .unwrap_or_else(|_| std::ffi::CString::new("").unwrap());
                    label_cstring.as_ptr() as *const u8
                } else {
                    std::ptr::null()
                };

                let device_desc = ffi::WGPUDeviceDescriptor {
                    next_in_chain: std::ptr::null(),
                    label: label_ptr,
                };

                // Make the async request
                unsafe {
                    let userdata = Arc::into_raw(result_clone) as *mut std::os::raw::c_void;
                    ffi::wgpuAdapterRequestDevice(
                        *adapter,
                        &device_desc,
                        device_callback,
                        userdata,
                        std::ptr::null_mut(),
                    );
                }

                // Poll for completion
                let start = std::time::Instant::now();
                let max_wait = std::time::Duration::from_millis(CALLBACK_MAX_WAIT_MS);

                loop {
                    if let Ok(guard) = result.lock() {
                        if guard.is_some() {
                            break;
                        }
                    }

                    if start.elapsed() > max_wait {
                        return Err(DawnError::DeviceCreationFailed);
                    }

                    // Small sleep to avoid busy waiting
                    std::thread::sleep(std::time::Duration::from_millis(CALLBACK_POLL_INTERVAL_MS));
                }

                // Extract result
                let callback_result = result
                    .lock()
                    .unwrap()
                    .take()
                    .ok_or(DawnError::DeviceCreationFailed)?;

                // Map status to error
                match callback_result.status {
                    ffi::WGPURequestDeviceStatus_Success => {
                        if let Some(device) = callback_result.device {
                            log::info!("Native Dawn device request succeeded");
                            Ok(DawnDevice {
                                inner: DawnDeviceInner::NativeDawn(device),
                            })
                        } else {
                            Err(DawnError::DeviceCreationFailed)
                        }
                    }
                    ffi::WGPURequestDeviceStatus_Error => {
                        log::error!("Dawn device error: {}", callback_result.message);
                        Err(DawnError::DeviceError(callback_result.message))
                    }
                    _ => {
                        log::error!("Unknown Dawn device status");
                        Err(DawnError::Unknown)
                    }
                }
            }
            DawnAdapterInner::WgpuFallback(adapter) => {
                log::info!("Requesting device with label: {:?}", descriptor.label);

                let device_result = adapter
                    .request_device(&wgpu::DeviceDescriptor {
                        label: descriptor.label.as_deref(),
                        required_features: wgpu::Features::empty(),
                        required_limits: wgpu::Limits::default(),
                        memory_hints: wgpu::MemoryHints::default(),
                        experimental_features: Default::default(),
                        trace: Default::default(),
                    })
                    .await;

                match device_result {
                    Ok((device, queue)) => {
                        log::info!("Device created successfully");
                        Ok(DawnDevice {
                            inner: DawnDeviceInner::WgpuFallback { device, queue },
                        })
                    }
                    Err(e) => {
                        log::error!("Device creation failed: {}", e);
                        Err(DawnError::DeviceCreationFailed)
                    }
                }
            }
        }
    }

    /// Get a reference to the underlying wgpu adapter (when using fallback)
    ///
    /// Returns None if using native Dawn
    pub fn wgpu_adapter(&self) -> Option<&wgpu::Adapter> {
        match &self.inner {
            #[cfg(dawn_enabled)]
            DawnAdapterInner::NativeDawn(_) => None,
            DawnAdapterInner::WgpuFallback(adapter) => Some(adapter),
        }
    }
}

#[cfg(feature = "dawn")]
impl Drop for DawnAdapter {
    fn drop(&mut self) {
        #[cfg(dawn_enabled)]
        if let DawnAdapterInner::NativeDawn(adapter) = &self.inner {
            unsafe {
                ffi::wgpuAdapterRelease(*adapter);
            }
        }
    }
}

/// Dawn adapter information
#[cfg(feature = "dawn")]
#[derive(Debug, Clone)]
pub struct DawnAdapterInfo {
    pub name: String,
    pub vendor: u32,
    pub device: u32,
    pub backend: DawnBackend,
}

/// Dawn backend types
#[cfg(feature = "dawn")]
#[derive(Debug, Clone, Copy)]
pub enum DawnBackend {
    D3D12,
    Metal,
    Vulkan,
    OpenGL,
    Null,
}

/// Dawn power preference
#[cfg(feature = "dawn")]
#[derive(Debug, Clone, Copy)]
pub enum DawnPowerPreference {
    Undefined,
    LowPower,
    HighPerformance,
}

#[cfg(feature = "dawn")]
impl DawnPowerPreference {
    #[allow(dead_code)]
    fn to_wgpu(self) -> ffi::WGPUPowerPreference {
        match self {
            Self::Undefined => ffi::WGPUPowerPreference_Undefined,
            Self::LowPower => ffi::WGPUPowerPreference_LowPower,
            Self::HighPerformance => ffi::WGPUPowerPreference_HighPerformance,
        }
    }
}

/// Dawn device descriptor
#[cfg(feature = "dawn")]
#[derive(Debug, Clone, Default)]
pub struct DawnDeviceDescriptor {
    pub label: Option<String>,
}

/// Dawn device wrapper
///
/// Represents a GPU device obtained from Dawn.
#[cfg(feature = "dawn")]
pub struct DawnDevice {
    inner: DawnDeviceInner,
}

#[cfg(feature = "dawn")]
enum DawnDeviceInner {
    #[cfg(dawn_enabled)]
    NativeDawn(ffi::WGPUDevice),
    WgpuFallback {
        device: wgpu::Device,
        queue: wgpu::Queue,
    },
}

#[cfg(feature = "dawn")]
impl DawnDevice {
    /// Get a reference to the underlying wgpu device (when using fallback)
    ///
    /// Returns None if using native Dawn
    pub fn wgpu_device(&self) -> Option<&wgpu::Device> {
        match &self.inner {
            #[cfg(dawn_enabled)]
            DawnDeviceInner::NativeDawn(_) => None,
            DawnDeviceInner::WgpuFallback { device, .. } => Some(device),
        }
    }

    /// Get a reference to the device queue (when using fallback)
    ///
    /// Returns None if using native Dawn
    pub fn wgpu_queue(&self) -> Option<&wgpu::Queue> {
        match &self.inner {
            #[cfg(dawn_enabled)]
            DawnDeviceInner::NativeDawn(_) => None,
            DawnDeviceInner::WgpuFallback { queue, .. } => Some(queue),
        }
    }
}

#[cfg(feature = "dawn")]
impl Drop for DawnDevice {
    fn drop(&mut self) {
        #[cfg(dawn_enabled)]
        if let DawnDeviceInner::NativeDawn(device) = &self.inner {
            unsafe {
                ffi::wgpuDeviceRelease(*device);
            }
        }
    }
}

/// Dawn-specific errors
#[derive(Debug)]
pub enum DawnError {
    InstanceCreationFailed,
    NoAdapterFound,
    AdapterUnavailable,
    AdapterError(String),
    DeviceCreationFailed,
    DeviceError(String),
    NotImplemented,
    Unknown,
}

impl std::fmt::Display for DawnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InstanceCreationFailed => write!(f, "Failed to create Dawn instance"),
            Self::NoAdapterFound => write!(f, "No suitable Dawn adapter found"),
            Self::AdapterUnavailable => write!(f, "Dawn adapter unavailable"),
            Self::AdapterError(msg) => write!(f, "Dawn adapter error: {}", msg),
            Self::DeviceCreationFailed => write!(f, "Failed to create Dawn device"),
            Self::DeviceError(msg) => write!(f, "Dawn device error: {}", msg),
            Self::NotImplemented => {
                write!(f, "Feature not yet implemented for native Dawn")
            }
            Self::Unknown => write!(f, "Unknown Dawn error"),
        }
    }
}

impl std::error::Error for DawnError {}

// Stub implementations for when dawn feature is disabled
#[cfg(not(feature = "dawn"))]
pub struct DawnInstance;

#[cfg(not(feature = "dawn"))]
impl DawnInstance {
    pub fn new() -> Result<Self, String> {
        Err("Dawn feature not enabled".to_string())
    }
}

#[cfg(test)]
#[cfg(feature = "dawn")]
mod tests {
    use super::*;

    #[test]
    fn test_dawn_instance_creation() {
        // Dawn instance creation should now succeed
        let result = DawnInstance::new();
        assert!(
            result.is_ok(),
            "Dawn instance creation should succeed with wgpu backend"
        );
    }

    #[test]
    fn test_power_preference_types() {
        // Test that the types compile
        let _pref = DawnPowerPreference::HighPerformance;
        let _undefined = DawnPowerPreference::Undefined;
        let _low = DawnPowerPreference::LowPower;
    }

    #[test]
    fn test_backend_types() {
        let backend = DawnBackend::Vulkan;
        assert!(matches!(backend, DawnBackend::Vulkan));
    }

    #[test]
    fn test_device_descriptor() {
        let desc = DawnDeviceDescriptor {
            label: Some("Test Device".to_string()),
        };
        assert_eq!(desc.label.as_deref(), Some("Test Device"));

        let default_desc = DawnDeviceDescriptor::default();
        assert!(default_desc.label.is_none());
    }

    #[test]
    fn test_dawn_error_types() {
        // Test that all error types can be created and displayed
        let err1 = DawnError::InstanceCreationFailed;
        assert!(err1.to_string().contains("instance"));

        let err2 = DawnError::NoAdapterFound;
        assert!(err2.to_string().contains("adapter"));

        let err3 = DawnError::AdapterUnavailable;
        assert!(err3.to_string().contains("unavailable"));

        let err4 = DawnError::AdapterError("test message".to_string());
        assert!(err4.to_string().contains("test message"));

        let err5 = DawnError::DeviceCreationFailed;
        assert!(err5.to_string().contains("device"));

        let err6 = DawnError::DeviceError("device error".to_string());
        assert!(err6.to_string().contains("device error"));

        let err7 = DawnError::NotImplemented;
        assert!(err7.to_string().contains("not yet implemented"));

        let err8 = DawnError::Unknown;
        assert!(err8.to_string().contains("Unknown"));
    }

    #[test]
    fn test_dawn_error_is_error_trait() {
        // Verify DawnError implements std::error::Error
        fn check_error<E: std::error::Error>(_e: &E) {}

        let err = DawnError::NoAdapterFound;
        check_error(&err);
    }
}
