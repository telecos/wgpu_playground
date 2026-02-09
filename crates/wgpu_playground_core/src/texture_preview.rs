/// Texture preview rendering for the Texture Config panel
///
/// Provides live visualization of texture configurations:
/// - Loaded images: Shows the image as a textured quad
/// - Procedural textures: Generates and displays procedural patterns
use crate::api_coverage::{ApiCategory, ApiCoverageTracker};
use wgpu::util::DeviceExt;

/// Vertex structure for texture quad rendering
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct TextureVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

/// State for texture preview rendering
pub struct TexturePreviewState {
    /// The render pipeline for texture preview
    texture_pipeline: Option<wgpu::RenderPipeline>,
    /// Preview vertex buffer (quad)
    preview_vertex_buffer: Option<wgpu::Buffer>,
    /// Preview index buffer
    preview_index_buffer: Option<wgpu::Buffer>,
    /// Preview texture
    preview_texture: Option<wgpu::Texture>,
    /// Preview texture view
    preview_texture_view: Option<wgpu::TextureView>,
    /// Sampler for texture preview
    sampler: Option<wgpu::Sampler>,
    /// Bind group for texture preview
    texture_bind_group: Option<wgpu::BindGroup>,
    /// Bind group layout
    bind_group_layout: Option<wgpu::BindGroupLayout>,
    /// Render texture for display
    render_texture: Option<wgpu::Texture>,
    /// Render texture view
    render_texture_view: Option<wgpu::TextureView>,
    /// Texture ID for egui display
    texture_id: Option<egui::TextureId>,
    /// Preview canvas size
    width: u32,
    height: u32,
}

impl TexturePreviewState {
    pub fn new() -> Self {
        Self {
            texture_pipeline: None,
            preview_vertex_buffer: None,
            preview_index_buffer: None,
            preview_texture: None,
            preview_texture_view: None,
            sampler: None,
            texture_bind_group: None,
            bind_group_layout: None,
            render_texture: None,
            render_texture_view: None,
            texture_id: None,
            width: 256,
            height: 256,
        }
    }

    /// Initialize rendering resources
    pub fn initialize(&mut self, device: &wgpu::Device) {
        self.init_render_texture(device);
        self.init_texture_pipeline(device);
        self.init_quad_geometry(device);
        self.init_sampler(device);
    }

    /// Initialize render texture
    fn init_render_texture(&mut self, device: &wgpu::Device) {
        let tracker = ApiCoverageTracker::global();
        tracker.record(ApiCategory::Texture, "create_texture");

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Texture Preview Render Texture"),
            size: wgpu::Extent3d {
                width: self.width,
                height: self.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        tracker.record(ApiCategory::Texture, "create_view");
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        self.render_texture = Some(texture);
        self.render_texture_view = Some(view);
    }

    /// Initialize sampler
    fn init_sampler(&mut self, device: &wgpu::Device) {
        let tracker = ApiCoverageTracker::global();
        tracker.record(ApiCategory::Sampler, "create_sampler");

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Texture Preview Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Linear,
            ..Default::default()
        });

        self.sampler = Some(sampler);
    }

    /// Initialize quad geometry (vertex and index buffers)
    fn init_quad_geometry(&mut self, device: &wgpu::Device) {
        let tracker = ApiCoverageTracker::global();

        // Create vertex buffer for textured quad
        let vertices = [
            TextureVertex {
                position: [-0.8, 0.8],
                tex_coords: [0.0, 0.0],
            },
            TextureVertex {
                position: [0.8, 0.8],
                tex_coords: [1.0, 0.0],
            },
            TextureVertex {
                position: [0.8, -0.8],
                tex_coords: [1.0, 1.0],
            },
            TextureVertex {
                position: [-0.8, -0.8],
                tex_coords: [0.0, 1.0],
            },
        ];

        tracker.record(ApiCategory::Buffer, "create_buffer");
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Texture Preview Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Create index buffer
        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        tracker.record(ApiCategory::Buffer, "create_buffer");
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Texture Preview Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        self.preview_vertex_buffer = Some(vertex_buffer);
        self.preview_index_buffer = Some(index_buffer);
    }

    /// Initialize texture preview pipeline
    fn init_texture_pipeline(&mut self, device: &wgpu::Device) {
        let tracker = ApiCoverageTracker::global();

        // Create shader for texture preview
        let shader_source = r#"
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4<f32>(input.position, 0.0, 1.0);
    output.tex_coords = input.tex_coords;
    return output;
}

@group(0) @binding(0) var tex: texture_2d<f32>;
@group(0) @binding(1) var tex_sampler: sampler;

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(tex, tex_sampler, input.tex_coords);
}
"#;

        tracker.record(ApiCategory::Shader, "create_shader_module");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Texture Preview Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // Create bind group layout
        tracker.record(ApiCategory::BindGroup, "create_bind_group_layout");
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Texture Preview Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        // Create pipeline layout
        tracker.record(ApiCategory::PipelineLayout, "create_pipeline_layout");
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Texture Preview Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            immediate_size: 0,
        });

        // Create render pipeline
        tracker.record(ApiCategory::RenderPipeline, "create_render_pipeline");
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Texture Preview Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<TextureVertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x2,
                            offset: 0,
                            shader_location: 0,
                        },
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x2,
                            offset: 8,
                            shader_location: 1,
                        },
                    ],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        self.bind_group_layout = Some(bind_group_layout);
        self.texture_pipeline = Some(pipeline);
    }

    /// Update preview texture from loaded image data
    pub fn update_from_image_data(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        image_data: &[u8],
        width: u32,
        height: u32,
    ) {
        let tracker = ApiCoverageTracker::global();

        // Create preview texture from image data
        tracker.record(ApiCategory::Texture, "create_texture");
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Texture Preview Source"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        // Write image data to texture
        tracker.record(ApiCategory::Queue, "write_texture");
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            image_data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );

        tracker.record(ApiCategory::Texture, "create_view");
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        self.preview_texture = Some(texture);
        self.preview_texture_view = Some(view);

        // Update bind group
        self.update_bind_group(device);
    }

    /// Generate and update procedural texture
    pub fn generate_procedural_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        width: u32,
        height: u32,
    ) {
        // Generate a simple procedural checkerboard pattern
        let mut data = vec![0u8; (width * height * 4) as usize];

        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;
                let checker = ((x / 32) + (y / 32)) % 2;

                if checker == 0 {
                    data[idx] = 200; // R
                    data[idx + 1] = 200; // G
                    data[idx + 2] = 200; // B
                    data[idx + 3] = 255; // A
                } else {
                    data[idx] = 100; // R
                    data[idx + 1] = 100; // G
                    data[idx + 2] = 100; // B
                    data[idx + 3] = 255; // A
                }
            }
        }

        self.update_from_image_data(device, queue, &data, width, height);
    }

    /// Update bind group with current texture
    fn update_bind_group(&mut self, device: &wgpu::Device) {
        let tracker = ApiCoverageTracker::global();

        if let (Some(texture_view), Some(sampler), Some(layout)) = (
            &self.preview_texture_view,
            &self.sampler,
            &self.bind_group_layout,
        ) {
            tracker.record(ApiCategory::BindGroup, "create_bind_group");
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Texture Preview Bind Group"),
                layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(sampler),
                    },
                ],
            });

            self.texture_bind_group = Some(bind_group);
        }
    }

    /// Render texture preview
    pub fn render(&self, device: &wgpu::Device, queue: &wgpu::Queue) -> Option<&wgpu::TextureView> {
        let tracker = ApiCoverageTracker::global();

        // Only render if we have a texture to preview
        if self.preview_texture.is_none() {
            return self.render_texture_view.as_ref();
        }

        // Create command encoder
        tracker.record(ApiCategory::CommandEncoder, "create_command_encoder");
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Texture Preview Encoder"),
        });

        if let Some(view) = &self.render_texture_view {
            {
                tracker.record(ApiCategory::RenderPass, "begin_render_pass");
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Texture Preview Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.05,
                                g: 0.05,
                                b: 0.1,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });

                // Render textured quad
                if let (Some(pipeline), Some(vertex_buffer), Some(index_buffer), Some(bind_group)) = (
                    &self.texture_pipeline,
                    &self.preview_vertex_buffer,
                    &self.preview_index_buffer,
                    &self.texture_bind_group,
                ) {
                    tracker.record(ApiCategory::RenderPass, "set_pipeline");
                    render_pass.set_pipeline(pipeline);
                    tracker.record(ApiCategory::RenderPass, "set_bind_group");
                    render_pass.set_bind_group(0, bind_group, &[]);
                    tracker.record(ApiCategory::RenderPass, "set_vertex_buffer");
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    tracker.record(ApiCategory::RenderPass, "set_index_buffer");
                    render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                    tracker.record(ApiCategory::RenderPass, "draw_indexed");
                    render_pass.draw_indexed(0..6, 0, 0..1);
                }
            }

            tracker.record(ApiCategory::Queue, "submit");
            queue.submit(Some(encoder.finish()));
        }

        self.render_texture_view.as_ref()
    }

    /// Get or register texture ID for egui
    ///
    /// Note: This method is only available when building for native targets
    /// due to wgpu version incompatibility with egui-wgpu on WASM.
    /// We use unsafe transmute to convert wgpu 28 types to wgpu 27 types
    /// as a temporary workaround until egui-wgpu supports wgpu 28.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_texture_id(
        &mut self,
        device: &wgpu::Device,
        renderer: &mut egui_wgpu::Renderer,
    ) -> Option<egui::TextureId> {
        if self.texture_id.is_none() {
            if let Some(view) = &self.render_texture_view {
                // SAFETY: wgpu 28 and wgpu 27 have the same memory layout for these types
                // This is a temporary workaround until egui-wgpu supports wgpu 28
                let device_27: &egui_wgpu::wgpu::Device = unsafe { std::mem::transmute(device) };
                let view_27: &egui_wgpu::wgpu::TextureView = unsafe { std::mem::transmute(view) };
                let id = renderer.register_native_texture(
                    device_27,
                    view_27,
                    egui_wgpu::wgpu::FilterMode::Linear,
                );
                self.texture_id = Some(id);
            }
        }
        self.texture_id
    }

    /// Get preview canvas size
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Check if texture preview is ready
    pub fn has_texture(&self) -> bool {
        self.preview_texture.is_some()
    }
}

impl Default for TexturePreviewState {
    fn default() -> Self {
        Self::new()
    }
}
