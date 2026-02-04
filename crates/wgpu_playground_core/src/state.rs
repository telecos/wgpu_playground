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
}
