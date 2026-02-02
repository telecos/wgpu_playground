//! Visual regression testing framework
//!
//! This module provides utilities for visual regression testing of GPU rendering:
//! - Capturing rendered output to images
//! - Comparing rendered output with reference images
//! - Generating difference images to highlight discrepancies
//!
//! # Example
//!
//! ```no_run
//! # use wgpu_playground_core::visual_regression::*;
//! # async fn example() {
//! # let device = todo!();
//! # let queue = todo!();
//! # let texture = todo!();
//! // Capture a rendered texture
//! let image = capture_texture(&device, &queue, &texture).await.unwrap();
//!
//! // Compare with reference image
//! let result = compare_with_reference(
//!     &image,
//!     "test_name",
//!     ComparisonConfig::default()
//! ).unwrap();
//!
//! assert!(result.is_match, "Visual regression detected!");
//! # }
//! ```

pub mod test_utils;

use image::{ImageBuffer, Rgba, RgbaImage};
use std::path::PathBuf;
use wgpu::{Device, Queue, Texture};

/// Configuration for image comparison
#[derive(Debug, Clone)]
pub struct ComparisonConfig {
    /// Maximum allowed pixel difference (0.0 - 1.0)
    /// where 0.0 means exact match and 1.0 means completely different
    pub threshold: f32,
    /// Whether to save diff images on failure
    pub save_diff: bool,
    /// Whether to update reference image if not found
    pub update_references: bool,
}

impl Default for ComparisonConfig {
    fn default() -> Self {
        // Check if we should update references from environment variable
        let update_references = std::env::var("UPDATE_VISUAL_REFERENCES")
            .map(|v| v == "1" || v.to_lowercase() == "true")
            .unwrap_or(false);

        Self {
            threshold: 0.01, // 1% difference allowed
            save_diff: true,
            update_references,
        }
    }
}

/// Result of image comparison
#[derive(Debug)]
pub struct ComparisonResult {
    /// Whether the images match within threshold
    pub is_match: bool,
    /// Difference metric (0.0 - 1.0)
    pub difference: f32,
    /// Path to diff image if generated
    pub diff_image_path: Option<PathBuf>,
}

/// Error types for visual regression testing
#[derive(Debug)]
pub enum VisualRegressionError {
    /// Failed to capture texture
    CaptureError(String),
    /// Failed to load reference image
    ReferenceLoadError(String),
    /// Failed to save image
    SaveError(String),
    /// Image dimensions don't match
    DimensionMismatch {
        expected: (u32, u32),
        actual: (u32, u32),
    },
}

impl std::fmt::Display for VisualRegressionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CaptureError(msg) => write!(f, "Capture error: {}", msg),
            Self::ReferenceLoadError(msg) => write!(f, "Reference load error: {}", msg),
            Self::SaveError(msg) => write!(f, "Save error: {}", msg),
            Self::DimensionMismatch { expected, actual } => {
                write!(
                    f,
                    "Dimension mismatch: expected {:?}, got {:?}",
                    expected, actual
                )
            }
        }
    }
}

impl std::error::Error for VisualRegressionError {}

/// Captures a GPU texture to an RGBA image
///
/// # Arguments
///
/// * `device` - The GPU device
/// * `queue` - The GPU queue
/// * `texture` - The texture to capture
///
/// # Returns
///
/// Returns an RGBA image buffer containing the texture data
pub async fn capture_texture(
    device: &Device,
    queue: &Queue,
    texture: &Texture,
) -> Result<RgbaImage, VisualRegressionError> {
    let size = texture.size();
    let width = size.width;
    let height = size.height;

    // Create a buffer to copy texture data to
    let bytes_per_pixel = 4; // RGBA
    let unpadded_bytes_per_row = width * bytes_per_pixel;
    let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    let padded_bytes_per_row = unpadded_bytes_per_row.div_ceil(align) * align;

    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Visual Regression Capture Buffer"),
        size: (padded_bytes_per_row * height) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    // Copy texture to buffer
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Visual Regression Copy Encoder"),
    });

    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(padded_bytes_per_row),
                rows_per_image: Some(height),
            },
        },
        size,
    );

    queue.submit(Some(encoder.finish()));

    // Map buffer and read data
    let buffer_slice = buffer.slice(..);
    let (sender, receiver) = futures_channel::oneshot::channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
        sender.send(result).ok();
    });

    let _ = device.poll(wgpu::PollType::Wait {
        submission_index: None,
        timeout: None,
    });
    receiver
        .await
        .map_err(|_| VisualRegressionError::CaptureError("Failed to receive map result".into()))?
        .map_err(|e| {
            VisualRegressionError::CaptureError(format!("Failed to map buffer: {:?}", e))
        })?;

    let data = buffer_slice.get_mapped_range();

    // Create image from buffer data (handling padding)
    let mut image_data = Vec::with_capacity((width * height * bytes_per_pixel) as usize);
    for row in 0..height {
        let row_start = (row * padded_bytes_per_row) as usize;
        let row_end = row_start + (width * bytes_per_pixel) as usize;
        image_data.extend_from_slice(&data[row_start..row_end]);
    }

    drop(data);
    buffer.unmap();

    ImageBuffer::from_raw(width, height, image_data)
        .ok_or_else(|| VisualRegressionError::CaptureError("Failed to create image buffer".into()))
}

/// Compares a captured image with a reference image
///
/// # Arguments
///
/// * `captured` - The captured image to test
/// * `test_name` - Name of the test (used to locate reference image)
/// * `config` - Comparison configuration
///
/// # Returns
///
/// Returns a comparison result indicating if images match
pub fn compare_with_reference(
    captured: &RgbaImage,
    test_name: &str,
    config: ComparisonConfig,
) -> Result<ComparisonResult, VisualRegressionError> {
    let reference_path = get_reference_path(test_name);
    let output_path = get_output_path(test_name);

    // Save the captured image for debugging
    captured
        .save(&output_path)
        .map_err(|e| VisualRegressionError::SaveError(format!("Failed to save output: {}", e)))?;

    // Load or create reference image
    let reference = if reference_path.exists() {
        image::open(&reference_path)
            .map_err(|e| {
                VisualRegressionError::ReferenceLoadError(format!(
                    "Failed to load reference: {}",
                    e
                ))
            })?
            .to_rgba8()
    } else if config.update_references {
        // Save captured image as new reference
        if let Some(parent) = reference_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                VisualRegressionError::SaveError(format!("Failed to create directory: {}", e))
            })?;
        }
        captured.save(&reference_path).map_err(|e| {
            VisualRegressionError::SaveError(format!("Failed to save reference: {}", e))
        })?;

        return Ok(ComparisonResult {
            is_match: true,
            difference: 0.0,
            diff_image_path: None,
        });
    } else {
        return Err(VisualRegressionError::ReferenceLoadError(format!(
            "Reference image not found: {:?}. Run with update_references=true to create it.",
            reference_path
        )));
    };

    // Check dimensions match
    if captured.dimensions() != reference.dimensions() {
        return Err(VisualRegressionError::DimensionMismatch {
            expected: reference.dimensions(),
            actual: captured.dimensions(),
        });
    }

    // Compare images pixel by pixel
    let (width, height) = captured.dimensions();
    let mut total_diff = 0.0f32;
    let mut diff_image = RgbaImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let captured_pixel = captured.get_pixel(x, y);
            let reference_pixel = reference.get_pixel(x, y);

            // Calculate per-channel difference
            let r_diff = (captured_pixel[0] as f32 - reference_pixel[0] as f32).abs() / 255.0;
            let g_diff = (captured_pixel[1] as f32 - reference_pixel[1] as f32).abs() / 255.0;
            let b_diff = (captured_pixel[2] as f32 - reference_pixel[2] as f32).abs() / 255.0;
            let a_diff = (captured_pixel[3] as f32 - reference_pixel[3] as f32).abs() / 255.0;

            let pixel_diff = (r_diff + g_diff + b_diff + a_diff) / 4.0;
            total_diff += pixel_diff;

            // Create diff visualization (red for differences)
            let diff_intensity = (pixel_diff * 255.0) as u8;
            diff_image.put_pixel(x, y, Rgba([diff_intensity, 0, 0, 255]));
        }
    }

    let difference = total_diff / (width * height) as f32;
    let is_match = difference <= config.threshold;

    // Save diff image if there's a mismatch
    let diff_image_path = if !is_match && config.save_diff {
        let diff_path = get_diff_path(test_name);
        if let Some(parent) = diff_path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        diff_image.save(&diff_path).ok();
        Some(diff_path)
    } else {
        None
    };

    Ok(ComparisonResult {
        is_match,
        difference,
        diff_image_path,
    })
}

/// Gets the path to a reference image
///
/// Note: Uses a relative path from CARGO_MANIFEST_DIR (the core crate directory)
/// to the workspace-level tests directory. This is intentional as visual regression
/// tests are workspace-level integration tests, not crate-specific unit tests.
/// The path is relative to ensure it works in different build configurations.
fn get_reference_path(test_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/visual_regression/reference")
        .join(format!("{}.png", test_name))
}

/// Gets the path to an output image
fn get_output_path(test_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/visual_regression/output")
        .join(format!("{}.png", test_name))
}

/// Gets the path to a diff image
fn get_diff_path(test_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/visual_regression/output")
        .join(format!("{}_diff.png", test_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comparison_config_default() {
        let config = ComparisonConfig::default();
        assert_eq!(config.threshold, 0.01);
        assert!(config.save_diff);
        // update_references depends on environment variable, so we can't assert a fixed value
        // Just verify it can be read
        let _ = config.update_references;
    }

    #[test]
    fn test_path_generation() {
        let ref_path = get_reference_path("test");
        assert!(ref_path.to_string_lossy().contains("reference"));
        assert!(ref_path.to_string_lossy().ends_with("test.png"));

        let out_path = get_output_path("test");
        assert!(out_path.to_string_lossy().contains("output"));

        let diff_path = get_diff_path("test");
        assert!(diff_path.to_string_lossy().contains("output"));
        assert!(diff_path.to_string_lossy().ends_with("test_diff.png"));
    }
}
