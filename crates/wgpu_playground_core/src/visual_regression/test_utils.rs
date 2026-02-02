//! Helper utilities for visual regression tests

use super::*;
use wgpu::{Device, Queue};

/// Helper to create a simple colored render target for testing
pub fn create_test_render_target(
    device: &Device,
    width: u32,
    height: u32,
) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Visual Test Render Target"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    })
}

/// Helper to run a visual regression test
///
/// # Arguments
///
/// * `test_name` - Name of the test
/// * `device` - GPU device
/// * `queue` - GPU queue
/// * `render_fn` - Function that renders to a texture
/// * `config` - Comparison configuration
///
/// # Returns
///
/// Returns the comparison result
pub async fn run_visual_test<F>(
    test_name: &str,
    device: &Device,
    queue: &Queue,
    render_fn: F,
    config: ComparisonConfig,
) -> Result<ComparisonResult, VisualRegressionError>
where
    F: FnOnce(&Device, &Queue) -> wgpu::Texture,
{
    // Render the scene
    let texture = render_fn(device, queue);

    // Capture the rendered output
    let image = capture_texture(device, queue, &texture).await?;

    // Compare with reference
    compare_with_reference(&image, test_name, config)
}

/// Helper macro to assert visual regression test passes
#[macro_export]
macro_rules! assert_visual_match {
    ($result:expr) => {
        assert!(
            $result.is_match,
            "Visual regression test failed! Difference: {:.4}%\nDiff image: {:?}",
            $result.difference * 100.0,
            $result.diff_image_path
        );
    };
    ($result:expr, $threshold:expr) => {
        assert!(
            $result.difference <= $threshold,
            "Visual regression test failed! Difference: {:.4}% exceeds threshold {:.4}%\nDiff image: {:?}",
            $result.difference * 100.0,
            $threshold * 100.0,
            $result.diff_image_path
        );
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_test_render_target() {
        // This test just ensures the function compiles
        // Actual GPU tests require a device
    }
}
