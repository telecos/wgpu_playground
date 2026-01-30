use std::num::{NonZeroU32, NonZeroU64};
use wgpu::ShaderStages;
use wgpu_playground_core::bind_group::{
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, SamplerBindingType,
    StorageTextureAccess, TextureSampleType, TextureViewDimension,
};

#[test]
fn test_uniform_buffer_layout() {
    let entry = BindGroupLayoutEntry::new(
        0,
        ShaderStages::VERTEX,
        BindingType::UniformBuffer {
            has_dynamic_offset: false,
            min_binding_size: NonZeroU64::new(64),
        },
    );

    let descriptor =
        BindGroupLayoutDescriptor::new(Some("uniform_buffer_layout")).with_entry(entry);

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

    let descriptor =
        BindGroupLayoutDescriptor::new(Some("storage_buffer_layout")).with_entry(entry);

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

    let descriptor =
        BindGroupLayoutDescriptor::new(Some("storage_texture_layout")).with_entry(entry);

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
                min_binding_size: NonZeroU64::new(64), // mat4x4<f32>
            },
        ),
        // Binding 1: Material properties uniform buffer (fragment stage)
        BindGroupLayoutEntry::new(
            1,
            ShaderStages::FRAGMENT,
            BindingType::UniformBuffer {
                has_dynamic_offset: false,
                min_binding_size: NonZeroU64::new(16), // vec4<f32>
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

    let descriptor =
        BindGroupLayoutDescriptor::new(Some("rendering_layout")).with_entries(&entries);

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
                min_binding_size: NonZeroU64::new(16),
            },
        ),
    ];

    let descriptor = BindGroupLayoutDescriptor::new(Some("compute_layout")).with_entries(&entries);

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
            min_binding_size: NonZeroU64::new(64),
        },
    );

    let descriptor = BindGroupLayoutDescriptor::new(Some("multi_stage_layout")).with_entry(entry);

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

    let descriptor = BindGroupLayoutDescriptor::new(Some("all_stages_layout")).with_entry(entry);

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
            min_binding_size: NonZeroU64::new(256),
        },
    );

    let descriptor =
        BindGroupLayoutDescriptor::new(Some("dynamic_uniform_layout")).with_entry(entry);

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

    let descriptor =
        BindGroupLayoutDescriptor::new(Some("dynamic_storage_layout")).with_entry(entry);

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
    .with_count(NonZeroU32::new(8).unwrap()); // Array of 8 textures

    let descriptor = BindGroupLayoutDescriptor::new(Some("texture_array_layout")).with_entry(entry);

    assert!(descriptor.validate().is_ok());
    assert_eq!(descriptor.entries()[0].count, NonZeroU32::new(8));
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

    let descriptor = BindGroupLayoutDescriptor::new(Some("cubemap_layout")).with_entry(entry);

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

    let descriptor = BindGroupLayoutDescriptor::new(Some("msaa_layout")).with_entry(entry);

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

    let descriptor = BindGroupLayoutDescriptor::new(Some("3d_texture_layout")).with_entry(entry);

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

    let descriptor =
        BindGroupLayoutDescriptor::new(Some("non_filtering_sampler_layout")).with_entry(entry);

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

    let descriptor =
        BindGroupLayoutDescriptor::new(Some("readonly_storage_texture_layout")).with_entry(entry);

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

    let descriptor =
        BindGroupLayoutDescriptor::new(Some("readwrite_storage_texture_layout")).with_entry(entry);

    assert!(descriptor.validate().is_ok());
}

// Integration tests for actual bind group creation with resources
mod bind_group_creation_tests {
    use super::*;
    use pollster::FutureExt;
    use std::num::NonZeroU64;
    use wgpu::ShaderStages;
    use wgpu_playground_core::bind_group::{
        BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
        BindingResource, BindingType, BufferBinding, SamplerBindingType, TextureSampleType,
        TextureViewDimension,
    };
    use wgpu_playground_core::buffer::{BufferDescriptor, BufferUsages};
    use wgpu_playground_core::sampler::SamplerDescriptor;
    use wgpu_playground_core::texture::TextureBuilder;

    // Helper to create a device for testing
    async fn create_test_device() -> Option<(wgpu::Device, wgpu::Queue)> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await?;

        adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Test Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: Default::default(),
                },
                None,
            )
            .await
            .ok()
    }

    #[test]
    fn test_uniform_buffer_bind_group() {
        let Some((device, _queue)) = create_test_device().block_on() else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a uniform buffer
        let buffer_desc = BufferDescriptor::new(
            Some("uniform_buffer"),
            256,
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        );
        let buffer = buffer_desc.create_buffer(&device).unwrap();

        // Create bind group layout
        let layout_desc = BindGroupLayoutDescriptor::new(Some("uniform_layout")).with_entry(
            BindGroupLayoutEntry::new(
                0,
                ShaderStages::VERTEX,
                BindingType::UniformBuffer {
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU64::new(256),
                },
            ),
        );
        let layout = layout_desc.create_layout(&device).unwrap();

        // Create bind group
        let bind_group_desc = BindGroupDescriptor::new(Some("uniform_bind_group"), &layout)
            .with_entry(BindGroupEntry::new(
                0,
                BindingResource::Buffer(BufferBinding::entire(&buffer)),
            ));

        let bind_group = bind_group_desc.create(&device).unwrap();
        assert!(std::ptr::addr_of!(bind_group) as usize != 0);
    }

    #[test]
    fn test_storage_buffer_bind_group() {
        let Some((device, _queue)) = create_test_device().block_on() else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a storage buffer
        let buffer_desc = BufferDescriptor::new(
            Some("storage_buffer"),
            1024,
            BufferUsages::STORAGE | BufferUsages::COPY_DST,
        );
        let buffer = buffer_desc.create_buffer(&device).unwrap();

        // Create bind group layout
        let layout_desc = BindGroupLayoutDescriptor::new(Some("storage_layout")).with_entry(
            BindGroupLayoutEntry::new(
                0,
                ShaderStages::COMPUTE,
                BindingType::StorageBuffer {
                    has_dynamic_offset: false,
                    min_binding_size: None,
                    read_only: false,
                },
            ),
        );
        let layout = layout_desc.create_layout(&device).unwrap();

        // Create bind group
        let bind_group_desc = BindGroupDescriptor::new(Some("storage_bind_group"), &layout)
            .with_entry(BindGroupEntry::new(
                0,
                BindingResource::Buffer(BufferBinding::entire(&buffer)),
            ));

        let bind_group = bind_group_desc.create(&device).unwrap();
        assert!(std::ptr::addr_of!(bind_group) as usize != 0);
    }

    #[test]
    fn test_texture_and_sampler_bind_group() {
        let Some((device, _queue)) = create_test_device().block_on() else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a texture
        let texture = TextureBuilder::new()
            .with_size(256, 256, 1)
            .with_format(wgpu::TextureFormat::Rgba8Unorm)
            .with_usage(wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST)
            .build(&device);

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create a sampler
        let sampler = SamplerDescriptor::new(None).create_sampler(&device).unwrap();

        // Create bind group layout
        let layout_desc = BindGroupLayoutDescriptor::new(Some("texture_sampler_layout"))
            .with_entry(BindGroupLayoutEntry::new(
                0,
                ShaderStages::FRAGMENT,
                BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
            ))
            .with_entry(BindGroupLayoutEntry::new(
                1,
                ShaderStages::FRAGMENT,
                BindingType::Sampler {
                    sampler_type: SamplerBindingType::Filtering,
                },
            ));
        let layout = layout_desc.create_layout(&device).unwrap();

        // Create bind group
        let bind_group_desc = BindGroupDescriptor::new(Some("texture_sampler_bind_group"), &layout)
            .with_entry(BindGroupEntry::new(
                0,
                BindingResource::TextureView(&texture_view),
            ))
            .with_entry(BindGroupEntry::new(1, BindingResource::Sampler(&sampler)));

        let bind_group = bind_group_desc.create(&device).unwrap();
        assert!(std::ptr::addr_of!(bind_group) as usize != 0);
    }

    #[test]
    fn test_buffer_binding_with_offset() {
        let Some((device, _queue)) = create_test_device().block_on() else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a large uniform buffer
        let buffer_desc = BufferDescriptor::new(
            Some("uniform_buffer"),
            1024,
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        );
        let buffer = buffer_desc.create_buffer(&device).unwrap();

        // Create bind group layout
        let layout_desc = BindGroupLayoutDescriptor::new(Some("uniform_layout")).with_entry(
            BindGroupLayoutEntry::new(
                0,
                ShaderStages::VERTEX,
                BindingType::UniformBuffer {
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU64::new(256),
                },
            ),
        );
        let layout = layout_desc.create_layout(&device).unwrap();

        // Create bind group with offset
        let bind_group_desc = BindGroupDescriptor::new(Some("uniform_bind_group"), &layout)
            .with_entry(BindGroupEntry::new(
                0,
                BindingResource::Buffer(BufferBinding::new(
                    &buffer,
                    256,
                    Some(NonZeroU64::new(256).unwrap()),
                )),
            ));

        let bind_group = bind_group_desc.create(&device).unwrap();
        assert!(std::ptr::addr_of!(bind_group) as usize != 0);
    }

    #[test]
    fn test_dynamic_offset_buffer() {
        let Some((device, _queue)) = create_test_device().block_on() else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a uniform buffer
        let buffer_desc = BufferDescriptor::new(
            Some("dynamic_uniform_buffer"),
            1024,
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        );
        let buffer = buffer_desc.create_buffer(&device).unwrap();

        // Create bind group layout with dynamic offset
        let layout_desc = BindGroupLayoutDescriptor::new(Some("dynamic_uniform_layout"))
            .with_entry(BindGroupLayoutEntry::new(
                0,
                ShaderStages::VERTEX,
                BindingType::UniformBuffer {
                    has_dynamic_offset: true,
                    min_binding_size: NonZeroU64::new(256),
                },
            ));
        let layout = layout_desc.create_layout(&device).unwrap();

        // Create bind group with dynamic offset buffer
        let bind_group_desc = BindGroupDescriptor::new(Some("dynamic_bind_group"), &layout)
            .with_entry(BindGroupEntry::new(
                0,
                BindingResource::Buffer(BufferBinding::new(
                    &buffer,
                    0,
                    Some(NonZeroU64::new(256).unwrap()),
                )),
            ));

        let bind_group = bind_group_desc.create(&device).unwrap();
        assert!(std::ptr::addr_of!(bind_group) as usize != 0);
    }

    #[test]
    fn test_multiple_buffer_bind_group() {
        let Some((device, _queue)) = create_test_device().block_on() else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create multiple buffers
        let uniform_buffer = BufferDescriptor::new(
            Some("uniform_buffer"),
            256,
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        )
        .create_buffer(&device)
        .unwrap();

        let storage_buffer = BufferDescriptor::new(
            Some("storage_buffer"),
            1024,
            BufferUsages::STORAGE | BufferUsages::COPY_DST,
        )
        .create_buffer(&device)
        .unwrap();

        // Create bind group layout
        let layout_desc = BindGroupLayoutDescriptor::new(Some("multi_buffer_layout"))
            .with_entry(BindGroupLayoutEntry::new(
                0,
                ShaderStages::VERTEX,
                BindingType::UniformBuffer {
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU64::new(256),
                },
            ))
            .with_entry(BindGroupLayoutEntry::new(
                1,
                ShaderStages::COMPUTE,
                BindingType::StorageBuffer {
                    has_dynamic_offset: false,
                    min_binding_size: None,
                    read_only: false,
                },
            ));
        let layout = layout_desc.create_layout(&device).unwrap();

        // Create bind group with multiple buffers
        let bind_group_desc = BindGroupDescriptor::new(Some("multi_buffer_bind_group"), &layout)
            .with_entry(BindGroupEntry::new(
                0,
                BindingResource::Buffer(BufferBinding::entire(&uniform_buffer)),
            ))
            .with_entry(BindGroupEntry::new(
                1,
                BindingResource::Buffer(BufferBinding::entire(&storage_buffer)),
            ));

        let bind_group = bind_group_desc.create(&device).unwrap();
        assert!(std::ptr::addr_of!(bind_group) as usize != 0);
    }

    #[test]
    fn test_complex_rendering_bind_group() {
        let Some((device, _queue)) = create_test_device().block_on() else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create resources
        let transform_buffer = BufferDescriptor::new(
            Some("transform_buffer"),
            64,
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        )
        .create_buffer(&device)
        .unwrap();

        let material_buffer = BufferDescriptor::new(
            Some("material_buffer"),
            16,
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        )
        .create_buffer(&device)
        .unwrap();

        let texture = TextureBuilder::new()
            .with_size(512, 512, 1)
            .with_format(wgpu::TextureFormat::Rgba8Unorm)
            .build(&device);
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = SamplerDescriptor::new(None).create_sampler(&device).unwrap();

        // Create bind group layout
        let layout_desc = BindGroupLayoutDescriptor::new(Some("rendering_layout"))
            .with_entry(BindGroupLayoutEntry::new(
                0,
                ShaderStages::VERTEX,
                BindingType::UniformBuffer {
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU64::new(64),
                },
            ))
            .with_entry(BindGroupLayoutEntry::new(
                1,
                ShaderStages::FRAGMENT,
                BindingType::UniformBuffer {
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU64::new(16),
                },
            ))
            .with_entry(BindGroupLayoutEntry::new(
                2,
                ShaderStages::FRAGMENT,
                BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
            ))
            .with_entry(BindGroupLayoutEntry::new(
                3,
                ShaderStages::FRAGMENT,
                BindingType::Sampler {
                    sampler_type: SamplerBindingType::Filtering,
                },
            ));
        let layout = layout_desc.create_layout(&device).unwrap();

        // Create bind group
        let bind_group_desc = BindGroupDescriptor::new(Some("rendering_bind_group"), &layout)
            .with_entry(BindGroupEntry::new(
                0,
                BindingResource::Buffer(BufferBinding::entire(&transform_buffer)),
            ))
            .with_entry(BindGroupEntry::new(
                1,
                BindingResource::Buffer(BufferBinding::entire(&material_buffer)),
            ))
            .with_entry(BindGroupEntry::new(
                2,
                BindingResource::TextureView(&texture_view),
            ))
            .with_entry(BindGroupEntry::new(3, BindingResource::Sampler(&sampler)));

        let bind_group = bind_group_desc.create(&device).unwrap();
        assert!(std::ptr::addr_of!(bind_group) as usize != 0);
    }

    #[test]
    fn test_bind_group_validation_empty() {
        let Some((device, _queue)) = create_test_device().block_on() else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let layout_desc = BindGroupLayoutDescriptor::new(Some("test_layout")).with_entry(
            BindGroupLayoutEntry::new(
                0,
                ShaderStages::VERTEX,
                BindingType::UniformBuffer {
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU64::new(256),
                },
            ),
        );
        let layout = layout_desc.create_layout(&device).unwrap();

        // Create empty bind group descriptor
        let bind_group_desc = BindGroupDescriptor::new(Some("empty_bind_group"), &layout);

        let result = bind_group_desc.validate();
        assert!(result.is_err());
        match result {
            Err(wgpu_playground_core::bind_group::BindGroupError::InvalidBinding(msg)) => {
                assert!(msg.contains("at least one entry"));
            }
            _ => panic!("Expected InvalidBinding error"),
        }
    }

    #[test]
    fn test_bind_group_validation_duplicate_binding() {
        let Some((device, _queue)) = create_test_device().block_on() else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let buffer = BufferDescriptor::new(
            Some("buffer"),
            256,
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        )
        .create_buffer(&device)
        .unwrap();

        let layout_desc = BindGroupLayoutDescriptor::new(Some("test_layout")).with_entry(
            BindGroupLayoutEntry::new(
                0,
                ShaderStages::VERTEX,
                BindingType::UniformBuffer {
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU64::new(256),
                },
            ),
        );
        let layout = layout_desc.create_layout(&device).unwrap();

        // Create bind group descriptor with duplicate bindings
        let bind_group_desc = BindGroupDescriptor::new(Some("duplicate_bind_group"), &layout)
            .with_entry(BindGroupEntry::new(
                0,
                BindingResource::Buffer(BufferBinding::entire(&buffer)),
            ))
            .with_entry(BindGroupEntry::new(
                0, // Duplicate binding number
                BindingResource::Buffer(BufferBinding::entire(&buffer)),
            ));

        let result = bind_group_desc.validate();
        assert!(result.is_err());
        match result {
            Err(wgpu_playground_core::bind_group::BindGroupError::DuplicateBinding(binding)) => {
                assert_eq!(binding, 0);
            }
            _ => panic!("Expected DuplicateBinding error"),
        }
    }
}
