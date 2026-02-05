mod common;

use wgpu_playground_core::api_coverage::{ApiCategory, ApiCoverageTracker};

#[test]
fn test_api_coverage_basic_tracking() {
    pollster::block_on(async {
        // Create a new tracker for this test
        let tracker = ApiCoverageTracker::new();

        // Manually record some API calls (simulating what the wrappers do)
        tracker.record(ApiCategory::Buffer, "create_buffer");
        tracker.record(ApiCategory::Texture, "create_texture");
        tracker.record(ApiCategory::RenderPass, "begin_render_pass");

        // Verify tracking
        assert_eq!(tracker.call_count(), 3);
        assert!(tracker.is_enabled());

        // Verify category filtering
        let buffer_calls = tracker.calls_for_category(ApiCategory::Buffer);
        assert_eq!(buffer_calls.len(), 1);
        assert_eq!(buffer_calls[0].method, "create_buffer");

        let texture_calls = tracker.calls_for_category(ApiCategory::Texture);
        assert_eq!(texture_calls.len(), 1);

        let render_calls = tracker.calls_for_category(ApiCategory::RenderPass);
        assert_eq!(render_calls.len(), 1);
    });
}

#[test]
fn test_api_coverage_enable_disable() {
    let tracker = ApiCoverageTracker::new();

    // Initially enabled
    assert!(tracker.is_enabled());
    tracker.record(ApiCategory::Buffer, "create_buffer");
    assert_eq!(tracker.call_count(), 1);

    // Disable tracking
    tracker.disable();
    assert!(!tracker.is_enabled());
    tracker.record(ApiCategory::Texture, "create_texture");
    assert_eq!(tracker.call_count(), 1); // Should not increase

    // Re-enable tracking
    tracker.enable();
    assert!(tracker.is_enabled());
    tracker.record(ApiCategory::Shader, "create_shader");
    assert_eq!(tracker.call_count(), 2); // Should increase
}

#[test]
fn test_api_coverage_reset() {
    let tracker = ApiCoverageTracker::new();

    tracker.record(ApiCategory::Buffer, "create_buffer");
    tracker.record(ApiCategory::Texture, "create_texture");
    assert_eq!(tracker.call_count(), 2);

    // Reset coverage
    tracker.reset();
    assert_eq!(tracker.call_count(), 0);

    // Can still track after reset
    tracker.record(ApiCategory::Shader, "create_shader");
    assert_eq!(tracker.call_count(), 1);
}

#[test]
fn test_api_coverage_snapshot() {
    let tracker = ApiCoverageTracker::new();

    tracker.record(ApiCategory::Buffer, "create_buffer");
    tracker.record(ApiCategory::Texture, "create_texture");

    // Take snapshot
    let snapshot = tracker.snapshot();
    assert_eq!(snapshot.call_count(), 2);

    // Verify snapshot is independent
    tracker.record(ApiCategory::Shader, "create_shader");
    assert_eq!(snapshot.call_count(), 2); // Snapshot unchanged
    assert_eq!(tracker.call_count(), 3); // Tracker updated
}

#[test]
fn test_api_coverage_json_export_import() {
    let tracker = ApiCoverageTracker::new();

    tracker.record(ApiCategory::Buffer, "create_buffer");
    tracker.record(ApiCategory::Texture, "create_texture");
    tracker.record(ApiCategory::RenderPass, "begin_render_pass");

    // Export to JSON
    let json = tracker.to_json().unwrap();
    assert!(json.contains("create_buffer"));
    assert!(json.contains("create_texture"));
    assert!(json.contains("begin_render_pass"));

    // Import into new tracker
    let tracker2 = ApiCoverageTracker::new();
    tracker2.from_json(&json).unwrap();

    assert_eq!(tracker2.call_count(), 3);
    let buffer_calls = tracker2.calls_for_category(ApiCategory::Buffer);
    assert_eq!(buffer_calls.len(), 1);
}

#[test]
fn test_api_coverage_merge() {
    let tracker1 = ApiCoverageTracker::new();
    tracker1.record(ApiCategory::Buffer, "create_buffer");
    tracker1.record(ApiCategory::Texture, "create_texture");

    let tracker2 = ApiCoverageTracker::new();
    tracker2.record(ApiCategory::Shader, "create_shader");
    tracker2.record(ApiCategory::RenderPass, "begin_render_pass");

    // Merge tracker2 into tracker1
    let snapshot2 = tracker2.snapshot();
    tracker1.merge(&snapshot2);

    assert_eq!(tracker1.call_count(), 4);
    assert_eq!(tracker1.calls_for_category(ApiCategory::Buffer).len(), 1);
    assert_eq!(tracker1.calls_for_category(ApiCategory::Texture).len(), 1);
    assert_eq!(tracker1.calls_for_category(ApiCategory::Shader).len(), 1);
    assert_eq!(
        tracker1.calls_for_category(ApiCategory::RenderPass).len(),
        1
    );
}

#[test]
fn test_api_coverage_with_session_name() {
    let tracker = ApiCoverageTracker::with_session_name("test_session");

    tracker.record(ApiCategory::Buffer, "create_buffer");

    let snapshot = tracker.snapshot();
    assert_eq!(snapshot.session_name, Some("test_session".to_string()));
    assert!(snapshot.start_time.is_some());
    assert_eq!(snapshot.call_count(), 1);
}

#[test]
fn test_api_coverage_category_stats() {
    let tracker = ApiCoverageTracker::new();

    // Record multiple calls in different categories
    tracker.record(ApiCategory::Buffer, "create_buffer");
    tracker.record(ApiCategory::Buffer, "write_buffer");
    tracker.record(ApiCategory::Texture, "create_texture");
    tracker.record(ApiCategory::RenderPass, "begin_render_pass");
    tracker.record(ApiCategory::RenderPass, "draw");

    let snapshot = tracker.snapshot();

    // Verify category counts
    assert_eq!(snapshot.category_count(ApiCategory::Buffer), 2);
    assert_eq!(snapshot.category_count(ApiCategory::Texture), 1);
    assert_eq!(snapshot.category_count(ApiCategory::RenderPass), 2);
    assert_eq!(snapshot.category_count(ApiCategory::Shader), 0);

    // Verify used categories
    let used = snapshot.used_categories();
    assert_eq!(used.len(), 3);
    assert!(used.contains(&ApiCategory::Buffer));
    assert!(used.contains(&ApiCategory::Texture));
    assert!(used.contains(&ApiCategory::RenderPass));
}

#[test]
fn test_api_coverage_duplicate_calls() {
    let tracker = ApiCoverageTracker::new();

    // Record the same API call multiple times
    tracker.record(ApiCategory::Buffer, "create_buffer");
    tracker.record(ApiCategory::Buffer, "create_buffer");
    tracker.record(ApiCategory::Buffer, "create_buffer");

    // Should only be tracked once (HashSet behavior)
    assert_eq!(tracker.call_count(), 1);
    assert_eq!(tracker.calls_for_category(ApiCategory::Buffer).len(), 1);
}
