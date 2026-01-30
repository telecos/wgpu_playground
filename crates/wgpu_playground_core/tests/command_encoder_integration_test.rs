use wgpu_playground_core::command_encoder::CommandEncoderOps;

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
fn test_copy_buffer_to_buffer_basic() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create source and destination buffers
        let src_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("source"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let dst_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("destination"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        // Write data to source buffer
        let data = vec![42u8; 256];
        queue.write_buffer(&src_buffer, 0, &data);

        // Create command encoder and perform copy
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("copy_encoder"),
        });

        let mut encoder_ops = CommandEncoderOps::new(&mut encoder);
        let result = encoder_ops.copy_buffer_to_buffer(&src_buffer, 0, &dst_buffer, 0, 256);

        assert!(result.is_ok());

        // Submit and wait
        queue.submit(std::iter::once(encoder.finish()));
        device.poll(wgpu::Maintain::Wait);

        // Verify the data was copied
        let (sender, receiver) = futures_channel::oneshot::channel();
        dst_buffer.slice(..).map_async(wgpu::MapMode::Read, move |result| {
            let _ = sender.send(result);
        });
        device.poll(wgpu::Maintain::Wait);
        receiver.await.unwrap().unwrap();

        {
            let view = dst_buffer.slice(..).get_mapped_range();
            assert_eq!(view[0], 42);
            assert_eq!(view[255], 42);
        }

        dst_buffer.unmap();
    });
}

#[test]
fn test_copy_buffer_to_buffer_with_offset() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create buffers
        let src_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("source"),
            size: 512,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let dst_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("destination"),
            size: 512,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        // Write test data
        let data = vec![123u8; 128];
        queue.write_buffer(&src_buffer, 0, &data);

        // Copy with offsets
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("copy_encoder"),
        });

        let mut encoder_ops = CommandEncoderOps::new(&mut encoder);
        let result = encoder_ops.copy_buffer_to_buffer(&src_buffer, 0, &dst_buffer, 128, 128);

        assert!(result.is_ok());

        queue.submit(std::iter::once(encoder.finish()));
        device.poll(wgpu::Maintain::Wait);

        // Verify
        let (sender, receiver) = futures_channel::oneshot::channel();
        dst_buffer.slice(128..256).map_async(wgpu::MapMode::Read, move |result| {
            let _ = sender.send(result);
        });
        device.poll(wgpu::Maintain::Wait);
        receiver.await.unwrap().unwrap();

        {
            let view = dst_buffer.slice(128..256).get_mapped_range();
            assert_eq!(view[0], 123);
        }

        dst_buffer.unmap();
    });
}

#[test]
fn test_copy_buffer_to_buffer_validation_zero_size() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("buffer"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("encoder"),
        });

        let mut encoder_ops = CommandEncoderOps::new(&mut encoder);
        let result = encoder_ops.copy_buffer_to_buffer(&buffer, 0, &buffer, 0, 0);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("greater than 0"));
    });
}

#[test]
fn test_copy_buffer_to_buffer_validation_unaligned_size() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("buffer"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("encoder"),
        });

        let mut encoder_ops = CommandEncoderOps::new(&mut encoder);
        // Size must be multiple of 4
        let result = encoder_ops.copy_buffer_to_buffer(&buffer, 0, &buffer, 0, 7);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("multiple of 4"));
    });
}

#[test]
fn test_copy_buffer_to_buffer_validation_unaligned_offset() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("buffer"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("encoder"),
        });

        let mut encoder_ops = CommandEncoderOps::new(&mut encoder);
        // Offset must be multiple of 4
        let result = encoder_ops.copy_buffer_to_buffer(&buffer, 3, &buffer, 0, 4);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("multiple of 4"));
    });
}

#[test]
fn test_copy_buffer_to_buffer_validation_overflow() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("buffer"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("encoder"),
        });

        let mut encoder_ops = CommandEncoderOps::new(&mut encoder);
        // Try to cause overflow with large offset and size
        let result = encoder_ops.copy_buffer_to_buffer(&buffer, u64::MAX - 100, &buffer, 0, 256);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("overflow"));
    });
}

#[test]
fn test_copy_buffer_to_buffer_validation_out_of_bounds() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let src_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("source"),
            size: 128,
            usage: wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let dst_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("destination"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("encoder"),
        });

        let mut encoder_ops = CommandEncoderOps::new(&mut encoder);
        // Try to copy more than source buffer size
        let result = encoder_ops.copy_buffer_to_buffer(&src_buffer, 0, &dst_buffer, 0, 256);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("exceeds buffer size"));
    });
}

#[test]
fn test_copy_buffer_to_texture_basic() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a buffer with image data
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("image_buffer"),
            size: 256, // 4x4 RGBA = 64 bytes, but we need more for alignment
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create a texture
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("test_texture"),
            size: wgpu::Extent3d {
                width: 4,
                height: 4,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        // Write data to buffer
        let data = vec![255u8; 64];
        queue.write_buffer(&buffer, 0, &data);

        // Copy buffer to texture
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("copy_encoder"),
        });

        let mut encoder_ops = CommandEncoderOps::new(&mut encoder);
        let result = encoder_ops.copy_buffer_to_texture(
            wgpu::ImageCopyBuffer {
                buffer: &buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(16), // 4 pixels * 4 bytes
                    rows_per_image: Some(4),
                },
            },
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::Extent3d {
                width: 4,
                height: 4,
                depth_or_array_layers: 1,
            },
        );

        assert!(result.is_ok());

        queue.submit(std::iter::once(encoder.finish()));
        device.poll(wgpu::Maintain::Wait);
    });
}

#[test]
fn test_copy_buffer_to_texture_validation_zero_size() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("buffer"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("texture"),
            size: wgpu::Extent3d {
                width: 4,
                height: 4,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("encoder"),
        });

        let mut encoder_ops = CommandEncoderOps::new(&mut encoder);
        let result = encoder_ops.copy_buffer_to_texture(
            wgpu::ImageCopyBuffer {
                buffer: &buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(16),
                    rows_per_image: Some(4),
                },
            },
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::Extent3d {
                width: 0, // Invalid
                height: 4,
                depth_or_array_layers: 1,
            },
        );

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("greater than 0"));
    });
}

#[test]
fn test_copy_buffer_to_texture_validation_missing_bytes_per_row() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("buffer"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("texture"),
            size: wgpu::Extent3d {
                width: 4,
                height: 4,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("encoder"),
        });

        let mut encoder_ops = CommandEncoderOps::new(&mut encoder);
        let result = encoder_ops.copy_buffer_to_texture(
            wgpu::ImageCopyBuffer {
                buffer: &buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: None, // Missing for height > 1
                    rows_per_image: Some(4),
                },
            },
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::Extent3d {
                width: 4,
                height: 4, // Height > 1
                depth_or_array_layers: 1,
            },
        );

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("bytes_per_row must be specified"));
    });
}

#[test]
fn test_copy_texture_to_buffer_basic() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a texture
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("test_texture"),
            size: wgpu::Extent3d {
                width: 4,
                height: 4,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        // Create a buffer to receive the texture data
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("readback_buffer"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        // Write data to texture first
        let data = vec![128u8; 64];
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(16),
                rows_per_image: Some(4),
            },
            wgpu::Extent3d {
                width: 4,
                height: 4,
                depth_or_array_layers: 1,
            },
        );

        // Copy texture to buffer
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("copy_encoder"),
        });

        let mut encoder_ops = CommandEncoderOps::new(&mut encoder);
        let result = encoder_ops.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyBuffer {
                buffer: &buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(16),
                    rows_per_image: Some(4),
                },
            },
            wgpu::Extent3d {
                width: 4,
                height: 4,
                depth_or_array_layers: 1,
            },
        );

        assert!(result.is_ok());

        queue.submit(std::iter::once(encoder.finish()));
        device.poll(wgpu::Maintain::Wait);

        // Verify data
        let (sender, receiver) = futures_channel::oneshot::channel();
        buffer.slice(..).map_async(wgpu::MapMode::Read, move |result| {
            let _ = sender.send(result);
        });
        device.poll(wgpu::Maintain::Wait);
        receiver.await.unwrap().unwrap();

        {
            let view = buffer.slice(..).get_mapped_range();
            assert_eq!(view[0], 128);
        }

        buffer.unmap();
    });
}

#[test]
fn test_copy_texture_to_buffer_validation_zero_size() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("texture"),
            size: wgpu::Extent3d {
                width: 4,
                height: 4,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("buffer"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("encoder"),
        });

        let mut encoder_ops = CommandEncoderOps::new(&mut encoder);
        let result = encoder_ops.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyBuffer {
                buffer: &buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(16),
                    rows_per_image: Some(4),
                },
            },
            wgpu::Extent3d {
                width: 4,
                height: 0, // Invalid
                depth_or_array_layers: 1,
            },
        );

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("greater than 0"));
    });
}

#[test]
fn test_copy_texture_to_buffer_validation_missing_rows_per_image() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("texture"),
            size: wgpu::Extent3d {
                width: 4,
                height: 4,
                depth_or_array_layers: 2,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("buffer"),
            size: 512,
            usage: wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("encoder"),
        });

        let mut encoder_ops = CommandEncoderOps::new(&mut encoder);
        let result = encoder_ops.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyBuffer {
                buffer: &buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(16),
                    rows_per_image: None, // Missing for depth/array layers > 1
                },
            },
            wgpu::Extent3d {
                width: 4,
                height: 4,
                depth_or_array_layers: 2, // > 1
            },
        );

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("rows_per_image must be specified"));
    });
}

#[test]
fn test_multiple_copy_operations() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create multiple buffers
        let buffer1 = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("buffer1"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let buffer2 = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("buffer2"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let buffer3 = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("buffer3"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        // Write initial data
        queue.write_buffer(&buffer1, 0, &vec![1u8; 128]);

        // Perform multiple copy operations
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("multi_copy_encoder"),
        });

        let mut encoder_ops = CommandEncoderOps::new(&mut encoder);
        
        // Copy from buffer1 to buffer2
        encoder_ops
            .copy_buffer_to_buffer(&buffer1, 0, &buffer2, 0, 128)
            .unwrap();
        
        // Copy from buffer2 to buffer3
        encoder_ops
            .copy_buffer_to_buffer(&buffer2, 0, &buffer3, 0, 128)
            .unwrap();

        queue.submit(std::iter::once(encoder.finish()));
        device.poll(wgpu::Maintain::Wait);

        // Verify final buffer has the data
        let (sender, receiver) = futures_channel::oneshot::channel();
        buffer3.slice(0..128).map_async(wgpu::MapMode::Read, move |result| {
            let _ = sender.send(result);
        });
        device.poll(wgpu::Maintain::Wait);
        receiver.await.unwrap().unwrap();

        {
            let view = buffer3.slice(0..128).get_mapped_range();
            assert_eq!(view[0], 1);
        }

        buffer3.unmap();
    });
}
