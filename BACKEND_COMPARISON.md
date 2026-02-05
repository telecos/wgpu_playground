# Backend Comparison Testing

Compare wgpu-rs and Dawn backend behavior using existing infrastructure.

## Quick Start

Run examples with different backends:
```bash
./scripts/compare_example.sh triangle
```

## Backend Comparison in Tests

Use the existing visual_regression module to compare backends:

```rust
use wgpu_playground_core::visual_regression::{capture_texture, compare_images};
use wgpu_playground_core::implementation::WebGPUImplementation;

// Run with wgpu-rs
std::env::set_var("WEBGPU_IMPL", "wgpu");
let (device1, queue1, texture1) = setup_and_render();
let img1 = capture_texture(&device1, &queue1, &texture1).await?;

// Run with Dawn  
std::env::set_var("WEBGPU_IMPL", "dawn");
let (device2, queue2, texture2) = setup_and_render();
let img2 = capture_texture(&device2, &queue2, &texture2).await?;

// Compare images
let result = compare_images(&img1, &img2, 0.01)?;
assert!(result.is_match, "Backend outputs differ by {:.2}%", result.difference * 100.0);
```

## Side-by-Side Visualization

Save comparison images:

```rust
use wgpu_playground_core::visual_regression::test_utils::save_side_by_side;

save_side_by_side(&wgpu_image, &dawn_image, Path::new("comparison.png"))?;
```

## Performance Comparison

Compare performance metrics:

```rust
use wgpu_playground_core::performance_metrics::PerformanceMetrics;

let mut perf_wgpu = PerformanceMetrics::new();
perf_wgpu.start_frame();
// ... render with wgpu ...
perf_wgpu.end_frame();

let mut perf_dawn = PerformanceMetrics::new();
perf_dawn.start_frame();
// ... render with Dawn ...
perf_dawn.end_frame();

println!("wgpu FPS: {}, Dawn FPS: {}", perf_wgpu.fps(), perf_dawn.fps());
println!("wgpu frame time: {}ms, Dawn frame time: {}ms", 
    perf_wgpu.average_frame_time_ms(), perf_dawn.average_frame_time_ms());
```

## Features

- **Visual Comparison**: Uses `compare_images()` with configurable tolerance
- **Performance Tracking**: Frame time, FPS, memory estimates via PerformanceMetrics
- **Side-by-Side Reports**: `save_side_by_side()` helper for visual reports
- **Automated Testing**: Shell script to run examples with both backends
