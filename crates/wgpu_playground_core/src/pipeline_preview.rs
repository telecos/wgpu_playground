/// Pipeline preview rendering for the Render Pipeline panel
///
/// Provides live visualization of pipeline configurations:
/// - Wireframe mode for topology changes
/// - Culling visualization
/// - Blend mode demonstration
/// - Depth testing effect
use crate::api_coverage::{ApiCategory, ApiCoverageTracker};
use crate::render_pipeline::{
    BlendState, CompareFunction, CullMode, DepthStencilState, FrontFace, MultisampleState,
    PrimitiveState, PrimitiveTopology,
};
use wgpu::util::DeviceExt;

/// Vertex structure for preview rendering
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct PreviewVertex {
    position: [f32; 3],
    color: [f32; 3],
}

/// State for pipeline preview rendering
pub struct RenderPipelinePreviewState {
    /// The render pipeline for preview
    pipeline: Option<wgpu::RenderPipeline>,
    /// Bind group layout for uniforms
    bind_group_layout: Option<wgpu::BindGroupLayout>,
    /// Preview vertex buffer (cube mesh)
    vertex_buffer: Option<wgpu::Buffer>,
    /// Preview index buffer
    index_buffer: Option<wgpu::Buffer>,
    /// Number of indices to draw
    index_count: u32,
    /// Render texture for preview
    render_texture: Option<wgpu::Texture>,
    /// Render texture view
    render_texture_view: Option<wgpu::TextureView>,
    /// Depth texture for depth testing
    depth_texture: Option<wgpu::Texture>,
    /// Depth texture view
    depth_texture_view: Option<wgpu::TextureView>,
    /// Texture ID for egui display
    #[allow(dead_code)]
    texture_id: Option<egui::TextureId>,
    /// Animation time for rotation
    time: f32,
    /// Preview canvas size
    width: u32,
    height: u32,
}

impl Default for RenderPipelinePreviewState {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderPipelinePreviewState {
    pub fn new() -> Self {
        Self {
            pipeline: None,
            bind_group_layout: None,
            vertex_buffer: None,
            index_buffer: None,
            index_count: 0,
            render_texture: None,
            render_texture_view: None,
            depth_texture: None,
            depth_texture_view: None,
            texture_id: None,
            time: 0.0,
            width: 256,
            height: 256,
        }
    }

    /// Initialize rendering resources
    pub fn initialize(&mut self, device: &wgpu::Device) {
        self.init_render_texture(device);
        self.init_depth_texture(device);
        self.init_geometry(device);
    }

    /// Initialize render texture
    fn init_render_texture(&mut self, device: &wgpu::Device) {
        let tracker = ApiCoverageTracker::global();
        tracker.record(ApiCategory::Texture, "create_texture");

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Pipeline Preview Texture"),
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

    /// Initialize depth texture
    fn init_depth_texture(&mut self, device: &wgpu::Device) {
        let tracker = ApiCoverageTracker::global();
        tracker.record(ApiCategory::Texture, "create_texture");

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Pipeline Preview Depth Texture"),
            size: wgpu::Extent3d {
                width: self.width,
                height: self.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth24Plus,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        tracker.record(ApiCategory::Texture, "create_view");
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        self.depth_texture = Some(texture);
        self.depth_texture_view = Some(view);
    }

    /// Initialize cube geometry for preview
    fn init_geometry(&mut self, device: &wgpu::Device) {
        let tracker = ApiCoverageTracker::global();

        // Create a cube with colored faces
        // Each face has a different color to visualize culling and blending
        let vertices = vec![
            // Front face (red)
            PreviewVertex {
                position: [-0.5, -0.5, 0.5],
                color: [1.0, 0.3, 0.3],
            },
            PreviewVertex {
                position: [0.5, -0.5, 0.5],
                color: [1.0, 0.3, 0.3],
            },
            PreviewVertex {
                position: [0.5, 0.5, 0.5],
                color: [1.0, 0.3, 0.3],
            },
            PreviewVertex {
                position: [-0.5, 0.5, 0.5],
                color: [1.0, 0.3, 0.3],
            },
            // Back face (green)
            PreviewVertex {
                position: [0.5, -0.5, -0.5],
                color: [0.3, 1.0, 0.3],
            },
            PreviewVertex {
                position: [-0.5, -0.5, -0.5],
                color: [0.3, 1.0, 0.3],
            },
            PreviewVertex {
                position: [-0.5, 0.5, -0.5],
                color: [0.3, 1.0, 0.3],
            },
            PreviewVertex {
                position: [0.5, 0.5, -0.5],
                color: [0.3, 1.0, 0.3],
            },
            // Top face (blue)
            PreviewVertex {
                position: [-0.5, 0.5, 0.5],
                color: [0.3, 0.3, 1.0],
            },
            PreviewVertex {
                position: [0.5, 0.5, 0.5],
                color: [0.3, 0.3, 1.0],
            },
            PreviewVertex {
                position: [0.5, 0.5, -0.5],
                color: [0.3, 0.3, 1.0],
            },
            PreviewVertex {
                position: [-0.5, 0.5, -0.5],
                color: [0.3, 0.3, 1.0],
            },
            // Bottom face (yellow)
            PreviewVertex {
                position: [-0.5, -0.5, -0.5],
                color: [1.0, 1.0, 0.3],
            },
            PreviewVertex {
                position: [0.5, -0.5, -0.5],
                color: [1.0, 1.0, 0.3],
            },
            PreviewVertex {
                position: [0.5, -0.5, 0.5],
                color: [1.0, 1.0, 0.3],
            },
            PreviewVertex {
                position: [-0.5, -0.5, 0.5],
                color: [1.0, 1.0, 0.3],
            },
            // Right face (magenta)
            PreviewVertex {
                position: [0.5, -0.5, 0.5],
                color: [1.0, 0.3, 1.0],
            },
            PreviewVertex {
                position: [0.5, -0.5, -0.5],
                color: [1.0, 0.3, 1.0],
            },
            PreviewVertex {
                position: [0.5, 0.5, -0.5],
                color: [1.0, 0.3, 1.0],
            },
            PreviewVertex {
                position: [0.5, 0.5, 0.5],
                color: [1.0, 0.3, 1.0],
            },
            // Left face (cyan)
            PreviewVertex {
                position: [-0.5, -0.5, -0.5],
                color: [0.3, 1.0, 1.0],
            },
            PreviewVertex {
                position: [-0.5, -0.5, 0.5],
                color: [0.3, 1.0, 1.0],
            },
            PreviewVertex {
                position: [-0.5, 0.5, 0.5],
                color: [0.3, 1.0, 1.0],
            },
            PreviewVertex {
                position: [-0.5, 0.5, -0.5],
                color: [0.3, 1.0, 1.0],
            },
        ];

        let indices: Vec<u16> = vec![
            // Front face
            0, 1, 2, 0, 2, 3, // Back face
            4, 5, 6, 4, 6, 7, // Top face
            8, 9, 10, 8, 10, 11, // Bottom face
            12, 13, 14, 12, 14, 15, // Right face
            16, 17, 18, 16, 18, 19, // Left face
            20, 21, 22, 20, 22, 23,
        ];

        self.index_count = indices.len() as u32;

        tracker.record(ApiCategory::Buffer, "create_buffer");
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Pipeline Preview Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        tracker.record(ApiCategory::Buffer, "create_buffer");
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Pipeline Preview Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        self.vertex_buffer = Some(vertex_buffer);
        self.index_buffer = Some(index_buffer);
    }

    /// Create or recreate the pipeline with the specified configuration
    pub fn update_pipeline(
        &mut self,
        device: &wgpu::Device,
        primitive: &PrimitiveState,
        depth_stencil: Option<&DepthStencilState>,
        blend: Option<&BlendState>,
        multisample: &MultisampleState,
    ) {
        // Create shader for pipeline preview
        let shader_source = r#"
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

struct Uniforms {
    mvp: mat4x4<f32>,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = uniforms.mvp * vec4<f32>(input.position, 1.0);
    output.color = input.color;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(input.color, 1.0);
}
"#;

        let tracker = ApiCoverageTracker::global();

        tracker.record(ApiCategory::Shader, "create_shader_module");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Pipeline Preview Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // Create bind group layout for uniforms
        tracker.record(ApiCategory::BindGroup, "create_bind_group_layout");
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Pipeline Preview Bind Group Layout"),
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

        tracker.record(ApiCategory::PipelineLayout, "create_pipeline_layout");
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Preview Layout"),
            bind_group_layouts: &[&bind_group_layout],
            immediate_size: 0,
        });

        // Convert our custom types to wgpu types
        let topology = match primitive.topology {
            PrimitiveTopology::PointList => wgpu::PrimitiveTopology::PointList,
            PrimitiveTopology::LineList => wgpu::PrimitiveTopology::LineList,
            PrimitiveTopology::LineStrip => wgpu::PrimitiveTopology::LineStrip,
            PrimitiveTopology::TriangleList => wgpu::PrimitiveTopology::TriangleList,
            PrimitiveTopology::TriangleStrip => wgpu::PrimitiveTopology::TriangleStrip,
        };

        let cull_mode = match primitive.cull_mode {
            CullMode::None => None,
            CullMode::Front => Some(wgpu::Face::Front),
            CullMode::Back => Some(wgpu::Face::Back),
        };

        let front_face = match primitive.front_face {
            FrontFace::Ccw => wgpu::FrontFace::Ccw,
            FrontFace::Cw => wgpu::FrontFace::Cw,
        };

        // Build primitive state
        let primitive_state = wgpu::PrimitiveState {
            topology,
            strip_index_format: None,
            front_face,
            cull_mode,
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        };

        // Build depth stencil state
        let depth_stencil_state = depth_stencil.map(|ds| {
            let compare = match ds.depth_compare {
                CompareFunction::Never => wgpu::CompareFunction::Never,
                CompareFunction::Less => wgpu::CompareFunction::Less,
                CompareFunction::Equal => wgpu::CompareFunction::Equal,
                CompareFunction::LessEqual => wgpu::CompareFunction::LessEqual,
                CompareFunction::Greater => wgpu::CompareFunction::Greater,
                CompareFunction::NotEqual => wgpu::CompareFunction::NotEqual,
                CompareFunction::GreaterEqual => wgpu::CompareFunction::GreaterEqual,
                CompareFunction::Always => wgpu::CompareFunction::Always,
            };

            wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: ds.depth_write_enabled,
                depth_compare: compare,
                stencil: wgpu::StencilState {
                    front: wgpu::StencilFaceState::IGNORE,
                    back: wgpu::StencilFaceState::IGNORE,
                    read_mask: 0,
                    write_mask: 0,
                },
                bias: wgpu::DepthBiasState::default(),
            }
        });

        // Build blend state
        let blend_state = blend.map(|b| {
            let convert_blend_factor = |factor| match factor {
                crate::render_pipeline::BlendFactor::Zero => wgpu::BlendFactor::Zero,
                crate::render_pipeline::BlendFactor::One => wgpu::BlendFactor::One,
                crate::render_pipeline::BlendFactor::Src => wgpu::BlendFactor::Src,
                crate::render_pipeline::BlendFactor::OneMinusSrc => wgpu::BlendFactor::OneMinusSrc,
                crate::render_pipeline::BlendFactor::SrcAlpha => wgpu::BlendFactor::SrcAlpha,
                crate::render_pipeline::BlendFactor::OneMinusSrcAlpha => {
                    wgpu::BlendFactor::OneMinusSrcAlpha
                }
                crate::render_pipeline::BlendFactor::Dst => wgpu::BlendFactor::Dst,
                crate::render_pipeline::BlendFactor::OneMinusDst => wgpu::BlendFactor::OneMinusDst,
                crate::render_pipeline::BlendFactor::DstAlpha => wgpu::BlendFactor::DstAlpha,
                crate::render_pipeline::BlendFactor::OneMinusDstAlpha => {
                    wgpu::BlendFactor::OneMinusDstAlpha
                }
                crate::render_pipeline::BlendFactor::SrcAlphaSaturated => {
                    wgpu::BlendFactor::SrcAlphaSaturated
                }
                crate::render_pipeline::BlendFactor::Constant => wgpu::BlendFactor::Constant,
                crate::render_pipeline::BlendFactor::OneMinusConstant => {
                    wgpu::BlendFactor::OneMinusConstant
                }
            };

            let convert_blend_operation = |op| match op {
                crate::render_pipeline::BlendOperation::Add => wgpu::BlendOperation::Add,
                crate::render_pipeline::BlendOperation::Subtract => wgpu::BlendOperation::Subtract,
                crate::render_pipeline::BlendOperation::ReverseSubtract => {
                    wgpu::BlendOperation::ReverseSubtract
                }
                crate::render_pipeline::BlendOperation::Min => wgpu::BlendOperation::Min,
                crate::render_pipeline::BlendOperation::Max => wgpu::BlendOperation::Max,
            };

            wgpu::BlendState {
                color: wgpu::BlendComponent {
                    src_factor: convert_blend_factor(b.color.src_factor),
                    dst_factor: convert_blend_factor(b.color.dst_factor),
                    operation: convert_blend_operation(b.color.operation),
                },
                alpha: wgpu::BlendComponent {
                    src_factor: convert_blend_factor(b.alpha.src_factor),
                    dst_factor: convert_blend_factor(b.alpha.dst_factor),
                    operation: convert_blend_operation(b.alpha.operation),
                },
            }
        });

        // Build multisample state
        let multisample_state = wgpu::MultisampleState {
            count: multisample.count,
            mask: !0,
            alpha_to_coverage_enabled: multisample.alpha_to_coverage_enabled,
        };

        tracker.record(ApiCategory::RenderPipeline, "create_render_pipeline");
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Pipeline Preview Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<PreviewVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x3,
                            offset: 0,
                            shader_location: 0,
                        },
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x3,
                            offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                            shader_location: 1,
                        },
                    ],
                }],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    blend: blend_state,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: primitive_state,
            depth_stencil: depth_stencil_state,
            multisample: multisample_state,
            multiview_mask: None,
            cache: None,
        });

        self.pipeline = Some(pipeline);
        self.bind_group_layout = Some(bind_group_layout);
    }

    /// Render the preview with the current pipeline configuration
    pub fn render(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        delta_time: f32,
    ) -> Option<&wgpu::TextureView> {
        let tracker = ApiCoverageTracker::global();

        self.time += delta_time;

        // Create MVP matrix for rotating cube
        let aspect = self.width as f32 / self.height as f32;
        let projection = perspective_matrix(45.0_f32.to_radians(), aspect, 0.1, 100.0);
        let view = view_matrix([0.0, 0.0, 3.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        let model = rotation_matrix_y(self.time) * rotation_matrix_x(self.time * 0.5);
        let mvp = projection * view * model;

        // Create uniform buffer with MVP matrix
        tracker.record(ApiCategory::Buffer, "create_buffer");
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Pipeline Preview Uniform Buffer"),
            contents: bytemuck::cast_slice(mvp.as_slice()),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        // Create bind group using stored bind group layout
        if let Some(bind_group_layout) = &self.bind_group_layout {
            tracker.record(ApiCategory::BindGroup, "create_bind_group");
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Pipeline Preview Bind Group"),
                layout: bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }],
            });

            // Render to the preview texture
            tracker.record(ApiCategory::CommandEncoder, "create_command_encoder");
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Pipeline Preview Encoder"),
            });

            if let (Some(view), Some(depth_view)) =
                (&self.render_texture_view, &self.depth_texture_view)
            {
                {
                    tracker.record(ApiCategory::RenderPass, "begin_render_pass");
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Pipeline Preview Render Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.1,
                                    g: 0.1,
                                    b: 0.15,
                                    a: 1.0,
                                }),
                                store: wgpu::StoreOp::Store,
                            },
                            depth_slice: None,
                        })],
                        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                            view: depth_view,
                            depth_ops: Some(wgpu::Operations {
                                load: wgpu::LoadOp::Clear(1.0),
                                store: wgpu::StoreOp::Store,
                            }),
                            stencil_ops: None,
                        }),
                        timestamp_writes: None,
                        occlusion_query_set: None,
                        multiview_mask: None,
                    });

                    // Render the cube
                    if let (Some(pipeline), Some(vertex_buffer), Some(index_buffer)) =
                        (&self.pipeline, &self.vertex_buffer, &self.index_buffer)
                    {
                        tracker.record(ApiCategory::RenderPass, "set_pipeline");
                        render_pass.set_pipeline(pipeline);
                        tracker.record(ApiCategory::RenderPass, "set_bind_group");
                        render_pass.set_bind_group(0, &bind_group, &[]);
                        tracker.record(ApiCategory::RenderPass, "set_vertex_buffer");
                        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                        tracker.record(ApiCategory::RenderPass, "set_index_buffer");
                        render_pass
                            .set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                        tracker.record(ApiCategory::RenderPass, "draw_indexed");
                        render_pass.draw_indexed(0..self.index_count, 0, 0..1);
                    }
                }

                tracker.record(ApiCategory::Queue, "submit");
                queue.submit(Some(encoder.finish()));
            }
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

    /// Get preview texture size
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

// Matrix helper functions
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Matrix4 {
    data: [[f32; 4]; 4],
}

impl Matrix4 {
    fn as_slice(&self) -> &[f32] {
        bytemuck::cast_slice(&self.data)
    }
}

impl std::ops::Mul for Matrix4 {
    type Output = Matrix4;

    #[allow(clippy::needless_range_loop)]
    fn mul(self, rhs: Matrix4) -> Matrix4 {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i][j] += self.data[i][k] * rhs.data[k][j];
                }
            }
        }
        Matrix4 { data: result }
    }
}

fn perspective_matrix(fovy: f32, aspect: f32, near: f32, far: f32) -> Matrix4 {
    let f = 1.0 / (fovy / 2.0).tan();
    Matrix4 {
        data: [
            [f / aspect, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (far + near) / (near - far), -1.0],
            [0.0, 0.0, (2.0 * far * near) / (near - far), 0.0],
        ],
    }
}

fn view_matrix(eye: [f32; 3], center: [f32; 3], up: [f32; 3]) -> Matrix4 {
    let f = normalize([center[0] - eye[0], center[1] - eye[1], center[2] - eye[2]]);
    let s = normalize(cross(f, up));
    let u = cross(s, f);

    Matrix4 {
        data: [
            [s[0], u[0], -f[0], 0.0],
            [s[1], u[1], -f[1], 0.0],
            [s[2], u[2], -f[2], 0.0],
            [-dot(s, eye), -dot(u, eye), dot(f, eye), 1.0],
        ],
    }
}

fn rotation_matrix_y(angle: f32) -> Matrix4 {
    let c = angle.cos();
    let s = angle.sin();
    Matrix4 {
        data: [
            [c, 0.0, s, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-s, 0.0, c, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}

fn rotation_matrix_x(angle: f32) -> Matrix4 {
    let c = angle.cos();
    let s = angle.sin();
    Matrix4 {
        data: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, c, -s, 0.0],
            [0.0, s, c, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}

fn normalize(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len < f32::EPSILON {
        [0.0, 0.0, 1.0] // Return default up vector for degenerate case
    } else {
        [v[0] / len, v[1] / len, v[2] / len]
    }
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
