# Visual Regression Testing Framework

## Overview

The visual regression testing framework allows you to automatically test GPU rendering output by comparing rendered images against known-good reference images. This catches unintended visual changes in your rendering code.

## Features

- **Automatic Texture Capture**: Captures GPU textures to RGBA PNG images
- **Image Comparison**: Pixel-by-pixel comparison with configurable tolerance
- **Difference Visualization**: Generates red-highlighted diff images on failures
- **Reference Management**: Easy workflow for creating and updating reference images
- **CI-Friendly**: Gracefully skips tests in environments without GPU

## Quick Start

### Running Tests

```bash
# Run all visual regression tests
cargo test --package wgpu_playground_core visual_regression

# Run a specific visual test
cargo test --package wgpu_playground_core test_visual_regression_triangle
```

### Creating Reference Images

On a machine with a GPU (local development machine):

```bash
# Generate reference images for all visual tests
UPDATE_VISUAL_REFERENCES=1 cargo test --package wgpu_playground_core visual_regression
```

This will:
1. Run all visual regression tests
2. Save captured images as new reference images
3. All tests will pass (no comparison performed)

## Writing Visual Regression Tests

### Basic Example

```rust
use wgpu_playground_core::visual_regression::*;
use wgpu_playground_core::visual_regression::test_utils::*;
use wgpu_playground_core::assert_visual_match;

#[test]
fn test_my_rendering() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU available");
            return;
        };

        let result = run_visual_test(
            "my_rendering",  // Test name (will look for reference/my_rendering.png)
            &device,
            &queue,
            |device, queue| {
                // Your rendering code here
                // Must return a wgpu::Texture
                render_my_scene(device, queue)
            },
            ComparisonConfig::default(),
        )
        .await;

        match result {
            Ok(comparison) => {
                assert_visual_match!(comparison);
                println!("✓ Visual test passed");
            }
            Err(VisualRegressionError::ReferenceLoadError(msg)) => {
                eprintln!("Note: {}", msg);
                eprintln!("Run with UPDATE_VISUAL_REFERENCES=1 to create reference");
            }
            Err(e) => panic!("Visual test failed: {}", e),
        }
    });
}
```

### Manual Texture Capture

For more control, you can manually capture and compare textures:

```rust
use wgpu_playground_core::visual_regression::*;

#[test]
fn test_manual_capture() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            return;
        };

        // Render to texture
        let texture = create_and_render_texture(&device, &queue);

        // Capture texture to image
        let image = capture_texture(&device, &queue, &texture)
            .await
            .unwrap();

        // Compare with reference
        let result = compare_with_reference(
            &image,
            "test_name",
            ComparisonConfig::default(),
        )
        .unwrap();

        assert!(result.is_match);
    });
}
```

## Configuration

### ComparisonConfig

Control how images are compared:

```rust
let config = ComparisonConfig {
    threshold: 0.01,           // 1% difference allowed (0.0-1.0)
    save_diff: true,           // Save diff images on failure
    update_references: false,  // Set to true to update references
};
```

**Threshold**: The maximum allowed average per-pixel difference (0.0 = exact match, 1.0 = completely different).

**Environment Variables**:
- `UPDATE_VISUAL_REFERENCES=1` - Automatically sets `update_references: true` in default config

### Custom Threshold

```rust
let config = ComparisonConfig {
    threshold: 0.05,  // Allow 5% difference
    ..Default::default()
};

let result = run_visual_test("test", &device, &queue, render_fn, config)
    .await
    .unwrap();

assert_visual_match!(result, 0.05);  // Check against custom threshold
```

## Directory Structure

```
tests/visual_regression/
├── reference/              # Reference images (committed to git)
│   ├── README.md          # This documentation
│   ├── triangle.png       # Reference for triangle test
│   └── solid_quad.png     # Reference for quad test
│
└── output/                 # Test outputs (not committed, in .gitignore)
    ├── triangle.png       # Captured output from test
    ├── triangle_diff.png  # Difference image (only created on failure)
    ├── solid_quad.png
    └── solid_quad_diff.png
```

## Best Practices

### 1. Keep Images Small

Use minimal resolution needed for the test:

```rust
let texture = create_test_render_target(&device, 256, 256);  // Not 4096x4096!
```

### 2. Test Specific Features

Each test should focus on one rendering feature:

```rust
// Good - tests one thing
test_triangle_blending()
test_triangle_culling()
test_triangle_depth()

// Bad - tests everything at once
test_entire_scene()
```

### 3. Use Descriptive Names

```rust
// Good
run_visual_test("pbr_material_metallic", ...)

// Bad
run_visual_test("test1", ...)
```

### 4. Handle Missing References

Always handle the case where reference images don't exist:

```rust
match result {
    Ok(comparison) => assert_visual_match!(comparison),
    Err(VisualRegressionError::ReferenceLoadError(msg)) => {
        eprintln!("{}", msg);
        // Don't panic - let user know how to fix
    }
    Err(e) => panic!("Test failed: {}", e),
}
```

### 5. Review Changes Carefully

When a visual test fails:

1. Check the diff image in `tests/visual_regression/output/`
2. Determine if the change is intentional
3. If intentional, update the reference:
   ```bash
   UPDATE_VISUAL_REFERENCES=1 cargo test test_name
   ```
4. Commit the updated reference image

## Troubleshooting

### "Reference image not found"

**Solution**: Generate reference images:
```bash
UPDATE_VISUAL_REFERENCES=1 cargo test visual_regression
```

### "Dimension mismatch"

Your render target size changed. Either:
- Update your test to match the reference size
- Regenerate the reference with the new size

### "Skipping test: No GPU available"

This is normal in CI environments without GPU. Tests will:
- Pass without running actual GPU code
- Print a message to stderr
- Not fail the test suite

### Small Differences Failing Tests

GPU drivers, backends, or hardware may produce slightly different output. Solutions:

1. **Increase threshold**:
   ```rust
   let config = ComparisonConfig {
       threshold: 0.02,  // Allow 2% difference
       ..Default::default()
   };
   ```

2. **Update reference** if new output is correct:
   ```bash
   UPDATE_VISUAL_REFERENCES=1 cargo test test_name
   ```

## API Reference

### Core Functions

#### `capture_texture`
```rust
pub async fn capture_texture(
    device: &Device,
    queue: &Queue,
    texture: &Texture,
) -> Result<RgbaImage, VisualRegressionError>
```

Captures a GPU texture to an RGBA image.

#### `compare_with_reference`
```rust
pub fn compare_with_reference(
    captured: &RgbaImage,
    test_name: &str,
    config: ComparisonConfig,
) -> Result<ComparisonResult, VisualRegressionError>
```

Compares a captured image with its reference.

### Helper Functions

#### `run_visual_test`
```rust
pub async fn run_visual_test<F>(
    test_name: &str,
    device: &Device,
    queue: &Queue,
    render_fn: F,
    config: ComparisonConfig,
) -> Result<ComparisonResult, VisualRegressionError>
where
    F: FnOnce(&Device, &Queue) -> wgpu::Texture
```

High-level helper that renders, captures, and compares in one call.

#### `create_test_render_target`
```rust
pub fn create_test_render_target(
    device: &Device,
    width: u32,
    height: u32,
) -> wgpu::Texture
```

Creates a texture suitable for rendering and capturing.

### Macros

#### `assert_visual_match!`
```rust
assert_visual_match!(result);                    // Use result.is_match
assert_visual_match!(result, threshold);         // Check custom threshold
```

Asserts that a visual comparison passed, with helpful error messages.

## Examples

See the following test files for examples:
- `crates/wgpu_playground_core/tests/visual_regression_test.rs` - Triangle and quad rendering tests

## Implementation Details

### Image Capture Process

1. Create a buffer with `MAP_READ` usage
2. Copy texture data to buffer using `copy_texture_to_buffer`
3. Map buffer and read pixel data
4. Handle row padding (GPU requires aligned row sizes)
5. Create `RgbaImage` from pixel data

### Comparison Algorithm

1. Load reference image (or create if `update_references` is true)
2. Compare dimensions
3. For each pixel:
   - Calculate per-channel absolute difference (0.0-1.0)
   - Average the four channels
   - Accumulate total difference
4. Calculate average difference across all pixels
5. Compare against threshold
6. Generate red-highlighted diff image if failed

### Diff Image Format

- Pixels with no difference: Black (RGB 0,0,0)
- Pixels with difference: Red with intensity proportional to difference
- Makes visual changes immediately obvious

## Future Enhancements

Potential improvements to the framework:

- [ ] Perceptual diff algorithms (SSIM, etc.)
- [ ] Region-based comparison (ignore certain areas)
- [ ] Automatic threshold calculation
- [ ] HTML report generation
- [ ] Parallel test execution optimization
- [ ] GPU-accelerated comparison
- [ ] Video/animation regression testing

## Contributing

When adding visual regression tests:

1. Write the test
2. Generate reference images locally (with GPU)
3. Verify the reference image looks correct
4. Commit both test code and reference images
5. Document what the test validates

## License

MIT - Same as the main project
