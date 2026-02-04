/// Playground state serialization module
///
/// This module provides serialization support for the playground state,
/// allowing users to save and load their work. It includes serializable
/// versions of panel configurations and conversion methods.
///
/// # Limitations
///
/// Some enum values (TextureFormat, TextureDimension, AddressMode, FilterMode, etc.)
/// are serialized as strings but not parsed back during import to avoid complexity.
/// These fields will retain their default values when loading state.
/// The string values are preserved in JSON for reference and future enhancement.
use base64::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Serializable version of BufferPanel state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferPanelState {
    pub label: String,
    pub size: String,
    pub usage_vertex: bool,
    pub usage_index: bool,
    pub usage_uniform: bool,
    pub usage_storage: bool,
    pub usage_indirect: bool,
    pub usage_copy_src: bool,
    pub usage_copy_dst: bool,
    pub usage_map_read: bool,
    pub usage_map_write: bool,
    pub usage_query_resolve: bool,
    pub mapped_at_creation: bool,
}

/// Serializable version of TexturePanel state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TexturePanelState {
    pub label: String,
    pub width: String,
    pub height: String,
    pub depth: String,
    pub mip_levels: String,
    pub sample_count: String,
    pub format: String,
    pub dimension: String,
    pub usage_copy_src: bool,
    pub usage_copy_dst: bool,
    pub usage_texture_binding: bool,
    pub usage_storage_binding: bool,
    pub usage_render_attachment: bool,
}

/// Serializable version of SamplerPanel state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamplerPanelState {
    pub label: String,
    pub address_mode_u: String,
    pub address_mode_v: String,
    pub address_mode_w: String,
    pub mag_filter: String,
    pub min_filter: String,
    pub mipmap_filter: String,
    pub lod_min_clamp: String,
    pub lod_max_clamp: String,
    pub compare: Option<String>,
    pub max_anisotropy: String,
}

/// Serializable version of ShaderEditor state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShaderEditorState {
    pub source_code: String,
    pub label: String,
    pub file_path: String,
}

/// Serializable version of RenderPipelinePanel state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderPipelinePanelState {
    pub label: String,
    pub vertex_entry_point: String,
    pub fragment_entry_point: String,
    pub topology: String,
    pub cull_mode: String,
    pub front_face: String,
    pub enable_depth_stencil: bool,
    pub depth_format: String,
    pub depth_write_enabled: bool,
    pub depth_compare: String,
    pub stencil_read_mask: String,
    pub stencil_write_mask: String,
    pub stencil_front_compare: String,
    pub stencil_front_fail_op: String,
    pub stencil_front_depth_fail_op: String,
    pub stencil_front_pass_op: String,
    pub stencil_back_compare: String,
    pub stencil_back_fail_op: String,
    pub stencil_back_depth_fail_op: String,
    pub stencil_back_pass_op: String,
    pub sample_count: u32,
    pub alpha_to_coverage_enabled: bool,
    pub target_format: String,
    pub blend_enabled: bool,
    pub color_blend_src: String,
    pub color_blend_dst: String,
    pub color_blend_op: String,
    pub alpha_blend_src: String,
    pub alpha_blend_dst: String,
    pub alpha_blend_op: String,
    pub write_red: bool,
    pub write_green: bool,
    pub write_blue: bool,
    pub write_alpha: bool,
}

/// Serializable version of ComputePipelinePanel state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputePipelinePanelState {
    pub label: String,
    pub entry_point: String,
}

/// Serializable version of BindGroupPanel state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindGroupPanelState {
    pub label: String,
    // Additional bind group configuration fields can be added here
}

/// Serializable version of BindGroupLayoutPanel state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindGroupLayoutPanelState {
    pub label: String,
    // Additional bind group layout configuration fields can be added here
}

/// Complete serializable playground state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaygroundState {
    /// Version of the state format for backward compatibility
    pub version: String,
    /// Buffer panel state
    pub buffer_panel: Option<BufferPanelState>,
    /// Texture panel state
    pub texture_panel: Option<TexturePanelState>,
    /// Sampler panel state
    pub sampler_panel: Option<SamplerPanelState>,
    /// Shader editor state
    pub shader_editor: Option<ShaderEditorState>,
    /// Render pipeline panel state
    pub render_pipeline_panel: Option<RenderPipelinePanelState>,
    /// Compute pipeline panel state
    pub compute_pipeline_panel: Option<ComputePipelinePanelState>,
    /// Bind group panel state
    pub bind_group_panel: Option<BindGroupPanelState>,
    /// Bind group layout panel state
    pub bind_group_layout_panel: Option<BindGroupLayoutPanelState>,
}

impl Default for PlaygroundState {
    fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            buffer_panel: None,
            texture_panel: None,
            sampler_panel: None,
            shader_editor: None,
            render_pipeline_panel: None,
            compute_pipeline_panel: None,
            bind_group_panel: None,
            bind_group_layout_panel: None,
        }
    }
}

impl PlaygroundState {
    /// Create a new playground state with the current version
    pub fn new() -> Self {
        Self::default()
    }

    /// Save the state to a JSON file
    pub fn save_to_file(&self, path: &Path) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(self).map_err(std::io::Error::other)?;
        std::fs::write(path, json)?;
        log::info!("Saved playground state to {:?}", path);
        Ok(())
    }

    /// Load state from a JSON file
    pub fn load_from_file(path: &Path) -> Result<Self, std::io::Error> {
        let json = std::fs::read_to_string(path)?;
        let state: Self = serde_json::from_str(&json).map_err(std::io::Error::other)?;
        log::info!("Loaded playground state from {:?}", path);
        Ok(state)
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Encode state to a URL-safe base64 string
    ///
    /// This compresses the JSON representation and encodes it in base64 (URL-safe variant)
    /// for sharing via URL parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// use wgpu_playground_core::state::PlaygroundState;
    ///
    /// let state = PlaygroundState::new();
    /// let encoded = state.to_url_encoded().unwrap();
    /// assert!(!encoded.is_empty());
    /// ```
    pub fn to_url_encoded(&self) -> Result<String, String> {
        // Serialize to JSON (compact format for smaller URLs)
        let json =
            serde_json::to_string(self).map_err(|e| format!("Failed to serialize state: {}", e))?;

        // Encode to base64 using URL-safe alphabet (no padding for shorter URLs)
        let encoded = BASE64_URL_SAFE_NO_PAD.encode(json.as_bytes());

        Ok(encoded)
    }

    /// Decode state from a URL-safe base64 string
    ///
    /// This decodes a base64-encoded string and deserializes the JSON state.
    ///
    /// # Examples
    ///
    /// ```
    /// use wgpu_playground_core::state::PlaygroundState;
    ///
    /// let state = PlaygroundState::new();
    /// let encoded = state.to_url_encoded().unwrap();
    /// let decoded = PlaygroundState::from_url_encoded(&encoded).unwrap();
    /// assert_eq!(decoded.version, state.version);
    /// ```
    pub fn from_url_encoded(encoded: &str) -> Result<Self, String> {
        // Decode from base64
        let json_bytes = BASE64_URL_SAFE_NO_PAD
            .decode(encoded.as_bytes())
            .map_err(|e| format!("Failed to decode base64: {}", e))?;

        // Convert bytes to string
        let json = String::from_utf8(json_bytes)
            .map_err(|e| format!("Failed to convert to UTF-8: {}", e))?;

        // Deserialize from JSON
        let state = serde_json::from_str(&json)
            .map_err(|e| format!("Failed to deserialize state: {}", e))?;

        Ok(state)
    }

    /// Generate a shareable URL with the current state encoded in the query parameter
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the application (e.g., "https://example.com/demo")
    ///
    /// # Examples
    ///
    /// ```
    /// use wgpu_playground_core::state::PlaygroundState;
    ///
    /// let state = PlaygroundState::new();
    /// let url = state.to_shareable_url("https://example.com").unwrap();
    /// assert!(url.starts_with("https://example.com?state="));
    /// ```
    pub fn to_shareable_url(&self, base_url: &str) -> Result<String, String> {
        let encoded_state = self.to_url_encoded()?;
        Ok(format!("{}?state={}", base_url, encoded_state))
    }

    /// Extract state from a URL query parameter
    ///
    /// # Arguments
    ///
    /// * `url` - The full URL or query string containing the state parameter
    ///
    /// # Examples
    ///
    /// ```
    /// use wgpu_playground_core::state::PlaygroundState;
    ///
    /// let state = PlaygroundState::new();
    /// let url = state.to_shareable_url("https://example.com").unwrap();
    /// let decoded = PlaygroundState::from_url(&url).unwrap();
    /// assert_eq!(decoded.version, state.version);
    /// ```
    pub fn from_url(url: &str) -> Result<Self, String> {
        // Extract query parameter from URL
        let query_start = url
            .find('?')
            .ok_or_else(|| "No query parameters found in URL".to_string())?;
        let query = &url[query_start + 1..];

        // Parse query parameters
        for param in query.split('&') {
            if let Some((key, value)) = param.split_once('=') {
                if key == "state" {
                    return Self::from_url_encoded(value);
                }
            }
        }

        Err("No 'state' parameter found in URL".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_serialization() {
        let state = PlaygroundState {
            version: "1.0".to_string(),
            buffer_panel: Some(BufferPanelState {
                label: "test_buffer".to_string(),
                size: "1024".to_string(),
                usage_vertex: true,
                usage_index: false,
                usage_uniform: false,
                usage_storage: false,
                usage_indirect: false,
                usage_copy_src: false,
                usage_copy_dst: true,
                usage_map_read: false,
                usage_map_write: false,
                usage_query_resolve: false,
                mapped_at_creation: false,
            }),
            texture_panel: None,
            sampler_panel: None,
            shader_editor: None,
            render_pipeline_panel: None,
            compute_pipeline_panel: None,
            bind_group_panel: None,
            bind_group_layout_panel: None,
        };

        // Test JSON serialization
        let json = state.to_json().unwrap();
        assert!(json.contains("test_buffer"));
        assert!(json.contains("1024"));

        // Test JSON deserialization
        let loaded_state = PlaygroundState::from_json(&json).unwrap();
        assert_eq!(loaded_state.version, "1.0");
        assert!(loaded_state.buffer_panel.is_some());
        let buffer_panel = loaded_state.buffer_panel.unwrap();
        assert_eq!(buffer_panel.label, "test_buffer");
        assert_eq!(buffer_panel.size, "1024");
        assert!(buffer_panel.usage_vertex);
        assert!(!buffer_panel.usage_index);
    }

    #[test]
    fn test_shader_editor_state_serialization() {
        let state = PlaygroundState {
            version: "1.0".to_string(),
            buffer_panel: None,
            texture_panel: None,
            sampler_panel: None,
            shader_editor: Some(ShaderEditorState {
                source_code: "@vertex fn main() {}".to_string(),
                label: "my_shader".to_string(),
                file_path: "shader.wgsl".to_string(),
            }),
            render_pipeline_panel: None,
            compute_pipeline_panel: None,
            bind_group_panel: None,
            bind_group_layout_panel: None,
        };

        let json = state.to_json().unwrap();
        let loaded_state = PlaygroundState::from_json(&json).unwrap();

        assert!(loaded_state.shader_editor.is_some());
        let shader = loaded_state.shader_editor.unwrap();
        assert_eq!(shader.source_code, "@vertex fn main() {}");
        assert_eq!(shader.label, "my_shader");
        assert_eq!(shader.file_path, "shader.wgsl");
    }

    #[test]
    fn test_empty_state_serialization() {
        let state = PlaygroundState::new();
        let json = state.to_json().unwrap();
        let loaded_state = PlaygroundState::from_json(&json).unwrap();

        assert_eq!(loaded_state.version, "1.0");
        assert!(loaded_state.buffer_panel.is_none());
        assert!(loaded_state.texture_panel.is_none());
        assert!(loaded_state.shader_editor.is_none());
    }

    #[test]
    fn test_url_encoding_empty_state() {
        let state = PlaygroundState::new();
        let encoded = state.to_url_encoded().unwrap();

        // Should produce a valid base64 string
        assert!(!encoded.is_empty());
        assert!(encoded
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_'));

        // Should be decodable
        let decoded = PlaygroundState::from_url_encoded(&encoded).unwrap();
        assert_eq!(decoded.version, state.version);
    }

    #[test]
    fn test_url_encoding_with_buffer_state() {
        let state = PlaygroundState {
            version: "1.0".to_string(),
            buffer_panel: Some(BufferPanelState {
                label: "vertex_buffer".to_string(),
                size: "2048".to_string(),
                usage_vertex: true,
                usage_index: false,
                usage_uniform: true,
                usage_storage: false,
                usage_indirect: false,
                usage_copy_src: true,
                usage_copy_dst: false,
                usage_map_read: false,
                usage_map_write: false,
                usage_query_resolve: false,
                mapped_at_creation: false,
            }),
            texture_panel: None,
            sampler_panel: None,
            shader_editor: None,
            render_pipeline_panel: None,
            compute_pipeline_panel: None,
            bind_group_panel: None,
            bind_group_layout_panel: None,
        };

        let encoded = state.to_url_encoded().unwrap();
        let decoded = PlaygroundState::from_url_encoded(&encoded).unwrap();

        assert_eq!(decoded.version, "1.0");
        assert!(decoded.buffer_panel.is_some());
        let buffer = decoded.buffer_panel.unwrap();
        assert_eq!(buffer.label, "vertex_buffer");
        assert_eq!(buffer.size, "2048");
        assert!(buffer.usage_vertex);
        assert!(buffer.usage_uniform);
        assert!(buffer.usage_copy_src);
    }

    #[test]
    fn test_url_encoding_with_shader_state() {
        let shader_code = r#"
@vertex
fn vs_main(@location(0) position: vec3<f32>) -> @builtin(position) vec4<f32> {
    return vec4<f32>(position, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
"#;

        let state = PlaygroundState {
            version: "1.0".to_string(),
            buffer_panel: None,
            texture_panel: None,
            sampler_panel: None,
            shader_editor: Some(ShaderEditorState {
                source_code: shader_code.to_string(),
                label: "red_shader".to_string(),
                file_path: "shader.wgsl".to_string(),
            }),
            render_pipeline_panel: None,
            compute_pipeline_panel: None,
            bind_group_panel: None,
            bind_group_layout_panel: None,
        };

        let encoded = state.to_url_encoded().unwrap();
        let decoded = PlaygroundState::from_url_encoded(&encoded).unwrap();

        assert!(decoded.shader_editor.is_some());
        let shader = decoded.shader_editor.unwrap();
        assert_eq!(shader.source_code, shader_code);
        assert_eq!(shader.label, "red_shader");
    }

    #[test]
    fn test_shareable_url_generation() {
        let state = PlaygroundState {
            version: "1.0".to_string(),
            buffer_panel: Some(BufferPanelState {
                label: "test".to_string(),
                size: "1024".to_string(),
                usage_vertex: true,
                usage_index: false,
                usage_uniform: false,
                usage_storage: false,
                usage_indirect: false,
                usage_copy_src: false,
                usage_copy_dst: false,
                usage_map_read: false,
                usage_map_write: false,
                usage_query_resolve: false,
                mapped_at_creation: false,
            }),
            texture_panel: None,
            sampler_panel: None,
            shader_editor: None,
            render_pipeline_panel: None,
            compute_pipeline_panel: None,
            bind_group_panel: None,
            bind_group_layout_panel: None,
        };

        let base_url = "https://example.com/playground";
        let url = state.to_shareable_url(base_url).unwrap();

        assert!(url.starts_with(base_url));
        assert!(url.contains("?state="));

        // Extract and verify state from URL
        let decoded = PlaygroundState::from_url(&url).unwrap();
        assert_eq!(decoded.version, "1.0");
        assert!(decoded.buffer_panel.is_some());
    }

    #[test]
    fn test_url_parsing_with_multiple_params() {
        let state = PlaygroundState::new();
        let encoded = state.to_url_encoded().unwrap();
        let url = format!("https://example.com?foo=bar&state={}&baz=qux", encoded);

        let decoded = PlaygroundState::from_url(&url).unwrap();
        assert_eq!(decoded.version, "1.0");
    }

    #[test]
    fn test_url_parsing_invalid_base64() {
        let url = "https://example.com?state=invalid!!!base64";
        let result = PlaygroundState::from_url(url);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to decode base64"));
    }

    #[test]
    fn test_url_parsing_no_state_param() {
        let url = "https://example.com?foo=bar&baz=qux";
        let result = PlaygroundState::from_url(url);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No 'state' parameter"));
    }

    #[test]
    fn test_url_parsing_no_query_params() {
        let url = "https://example.com";
        let result = PlaygroundState::from_url(url);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No query parameters"));
    }

    #[test]
    fn test_complex_state_roundtrip() {
        let state = PlaygroundState {
            version: "1.0".to_string(),
            buffer_panel: Some(BufferPanelState {
                label: "complex_buffer".to_string(),
                size: "4096".to_string(),
                usage_vertex: true,
                usage_index: true,
                usage_uniform: true,
                usage_storage: true,
                usage_indirect: false,
                usage_copy_src: true,
                usage_copy_dst: true,
                usage_map_read: true,
                usage_map_write: false,
                usage_query_resolve: false,
                mapped_at_creation: true,
            }),
            texture_panel: Some(TexturePanelState {
                label: "render_target".to_string(),
                width: "1024".to_string(),
                height: "768".to_string(),
                depth: "1".to_string(),
                mip_levels: "1".to_string(),
                sample_count: "1".to_string(),
                format: "Rgba8Unorm".to_string(),
                dimension: "D2".to_string(),
                usage_copy_src: false,
                usage_copy_dst: true,
                usage_texture_binding: true,
                usage_storage_binding: false,
                usage_render_attachment: true,
            }),
            sampler_panel: Some(SamplerPanelState {
                label: "linear_sampler".to_string(),
                address_mode_u: "Repeat".to_string(),
                address_mode_v: "Repeat".to_string(),
                address_mode_w: "ClampToEdge".to_string(),
                mag_filter: "Linear".to_string(),
                min_filter: "Linear".to_string(),
                mipmap_filter: "Linear".to_string(),
                lod_min_clamp: "0.0".to_string(),
                lod_max_clamp: "32.0".to_string(),
                compare: None,
                max_anisotropy: "1".to_string(),
            }),
            shader_editor: Some(ShaderEditorState {
                source_code: "@vertex fn main() {}".to_string(),
                label: "test_shader".to_string(),
                file_path: "test.wgsl".to_string(),
            }),
            render_pipeline_panel: None,
            compute_pipeline_panel: None,
            bind_group_panel: None,
            bind_group_layout_panel: None,
        };

        // Test full roundtrip
        let encoded = state.to_url_encoded().unwrap();
        let decoded = PlaygroundState::from_url_encoded(&encoded).unwrap();

        assert_eq!(decoded.version, state.version);
        assert!(decoded.buffer_panel.is_some());
        assert!(decoded.texture_panel.is_some());
        assert!(decoded.sampler_panel.is_some());
        assert!(decoded.shader_editor.is_some());

        let buffer = decoded.buffer_panel.unwrap();
        assert_eq!(buffer.label, "complex_buffer");
        assert_eq!(buffer.size, "4096");
        assert!(buffer.usage_vertex);
        assert!(buffer.mapped_at_creation);

        let texture = decoded.texture_panel.unwrap();
        assert_eq!(texture.label, "render_target");
        assert_eq!(texture.width, "1024");
        assert_eq!(texture.height, "768");
    }
}
