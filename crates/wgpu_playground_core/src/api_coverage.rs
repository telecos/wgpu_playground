//! API Coverage Tracker Module
//!
//! This module provides runtime tracking of WebGPU API usage during a session.
//! It helps users understand which parts of the API they've exercised.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

/// Categories of WebGPU APIs matching the WebGPU spec structure
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ApiCategory {
    /// Device and adapter operations
    Device,
    /// Queue operations (submit, write)
    Queue,
    /// Buffer creation and operations
    Buffer,
    /// Texture creation and operations
    Texture,
    /// Sampler creation
    Sampler,
    /// Shader module operations
    Shader,
    /// Render pipeline operations
    RenderPipeline,
    /// Compute pipeline operations
    ComputePipeline,
    /// Bind group and layout operations
    BindGroup,
    /// Pipeline layout operations
    PipelineLayout,
    /// Render pass operations
    RenderPass,
    /// Compute pass operations
    ComputePass,
    /// Command encoder operations
    CommandEncoder,
    /// Render bundle operations
    RenderBundle,
    /// Query set operations
    QuerySet,
}

impl ApiCategory {
    /// Get a human-readable name for the category
    pub fn name(&self) -> &'static str {
        match self {
            ApiCategory::Device => "Device",
            ApiCategory::Queue => "Queue",
            ApiCategory::Buffer => "Buffer",
            ApiCategory::Texture => "Texture",
            ApiCategory::Sampler => "Sampler",
            ApiCategory::Shader => "Shader",
            ApiCategory::RenderPipeline => "Render Pipeline",
            ApiCategory::ComputePipeline => "Compute Pipeline",
            ApiCategory::BindGroup => "Bind Group",
            ApiCategory::PipelineLayout => "Pipeline Layout",
            ApiCategory::RenderPass => "Render Pass",
            ApiCategory::ComputePass => "Compute Pass",
            ApiCategory::CommandEncoder => "Command Encoder",
            ApiCategory::RenderBundle => "Render Bundle",
            ApiCategory::QuerySet => "Query Set",
        }
    }

    /// Get all categories
    pub fn all() -> Vec<ApiCategory> {
        vec![
            ApiCategory::Device,
            ApiCategory::Queue,
            ApiCategory::Buffer,
            ApiCategory::Texture,
            ApiCategory::Sampler,
            ApiCategory::Shader,
            ApiCategory::RenderPipeline,
            ApiCategory::ComputePipeline,
            ApiCategory::BindGroup,
            ApiCategory::PipelineLayout,
            ApiCategory::RenderPass,
            ApiCategory::ComputePass,
            ApiCategory::CommandEncoder,
            ApiCategory::RenderBundle,
            ApiCategory::QuerySet,
        ]
    }
}

/// Specific API calls that can be tracked
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ApiCall {
    /// The category this API call belongs to
    pub category: ApiCategory,
    /// The specific API method name
    pub method: String,
}

impl ApiCall {
    /// Create a new API call
    pub fn new(category: ApiCategory, method: impl Into<String>) -> Self {
        Self {
            category,
            method: method.into(),
        }
    }
}

/// Serializable coverage data for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageData {
    /// Set of API calls that have been made
    pub calls: HashSet<ApiCall>,
    /// Session name or identifier
    pub session_name: Option<String>,
    /// Timestamp when coverage tracking started
    pub start_time: Option<String>,
}

impl Default for CoverageData {
    fn default() -> Self {
        Self {
            calls: HashSet::new(),
            session_name: None,
            start_time: None,
        }
    }
}

impl CoverageData {
    /// Create new coverage data
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with a session name
    pub fn with_session_name(name: impl Into<String>) -> Self {
        Self {
            calls: HashSet::new(),
            session_name: Some(name.into()),
            start_time: Some(chrono::Utc::now().to_rfc3339()),
        }
    }

    /// Get the number of unique API calls made
    pub fn call_count(&self) -> usize {
        self.calls.len()
    }

    /// Get calls by category
    pub fn calls_by_category(&self, category: ApiCategory) -> Vec<&ApiCall> {
        self.calls
            .iter()
            .filter(|call| call.category == category)
            .collect()
    }

    /// Get count of calls by category
    pub fn category_count(&self, category: ApiCategory) -> usize {
        self.calls_by_category(category).len()
    }

    /// Get all categories that have been used
    pub fn used_categories(&self) -> Vec<ApiCategory> {
        let mut categories: Vec<_> = self
            .calls
            .iter()
            .map(|call| call.category)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        categories.sort_by_key(|c| c.name());
        categories
    }

    /// Calculate coverage percentage (based on predefined API count)
    /// This is a rough estimate - actual WebGPU API surface is larger
    pub fn coverage_percentage(&self) -> f64 {
        // Rough estimate of total WebGPU API methods
        const ESTIMATED_TOTAL_APIS: usize = 100;
        (self.call_count() as f64 / ESTIMATED_TOTAL_APIS as f64 * 100.0).min(100.0)
    }
}

/// Thread-safe API coverage tracker
#[derive(Clone)]
pub struct ApiCoverageTracker {
    data: Arc<Mutex<CoverageData>>,
    enabled: Arc<Mutex<bool>>,
}

impl Default for ApiCoverageTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiCoverageTracker {
    /// Create a new API coverage tracker
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(CoverageData::new())),
            enabled: Arc::new(Mutex::new(true)),
        }
    }

    /// Create a tracker with a session name
    pub fn with_session_name(name: impl Into<String>) -> Self {
        Self {
            data: Arc::new(Mutex::new(CoverageData::with_session_name(name))),
            enabled: Arc::new(Mutex::new(true)),
        }
    }

    /// Get the global API coverage tracker (thread-local singleton)
    ///
    /// This provides a convenient way to access a shared tracker instance
    /// across the application without needing to pass it explicitly.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn global() -> &'static ApiCoverageTracker {
        use std::sync::OnceLock;
        static GLOBAL_TRACKER: OnceLock<ApiCoverageTracker> = OnceLock::new();
        GLOBAL_TRACKER.get_or_init(|| ApiCoverageTracker::new())
    }

    /// Get the global API coverage tracker (WASM version)
    #[cfg(target_arch = "wasm32")]
    pub fn global() -> &'static ApiCoverageTracker {
        use std::sync::OnceLock;
        static GLOBAL_TRACKER: OnceLock<ApiCoverageTracker> = OnceLock::new();
        GLOBAL_TRACKER.get_or_init(|| ApiCoverageTracker::new())
    }

    /// Record an API call
    pub fn record(&self, category: ApiCategory, method: impl Into<String>) {
        if !*self.enabled.lock().unwrap() {
            return;
        }

        let call = ApiCall::new(category, method);
        let mut data = self.data.lock().unwrap();
        data.calls.insert(call);
    }

    /// Enable tracking
    pub fn enable(&self) {
        *self.enabled.lock().unwrap() = true;
    }

    /// Disable tracking
    pub fn disable(&self) {
        *self.enabled.lock().unwrap() = false;
    }

    /// Check if tracking is enabled
    pub fn is_enabled(&self) -> bool {
        *self.enabled.lock().unwrap()
    }

    /// Get a snapshot of the current coverage data
    pub fn snapshot(&self) -> CoverageData {
        self.data.lock().unwrap().clone()
    }

    /// Reset all tracked data
    pub fn reset(&self) {
        let mut data = self.data.lock().unwrap();
        *data = CoverageData::new();
    }

    /// Reset with a new session name
    pub fn reset_with_session(&self, name: impl Into<String>) {
        let mut data = self.data.lock().unwrap();
        *data = CoverageData::with_session_name(name);
    }

    /// Get the current call count
    pub fn call_count(&self) -> usize {
        self.data.lock().unwrap().call_count()
    }

    /// Get calls for a specific category
    pub fn calls_for_category(&self, category: ApiCategory) -> Vec<ApiCall> {
        self.data
            .lock()
            .unwrap()
            .calls_by_category(category)
            .into_iter()
            .cloned()
            .collect()
    }

    /// Export coverage data as JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        let data = self.snapshot();
        serde_json::to_string_pretty(&data)
    }

    /// Import coverage data from JSON
    pub fn from_json(&self, json: &str) -> Result<(), serde_json::Error> {
        let imported: CoverageData = serde_json::from_str(json)?;
        let mut data = self.data.lock().unwrap();
        *data = imported;
        Ok(())
    }

    /// Merge coverage data from another tracker
    pub fn merge(&self, other: &CoverageData) {
        let mut data = self.data.lock().unwrap();
        for call in &other.calls {
            data.calls.insert(call.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_call_creation() {
        let call = ApiCall::new(ApiCategory::Buffer, "create_buffer");
        assert_eq!(call.category, ApiCategory::Buffer);
        assert_eq!(call.method, "create_buffer");
    }

    #[test]
    fn test_coverage_data_basic() {
        let mut data = CoverageData::new();
        assert_eq!(data.call_count(), 0);

        data.calls
            .insert(ApiCall::new(ApiCategory::Buffer, "create_buffer"));
        assert_eq!(data.call_count(), 1);

        data.calls
            .insert(ApiCall::new(ApiCategory::Texture, "create_texture"));
        assert_eq!(data.call_count(), 2);
    }

    #[test]
    fn test_coverage_data_by_category() {
        let mut data = CoverageData::new();
        data.calls
            .insert(ApiCall::new(ApiCategory::Buffer, "create_buffer"));
        data.calls
            .insert(ApiCall::new(ApiCategory::Buffer, "write_buffer"));
        data.calls
            .insert(ApiCall::new(ApiCategory::Texture, "create_texture"));

        assert_eq!(data.category_count(ApiCategory::Buffer), 2);
        assert_eq!(data.category_count(ApiCategory::Texture), 1);
        assert_eq!(data.category_count(ApiCategory::Shader), 0);
    }

    #[test]
    fn test_tracker_basic_operations() {
        let tracker = ApiCoverageTracker::new();
        assert_eq!(tracker.call_count(), 0);
        assert!(tracker.is_enabled());

        tracker.record(ApiCategory::Buffer, "create_buffer");
        assert_eq!(tracker.call_count(), 1);

        tracker.record(ApiCategory::Texture, "create_texture");
        assert_eq!(tracker.call_count(), 2);
    }

    #[test]
    fn test_tracker_enable_disable() {
        let tracker = ApiCoverageTracker::new();
        tracker.record(ApiCategory::Buffer, "create_buffer");
        assert_eq!(tracker.call_count(), 1);

        tracker.disable();
        assert!(!tracker.is_enabled());
        tracker.record(ApiCategory::Texture, "create_texture");
        assert_eq!(tracker.call_count(), 1); // Should not increase

        tracker.enable();
        assert!(tracker.is_enabled());
        tracker.record(ApiCategory::Shader, "create_shader");
        assert_eq!(tracker.call_count(), 2); // Should increase
    }

    #[test]
    fn test_tracker_reset() {
        let tracker = ApiCoverageTracker::new();
        tracker.record(ApiCategory::Buffer, "create_buffer");
        tracker.record(ApiCategory::Texture, "create_texture");
        assert_eq!(tracker.call_count(), 2);

        tracker.reset();
        assert_eq!(tracker.call_count(), 0);
    }

    #[test]
    fn test_tracker_snapshot() {
        let tracker = ApiCoverageTracker::new();
        tracker.record(ApiCategory::Buffer, "create_buffer");

        let snapshot = tracker.snapshot();
        assert_eq!(snapshot.call_count(), 1);

        // Further changes shouldn't affect snapshot
        tracker.record(ApiCategory::Texture, "create_texture");
        assert_eq!(snapshot.call_count(), 1);
        assert_eq!(tracker.call_count(), 2);
    }

    #[test]
    fn test_json_serialization() {
        let tracker = ApiCoverageTracker::new();
        tracker.record(ApiCategory::Buffer, "create_buffer");
        tracker.record(ApiCategory::Texture, "create_texture");

        let json = tracker.to_json().unwrap();
        assert!(json.contains("create_buffer"));
        assert!(json.contains("create_texture"));

        let tracker2 = ApiCoverageTracker::new();
        tracker2.from_json(&json).unwrap();
        assert_eq!(tracker2.call_count(), 2);
    }

    #[test]
    fn test_merge_coverage() {
        let tracker1 = ApiCoverageTracker::new();
        tracker1.record(ApiCategory::Buffer, "create_buffer");

        let mut data2 = CoverageData::new();
        data2
            .calls
            .insert(ApiCall::new(ApiCategory::Texture, "create_texture"));

        tracker1.merge(&data2);
        assert_eq!(tracker1.call_count(), 2);
    }

    #[test]
    fn test_session_name() {
        let data = CoverageData::with_session_name("test_session");
        assert_eq!(data.session_name, Some("test_session".to_string()));
        assert!(data.start_time.is_some());
    }

    #[test]
    fn test_used_categories() {
        let mut data = CoverageData::new();
        data.calls
            .insert(ApiCall::new(ApiCategory::Buffer, "create_buffer"));
        data.calls
            .insert(ApiCall::new(ApiCategory::Buffer, "write_buffer"));
        data.calls
            .insert(ApiCall::new(ApiCategory::Texture, "create_texture"));

        let categories = data.used_categories();
        assert_eq!(categories.len(), 2);
        assert!(categories.contains(&ApiCategory::Buffer));
        assert!(categories.contains(&ApiCategory::Texture));
    }

    #[test]
    fn test_category_names() {
        assert_eq!(ApiCategory::Buffer.name(), "Buffer");
        assert_eq!(ApiCategory::RenderPipeline.name(), "Render Pipeline");
        assert_eq!(ApiCategory::ComputePass.name(), "Compute Pass");
    }

    #[test]
    fn test_all_categories() {
        let categories = ApiCategory::all();
        assert!(categories.len() >= 15);
        assert!(categories.contains(&ApiCategory::Buffer));
        assert!(categories.contains(&ApiCategory::RenderPass));
    }
}
