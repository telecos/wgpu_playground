# API Coverage Tracker

The API Coverage Tracker is a runtime monitoring system that records which WebGPU APIs have been called during a session. This helps developers understand which parts of the WebGPU API surface they've exercised in their applications.

## Features

- **Automatic Tracking**: API calls are automatically tracked through wrapper functions
- **Category Organization**: APIs are organized by WebGPU categories (Buffer, Texture, Pipeline, etc.)
- **Persistent Storage**: Coverage data can be saved and loaded for session comparisons
- **UI Panel**: Interactive panel for viewing coverage statistics
- **Export/Import**: Coverage data can be exported to JSON for analysis
- **Enable/Disable**: Tracking can be toggled on/off at runtime

## Usage

### Basic Tracking

The API tracker uses a global singleton for convenience:

```rust
use wgpu_playground_core::api_coverage::{ApiCategory, ApiCoverageTracker};

// Get the global tracker
let tracker = ApiCoverageTracker::global();

// Check current coverage
println!("Total API calls tracked: {}", tracker.call_count());

// Get calls for a specific category
let buffer_calls = tracker.calls_for_category(ApiCategory::Buffer);
println!("Buffer operations: {}", buffer_calls.len());
```

### Manual Recording

You can also manually record API calls:

```rust
// Record a custom API call
tracker.record(ApiCategory::Buffer, "custom_operation");
```

### Enable/Disable Tracking

```rust
// Pause tracking
tracker.disable();

// Resume tracking
tracker.enable();

// Check status
if tracker.is_enabled() {
    println!("Tracking is active");
}
```

### Export and Import

```rust
// Export coverage data to JSON
let json = tracker.to_json().unwrap();
std::fs::write("coverage.json", json).unwrap();

// Import coverage data
let json = std::fs::read_to_string("coverage.json").unwrap();
tracker.from_json(&json).unwrap();
```

### Session Tracking

```rust
// Create a tracker with a session name
let tracker = ApiCoverageTracker::with_session_name("my_rendering_session");

// Take a snapshot of current coverage
let snapshot = tracker.snapshot();
println!("Session: {:?}", snapshot.session_name);
println!("Started: {:?}", snapshot.start_time);
```

### Merging Coverage Data

```rust
// Merge coverage from multiple sessions
let tracker1 = ApiCoverageTracker::new();
tracker1.record(ApiCategory::Buffer, "create_buffer");

let tracker2 = ApiCoverageTracker::new();
tracker2.record(ApiCategory::Texture, "create_texture");

// Merge tracker2 into tracker1
let snapshot2 = tracker2.snapshot();
tracker1.merge(&snapshot2);
```

## API Categories

The tracker organizes APIs into the following categories:

- **Device**: Device and adapter operations
- **Queue**: Queue operations (submit, write)
- **Buffer**: Buffer creation and operations
- **Texture**: Texture creation and operations
- **Sampler**: Sampler creation
- **Shader**: Shader module operations
- **RenderPipeline**: Render pipeline operations
- **ComputePipeline**: Compute pipeline operations
- **BindGroup**: Bind group and layout operations
- **PipelineLayout**: Pipeline layout operations
- **RenderPass**: Render pass operations
- **ComputePass**: Compute pass operations
- **CommandEncoder**: Command encoder operations
- **RenderBundle**: Render bundle operations
- **QuerySet**: Query set operations

## UI Panel

The API Coverage Panel provides a visual interface for viewing coverage:

```rust
use wgpu_playground_core::api_coverage_panel::ApiCoveragePanel;
use wgpu_playground_core::api_coverage::ApiCoverageTracker;

let mut panel = ApiCoveragePanel::new();
let tracker = ApiCoverageTracker::global();

// In your UI rendering code
panel.show(&ctx, &tracker);
```

### Panel Features

- **Overall Statistics**: Total API calls and approximate coverage percentage
- **Category Breakdown**: View calls by category with progress bars
- **Filtering**: Filter by category or search text
- **Export**: Save coverage data to JSON file
- **Controls**: Enable/disable tracking, reset coverage

## Integration

API tracking is automatically integrated into core wrapper functions:

- `BufferDescriptor::create_buffer()` - Tracks "create_buffer"
- `BufferOps::map_read()` - Tracks "map_read"
- `BufferOps::map_write()` - Tracks "map_write"
- `RenderPassEncoder::begin()` - Tracks "begin_render_pass"
- `RenderPassEncoder::set_pipeline()` - Tracks "set_pipeline"
- `RenderPassEncoder::draw()` - Tracks "draw"

More integrations can be added to other wrapper functions as needed.

## State Persistence

Coverage data is integrated into the `PlaygroundState` for automatic save/load:

```rust
use wgpu_playground_core::state::PlaygroundState;

let mut state = PlaygroundState::new();

// Coverage data is automatically included when saving state
state.save_to_file("playground_state.json").unwrap();

// And restored when loading
let loaded_state = PlaygroundState::load_from_file("playground_state.json").unwrap();
```

## Testing

The tracker is designed to not interfere with tests. Tracking calls are wrapped in `#[cfg(not(test))]` to prevent test pollution.

## Examples

See the integration tests in `tests/api_coverage_integration_test.rs` for comprehensive usage examples.

## Implementation Notes

- The tracker uses a `HashSet` internally, so duplicate API calls are only counted once
- All operations are thread-safe using `Arc<Mutex<T>>`
- The global singleton is implemented using `OnceLock` for both native and WASM targets
- Coverage percentage is a rough estimate based on an assumed total API surface
