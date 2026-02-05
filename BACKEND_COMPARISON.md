# Backend Comparison Testing

Compare wgpu-rs and Dawn backend behavior using existing infrastructure.

## Usage

```bash
./scripts/compare_example.sh triangle
```

This runs the example with both `WEBGPU_IMPL=wgpu` and `WEBGPU_IMPL=dawn` and saves logs.

## Visual Comparison

The visual_regression module already has `capture_texture()`. To use it in tests:

```rust
use wgpu_playground_core::visual_regression::capture_texture;

// Capture with wgpu
std::env::set_var("WEBGPU_IMPL", "wgpu");
let img1 = capture_texture(&device, &queue, &texture).await?;

// Capture with Dawn  
std::env::set_var("WEBGPU_IMPL", "dawn");
let img2 = capture_texture(&device, &queue, &texture).await?;
```

## Performance Comparison

The PerformanceMetrics struct tracks frame times. Compare snapshots:

```rust
use wgpu_playground_core::performance_metrics::PerformanceMetrics;

let mut perf = PerformanceMetrics::new();
perf.start_frame();
// ... render ...
perf.end_frame();

println!("FPS: {}", perf.fps());
println!("Frame time: {}ms", perf.average_frame_time_ms());
```
