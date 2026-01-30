use wgpu_playground_core::buffer::{BufferDescriptor, BufferOps, BufferUsages};

// Helper function to create a test device and queue
async fn create_test_device() -> Option<(wgpu::Device, wgpu::Queue)> {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await?;

    adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: Some("Test Device"),
                memory_hints: Default::default(),
            },
            None,
        )
        .await
        .ok()
}

#[test]
fn test_vertex_buffer_creation() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let descriptor = BufferDescriptor::new(
            Some("vertex_buffer"),
            256,
            BufferUsages::VERTEX | BufferUsages::COPY_DST,
        );

        let buffer = descriptor.create_buffer(&device).unwrap();
        assert_eq!(buffer.size(), 256);
    });
}

#[test]
fn test_index_buffer_creation() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let descriptor = BufferDescriptor::new(
            Some("index_buffer"),
            128,
            BufferUsages::INDEX | BufferUsages::COPY_DST,
        );

        let buffer = descriptor.create_buffer(&device).unwrap();
        assert_eq!(buffer.size(), 128);
    });
}

#[test]
fn test_uniform_buffer_creation() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let descriptor = BufferDescriptor::new(
            Some("uniform_buffer"),
            256,
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        );

        let buffer = descriptor.create_buffer(&device).unwrap();
        assert_eq!(buffer.size(), 256);
    });
}

#[test]
fn test_storage_buffer_creation() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let descriptor = BufferDescriptor::new(
            Some("storage_buffer"),
            1024,
            BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
        );

        let buffer = descriptor.create_buffer(&device).unwrap();
        assert_eq!(buffer.size(), 1024);
    });
}

#[test]
fn test_indirect_buffer_creation() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let descriptor = BufferDescriptor::new(
            Some("indirect_buffer"),
            64,
            BufferUsages::INDIRECT | BufferUsages::COPY_DST,
        );

        let buffer = descriptor.create_buffer(&device).unwrap();
        assert_eq!(buffer.size(), 64);
    });
}

#[test]
fn test_query_resolve_buffer_creation() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let descriptor = BufferDescriptor::new(
            Some("query_resolve_buffer"),
            512,
            BufferUsages::QUERY_RESOLVE | BufferUsages::COPY_SRC,
        );

        let buffer = descriptor.create_buffer(&device).unwrap();
        assert_eq!(buffer.size(), 512);
    });
}

#[test]
fn test_copy_buffer_creation() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let descriptor = BufferDescriptor::new(
            Some("copy_buffer"),
            256,
            BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
        );

        let buffer = descriptor.create_buffer(&device).unwrap();
        assert_eq!(buffer.size(), 256);
    });
}

#[test]
fn test_buffer_with_all_usage_flags() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a buffer with multiple usage flags (not MAP_READ and MAP_WRITE together)
        let descriptor = BufferDescriptor::new(
            Some("multi_usage_buffer"),
            1024,
            BufferUsages::VERTEX
                | BufferUsages::INDEX
                | BufferUsages::UNIFORM
                | BufferUsages::STORAGE
                | BufferUsages::INDIRECT
                | BufferUsages::COPY_SRC
                | BufferUsages::COPY_DST
                | BufferUsages::QUERY_RESOLVE,
        );

        let buffer = descriptor.create_buffer(&device).unwrap();
        assert_eq!(buffer.size(), 1024);
    });
}

#[test]
fn test_map_read_buffer() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a buffer with MAP_READ and COPY_DST
        let descriptor = BufferDescriptor::new(
            Some("read_buffer"),
            256,
            BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        );

        let buffer = descriptor.create_buffer(&device).unwrap();

        // Write some data to the buffer first
        let data = vec![1u8; 256];
        queue.write_buffer(&buffer, 0, &data);

        // Wait for the write to complete
        device.poll(wgpu::Maintain::Wait);

        // Map the buffer for reading
        BufferOps::map_read(&buffer).await.unwrap();

        // Get the mapped range
        let view = BufferOps::get_mapped_range(&buffer);
        assert_eq!(view.len(), 256);

        // Drop the view before unmapping
        drop(view);

        // Unmap the buffer
        BufferOps::unmap(&buffer);
    });
}

#[test]
fn test_map_write_buffer() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a buffer with MAP_WRITE and COPY_SRC
        let descriptor = BufferDescriptor::new(
            Some("write_buffer"),
            256,
            BufferUsages::MAP_WRITE | BufferUsages::COPY_SRC,
        );

        let buffer = descriptor.create_buffer(&device).unwrap();

        // Map the buffer for writing
        BufferOps::map_write(&buffer).await.unwrap();

        // Get the mutable mapped range
        {
            let mut view = BufferOps::get_mapped_range_mut(&buffer);
            assert_eq!(view.len(), 256);

            // Write some data
            view.copy_from_slice(&vec![42u8; 256]);
        } // view is dropped here

        // Unmap the buffer
        BufferOps::unmap(&buffer);
    });
}

#[test]
fn test_buffer_mapped_at_creation() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let descriptor = BufferDescriptor::new(
            Some("mapped_buffer"),
            256,
            BufferUsages::VERTEX | BufferUsages::COPY_DST,
        )
        .with_mapped_at_creation(true);

        let buffer = descriptor.create_buffer(&device).unwrap();

        // The buffer is already mapped, we can get the mapped range immediately
        {
            let mut view = BufferOps::get_mapped_range_mut(&buffer);
            assert_eq!(view.len(), 256);

            // Write some data
            view.copy_from_slice(&vec![1u8; 256]);
        }

        // Unmap the buffer
        BufferOps::unmap(&buffer);
    });
}

#[test]
fn test_buffer_size_validation() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Try to create a buffer with size 0
        let descriptor = BufferDescriptor::new(
            Some("invalid_buffer"),
            0,
            BufferUsages::VERTEX,
        );

        let result = descriptor.create_buffer(&device);
        assert!(result.is_err());
    });
}

#[test]
fn test_buffer_usage_validation() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Try to create a buffer with empty usage
        let descriptor = BufferDescriptor::new(
            Some("invalid_buffer"),
            256,
            BufferUsages::empty(),
        );

        let result = descriptor.create_buffer(&device);
        assert!(result.is_err());
    });
}

#[test]
fn test_buffer_map_read_write_validation() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Try to create a buffer with both MAP_READ and MAP_WRITE
        let descriptor = BufferDescriptor::new(
            Some("invalid_buffer"),
            256,
            BufferUsages::MAP_READ | BufferUsages::MAP_WRITE,
        );

        let result = descriptor.create_buffer(&device);
        assert!(result.is_err());
    });
}

#[test]
fn test_multiple_buffers_with_different_usages() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create multiple buffers with different usage flags
        let vertex_buffer = BufferDescriptor::new(
            Some("vertex"),
            256,
            BufferUsages::VERTEX | BufferUsages::COPY_DST,
        )
        .create_buffer(&device)
        .unwrap();

        let index_buffer = BufferDescriptor::new(
            Some("index"),
            128,
            BufferUsages::INDEX | BufferUsages::COPY_DST,
        )
        .create_buffer(&device)
        .unwrap();

        let uniform_buffer = BufferDescriptor::new(
            Some("uniform"),
            256,
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        )
        .create_buffer(&device)
        .unwrap();

        let storage_buffer = BufferDescriptor::new(
            Some("storage"),
            1024,
            BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
        )
        .create_buffer(&device)
        .unwrap();

        // Verify all buffers were created successfully
        assert_eq!(vertex_buffer.size(), 256);
        assert_eq!(index_buffer.size(), 128);
        assert_eq!(uniform_buffer.size(), 256);
        assert_eq!(storage_buffer.size(), 1024);
    });
}

#[test]
fn test_buffer_copy_operations() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create source and destination buffers
        let src_buffer = BufferDescriptor::new(
            Some("src_buffer"),
            256,
            BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
        )
        .create_buffer(&device)
        .unwrap();

        let dst_buffer = BufferDescriptor::new(
            Some("dst_buffer"),
            256,
            BufferUsages::COPY_DST | BufferUsages::MAP_READ,
        )
        .create_buffer(&device)
        .unwrap();

        // Write data to source buffer
        let data = vec![42u8; 256];
        queue.write_buffer(&src_buffer, 0, &data);

        // Copy from source to destination
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("copy_encoder"),
        });

        encoder.copy_buffer_to_buffer(&src_buffer, 0, &dst_buffer, 0, 256);

        queue.submit(std::iter::once(encoder.finish()));

        // Wait for operations to complete
        device.poll(wgpu::Maintain::Wait);

        // Map and verify destination buffer
        BufferOps::map_read(&dst_buffer).await.unwrap();
        {
            let view = BufferOps::get_mapped_range(&dst_buffer);
            assert_eq!(view[0], 42);
            assert_eq!(view[255], 42);
        }
        BufferOps::unmap(&dst_buffer);
    });
}

#[test]
fn test_buffer_read_back() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a buffer for reading back data
        let descriptor = BufferDescriptor::new(
            Some("readback_buffer"),
            256,
            BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        );

        let buffer = descriptor.create_buffer(&device).unwrap();

        // Write test data
        let test_data: Vec<u8> = (0..256).map(|i| i as u8).collect();
        queue.write_buffer(&buffer, 0, &test_data);

        // Wait for write to complete
        device.poll(wgpu::Maintain::Wait);

        // Map and read the buffer
        BufferOps::map_read(&buffer).await.unwrap();
        {
            let view = BufferOps::get_mapped_range(&buffer);
            
            // Verify the data matches what we wrote
            for (i, &byte) in view.iter().enumerate() {
                assert_eq!(byte, i as u8);
            }
        }
        BufferOps::unmap(&buffer);
    });
}
