# Visual Regression Test Reference Images

This directory contains reference images for visual regression tests.

## About Visual Regression Testing

Visual regression tests compare rendered GPU output against known-good reference images to catch unintended visual changes.

## Directory Structure

- `reference/` - Reference images (committed to repository)
- `output/` - Test output and diff images (not committed, auto-generated during tests)

## Reference Images

Reference images are PNG files named after their corresponding test (e.g., `triangle_test.png`).

### Updating Reference Images

To update or create reference images, run tests with the `UPDATE_VISUAL_REFERENCES` environment variable:

```bash
UPDATE_VISUAL_REFERENCES=1 cargo test visual_regression
```

This will:
1. Run all visual regression tests
2. Save captured images as new reference images
3. Skip comparison (all tests pass)

### Manual Reference Image Creation

You can also manually create reference images:

1. Run your rendering code
2. Capture the output
3. Save as `tests/visual_regression/reference/<test_name>.png`

## Test Output

When tests run, they generate:
- `output/<test_name>.png` - The captured test output
- `output/<test_name>_diff.png` - Difference visualization (only on failure)

The diff image highlights differences in red, making it easy to spot regressions.

## Best Practices

1. **Use descriptive test names** - Makes it easy to identify which test failed
2. **Keep images small** - Use minimal resolution needed for the test (e.g., 256x256)
3. **Test specific features** - Each test should focus on one rendering feature
4. **Review diffs carefully** - Not all differences are bugs; some may be acceptable
5. **Update references intentionally** - Only update when you've verified the new output is correct

## Example Test

```rust
#[test]
fn test_triangle_rendering() {
    pollster::block_on(async {
        let (device, queue) = create_test_device().await.unwrap();
        
        let result = run_visual_test(
            "triangle",
            &device,
            &queue,
            |device, queue| {
                // Render a triangle
                render_triangle(device, queue)
            },
            ComparisonConfig::default(),
        ).await.unwrap();
        
        assert_visual_match!(result);
    });
}
```

## Troubleshooting

### Test fails with "Reference image not found"

Run with `UPDATE_VISUAL_REFERENCES=1` to create initial reference images.

### Test fails with dimension mismatch

Ensure your render target size matches the reference image size. Either:
- Update your test to use the correct size
- Regenerate the reference image with the new size

### Test fails with small differences

If differences are acceptable (e.g., GPU driver variations):
- Increase the threshold in `ComparisonConfig`
- Or update the reference image if the new output is correct
