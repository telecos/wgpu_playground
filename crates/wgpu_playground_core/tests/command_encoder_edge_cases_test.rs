mod common;

use common::create_test_device;
use wgpu_playground_core::command_encoder::CommandEncoderOps;

/// Test that we can chain multiple buffer copies at buffer boundaries
#[test]
fn test_buffer_copy_at_boundaries() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create buffers at exact boundary sizes
        let src = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Source"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let dst = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Destination"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        queue.write_buffer(&src, 0, &vec![42u8; 256]);

        let mut encoder = CommandEncoderOps::new(&device, Some("Boundary Test"));
        // Copy exact buffer size
        encoder.copy_buffer_to_buffer(&src, 0, &dst, 0, 256);

        let command_buffer = encoder.finish();
        queue.submit(std::iter::once(command_buffer));
        device.poll(wgpu::Maintain::Wait);

        let buffer_slice = dst.slice(..);
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
        dst.unmap();
    });
}

/// Test copying from the end of a buffer
#[test]
fn test_buffer_copy_from_end() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let src = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Source"),
            size: 512,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let dst = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Destination"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        // Write different patterns
        queue.write_buffer(&src, 0, &vec![10u8; 256]);
        queue.write_buffer(&src, 256, &vec![20u8; 256]);

        let mut encoder = CommandEncoderOps::new(&device, Some("End Copy Test"));
        // Copy from the very end of the source buffer
        encoder.copy_buffer_to_buffer(&src, 256, &dst, 0, 256);

        let command_buffer = encoder.finish();
        queue.submit(std::iter::once(command_buffer));
        device.poll(wgpu::Maintain::Wait);

        let buffer_slice = dst.slice(..);
        let (tx, rx) = futures_channel::oneshot::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        device.poll(wgpu::Maintain::Wait);
        rx.await.unwrap().unwrap();

        let data = buffer_slice.get_mapped_range();
        // Should have copied the second half (value 20)
        assert_eq!(data[0], 20);
        assert_eq!(data[255], 20);
        drop(data);
        dst.unmap();
    });
}

/// Test copying to the end of a buffer
#[test]
fn test_buffer_copy_to_end() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let src = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Source"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let dst = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Destination"),
            size: 512,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        queue.write_buffer(&src, 0, &vec![30u8; 256]);
        queue.write_buffer(&dst, 0, &vec![0u8; 512]);

        let mut encoder = CommandEncoderOps::new(&device, Some("End Copy Test"));
        // Copy to the very end of the destination buffer
        encoder.copy_buffer_to_buffer(&src, 0, &dst, 256, 256);

        let command_buffer = encoder.finish();
        queue.submit(std::iter::once(command_buffer));
        device.poll(wgpu::Maintain::Wait);

        let buffer_slice = dst.slice(..);
        let (tx, rx) = futures_channel::oneshot::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        device.poll(wgpu::Maintain::Wait);
        rx.await.unwrap().unwrap();

        let data = buffer_slice.get_mapped_range();
        // First half should be 0
        assert_eq!(data[0], 0);
        assert_eq!(data[255], 0);
        // Second half should be 30
        assert_eq!(data[256], 30);
        assert_eq!(data[511], 30);
        drop(data);
        dst.unmap();
    });
}

/// Test small aligned copies (4 bytes)
#[test]
fn test_small_aligned_copies() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let src = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Source"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let dst = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Destination"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        // Write unique values at each 4-byte boundary
        let mut data = vec![0u8; 256];
        for i in 0..64 {
            let value = i as u8 + 1;
            data[i * 4] = value;
            data[i * 4 + 1] = value;
            data[i * 4 + 2] = value;
            data[i * 4 + 3] = value;
        }
        queue.write_buffer(&src, 0, &data);

        let mut encoder = CommandEncoderOps::new(&device, Some("Small Copy Test"));

        // Copy several 4-byte chunks
        encoder.copy_buffer_to_buffer(&src, 0, &dst, 0, 4);
        encoder.copy_buffer_to_buffer(&src, 4, &dst, 4, 4);
        encoder.copy_buffer_to_buffer(&src, 8, &dst, 8, 4);
        encoder.copy_buffer_to_buffer(&src, 12, &dst, 12, 4);

        let command_buffer = encoder.finish();
        queue.submit(std::iter::once(command_buffer));
        device.poll(wgpu::Maintain::Wait);

        let buffer_slice = dst.slice(0..16);
        let (tx, rx) = futures_channel::oneshot::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        device.poll(wgpu::Maintain::Wait);
        rx.await.unwrap().unwrap();

        let result = buffer_slice.get_mapped_range();
        assert_eq!(result[0], 1);
        assert_eq!(result[4], 2);
        assert_eq!(result[8], 3);
        assert_eq!(result[12], 4);
        drop(result);
        dst.unmap();
    });
}

/// Test large buffer copies (multiple MB)
#[test]
fn test_large_buffer_copy() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // 1 MB buffer
        let size = 1024 * 1024;

        let src = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Large Source"),
            size,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let dst = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Large Destination"),
            size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        // Write some data
        let data = vec![99u8; size as usize];
        queue.write_buffer(&src, 0, &data);

        let mut encoder = CommandEncoderOps::new(&device, Some("Large Copy Test"));
        encoder.copy_buffer_to_buffer(&src, 0, &dst, 0, size);

        let command_buffer = encoder.finish();
        queue.submit(std::iter::once(command_buffer));
        device.poll(wgpu::Maintain::Wait);

        // Verify a sample of the data
        let buffer_slice = dst.slice(0..1024);
        let (tx, rx) = futures_channel::oneshot::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        device.poll(wgpu::Maintain::Wait);
        rx.await.unwrap().unwrap();

        let result = buffer_slice.get_mapped_range();
        assert_eq!(result[0], 99);
        assert_eq!(result[512], 99);
        assert_eq!(result[1023], 99);
        drop(result);
        dst.unmap();
    });
}

/// Test overlapping encoder submissions (command buffers submitted separately)
#[test]
fn test_overlapping_encoder_submissions() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let buffer1 = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Buffer 1"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let buffer2 = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Buffer 2"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        queue.write_buffer(&buffer1, 0, &vec![55u8; 256]);

        // Create two separate encoders working on the same buffers
        let mut encoder1 = CommandEncoderOps::new(&device, Some("Encoder 1"));
        encoder1.copy_buffer_to_buffer(&buffer1, 0, &buffer2, 0, 128);
        let cmd1 = encoder1.finish();

        let mut encoder2 = CommandEncoderOps::new(&device, Some("Encoder 2"));
        encoder2.copy_buffer_to_buffer(&buffer1, 128, &buffer2, 128, 128);
        let cmd2 = encoder2.finish();

        // Submit both command buffers together
        queue.submit([cmd1, cmd2]);
        device.poll(wgpu::Maintain::Wait);

        let buffer_slice = buffer2.slice(..);
        let (tx, rx) = futures_channel::oneshot::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        device.poll(wgpu::Maintain::Wait);
        rx.await.unwrap().unwrap();

        let data = buffer_slice.get_mapped_range();
        assert_eq!(data[0], 55);
        assert_eq!(data[127], 55);
        assert_eq!(data[128], 55);
        assert_eq!(data[255], 55);
        drop(data);
        buffer2.unmap();
    });
}

/// Test texture copy with different mip levels
#[test]
fn test_texture_copy_with_mip_levels() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create texture with multiple mip levels
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Mipped Texture"),
            size: wgpu::Extent3d {
                width: 256,
                height: 256,
                depth_or_array_layers: 1,
            },
            mip_level_count: 4, // 256x256, 128x128, 64x64, 32x32
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        // Create buffers for different mip levels
        let buffer_mip0 = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Mip 0 Buffer"),
            size: 256 * 256 * 4,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let buffer_mip1 = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Mip 1 Buffer"),
            size: 128 * 128 * 4,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Write data
        queue.write_buffer(&buffer_mip0, 0, &vec![100u8; 256 * 256 * 4]);
        queue.write_buffer(&buffer_mip1, 0, &vec![50u8; 128 * 128 * 4]);

        let mut encoder = CommandEncoderOps::new(&device, Some("Mip Copy Test"));

        // Copy to mip level 0 (256x256)
        encoder.copy_buffer_to_texture(
            wgpu::ImageCopyBuffer {
                buffer: &buffer_mip0,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(256 * 4),
                    rows_per_image: Some(256),
                },
            },
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::Extent3d {
                width: 256,
                height: 256,
                depth_or_array_layers: 1,
            },
        );

        // Copy to mip level 1 (128x128)
        encoder.copy_buffer_to_texture(
            wgpu::ImageCopyBuffer {
                buffer: &buffer_mip1,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(256), // 128 * 4, but must be aligned to 256
                    rows_per_image: Some(128),
                },
            },
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 1,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::Extent3d {
                width: 128,
                height: 128,
                depth_or_array_layers: 1,
            },
        );

        let command_buffer = encoder.finish();
        queue.submit(std::iter::once(command_buffer));
        device.poll(wgpu::Maintain::Wait);
    });
}

/// Test texture-to-texture copy with different formats (same family)
#[test]
fn test_texture_to_texture_same_format() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let src_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Source Texture"),
            size: wgpu::Extent3d {
                width: 64,
                height: 64,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        let dst_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Destination Texture"),
            size: wgpu::Extent3d {
                width: 64,
                height: 64,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        // Write to source texture
        let data = vec![75u8; 64 * 64 * 4];
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &src_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(256), // 64 * 4
                rows_per_image: Some(64),
            },
            wgpu::Extent3d {
                width: 64,
                height: 64,
                depth_or_array_layers: 1,
            },
        );

        let mut encoder = CommandEncoderOps::new(&device, Some("Texture Copy Test"));
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
                width: 64,
                height: 64,
                depth_or_array_layers: 1,
            },
        );

        let command_buffer = encoder.finish();
        queue.submit(std::iter::once(command_buffer));
        device.poll(wgpu::Maintain::Wait);

        // Verify by reading back
        let readback = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Readback"),
            size: 64 * 64 * 4,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let mut encoder2 = CommandEncoderOps::new(&device, Some("Readback"));
        encoder2.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                texture: &dst_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyBuffer {
                buffer: &readback,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(256),
                    rows_per_image: Some(64),
                },
            },
            wgpu::Extent3d {
                width: 64,
                height: 64,
                depth_or_array_layers: 1,
            },
        );

        queue.submit(std::iter::once(encoder2.finish()));
        device.poll(wgpu::Maintain::Wait);

        let buffer_slice = readback.slice(0..64);
        let (tx, rx) = futures_channel::oneshot::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        device.poll(wgpu::Maintain::Wait);
        rx.await.unwrap().unwrap();

        let result = buffer_slice.get_mapped_range();
        assert_eq!(result[0], 75);
        assert_eq!(result[32], 75);
        assert_eq!(result[63], 75);
        drop(result);
        readback.unmap();
    });
}
