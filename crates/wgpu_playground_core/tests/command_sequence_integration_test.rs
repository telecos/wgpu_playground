mod common;

use common::create_test_device;
use wgpu_playground_core::command_encoder::CommandEncoderOps;
use wgpu_playground_core::compute::ComputePipelineDescriptor;
use wgpu_playground_core::compute_pass_encoder::{ComputePassDescriptor, ComputePassEncoder};
use wgpu_playground_core::render_pass_encoder::{
    Color, RenderPassColorAttachment, RenderPassDescriptor, RenderPassEncoder,
};
use wgpu_playground_core::shader::ShaderModule;

/// Test that we can create multiple encoders and submit them in sequence
#[test]
fn test_sequential_command_submission() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create buffers for testing
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

        // Write initial data
        queue.write_buffer(&buffer1, 0, &vec![1u8; 256]);
        queue.write_buffer(&buffer2, 0, &vec![2u8; 256]);

        // First encoder: buffer1 -> buffer3
        let mut encoder1 = CommandEncoderOps::new(&device, Some("Encoder 1"));
        encoder1.copy_buffer_to_buffer(&buffer1, 0, &buffer3, 0, 128);
        let cmd1 = encoder1.finish();

        // Second encoder: buffer2 -> buffer3 (second half)
        let mut encoder2 = CommandEncoderOps::new(&device, Some("Encoder 2"));
        encoder2.copy_buffer_to_buffer(&buffer2, 0, &buffer3, 128, 128);
        let cmd2 = encoder2.finish();

        // Submit in sequence
        queue.submit(std::iter::once(cmd1));
        queue.submit(std::iter::once(cmd2));
        device.poll(wgpu::Maintain::Wait);

        // Verify the result
        let buffer_slice = buffer3.slice(..);
        let (tx, rx) = futures_channel::oneshot::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        device.poll(wgpu::Maintain::Wait);
        rx.await.unwrap().unwrap();

        let data = buffer_slice.get_mapped_range();
        // First half should be 1, second half should be 2
        assert_eq!(data[0], 1);
        assert_eq!(data[127], 1);
        assert_eq!(data[128], 2);
        assert_eq!(data[255], 2);
        drop(data);
        buffer3.unmap();
    });
}

/// Test combining render pass and copy operations in a single encoder
#[test]
fn test_render_and_copy_sequence() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a render target
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Render Target"),
            size: wgpu::Extent3d {
                width: 256,
                height: 256,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create a buffer to copy texture data to
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Copy Target"),
            size: 256 * 256 * 4, // RGBA8
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        // Single encoder with render pass followed by copy
        let mut encoder = CommandEncoderOps::new(&device, Some("Combined Encoder"));

        // Render pass
        {
            let descriptor = RenderPassDescriptor::new()
                .with_label("Clear Pass")
                .with_color_attachment(RenderPassColorAttachment::clear(&view, Color::RED));

            let render_pass = RenderPassEncoder::begin(encoder.inner_mut(), &descriptor);
            assert!(render_pass.is_ok());
        }

        // Copy operation after render pass
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
                    bytes_per_row: Some(256 * 4),
                    rows_per_image: Some(256),
                },
            },
            wgpu::Extent3d {
                width: 256,
                height: 256,
                depth_or_array_layers: 1,
            },
        );

        let command_buffer = encoder.finish();
        queue.submit(std::iter::once(command_buffer));
        device.poll(wgpu::Maintain::Wait);
    });
}

/// Test combining compute pass and copy operations in a single encoder
#[test]
fn test_compute_and_copy_sequence() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a compute shader
        let shader_source = r#"
@group(0) @binding(0)
var<storage, read_write> output: array<u32>;

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    if (global_id.x < arrayLength(&output)) {
        output[global_id.x] = global_id.x * 2u;
    }
}
"#;

        let shader = ShaderModule::from_source(shader_source, Some("test_compute")).unwrap();

        let pipeline_descriptor = ComputePipelineDescriptor::new(Some("test_pipeline"))
            .with_shader(shader)
            .with_entry_point("main");

        let pipeline = pipeline_descriptor.create_pipeline(&device).unwrap();

        // Create storage buffer
        let buffer_size = 64 * std::mem::size_of::<u32>() as u64;
        let storage_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Storage Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        // Create readback buffer
        let readback_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Readback Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        // Create bind group
        let bind_group_layout = pipeline.get_bind_group_layout(0);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("test_bind_group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: storage_buffer.as_entire_binding(),
            }],
        });

        // Single encoder with compute pass followed by copy
        let mut encoder = CommandEncoderOps::new(&device, Some("Compute and Copy"));

        // Compute pass
        {
            let descriptor = ComputePassDescriptor::new().with_label("Compute Pass");
            let mut compute_pass =
                ComputePassEncoder::begin(encoder.inner_mut(), &descriptor).unwrap();
            compute_pass.set_pipeline(&pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch(64, 1, 1);
        }

        // Copy operation after compute pass
        encoder.copy_buffer_to_buffer(&storage_buffer, 0, &readback_buffer, 0, buffer_size);

        let command_buffer = encoder.finish();
        queue.submit(std::iter::once(command_buffer));
        device.poll(wgpu::Maintain::Wait);

        // Verify the compute results
        let buffer_slice = readback_buffer.slice(..);
        let (tx, rx) = futures_channel::oneshot::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        device.poll(wgpu::Maintain::Wait);
        rx.await.unwrap().unwrap();

        let data = buffer_slice.get_mapped_range();
        let data_u32 = bytemuck::cast_slice::<u8, u32>(&data);
        // Verify compute shader wrote correct values
        assert_eq!(data_u32[0], 0);
        assert_eq!(data_u32[1], 2);
        assert_eq!(data_u32[2], 4);
        drop(data);
        readback_buffer.unmap();
    });
}

/// Test multiple render passes in sequence
#[test]
fn test_multiple_render_passes_sequence() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create two render targets
        let texture1 = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Render Target 1"),
            size: wgpu::Extent3d {
                width: 256,
                height: 256,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let texture2 = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Render Target 2"),
            size: wgpu::Extent3d {
                width: 256,
                height: 256,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let view1 = texture1.create_view(&wgpu::TextureViewDescriptor::default());
        let view2 = texture2.create_view(&wgpu::TextureViewDescriptor::default());

        // Single encoder with multiple render passes
        let mut encoder = CommandEncoderOps::new(&device, Some("Multi-Pass Encoder"));

        // First render pass
        {
            let descriptor = RenderPassDescriptor::new()
                .with_label("Pass 1")
                .with_color_attachment(RenderPassColorAttachment::clear(&view1, Color::RED));

            let render_pass = RenderPassEncoder::begin(encoder.inner_mut(), &descriptor);
            assert!(render_pass.is_ok());
        }

        // Second render pass
        {
            let descriptor = RenderPassDescriptor::new()
                .with_label("Pass 2")
                .with_color_attachment(RenderPassColorAttachment::clear(&view2, Color::BLUE));

            let render_pass = RenderPassEncoder::begin(encoder.inner_mut(), &descriptor);
            assert!(render_pass.is_ok());
        }

        let command_buffer = encoder.finish();
        queue.submit(std::iter::once(command_buffer));
        device.poll(wgpu::Maintain::Wait);
    });
}

/// Test multiple compute passes in sequence
#[test]
fn test_multiple_compute_passes_sequence() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a simple compute shader
        let shader_source = r#"
@group(0) @binding(0)
var<storage, read_write> data: array<u32>;

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    if (global_id.x < arrayLength(&data)) {
        data[global_id.x] = data[global_id.x] + 1u;
    }
}
"#;

        let shader = ShaderModule::from_source(shader_source, Some("test_compute")).unwrap();

        let pipeline_descriptor = ComputePipelineDescriptor::new(Some("test_pipeline"))
            .with_shader(shader)
            .with_entry_point("main");

        let pipeline = pipeline_descriptor.create_pipeline(&device).unwrap();

        // Create storage buffer initialized with zeros
        let buffer_size = 64 * std::mem::size_of::<u32>() as u64;
        let storage_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Storage Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_SRC
                | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Initialize with zeros
        queue.write_buffer(&storage_buffer, 0, &vec![0u8; buffer_size as usize]);

        // Create bind group
        let bind_group_layout = pipeline.get_bind_group_layout(0);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("test_bind_group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: storage_buffer.as_entire_binding(),
            }],
        });

        // Single encoder with multiple compute passes
        let mut encoder = CommandEncoderOps::new(&device, Some("Multi-Compute Encoder"));

        // First compute pass (increment by 1)
        {
            let descriptor = ComputePassDescriptor::new().with_label("Compute Pass 1");
            let mut compute_pass =
                ComputePassEncoder::begin(encoder.inner_mut(), &descriptor).unwrap();
            compute_pass.set_pipeline(&pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch(64, 1, 1);
        }

        // Second compute pass (increment by 1 again)
        {
            let descriptor = ComputePassDescriptor::new().with_label("Compute Pass 2");
            let mut compute_pass =
                ComputePassEncoder::begin(encoder.inner_mut(), &descriptor).unwrap();
            compute_pass.set_pipeline(&pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch(64, 1, 1);
        }

        let command_buffer = encoder.finish();
        queue.submit(std::iter::once(command_buffer));
        device.poll(wgpu::Maintain::Wait);

        // Verify the result - each element should be 2 (incremented twice)
        let readback_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Readback"),
            size: buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let mut copy_encoder = CommandEncoderOps::new(&device, Some("Copy"));
        copy_encoder.copy_buffer_to_buffer(&storage_buffer, 0, &readback_buffer, 0, buffer_size);
        queue.submit(std::iter::once(copy_encoder.finish()));
        device.poll(wgpu::Maintain::Wait);

        let buffer_slice = readback_buffer.slice(..);
        let (tx, rx) = futures_channel::oneshot::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        device.poll(wgpu::Maintain::Wait);
        rx.await.unwrap().unwrap();

        let data = buffer_slice.get_mapped_range();
        let data_u32 = bytemuck::cast_slice::<u8, u32>(&data);
        // Each value should be 2 (incremented twice)
        assert_eq!(data_u32[0], 2);
        assert_eq!(data_u32[31], 2);
        assert_eq!(data_u32[63], 2);
        drop(data);
        readback_buffer.unmap();
    });
}

/// Test complex sequence: copy -> compute -> render -> copy
#[test]
fn test_complex_command_sequence() {
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
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });

        // Create texture for render pass
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Render Target"),
            size: wgpu::Extent3d {
                width: 16,
                height: 16,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create readback buffer for texture
        let texture_readback = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Texture Readback"),
            size: 16 * 16 * 4,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        // Write initial data
        queue.write_buffer(&src_buffer, 0, &vec![42u8; 256]);

        // Create a simple compute shader
        let shader_source = r#"
@group(0) @binding(0)
var<storage, read_write> data: array<u32>;

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    if (global_id.x < arrayLength(&data)) {
        data[global_id.x] = global_id.x;
    }
}
"#;

        let shader = ShaderModule::from_source(shader_source, Some("test_compute")).unwrap();
        let pipeline_descriptor = ComputePipelineDescriptor::new(Some("test_pipeline"))
            .with_shader(shader)
            .with_entry_point("main");
        let pipeline = pipeline_descriptor.create_pipeline(&device).unwrap();

        let bind_group_layout = pipeline.get_bind_group_layout(0);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("test_bind_group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: dst_buffer.as_entire_binding(),
            }],
        });

        // Complex command sequence in a single encoder
        let mut encoder = CommandEncoderOps::new(&device, Some("Complex Sequence"));

        // 1. Copy operation
        encoder.copy_buffer_to_buffer(&src_buffer, 0, &dst_buffer, 0, 256);

        // 2. Compute pass
        {
            let descriptor = ComputePassDescriptor::new().with_label("Compute");
            let mut compute_pass =
                ComputePassEncoder::begin(encoder.inner_mut(), &descriptor).unwrap();
            compute_pass.set_pipeline(&pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch(64, 1, 1);
        }

        // 3. Render pass
        {
            let descriptor = RenderPassDescriptor::new()
                .with_label("Render")
                .with_color_attachment(RenderPassColorAttachment::clear(&view, Color::GREEN));

            let render_pass = RenderPassEncoder::begin(encoder.inner_mut(), &descriptor);
            assert!(render_pass.is_ok());
        }

        // 4. Copy texture to buffer
        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyBuffer {
                buffer: &texture_readback,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(16 * 4),
                    rows_per_image: Some(16),
                },
            },
            wgpu::Extent3d {
                width: 16,
                height: 16,
                depth_or_array_layers: 1,
            },
        );

        let command_buffer = encoder.finish();
        queue.submit(std::iter::once(command_buffer));
        device.poll(wgpu::Maintain::Wait);
    });
}

/// Test clear buffer operation
#[test]
fn test_clear_buffer() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a buffer
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Test Buffer"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        // Write some data
        queue.write_buffer(&buffer, 0, &vec![42u8; 256]);

        // Clear using encoder
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Clear Encoder"),
        });

        encoder.clear_buffer(&buffer, 0, None);

        queue.submit(std::iter::once(encoder.finish()));
        device.poll(wgpu::Maintain::Wait);

        // Verify buffer is cleared
        let buffer_slice = buffer.slice(..);
        let (tx, rx) = futures_channel::oneshot::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        device.poll(wgpu::Maintain::Wait);
        rx.await.unwrap().unwrap();

        let data = buffer_slice.get_mapped_range();
        // All bytes should be 0 after clear
        assert_eq!(data[0], 0);
        assert_eq!(data[127], 0);
        assert_eq!(data[255], 0);
        drop(data);
        buffer.unmap();
    });
}

/// Test partial buffer clear
#[test]
fn test_partial_clear_buffer() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a buffer
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Test Buffer"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        // Write some data
        queue.write_buffer(&buffer, 0, &vec![42u8; 256]);

        // Clear only middle portion (64 bytes at offset 96)
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Clear Encoder"),
        });

        encoder.clear_buffer(&buffer, 96, Some(64));

        queue.submit(std::iter::once(encoder.finish()));
        device.poll(wgpu::Maintain::Wait);

        // Verify partial clear
        let buffer_slice = buffer.slice(..);
        let (tx, rx) = futures_channel::oneshot::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        device.poll(wgpu::Maintain::Wait);
        rx.await.unwrap().unwrap();

        let data = buffer_slice.get_mapped_range();
        // Before cleared region should still be 42
        assert_eq!(data[0], 42);
        assert_eq!(data[95], 42);
        // Cleared region should be 0
        assert_eq!(data[96], 0);
        assert_eq!(data[159], 0);
        // After cleared region should still be 42
        assert_eq!(data[160], 42);
        assert_eq!(data[255], 42);
        drop(data);
        buffer.unmap();
    });
}

/// Test interleaved copy and clear operations
#[test]
fn test_copy_and_clear_sequence() {
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

        // Write data to source
        queue.write_buffer(&src, 0, &vec![100u8; 256]);

        let mut encoder = CommandEncoderOps::new(&device, Some("Copy and Clear"));

        // Copy first half
        encoder.copy_buffer_to_buffer(&src, 0, &dst, 0, 128);

        // Clear source
        encoder.inner_mut().clear_buffer(&src, 0, None);

        // Copy second half (should now be zeros from cleared source)
        encoder.copy_buffer_to_buffer(&src, 0, &dst, 128, 128);

        queue.submit(std::iter::once(encoder.finish()));
        device.poll(wgpu::Maintain::Wait);

        // Verify
        let buffer_slice = dst.slice(..);
        let (tx, rx) = futures_channel::oneshot::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        device.poll(wgpu::Maintain::Wait);
        rx.await.unwrap().unwrap();

        let data = buffer_slice.get_mapped_range();
        // First half should be original data
        assert_eq!(data[0], 100);
        assert_eq!(data[127], 100);
        // Second half should be zeros (from cleared buffer)
        assert_eq!(data[128], 0);
        assert_eq!(data[255], 0);
        drop(data);
        dst.unmap();
    });
}
