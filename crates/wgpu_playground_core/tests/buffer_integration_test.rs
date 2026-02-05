mod common;

use common::create_test_device;
use wgpu::PollType;
use wgpu_playground_core::buffer::{BufferDescriptor, BufferOps, BufferUsages};

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
#[cfg_attr(
    all(target_os = "linux", target_env = "gnu"),
    ignore = "Hangs in CI with lavapipe software rendering"
)]
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

        // Poll device to ensure write is processed before mapping
        let _ = device.poll(PollType::Poll);

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
#[cfg_attr(
    all(target_os = "linux", target_env = "gnu"),
    ignore = "Hangs in CI with lavapipe software rendering"
)]
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

        // Poll device before mapping
        let _ = device.poll(PollType::Poll);

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
        let descriptor = BufferDescriptor::new(Some("invalid_buffer"), 0, BufferUsages::VERTEX);

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
        let descriptor = BufferDescriptor::new(Some("invalid_buffer"), 256, BufferUsages::empty());

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
#[cfg_attr(
    all(target_os = "linux", target_env = "gnu"),
    ignore = "Hangs in CI with lavapipe software rendering"
)]
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

        // Poll device to ensure operations complete before mapping
        let _ = device.poll(PollType::Poll);

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
#[cfg_attr(
    all(target_os = "linux", target_env = "gnu"),
    ignore = "Hangs in CI with lavapipe software rendering"
)]
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

        // Poll device before mapping
        let _ = device.poll(PollType::Poll);

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

#[test]
#[cfg_attr(
    all(target_os = "linux", target_env = "gnu"),
    ignore = "Hangs in CI with lavapipe software rendering"
)]
fn test_buffer_write_then_read_back() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a buffer we can write to and read from
        let buffer = BufferDescriptor::new(
            Some("rw_buffer"),
            1024,
            BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
        )
        .create_buffer(&device)
        .unwrap();

        // Create a staging buffer for reading back
        let staging = BufferDescriptor::new(
            Some("staging"),
            1024,
            BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        )
        .create_buffer(&device)
        .unwrap();

        // Write pattern to buffer
        let pattern: Vec<u32> = (0..256).map(|i| i * 2).collect();
        let bytes = bytemuck::cast_slice(&pattern);
        queue.write_buffer(&buffer, 0, bytes);

        // Copy to staging buffer
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("copy_encoder"),
        });
        encoder.copy_buffer_to_buffer(&buffer, 0, &staging, 0, 1024);
        queue.submit(std::iter::once(encoder.finish()));

        // Poll device before mapping
        let _ = device.poll(PollType::Poll);

        // Read back and verify
        BufferOps::map_read(&staging).await.unwrap();
        {
            let view = BufferOps::get_mapped_range(&staging);
            let read_data: &[u32] = bytemuck::cast_slice(&view);
            for (i, &value) in read_data.iter().enumerate() {
                assert_eq!(value, (i * 2) as u32);
            }
        }
        BufferOps::unmap(&staging);
    });
}

#[test]
#[cfg_attr(
    all(target_os = "linux", target_env = "gnu"),
    ignore = "Hangs in CI with lavapipe software rendering"
)]
fn test_buffer_partial_write() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let buffer = BufferDescriptor::new(
            Some("partial_write"),
            1024,
            BufferUsages::COPY_DST | BufferUsages::MAP_READ,
        )
        .create_buffer(&device)
        .unwrap();

        // Write to first half
        let data1 = vec![0xAA_u8; 512];
        queue.write_buffer(&buffer, 0, &data1);

        // Write to second half
        let data2 = vec![0xBB_u8; 512];
        queue.write_buffer(&buffer, 512, &data2);

        // Poll device before mapping
        let _ = device.poll(PollType::Poll);

        // Verify both halves
        BufferOps::map_read(&buffer).await.unwrap();
        {
            let view = BufferOps::get_mapped_range(&buffer);
            assert!(view[..512].iter().all(|&b| b == 0xAA));
            assert!(view[512..].iter().all(|&b| b == 0xBB));
        }
        BufferOps::unmap(&buffer);
    });
}

#[test]
#[cfg_attr(
    all(target_os = "linux", target_env = "gnu"),
    ignore = "Hangs in CI with lavapipe software rendering"
)]
fn test_buffer_zero_initialization() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create buffer with MAP_READ to verify contents
        let buffer = BufferDescriptor::new(
            Some("zero_init"),
            256,
            BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        )
        .create_buffer(&device)
        .unwrap();

        // Poll device before mapping
        let _ = device.poll(PollType::Poll);

        // Buffers should be zero-initialized
        BufferOps::map_read(&buffer).await.unwrap();
        {
            let view = BufferOps::get_mapped_range(&buffer);
            assert!(view.iter().all(|&b| b == 0));
        }
        BufferOps::unmap(&buffer);
    });
}

#[test]
#[cfg_attr(
    all(target_os = "linux", target_env = "gnu"),
    ignore = "Hangs in CI with lavapipe software rendering"
)]
fn test_buffer_overwrite_data() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let buffer = BufferDescriptor::new(
            Some("overwrite"),
            256,
            BufferUsages::COPY_DST | BufferUsages::MAP_READ,
        )
        .create_buffer(&device)
        .unwrap();

        // Write initial data
        queue.write_buffer(&buffer, 0, &vec![1u8; 256]);

        // Overwrite with different data
        queue.write_buffer(&buffer, 0, &vec![2u8; 256]);

        // Poll device before mapping
        let _ = device.poll(PollType::Poll);

        // Verify overwrite worked
        BufferOps::map_read(&buffer).await.unwrap();
        {
            let view = BufferOps::get_mapped_range(&buffer);
            assert!(view.iter().all(|&b| b == 2));
        }
        BufferOps::unmap(&buffer);
    });
}

#[test]
#[cfg_attr(
    all(target_os = "linux", target_env = "gnu"),
    ignore = "Hangs in CI with lavapipe software rendering"
)]
fn test_buffer_large_data_transfer() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let size = 1024 * 1024; // 1 MB
        let buffer = BufferDescriptor::new(
            Some("large_buffer"),
            size,
            BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
        )
        .create_buffer(&device)
        .unwrap();

        // Write large data
        let data = vec![0x42_u8; size as usize];
        queue.write_buffer(&buffer, 0, &data);

        // Copy to readback buffer to verify
        let readback = BufferDescriptor::new(
            Some("readback"),
            size,
            BufferUsages::COPY_DST | BufferUsages::MAP_READ,
        )
        .create_buffer(&device)
        .unwrap();

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("copy"),
        });
        encoder.copy_buffer_to_buffer(&buffer, 0, &readback, 0, size);
        queue.submit(std::iter::once(encoder.finish()));

        // Poll device before mapping
        let _ = device.poll(PollType::Poll);

        // Verify first and last bytes
        BufferOps::map_read(&readback).await.unwrap();
        {
            let view = BufferOps::get_mapped_range(&readback);
            assert_eq!(view[0], 0x42);
            assert_eq!(view[view.len() - 1], 0x42);
        }
        BufferOps::unmap(&readback);
    });
}

#[test]
#[cfg_attr(
    all(target_os = "linux", target_env = "gnu"),
    ignore = "Hangs in CI with lavapipe software rendering"
)]
fn test_buffer_multiple_map_unmap_cycles() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let buffer = BufferDescriptor::new(
            Some("cycle_buffer"),
            256,
            BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        )
        .create_buffer(&device)
        .unwrap();

        queue.write_buffer(&buffer, 0, &vec![1u8; 256]);

        // Poll device before mapping
        let _ = device.poll(PollType::Poll);

        // Map and unmap multiple times
        for _ in 0..3 {
            BufferOps::map_read(&buffer).await.unwrap();
            {
                let view = BufferOps::get_mapped_range(&buffer);
                assert_eq!(view[0], 1);
            }
            BufferOps::unmap(&buffer);
        }
    });
}

#[test]
#[cfg_attr(
    all(target_os = "linux", target_env = "gnu"),
    ignore = "Hangs in CI with lavapipe software rendering"
)]
fn test_buffer_map_write_modify_read() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a MAP_WRITE buffer
        let write_buffer = BufferDescriptor::new(
            Some("write_buf"),
            256,
            BufferUsages::MAP_WRITE | BufferUsages::COPY_SRC,
        )
        .create_buffer(&device)
        .unwrap();

        // Create a MAP_READ buffer for reading back
        let read_buffer = BufferDescriptor::new(
            Some("read_buf"),
            256,
            BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        )
        .create_buffer(&device)
        .unwrap();

        // Poll device before mapping
        let _ = device.poll(PollType::Poll);

        // Write data using MAP_WRITE
        BufferOps::map_write(&write_buffer).await.unwrap();
        {
            let mut view = BufferOps::get_mapped_range_mut(&write_buffer);
            for (i, byte) in view.iter_mut().enumerate() {
                *byte = (i % 256) as u8;
            }
        }
        BufferOps::unmap(&write_buffer);

        // Copy to read buffer
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("copy"),
        });
        encoder.copy_buffer_to_buffer(&write_buffer, 0, &read_buffer, 0, 256);
        queue.submit(std::iter::once(encoder.finish()));

        // Poll device before mapping
        let _ = device.poll(PollType::Poll);

        // Verify using MAP_READ
        BufferOps::map_read(&read_buffer).await.unwrap();
        {
            let view = BufferOps::get_mapped_range(&read_buffer);
            for (i, &byte) in view.iter().enumerate() {
                assert_eq!(byte, (i % 256) as u8);
            }
        }
        BufferOps::unmap(&read_buffer);
    });
}

#[test]
#[cfg_attr(
    all(target_os = "linux", target_env = "gnu"),
    ignore = "Hangs in CI with lavapipe software rendering"
)]
fn test_buffer_aligned_access() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create buffer for aligned u32 access
        let buffer = BufferDescriptor::new(
            Some("aligned"),
            256,
            BufferUsages::COPY_DST | BufferUsages::MAP_READ,
        )
        .create_buffer(&device)
        .unwrap();

        // Write u32 values
        let data: Vec<u32> = (0..64).collect();
        queue.write_buffer(&buffer, 0, bytemuck::cast_slice(&data));

        // Poll device before mapping
        let _ = device.poll(PollType::Poll);

        // Read back as u32
        BufferOps::map_read(&buffer).await.unwrap();
        {
            let view = BufferOps::get_mapped_range(&buffer);
            let u32_view: &[u32] = bytemuck::cast_slice(&view);
            for (i, &value) in u32_view.iter().enumerate() {
                assert_eq!(value, i as u32);
            }
        }
        BufferOps::unmap(&buffer);
    });
}

#[test]
#[cfg_attr(
    all(target_os = "linux", target_env = "gnu"),
    ignore = "Hangs in CI with lavapipe software rendering"
)]
fn test_buffer_empty_write() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let buffer = BufferDescriptor::new(
            Some("empty_write"),
            256,
            BufferUsages::COPY_DST | BufferUsages::MAP_READ,
        )
        .create_buffer(&device)
        .unwrap();

        // Write empty slice (should not crash)
        queue.write_buffer(&buffer, 0, &[]);

        // No need to poll when writing empty data - no work was submitted
        // But we still need to poll before mapping
        let _ = device.poll(PollType::Poll);

        // Verify buffer is still zero-initialized
        BufferOps::map_read(&buffer).await.unwrap();
        {
            let view = BufferOps::get_mapped_range(&buffer);
            assert!(view.iter().all(|&b| b == 0));
        }
        BufferOps::unmap(&buffer);
    });
}

#[test]
fn test_buffer_descriptor_validation_in_create() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // These should fail at descriptor validation, not device creation
        let invalid_descriptors = vec![
            BufferDescriptor::new(Some("zero_size"), 0, BufferUsages::VERTEX),
            BufferDescriptor::new(Some("empty_usage"), 256, BufferUsages::empty()),
            BufferDescriptor::new(
                Some("conflicting_map"),
                256,
                BufferUsages::MAP_READ | BufferUsages::MAP_WRITE,
            ),
        ];

        for descriptor in invalid_descriptors {
            let result = descriptor.create_buffer(&device);
            assert!(
                result.is_err(),
                "Expected error for descriptor: {:?}",
                descriptor
            );
        }
    });
}

#[test]
#[cfg_attr(
    all(target_os = "linux", target_env = "gnu"),
    ignore = "Hangs in CI with lavapipe software rendering"
)]
fn test_buffer_concurrent_access_different_buffers() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create multiple buffers and write to them
        let buffers: Vec<_> = (0..5)
            .map(|i| {
                BufferDescriptor::new(
                    Some(&format!("buffer_{}", i)),
                    256,
                    BufferUsages::COPY_DST | BufferUsages::MAP_READ,
                )
                .create_buffer(&device)
                .unwrap()
            })
            .collect();

        // Write different data to each buffer
        for (i, buffer) in buffers.iter().enumerate() {
            queue.write_buffer(buffer, 0, &vec![i as u8; 256]);
        }

        // Poll device before mapping
        let _ = device.poll(PollType::Poll);

        // Verify each buffer has correct data
        for (i, buffer) in buffers.iter().enumerate() {
            BufferOps::map_read(buffer).await.unwrap();
            {
                let view = BufferOps::get_mapped_range(buffer);
                assert!(view.iter().all(|&b| b == i as u8));
            }
            BufferOps::unmap(buffer);
        }
    });
}

#[test]
fn test_buffer_all_individual_usage_flags() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Test each usage flag individually (with COPY_DST for most)
        let test_cases = vec![
            ("vertex", BufferUsages::VERTEX | BufferUsages::COPY_DST),
            ("index", BufferUsages::INDEX | BufferUsages::COPY_DST),
            ("uniform", BufferUsages::UNIFORM | BufferUsages::COPY_DST),
            ("storage", BufferUsages::STORAGE | BufferUsages::COPY_DST),
            ("indirect", BufferUsages::INDIRECT | BufferUsages::COPY_DST),
            ("copy_src", BufferUsages::COPY_SRC | BufferUsages::COPY_DST),
            ("copy_dst", BufferUsages::COPY_DST),
            ("map_read", BufferUsages::MAP_READ | BufferUsages::COPY_DST),
            (
                "map_write",
                BufferUsages::MAP_WRITE | BufferUsages::COPY_SRC,
            ),
            (
                "query_resolve",
                BufferUsages::QUERY_RESOLVE | BufferUsages::COPY_SRC,
            ),
        ];

        for (name, usage) in test_cases {
            let descriptor = BufferDescriptor::new(Some(name), 256, usage);
            let buffer = descriptor.create_buffer(&device);
            assert!(
                buffer.is_ok(),
                "Failed to create buffer with usage: {}",
                name
            );
        }
    });
}

#[test]
fn test_buffer_name_preservation() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let descriptor = BufferDescriptor::new(
            Some("test_name"),
            256,
            BufferUsages::VERTEX | BufferUsages::COPY_DST,
        );

        // Verify label is preserved through descriptor
        assert_eq!(descriptor.label(), Some("test_name"));

        // Create buffer and verify it doesn't panic
        let _buffer = descriptor.create_buffer(&device).unwrap();
    });
}
