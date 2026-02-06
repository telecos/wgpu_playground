//! API Reference panel for displaying inline WebGPU API documentation

use egui::{CollapsingHeader, Color32, RichText, ScrollArea, Ui};
use std::collections::HashMap;

/// Categories of WebGPU APIs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApiReferenceCategory {
    Adapter,
    Device,
    Queue,
    Buffer,
    Texture,
    Sampler,
    Shader,
    RenderPipeline,
    ComputePipeline,
    BindGroup,
    CommandEncoder,
    RenderPass,
    ComputePass,
}

impl ApiReferenceCategory {
    /// Get all categories
    pub fn all() -> Vec<Self> {
        vec![
            Self::Adapter,
            Self::Device,
            Self::Queue,
            Self::Buffer,
            Self::Texture,
            Self::Sampler,
            Self::Shader,
            Self::RenderPipeline,
            Self::ComputePipeline,
            Self::BindGroup,
            Self::CommandEncoder,
            Self::RenderPass,
            Self::ComputePass,
        ]
    }

    /// Get category display name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Adapter => "Adapter",
            Self::Device => "Device",
            Self::Queue => "Queue",
            Self::Buffer => "Buffer",
            Self::Texture => "Texture",
            Self::Sampler => "Sampler",
            Self::Shader => "Shader Module",
            Self::RenderPipeline => "Render Pipeline",
            Self::ComputePipeline => "Compute Pipeline",
            Self::BindGroup => "Bind Group",
            Self::CommandEncoder => "Command Encoder",
            Self::RenderPass => "Render Pass",
            Self::ComputePass => "Compute Pass",
        }
    }

    /// Get category description
    pub fn description(&self) -> &'static str {
        match self {
            Self::Adapter => "Represents a physical GPU adapter. Used to request a Device.",
            Self::Device => {
                "Main interface for GPU operations. Creates resources and command encoders."
            }
            Self::Queue => "Executes command buffers and transfers data to GPU memory.",
            Self::Buffer => {
                "GPU-accessible memory for vertex data, indices, uniforms, and storage."
            }
            Self::Texture => "Multi-dimensional arrays for images, render targets, and sampling.",
            Self::Sampler => "Defines how textures are sampled (filtering, addressing, etc.).",
            Self::Shader => "WGSL shader code compiled for GPU execution.",
            Self::RenderPipeline => "Complete graphics pipeline state for rendering operations.",
            Self::ComputePipeline => "Pipeline for compute shader execution.",
            Self::BindGroup => "Set of resources (buffers, textures, samplers) bound to shaders.",
            Self::CommandEncoder => "Records GPU commands into command buffers for execution.",
            Self::RenderPass => "Sequence of rendering operations with attachments.",
            Self::ComputePass => "Sequence of compute operations.",
        }
    }

    /// Get WebGPU specification URL
    pub fn spec_url(&self) -> &'static str {
        match self {
            Self::Adapter => "https://www.w3.org/TR/webgpu/#gpu-adapter",
            Self::Device => "https://www.w3.org/TR/webgpu/#gpu-device",
            Self::Queue => "https://www.w3.org/TR/webgpu/#gpu-queue",
            Self::Buffer => "https://www.w3.org/TR/webgpu/#gpu-buffer",
            Self::Texture => "https://www.w3.org/TR/webgpu/#gpu-texture",
            Self::Sampler => "https://www.w3.org/TR/webgpu/#gpu-sampler",
            Self::Shader => "https://www.w3.org/TR/webgpu/#gpu-shadermodule",
            Self::RenderPipeline => "https://www.w3.org/TR/webgpu/#gpu-renderpipeline",
            Self::ComputePipeline => "https://www.w3.org/TR/webgpu/#gpu-computepipeline",
            Self::BindGroup => "https://www.w3.org/TR/webgpu/#gpu-bindgroup",
            Self::CommandEncoder => "https://www.w3.org/TR/webgpu/#gpu-commandencoder",
            Self::RenderPass => "https://www.w3.org/TR/webgpu/#render-passes",
            Self::ComputePass => "https://www.w3.org/TR/webgpu/#compute-passes",
        }
    }
}

/// API method information
#[derive(Clone)]
pub struct ApiMethod {
    pub name: &'static str,
    pub description: &'static str,
    pub signature: &'static str,
    pub example: Option<&'static str>,
}

/// Get API methods for a category
fn get_api_methods(category: ApiReferenceCategory) -> Vec<ApiMethod> {
    match category {
        ApiReferenceCategory::Adapter => vec![
            ApiMethod {
                name: "request_device",
                description: "Requests a device from the adapter with specified features and limits.",
                signature: "async fn request_device(&self, descriptor: &DeviceDescriptor) -> Result<Device, RequestDeviceError>",
                example: Some("let device = adapter.request_device(&Default::default()).await?;"),
            },
            ApiMethod {
                name: "get_info",
                description: "Gets information about the adapter (name, backend, device type).",
                signature: "fn get_info(&self) -> AdapterInfo",
                example: Some("let info = adapter.get_info();"),
            },
        ],
        ApiReferenceCategory::Device => vec![
            ApiMethod {
                name: "create_buffer",
                description: "Creates a GPU buffer with the specified size and usage flags.",
                signature: "fn create_buffer(&self, descriptor: &BufferDescriptor) -> Buffer",
                example: Some("let buffer = device.create_buffer(&BufferDescriptor {\n    label: Some(\"Vertex Buffer\"),\n    size: 1024,\n    usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,\n    mapped_at_creation: false,\n});"),
            },
            ApiMethod {
                name: "create_texture",
                description: "Creates a GPU texture with specified dimensions and format.",
                signature: "fn create_texture(&self, descriptor: &TextureDescriptor) -> Texture",
                example: Some("let texture = device.create_texture(&TextureDescriptor {\n    size: Extent3d { width: 256, height: 256, depth_or_array_layers: 1 },\n    format: TextureFormat::Rgba8UnormSrgb,\n    usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,\n    ..Default::default()\n});"),
            },
            ApiMethod {
                name: "create_shader_module",
                description: "Creates a shader module from WGSL source code.",
                signature: "fn create_shader_module(&self, descriptor: ShaderModuleDescriptor) -> ShaderModule",
                example: Some("let shader = device.create_shader_module(ShaderModuleDescriptor {\n    label: Some(\"Shader\"),\n    source: ShaderSource::Wgsl(source.into()),\n});"),
            },
            ApiMethod {
                name: "create_render_pipeline",
                description: "Creates a render pipeline with vertex/fragment shaders and state.",
                signature: "fn create_render_pipeline(&self, descriptor: &RenderPipelineDescriptor) -> RenderPipeline",
                example: Some("let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {\n    vertex: VertexState { ... },\n    fragment: Some(FragmentState { ... }),\n    ..Default::default()\n});"),
            },
            ApiMethod {
                name: "create_compute_pipeline",
                description: "Creates a compute pipeline for compute shader execution.",
                signature: "fn create_compute_pipeline(&self, descriptor: &ComputePipelineDescriptor) -> ComputePipeline",
                example: Some("let pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {\n    compute: ComputeState { ... },\n    ..Default::default()\n});"),
            },
            ApiMethod {
                name: "create_command_encoder",
                description: "Creates a command encoder for recording GPU commands.",
                signature: "fn create_command_encoder(&self, descriptor: &CommandEncoderDescriptor) -> CommandEncoder",
                example: Some("let encoder = device.create_command_encoder(&CommandEncoderDescriptor { label: Some(\"Encoder\") });"),
            },
            ApiMethod {
                name: "create_bind_group",
                description: "Creates a bind group binding resources to shader stages.",
                signature: "fn create_bind_group(&self, descriptor: &BindGroupDescriptor) -> BindGroup",
                example: Some("let bind_group = device.create_bind_group(&BindGroupDescriptor {\n    layout: &bind_group_layout,\n    entries: &[...],\n    ..Default::default()\n});"),
            },
        ],
        ApiReferenceCategory::Queue => vec![
            ApiMethod {
                name: "submit",
                description: "Submits command buffers to the GPU for execution.",
                signature: "fn submit<I: IntoIterator<Item = CommandBuffer>>(&self, command_buffers: I) -> SubmissionIndex",
                example: Some("queue.submit(std::iter::once(encoder.finish()));"),
            },
            ApiMethod {
                name: "write_buffer",
                description: "Writes data to a buffer, updating GPU memory.",
                signature: "fn write_buffer(&self, buffer: &Buffer, offset: BufferAddress, data: &[u8])",
                example: Some("queue.write_buffer(&buffer, 0, bytemuck::cast_slice(&vertices));"),
            },
            ApiMethod {
                name: "write_texture",
                description: "Writes image data to a texture.",
                signature: "fn write_texture(&self, texture: TexelCopyTextureInfo, data: &[u8], data_layout: TexelCopyBufferLayout, size: Extent3d)",
                example: Some("queue.write_texture(texture.as_image_copy(), &image_data, layout, size);"),
            },
        ],
        ApiReferenceCategory::Buffer => vec![
            ApiMethod {
                name: "slice",
                description: "Creates a view into a buffer for mapping or copying.",
                signature: "fn slice<S: RangeBounds<BufferAddress>>(&self, bounds: S) -> BufferSlice",
                example: Some("let slice = buffer.slice(..);"),
            },
            ApiMethod {
                name: "map_async",
                description: "Maps buffer memory for CPU access (async operation).",
                signature: "fn map_async(&self, mode: MapMode, range: Range<BufferAddress>) -> impl Future<Output = Result<(), BufferAsyncError>>",
                example: Some("buffer.slice(..).map_async(MapMode::Read).await?;"),
            },
            ApiMethod {
                name: "unmap",
                description: "Unmaps buffer memory, making it available to GPU again.",
                signature: "fn unmap(&self)",
                example: Some("buffer.unmap();"),
            },
        ],
        ApiReferenceCategory::Texture => vec![
            ApiMethod {
                name: "create_view",
                description: "Creates a texture view for use in bind groups or render passes.",
                signature: "fn create_view(&self, descriptor: &TextureViewDescriptor) -> TextureView",
                example: Some("let view = texture.create_view(&TextureViewDescriptor::default());"),
            },
            ApiMethod {
                name: "as_image_copy",
                description: "Creates an image copy descriptor for texture copying operations.",
                signature: "fn as_image_copy(&self) -> TexelCopyTextureInfo",
                example: Some("let copy = texture.as_image_copy();"),
            },
        ],
        ApiReferenceCategory::Sampler => vec![
            ApiMethod {
                name: "create_sampler",
                description: "Creates a sampler (created via device.create_sampler).",
                signature: "fn create_sampler(&self, descriptor: &SamplerDescriptor) -> Sampler",
                example: Some("let sampler = device.create_sampler(&SamplerDescriptor {\n    address_mode_u: AddressMode::Repeat,\n    mag_filter: FilterMode::Linear,\n    min_filter: FilterMode::Linear,\n    ..Default::default()\n});"),
            },
        ],
        ApiReferenceCategory::Shader => vec![
            ApiMethod {
                name: "get_compilation_info",
                description: "Gets compilation messages and errors from shader compilation.",
                signature: "async fn get_compilation_info(&self) -> CompilationInfo",
                example: Some("let info = shader.get_compilation_info().await;"),
            },
        ],
        ApiReferenceCategory::RenderPipeline => vec![
            ApiMethod {
                name: "get_bind_group_layout",
                description: "Gets the bind group layout at the specified index (auto layout).",
                signature: "fn get_bind_group_layout(&self, index: u32) -> BindGroupLayout",
                example: Some("let layout = pipeline.get_bind_group_layout(0);"),
            },
        ],
        ApiReferenceCategory::ComputePipeline => vec![
            ApiMethod {
                name: "get_bind_group_layout",
                description: "Gets the bind group layout at the specified index (auto layout).",
                signature: "fn get_bind_group_layout(&self, index: u32) -> BindGroupLayout",
                example: Some("let layout = compute_pipeline.get_bind_group_layout(0);"),
            },
        ],
        ApiReferenceCategory::BindGroup => vec![
            ApiMethod {
                name: "create_bind_group_layout",
                description: "Creates a bind group layout describing resource bindings.",
                signature: "fn create_bind_group_layout(&self, descriptor: &BindGroupLayoutDescriptor) -> BindGroupLayout",
                example: Some("let layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {\n    entries: &[BindGroupLayoutEntry { ... }],\n    ..Default::default()\n});"),
            },
        ],
        ApiReferenceCategory::CommandEncoder => vec![
            ApiMethod {
                name: "begin_render_pass",
                description: "Begins a render pass with color and depth attachments.",
                signature: "fn begin_render_pass<'a>(&'a mut self, descriptor: &RenderPassDescriptor<'a>) -> RenderPass<'a>",
                example: Some("let render_pass = encoder.begin_render_pass(&RenderPassDescriptor {\n    color_attachments: &[...],\n    depth_stencil_attachment: Some(...),\n    ..Default::default()\n});"),
            },
            ApiMethod {
                name: "begin_compute_pass",
                description: "Begins a compute pass for compute shader execution.",
                signature: "fn begin_compute_pass<'a>(&'a mut self, descriptor: &ComputePassDescriptor<'a>) -> ComputePass<'a>",
                example: Some("let compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor { label: Some(\"Compute\") });"),
            },
            ApiMethod {
                name: "copy_buffer_to_buffer",
                description: "Copies data from one buffer to another.",
                signature: "fn copy_buffer_to_buffer(&mut self, source: &Buffer, source_offset: BufferAddress, destination: &Buffer, destination_offset: BufferAddress, copy_size: BufferAddress)",
                example: Some("encoder.copy_buffer_to_buffer(&src, 0, &dst, 0, 1024);"),
            },
            ApiMethod {
                name: "finish",
                description: "Finishes recording and returns a command buffer for submission.",
                signature: "fn finish(self) -> CommandBuffer",
                example: Some("let command_buffer = encoder.finish();"),
            },
        ],
        ApiReferenceCategory::RenderPass => vec![
            ApiMethod {
                name: "set_pipeline",
                description: "Sets the active render pipeline.",
                signature: "fn set_pipeline(&mut self, pipeline: &RenderPipeline)",
                example: Some("render_pass.set_pipeline(&pipeline);"),
            },
            ApiMethod {
                name: "set_bind_group",
                description: "Sets a bind group for the current pipeline.",
                signature: "fn set_bind_group(&mut self, index: u32, bind_group: &BindGroup, offsets: &[DynamicOffset])",
                example: Some("render_pass.set_bind_group(0, &bind_group, &[]);"),
            },
            ApiMethod {
                name: "set_vertex_buffer",
                description: "Sets a vertex buffer for vertex shader input.",
                signature: "fn set_vertex_buffer(&mut self, slot: u32, buffer_slice: BufferSlice)",
                example: Some("render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));"),
            },
            ApiMethod {
                name: "set_index_buffer",
                description: "Sets an index buffer for indexed drawing.",
                signature: "fn set_index_buffer(&mut self, buffer_slice: BufferSlice, index_format: IndexFormat)",
                example: Some("render_pass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint16);"),
            },
            ApiMethod {
                name: "draw",
                description: "Draws primitives from vertex buffers.",
                signature: "fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>)",
                example: Some("render_pass.draw(0..3, 0..1);"),
            },
            ApiMethod {
                name: "draw_indexed",
                description: "Draws indexed primitives using an index buffer.",
                signature: "fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>)",
                example: Some("render_pass.draw_indexed(0..6, 0, 0..1);"),
            },
        ],
        ApiReferenceCategory::ComputePass => vec![
            ApiMethod {
                name: "set_pipeline",
                description: "Sets the active compute pipeline.",
                signature: "fn set_pipeline(&mut self, pipeline: &ComputePipeline)",
                example: Some("compute_pass.set_pipeline(&compute_pipeline);"),
            },
            ApiMethod {
                name: "set_bind_group",
                description: "Sets a bind group for the current compute pipeline.",
                signature: "fn set_bind_group(&mut self, index: u32, bind_group: &BindGroup, offsets: &[DynamicOffset])",
                example: Some("compute_pass.set_bind_group(0, &bind_group, &[]);"),
            },
            ApiMethod {
                name: "dispatch_workgroups",
                description: "Dispatches compute shader workgroups.",
                signature: "fn dispatch_workgroups(&mut self, x: u32, y: u32, z: u32)",
                example: Some("compute_pass.dispatch_workgroups(8, 8, 1);"),
            },
        ],
    }
}

/// Panel for displaying API reference documentation
pub struct ApiReferencePanel {
    /// Currently selected category
    selected_category: Option<ApiReferenceCategory>,
    /// Search/filter text
    filter_text: String,
    /// Expanded methods
    expanded_methods: HashMap<String, bool>,
}

impl Default for ApiReferencePanel {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiReferencePanel {
    /// Create a new API reference panel
    pub fn new() -> Self {
        Self {
            selected_category: None,
            filter_text: String::new(),
            expanded_methods: HashMap::new(),
        }
    }

    /// Render the panel UI
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.heading("📖 WebGPU API Reference");
        ui.add_space(5.0);

        ui.label("Browse WebGPU API documentation and examples. Click on categories to view methods and usage examples.");
        ui.add_space(10.0);

        // Search/filter bar
        ui.horizontal(|ui| {
            ui.label("🔍 Search:");
            ui.text_edit_singleline(&mut self.filter_text);
            if ui.button("Clear").clicked() {
                self.filter_text.clear();
                self.selected_category = None;
            }
        });

        ui.separator();
        ui.add_space(5.0);

        // Two-column layout: category list and details
        ui.horizontal(|ui| {
            // Left panel: category list
            ui.vertical(|ui| {
                ui.set_width(200.0);
                ui.heading("Categories");
                ui.separator();

                ScrollArea::vertical().show(ui, |ui| {
                    for category in ApiReferenceCategory::all() {
                        let is_selected = self.selected_category == Some(category);
                        let response = ui.selectable_label(is_selected, category.name());
                        if response.clicked() {
                            self.selected_category = Some(category);
                        }
                        response.on_hover_text(category.description());
                    }
                });
            });

            ui.separator();

            // Right panel: category details
            ui.vertical(|ui| {
                if let Some(category) = self.selected_category {
                    self.render_category_details(ui, category);
                } else {
                    ui.vertical_centered(|ui| {
                        ui.add_space(50.0);
                        ui.label(
                            RichText::new("Select a category to view API reference")
                                .color(Color32::GRAY)
                                .size(16.0),
                        );
                    });
                }
            });
        });
    }

    fn render_category_details(&mut self, ui: &mut Ui, category: ApiReferenceCategory) {
        ScrollArea::vertical().show(ui, |ui| {
            // Category header
            ui.heading(category.name());
            ui.add_space(5.0);
            ui.label(category.description());
            ui.add_space(10.0);

            // Specification link
            ui.horizontal(|ui| {
                ui.label("📄 Specification:");
                if ui
                    .link(category.spec_url())
                    .on_hover_text("Open WebGPU specification in browser")
                    .clicked()
                {
                    self.open_url(category.spec_url());
                }
            });

            ui.separator();
            ui.add_space(10.0);

            // Methods
            ui.heading("Methods");
            ui.add_space(5.0);

            let methods = get_api_methods(category);
            let filter_lower = self.filter_text.to_lowercase();

            for method in &methods {
                // Apply filter
                if !filter_lower.is_empty() {
                    let method_text =
                        format!("{} {}", method.name, method.description).to_lowercase();
                    if !method_text.contains(&filter_lower) {
                        continue;
                    }
                }

                let method_key = format!("{:?}::{}", category, method.name);
                let is_expanded = *self.expanded_methods.get(&method_key).unwrap_or(&false);

                CollapsingHeader::new(RichText::new(method.name).strong().size(15.0))
                    .default_open(is_expanded)
                    .show(ui, |ui| {
                        self.expanded_methods.insert(method_key.clone(), true);

                        ui.label(method.description);
                        ui.add_space(5.0);

                        // Signature
                        ui.label(RichText::new("Signature:").strong());
                        ui.horizontal(|ui| {
                            ui.add_space(10.0);
                            ui.label(
                                RichText::new(method.signature)
                                    .code()
                                    .color(Color32::from_rgb(200, 200, 255)),
                            );
                        });
                        ui.add_space(5.0);

                        // Example
                        if let Some(example) = method.example {
                            ui.label(RichText::new("Example:").strong());
                            ui.horizontal(|ui| {
                                ui.add_space(10.0);
                                ui.vertical(|ui| {
                                    ui.label(
                                        RichText::new(example)
                                            .code()
                                            .color(Color32::from_rgb(200, 255, 200)),
                                    );
                                });
                            });
                        }

                        ui.add_space(5.0);
                    });

                // Track collapsed state
                if !is_expanded {
                    self.expanded_methods.insert(method_key, false);
                }
            }
        });
    }

    fn open_url(&self, url: &str) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Err(e) = webbrowser::open(url) {
                log::error!("Failed to open browser: {}", e);
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            match web_sys::window() {
                Some(w) => {
                    if let Err(e) = w.open_with_url(url) {
                        log::error!("Failed to open documentation: {:?}", e);
                    }
                }
                None => {
                    log::error!("Failed to get window object");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panel_creation() {
        let panel = ApiReferencePanel::new();
        assert!(panel.selected_category.is_none());
        assert_eq!(panel.filter_text, "");
        assert!(panel.expanded_methods.is_empty());
    }

    #[test]
    fn test_all_categories_have_names() {
        for category in ApiReferenceCategory::all() {
            assert!(!category.name().is_empty());
            assert!(!category.description().is_empty());
            assert!(category.spec_url().starts_with("https://"));
        }
    }

    #[test]
    fn test_all_categories_have_methods() {
        for category in ApiReferenceCategory::all() {
            let methods = get_api_methods(category);
            assert!(
                !methods.is_empty(),
                "Category {:?} has no methods",
                category
            );
            for method in methods {
                assert!(!method.name.is_empty());
                assert!(!method.description.is_empty());
                assert!(!method.signature.is_empty());
            }
        }
    }
}
