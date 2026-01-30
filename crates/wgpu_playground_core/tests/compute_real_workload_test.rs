use wgpu_playground_core::{
    buffer::BufferOps,
    command_encoder::CommandEncoderOps,
    compute_pipeline::{create_compute_pipeline, ComputePipelineDescriptor},
    queue::QueueOps,
};

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

/// Test a complete compute shader workflow: vector addition
/// Demonstrates compute pipeline creation, bind groups, and dispatch with actual computation
#[test]
fn test_vector_addition_compute_shader() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create compute shader that adds two vectors
        let shader_source = r#"
            @group(0) @binding(0)
            var<storage, read> input_a: array<f32>;

            @group(0) @binding(1)
            var<storage, read> input_b: array<f32>;

            @group(0) @binding(2)
            var<storage, read_write> output: array<f32>;

            @compute @workgroup_size(64)
            fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
                let index = global_id.x;
                output[index] = input_a[index] + input_b[index];
            }
        "#;

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Vector Addition Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // Create input data
        let size = 256u64;
        let input_a: Vec<f32> = (0..size).map(|i| i as f32).collect();
        let input_b: Vec<f32> = (0..size).map(|i| (i * 2) as f32).collect();

        // Create storage buffers
        let buffer_a = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Input Buffer A"),
            size: size * 4,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let buffer_b = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Input Buffer B"),
            size: size * 4,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let buffer_output = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Output Buffer"),
            size: size * 4,
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_SRC
                | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create staging buffer for reading results
        let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Staging Buffer"),
            size: size * 4,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Write input data
        queue.write_buffer(&buffer_a, 0, bytemuck::cast_slice(&input_a));
        queue.write_buffer(&buffer_b, 0, bytemuck::cast_slice(&input_b));

        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Compute Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Compute Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer_a.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: buffer_b.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: buffer_output.as_entire_binding(),
                },
            ],
        });

        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Compute Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create compute pipeline
        let descriptor = ComputePipelineDescriptor::new(&shader_module, "main")
            .with_label("Vector Addition Pipeline")
            .with_layout(&pipeline_layout);
        let pipeline = create_compute_pipeline(&device, &descriptor);

        // Create command encoder and execute compute pass
        let mut encoder = CommandEncoderOps::new(&device, Some("Compute Encoder"));
        {
            let mut compute_pass = encoder.begin_compute_pass(Some("Vector Addition Pass"));
            compute_pass.set_pipeline(&pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            // Dispatch 4 workgroups (4 * 64 = 256 threads)
            compute_pass.dispatch_workgroups(4, 1, 1);
        }

        // Copy result to staging buffer
        encoder
            .inner_mut()
            .copy_buffer_to_buffer(&buffer_output, 0, &staging_buffer, 0, size * 4);

        // Submit and wait
        let command_buffer = encoder.finish();
        let queue_ops = QueueOps::new(&queue);
        queue_ops.submit(std::iter::once(command_buffer));

        // Read back results
        BufferOps::map_read(&staging_buffer).await.unwrap();
        let data = BufferOps::get_mapped_range(&staging_buffer);
        let result: Vec<f32> = bytemuck::cast_slice(&data).to_vec();
        drop(data);
        BufferOps::unmap(&staging_buffer);

        // Verify results
        for i in 0..size as usize {
            let expected = input_a[i] + input_b[i];
            assert_eq!(
                result[i], expected,
                "Mismatch at index {}: got {}, expected {}",
                i, result[i], expected
            );
        }

        println!(
            "Vector addition test passed! All {} elements computed correctly.",
            size
        );
    });
}

/// Test matrix multiplication using compute shaders
#[test]
fn test_matrix_multiply_compute_shader() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Simple 4x4 matrix multiplication
        let shader_source = r#"
            @group(0) @binding(0)
            var<storage, read> matrix_a: array<f32>;

            @group(0) @binding(1)
            var<storage, read> matrix_b: array<f32>;

            @group(0) @binding(2)
            var<storage, read_write> matrix_result: array<f32>;

            const SIZE: u32 = 4u;

            @compute @workgroup_size(4, 4)
            fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
                let row = global_id.y;
                let col = global_id.x;

                if (row >= SIZE || col >= SIZE) {
                    return;
                }

                var sum = 0.0;
                for (var k = 0u; k < SIZE; k = k + 1u) {
                    sum = sum + matrix_a[row * SIZE + k] * matrix_b[k * SIZE + col];
                }

                matrix_result[row * SIZE + col] = sum;
            }
        "#;

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Matrix Multiply Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // Create 4x4 identity matrices
        #[rustfmt::skip]
        let matrix_a: Vec<f32> = vec![
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ];

        #[rustfmt::skip]
        let matrix_b: Vec<f32> = vec![
            2.0, 0.0, 0.0, 0.0,
            0.0, 2.0, 0.0, 0.0,
            0.0, 0.0, 2.0, 0.0,
            0.0, 0.0, 0.0, 2.0,
        ];

        let buffer_size = 16 * 4; // 16 floats * 4 bytes

        // Create buffers
        let buffer_a = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Matrix A"),
            size: buffer_size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let buffer_b = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Matrix B"),
            size: buffer_size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let buffer_result = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Matrix Result"),
            size: buffer_size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Staging Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Write matrices
        queue.write_buffer(&buffer_a, 0, bytemuck::cast_slice(&matrix_a));
        queue.write_buffer(&buffer_b, 0, bytemuck::cast_slice(&matrix_b));

        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Matrix Multiply Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Matrix Multiply Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer_a.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: buffer_b.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: buffer_result.as_entire_binding(),
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Matrix Multiply Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create pipeline
        let descriptor = ComputePipelineDescriptor::new(&shader_module, "main")
            .with_label("Matrix Multiply Pipeline")
            .with_layout(&pipeline_layout);
        let pipeline = create_compute_pipeline(&device, &descriptor);

        // Execute compute pass
        let mut encoder = CommandEncoderOps::new(&device, Some("Matrix Multiply Encoder"));
        {
            let mut compute_pass = encoder.begin_compute_pass(Some("Matrix Multiply Pass"));
            compute_pass.set_pipeline(&pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups(1, 1, 1); // Single workgroup of 4x4 threads
        }

        encoder.inner_mut().copy_buffer_to_buffer(
            &buffer_result,
            0,
            &staging_buffer,
            0,
            buffer_size,
        );

        let command_buffer = encoder.finish();
        let queue_ops = QueueOps::new(&queue);
        queue_ops.submit(std::iter::once(command_buffer));

        // Read results
        BufferOps::map_read(&staging_buffer).await.unwrap();
        let data = BufferOps::get_mapped_range(&staging_buffer);
        let result: Vec<f32> = bytemuck::cast_slice(&data).to_vec();
        drop(data);
        BufferOps::unmap(&staging_buffer);

        // Expected result: 2 * identity matrix
        #[rustfmt::skip]
        let expected: Vec<f32> = vec![
            2.0, 0.0, 0.0, 0.0,
            0.0, 2.0, 0.0, 0.0,
            0.0, 0.0, 2.0, 0.0,
            0.0, 0.0, 0.0, 2.0,
        ];

        for i in 0..16 {
            assert!(
                (result[i] - expected[i]).abs() < 0.0001,
                "Mismatch at index {}: got {}, expected {}",
                i,
                result[i],
                expected[i]
            );
        }

        println!("Matrix multiplication test passed!");
    });
}
