use wgpu::TextureDimension;
use wgpu_playground_core::buffer::BufferUsages;
use wgpu_playground_core::resource_inspector::{
    BufferInfo, ComputePipelineInfo, RenderPipelineInfo, ResourceFilter, ResourceInfo,
    ResourceInspectorPanel, ResourceState, TextureInfo,
};

#[test]
fn test_resource_inspector_panel_creation() {
    let panel = ResourceInspectorPanel::new();
    assert_eq!(panel.resource_count(), 0);
}

#[test]
fn test_add_multiple_resource_types() {
    let mut panel = ResourceInspectorPanel::new();

    // Add a buffer
    panel.add_buffer(BufferInfo {
        label: Some("test_buffer".to_string()),
        size: 1024,
        usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        mapped_at_creation: false,
        state: ResourceState::Active,
    });

    // Add a texture
    panel.add_texture(TextureInfo {
        label: Some("test_texture".to_string()),
        width: 256,
        height: 256,
        depth_or_array_layers: 1,
        dimension: TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        mip_level_count: 1,
        sample_count: 1,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        state: ResourceState::Active,
    });

    // Add a render pipeline
    panel.add_render_pipeline(RenderPipelineInfo {
        label: Some("test_render_pipeline".to_string()),
        vertex_entry_point: "vs_main".to_string(),
        fragment_entry_point: Some("fs_main".to_string()),
        state: ResourceState::Active,
    });

    // Add a compute pipeline
    panel.add_compute_pipeline(ComputePipelineInfo {
        label: Some("test_compute_pipeline".to_string()),
        entry_point: "cs_main".to_string(),
        state: ResourceState::Active,
    });

    assert_eq!(panel.resource_count(), 4);
}

#[test]
fn test_filter_by_type() {
    let mut panel = ResourceInspectorPanel::new();

    panel.add_buffer(BufferInfo {
        label: Some("buffer1".to_string()),
        size: 1024,
        usage: BufferUsages::VERTEX,
        mapped_at_creation: false,
        state: ResourceState::Active,
    });

    panel.add_texture(TextureInfo {
        label: Some("texture1".to_string()),
        width: 256,
        height: 256,
        depth_or_array_layers: 1,
        dimension: TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        mip_level_count: 1,
        sample_count: 1,
        usage: wgpu::TextureUsages::TEXTURE_BINDING,
        state: ResourceState::Active,
    });

    panel.add_render_pipeline(RenderPipelineInfo {
        label: Some("pipeline1".to_string()),
        vertex_entry_point: "vs_main".to_string(),
        fragment_entry_point: Some("fs_main".to_string()),
        state: ResourceState::Active,
    });

    assert_eq!(panel.resource_count(), 3);

    // Test each filter
    let buffer_filter = ResourceFilter::Buffers;
    let texture_filter = ResourceFilter::Textures;
    let pipeline_filter = ResourceFilter::Pipelines;

    // Verify filter matches work correctly
    for resource in panel.resources() {
        match resource {
            ResourceInfo::Buffer(_) => {
                assert!(buffer_filter.matches(resource));
                assert!(!texture_filter.matches(resource));
                assert!(!pipeline_filter.matches(resource));
            }
            ResourceInfo::Texture(_) => {
                assert!(!buffer_filter.matches(resource));
                assert!(texture_filter.matches(resource));
                assert!(!pipeline_filter.matches(resource));
            }
            ResourceInfo::RenderPipeline(_) => {
                assert!(!buffer_filter.matches(resource));
                assert!(!texture_filter.matches(resource));
                assert!(pipeline_filter.matches(resource));
            }
            ResourceInfo::ComputePipeline(_) => {
                assert!(!buffer_filter.matches(resource));
                assert!(!texture_filter.matches(resource));
                assert!(pipeline_filter.matches(resource));
            }
        }
    }
}

#[test]
fn test_memory_usage_calculation() {
    let buffer_info = BufferInfo {
        label: Some("buffer".to_string()),
        size: 2048,
        usage: BufferUsages::VERTEX,
        mapped_at_creation: false,
        state: ResourceState::Active,
    };

    let texture_info = TextureInfo {
        label: Some("texture".to_string()),
        width: 256,
        height: 256,
        depth_or_array_layers: 1,
        dimension: TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        mip_level_count: 1,
        sample_count: 1,
        usage: wgpu::TextureUsages::TEXTURE_BINDING,
        state: ResourceState::Active,
    };

    let render_pipeline_info = RenderPipelineInfo {
        label: Some("pipeline".to_string()),
        vertex_entry_point: "vs_main".to_string(),
        fragment_entry_point: Some("fs_main".to_string()),
        state: ResourceState::Active,
    };

    let buffer_resource = ResourceInfo::Buffer(buffer_info);
    let texture_resource = ResourceInfo::Texture(texture_info);
    let pipeline_resource = ResourceInfo::RenderPipeline(render_pipeline_info);

    // Buffer memory should equal its size
    assert_eq!(buffer_resource.memory_usage(), 2048);

    // Texture memory: 256 * 256 * 1 * 4 bytes (RGBA8) * 1 sample = 262144 bytes
    assert_eq!(texture_resource.memory_usage(), 262144);

    // Pipeline has a small fixed overhead
    assert_eq!(pipeline_resource.memory_usage(), 1024);
}

#[test]
fn test_resource_state_transitions() {
    let states = [
        ResourceState::Active,
        ResourceState::InUse,
        ResourceState::Destroyed,
    ];

    for state in states {
        // Verify state string representations
        match state {
            ResourceState::Active => {
                assert_eq!(state.as_str(), "Active");
                assert_eq!(state.emoji(), "✓");
            }
            ResourceState::InUse => {
                assert_eq!(state.as_str(), "In Use");
                assert_eq!(state.emoji(), "🔄");
            }
            ResourceState::Destroyed => {
                assert_eq!(state.as_str(), "Destroyed");
                assert_eq!(state.emoji(), "❌");
            }
        }
    }
}

#[test]
fn test_demo_resources() {
    let mut panel = ResourceInspectorPanel::new();
    panel.add_demo_resources();

    // Demo resources should include at least:
    // - 3 buffers (vertex, index, uniform)
    // - 2 textures (color, depth)
    // - 2 pipelines (render, compute)
    assert!(panel.resource_count() >= 7);

    // Verify we have different resource types
    let mut has_buffer = false;
    let mut has_texture = false;
    let mut has_render_pipeline = false;
    let mut has_compute_pipeline = false;

    for resource in panel.resources() {
        match resource {
            ResourceInfo::Buffer(_) => has_buffer = true,
            ResourceInfo::Texture(_) => has_texture = true,
            ResourceInfo::RenderPipeline(_) => has_render_pipeline = true,
            ResourceInfo::ComputePipeline(_) => has_compute_pipeline = true,
        }
    }

    assert!(has_buffer);
    assert!(has_texture);
    assert!(has_render_pipeline);
    assert!(has_compute_pipeline);
}

#[test]
fn test_resource_info_accessors() {
    let buffer_info = BufferInfo {
        label: Some("test_buffer".to_string()),
        size: 1024,
        usage: BufferUsages::VERTEX,
        mapped_at_creation: false,
        state: ResourceState::Active,
    };

    let resource = ResourceInfo::Buffer(buffer_info);

    assert_eq!(resource.label(), Some("test_buffer"));
    assert_eq!(resource.state(), ResourceState::Active);
    assert_eq!(resource.type_name(), "Buffer");
}

#[test]
fn test_clear_resources() {
    let mut panel = ResourceInspectorPanel::new();
    panel.add_demo_resources();

    assert!(panel.resource_count() > 0);

    panel.clear();
    assert_eq!(panel.resource_count(), 0);
}

#[test]
fn test_texture_with_mip_levels() {
    let texture_info = TextureInfo {
        label: Some("mipped_texture".to_string()),
        width: 512,
        height: 512,
        depth_or_array_layers: 1,
        dimension: TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        mip_level_count: 4, // 512x512, 256x256, 128x128, 64x64
        sample_count: 1,
        usage: wgpu::TextureUsages::TEXTURE_BINDING,
        state: ResourceState::Active,
    };

    let resource = ResourceInfo::Texture(texture_info);

    // Calculate expected memory:
    // Level 0: 512 * 512 * 4 = 1048576
    // Level 1: 256 * 256 * 4 = 262144
    // Level 2: 128 * 128 * 4 = 65536
    // Level 3: 64 * 64 * 4 = 16384
    // Total: 1392640
    assert_eq!(resource.memory_usage(), 1392640);
}

#[test]
fn test_multisampled_texture_memory() {
    let texture_info = TextureInfo {
        label: Some("msaa_texture".to_string()),
        width: 256,
        height: 256,
        depth_or_array_layers: 1,
        dimension: TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        mip_level_count: 1,
        sample_count: 4, // 4x MSAA
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        state: ResourceState::Active,
    };

    let resource = ResourceInfo::Texture(texture_info);

    // 256 * 256 * 4 bytes * 4 samples = 1048576
    assert_eq!(resource.memory_usage(), 1048576);
}

#[test]
fn test_resource_label_optional() {
    let buffer_info = BufferInfo {
        label: None,
        size: 1024,
        usage: BufferUsages::VERTEX,
        mapped_at_creation: false,
        state: ResourceState::Active,
    };

    let resource = ResourceInfo::Buffer(buffer_info);
    assert_eq!(resource.label(), None);
}

#[test]
fn test_compute_pipeline_without_fragment() {
    let compute_info = ComputePipelineInfo {
        label: Some("compute_shader".to_string()),
        entry_point: "main".to_string(),
        state: ResourceState::Active,
    };

    let resource = ResourceInfo::ComputePipeline(compute_info);
    assert_eq!(resource.label(), Some("compute_shader"));
    assert_eq!(resource.type_name(), "Compute Pipeline");
}

#[test]
fn test_render_pipeline_without_fragment() {
    let render_info = RenderPipelineInfo {
        label: Some("vertex_only_pipeline".to_string()),
        vertex_entry_point: "vs_main".to_string(),
        fragment_entry_point: None,
        state: ResourceState::Active,
    };

    let resource = ResourceInfo::RenderPipeline(render_info);
    assert_eq!(resource.type_name(), "Render Pipeline");
}

#[test]
fn test_buffer_usage_flags() {
    let buffer_info = BufferInfo {
        label: Some("multi_usage_buffer".to_string()),
        size: 4096,
        usage: BufferUsages::VERTEX | BufferUsages::INDEX | BufferUsages::COPY_DST,
        mapped_at_creation: false,
        state: ResourceState::Active,
    };

    let resource = ResourceInfo::Buffer(buffer_info.clone());
    assert_eq!(resource.memory_usage(), 4096);

    // Verify usage flags are preserved
    assert!(buffer_info.usage.contains(BufferUsages::VERTEX));
    assert!(buffer_info.usage.contains(BufferUsages::INDEX));
    assert!(buffer_info.usage.contains(BufferUsages::COPY_DST));
}

#[test]
fn test_3d_texture_memory() {
    let texture_info = TextureInfo {
        label: Some("3d_texture".to_string()),
        width: 128,
        height: 128,
        depth_or_array_layers: 64, // 3D texture with 64 layers
        dimension: TextureDimension::D3,
        format: wgpu::TextureFormat::Rgba8Unorm,
        mip_level_count: 1,
        sample_count: 1,
        usage: wgpu::TextureUsages::TEXTURE_BINDING,
        state: ResourceState::Active,
    };

    let resource = ResourceInfo::Texture(texture_info);

    // 128 * 128 * 64 * 4 bytes = 4194304
    assert_eq!(resource.memory_usage(), 4194304);
}

#[test]
fn test_depth_texture_format() {
    let texture_info = TextureInfo {
        label: Some("depth_buffer".to_string()),
        width: 1920,
        height: 1080,
        depth_or_array_layers: 1,
        dimension: TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        mip_level_count: 1,
        sample_count: 1,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        state: ResourceState::InUse,
    };

    let resource = ResourceInfo::Texture(texture_info);

    // 1920 * 1080 * 4 bytes (32-bit depth) = 8294400
    assert_eq!(resource.memory_usage(), 8294400);
    assert_eq!(resource.state(), ResourceState::InUse);
}
