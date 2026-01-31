mod common;

use common::create_test_device;
use wgpu_playground_core::queue::{submit_single, write_buffer_typed, QueueOps};

// Helper struct for creating test textures with fewer function parameters
struct TestTextureParams {
    label: &'static str,
    width: u32,
    height: u32,
    depth_or_array_layers: u32,
    mip_level_count: u32,
    dimension: wgpu::TextureDimension,
    format: wgpu::TextureFormat,
    usage: wgpu::TextureUsages,
}

// Helper function to create a test texture with specified parameters
fn create_test_texture(device: &wgpu::Device, params: TestTextureParams) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some(params.label),
        size: wgpu::Extent3d {
            width: params.width,
            height: params.height,
            depth_or_array_layers: params.depth_or_array_layers,
        },
        mip_level_count: params.mip_level_count,
        sample_count: 1,
        dimension: params.dimension,
        format: params.format,
        usage: params.usage,
        view_formats: &[],
    })
}

// Helper function to create a pair of textures for copy testing
fn create_texture_pair(
    device: &wgpu::Device,
    width: u32,
    height: u32,
    depth_or_array_layers: u32,
    mip_level_count: u32,
    dimension: wgpu::TextureDimension,
    format: wgpu::TextureFormat,
) -> (wgpu::Texture, wgpu::Texture) {
    let src = create_test_texture(
        device,
        TestTextureParams {
            label: "Source Texture",
            width,
            height,
            depth_or_array_layers,
            mip_level_count,
            dimension,
            format,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::COPY_DST,
        },
    );
    let dst = create_test_texture(
        device,
        TestTextureParams {
            label: "Destination Texture",
            width,
            height,
            depth_or_array_layers,
            mip_level_count,
            dimension,
            format,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::COPY_SRC,
        },
    );
    (src, dst)
}

#[test]
fn test_queue_ops_creation() {
    pollster::block_on(async {
        let Some((_device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };
        let queue_ops = QueueOps::new(&queue);
        assert!(std::ptr::eq(queue_ops.inner(), &queue));
    });
}

#[test]
fn test_write_buffer_operation() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a buffer with COPY_DST usage
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Test Buffer"),
            size: 64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let queue_ops = QueueOps::new(&queue);
        let data = [1.0f32, 2.0, 3.0, 4.0];

        // Test write_buffer - this should not panic
        queue_ops.write_buffer(&buffer, 0, bytemuck::cast_slice(&data));

        // Poll the device to ensure the operation completes
        device.poll(wgpu::Maintain::Wait);
    });
}

#[test]
fn test_write_buffer_typed_helper() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a buffer with COPY_DST usage
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Test Buffer"),
            size: 64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let data = [1.0f32, 2.0, 3.0, 4.0];

        // Test typed helper - this should not panic
        write_buffer_typed(&queue, &buffer, 0, &data);

        // Poll the device to ensure the operation completes
        device.poll(wgpu::Maintain::Wait);
    });
}

#[test]
fn test_submit_command_buffer() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a simple command buffer
        let encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Test Encoder"),
        });
        let command_buffer = encoder.finish();

        let queue_ops = QueueOps::new(&queue);

        // Test submit - this should not panic and should return a submission index
        let _submission_index = queue_ops.submit(std::iter::once(command_buffer));

        // Poll the device to ensure the operation completes
        device.poll(wgpu::Maintain::Wait);
    });
}

#[test]
fn test_submit_single_helper() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a simple command buffer
        let encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Test Encoder"),
        });
        let command_buffer = encoder.finish();

        // Test submit_single helper - this should not panic
        let _submission_index = submit_single(&queue, command_buffer);

        // Poll the device to ensure the operation completes
        device.poll(wgpu::Maintain::Wait);
    });
}

#[test]
fn test_write_texture_operation() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a texture with COPY_DST usage
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Test Texture"),
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

        let queue_ops = QueueOps::new(&queue);

        // Create test data (4x4 RGBA texture = 64 bytes)
        let data = vec![255u8; 64];

        // Test write_texture - this should not panic
        queue_ops.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(16), // 4 pixels * 4 bytes per pixel
                rows_per_image: Some(4),
            },
            wgpu::Extent3d {
                width: 4,
                height: 4,
                depth_or_array_layers: 1,
            },
        );

        // Poll the device to ensure the operation completes
        device.poll(wgpu::Maintain::Wait);
    });
}

#[test]
fn test_multiple_buffer_writes() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create multiple buffers
        let buffer1 = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Test Buffer 1"),
            size: 64,
            usage: wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let buffer2 = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Test Buffer 2"),
            size: 64,
            usage: wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let queue_ops = QueueOps::new(&queue);

        // Write to multiple buffers
        let data1 = [1.0f32, 2.0, 3.0, 4.0];
        let data2 = [5.0f32, 6.0, 7.0, 8.0];

        queue_ops.write_buffer(&buffer1, 0, bytemuck::cast_slice(&data1));
        queue_ops.write_buffer(&buffer2, 0, bytemuck::cast_slice(&data2));

        // Poll the device to ensure operations complete
        device.poll(wgpu::Maintain::Wait);
    });
}

#[test]
fn test_submit_multiple_command_buffers() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create multiple command buffers
        let encoder1 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Test Encoder 1"),
        });
        let command_buffer1 = encoder1.finish();

        let encoder2 = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Test Encoder 2"),
        });
        let command_buffer2 = encoder2.finish();

        let queue_ops = QueueOps::new(&queue);

        // Submit multiple command buffers at once
        let _submission_index = queue_ops.submit([command_buffer1, command_buffer2]);

        // Poll the device to ensure operations complete
        device.poll(wgpu::Maintain::Wait);
    });
}

#[test]
fn test_buffer_write_with_offset() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a buffer
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Test Buffer"),
            size: 128,
            usage: wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let queue_ops = QueueOps::new(&queue);

        // Write data at different offsets
        let data1 = [1.0f32, 2.0, 3.0, 4.0];
        let data2 = [5.0f32, 6.0, 7.0, 8.0];

        queue_ops.write_buffer(&buffer, 0, bytemuck::cast_slice(&data1));
        queue_ops.write_buffer(&buffer, 64, bytemuck::cast_slice(&data2));

        // Poll the device to ensure operations complete
        device.poll(wgpu::Maintain::Wait);
    });
}

#[test]
fn test_copy_texture_to_texture_basic() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let (src_texture, dst_texture) = create_texture_pair(
            &device,
            256,
            256,
            1,
            1,
            wgpu::TextureDimension::D2,
            wgpu::TextureFormat::Rgba8Unorm,
        );

        // Test copy operation using helper function
        use wgpu_playground_core::queue::copy_texture_to_texture;
        let _submission_index = copy_texture_to_texture(
            &device,
            &queue,
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
                width: 256,
                height: 256,
                depth_or_array_layers: 1,
            },
        );

        // Poll the device to ensure the operation completes
        device.poll(wgpu::Maintain::Wait);
    });
}

#[test]
fn test_copy_texture_to_texture_with_queue_ops() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let (src_texture, dst_texture) = create_texture_pair(
            &device,
            128,
            128,
            1,
            1,
            wgpu::TextureDimension::D2,
            wgpu::TextureFormat::Rgba8Unorm,
        );

        // Test copy operation using QueueOps
        let queue_ops = QueueOps::with_device(&queue, &device);
        let _submission_index = queue_ops.copy_texture_to_texture(
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
                width: 128,
                height: 128,
                depth_or_array_layers: 1,
            },
        );

        // Poll the device to ensure the operation completes
        device.poll(wgpu::Maintain::Wait);
    });
}

#[test]
fn test_copy_texture_with_different_mip_levels() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let (src_texture, dst_texture) = create_texture_pair(
            &device,
            256,
            256,
            1,
            4, // 256x256, 128x128, 64x64, 32x32
            wgpu::TextureDimension::D2,
            wgpu::TextureFormat::Rgba8Unorm,
        );

        let queue_ops = QueueOps::with_device(&queue, &device);

        // Copy mip level 0 (256x256)
        queue_ops.copy_texture_to_texture(
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
                width: 256,
                height: 256,
                depth_or_array_layers: 1,
            },
        );

        // Copy mip level 1 (128x128)
        queue_ops.copy_texture_to_texture(
            wgpu::ImageCopyTexture {
                texture: &src_texture,
                mip_level: 1,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyTexture {
                texture: &dst_texture,
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

        // Copy mip level 2 (64x64)
        queue_ops.copy_texture_to_texture(
            wgpu::ImageCopyTexture {
                texture: &src_texture,
                mip_level: 2,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyTexture {
                texture: &dst_texture,
                mip_level: 2,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::Extent3d {
                width: 64,
                height: 64,
                depth_or_array_layers: 1,
            },
        );

        // Poll the device to ensure operations complete
        device.poll(wgpu::Maintain::Wait);
    });
}

#[test]
fn test_copy_texture_with_array_layers() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let (src_texture, dst_texture) = create_texture_pair(
            &device,
            128,
            128,
            4, // 4 array layers
            1,
            wgpu::TextureDimension::D2,
            wgpu::TextureFormat::Rgba8Unorm,
        );

        let queue_ops = QueueOps::with_device(&queue, &device);

        // Copy all array layers at once
        queue_ops.copy_texture_to_texture(
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
                width: 128,
                height: 128,
                depth_or_array_layers: 4,
            },
        );

        // Poll the device to ensure operations complete
        device.poll(wgpu::Maintain::Wait);
    });
}

#[test]
fn test_copy_texture_with_specific_array_layer() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let (src_texture, dst_texture) = create_texture_pair(
            &device,
            64,
            64,
            8,
            1,
            wgpu::TextureDimension::D2,
            wgpu::TextureFormat::Rgba8Unorm,
        );

        let queue_ops = QueueOps::with_device(&queue, &device);

        // Copy a specific layer (layer 2 to layer 5)
        queue_ops.copy_texture_to_texture(
            wgpu::ImageCopyTexture {
                texture: &src_texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: 0,
                    y: 0,
                    z: 2, // Source layer 2
                },
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyTexture {
                texture: &dst_texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: 0,
                    y: 0,
                    z: 5, // Destination layer 5
                },
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::Extent3d {
                width: 64,
                height: 64,
                depth_or_array_layers: 1,
            },
        );

        // Poll the device to ensure operations complete
        device.poll(wgpu::Maintain::Wait);
    });
}

#[test]
fn test_copy_texture_with_depth_aspect() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create depth textures with RENDER_ATTACHMENT usage
        let src_texture = create_test_texture(
            &device,
            TestTextureParams {
                label: "Source Depth Texture",
                width: 256,
                height: 256,
                depth_or_array_layers: 1,
                mip_level_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth32Float,
                usage: wgpu::TextureUsages::COPY_SRC
                    | wgpu::TextureUsages::COPY_DST
                    | wgpu::TextureUsages::RENDER_ATTACHMENT,
            },
        );

        let dst_texture = create_test_texture(
            &device,
            TestTextureParams {
                label: "Destination Depth Texture",
                width: 256,
                height: 256,
                depth_or_array_layers: 1,
                mip_level_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth32Float,
                usage: wgpu::TextureUsages::COPY_DST
                    | wgpu::TextureUsages::COPY_SRC
                    | wgpu::TextureUsages::RENDER_ATTACHMENT,
            },
        );

        let queue_ops = QueueOps::with_device(&queue, &device);

        // Copy depth aspect
        queue_ops.copy_texture_to_texture(
            wgpu::ImageCopyTexture {
                texture: &src_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::DepthOnly,
            },
            wgpu::ImageCopyTexture {
                texture: &dst_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::DepthOnly,
            },
            wgpu::Extent3d {
                width: 256,
                height: 256,
                depth_or_array_layers: 1,
            },
        );

        // Poll the device to ensure operations complete
        device.poll(wgpu::Maintain::Wait);
    });
}

#[test]
fn test_copy_texture_partial_region() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let (src_texture, dst_texture) = create_texture_pair(
            &device,
            256,
            256,
            1,
            1,
            wgpu::TextureDimension::D2,
            wgpu::TextureFormat::Rgba8Unorm,
        );

        let queue_ops = QueueOps::with_device(&queue, &device);

        // Copy a partial region (64x64 from position (64, 64) to position (128, 128))
        queue_ops.copy_texture_to_texture(
            wgpu::ImageCopyTexture {
                texture: &src_texture,
                mip_level: 0,
                origin: wgpu::Origin3d { x: 64, y: 64, z: 0 },
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyTexture {
                texture: &dst_texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: 128,
                    y: 128,
                    z: 0,
                },
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::Extent3d {
                width: 64,
                height: 64,
                depth_or_array_layers: 1,
            },
        );

        // Poll the device to ensure operations complete
        device.poll(wgpu::Maintain::Wait);
    });
}

#[test]
fn test_copy_texture_3d() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let (src_texture, dst_texture) = create_texture_pair(
            &device,
            64,
            64,
            64,
            1,
            wgpu::TextureDimension::D3,
            wgpu::TextureFormat::Rgba8Unorm,
        );

        let queue_ops = QueueOps::with_device(&queue, &device);

        // Copy the entire 3D texture
        queue_ops.copy_texture_to_texture(
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
                depth_or_array_layers: 64,
            },
        );

        // Poll the device to ensure operations complete
        device.poll(wgpu::Maintain::Wait);
    });
}
