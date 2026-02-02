/// Comprehensive error handling for WebGPU operations
///
/// This module provides error types, error scopes, and utilities for handling
/// validation errors, out-of-memory errors, and internal errors.
use std::fmt;
use std::sync::{Arc, Mutex};

/// Type alias for a WGPU error callback
///
/// This callback is designed for use with `device.on_uncaptured_error()` to handle
/// GPU errors that occur outside of error scopes. The `Send + Sync` bounds allow
/// the callback to be safely shared across threads, which is required by wgpu's API.
type WgpuErrorCallback = Box<dyn Fn(wgpu::Error) + Send + Sync>;

/// Types of WebGPU errors that can occur
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorType {
    /// Validation error - occurs when API usage violates WebGPU specification
    Validation,
    /// Out of memory error - occurs when GPU runs out of memory
    OutOfMemory,
    /// Internal error - occurs due to GPU driver or hardware issues
    Internal,
    /// Device lost error - occurs when GPU device is lost or reset
    DeviceLost,
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorType::Validation => write!(f, "Validation"),
            ErrorType::OutOfMemory => write!(f, "OutOfMemory"),
            ErrorType::Internal => write!(f, "Internal"),
            ErrorType::DeviceLost => write!(f, "DeviceLost"),
        }
    }
}

/// A WebGPU error with type and message
#[derive(Debug, Clone)]
pub struct Error {
    /// The type of error
    pub error_type: ErrorType,
    /// Human-readable error message
    pub message: String,
}

impl Error {
    /// Create a new error
    pub fn new(error_type: ErrorType, message: String) -> Self {
        Self {
            error_type,
            message,
        }
    }

    /// Create a validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Self::new(ErrorType::Validation, message.into())
    }

    /// Create an out-of-memory error
    pub fn out_of_memory(message: impl Into<String>) -> Self {
        Self::new(ErrorType::OutOfMemory, message.into())
    }

    /// Create an internal error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(ErrorType::Internal, message.into())
    }

    /// Create a device lost error
    pub fn device_lost(message: impl Into<String>) -> Self {
        Self::new(ErrorType::DeviceLost, message.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} error: {}", self.error_type, self.message)
    }
}

impl std::error::Error for Error {}

/// Convert wgpu::Error to our Error type
impl From<wgpu::Error> for Error {
    fn from(err: wgpu::Error) -> Self {
        match err {
            wgpu::Error::OutOfMemory { source } => {
                Error::out_of_memory(format!("GPU out of memory: {}", source))
            }
            wgpu::Error::Validation { source, .. } => {
                Error::validation(format!("Validation error: {}", source))
            }
            wgpu::Error::Internal { source, .. } => {
                Error::internal(format!("Internal error: {}", source))
            }
        }
    }
}

/// Error filter for selecting which errors to capture
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorFilter {
    /// Capture validation errors only
    Validation,
    /// Capture out-of-memory errors only
    OutOfMemory,
    /// Capture internal errors only
    Internal,
}

impl ErrorFilter {
    /// Check if this filter matches an error type
    pub fn matches(&self, error_type: &ErrorType) -> bool {
        matches!(
            (self, error_type),
            (ErrorFilter::Validation, ErrorType::Validation)
                | (ErrorFilter::OutOfMemory, ErrorType::OutOfMemory)
                | (ErrorFilter::Internal, ErrorType::Internal)
        )
    }

    /// Convert to wgpu::ErrorFilter
    pub fn to_wgpu(&self) -> wgpu::ErrorFilter {
        match self {
            ErrorFilter::Validation => wgpu::ErrorFilter::Validation,
            ErrorFilter::OutOfMemory => wgpu::ErrorFilter::OutOfMemory,
            ErrorFilter::Internal => wgpu::ErrorFilter::Internal,
        }
    }
}

/// Error scope for capturing GPU errors during operations
///
/// Error scopes allow you to catch specific types of errors that occur
/// during a sequence of GPU operations. This is useful for:
/// - Testing and validation
/// - Graceful error recovery
/// - Detailed error reporting
///
/// # Example
/// ```no_run
/// use wgpu_playground_core::error::{ErrorScope, ErrorFilter};
/// # async fn example(device: &wgpu::Device) {
/// // Push an error scope to capture validation errors
/// ErrorScope::push(device, ErrorFilter::Validation);
///
/// // Perform operations that might generate errors
/// // ... GPU operations ...
///
/// // Pop the scope and check for errors
/// if let Some(error) = ErrorScope::pop(device).await {
///     eprintln!("Caught error: {}", error);
/// }
/// # }
/// ```
pub struct ErrorScope;

impl ErrorScope {
    /// Push an error scope onto the device's error scope stack
    ///
    /// All GPU errors of the specified type that occur after this call
    /// will be captured until the scope is popped.
    ///
    /// # Arguments
    /// * `device` - The GPU device
    /// * `filter` - The type of errors to capture
    pub fn push(device: &wgpu::Device, filter: ErrorFilter) {
        device.push_error_scope(filter.to_wgpu());
        log::debug!("Pushed error scope: {:?}", filter);
    }

    /// Pop an error scope from the device's error scope stack
    ///
    /// Returns the first error that was captured in this scope, if any.
    /// This is an async operation that waits for the GPU to finish
    /// processing all commands in the scope.
    ///
    /// # Arguments
    /// * `device` - The GPU device
    ///
    /// # Returns
    /// The first error captured in this scope, or None if no errors occurred
    pub async fn pop(device: &wgpu::Device) -> Option<Error> {
        let result = device.pop_error_scope().await;
        match result {
            Some(err) => {
                let error = Error::from(err);
                log::warn!("Error scope captured: {}", error);
                Some(error)
            }
            None => {
                log::debug!("Error scope popped with no errors");
                None
            }
        }
    }
}

/// Error callback for handling uncaptured errors
///
/// This provides a way to register custom error handlers that will be
/// called when GPU errors occur outside of error scopes.
pub type ErrorCallback = Box<dyn Fn(Error) + Send + Sync>;

/// Error handler for managing device-level error callbacks
pub struct ErrorHandler {
    callbacks: Arc<Mutex<Vec<ErrorCallback>>>,
}

impl ErrorHandler {
    /// Create a new error handler
    pub fn new() -> Self {
        Self {
            callbacks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Register an error callback
    ///
    /// The callback will be invoked whenever an uncaptured GPU error occurs.
    ///
    /// # Arguments
    /// * `callback` - Function to call when an error occurs
    pub fn on_error<F>(&mut self, callback: F)
    where
        F: Fn(Error) + Send + Sync + 'static,
    {
        let mut callbacks = self.callbacks.lock().unwrap();
        callbacks.push(Box::new(callback));
        log::debug!("Registered error callback");
    }

    /// Invoke all registered callbacks with an error
    ///
    /// # Arguments
    /// * `error` - The error that occurred
    pub fn handle_error(&self, error: Error) {
        log::error!("Handling error: {}", error);
        let callbacks = self.callbacks.lock().unwrap();
        for callback in callbacks.iter() {
            callback(error.clone());
        }
    }

    /// Create a wgpu error callback that forwards to this handler
    ///
    /// Returns a boxed closure suitable for use with `device.on_uncaptured_error()`
    pub fn create_wgpu_callback(&self) -> WgpuErrorCallback {
        let callbacks = Arc::clone(&self.callbacks);
        Box::new(move |wgpu_error| {
            let error = Error::from(wgpu_error);
            let callbacks = callbacks.lock().unwrap();
            for callback in callbacks.iter() {
                callback(error.clone());
            }
        })
    }
}

impl Default for ErrorHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Device lost reason
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeviceLostReason {
    /// Device was intentionally destroyed
    Destroyed,
    /// Device was lost due to unknown reason (possibly driver crash or GPU reset)
    Unknown,
}

impl From<wgpu::DeviceLostReason> for DeviceLostReason {
    fn from(reason: wgpu::DeviceLostReason) -> Self {
        match reason {
            wgpu::DeviceLostReason::Unknown => DeviceLostReason::Unknown,
            wgpu::DeviceLostReason::Destroyed => DeviceLostReason::Destroyed,
        }
    }
}

impl fmt::Display for DeviceLostReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceLostReason::Destroyed => write!(f, "Destroyed"),
            DeviceLostReason::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Device lost callback
pub type DeviceLostCallback = Box<dyn Fn(DeviceLostReason, String) + Send + 'static>;

/// Setup comprehensive error handling for a device
///
/// This is a convenience function that sets up:
/// - Uncaptured error callback
/// - Device lost callback
/// - Default logging for all errors
///
/// # Arguments
/// * `device` - The GPU device to configure
///
/// # Example
/// ```no_run
/// use wgpu_playground_core::error::setup_device_error_handling;
/// # async fn example(device: &wgpu::Device) {
/// setup_device_error_handling(device);
/// // Now the device will log all errors automatically
/// # }
/// ```
pub fn setup_device_error_handling(device: &wgpu::Device) {
    // Set up uncaptured error callback
    device.on_uncaptured_error(Arc::new(|error| {
        let err = Error::from(error);
        log::error!("Uncaptured GPU error: {}", err);
    }));

    // Set up device lost callback
    device.set_device_lost_callback(|reason, message| {
        let reason = DeviceLostReason::from(reason);
        log::error!("Device lost! Reason: {}, Message: {}", reason, message);
    });

    log::info!("Device error handling configured");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_type_display() {
        assert_eq!(ErrorType::Validation.to_string(), "Validation");
        assert_eq!(ErrorType::OutOfMemory.to_string(), "OutOfMemory");
        assert_eq!(ErrorType::Internal.to_string(), "Internal");
        assert_eq!(ErrorType::DeviceLost.to_string(), "DeviceLost");
    }

    #[test]
    fn test_error_creation() {
        let err = Error::validation("test error");
        assert_eq!(err.error_type, ErrorType::Validation);
        assert_eq!(err.message, "test error");

        let err = Error::out_of_memory("oom error");
        assert_eq!(err.error_type, ErrorType::OutOfMemory);
        assert_eq!(err.message, "oom error");

        let err = Error::internal("internal error");
        assert_eq!(err.error_type, ErrorType::Internal);
        assert_eq!(err.message, "internal error");

        let err = Error::device_lost("device lost");
        assert_eq!(err.error_type, ErrorType::DeviceLost);
        assert_eq!(err.message, "device lost");
    }

    #[test]
    fn test_error_display() {
        let err = Error::validation("test");
        assert_eq!(err.to_string(), "Validation error: test");
    }

    #[test]
    fn test_error_filter_matches() {
        assert!(ErrorFilter::Validation.matches(&ErrorType::Validation));
        assert!(!ErrorFilter::Validation.matches(&ErrorType::OutOfMemory));
        assert!(!ErrorFilter::Validation.matches(&ErrorType::Internal));

        assert!(ErrorFilter::OutOfMemory.matches(&ErrorType::OutOfMemory));
        assert!(!ErrorFilter::OutOfMemory.matches(&ErrorType::Validation));

        assert!(ErrorFilter::Internal.matches(&ErrorType::Internal));
        assert!(!ErrorFilter::Internal.matches(&ErrorType::Validation));
    }

    #[test]
    fn test_error_handler_creation() {
        let handler = ErrorHandler::new();
        assert_eq!(handler.callbacks.lock().unwrap().len(), 0);
    }

    #[test]
    fn test_error_handler_registration() {
        let mut handler = ErrorHandler::new();
        handler.on_error(|_| {});
        assert_eq!(handler.callbacks.lock().unwrap().len(), 1);

        handler.on_error(|_| {});
        assert_eq!(handler.callbacks.lock().unwrap().len(), 2);
    }

    #[test]
    fn test_device_lost_reason_display() {
        assert_eq!(DeviceLostReason::Destroyed.to_string(), "Destroyed");
        assert_eq!(DeviceLostReason::Unknown.to_string(), "Unknown");
    }

    #[test]
    fn test_device_lost_reason_conversion() {
        assert_eq!(
            DeviceLostReason::from(wgpu::DeviceLostReason::Unknown),
            DeviceLostReason::Unknown
        );
        assert_eq!(
            DeviceLostReason::from(wgpu::DeviceLostReason::Destroyed),
            DeviceLostReason::Destroyed
        );
    }
}
