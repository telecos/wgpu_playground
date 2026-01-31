use crate::examples::{get_all_examples, Example, ExampleCategory};
use wgpu::{Device, Queue};

/// Rendering state for executable examples
enum RenderState {
    None,
    Triangle {
        pipeline: wgpu::RenderPipeline,
        vertex_buffer: wgpu::Buffer,
    },
    Cube {
        pipeline: wgpu::RenderPipeline,
        vertex_buffer: wgpu::Buffer,
        index_buffer: wgpu::Buffer,
        bind_group: wgpu::BindGroup,
        uniform_buffer: wgpu::Buffer,
        depth_texture: wgpu::Texture,
        depth_view: wgpu::TextureView,
        time: f32,
    },
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
            render_texture_id: None,
            is_example_running: false,
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

        self.render_state = RenderState::Triangle {
            pipeline,
            vertex_buffer,
        };
    }

    fn render_current_example(&mut self, device: &Device, queue: &Queue) {
        if let Some(view) = &self.render_texture_view {
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Example Render Encoder"),
            });

            {
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
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

                match &self.render_state {
                    RenderState::Triangle {
                        pipeline,
                        vertex_buffer,
                    } => {
                        render_pass.set_pipeline(pipeline);
                        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                        render_pass.draw(0..3, 0..1);
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
                            .selectable_label(is_selected, format!("{} {}", category_icon, example.name))
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
                        if example_category == ExampleCategory::Rendering {
                            if ui.button(if self.is_example_running {
                                "⏹ Stop Example"
                            } else {
                                "▶ Run Example"
                            }).clicked() {
                                if self.is_example_running {
                                    self.is_example_running = false;
                                    self.render_state = RenderState::None;
                                } else {
                                    self.is_example_running = true;
                                    // Create render state based on example
                                    if example_id == "triangle" {
                                        self.create_triangle_render_state(device, queue);
                                    }
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

                            // Display the rendered texture
                            if self.render_texture.is_some() {
                                // Show a colored rectangle as placeholder for the rendered content
                                let (rect, _response) = ui.allocate_exact_size(
                                    egui::vec2(512.0, 512.0),
                                    egui::Sense::hover(),
                                );
                                
                                ui.painter().rect_filled(
                                    rect,
                                    0.0,
                                    egui::Color32::from_rgb(20, 20, 40),
                                );
                                
                                ui.painter().text(
                                    rect.center(),
                                    egui::Align2::CENTER_CENTER,
                                    "▶ Example is rendering!\n(Texture display integration pending)",
                                    egui::FontId::proportional(16.0),
                                    egui::Color32::WHITE,
                                );
                            }
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
        });
    }
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
    }

    #[test]
    fn test_rendering_panel_default() {
        let panel = RenderingPanel::default();
        assert_eq!(panel.examples.len(), 4);
        assert!(!panel.is_example_running);
    }
}
