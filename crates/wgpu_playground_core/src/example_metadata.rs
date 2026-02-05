//! Example metadata including API coverage information
//!
//! This module provides metadata about what WebGPU APIs each example demonstrates

use crate::api_coverage::ApiCategory;

/// Retrieves the list of API categories demonstrated by a given example
pub fn get_example_api_tags(example_id: &str) -> Vec<ApiCategory> {
    match example_id {
        "triangle" => vec![
            ApiCategory::Buffer,
            ApiCategory::Shader,
            ApiCategory::RenderPipeline,
            ApiCategory::RenderPass,
            ApiCategory::CommandEncoder,
            ApiCategory::Queue,
        ],
        "cube" => vec![
            ApiCategory::Buffer,
            ApiCategory::Shader,
            ApiCategory::RenderPipeline,
            ApiCategory::BindGroup,
            ApiCategory::RenderPass,
            ApiCategory::CommandEncoder,
            ApiCategory::Queue,
        ],
        "texture_mapping" => vec![
            ApiCategory::Buffer,
            ApiCategory::Texture,
            ApiCategory::Sampler,
            ApiCategory::Shader,
            ApiCategory::RenderPipeline,
            ApiCategory::BindGroup,
            ApiCategory::RenderPass,
            ApiCategory::CommandEncoder,
            ApiCategory::Queue,
        ],
        "compute_shader" => vec![
            ApiCategory::Buffer,
            ApiCategory::Shader,
            ApiCategory::ComputePipeline,
            ApiCategory::BindGroup,
            ApiCategory::ComputePass,
            ApiCategory::CommandEncoder,
            ApiCategory::Queue,
        ],
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangle_has_basic_rendering_apis() {
        let tags = get_example_api_tags("triangle");
        assert!(tags.contains(&ApiCategory::RenderPass));
        assert!(tags.contains(&ApiCategory::Shader));
    }

    #[test]
    fn compute_shader_has_compute_apis() {
        let tags = get_example_api_tags("compute_shader");
        assert!(tags.contains(&ApiCategory::ComputePipeline));
        assert!(tags.contains(&ApiCategory::ComputePass));
    }

    #[test]
    fn texture_example_has_texture_apis() {
        let tags = get_example_api_tags("texture_mapping");
        assert!(tags.contains(&ApiCategory::Texture));
        assert!(tags.contains(&ApiCategory::Sampler));
    }

    #[test]
    fn unknown_example_returns_empty() {
        let tags = get_example_api_tags("unknown_example");
        assert!(tags.is_empty());
    }
}
