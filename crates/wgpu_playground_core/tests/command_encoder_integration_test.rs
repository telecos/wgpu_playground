use wgpu_playground_core::command_encoder::{copy_buffer, create_encoder, CommandEncoderOps};

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
fn test_encoder_creation() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Test creation with label
        let _encoder = CommandEncoderOps::new(&device, Some("Test Encoder"));

        // Test creation without label
        let _encoder = CommandEncoderOps::new(&device, None);
    });
}

#[test]
fn test_encoder_finish() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let encoder = CommandEncoderOps::new(&device, Some("Test Encoder"));

        // Finish the encoder to get a command buffer
        let command_buffer = encoder.finish();

        // Submit the command buffer
        let _submission_index = queue.submit(std::iter::once(command_buffer));

        // Poll to ensure completion
        device.poll(wgpu::Maintain::Wait);
    });
}

#[test]
fn test_copy_buffer_to_buffer() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create source and destination buffers
        let src_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Source Buffer"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let dst_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Destination Buffer"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        // Write test data to source buffer
        let test_data = vec![42u8; 256];
        queue.write_buffer(&src_buffer, 0, &test_data);

        // Create encoder and copy
        let mut encoder = CommandEncoderOps::new(&device, Some("Copy Encoder"));
        encoder.copy_buffer_to_buffer(&src_buffer, 0, &dst_buffer, 0, 256);
        let command_buffer = encoder.finish();

        // Submit and wait
        queue.submit(std::iter::once(command_buffer));
        device.poll(wgpu::Maintain::Wait);

        // Verify the copy by mapping the destination buffer
        let buffer_slice = dst_buffer.slice(..);
        let (tx, rx) = futures_channel::oneshot::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        device.poll(wgpu::Maintain::Wait);
        rx.await.unwrap().unwrap();

        let data = buffer_slice.get_mapped_range();
        assert_eq!(data[0], 42);
        assert_eq!(data[255], 42);
        drop(data);
        dst_buffer.unmap();
    });
}

#[test]
fn test_copy_buffer_with_offset() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create buffers
        let src_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Source Buffer"),
            size: 512,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let dst_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Destination Buffer"),
            size: 512,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        // Write test data at offset 64 in source buffer
        let test_data = vec![99u8; 128];
        queue.write_buffer(&src_buffer, 64, &test_data);

        // Copy with offsets
        let mut encoder = CommandEncoderOps::new(&device, Some("Offset Copy Encoder"));
        encoder.copy_buffer_to_buffer(&src_buffer, 64, &dst_buffer, 128, 128);
        let command_buffer = encoder.finish();

        queue.submit(std::iter::once(command_buffer));
        device.poll(wgpu::Maintain::Wait);

        // Verify the copy
        let buffer_slice = dst_buffer.slice(128..256);
        let (tx, rx) = futures_channel::oneshot::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        device.poll(wgpu::Maintain::Wait);
        rx.await.unwrap().unwrap();

        let data = buffer_slice.get_mapped_range();
        assert_eq!(data[0], 99);
        assert_eq!(data[127], 99);
        drop(data);
        dst_buffer.unmap();
    });
}

#[test]
fn test_copy_buffer_to_texture() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a buffer with texture data
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Texture Data Buffer"),
            size: 64, // 4x4 RGBA texture = 64 bytes
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create a texture
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Destination Texture"),
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

        // Write test data to buffer
        let test_data = vec![255u8; 64];
        queue.write_buffer(&buffer, 0, &test_data);

        // Copy buffer to texture
        let mut encoder = CommandEncoderOps::new(&device, Some("Buffer to Texture Encoder"));
        encoder.copy_buffer_to_texture(
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
        let command_buffer = encoder.finish();

        queue.submit(std::iter::once(command_buffer));
        device.poll(wgpu::Maintain::Wait);
    });
}

#[test]
fn test_copy_texture_to_buffer() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a texture
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Source Texture"),
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

        // Create a buffer for readback
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Readback Buffer"),
            size: 64, // 4x4 RGBA texture = 64 bytes
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        // Write test data to texture
        let test_data = vec![128u8; 64];
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &test_data,
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
        let mut encoder = CommandEncoderOps::new(&device, Some("Texture to Buffer Encoder"));
        encoder.copy_texture_to_buffer(
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
        let command_buffer = encoder.finish();

        queue.submit(std::iter::once(command_buffer));
        device.poll(wgpu::Maintain::Wait);

        // Verify the copy
        let buffer_slice = buffer.slice(..);
        let (tx, rx) = futures_channel::oneshot::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        device.poll(wgpu::Maintain::Wait);
        rx.await.unwrap().unwrap();

        let data = buffer_slice.get_mapped_range();
        assert_eq!(data[0], 128);
        assert_eq!(data[63], 128);
        drop(data);
        buffer.unmap();
    });
}

#[test]
fn test_copy_texture_to_texture() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create source texture
        let src_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Source Texture"),
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

        // Create destination texture
        let dst_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Destination Texture"),
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

        // Write test data to source texture
        let test_data = vec![200u8; 64];
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &src_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &test_data,
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

        // Copy texture to texture
        let mut encoder = CommandEncoderOps::new(&device, Some("Texture to Texture Encoder"));
        encoder.copy_texture_to_texture(
            wgpu::ImageCopyTexture {
                texture: &src_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyTexture {
                texture: &dst_texture,
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
        let command_buffer = encoder.finish();

        queue.submit(std::iter::once(command_buffer));
        device.poll(wgpu::Maintain::Wait);

        // Verify by reading back the destination texture
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Readback Buffer"),
            size: 64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let mut encoder = CommandEncoderOps::new(&device, Some("Verification Encoder"));
        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                texture: &dst_texture,
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
        let command_buffer = encoder.finish();

        queue.submit(std::iter::once(command_buffer));
        device.poll(wgpu::Maintain::Wait);

        let buffer_slice = buffer.slice(..);
        let (tx, rx) = futures_channel::oneshot::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        device.poll(wgpu::Maintain::Wait);
        rx.await.unwrap().unwrap();

        let data = buffer_slice.get_mapped_range();
        assert_eq!(data[0], 200);
        assert_eq!(data[63], 200);
        drop(data);
        buffer.unmap();
    });
}

#[test]
fn test_multiple_copy_commands() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create multiple buffers
        let buffer1 = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Buffer 1"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let buffer2 = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Buffer 2"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let buffer3 = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Buffer 3"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        // Write test data
        queue.write_buffer(&buffer1, 0, &vec![10u8; 256]);
        queue.write_buffer(&buffer2, 0, &vec![20u8; 256]);

        // Create encoder with multiple copy commands
        let mut encoder = CommandEncoderOps::new(&device, Some("Multiple Copy Encoder"));
        encoder.copy_buffer_to_buffer(&buffer1, 0, &buffer3, 0, 128);
        encoder.copy_buffer_to_buffer(&buffer2, 0, &buffer3, 128, 128);
        let command_buffer = encoder.finish();

        queue.submit(std::iter::once(command_buffer));
        device.poll(wgpu::Maintain::Wait);

        // Verify the copies
        let buffer_slice = buffer3.slice(..);
        let (tx, rx) = futures_channel::oneshot::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        device.poll(wgpu::Maintain::Wait);
        rx.await.unwrap().unwrap();

        let data = buffer_slice.get_mapped_range();
        // First 128 bytes should be 10
        assert_eq!(data[0], 10);
        assert_eq!(data[127], 10);
        // Last 128 bytes should be 20
        assert_eq!(data[128], 20);
        assert_eq!(data[255], 20);
        drop(data);
        buffer3.unmap();
    });
}

#[test]
fn test_create_encoder_helper() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Test the helper function
        let _encoder = create_encoder(&device, Some("Helper Test"));
        let _encoder = create_encoder(&device, None);
    });
}

#[test]
fn test_copy_buffer_helper() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create buffers
        let src_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Source"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let dst_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Destination"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        // Write test data
        queue.write_buffer(&src_buffer, 0, &vec![77u8; 256]);

        // Use the helper function
        let command_buffer = copy_buffer(&device, &src_buffer, 0, &dst_buffer, 0, 256);
        queue.submit(std::iter::once(command_buffer));
        device.poll(wgpu::Maintain::Wait);

        // Verify the copy
        let buffer_slice = dst_buffer.slice(..);
        let (tx, rx) = futures_channel::oneshot::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        device.poll(wgpu::Maintain::Wait);
        rx.await.unwrap().unwrap();

        let data = buffer_slice.get_mapped_range();
        assert_eq!(data[0], 77);
        assert_eq!(data[255], 77);
        drop(data);
        dst_buffer.unmap();
    });
}

#[test]
fn test_encoder_inner_access() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let mut encoder = CommandEncoderOps::new(&device, Some("Inner Test"));

        // Test mutable access to inner encoder
        let _inner_mut: &mut wgpu::CommandEncoder = encoder.inner_mut();

        // Test immutable access to inner encoder
        let _inner: &wgpu::CommandEncoder = encoder.inner();
    });
}

#[test]
fn test_submit_multiple_encoders() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create multiple encoders
        let encoder1 = CommandEncoderOps::new(&device, Some("Encoder 1"));
        let encoder2 = CommandEncoderOps::new(&device, Some("Encoder 2"));
        let encoder3 = CommandEncoderOps::new(&device, Some("Encoder 3"));

        // Finish all encoders
        let cmd1 = encoder1.finish();
        let cmd2 = encoder2.finish();
        let cmd3 = encoder3.finish();

        // Submit all command buffers at once
        queue.submit([cmd1, cmd2, cmd3]);
        device.poll(wgpu::Maintain::Wait);
    });
}
