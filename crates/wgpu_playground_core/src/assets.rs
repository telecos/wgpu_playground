use std::path::{Path, PathBuf};

/// Asset loading infrastructure for both native and web builds
///
/// This module provides utilities for loading static assets like shaders,
/// textures, and models. It handles path resolution differently for native
/// and web builds to ensure assets are loaded correctly in both environments.
/// Validates that a filename doesn't contain path traversal sequences
///
/// # Arguments
/// * `filename` - The filename to validate
///
/// # Returns
/// Ok(()) if the filename is safe, Err otherwise
fn validate_filename(filename: &str) -> Result<(), std::io::Error> {
    // Check for path separators and parent directory references
    if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Filename must not contain path separators or parent directory references",
        ));
    }

    // Ensure filename is not empty
    if filename.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Filename must not be empty",
        ));
    }

    Ok(())
}

/// Get the base assets directory path
///
/// For native builds, this returns the path to the assets directory relative to the workspace root.
/// For web builds, this returns a path suitable for web asset loading.
#[cfg(not(target_arch = "wasm32"))]
pub fn assets_dir() -> PathBuf {
    // For native builds, assets are relative to the workspace root
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("Failed to find workspace root")
        .join("assets")
}

#[cfg(target_arch = "wasm32")]
pub fn assets_dir() -> PathBuf {
    // For web builds, assets are served from the /assets path
    PathBuf::from("/assets")
}

/// Get the path to the shaders directory
pub fn shaders_dir() -> PathBuf {
    assets_dir().join("shaders")
}

/// Get the path to the textures directory
pub fn textures_dir() -> PathBuf {
    assets_dir().join("textures")
}

/// Get the path to the models directory
pub fn models_dir() -> PathBuf {
    assets_dir().join("models")
}

/// Load a shader file from the shaders directory
///
/// # Arguments
/// * `filename` - The name of the shader file (e.g., "example.wgsl")
///
/// # Returns
/// The shader source code as a String
///
/// # Errors
/// Returns an error if the file cannot be read or if the filename is invalid
pub fn load_shader(filename: &str) -> Result<String, std::io::Error> {
    validate_filename(filename)?;
    let path = shaders_dir().join(filename);
    load_string_from_path(&path)
}

/// Load a file as a string from a given path
///
/// # Arguments
/// * `path` - The path to the file
///
/// # Returns
/// The file contents as a String
///
/// # Errors
/// Returns an error if the file cannot be read
#[cfg(not(target_arch = "wasm32"))]
pub fn load_string_from_path(path: &Path) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

#[cfg(target_arch = "wasm32")]
pub fn load_string_from_path(_path: &Path) -> Result<String, std::io::Error> {
    // For web builds, we would use fetch API
    // This is a placeholder implementation
    // In a real implementation, this would need to be async and use web_sys
    Err(std::io::Error::other(
        "Synchronous file loading not supported on web - use async loading",
    ))
}

/// Load binary data from a file
///
/// # Arguments
/// * `path` - The path to the file
///
/// # Returns
/// The file contents as a `Vec<u8>`
///
/// # Errors
/// Returns an error if the file cannot be read
#[cfg(not(target_arch = "wasm32"))]
pub fn load_binary_from_path(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    std::fs::read(path)
}

#[cfg(target_arch = "wasm32")]
pub fn load_binary_from_path(_path: &Path) -> Result<Vec<u8>, std::io::Error> {
    // For web builds, we would use fetch API
    // This is a placeholder implementation
    Err(std::io::Error::other(
        "Synchronous file loading not supported on web - use async loading",
    ))
}

/// Load a texture file from the textures directory
///
/// # Arguments
/// * `filename` - The name of the texture file (e.g., "texture.png")
///
/// # Returns
/// The texture data as a `Vec<u8>`
///
/// # Errors
/// Returns an error if the file cannot be read or if the filename is invalid
pub fn load_texture(filename: &str) -> Result<Vec<u8>, std::io::Error> {
    validate_filename(filename)?;
    let path = textures_dir().join(filename);
    load_binary_from_path(&path)
}

/// Load a model file from the models directory
///
/// # Arguments
/// * `filename` - The name of the model file (e.g., "model.obj")
///
/// # Returns
/// The model data as a `Vec<u8>`
///
/// # Errors
/// Returns an error if the file cannot be read or if the filename is invalid
pub fn load_model(filename: &str) -> Result<Vec<u8>, std::io::Error> {
    validate_filename(filename)?;
    let path = models_dir().join(filename);
    load_binary_from_path(&path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assets_dir_exists() {
        let assets = assets_dir();
        assert!(assets.ends_with("assets"));
    }

    #[test]
    fn test_shaders_dir() {
        let shaders = shaders_dir();
        assert!(shaders.ends_with("shaders"));
    }

    #[test]
    fn test_textures_dir() {
        let textures = textures_dir();
        assert!(textures.ends_with("textures"));
    }

    #[test]
    fn test_models_dir() {
        let models = models_dir();
        assert!(models.ends_with("models"));
    }

    #[test]
    fn test_validate_filename_valid() {
        assert!(validate_filename("shader.wgsl").is_ok());
        assert!(validate_filename("texture.png").is_ok());
        assert!(validate_filename("model.obj").is_ok());
    }

    #[test]
    fn test_validate_filename_path_traversal() {
        assert!(validate_filename("../etc/passwd").is_err());
        assert!(validate_filename("..\\windows\\system32").is_err());
        assert!(validate_filename("subdir/../file.wgsl").is_err());
    }

    #[test]
    fn test_validate_filename_path_separators() {
        assert!(validate_filename("subdir/shader.wgsl").is_err());
        assert!(validate_filename("subdir\\shader.wgsl").is_err());
    }

    #[test]
    fn test_validate_filename_empty() {
        assert!(validate_filename("").is_err());
    }
}
