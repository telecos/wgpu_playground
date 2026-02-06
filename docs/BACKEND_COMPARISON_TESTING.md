# Backend Comparison Testing Framework

This testing framework enables cross-validation of rendering between wgpu-rs and Dawn backends, ensuring consistent visual output and tracking performance characteristics.

## Overview

The framework extends the existing visual regression testing infrastructure to support:

- **Automated screenshot capture** for both backends
- **Pixel-difference comparison** with configurable tolerance
- **Performance benchmarking** (frame time measurements)
- **HTML report generation** with side-by-side image comparisons

## Architecture

### Components

1. **Backend Cross-Check Tests** (`tests/backend_crosscheck_test.rs`)
   - Integration tests that run identical rendering code on wgpu-rs
   - Compares output against reference images
   - Framework extensible for Dawn when fully integrated

2. **HTML Report Generator** (`src/report_html.rs`)
   - Creates visual comparison reports
   - Side-by-side image display
   - Performance metrics tables
   - Clean, responsive HTML output

3. **Visual Regression Foundation** (`src/visual_regression/`)
   - Existing texture capture utilities
   - Image comparison with threshold
   - Reference image management

## Usage

### Running Cross-Check Tests

```bash
# Run all backend comparison tests
cargo test --package wgpu_playground_core backend_crosscheck

# Create initial reference images
UPDATE_VISUAL_REFERENCES=1 cargo test backend_crosscheck
```

### Generating HTML Reports

```rust
use wgpu_playground_core::report_html::ReportBuilder;

let mut report = ReportBuilder::new("triangle_test");

// Add visual comparison
report.add_image_pair(
    Path::new("output/triangle_wgpu.png"),
    Path::new("output/triangle_dawn.png"),
    "Colored triangle rendering"
);

// Add performance metrics
report.add_metrics_table(10.5, Some(12.3));

// Save report
report.save_to(Path::new("output/report.html"))?;
```

### Writing Comparison Tests

```rust
#[test]
fn backend_cross_check_my_example() {
    pollster::block_on(async {
        let Some((dev, q)) = create_test_device().await else {
            return;  // Skip if no GPU
        };

        // Your rendering code
        let texture = my_render_function(&dev, &q);
        
        // Capture and compare
        let img = capture_texture(&dev, &q, &texture).await.unwrap();
        let result = compare_with_reference(&img, "my_test", cfg).unwrap();
        
        assert!(result.is_match, "Visual mismatch: {:.2}%", result.difference * 100.0);
    });
}
```

## Configuration

### Visual Comparison Thresholds

Default threshold is 2% pixel difference. Adjust in `ComparisonConfig`:

```rust
let cfg = ComparisonConfig {
    threshold: 0.01,  // 1% tolerance
    save_diff: true,
    update_references: false,
};
```

### Output Directories

- **Reference images**: `tests/visual_regression/reference/`
- **Test outputs**: `tests/visual_regression/output/`
- **Diff images**: `tests/visual_regression/output/*_diff.png`
- **HTML reports**: `tests/dual_backend_artifacts/`

## Test Artifacts

When tests run, they generate:

1. **Output images** - Captured rendering from each backend
2. **Diff images** - Red-channel visualization of pixel differences
3. **HTML reports** - Complete comparison with metrics

## Performance Metrics

The framework measures:

- **Average frame time** - Mean render duration across samples
- **Relative performance** - Ratio between backends
- **Statistical data** - Median, P95, P99 percentiles (when using timing module)

## CI Integration

Tests automatically detect headless environments via `CI` or `WGPU_HEADLESS` environment variables and use software rendering when necessary.

Skipped tests in CI:
```rust
#[cfg_attr(all(target_os = "linux", target_env = "gnu"), ignore = "Requires hardware GPU")]
```

## Extending for Dawn

The framework is designed for easy Dawn integration:

1. Add Dawn device creation in test harness
2. Run identical render function on both backends
3. Compare outputs automatically
4. Generate unified report

Current Dawn status: Stubbed for future implementation when Dawn FFI is fully available.

## Examples

### Triangle Cross-Check

```bash
cargo test backend_cross_check_triangle
```

This test:
1. Renders a colored triangle using wgpu-rs
2. Captures the output texture
3. Compares against reference image
4. Reports pixel difference percentage

### Generating Comparison Reports

See `examples/generate_comparison_report.rs` (to be added) for complete example of report generation workflow.

## Troubleshooting

### "Reference image not found"

Create baseline images:
```bash
UPDATE_VISUAL_REFERENCES=1 cargo test backend_crosscheck
```

### "No GPU adapter available"

Tests automatically skip when no GPU is present. This is normal in some CI environments.

### High pixel difference

- Check rendering code for non-determinism (timestamps, randomness)
- Verify shader consistency
- Consider increasing threshold for minor driver differences

## Future Enhancements

- [ ] Full Dawn backend integration
- [ ] Memory usage tracking
- [ ] Automated regression detection in CI
- [ ] Batch test runner for all examples
- [ ] JSON export for programmatic analysis
