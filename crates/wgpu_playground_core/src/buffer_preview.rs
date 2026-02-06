/// Buffer preview rendering for the Buffer Config panel
/// 
/// Provides live visualization of buffer configurations:
/// - Vertex buffers: Shows a simple mesh preview
/// - Uniform buffers: Shows animated values
use crate::buffer::BufferUsages;
use wgpu::util::DeviceExt;

/// Vertex structure for preview rendering
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct PreviewVertex {
    position: [f32; 2],
    color: [f32; 3],
}

/// Uniform structure for preview rendering
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct PreviewUniforms {
    time: f32,
    _padding: [f32; 3], // Padding to align to 16 bytes
}

/// State for buffer preview rendering
pub struct BufferPreviewState {
    /// The render pipeline for vertex buffer preview
    vertex_pipeline: Option<wgpu::RenderPipeline>,
    /// The render pipeline for uniform buffer preview
    uniform_pipeline: Option<wgpu::RenderPipeline>,
    /// Preview vertex buffer
    preview_vertex_buffer: Option<wgpu::Buffer>,
    /// Preview uniform buffer
    preview_uniform_buffer: Option<wgpu::Buffer>,
    /// Bind group for uniform preview
    uniform_bind_group: Option<wgpu::BindGroup>,
    /// Render texture for preview
    render_texture: Option<wgpu::Texture>,
    /// Render texture view
    render_texture_view: Option<wgpu::TextureView>,
    /// Texture ID for egui display
    texture_id: Option<egui::TextureId>,
    /// Animation time
    time: f32,
    /// Preview canvas size
    width: u32,
    height: u32,
}

impl BufferPreviewState {
    pub fn new() -> Self {
        Self {
            vertex_pipeline: None,
            uniform_pipeline: None,
            preview_vertex_buffer: None,
            preview_uniform_buffer: None,
            uniform_bind_group: None,
            render_texture: None,
            render_texture_view: None,
            texture_id: None,
            time: 0.0,
            width: 256,
            height: 256,
        }
    }

    /// Initialize rendering resources
    pub fn initialize(&mut self, device: &wgpu::Device) {
        self.init_render_texture(device);
        self.init_vertex_preview(device);
        self.init_uniform_preview(device);
    }

    /// Initialize render texture
    fn init_render_texture(&mut self, device: &wgpu::Device) {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Buffer Preview Texture"),
            size: wgpu::Extent3d {
                width: self.width,
                height: self.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        self.render_texture = Some(texture);
        self.render_texture_view = Some(view);
    }

    /// Initialize vertex buffer preview resources
    fn init_vertex_preview(&mut self, device: &wgpu::Device) {
        // Create shader for vertex preview
        let shader_source = r#"
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>,
) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4<f32>(position, 0.0, 1.0);
    output.color = color;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(input.color, 1.0);
}
"#;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Buffer Preview Vertex Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // Create vertex buffer for triangle
        let vertices = [
            PreviewVertex { position: [0.0, 0.5], color: [1.0, 0.5, 0.5] },
            PreviewVertex { position: [-0.5, -0.5], color: [0.5, 1.0, 0.5] },
            PreviewVertex { position: [0.5, -0.5], color: [0.5, 0.5, 1.0] },
        ];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Preview Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        // Create render pipeline
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Vertex Preview Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Vertex Preview Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<PreviewVertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x2,
                            offset: 0,
                            shader_location: 0,
                        },
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x3,
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
            multiview: None,
            cache: None,
        });

        self.preview_vertex_buffer = Some(vertex_buffer);
        self.vertex_pipeline = Some(pipeline);
    }

    /// Initialize uniform buffer preview resources
    fn init_uniform_preview(&mut self, device: &wgpu::Device) {
        // Create shader for uniform preview
        let shader_source = r#"
struct Uniforms {
    time: f32,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var output: VertexOutput;
    
    // Create a quad
    let x = f32((vertex_index & 1u) * 2u) - 1.0;
    let y = f32((vertex_index & 2u)) - 1.0;
    
    output.position = vec4<f32>(x, y, 0.0, 1.0);
    
    // Color based on time
    let t = uniforms.time;
    output.color = vec3<f32>(
        0.5 + 0.5 * sin(t),
        0.5 + 0.5 * sin(t + 2.094),
        0.5 + 0.5 * sin(t + 4.189)
    );
    
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(input.color, 1.0);
}
"#;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Buffer Preview Uniform Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // Create uniform buffer
        let uniforms = PreviewUniforms {
            time: 0.0,
            _padding: [0.0; 3],
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Preview Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Preview Uniform Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Preview Uniform Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // Create pipeline
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Uniform Preview Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Uniform Preview Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
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
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        self.preview_uniform_buffer = Some(uniform_buffer);
        self.uniform_pipeline = Some(pipeline);
        self.uniform_bind_group = Some(bind_group);
    }

    /// Render preview based on buffer usage
    pub fn render(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        usage: BufferUsages,
        delta_time: f32,
    ) -> Option<&wgpu::TextureView> {
        // Update animation time
        self.time += delta_time;

        // Create command encoder
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Buffer Preview Encoder"),
        });

        // Determine which preview to render
        let is_vertex = usage.contains(BufferUsages::VERTEX);
        let is_uniform = usage.contains(BufferUsages::UNIFORM);

        if let Some(view) = &self.render_texture_view {
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Buffer Preview Render Pass"),
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
                });

                if is_vertex {
                    // Render vertex buffer preview
                    if let (Some(pipeline), Some(vertex_buffer)) = (&self.vertex_pipeline, &self.preview_vertex_buffer) {
                        render_pass.set_pipeline(pipeline);
                        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                        render_pass.draw(0..3, 0..1);
                    }
                } else if is_uniform {
                    // Update uniform buffer
                    if let Some(uniform_buffer) = &self.preview_uniform_buffer {
                        let uniforms = PreviewUniforms {
                            time: self.time,
                            _padding: [0.0; 3],
                        };
                        queue.write_buffer(uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
                    }

                    // Render uniform buffer preview
                    if let (Some(pipeline), Some(bind_group)) = (&self.uniform_pipeline, &self.uniform_bind_group) {
                        render_pass.set_pipeline(pipeline);
                        render_pass.set_bind_group(0, bind_group, &[]);
                        render_pass.draw(0..4, 0..1);
                    }
                }
            }

            queue.submit(Some(encoder.finish()));
        }

        self.render_texture_view.as_ref()
    }

    /// Get or register texture ID for egui
    pub fn get_texture_id(
        &mut self,
        device: &wgpu::Device,
        renderer: &mut egui_wgpu::Renderer,
    ) -> Option<egui::TextureId> {
        if self.texture_id.is_none() {
            if let Some(view) = &self.render_texture_view {
                let id = renderer.register_native_texture(
                    device,
                    view,
                    wgpu::FilterMode::Linear,
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
}

impl Default for BufferPreviewState {
    fn default() -> Self {
        Self::new()
    }
}
