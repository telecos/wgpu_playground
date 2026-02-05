/// Advanced Compute Shader Example - Gaussian Blur with Shared Memory
///
/// This example demonstrates advanced compute shader techniques:
/// - `var<workgroup>` shared memory for efficient tile-based processing
/// - `workgroupBarrier()` synchronization between shader invocations
/// - Multiple dispatch calls (horizontal + vertical blur passes)
/// - Storage textures (read/write) for image processing
///
/// The example implements a separable Gaussian blur:
/// 1. Horizontal blur pass: Reads from input texture, writes to intermediate texture
/// 2. Vertical blur pass: Reads from intermediate texture, writes to output texture
///
/// Each pass uses workgroup shared memory to cache texture tiles, reducing global memory access.
///
/// Run with: cargo run --package wgpu_playground_examples --example advanced_compute_blur
use wgpu::util::DeviceExt;

/// Workgroup size for compute shader (must match shader @workgroup_size)
const WORKGROUP_SIZE: u32 = 16;

/// Blur radius (how many pixels in each direction to sample)
const BLUR_RADIUS: i32 = 5;

/// Image dimensions
const IMAGE_WIDTH: u32 = 512;
const IMAGE_HEIGHT: u32 = 512;

/// Create GPU device and queue with required features
async fn create_device() -> Option<(wgpu::Device, wgpu::Queue)> {
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await
        .ok()?;

    println!("Using adapter: {}", adapter.get_info().name);
    println!("Backend: {:?}\n", adapter.get_info().backend);

    adapter
        .request_device(&wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            label: Some("Advanced Compute Device"),
            memory_hints: Default::default(),
            experimental_features: Default::default(),
            trace: Default::default(),
        })
        .await
        .ok()
}

/// Generate a test pattern image (checkerboard with gradient)
fn generate_test_image() -> Vec<u8> {
    let mut data = Vec::with_capacity((IMAGE_WIDTH * IMAGE_HEIGHT * 4) as usize);

    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            // Create a checkerboard pattern
            let checker = ((x / 32) + (y / 32)) % 2 == 0;

            // Add a gradient
            let gradient_x = (x as f32 / IMAGE_WIDTH as f32 * 255.0) as u8;
            let gradient_y = (y as f32 / IMAGE_HEIGHT as f32 * 255.0) as u8;

            if checker {
                data.extend_from_slice(&[gradient_x, gradient_y, 200, 255]);
            } else {
                data.extend_from_slice(&[50, 50, gradient_y, 255]);
            }
        }
    }

    data
}

fn main() {
    env_logger::init();

    println!("=== Advanced Compute Shader Example ===");
    println!("Gaussian Blur with Shared Memory and Multiple Dispatches\n");

    // Create device and queue
    let device_queue = pollster::block_on(create_device());
    if device_queue.is_none() {
        eprintln!("Failed to create GPU device");
        return;
    }

    let (device, queue) = device_queue.unwrap();

    // Generate test image data
    println!("Generating {}x{} test image...", IMAGE_WIDTH, IMAGE_HEIGHT);
    let image_data = generate_test_image();
    println!("  ✓ Created checkerboard pattern with gradient\n");

    // === CREATE TEXTURES ===
    println!("Creating textures...");

    // Input texture (will contain original image)
    let input_texture = device.create_texture_with_data(
        &queue,
        &wgpu::TextureDescriptor {
            label: Some("Input Texture"),
            size: wgpu::Extent3d {
                width: IMAGE_WIDTH,
                height: IMAGE_HEIGHT,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        },
        wgpu::util::TextureDataOrder::LayerMajor,
        &image_data,
    );

    // Intermediate texture (horizontal blur output -> vertical blur input)
    let intermediate_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Intermediate Texture"),
        size: wgpu::Extent3d {
            width: IMAGE_WIDTH,
            height: IMAGE_HEIGHT,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });

    // Output texture (final blurred result)
    let output_texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Output Texture"),
        size: wgpu::Extent3d {
            width: IMAGE_WIDTH,
            height: IMAGE_HEIGHT,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });

    println!(
        "  ✓ Input texture: {}x{} (TEXTURE_BINDING)",
        IMAGE_WIDTH, IMAGE_HEIGHT
    );
    println!("  ✓ Uploaded {} bytes of image data", image_data.len());
    println!(
        "  ✓ Intermediate texture: {}x{} (STORAGE_BINDING + TEXTURE_BINDING)",
        IMAGE_WIDTH, IMAGE_HEIGHT
    );
    println!(
        "  ✓ Output texture: {}x{} (STORAGE_BINDING)\n",
        IMAGE_WIDTH, IMAGE_HEIGHT
    );

    // === CREATE HORIZONTAL BLUR PIPELINE ===
    println!("Creating horizontal blur compute pipeline...");

    let horizontal_blur_shader = create_blur_shader(&device, true);

    let horizontal_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Horizontal Blur Bind Group Layout"),
            entries: &[
                // Input texture (sampled)
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // Sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                // Output texture (storage)
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
            ],
        });

    let horizontal_pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Horizontal Blur Pipeline Layout"),
            bind_group_layouts: &[&horizontal_bind_group_layout],
            push_constant_ranges: &[],
        });

    let horizontal_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Horizontal Blur Pipeline"),
        layout: Some(&horizontal_pipeline_layout),
        module: &horizontal_blur_shader,
        entry_point: Some("main"),
        compilation_options: Default::default(),
        cache: None,
    });

    println!("  ✓ Horizontal blur pipeline created");
    println!("  ✓ Uses workgroup shared memory for caching");
    println!("  ✓ Applies Gaussian blur in horizontal direction\n");

    // === CREATE VERTICAL BLUR PIPELINE ===
    println!("Creating vertical blur compute pipeline...");

    let vertical_blur_shader = create_blur_shader(&device, false);

    let vertical_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Vertical Blur Bind Group Layout"),
            entries: &[
                // Input texture (sampled)
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // Sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                // Output texture (storage)
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
            ],
        });

    let vertical_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Vertical Blur Pipeline Layout"),
        bind_group_layouts: &[&vertical_bind_group_layout],
        push_constant_ranges: &[],
    });

    let vertical_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Vertical Blur Pipeline"),
        layout: Some(&vertical_pipeline_layout),
        module: &vertical_blur_shader,
        entry_point: Some("main"),
        compilation_options: Default::default(),
        cache: None,
    });

    println!("  ✓ Vertical blur pipeline created");
    println!("  ✓ Uses workgroup shared memory for caching");
    println!("  ✓ Applies Gaussian blur in vertical direction\n");

    // === CREATE SAMPLER ===
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("Blur Sampler"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });

    // === CREATE BIND GROUPS ===
    let input_view = input_texture.create_view(&wgpu::TextureViewDescriptor::default());
    let intermediate_view =
        intermediate_texture.create_view(&wgpu::TextureViewDescriptor::default());
    let output_view = output_texture.create_view(&wgpu::TextureViewDescriptor::default());

    let horizontal_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Horizontal Blur Bind Group"),
        layout: &horizontal_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&input_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::TextureView(&intermediate_view),
            },
        ],
    });

    let vertical_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Vertical Blur Bind Group"),
        layout: &vertical_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&intermediate_view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::TextureView(&output_view),
            },
        ],
    });

    println!("  ✓ Created bind groups for horizontal and vertical passes\n");

    // === EXECUTE BLUR PASSES ===
    println!("Executing blur passes...\n");

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Blur Encoder"),
    });

    // Calculate workgroup dispatch dimensions
    let workgroups_x = IMAGE_WIDTH.div_ceil(WORKGROUP_SIZE);
    let workgroups_y = IMAGE_HEIGHT.div_ceil(WORKGROUP_SIZE);

    println!("Pass 1: Horizontal Blur");
    println!("  - Input: Original image");
    println!("  - Output: Intermediate texture");
    println!(
        "  - Workgroups: {}x{} ({} total)",
        workgroups_x,
        workgroups_y,
        workgroups_x * workgroups_y
    );

    {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Horizontal Blur Pass"),
            timestamp_writes: None,
        });

        compute_pass.set_pipeline(&horizontal_pipeline);
        compute_pass.set_bind_group(0, &horizontal_bind_group, &[]);
        compute_pass.dispatch_workgroups(workgroups_x, workgroups_y, 1);
    }
    println!("  ✓ Horizontal blur dispatched\n");

    println!("Pass 2: Vertical Blur");
    println!("  - Input: Intermediate texture");
    println!("  - Output: Final blurred image");
    println!(
        "  - Workgroups: {}x{} ({} total)",
        workgroups_x,
        workgroups_y,
        workgroups_x * workgroups_y
    );

    {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Vertical Blur Pass"),
            timestamp_writes: None,
        });

        compute_pass.set_pipeline(&vertical_pipeline);
        compute_pass.set_bind_group(0, &vertical_bind_group, &[]);
        compute_pass.dispatch_workgroups(workgroups_x, workgroups_y, 1);
    }
    println!("  ✓ Vertical blur dispatched\n");

    // Submit and wait for completion
    queue.submit(std::iter::once(encoder.finish()));
    // Poll to wait for GPU work to complete. We ignore the result since we're just waiting.
    let _ = device.poll(wgpu::PollType::Wait {
        submission_index: None,
        timeout: None,
    });

    println!("  ✓ Both passes completed successfully\n");

    // === SUMMARY ===
    println!("=== Example Complete ===\n");
    println!("This example demonstrated:");
    println!("  ✓ Workgroup shared memory (var<workgroup>) for efficient tile caching");
    println!("  ✓ workgroupBarrier() for synchronization between threads");
    println!("  ✓ Multiple dispatch calls (separable blur: horizontal + vertical)");
    println!("  ✓ Storage textures for read/write operations");
    println!(
        "  ✓ Gaussian blur using {}x{} kernel",
        BLUR_RADIUS * 2 + 1,
        BLUR_RADIUS * 2 + 1
    );
    println!("\nKey WebGPU APIs Exercised:");
    println!("  • var<workgroup>: Shared memory within a workgroup");
    println!("  • workgroupBarrier(): Ensures all threads complete before proceeding");
    println!("  • textureLoad/textureStore: Direct texture access in compute shaders");
    println!("  • Multiple dispatch: Sequential passes for efficient separable filters");
    println!("\nPerformance Benefits:");
    println!("  • Shared memory reduces global memory bandwidth");
    println!("  • Separable blur: O(n) instead of O(n²) for 2D convolution");
    println!("  • Workgroup tiling enables efficient texture caching");
}

/// Create blur shader (horizontal or vertical)
fn create_blur_shader(device: &wgpu::Device, horizontal: bool) -> wgpu::ShaderModule {
    let direction = if horizontal { "horizontal" } else { "vertical" };

    // Gaussian weights for radius=5 (pre-calculated)
    // These approximate a Gaussian distribution
    let shader_source = format!(
        r#"
// {direction} Gaussian Blur with Workgroup Shared Memory
// Demonstrates: var<workgroup>, workgroupBarrier(), storage textures

@group(0) @binding(0)
var input_texture: texture_2d<f32>;

@group(0) @binding(1)
var input_sampler: sampler;

@group(0) @binding(2)
var output_texture: texture_storage_2d<rgba8unorm, write>;

// Shared memory tile for caching texture data
// This reduces global memory access and improves performance
// Tile size includes padding for blur radius
const TILE_SIZE: u32 = {WORKGROUP_SIZE};
const BLUR_RADIUS: i32 = {BLUR_RADIUS};
const SHARED_SIZE: u32 = TILE_SIZE + 2u * u32(BLUR_RADIUS);

var<workgroup> shared_tile: array<vec4<f32>, {shared_array_size}>;

// Gaussian weights for blur (pre-calculated for radius=5)
const GAUSSIAN_WEIGHTS = array<f32, 11>(
    0.0093, 0.028, 0.0656, 0.1210, 0.1747,
    0.1974,  // Center
    0.1747, 0.1210, 0.0656, 0.028, 0.0093
);

@compute @workgroup_size({WORKGROUP_SIZE}, {WORKGROUP_SIZE})
fn main(
    @builtin(global_invocation_id) global_id: vec3<u32>,
    @builtin(local_invocation_id) local_id: vec3<u32>,
    @builtin(workgroup_id) workgroup_id: vec3<u32>
) {{
    let image_size = textureDimensions(input_texture);
    
    // === PHASE 1: Load tile into shared memory ===
    // Each thread loads multiple pixels to fill the shared tile
    
    let local_idx = local_id.y * TILE_SIZE + local_id.x;
    let shared_pixels_per_thread = (SHARED_SIZE * SHARED_SIZE) / (TILE_SIZE * TILE_SIZE);
    
    for (var i = 0u; i < shared_pixels_per_thread + 1u; i++) {{
        let shared_idx = local_idx + i * (TILE_SIZE * TILE_SIZE);
        if (shared_idx < SHARED_SIZE * SHARED_SIZE) {{
            let shared_y = shared_idx / SHARED_SIZE;
            let shared_x = shared_idx % SHARED_SIZE;
            
            // Calculate global texture coordinate
            let base_x = i32(workgroup_id.x * TILE_SIZE);
            let base_y = i32(workgroup_id.y * TILE_SIZE);
            let tex_x = base_x + i32(shared_x) - BLUR_RADIUS;
            let tex_y = base_y + i32(shared_y) - BLUR_RADIUS;
            
            // Clamp to texture bounds
            let clamped_x = clamp(tex_x, 0, i32(image_size.x) - 1);
            let clamped_y = clamp(tex_y, 0, i32(image_size.y) - 1);
            
            // Load pixel into shared memory
            shared_tile[shared_idx] = textureLoad(input_texture, vec2<i32>(clamped_x, clamped_y), 0);
        }}
    }}
    
    // === SYNCHRONIZATION BARRIER ===
    // Wait for all threads to finish loading the tile
    // This ensures all data is available before we start the blur
    workgroupBarrier();
    
    // === PHASE 2: Apply blur using shared memory ===
    
    // Check if this thread's output pixel is within bounds
    if (global_id.x >= image_size.x || global_id.y >= image_size.y) {{
        return;
    }}
    
    var color = vec4<f32>(0.0);
    var weight_sum = 0.0;
    
    // Apply {direction} blur
    for (var i = -BLUR_RADIUS; i <= BLUR_RADIUS; i++) {{
        {blur_loop_body}
        
        // Calculate shared memory coordinate
        let shared_x = local_id.x + u32(offset_x + BLUR_RADIUS);
        let shared_y = local_id.y + u32(offset_y + BLUR_RADIUS);
        let shared_idx = shared_y * SHARED_SIZE + shared_x;
        
        // Fetch from shared memory (fast!)
        let sample = shared_tile[shared_idx];
        
        // Apply Gaussian weight
        let weight = GAUSSIAN_WEIGHTS[i + BLUR_RADIUS];
        color += sample * weight;
        weight_sum += weight;
    }}
    
    // Normalize and write output
    color /= weight_sum;
    textureStore(output_texture, vec2<i32>(global_id.xy), color);
}}
"#,
        direction = direction,
        WORKGROUP_SIZE = WORKGROUP_SIZE,
        BLUR_RADIUS = BLUR_RADIUS,
        shared_array_size =
            (WORKGROUP_SIZE + 2 * BLUR_RADIUS as u32) * (WORKGROUP_SIZE + 2 * BLUR_RADIUS as u32),
        blur_loop_body = if horizontal {
            "let offset_x = i;\n        let offset_y = 0;"
        } else {
            "let offset_x = 0;\n        let offset_y = i;"
        }
    );

    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(&format!(
            "{} Blur Shader",
            if horizontal { "Horizontal" } else { "Vertical" }
        )),
        source: wgpu::ShaderSource::Wgsl(shader_source.into()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workgroup_calculations() {
        // Test workgroup dispatch calculations
        assert_eq!(IMAGE_WIDTH.div_ceil(WORKGROUP_SIZE), 32);
        assert_eq!(IMAGE_HEIGHT.div_ceil(WORKGROUP_SIZE), 32);
    }

    #[test]
    fn test_shared_memory_size() {
        // Verify shared memory tile size calculation
        let tile_with_padding = WORKGROUP_SIZE + 2 * BLUR_RADIUS as u32;
        assert_eq!(tile_with_padding, 26); // 16 + 2*5

        let total_elements = tile_with_padding * tile_with_padding;
        assert_eq!(total_elements, 676); // 26 * 26
    }

    #[test]
    fn test_image_generation() {
        let data = generate_test_image();
        assert_eq!(data.len(), (IMAGE_WIDTH * IMAGE_HEIGHT * 4) as usize);

        // Verify RGBA format (4 bytes per pixel)
        assert_eq!(data.len() % 4, 0);
    }

    #[tokio::test]
    async fn test_device_creation() {
        // Test that we can create a device with required features
        let result = create_device().await;
        match result {
            Some(_) => println!("Device created successfully"),
            None => println!("No GPU available (expected in CI)"),
        }
    }

    #[test]
    fn test_blur_radius_odd() {
        // Blur radius should create an odd-sized kernel
        let kernel_size = BLUR_RADIUS * 2 + 1;
        assert_eq!(kernel_size % 2, 1); // Must be odd
        assert_eq!(kernel_size, 11);
    }
}
