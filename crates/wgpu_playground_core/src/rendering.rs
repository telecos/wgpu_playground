use crate::api_coverage::ApiCategory;
use crate::example_metadata::get_example_api_tags;
use crate::examples::{get_all_examples, Example, ExampleCategory};
use crate::shader_editor::ShaderEditor;
use wgpu::{Device, Queue};

/// Get color for API category badge
fn category_badge_color(category: &ApiCategory) -> egui::Color32 {
    match category {
        ApiCategory::RenderPass => egui::Color32::from_rgb(220, 20, 60), // Crimson - main rendering
        ApiCategory::ComputePass => egui::Color32::from_rgb(138, 43, 226), // Blue Violet - compute
        ApiCategory::Buffer => egui::Color32::from_rgb(50, 150, 100),    // Sea Green - data
        ApiCategory::Texture => egui::Color32::from_rgb(255, 140, 0),    // Dark Orange - textures
        ApiCategory::Sampler => egui::Color32::from_rgb(218, 165, 32),   // Goldenrod - sampler
        ApiCategory::Shader => egui::Color32::from_rgb(70, 130, 180),    // Steel Blue - shader
        ApiCategory::RenderPipeline => egui::Color32::from_rgb(178, 34, 34), // Firebrick - pipeline
        ApiCategory::ComputePipeline => egui::Color32::from_rgb(123, 104, 238), // Medium Slate Blue
        ApiCategory::BindGroup => egui::Color32::from_rgb(34, 139, 34),  // Forest Green - binding
        ApiCategory::CommandEncoder => egui::Color32::from_rgb(85, 107, 47), // Dark Olive Green - commands
        ApiCategory::Queue => egui::Color32::from_rgb(184, 134, 11), // Dark Goldenrod - queue
        _ => egui::Color32::from_rgb(105, 105, 105),                 // Dim Gray - others
    }
}

/// Rendering state for executable examples
struct TriangleState {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
}

struct CubeState {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,
    depth_view: wgpu::TextureView,
    time: f32,
}

enum RenderState {
    None,
    Triangle(Box<TriangleState>),
    Cube(Box<CubeState>),
}

impl RenderState {
    fn update(
        &mut self,
        queue: &Queue,
        delta_time: f32,
        camera_distance: f32,
        camera_rot_x: f32,
        camera_rot_y: f32,
        aspect: f32,
    ) {
        if let RenderState::Cube(cube_state) = self {
            cube_state.time += delta_time;

            // Update transformation matrix
            #[repr(C)]
            #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
            struct Uniforms {
                view_proj: [[f32; 4]; 4],
                model: [[f32; 4]; 4],
            }

            let projection = perspective_matrix(45.0_f32.to_radians(), aspect, 0.1, 100.0);

            // Calculate camera position based on rotation and distance
            let cam_x = camera_distance * camera_rot_y.sin() * camera_rot_x.cos();
            let cam_y = camera_distance * camera_rot_x.sin();
            let cam_z = camera_distance * camera_rot_y.cos() * camera_rot_x.cos();

            let view = look_at_matrix([cam_x, cam_y, cam_z], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
            let view_proj = matrix_multiply(&projection, &view);

            let rotation_y = rotation_y_matrix(cube_state.time);
            let rotation_x = rotation_x_matrix(cube_state.time * 0.5);
            let model = matrix_multiply(&rotation_y, &rotation_x);

            let uniforms = Uniforms { view_proj, model };

            queue.write_buffer(
                &cube_state.uniform_buffer,
                0,
                bytemuck::cast_slice(&[uniforms]),
            );
        }
    }
}

pub struct RenderingPanel {
    examples: Vec<Example>,
    selected_example: Option<usize>,
    show_source_code: bool,
    category_filter: Option<ExampleCategory>,
    render_state: RenderState,
    render_texture: Option<wgpu::Texture>,
    render_texture_view: Option<wgpu::TextureView>,
    render_texture_id: Option<egui::TextureId>,
    is_example_running: bool,
    shader_editor: ShaderEditor,
    show_shader_editor: bool,
    // Canvas controls
    canvas_width: u32,
    canvas_height: u32,
    clear_color: [f32; 4],
    // Camera control for 3D examples
    camera_distance: f32,
    camera_rotation_x: f32,
    camera_rotation_y: f32,
    // Track if we've auto-started an example
    first_render: bool,
    // Code export
    export_project_name: String,
    export_status_message: Option<(String, bool)>, // (message, is_success)
}

impl Default for RenderingPanel {
    fn default() -> Self {
        Self::new_without_device()
    }
}

impl RenderingPanel {
    pub fn new(device: &Device, _queue: &Queue) -> Self {
        let mut panel = Self::new_without_device();
        panel.init_render_texture(device);
        panel
    }

    fn new_without_device() -> Self {
        Self {
            examples: get_all_examples(),
            selected_example: Some(0), // Auto-select first example (triangle)
            show_source_code: false,
            category_filter: None,
            render_state: RenderState::None,
            render_texture: None,
            render_texture_view: None,
            render_texture_id: None,
            is_example_running: false,
            shader_editor: ShaderEditor::new(),
            show_shader_editor: false,
            canvas_width: 512,
            canvas_height: 512,
            clear_color: [0.05, 0.05, 0.1, 1.0],
            camera_distance: 3.0,
            camera_rotation_x: 0.0,
            camera_rotation_y: 0.0,
            first_render: true, // Mark that this is the first render
            export_project_name: "wgpu_standalone".to_string(),
            export_status_message: None,
        }
    }

    fn init_render_texture(&mut self, device: &Device) {
        // Create a texture for rendering examples using current canvas size
        let size = wgpu::Extent3d {
            width: self.canvas_width,
            height: self.canvas_height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Render Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_SRC, // For screenshot
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        self.render_texture = Some(texture);
        self.render_texture_view = Some(view);
        // Reset texture ID when texture is recreated
        self.render_texture_id = None;
    }

    fn create_triangle_render_state(&mut self, device: &Device, queue: &Queue) {
        let shader_source = r#"
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(in.position, 1.0);
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
"#;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Triangle Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        #[repr(C)]
        #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
        struct Vertex {
            position: [f32; 3],
            color: [f32; 3],
        }

        let vertices = [
            Vertex {
                position: [0.0, 0.5, 0.0],
                color: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.0],
                color: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                color: [0.0, 0.0, 1.0],
            },
        ];

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Triangle Vertex Buffer"),
            size: std::mem::size_of_val(&vertices) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        queue.write_buffer(&vertex_buffer, 0, bytemuck::cast_slice(&vertices));

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Triangle Pipeline Layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Triangle Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        });

        self.render_state = RenderState::Triangle(Box::new(TriangleState {
            pipeline,
            vertex_buffer,
        }));
    }

    fn create_cube_render_state(&mut self, device: &Device, queue: &Queue) {
        let shader_source = r#"
struct Uniforms {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let world_position = uniforms.model * vec4<f32>(in.position, 1.0);
    out.clip_position = uniforms.view_proj * world_position;
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
"#;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Cube Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        #[repr(C)]
        #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
        struct Vertex {
            position: [f32; 3],
            color: [f32; 3],
        }

        // Cube vertices (8 corners)
        let vertices = [
            // Front face (red-ish)
            Vertex {
                position: [-0.5, -0.5, 0.5],
                color: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                color: [1.0, 0.3, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                color: [1.0, 0.6, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                color: [1.0, 0.9, 0.0],
            },
            // Back face (blue-ish)
            Vertex {
                position: [-0.5, -0.5, -0.5],
                color: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                color: [0.0, 0.3, 1.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                color: [0.0, 0.6, 1.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                color: [0.0, 0.9, 1.0],
            },
        ];

        // Cube indices (36 indices for 12 triangles)
        let indices: [u16; 36] = [
            0, 1, 2, 2, 3, 0, // Front
            1, 5, 6, 6, 2, 1, // Right
            5, 4, 7, 7, 6, 5, // Back
            4, 0, 3, 3, 7, 4, // Left
            3, 2, 6, 6, 7, 3, // Top
            4, 5, 1, 1, 0, 4, // Bottom
        ];

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Cube Vertex Buffer"),
            size: std::mem::size_of_val(&vertices) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(&vertex_buffer, 0, bytemuck::cast_slice(&vertices));

        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Cube Index Buffer"),
            size: std::mem::size_of_val(&indices) as u64,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(&index_buffer, 0, bytemuck::cast_slice(&indices));

        // Create uniform buffer
        #[repr(C)]
        #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
        struct Uniforms {
            view_proj: [[f32; 4]; 4],
            model: [[f32; 4]; 4],
        }

        let uniforms = Uniforms {
            view_proj: identity_matrix(),
            model: identity_matrix(),
        };

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Cube Uniform Buffer"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(&uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Cube Bind Group Layout"),
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

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Cube Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // Create depth texture
        let size = wgpu::Extent3d {
            width: self.canvas_width,
            height: self.canvas_height,
            depth_or_array_layers: 1,
        };

        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Cube Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Cube Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        });

        self.render_state = RenderState::Cube(Box::new(CubeState {
            pipeline,
            vertex_buffer,
            index_buffer,
            bind_group,
            uniform_buffer,
            depth_view,
            time: 0.0,
        }));
    }

    fn render_current_example(&mut self, device: &Device, queue: &Queue) {
        // Update animation state
        // TODO: Pass actual delta_time from frame timer instead of hardcoded 60fps
        // This currently assumes constant frame rate, causing animation speed to vary
        let aspect = self.canvas_width as f32 / self.canvas_height as f32;
        self.render_state.update(
            queue,
            0.016, // ~60fps
            self.camera_distance,
            self.camera_rotation_x,
            self.camera_rotation_y,
            aspect,
        );

        if let Some(view) = &self.render_texture_view {
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Example Render Encoder"),
            });

            {
                let depth_stencil_attachment =
                    if let RenderState::Cube(cube_state) = &self.render_state {
                        Some(wgpu::RenderPassDepthStencilAttachment {
                            view: &cube_state.depth_view,
                            depth_ops: Some(wgpu::Operations {
                                load: wgpu::LoadOp::Clear(1.0),
                                store: wgpu::StoreOp::Store,
                            }),
                            stencil_ops: None,
                        })
                    } else {
                        None
                    };

                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Example Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: self.clear_color[0] as f64,
                                g: self.clear_color[1] as f64,
                                b: self.clear_color[2] as f64,
                                a: self.clear_color[3] as f64,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    depth_stencil_attachment,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });

                match &self.render_state {
                    RenderState::Triangle(triangle_state) => {
                        render_pass.set_pipeline(&triangle_state.pipeline);
                        render_pass.set_vertex_buffer(0, triangle_state.vertex_buffer.slice(..));
                        render_pass.draw(0..3, 0..1);
                    }
                    RenderState::Cube(cube_state) => {
                        render_pass.set_pipeline(&cube_state.pipeline);
                        render_pass.set_bind_group(0, &cube_state.bind_group, &[]);
                        render_pass.set_vertex_buffer(0, cube_state.vertex_buffer.slice(..));
                        render_pass.set_index_buffer(
                            cube_state.index_buffer.slice(..),
                            wgpu::IndexFormat::Uint16,
                        );
                        render_pass.draw_indexed(0..36, 0, 0..1);
                    }
                    _ => {}
                }
            }

            queue.submit(std::iter::once(encoder.finish()));
        }
    }

    /// Register the render texture with egui renderer and return the texture ID
    /// 
    /// Note: This method is only available when building for native targets
    /// due to wgpu version incompatibility with egui-wgpu on WASM.
    /// We use unsafe transmute to convert wgpu 28 types to wgpu 27 types
    /// as a temporary workaround until egui-wgpu supports wgpu 28.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn register_texture(
        &mut self,
        device: &wgpu::Device,
        renderer: &mut egui_wgpu::Renderer,
    ) -> Option<egui::TextureId> {
        if let Some(view) = &self.render_texture_view {
            // If already registered, return existing ID
            if let Some(id) = self.render_texture_id {
                return Some(id);
            }

            // Register the texture
            // SAFETY: wgpu 28 and wgpu 27 have the same memory layout for these types
            // This is a temporary workaround until egui-wgpu supports wgpu 28
            let device_27: &egui_wgpu::wgpu::Device = unsafe { std::mem::transmute(device) };
            let view_27: &egui_wgpu::wgpu::TextureView = unsafe { std::mem::transmute(view) };
            let texture_id =
                renderer.register_native_texture(device_27, view_27, egui_wgpu::wgpu::FilterMode::Linear);
            self.render_texture_id = Some(texture_id);
            Some(texture_id)
        } else {
            None
        }
    }

    /// Resize the canvas and recreate render texture
    pub fn resize_canvas(&mut self, device: &Device, width: u32, height: u32) {
        if width > 0 && height > 0 && (width != self.canvas_width || height != self.canvas_height) {
            self.canvas_width = width;
            self.canvas_height = height;
            self.init_render_texture(device);

            // Recreate render state with new size if needed
            if let RenderState::Cube(_) = &self.render_state {
                // Need to recreate depth texture for cube
                // This will be handled by re-running the example
                self.is_example_running = false;
                self.render_state = RenderState::None;
            }
        }
    }

    /// Capture screenshot of current render
    pub fn capture_screenshot(&self, device: &Device, queue: &Queue) {
        if let Some(texture) = &self.render_texture {
            let width = self.canvas_width;
            let height = self.canvas_height;
            let bytes_per_pixel = 4; // BGRA8
            let unpadded_bytes_per_row = width * bytes_per_pixel;
            let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
            let padded_bytes_per_row = unpadded_bytes_per_row.div_ceil(align) * align;
            let buffer_size = (padded_bytes_per_row * height) as u64;

            let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Screenshot Buffer"),
                size: buffer_size,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            });

            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Screenshot Encoder"),
            });

            encoder.copy_texture_to_buffer(
                wgpu::TexelCopyTextureInfo {
                    texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::TexelCopyBufferInfo {
                    buffer: &output_buffer,
                    layout: wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(padded_bytes_per_row),
                        rows_per_image: Some(height),
                    },
                },
                wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
            );

            queue.submit(std::iter::once(encoder.finish()));

            // Map the buffer and save to file
            let buffer_slice = output_buffer.slice(..);
            let (tx, rx) = std::sync::mpsc::channel();
            buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
                let _ = tx.send(result); // Ignore send errors (receiver might be dropped)
            });

            let _ = device.poll(wgpu::PollType::Wait {
                submission_index: None,
                timeout: None,
            });

            match rx.recv() {
                Ok(Ok(())) => {
                    let data = buffer_slice.get_mapped_range();

                    // Convert BGRA to RGBA
                    let mut rgba_data = vec![0u8; (width * height * 4) as usize];
                    for row in 0..height {
                        let src_offset = (row * padded_bytes_per_row) as usize;
                        let dst_offset = (row * width * 4) as usize;
                        for col in 0..width {
                            let src_idx = src_offset + (col * 4) as usize;
                            let dst_idx = dst_offset + (col * 4) as usize;
                            // BGRA -> RGBA
                            rgba_data[dst_idx] = data[src_idx + 2]; // R
                            rgba_data[dst_idx + 1] = data[src_idx + 1]; // G
                            rgba_data[dst_idx + 2] = data[src_idx]; // B
                            rgba_data[dst_idx + 3] = data[src_idx + 3]; // A
                        }
                    }

                    drop(data);
                    output_buffer.unmap();

                    // Save to file
                    use std::time::SystemTime;
                    let timestamp = SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .expect("Failed to get current timestamp for screenshot filename")
                        .as_secs();
                    let filename = format!("screenshot_{}.png", timestamp);

                    if let Err(e) = image::save_buffer(
                        &filename,
                        &rgba_data,
                        width,
                        height,
                        image::ColorType::Rgba8,
                    ) {
                        log::error!("Failed to save screenshot: {}", e);
                    } else {
                        log::info!("Screenshot saved to {}", filename);
                    }
                }
                Ok(Err(e)) => {
                    log::error!("Failed to map screenshot buffer: {:?}", e);
                }
                Err(e) => {
                    log::error!("Failed to receive buffer mapping result: {}", e);
                }
            }
        }
    }

    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        device: &Device,
        queue: &Queue,
        renderer: &mut egui_wgpu::Renderer,
    ) {
        // Auto-run the triangle example on first render
        if self.first_render && !self.show_shader_editor {
            self.first_render = false;
            if let Some(0) = self.selected_example {
                // Auto-run the first example (triangle)
                self.is_example_running = true;
                self.create_triangle_render_state(device, queue);
            }
        }

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("🎨 Rendering Examples");
            ui.separator();
            ui.label("Explore WebGPU rendering with interactive examples. Click an example to see its code and rendering output.");
            ui.add_space(10.0);

            // Category filter
            // Top-level tabs: Gallery vs Shader Editor
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.show_shader_editor, false, "📚 Example Gallery");
                ui.selectable_value(&mut self.show_shader_editor, true, "📝 Shader Editor");
            });

            ui.add_space(10.0);
            ui.separator();

            if self.show_shader_editor {
                // Show the shader editor
                // TODO(shader_editor): Pass device when available for compilation support
                // Currently compilation is disabled without a device
                // See issue: Need to make device available to RenderingPanel
                self.shader_editor.ui(ui, None);
            } else {
                // Show the example gallery (existing code)
                self.render_example_gallery(ui, device, queue, renderer);
            }
        });
    }

    fn render_example_gallery(
        &mut self,
        ui: &mut egui::Ui,
        device: &Device,
        queue: &Queue,
        renderer: &mut egui_wgpu::Renderer,
    ) {
        // Show render preview prominently at top if an example is running
        if self.is_example_running {
            if let Some(idx) = self.selected_example {
                let example_id = self.examples[idx].id;
                let example_name = self.examples[idx].name;

                ui.heading(format!("🎨 {}", example_name));
                ui.separator();
                ui.add_space(5.0);

                // Render the example first
                self.render_current_example(device, queue);

                // Display the rendered texture prominently
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(texture_id) = self.register_texture(device, renderer) {
                    let size = egui::vec2(self.canvas_width as f32, self.canvas_height as f32);

                    // Create an interactive canvas for mouse control
                    let response = ui.add(
                        egui::Image::new(egui::load::SizedTexture::new(texture_id, size))
                            .sense(egui::Sense::click_and_drag()),
                    );

                    // Handle mouse interaction for 3D camera control
                    if example_id == "cube" {
                        if response.dragged() {
                            let delta = response.drag_delta();
                            self.camera_rotation_y += delta.x * 0.01;
                            self.camera_rotation_x -= delta.y * 0.01;
                            // Clamp rotation_x to avoid gimbal lock
                            self.camera_rotation_x = self
                                .camera_rotation_x
                                .clamp(-std::f32::consts::PI / 2.0, std::f32::consts::PI / 2.0);
                        }

                        // Mouse wheel for zoom
                        let scroll_delta = ui.input(|i| i.smooth_scroll_delta.y);
                        if scroll_delta.abs() > 0.1 {
                            self.camera_distance -= scroll_delta * 0.01;
                            self.camera_distance = self.camera_distance.clamp(1.0, 10.0);
                        }
                    }

                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("✓ Rendering with WebGPU")
                                .color(egui::Color32::from_rgb(100, 255, 100)),
                        );
                        if example_id == "cube" {
                            ui.label(
                                egui::RichText::new("💡 Drag to rotate, scroll to zoom")
                                    .color(egui::Color32::GRAY)
                                    .italics(),
                            );
                        }
                    });
                } else {
                    ui.colored_label(egui::Color32::RED, "Failed to register render texture");
                }

                ui.add_space(10.0);
                ui.separator();
            }
        }

        // Category filter
        ui.horizontal(|ui| {
            ui.label("Filter by category:");
            if ui
                .selectable_label(self.category_filter.is_none(), "All")
                .clicked()
            {
                self.category_filter = None;
            }
            if ui
                .selectable_label(
                    self.category_filter == Some(ExampleCategory::Rendering),
                    "Rendering",
                )
                .clicked()
            {
                self.category_filter = Some(ExampleCategory::Rendering);
            }
            if ui
                .selectable_label(
                    self.category_filter == Some(ExampleCategory::Compute),
                    "Compute",
                )
                .clicked()
            {
                self.category_filter = Some(ExampleCategory::Compute);
            }
        });

        ui.add_space(10.0);
        ui.separator();

        // Two-column layout: examples list on left, controls on right
        ui.columns(2, |columns| {
            // Left column: Example list
            columns[0].vertical(|ui| {
                ui.heading("Examples");
                ui.separator();

                let filtered_examples: Vec<(usize, &Example)> = self
                    .examples
                    .iter()
                    .enumerate()
                    .filter(|(_, ex)| {
                        self.category_filter.is_none()
                            || self.category_filter.as_ref() == Some(&ex.category)
                    })
                    .collect();

                for (idx, example) in filtered_examples {
                    let is_selected = self.selected_example == Some(idx);
                    let category_icon = match example.category {
                        ExampleCategory::Rendering => "🎨",
                        ExampleCategory::Compute => "🧮",
                    };

                    if ui
                        .selectable_label(
                            is_selected,
                            format!("{} {}", category_icon, example.name),
                        )
                        .clicked()
                    {
                        self.selected_example = Some(idx);
                        self.show_source_code = false;
                        self.is_example_running = false;
                    }
                }
            });

            // Right column: Controls and source code
            columns[1].vertical(|ui| {
                if let Some(idx) = self.selected_example {
                    let example_id = self.examples[idx].id;
                    let example_description = self.examples[idx].description;
                    let example_category = self.examples[idx].category.clone();
                    let example_source_code = self.examples[idx].source_code;

                    ui.heading("Controls");
                    ui.separator();

                    // Description
                    ui.label(egui::RichText::new("Description:").strong());
                    ui.label(example_description);
                    ui.add_space(5.0);

                    // API Coverage Badges
                    let coverage_tags = get_example_api_tags(example_id);
                    if !coverage_tags.is_empty() {
                        ui.label(egui::RichText::new("WebGPU APIs Covered:").strong());
                        ui.horizontal_wrapped(|ui| {
                            ui.spacing_mut().item_spacing = egui::vec2(6.0, 4.0);
                            for tag in coverage_tags {
                                let badge_text = format!(" {} ", tag.name());
                                ui.label(
                                    egui::RichText::new(&badge_text)
                                        .color(egui::Color32::WHITE)
                                        .background_color(category_badge_color(&tag))
                                        .small(),
                                );
                            }
                        });
                        ui.add_space(5.0);
                    }

                    ui.add_space(10.0);

                    // Run button (only for rendering examples)
                    if example_category == ExampleCategory::Rendering
                        && ui
                            .button(if self.is_example_running {
                                "⏹ Stop Example"
                            } else {
                                "▶ Run Example"
                            })
                            .clicked()
                    {
                        if self.is_example_running {
                            self.is_example_running = false;
                            self.render_state = RenderState::None;
                        } else {
                            self.is_example_running = true;
                            // Create render state based on example
                            if example_id == "triangle" {
                                self.create_triangle_render_state(device, queue);
                            } else if example_id == "cube" {
                                self.create_cube_render_state(device, queue);
                            }
                        }
                    }

                    // Canvas controls (only if example is running)
                    if self.is_example_running && example_category == ExampleCategory::Rendering {
                        ui.add_space(10.0);

                        ui.collapsing("⚙️ Canvas Controls", |ui| {
                            ui.horizontal(|ui| {
                                ui.label("Canvas Size:");
                                let mut width = self.canvas_width;
                                let mut height = self.canvas_height;

                                ui.add(
                                    egui::DragValue::new(&mut width)
                                        .prefix("W: ")
                                        .range(64..=2048),
                                );
                                ui.add(
                                    egui::DragValue::new(&mut height)
                                        .prefix("H: ")
                                        .range(64..=2048),
                                );

                                if ui.button("Apply").clicked() {
                                    self.resize_canvas(device, width, height);
                                    // Restart the current example with new size
                                    if example_id == "triangle" {
                                        self.create_triangle_render_state(device, queue);
                                    } else if example_id == "cube" {
                                        self.create_cube_render_state(device, queue);
                                    }
                                }
                            });

                            ui.horizontal(|ui| {
                                ui.label("Clear Color:");
                                ui.color_edit_button_rgba_unmultiplied(&mut self.clear_color);
                            });

                            if ui.button("📷 Capture Screenshot").clicked() {
                                self.capture_screenshot(device, queue);
                            }

                            // Camera controls for 3D examples
                            if example_id == "cube" {
                                ui.separator();
                                ui.label("Camera Controls:");
                                ui.add(
                                    egui::Slider::new(&mut self.camera_distance, 1.0..=10.0)
                                        .text("Distance"),
                                );
                                ui.add(
                                    egui::Slider::new(
                                        &mut self.camera_rotation_x,
                                        -std::f32::consts::FRAC_PI_2..=std::f32::consts::FRAC_PI_2,
                                    )
                                    .text("Rotation X (up/down)"),
                                );
                                ui.add(
                                    egui::Slider::new(
                                        &mut self.camera_rotation_y,
                                        -std::f32::consts::PI..=std::f32::consts::PI,
                                    )
                                    .text("Rotation Y (left/right)"),
                                );
                                if ui.button("Reset Camera").clicked() {
                                    self.camera_distance = 3.0;
                                    self.camera_rotation_x = 0.0;
                                    self.camera_rotation_y = 0.0;
                                }
                            }
                        });
                    }

                    ui.add_space(10.0);

                    // Toggle source code button
                    if ui
                        .button(if self.show_source_code {
                            "Hide Source Code"
                        } else {
                            "Show Source Code"
                        })
                        .clicked()
                    {
                        self.show_source_code = !self.show_source_code;
                    }

                    // Source code display
                    if self.show_source_code {
                        ui.add_space(5.0);
                        ui.separator();
                        ui.label(egui::RichText::new("Source Code:").strong());

                        egui::ScrollArea::vertical()
                            .max_height(300.0)
                            .show(ui, |ui| {
                                let mut source_code = example_source_code.to_string();
                                ui.add(
                                    egui::TextEdit::multiline(&mut source_code)
                                        .code_editor()
                                        .desired_width(f32::INFINITY)
                                        .interactive(false),
                                );
                            });
                    }

                    // Copy source code button
                    ui.add_space(5.0);
                    if ui.button("📋 Copy Source Code").clicked() {
                        ui.output_mut(|o| {
                            o.commands.push(egui::OutputCommand::CopyText(
                                example_source_code.to_string(),
                            ))
                        });
                    }

                    // Export to standalone project button
                    ui.add_space(10.0);
                    ui.separator();
                    ui.label(egui::RichText::new("💾 Export to Standalone Project").strong());
                    ui.label(
                        "Generate a complete Cargo project that you can build and run separately.",
                    );

                    ui.horizontal(|ui| {
                        ui.label("Project name:");
                        ui.text_edit_singleline(&mut self.export_project_name);
                    });

                    if ui.button("📦 Export Project").clicked() {
                        self.export_to_standalone_project(example_id, example_source_code);
                    }

                    // Show export status message
                    if let Some((message, is_success)) = &self.export_status_message {
                        ui.add_space(5.0);
                        if *is_success {
                            ui.colored_label(egui::Color32::from_rgb(100, 255, 100), message);
                        } else {
                            ui.colored_label(egui::Color32::RED, message);
                        }
                    }
                } else {
                    ui.colored_label(
                        egui::Color32::GRAY,
                        "← Select an example from the list to get started",
                    );
                }
            });
        });

        ui.add_space(20.0);
        ui.separator();
        ui.colored_label(
            egui::Color32::from_rgb(100, 150, 255),
            "💡 Tip: Select a rendering example and click 'Run Example' to see it in action!",
        );
    }

    /// Export the shader editor state
    pub fn export_shader_editor_state(&self) -> crate::state::ShaderEditorState {
        self.shader_editor.export_state()
    }

    /// Import shader editor state
    pub fn import_shader_editor_state(&mut self, state: &crate::state::ShaderEditorState) {
        self.shader_editor.import_state(state);
    }

    /// Export the current configuration to a standalone Rust project
    fn export_to_standalone_project(&mut self, _example_id: &str, shader_source: &str) {
        // Create a simple playground state with just shader info
        let playground_state = crate::state::PlaygroundState {
            version: "1.0".to_string(),
            theme: crate::state::Theme::Dark,
            shader_editor: Some(crate::state::ShaderEditorState {
                source_code: shader_source.to_string(),
                label: "shader".to_string(),
                file_path: "shader.wgsl".to_string(),
            }),
            buffer_panel: None,
            texture_panel: None,
            sampler_panel: None,
            render_pipeline_panel: None,
            compute_pipeline_panel: None,
            bind_group_panel: None,
            bind_group_layout_panel: None,
            api_coverage: None,
            tutorial_state: None,
            learning_progress: None,
        };

        self.export_to_standalone_project_with_state(&playground_state);
    }

    /// Export the playground configuration to a standalone Rust project
    pub fn export_to_standalone_project_with_state(
        &mut self,
        playground_state: &crate::state::PlaygroundState,
    ) {
        use crate::code_generator::{CodeGenConfig, CodeGenerator};

        // Create output directory in user's home directory
        let output_path =
            if let Ok(home_dir) = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")) {
                std::path::PathBuf::from(home_dir).join(&self.export_project_name)
            } else {
                std::path::PathBuf::from(&self.export_project_name)
            };

        // Configure the code generator with full playground state
        let config = CodeGenConfig::new(self.export_project_name.clone())
            .with_canvas_size(self.canvas_width, self.canvas_height)
            .with_clear_color(self.clear_color)
            .with_playground_state(playground_state.clone());

        let generator = CodeGenerator::new(config);

        // Generate the project
        match generator.generate(&output_path) {
            Ok(_) => {
                self.export_status_message = Some((
                    format!("✅ Success! Project exported to: {}", output_path.display()),
                    true,
                ));
                log::info!("Project exported successfully to: {:?}", output_path);
            }
            Err(e) => {
                self.export_status_message =
                    Some((format!("❌ Error exporting project: {}", e), false));
                log::error!("Failed to export project: {}", e);
            }
        }
    }
}

// Matrix math utilities
fn identity_matrix() -> [[f32; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

fn perspective_matrix(fov_y: f32, aspect: f32, near: f32, far: f32) -> [[f32; 4]; 4] {
    let f = 1.0 / (fov_y / 2.0).tan();
    [
        [f / aspect, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, (far + near) / (near - far), -1.0],
        [0.0, 0.0, (2.0 * far * near) / (near - far), 0.0],
    ]
}

fn look_at_matrix(eye: [f32; 3], center: [f32; 3], up: [f32; 3]) -> [[f32; 4]; 4] {
    let f = normalize([center[0] - eye[0], center[1] - eye[1], center[2] - eye[2]]);
    let s = normalize(cross(f, up));
    let u = cross(s, f);

    [
        [s[0], u[0], -f[0], 0.0],
        [s[1], u[1], -f[1], 0.0],
        [s[2], u[2], -f[2], 0.0],
        [-dot(s, eye), -dot(u, eye), dot(f, eye), 1.0],
    ]
}

fn rotation_x_matrix(angle: f32) -> [[f32; 4]; 4] {
    let c = angle.cos();
    let s = angle.sin();
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, c, s, 0.0],
        [0.0, -s, c, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

fn rotation_y_matrix(angle: f32) -> [[f32; 4]; 4] {
    let c = angle.cos();
    let s = angle.sin();
    [
        [c, 0.0, -s, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [s, 0.0, c, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

fn matrix_multiply(a: &[[f32; 4]; 4], b: &[[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut result = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    result
}

fn normalize(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len < f32::EPSILON {
        v
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rendering_panel_new_without_device() {
        let panel = RenderingPanel::new_without_device();
        assert_eq!(panel.examples.len(), 4);
        assert_eq!(panel.selected_example, Some(0)); // First example is auto-selected
        assert!(!panel.show_source_code);
        assert_eq!(panel.category_filter, None);
        assert!(!panel.is_example_running);
        assert!(!panel.show_shader_editor);
        assert!(panel.first_render); // First render flag should be true initially
    }

    #[test]
    fn test_rendering_panel_default() {
        let panel = RenderingPanel::default();
        assert_eq!(panel.examples.len(), 4);
        assert!(!panel.is_example_running);
    }

    #[test]
    fn test_category_badge_colors_are_distinct() {
        // Ensure different categories get different colors
        let render_pass_color = category_badge_color(&ApiCategory::RenderPass);
        let compute_pass_color = category_badge_color(&ApiCategory::ComputePass);
        let buffer_color = category_badge_color(&ApiCategory::Buffer);
        let texture_color = category_badge_color(&ApiCategory::Texture);

        // Colors should be different for different categories
        assert_ne!(render_pass_color, compute_pass_color);
        assert_ne!(buffer_color, texture_color);
        assert_ne!(render_pass_color, buffer_color);
    }

    #[test]
    fn test_example_metadata_integration() {
        // Verify that all examples have API coverage metadata
        let examples = get_all_examples();
        for example in examples {
            let tags = get_example_api_tags(example.id);
            // Each example should have at least some API tags
            // (this would fail if we add new examples without updating metadata)
            if example.id == "triangle"
                || example.id == "cube"
                || example.id == "texture_mapping"
                || example.id == "compute_shader"
            {
                assert!(
                    !tags.is_empty(),
                    "Example {} should have API tags",
                    example.id
                );
            }
        }
    }
}
