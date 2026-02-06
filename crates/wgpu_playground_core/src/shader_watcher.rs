use std::path::PathBuf;

#[cfg(not(target_arch = "wasm32"))]
use std::sync::mpsc::{channel, Receiver};
#[cfg(not(target_arch = "wasm32"))]
use std::sync::{Arc, Mutex};

/// Represents a shader file change event
#[derive(Debug, Clone)]
pub struct ShaderChangeEvent {
    /// The filename of the shader that changed (e.g., "triangle.wgsl")
    pub filename: String,
    /// The full path to the shader file
    pub path: PathBuf,
}

/// A shader file watcher that monitors shader directory for changes
///
/// This is only available on native platforms (not WASM).
/// On WASM platforms, the watcher can be created but will not detect any changes.
#[cfg(not(target_arch = "wasm32"))]
pub struct ShaderWatcher {
    _watcher: notify::RecommendedWatcher,
    receiver: Arc<Mutex<Receiver<ShaderChangeEvent>>>,
}

/// Type alias for the result type returned by ShaderWatcher operations
type WatcherResult<T> = Result<T, Box<dyn std::error::Error>>;

#[cfg(not(target_arch = "wasm32"))]
impl ShaderWatcher {
    /// Create a new shader watcher
    ///
    /// # Returns
    /// A Result containing the ShaderWatcher or an error
    ///
    /// # Examples
    /// ```no_run
    /// use wgpu_playground_core::shader_watcher::ShaderWatcher;
    ///
    /// let watcher = ShaderWatcher::new().unwrap();
    /// // Poll for changes
    /// if let Some(event) = watcher.poll() {
    ///     println!("Shader changed: {}", event.filename);
    /// }
    /// ```
    pub fn new() -> WatcherResult<Self> {
        use notify::{Event, EventKind, RecursiveMode, Watcher};

        let (tx, rx) = channel();
        let tx = Arc::new(Mutex::new(tx));

        let shader_dir = crate::assets::shaders_dir();

        log::info!("Starting shader watcher for directory: {:?}", shader_dir);

        let watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
            match res {
                Ok(event) => {
                    // Only process modify events for .wgsl files
                    if matches!(event.kind, EventKind::Modify(_)) {
                        for path in event.paths {
                            if path.extension().and_then(|s| s.to_str()) == Some("wgsl") {
                                if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                                    log::info!("Detected shader change: {}", filename);
                                    let change_event = ShaderChangeEvent {
                                        filename: filename.to_string(),
                                        path: path.clone(),
                                    };

                                    if let Ok(tx) = tx.lock() {
                                        let _ = tx.send(change_event);
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => log::error!("Shader watcher error: {:?}", e),
            }
        })?;

        // Watch the shader directory
        let mut watcher = watcher;
        watcher.watch(&shader_dir, RecursiveMode::NonRecursive)?;

        Ok(Self {
            _watcher: watcher,
            receiver: Arc::new(Mutex::new(rx)),
        })
    }

    /// Poll for shader change events
    ///
    /// This is non-blocking and returns None if no events are pending
    ///
    /// # Returns
    /// Some(ShaderChangeEvent) if a shader changed, None otherwise
    pub fn poll(&self) -> Option<ShaderChangeEvent> {
        if let Ok(rx) = self.receiver.lock() {
            rx.try_recv().ok()
        } else {
            None
        }
    }

    /// Get all pending shader change events
    ///
    /// # Returns
    /// A vector of all pending shader change events
    pub fn poll_all(&self) -> Vec<ShaderChangeEvent> {
        let mut events = Vec::new();
        while let Some(event) = self.poll() {
            events.push(event);
        }
        events
    }
}

// WASM stub implementation
/// WASM stub that provides the same API but without file watching functionality
#[cfg(target_arch = "wasm32")]
pub struct ShaderWatcher;

#[cfg(target_arch = "wasm32")]
impl ShaderWatcher {
    /// Create a new shader watcher (WASM stub - does nothing)
    ///
    /// On WASM, this creates a stub that never reports any file changes
    /// since file system access is not available in browsers.
    pub fn new() -> WatcherResult<Self> {
        Ok(Self)
    }

    pub fn poll(&self) -> Option<ShaderChangeEvent> {
        None
    }

    pub fn poll_all(&self) -> Vec<ShaderChangeEvent> {
        Vec::new()
    }
}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
    use super::*;

    #[test]
    fn test_shader_watcher_creation() {
        // Just test that we can create a watcher without crashing
        let watcher = ShaderWatcher::new();
        assert!(watcher.is_ok());
    }

    #[test]
    fn test_shader_watcher_poll_empty() {
        let watcher = ShaderWatcher::new().unwrap();
        // Should return None when no changes
        assert!(watcher.poll().is_none());
    }

    #[test]
    fn test_shader_watcher_poll_all_empty() {
        let watcher = ShaderWatcher::new().unwrap();
        // Should return empty vec when no changes
        assert!(watcher.poll_all().is_empty());
    }
}
