//! Mathematical utility functions for 3D vector and matrix operations
//!
//! This module provides common vector math operations used across the codebase,
//! particularly for rendering previews and transformations.

/// Normalizes a 3D vector to unit length
///
/// # Arguments
/// * `v` - The input vector to normalize
///
/// # Returns
/// A normalized vector with length 1.0, or the original vector if its length is near zero
///
/// # Examples
/// ```
/// use wgpu_playground_core::math_utils::normalize;
///
/// let v = [3.0, 4.0, 0.0];
/// let normalized = normalize(v);
/// // Result should be approximately [0.6, 0.8, 0.0]
/// ```
pub fn normalize(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len < f32::EPSILON {
        v
    } else {
        [v[0] / len, v[1] / len, v[2] / len]
    }
}

/// Computes the cross product of two 3D vectors
///
/// # Arguments
/// * `a` - First vector
/// * `b` - Second vector
///
/// # Returns
/// A vector perpendicular to both input vectors
///
/// # Examples
/// ```
/// use wgpu_playground_core::math_utils::cross;
///
/// let a = [1.0, 0.0, 0.0];
/// let b = [0.0, 1.0, 0.0];
/// let result = cross(a, b);
/// // Result should be [0.0, 0.0, 1.0]
/// ```
pub fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Computes the dot product of two 3D vectors
///
/// # Arguments
/// * `a` - First vector
/// * `b` - Second vector
///
/// # Returns
/// The scalar dot product
///
/// # Examples
/// ```
/// use wgpu_playground_core::math_utils::dot;
///
/// let a = [1.0, 2.0, 3.0];
/// let b = [4.0, 5.0, 6.0];
/// let result = dot(a, b);
/// // Result should be 1*4 + 2*5 + 3*6 = 32.0
/// ```
pub fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize() {
        let v = [3.0, 4.0, 0.0];
        let normalized = normalize(v);
        let len = (normalized[0] * normalized[0]
            + normalized[1] * normalized[1]
            + normalized[2] * normalized[2])
        .sqrt();
        assert!((len - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_normalize_zero_vector() {
        let v = [0.0, 0.0, 0.0];
        let normalized = normalize(v);
        assert_eq!(normalized, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_cross_product() {
        let a = [1.0, 0.0, 0.0];
        let b = [0.0, 1.0, 0.0];
        let result = cross(a, b);
        assert_eq!(result, [0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_cross_product_perpendicular() {
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];
        let result = cross(a, b);
        // Cross product should be perpendicular to both vectors
        assert!((dot(result, a)).abs() < 1e-6);
        assert!((dot(result, b)).abs() < 1e-6);
    }

    #[test]
    fn test_dot_product() {
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];
        let result = dot(a, b);
        assert_eq!(result, 32.0);
    }

    #[test]
    fn test_dot_product_orthogonal() {
        let a = [1.0, 0.0, 0.0];
        let b = [0.0, 1.0, 0.0];
        let result = dot(a, b);
        assert_eq!(result, 0.0);
    }
}
