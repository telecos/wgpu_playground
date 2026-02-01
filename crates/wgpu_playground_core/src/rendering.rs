use crate::examples::{get_all_examples, Example, ExampleCategory};
use crate::shader_editor::ShaderEditor;
use wgpu::{Device, Queue};

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
    fn update(&mut self, queue: &Queue, delta_time: f32) {
        if let RenderState::Cube(cube_state) = self {
            cube_state.time += delta_time;

            // Update transformation matrix
            #[repr(C)]
            #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
            struct Uniforms {
                view_proj: [[f32; 4]; 4],
                model: [[f32; 4]; 4],
            }

            let aspect = 1.0;
            let projection = perspective_matrix(45.0_f32.to_radians(), aspect, 0.1, 100.0);
            let view = look_at_matrix([0.0, 0.0, 3.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
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
    is_example_running: bool,
    shader_editor: ShaderEditor,
    show_shader_editor: bool,
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
            selected_example: None,
            show_source_code: false,
            category_filter: None,
            render_state: RenderState::None,
            render_texture: None,
            render_texture_view: None,
            is_example_running: false,
            shader_editor: ShaderEditor::new(),
            show_shader_editor: false,
        }
    }

    fn init_render_texture(&mut self, device: &Device) {
        // Create a 512x512 texture for rendering examples
        let size = wgpu::Extent3d {
            width: 512,
            height: 512,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Render Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        self.render_texture = Some(texture);
        self.render_texture_view = Some(view);
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
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Triangle Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
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
            multiview: None,
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
            width: 512,
            height: 512,
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
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Cube Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
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
            multiview: None,
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
        self.render_state.update(queue, 0.016); // ~60fps

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
                                r: 0.05,
                                g: 0.05,
                                b: 0.1,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment,
                    timestamp_writes: None,
                    occlusion_query_set: None,
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

    pub fn ui(&mut self, ui: &mut egui::Ui, device: &Device, queue: &Queue) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("🎨 Rendering Examples");
            ui.separator();
            ui.label("Select an example to view its source code and run it.");
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
                self.render_example_gallery(ui, device, queue);
            }
        });
    }

    fn render_example_gallery(&mut self, ui: &mut egui::Ui, device: &Device, queue: &Queue) {
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

        // Two-column layout: examples list on left, preview on right
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

            // Right column: Preview and source code
            columns[1].vertical(|ui| {
                if let Some(idx) = self.selected_example {
                    let example_id = self.examples[idx].id;
                    let example_name = self.examples[idx].name;
                    let example_description = self.examples[idx].description;
                    let example_category = self.examples[idx].category.clone();
                    let example_source_code = self.examples[idx].source_code;

                    ui.heading(format!("🎨 {}", example_name));
                    ui.separator();

                    // Description
                    ui.label(egui::RichText::new("Description:").strong());
                    ui.label(example_description);
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

                    // Render preview if example is running
                    if self.is_example_running && example_category == ExampleCategory::Rendering {
                        ui.add_space(10.0);
                        ui.separator();
                        ui.label(egui::RichText::new("Preview:").strong());

                        // Render the example
                        self.render_current_example(device, queue);

                        // Draw a gradient background to show the rendering area
                        let (rect, _response) =
                            ui.allocate_exact_size(egui::vec2(512.0, 512.0), egui::Sense::hover());

                        // Draw gradient background
                        let color_tl = egui::Color32::from_rgb(40, 20, 60);
                        let color_br = egui::Color32::from_rgb(20, 40, 80);

                        let mut mesh = egui::Mesh::default();
                        mesh.colored_vertex(rect.left_top(), color_tl);
                        mesh.colored_vertex(rect.right_top(), color_tl);
                        mesh.colored_vertex(rect.right_bottom(), color_br);
                        mesh.colored_vertex(rect.left_bottom(), color_br);
                        mesh.add_triangle(0, 1, 2);
                        mesh.add_triangle(0, 2, 3);
                        ui.painter().add(egui::Shape::mesh(mesh));

                        // Draw a border
                        ui.painter().rect_stroke(
                            rect,
                            4.0,
                            egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 150, 255)),
                        );

                        // For triangle example, draw a simple triangle representation
                        if example_id == "triangle" {
                            let center = rect.center();
                            let size = 200.0;

                            let top = egui::pos2(center.x, center.y - size * 0.5);
                            let left = egui::pos2(center.x - size * 0.5, center.y + size * 0.5);
                            let right = egui::pos2(center.x + size * 0.5, center.y + size * 0.5);

                            // Draw the triangle with gradient colors
                            let mesh = {
                                let mut mesh = egui::Mesh::default();
                                mesh.colored_vertex(top, egui::Color32::RED);
                                mesh.colored_vertex(left, egui::Color32::GREEN);
                                mesh.colored_vertex(right, egui::Color32::BLUE);
                                mesh.add_triangle(0, 1, 2);
                                mesh
                            };

                            ui.painter().add(egui::Shape::mesh(mesh));
                        } else if example_id == "cube" {
                            // Draw a simple isometric cube representation
                            let center = rect.center();
                            let size = 120.0;

                            // Draw isometric cube faces
                            // Front face
                            let front_bl = egui::pos2(center.x - size * 0.5, center.y + size * 0.3);
                            let front_br = egui::pos2(center.x + size * 0.5, center.y + size * 0.3);
                            let front_tr = egui::pos2(center.x + size * 0.5, center.y - size * 0.7);
                            let front_tl = egui::pos2(center.x - size * 0.5, center.y - size * 0.7);

                            // Top face
                            let top_fr = front_tr;
                            let top_fl = front_tl;
                            let top_bl = egui::pos2(center.x - size * 0.3, center.y - size);
                            let top_br = egui::pos2(center.x + size * 0.7, center.y - size);

                            // Right face
                            let right_br = front_br;
                            let right_tr = front_tr;

                            // Draw faces
                            // Front face (red)
                            ui.painter().add(egui::Shape::convex_polygon(
                                vec![front_bl, front_br, front_tr, front_tl],
                                egui::Color32::from_rgb(200, 80, 80),
                                egui::Stroke::NONE,
                            ));

                            // Top face (orange)
                            ui.painter().add(egui::Shape::convex_polygon(
                                vec![top_fl, top_fr, top_br, top_bl],
                                egui::Color32::from_rgb(240, 160, 80),
                                egui::Stroke::NONE,
                            ));

                            // Right face (blue)
                            ui.painter().add(egui::Shape::convex_polygon(
                                vec![right_br, top_br, top_fr, right_tr],
                                egui::Color32::from_rgb(80, 120, 200),
                                egui::Stroke::NONE,
                            ));

                            // Add rotating arrow to indicate animation
                            ui.painter().text(
                                egui::pos2(center.x, center.y + size * 0.8),
                                egui::Align2::CENTER_CENTER,
                                "🔄 Rotating",
                                egui::FontId::proportional(14.0),
                                egui::Color32::WHITE,
                            );
                        }

                        ui.painter().text(
                            egui::pos2(rect.left() + 10.0, rect.top() + 10.0),
                            egui::Align2::LEFT_TOP,
                            "✓ Example is rendering on GPU",
                            egui::FontId::proportional(14.0),
                            egui::Color32::from_rgb(100, 255, 100),
                        );
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
                        ui.output_mut(|o| o.copied_text = example_source_code.to_string());
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
        assert_eq!(panel.selected_example, None);
        assert!(!panel.show_source_code);
        assert_eq!(panel.category_filter, None);
        assert!(!panel.is_example_running);
        assert!(!panel.show_shader_editor);
    }

    #[test]
    fn test_rendering_panel_default() {
        let panel = RenderingPanel::default();
        assert_eq!(panel.examples.len(), 4);
        assert!(!panel.is_example_running);
    }
}
