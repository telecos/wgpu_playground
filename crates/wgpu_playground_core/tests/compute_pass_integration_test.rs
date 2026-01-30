use wgpu_playground_core::{
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

#[test]
fn test_compute_pipeline_creation() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a simple compute shader
        let shader_source = r#"
            @compute @workgroup_size(64)
            fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
                // Simple compute shader that does nothing
            }
        "#;

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Test Compute Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // Create compute pipeline using our abstraction
        let descriptor = ComputePipelineDescriptor::new(&shader_module, "main")
            .with_label("Test Compute Pipeline");

        let _pipeline = create_compute_pipeline(&device, &descriptor);
    });
}

#[test]
fn test_compute_pass_with_dispatch() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a simple compute shader
        let shader_source = r#"
            @compute @workgroup_size(64)
            fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
                // Simple compute shader
            }
        "#;

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Test Compute Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // Create compute pipeline
        let descriptor = ComputePipelineDescriptor::new(&shader_module, "main");
        let pipeline = create_compute_pipeline(&device, &descriptor);

        // Create command encoder and compute pass
        let mut encoder = CommandEncoderOps::new(&device, Some("Test Encoder"));
        {
            let mut compute_pass = encoder.begin_compute_pass(Some("Test Compute Pass"));
            compute_pass.set_pipeline(&pipeline);
            compute_pass.dispatch_workgroups(1, 1, 1);
        } // Compute pass ends here

        // Finish and submit
        let command_buffer = encoder.finish();
        let queue_ops = QueueOps::new(&queue);
        queue_ops.submit(std::iter::once(command_buffer));

        // Wait for completion
        device.poll(wgpu::Maintain::Wait);
    });
}

#[test]
fn test_compute_pass_with_bind_group() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a compute shader that uses a storage buffer
        let shader_source = r#"
            @group(0) @binding(0)
            var<storage, read_write> data: array<f32>;

            @compute @workgroup_size(64)
            fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
                let index = global_id.x;
                data[index] = f32(index);
            }
        "#;

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Test Compute Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // Create a storage buffer
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Storage Buffer"),
            size: 256 * 4, // 256 floats
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Compute Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Compute Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Compute Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create compute pipeline with layout
        let descriptor =
            ComputePipelineDescriptor::new(&shader_module, "main").with_layout(&pipeline_layout);
        let pipeline = create_compute_pipeline(&device, &descriptor);

        // Create command encoder and compute pass
        let mut encoder = CommandEncoderOps::new(&device, Some("Test Encoder"));
        {
            let mut compute_pass = encoder.begin_compute_pass(Some("Test Compute Pass"));
            compute_pass.set_pipeline(&pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups(4, 1, 1); // 4 workgroups of 64 threads = 256 threads
        }

        // Finish and submit
        let command_buffer = encoder.finish();
        let queue_ops = QueueOps::new(&queue);
        queue_ops.submit(std::iter::once(command_buffer));

        // Wait for completion
        device.poll(wgpu::Maintain::Wait);
    });
}

#[test]
fn test_compute_pass_with_dispatch_workgroups_indirect() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a simple compute shader
        let shader_source = r#"
            @compute @workgroup_size(64)
            fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
                // Simple compute shader
            }
        "#;

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Test Compute Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // Create compute pipeline
        let descriptor = ComputePipelineDescriptor::new(&shader_module, "main");
        let pipeline = create_compute_pipeline(&device, &descriptor);

        // Create indirect buffer with dispatch parameters (x=2, y=1, z=1)
        let indirect_data: [u32; 3] = [2, 1, 1];
        let indirect_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Indirect Buffer"),
            size: 12, // 3 u32s
            usage: wgpu::BufferUsages::INDIRECT | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Write dispatch parameters to the buffer
        queue.write_buffer(&indirect_buffer, 0, bytemuck::cast_slice(&indirect_data));

        // Create command encoder and compute pass
        let mut encoder = CommandEncoderOps::new(&device, Some("Test Encoder"));
        {
            let mut compute_pass = encoder.begin_compute_pass(Some("Test Compute Pass"));
            compute_pass.set_pipeline(&pipeline);
            compute_pass.dispatch_workgroups_indirect(&indirect_buffer, 0);
        }

        // Finish and submit
        let command_buffer = encoder.finish();
        let queue_ops = QueueOps::new(&queue);
        queue_ops.submit(std::iter::once(command_buffer));

        // Wait for completion
        device.poll(wgpu::Maintain::Wait);
    });
}

#[test]
fn test_multiple_compute_passes() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // Create a simple compute shader
        let shader_source = r#"
            @compute @workgroup_size(64)
            fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
                // Simple compute shader
            }
        "#;

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Test Compute Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // Create compute pipeline
        let descriptor = ComputePipelineDescriptor::new(&shader_module, "main");
        let pipeline = create_compute_pipeline(&device, &descriptor);

        // Create command encoder with multiple compute passes
        let mut encoder = CommandEncoderOps::new(&device, Some("Test Encoder"));

        // First compute pass
        {
            let mut compute_pass = encoder.begin_compute_pass(Some("First Compute Pass"));
            compute_pass.set_pipeline(&pipeline);
            compute_pass.dispatch_workgroups(1, 1, 1);
        }

        // Second compute pass
        {
            let mut compute_pass = encoder.begin_compute_pass(Some("Second Compute Pass"));
            compute_pass.set_pipeline(&pipeline);
            compute_pass.dispatch_workgroups(2, 1, 1);
        }

        // Finish and submit
        let command_buffer = encoder.finish();
        let queue_ops = QueueOps::new(&queue);
        queue_ops.submit(std::iter::once(command_buffer));

        // Wait for completion
        device.poll(wgpu::Maintain::Wait);
    });
}
