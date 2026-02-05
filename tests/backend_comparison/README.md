# Backend Comparison Testing

Tools for comparing rendering outputs between wgpu-rs and Dawn backends.

## Quick Start

```bash
# Compare two PNG images
cargo run --bin backend_validator diff image1.png image2.png

# Generate HTML report
cargo run --bin backend_validator html ./output_directory/
```

## Workflow

1. Run tests that save images with naming: `testname_wgpu.png` and `testname_dawn.png`
2. Use validator tool to compare or generate reports
3. Review HTML output for visual differences

## Example

```bash
# After running visual regression tests
cargo run --bin backend_validator html tests/visual_regression/output/
```

See the wgpu_playground_core README for more details on writing comparison tests.
