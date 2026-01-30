use wgpu_playground_core::queue::{submit_single, write_buffer_typed, QueueOps};

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
