use wgpu_playground_core::bind_group::{
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, SamplerBindingType,
    StorageTextureAccess, TextureSampleType, TextureViewDimension,
};
use wgpu::ShaderStages;

#[test]
fn test_uniform_buffer_layout() {
    let entry = BindGroupLayoutEntry::new(
        0,
        ShaderStages::VERTEX,
        BindingType::UniformBuffer {
            has_dynamic_offset: false,
            min_binding_size: Some(64),
        },
    );

    let descriptor = BindGroupLayoutDescriptor::new(Some("uniform_buffer_layout"))
        .with_entry(entry);

    assert!(descriptor.validate().is_ok());
    assert_eq!(descriptor.entries().len(), 1);
    assert_eq!(descriptor.label(), Some("uniform_buffer_layout"));
}

#[test]
fn test_storage_buffer_layout() {
    let entry = BindGroupLayoutEntry::new(
        0,
        ShaderStages::COMPUTE,
        BindingType::StorageBuffer {
            has_dynamic_offset: false,
            min_binding_size: None,
            read_only: false,
        },
    );

    let descriptor = BindGroupLayoutDescriptor::new(Some("storage_buffer_layout"))
        .with_entry(entry);

    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_texture_and_sampler_layout() {
    let texture_entry = BindGroupLayoutEntry::new(
        0,
        ShaderStages::FRAGMENT,
        BindingType::Texture {
            sample_type: TextureSampleType::Float { filterable: true },
            view_dimension: TextureViewDimension::D2,
            multisampled: false,
        },
    );

    let sampler_entry = BindGroupLayoutEntry::new(
        1,
        ShaderStages::FRAGMENT,
        BindingType::Sampler {
            sampler_type: SamplerBindingType::Filtering,
        },
    );

    let descriptor = BindGroupLayoutDescriptor::new(Some("texture_sampler_layout"))
        .with_entry(texture_entry)
        .with_entry(sampler_entry);

    assert!(descriptor.validate().is_ok());
    assert_eq!(descriptor.entries().len(), 2);
}

#[test]
fn test_storage_texture_layout() {
    let entry = BindGroupLayoutEntry::new(
        0,
        ShaderStages::COMPUTE,
        BindingType::StorageTexture {
            access: StorageTextureAccess::WriteOnly,
            format: wgpu::TextureFormat::Rgba8Unorm,
            view_dimension: TextureViewDimension::D2,
        },
    );

    let descriptor = BindGroupLayoutDescriptor::new(Some("storage_texture_layout"))
        .with_entry(entry);

    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_complex_multi_binding_layout() {
    // Simulate a typical rendering pipeline with multiple bindings
    let entries = vec![
        // Binding 0: Transform uniform buffer (vertex stage)
        BindGroupLayoutEntry::new(
            0,
            ShaderStages::VERTEX,
            BindingType::UniformBuffer {
                has_dynamic_offset: false,
                min_binding_size: Some(64), // mat4x4<f32>
            },
        ),
        // Binding 1: Material properties uniform buffer (fragment stage)
        BindGroupLayoutEntry::new(
            1,
            ShaderStages::FRAGMENT,
            BindingType::UniformBuffer {
                has_dynamic_offset: false,
                min_binding_size: Some(16), // vec4<f32>
            },
        ),
        // Binding 2: Diffuse texture (fragment stage)
        BindGroupLayoutEntry::new(
            2,
            ShaderStages::FRAGMENT,
            BindingType::Texture {
                sample_type: TextureSampleType::Float { filterable: true },
                view_dimension: TextureViewDimension::D2,
                multisampled: false,
            },
        ),
        // Binding 3: Sampler (fragment stage)
        BindGroupLayoutEntry::new(
            3,
            ShaderStages::FRAGMENT,
            BindingType::Sampler {
                sampler_type: SamplerBindingType::Filtering,
            },
        ),
    ];

    let descriptor = BindGroupLayoutDescriptor::new(Some("rendering_layout"))
        .with_entries(&entries);

    assert!(descriptor.validate().is_ok());
    assert_eq!(descriptor.entries().len(), 4);

    // Verify each binding
    assert_eq!(descriptor.entries()[0].binding, 0);
    assert_eq!(descriptor.entries()[1].binding, 1);
    assert_eq!(descriptor.entries()[2].binding, 2);
    assert_eq!(descriptor.entries()[3].binding, 3);

    // Verify visibility
    assert_eq!(descriptor.entries()[0].visibility, ShaderStages::VERTEX);
    assert_eq!(descriptor.entries()[1].visibility, ShaderStages::FRAGMENT);
    assert_eq!(descriptor.entries()[2].visibility, ShaderStages::FRAGMENT);
    assert_eq!(descriptor.entries()[3].visibility, ShaderStages::FRAGMENT);
}

#[test]
fn test_compute_pipeline_layout() {
    // Typical compute pipeline layout with storage buffers
    let entries = vec![
        // Input storage buffer (read-only)
        BindGroupLayoutEntry::new(
            0,
            ShaderStages::COMPUTE,
            BindingType::StorageBuffer {
                has_dynamic_offset: false,
                min_binding_size: None,
                read_only: true,
            },
        ),
        // Output storage buffer (read-write)
        BindGroupLayoutEntry::new(
            1,
            ShaderStages::COMPUTE,
            BindingType::StorageBuffer {
                has_dynamic_offset: false,
                min_binding_size: None,
                read_only: false,
            },
        ),
        // Parameters uniform buffer
        BindGroupLayoutEntry::new(
            2,
            ShaderStages::COMPUTE,
            BindingType::UniformBuffer {
                has_dynamic_offset: false,
                min_binding_size: Some(16),
            },
        ),
    ];

    let descriptor = BindGroupLayoutDescriptor::new(Some("compute_layout"))
        .with_entries(&entries);

    assert!(descriptor.validate().is_ok());
    assert_eq!(descriptor.entries().len(), 3);
}

#[test]
fn test_multi_stage_visibility() {
    // Binding visible to multiple shader stages
    let entry = BindGroupLayoutEntry::new(
        0,
        ShaderStages::VERTEX | ShaderStages::FRAGMENT,
        BindingType::UniformBuffer {
            has_dynamic_offset: false,
            min_binding_size: Some(64),
        },
    );

    let descriptor = BindGroupLayoutDescriptor::new(Some("multi_stage_layout"))
        .with_entry(entry);

    assert!(descriptor.validate().is_ok());
    assert!(descriptor.entries()[0]
        .visibility
        .contains(ShaderStages::VERTEX));
    assert!(descriptor.entries()[0]
        .visibility
        .contains(ShaderStages::FRAGMENT));
}

#[test]
fn test_all_shader_stages_visibility() {
    // Binding visible to all shader stages
    let entry = BindGroupLayoutEntry::new(
        0,
        ShaderStages::VERTEX | ShaderStages::FRAGMENT | ShaderStages::COMPUTE,
        BindingType::UniformBuffer {
            has_dynamic_offset: false,
            min_binding_size: None,
        },
    );

    let descriptor = BindGroupLayoutDescriptor::new(Some("all_stages_layout"))
        .with_entry(entry);

    assert!(descriptor.validate().is_ok());
    assert!(descriptor.entries()[0]
        .visibility
        .contains(ShaderStages::VERTEX));
    assert!(descriptor.entries()[0]
        .visibility
        .contains(ShaderStages::FRAGMENT));
    assert!(descriptor.entries()[0]
        .visibility
        .contains(ShaderStages::COMPUTE));
}

#[test]
fn test_dynamic_offset_uniform_buffer() {
    let entry = BindGroupLayoutEntry::new(
        0,
        ShaderStages::VERTEX,
        BindingType::UniformBuffer {
            has_dynamic_offset: true,
            min_binding_size: Some(256),
        },
    );

    let descriptor = BindGroupLayoutDescriptor::new(Some("dynamic_uniform_layout"))
        .with_entry(entry);

    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_dynamic_offset_storage_buffer() {
    let entry = BindGroupLayoutEntry::new(
        0,
        ShaderStages::COMPUTE,
        BindingType::StorageBuffer {
            has_dynamic_offset: true,
            min_binding_size: None,
            read_only: false,
        },
    );

    let descriptor = BindGroupLayoutDescriptor::new(Some("dynamic_storage_layout"))
        .with_entry(entry);

    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_texture_array_binding() {
    let entry = BindGroupLayoutEntry::new(
        0,
        ShaderStages::FRAGMENT,
        BindingType::Texture {
            sample_type: TextureSampleType::Float { filterable: true },
            view_dimension: TextureViewDimension::D2,
            multisampled: false,
        },
    )
    .with_count(8); // Array of 8 textures

    let descriptor = BindGroupLayoutDescriptor::new(Some("texture_array_layout"))
        .with_entry(entry);

    assert!(descriptor.validate().is_ok());
    assert_eq!(descriptor.entries()[0].count, Some(8));
}

#[test]
fn test_cube_map_texture() {
    let entry = BindGroupLayoutEntry::new(
        0,
        ShaderStages::FRAGMENT,
        BindingType::Texture {
            sample_type: TextureSampleType::Float { filterable: true },
            view_dimension: TextureViewDimension::Cube,
            multisampled: false,
        },
    );

    let descriptor = BindGroupLayoutDescriptor::new(Some("cubemap_layout"))
        .with_entry(entry);

    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_depth_texture() {
    let entry = BindGroupLayoutEntry::new(
        0,
        ShaderStages::FRAGMENT,
        BindingType::Texture {
            sample_type: TextureSampleType::Depth,
            view_dimension: TextureViewDimension::D2,
            multisampled: false,
        },
    );

    let sampler_entry = BindGroupLayoutEntry::new(
        1,
        ShaderStages::FRAGMENT,
        BindingType::Sampler {
            sampler_type: SamplerBindingType::Comparison,
        },
    );

    let descriptor = BindGroupLayoutDescriptor::new(Some("depth_layout"))
        .with_entry(entry)
        .with_entry(sampler_entry);

    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_multisampled_texture() {
    let entry = BindGroupLayoutEntry::new(
        0,
        ShaderStages::FRAGMENT,
        BindingType::Texture {
            sample_type: TextureSampleType::Float { filterable: false },
            view_dimension: TextureViewDimension::D2,
            multisampled: true,
        },
    );

    let descriptor = BindGroupLayoutDescriptor::new(Some("msaa_layout"))
        .with_entry(entry);

    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_integer_texture() {
    let uint_entry = BindGroupLayoutEntry::new(
        0,
        ShaderStages::FRAGMENT,
        BindingType::Texture {
            sample_type: TextureSampleType::Uint,
            view_dimension: TextureViewDimension::D2,
            multisampled: false,
        },
    );

    let sint_entry = BindGroupLayoutEntry::new(
        1,
        ShaderStages::FRAGMENT,
        BindingType::Texture {
            sample_type: TextureSampleType::Sint,
            view_dimension: TextureViewDimension::D2,
            multisampled: false,
        },
    );

    let descriptor = BindGroupLayoutDescriptor::new(Some("integer_texture_layout"))
        .with_entry(uint_entry)
        .with_entry(sint_entry);

    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_3d_texture() {
    let entry = BindGroupLayoutEntry::new(
        0,
        ShaderStages::FRAGMENT,
        BindingType::Texture {
            sample_type: TextureSampleType::Float { filterable: true },
            view_dimension: TextureViewDimension::D3,
            multisampled: false,
        },
    );

    let descriptor = BindGroupLayoutDescriptor::new(Some("3d_texture_layout"))
        .with_entry(entry);

    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_non_filtering_sampler() {
    let entry = BindGroupLayoutEntry::new(
        0,
        ShaderStages::FRAGMENT,
        BindingType::Sampler {
            sampler_type: SamplerBindingType::NonFiltering,
        },
    );

    let descriptor = BindGroupLayoutDescriptor::new(Some("non_filtering_sampler_layout"))
        .with_entry(entry);

    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_read_only_storage_texture() {
    let entry = BindGroupLayoutEntry::new(
        0,
        ShaderStages::COMPUTE,
        BindingType::StorageTexture {
            access: StorageTextureAccess::ReadOnly,
            format: wgpu::TextureFormat::Rgba8Unorm,
            view_dimension: TextureViewDimension::D2,
        },
    );

    let descriptor = BindGroupLayoutDescriptor::new(Some("readonly_storage_texture_layout"))
        .with_entry(entry);

    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_read_write_storage_texture() {
    let entry = BindGroupLayoutEntry::new(
        0,
        ShaderStages::COMPUTE,
        BindingType::StorageTexture {
            access: StorageTextureAccess::ReadWrite,
            format: wgpu::TextureFormat::Rgba8Unorm,
            view_dimension: TextureViewDimension::D2,
        },
    );

    let descriptor = BindGroupLayoutDescriptor::new(Some("readwrite_storage_texture_layout"))
        .with_entry(entry);

    assert!(descriptor.validate().is_ok());
}
