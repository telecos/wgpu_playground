/// Code generation module for exporting playground configuration as standalone Rust project
use std::path::Path;

use crate::state::{
    BufferPanelState, ComputePipelinePanelState, PlaygroundState, RenderPipelinePanelState,
    SamplerPanelState, ShaderEditorState, TexturePanelState,
};

/// Default buffer size in bytes when parsing fails
const DEFAULT_BUFFER_SIZE: u64 = 256;

/// Default texture width in pixels when parsing fails
const DEFAULT_TEXTURE_WIDTH: u32 = 256;

/// Default texture height in pixels when parsing fails
const DEFAULT_TEXTURE_HEIGHT: u32 = 256;

/// Default texture depth when parsing fails
const DEFAULT_TEXTURE_DEPTH: u32 = 1;

/// Configuration for code generation
#[derive(Debug, Clone)]
pub struct CodeGenConfig {
    /// Project name
    pub project_name: String,
    /// Shader source code (WGSL)
    pub shader_source: Option<String>,
    /// Example type being exported
    pub example_type: ExampleType,
    /// Canvas dimensions
    pub canvas_width: u32,
    pub canvas_height: u32,
    /// Clear color [r, g, b, a]
    pub clear_color: [f32; 4],
    /// Full playground state (optional, for advanced export)
    pub playground_state: Option<PlaygroundState>,
}

/// Type of example to generate
#[derive(Debug, Clone, PartialEq)]
pub enum ExampleType {
    /// Basic triangle rendering
    Triangle,
    /// 3D rotating cube
    Cube,
    /// Custom shader code
    Custom,
}

impl Default for CodeGenConfig {
    fn default() -> Self {
        Self {
            project_name: "wgpu_standalone".to_string(),
            shader_source: None,
            example_type: ExampleType::Triangle,
            canvas_width: 800,
            canvas_height: 600,
            clear_color: [0.1, 0.1, 0.1, 1.0],
            playground_state: None,
        }
    }
}

impl CodeGenConfig {
    /// Create a new configuration
    pub fn new(project_name: String) -> Self {
        Self {
            project_name,
            ..Default::default()
        }
    }

    /// Set shader source
    pub fn with_shader(mut self, source: String) -> Self {
        self.shader_source = Some(source);
        self
    }

    /// Set example type
    pub fn with_example_type(mut self, example_type: ExampleType) -> Self {
        self.example_type = example_type;
        self
    }

    /// Set canvas dimensions
    pub fn with_canvas_size(mut self, width: u32, height: u32) -> Self {
        self.canvas_width = width;
        self.canvas_height = height;
        self
    }

    /// Set clear color
    pub fn with_clear_color(mut self, color: [f32; 4]) -> Self {
        self.clear_color = color;
        self
    }

    /// Set playground state for advanced export
    pub fn with_playground_state(mut self, state: PlaygroundState) -> Self {
        self.playground_state = Some(state);
        self
    }
}

/// Code generator for creating standalone Rust projects
pub struct CodeGenerator {
    config: CodeGenConfig,
}

impl CodeGenerator {
    /// Create a new code generator
    pub fn new(config: CodeGenConfig) -> Self {
        Self { config }
    }

    /// Generate the complete project structure
    pub fn generate(&self, output_dir: &Path) -> Result<(), std::io::Error> {
        // Create project directory
        std::fs::create_dir_all(output_dir)?;

        // Generate Cargo.toml
        self.generate_cargo_toml(output_dir)?;

        // Generate main.rs
        self.generate_main_rs(output_dir)?;

        // Generate shader file if custom shader
        if let Some(ref shader_source) = self.config.shader_source {
            self.generate_shader_file(output_dir, shader_source)?;
        }

        // Generate README
        self.generate_readme(output_dir)?;

        Ok(())
    }

    /// Generate Cargo.toml file
    fn generate_cargo_toml(&self, output_dir: &Path) -> Result<(), std::io::Error> {
        let cargo_content = self.get_cargo_toml_content();
        std::fs::write(output_dir.join("Cargo.toml"), cargo_content)
    }

    fn get_cargo_toml_content(&self) -> String {
        format!(
            "[package]\n\
            name = \"{}\"\n\
            version = \"0.1.0\"\n\
            edition = \"2021\"\n\
            \n\
            [dependencies]\n\
            wgpu = \"27.0\"\n\
            winit = \"0.30\"\n\
            pollster = \"0.4\"\n\
            env_logger = \"0.11\"\n\
            log = \"0.4\"\n\
            bytemuck = {{ version = \"1.19\", features = [\"derive\"] }}\n\
            \n\
            [[bin]]\n\
            name = \"{}\"\n\
            path = \"src/main.rs\"\n",
            self.config.project_name, self.config.project_name
        )
    }

    /// Generate main.rs file
    fn generate_main_rs(&self, output_dir: &Path) -> Result<(), std::io::Error> {
        let src_dir = output_dir.join("src");
        std::fs::create_dir_all(&src_dir)?;

        let content = if let Some(ref playground_state) = self.config.playground_state {
            // Generate advanced main.rs based on playground state
            self.generate_playground_main(playground_state)
        } else {
            // Generate simple example main.rs
            match self.config.example_type {
                ExampleType::Triangle => self.generate_triangle_main(),
                ExampleType::Cube => self.generate_cube_main(),
                ExampleType::Custom => self.generate_custom_main(),
            }
        };

        std::fs::write(src_dir.join("main.rs"), content)
    }

    /// Get default shader source
    fn get_default_shader_source(&self) -> &str {
        self.config.shader_source.as_deref().unwrap_or(
            "struct VertexInput {\n\
                @location(0) position: vec3<f32>,\n\
                @location(1) color: vec3<f32>,\n\
            }\n\
            \n\
            struct VertexOutput {\n\
                @builtin(position) clip_position: vec4<f32>,\n\
                @location(0) color: vec3<f32>,\n\
            }\n\
            \n\
            @vertex\n\
            fn vs_main(in: VertexInput) -> VertexOutput {\n\
                var out: VertexOutput;\n\
                out.clip_position = vec4<f32>(in.position, 1.0);\n\
                out.color = in.color;\n\
                return out;\n\
            }\n\
            \n\
            @fragment\n\
            fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {\n\
                return vec4<f32>(in.color, 1.0);\n\
            }",
        )
    }

    /// Generate triangle example main.rs
    fn generate_triangle_main(&self) -> String {
        let shader_source = self.get_default_shader_source();

        format!(
            "use std::sync::Arc;\n\
            use winit::{{\n    \
                event::*,\n    \
                event_loop::EventLoop,\n    \
                window::Window,\n\
            }};\n\
            \n\
            const SHADER_SOURCE: &str = r#\"{}\"#;\n\
            \n\
            #[repr(C)]\n\
            #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]\n\
            struct Vertex {{\n    \
                position: [f32; 3],\n    \
                color: [f32; 3],\n\
            }}\n\
            \n\
            struct State {{\n    \
                surface: wgpu::Surface<'static>,\n    \
                device: wgpu::Device,\n    \
                queue: wgpu::Queue,\n    \
                config: wgpu::SurfaceConfiguration,\n    \
                size: winit::dpi::PhysicalSize<u32>,\n    \
                window: Arc<Window>,\n    \
                pipeline: wgpu::RenderPipeline,\n    \
                vertex_buffer: wgpu::Buffer,\n\
            }}\n\
            \n\
            impl State {{\n    \
                async fn new(window: Arc<Window>) -> Self {{\n        \
                    let size = window.inner_size();\n\
                    \n        \
                    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {{\n            \
                        backends: wgpu::Backends::all(),\n            \
                        ..Default::default()\n        \
                    }});\n\
                    \n        \
                    let surface = instance.create_surface(window.clone()).unwrap();\n\
                    \n        \
                    let adapter = instance\n            \
                        .request_adapter(&wgpu::RequestAdapterOptions {{\n                \
                            power_preference: wgpu::PowerPreference::default(),\n                \
                            compatible_surface: Some(&surface),\n                \
                            force_fallback_adapter: false,\n            \
                        }})\n            \
                        .await\n            \
                        .unwrap();\n\
                    \n        \
                    let (device, queue) = adapter\n            \
                        .request_device(\n                \
                            &wgpu::DeviceDescriptor {{\n                    \
                                required_features: wgpu::Features::empty(),\n                    \
                                required_limits: wgpu::Limits::default(),\n                    \
                                label: Some(\"Device\"),\n                    \
                                memory_hints: Default::default(),\n                    \
                                experimental_features: Default::default(),\n                    \
                                trace: wgpu::Trace::Off,\n                \
                            }}\n            \
                        )\n            \
                        .await\n            \
                        .unwrap();\n\
                    \n        \
                    let surface_caps = surface.get_capabilities(&adapter);\n        \
                    let surface_format = surface_caps\n            \
                        .formats\n            \
                        .iter()\n            \
                        .find(|f| f.is_srgb())\n            \
                        .copied()\n            \
                        .unwrap_or(surface_caps.formats[0]);\n\
                    \n        \
                    let config = wgpu::SurfaceConfiguration {{\n            \
                        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,\n            \
                        format: surface_format,\n            \
                        width: size.width,\n            \
                        height: size.height,\n            \
                        present_mode: wgpu::PresentMode::Fifo,\n            \
                        alpha_mode: surface_caps.alpha_modes[0],\n            \
                        view_formats: vec![],\n            \
                        desired_maximum_frame_latency: 2,\n        \
                    }};\n        \
                    surface.configure(&device, &config);\n\
                    \n        \
                    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {{\n            \
                        label: Some(\"Shader\"),\n            \
                        source: wgpu::ShaderSource::Wgsl(SHADER_SOURCE.into()),\n        \
                    }});\n\
                    \n        \
                    let vertices = [\n            \
                        Vertex {{\n                \
                            position: [0.0, 0.5, 0.0],\n                \
                            color: [1.0, 0.0, 0.0],\n            \
                        }},\n            \
                        Vertex {{\n                \
                            position: [-0.5, -0.5, 0.0],\n                \
                            color: [0.0, 1.0, 0.0],\n            \
                        }},\n            \
                        Vertex {{\n                \
                            position: [0.5, -0.5, 0.0],\n                \
                            color: [0.0, 0.0, 1.0],\n            \
                        }},\n        \
                    ];\n\
                    \n        \
                    let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {{\n            \
                        label: Some(\"Vertex Buffer\"),\n            \
                        size: std::mem::size_of_val(&vertices) as u64,\n            \
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,\n            \
                        mapped_at_creation: false,\n        \
                    }});\n\
                    \n        \
                    queue.write_buffer(&vertex_buffer, 0, bytemuck::cast_slice(&vertices));\n\
                    \n        \
                    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {{\n            \
                        label: Some(\"Pipeline Layout\"),\n            \
                        bind_group_layouts: &[],\n            \
                        push_constant_ranges: &[],\n        \
                    }});\n\
                    \n        \
                    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {{\n            \
                        label: Some(\"Render Pipeline\"),\n            \
                        layout: Some(&pipeline_layout),\n            \
                        vertex: wgpu::VertexState {{\n                \
                            module: &shader,\n                \
                            entry_point: Some(\"vs_main\"),\n                \
                            buffers: &[wgpu::VertexBufferLayout {{\n                    \
                                array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,\n                    \
                                step_mode: wgpu::VertexStepMode::Vertex,\n                    \
                                attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],\n                \
                            }}],\n                \
                            compilation_options: Default::default(),\n            \
                        }},\n            \
                        fragment: Some(wgpu::FragmentState {{\n                \
                            module: &shader,\n                \
                            entry_point: Some(\"fs_main\"),\n                \
                            targets: &[Some(wgpu::ColorTargetState {{\n                    \
                                format: config.format,\n                    \
                                blend: Some(wgpu::BlendState::REPLACE),\n                    \
                                write_mask: wgpu::ColorWrites::ALL,\n                \
                            }})],\n                \
                            compilation_options: Default::default(),\n            \
                        }}),\n            \
                        primitive: wgpu::PrimitiveState {{\n                \
                            topology: wgpu::PrimitiveTopology::TriangleList,\n                \
                            strip_index_format: None,\n                \
                            front_face: wgpu::FrontFace::Ccw,\n                \
                            cull_mode: Some(wgpu::Face::Back),\n                \
                            polygon_mode: wgpu::PolygonMode::Fill,\n                \
                            unclipped_depth: false,\n                \
                            conservative: false,\n            \
                        }},\n            \
                        depth_stencil: None,\n            \
                        multisample: wgpu::MultisampleState {{\n                \
                            count: 1,\n                \
                            mask: !0,\n                \
                            alpha_to_coverage_enabled: false,\n            \
                        }},\n            \
                        multiview: None,\n            \
                        cache: None,\n        \
                    }});\n\
                    \n        \
                    Self {{\n            \
                        window,\n            \
                        surface,\n            \
                        device,\n            \
                        queue,\n            \
                        config,\n            \
                        size,\n            \
                        pipeline,\n            \
                        vertex_buffer,\n        \
                    }}\n    \
                }}\n\
                \n    \
                fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {{\n        \
                    if new_size.width > 0 && new_size.height > 0 {{\n            \
                        self.size = new_size;\n            \
                        self.config.width = new_size.width;\n            \
                        self.config.height = new_size.height;\n            \
                        self.surface.configure(&self.device, &self.config);\n        \
                    }}\n    \
                }}\n\
                \n    \
                fn render(&mut self) -> Result<(), wgpu::SurfaceError> {{\n        \
                    let output = self.surface.get_current_texture()?;\n        \
                    let view = output\n            \
                        .texture\n            \
                        .create_view(&wgpu::TextureViewDescriptor::default());\n\
                    \n        \
                    let mut encoder = self\n            \
                        .device\n            \
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {{\n                \
                            label: Some(\"Render Encoder\"),\n            \
                        }});\n\
                    \n        \
                    {{\n            \
                        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {{\n                \
                            label: Some(\"Render Pass\"),\n                \
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {{\n                    \
                                view: &view,\n                    \
                                resolve_target: None,\n                    \
                                ops: wgpu::Operations {{\n                        \
                                    load: wgpu::LoadOp::Clear(wgpu::Color {{\n                            \
                                        r: {:.2},\n                            \
                                        g: {:.2},\n                            \
                                        b: {:.2},\n                            \
                                        a: {:.2},\n                        \
                                    }}),\n                        \
                                    store: wgpu::StoreOp::Store,\n                    \
                                }},\n                    \
                                depth_slice: None,\n                \
                            }})],\n                \
                            depth_stencil_attachment: None,\n                \
                            occlusion_query_set: None,\n                \
                            timestamp_writes: None,\n            \
                        }});\n\
                        \n            \
                        render_pass.set_pipeline(&self.pipeline);\n            \
                        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));\n            \
                        render_pass.draw(0..3, 0..1);\n        \
                    }}\n\
                    \n        \
                    self.queue.submit(std::iter::once(encoder.finish()));\n        \
                    output.present();\n\
                    \n        \
                    Ok(())\n    \
                }}\n\
            }}\n\
            \n\
            fn main() {{\n    \
                env_logger::init();\n    \
                \n    \
                let event_loop = EventLoop::new().unwrap();\n    \
                let window_attributes = Window::default_attributes()\n        \
                    .with_title(\"{}\")\n        \
                    .with_inner_size(winit::dpi::PhysicalSize::new({}, {}));\n    \
                \n    \
                let window = Arc::new(\n        \
                    event_loop.create_window(window_attributes).unwrap()\n    \
                );\n\
                \n    \
                let mut state = pollster::block_on(State::new(window.clone()));\n\
                \n    \
                event_loop\n        \
                    .run(move |event, control_flow| match event {{\n            \
                        winit::event::Event::WindowEvent {{\n                \
                            ref event,\n                \
                            window_id,\n            \
                        }} if window_id == state.window.id() => match event {{\n                \
                            WindowEvent::CloseRequested => control_flow.exit(),\n                \
                            WindowEvent::Resized(physical_size) => {{\n                    \
                                state.resize(*physical_size);\n                \
                            }}\n                \
                            WindowEvent::RedrawRequested => {{\n                    \
                                match state.render() {{\n                        \
                                    Ok(_) => {{}},\n                        \
                                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),\n                        \
                                    Err(wgpu::SurfaceError::OutOfMemory) => control_flow.exit(),\n                        \
                                    Err(e) => eprintln!(\"{{:?}}\", e),\n                    \
                                }}\n                \
                            }}\n                \
                            _ => {{}}\n            \
                        }},\n            \
                        winit::event::Event::AboutToWait => {{\n                \
                            state.window.request_redraw();\n            \
                        }}\n            \
                        _ => {{}}\n        \
                    }})\n        \
                    .unwrap();\n\
            }}\n",
            shader_source,
            self.config.clear_color[0],
            self.config.clear_color[1],
            self.config.clear_color[2],
            self.config.clear_color[3],
            self.config.project_name,
            self.config.canvas_width,
            self.config.canvas_height
        )
    }

    /// Generate cube example main.rs
    fn generate_cube_main(&self) -> String {
        // For now, similar to triangle but we can expand this later
        self.generate_triangle_main()
    }

    /// Generate custom shader main.rs
    fn generate_custom_main(&self) -> String {
        self.generate_triangle_main()
    }

    /// Generate shader file
    fn generate_shader_file(
        &self,
        output_dir: &Path,
        shader_source: &str,
    ) -> Result<(), std::io::Error> {
        let shaders_dir = output_dir.join("shaders");
        std::fs::create_dir_all(&shaders_dir)?;
        std::fs::write(shaders_dir.join("shader.wgsl"), shader_source)
    }

    /// Generate README.md file
    fn generate_readme(&self, output_dir: &Path) -> Result<(), std::io::Error> {
        let content = format!(
            "# {}\n\
            \n\
            Standalone WebGPU project generated from wgpu_playground.\n\
            \n\
            ## Building\n\
            \n\
            ```bash\n\
            cargo build --release\n\
            ```\n\
            \n\
            ## Running\n\
            \n\
            ```bash\n\
            cargo run --release\n\
            ```\n\
            \n\
            ## Requirements\n\
            \n\
            - Rust (latest stable version)\n\
            - A GPU with WebGPU support (Vulkan, Metal, or DirectX 12)\n\
            \n\
            ## Note\n\
            \n\
            This code uses winit 0.30. If you encounter compilation errors,\n\
            you may need to update the event loop code to use ApplicationHandler.\n\
            See the winit 0.30 migration guide for details.\n\
            \n\
            ## Generated Configuration\n\
            \n\
            - Canvas size: {}x{}\n\
            - Clear color: RGB({:.2}, {:.2}, {:.2})\n\
            - Example type: {:?}\n\
            \n\
            ---\n\
            \n\
            Generated by wgpu_playground\n",
            self.config.project_name,
            self.config.canvas_width,
            self.config.canvas_height,
            self.config.clear_color[0],
            self.config.clear_color[1],
            self.config.clear_color[2],
            self.config.example_type
        );

        std::fs::write(output_dir.join("README.md"), content)
    }

    /// Generate main.rs from playground state
    fn generate_playground_main(&self, state: &PlaygroundState) -> String {
        let mut code = String::new();

        // Add imports
        code.push_str(
            "use std::sync::Arc;\n\
            use winit::{\n    \
                event::*,\n    \
                event_loop::EventLoop,\n    \
                window::Window,\n\
            };\n\
            \n",
        );

        // Add shader source if available
        if let Some(ref shader_editor) = state.shader_editor {
            code.push_str(&format!(
                "const SHADER_SOURCE: &str = r#\"{}\"#;\n\n",
                shader_editor.source_code
            ));
        }

        // Add State struct
        code.push_str(&self.generate_state_struct(state));

        // Add State implementation
        code.push_str(&self.generate_state_impl(state));

        // Add main function
        code.push_str(&self.generate_main_function(state));

        code
    }

    /// Generate State struct based on playground configuration
    fn generate_state_struct(&self, state: &PlaygroundState) -> String {
        let mut code = String::from("struct State {\n");
        code.push_str("    surface: wgpu::Surface<'static>,\n");
        code.push_str("    device: wgpu::Device,\n");
        code.push_str("    queue: wgpu::Queue,\n");
        code.push_str("    config: wgpu::SurfaceConfiguration,\n");
        code.push_str("    size: winit::dpi::PhysicalSize<u32>,\n");
        code.push_str("    window: Arc<Window>,\n");

        // Add fields based on configuration
        if state.buffer_panel.is_some() {
            code.push_str("    buffer: wgpu::Buffer,\n");
        }

        if state.texture_panel.is_some() {
            code.push_str("    texture: wgpu::Texture,\n");
            code.push_str("    texture_view: wgpu::TextureView,\n");
        }

        if state.sampler_panel.is_some() {
            code.push_str("    sampler: wgpu::Sampler,\n");
        }

        if state.shader_editor.is_some() {
            code.push_str("    shader_module: wgpu::ShaderModule,\n");
        }

        if state.render_pipeline_panel.is_some() {
            code.push_str("    render_pipeline: wgpu::RenderPipeline,\n");
        }

        if state.compute_pipeline_panel.is_some() {
            code.push_str("    compute_pipeline: wgpu::ComputePipeline,\n");
        }

        code.push_str("}\n\n");
        code
    }

    /// Generate State implementation
    fn generate_state_impl(&self, state: &PlaygroundState) -> String {
        let mut code =
            String::from("impl State {\n    async fn new(window: Arc<Window>) -> Self {\n");

        // Basic setup
        code.push_str(
            "        let size = window.inner_size();\n\
            \n        \
            let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {\n            \
                backends: wgpu::Backends::all(),\n            \
                ..Default::default()\n        \
            });\n\
            \n        \
            let surface = instance.create_surface(window.clone()).unwrap();\n\
            \n        \
            let adapter = instance\n            \
                .request_adapter(&wgpu::RequestAdapterOptions {\n                \
                    power_preference: wgpu::PowerPreference::default(),\n                \
                    compatible_surface: Some(&surface),\n                \
                    force_fallback_adapter: false,\n            \
                })\n            \
                .await\n            \
                .unwrap();\n\
            \n        \
            let (device, queue) = adapter\n            \
                .request_device(\n                \
                    &wgpu::DeviceDescriptor {\n                        \
                        required_features: wgpu::Features::empty(),\n                        \
                        required_limits: wgpu::Limits::default(),\n                        \
                        label: Some(\"Device\"),\n                        \
                        memory_hints: Default::default(),\n                        \
                        experimental_features: Default::default(),\n                        \
                        trace: wgpu::Trace::Off,\n                    \
                    },\n            \
                )\n            \
                .await\n            \
                .unwrap();\n\
            \n        \
            let surface_caps = surface.get_capabilities(&adapter);\n        \
            let surface_format = surface_caps\n            \
                .formats\n            \
                .iter()\n            \
                .find(|f| f.is_srgb())\n            \
                .copied()\n            \
                .unwrap_or(surface_caps.formats[0]);\n\
            \n        \
            let config = wgpu::SurfaceConfiguration {\n            \
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,\n            \
                format: surface_format,\n            \
                width: size.width,\n            \
                height: size.height,\n            \
                present_mode: wgpu::PresentMode::Fifo,\n            \
                alpha_mode: surface_caps.alpha_modes[0],\n            \
                view_formats: vec![],\n            \
                desired_maximum_frame_latency: 2,\n        \
            };\n        \
            surface.configure(&device, &config);\n\n",
        );

        // Add buffer creation if configured
        if let Some(ref buffer_state) = state.buffer_panel {
            code.push_str(&self.generate_buffer_creation(buffer_state));
        }

        // Add texture creation if configured
        if let Some(ref texture_state) = state.texture_panel {
            code.push_str(&self.generate_texture_creation(texture_state));
        }

        // Add sampler creation if configured
        if let Some(ref sampler_state) = state.sampler_panel {
            code.push_str(&self.generate_sampler_creation(sampler_state));
        }

        // Add shader module creation if configured
        if let Some(ref shader_state) = state.shader_editor {
            code.push_str(&self.generate_shader_module_creation(shader_state));
        }

        // Add render pipeline creation if configured
        if let Some(ref pipeline_state) = state.render_pipeline_panel {
            code.push_str(&self.generate_render_pipeline_creation(pipeline_state));
        }

        // Add compute pipeline creation if configured
        if let Some(ref compute_state) = state.compute_pipeline_panel {
            code.push_str(&self.generate_compute_pipeline_creation(compute_state));
        }

        // Return State instance
        code.push_str("        Self {\n");
        code.push_str("            window,\n");
        code.push_str("            surface,\n");
        code.push_str("            device,\n");
        code.push_str("            queue,\n");
        code.push_str("            config,\n");
        code.push_str("            size,\n");

        if state.buffer_panel.is_some() {
            code.push_str("            buffer,\n");
        }
        if state.texture_panel.is_some() {
            code.push_str("            texture,\n");
            code.push_str("            texture_view,\n");
        }
        if state.sampler_panel.is_some() {
            code.push_str("            sampler,\n");
        }
        if state.shader_editor.is_some() {
            code.push_str("            shader_module,\n");
        }
        if state.render_pipeline_panel.is_some() {
            code.push_str("            render_pipeline,\n");
        }
        if state.compute_pipeline_panel.is_some() {
            code.push_str("            compute_pipeline,\n");
        }

        code.push_str("        }\n");
        code.push_str("    }\n");

        // Add resize method
        code.push_str(
            "\n    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {\n        \
                if new_size.width > 0 && new_size.height > 0 {\n            \
                    self.size = new_size;\n            \
                    self.config.width = new_size.width;\n            \
                    self.config.height = new_size.height;\n            \
                    self.surface.configure(&self.device, &self.config);\n        \
                }\n    \
            }\n",
        );

        // Add render method
        code.push_str(&self.generate_render_method(state));

        code.push_str("}\n\n");
        code
    }

    /// Generate buffer creation code
    fn generate_buffer_creation(&self, buffer_state: &BufferPanelState) -> String {
        let size = buffer_state
            .size
            .parse::<u64>()
            .inspect_err(|e| {
                log::warn!(
                    "Failed to parse buffer size '{}': {}. Using default {}.",
                    buffer_state.size,
                    e,
                    DEFAULT_BUFFER_SIZE
                )
            })
            .unwrap_or(DEFAULT_BUFFER_SIZE);
        let mut usage_flags = Vec::new();

        if buffer_state.usage_vertex {
            usage_flags.push("wgpu::BufferUsages::VERTEX");
        }
        if buffer_state.usage_index {
            usage_flags.push("wgpu::BufferUsages::INDEX");
        }
        if buffer_state.usage_uniform {
            usage_flags.push("wgpu::BufferUsages::UNIFORM");
        }
        if buffer_state.usage_storage {
            usage_flags.push("wgpu::BufferUsages::STORAGE");
        }
        if buffer_state.usage_copy_src {
            usage_flags.push("wgpu::BufferUsages::COPY_SRC");
        }
        if buffer_state.usage_copy_dst {
            usage_flags.push("wgpu::BufferUsages::COPY_DST");
        }

        let usage = if usage_flags.is_empty() {
            "wgpu::BufferUsages::VERTEX".to_string()
        } else {
            usage_flags.join(" | ")
        };

        format!(
            "        let buffer = device.create_buffer(&wgpu::BufferDescriptor {{\n            \
                label: Some(\"{}\"),\n            \
                size: {},\n            \
                usage: {},\n            \
                mapped_at_creation: {},\n        \
            }});\n\n",
            buffer_state.label, size, usage, buffer_state.mapped_at_creation
        )
    }

    /// Generate texture creation code
    fn generate_texture_creation(&self, texture_state: &TexturePanelState) -> String {
        let width = texture_state
            .width
            .parse::<u32>()
            .inspect_err(|e| {
                log::warn!(
                    "Failed to parse texture width '{}': {}. Using default {}.",
                    texture_state.width,
                    e,
                    DEFAULT_TEXTURE_WIDTH
                )
            })
            .unwrap_or(DEFAULT_TEXTURE_WIDTH);
        let height = texture_state
            .height
            .parse::<u32>()
            .inspect_err(|e| {
                log::warn!(
                    "Failed to parse texture height '{}': {}. Using default {}.",
                    texture_state.height,
                    e,
                    DEFAULT_TEXTURE_HEIGHT
                )
            })
            .unwrap_or(DEFAULT_TEXTURE_HEIGHT);
        let depth = texture_state
            .depth
            .parse::<u32>()
            .inspect_err(|e| {
                log::warn!(
                    "Failed to parse texture depth '{}': {}. Using default {}.",
                    texture_state.depth,
                    e,
                    DEFAULT_TEXTURE_DEPTH
                )
            })
            .unwrap_or(DEFAULT_TEXTURE_DEPTH);

        let mut usage_flags = Vec::new();
        if texture_state.usage_texture_binding {
            usage_flags.push("wgpu::TextureUsages::TEXTURE_BINDING");
        }
        if texture_state.usage_copy_dst {
            usage_flags.push("wgpu::TextureUsages::COPY_DST");
        }
        if texture_state.usage_render_attachment {
            usage_flags.push("wgpu::TextureUsages::RENDER_ATTACHMENT");
        }

        let usage = if usage_flags.is_empty() {
            "wgpu::TextureUsages::TEXTURE_BINDING".to_string()
        } else {
            usage_flags.join(" | ")
        };

        format!(
            "        let texture = device.create_texture(&wgpu::TextureDescriptor {{\n            \
                label: Some(\"{}\"),\n            \
                size: wgpu::Extent3d {{\n                \
                    width: {},\n                \
                    height: {},\n                \
                    depth_or_array_layers: {},\n            \
                }},\n            \
                mip_level_count: 1,\n            \
                sample_count: 1,\n            \
                dimension: wgpu::TextureDimension::D2,\n            \
                format: wgpu::TextureFormat::Rgba8UnormSrgb,\n            \
                usage: {},\n            \
                view_formats: &[],\n        \
            }});\n\
            \n        \
            let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());\n\n",
            texture_state.label, width, height, depth, usage
        )
    }

    /// Generate sampler creation code
    fn generate_sampler_creation(&self, sampler_state: &SamplerPanelState) -> String {
        format!(
            "        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {{\n            \
                label: Some(\"{}\"),\n            \
                address_mode_u: wgpu::AddressMode::Repeat,\n            \
                address_mode_v: wgpu::AddressMode::Repeat,\n            \
                address_mode_w: wgpu::AddressMode::Repeat,\n            \
                mag_filter: wgpu::FilterMode::Linear,\n            \
                min_filter: wgpu::FilterMode::Linear,\n            \
                mipmap_filter: wgpu::FilterMode::Linear,\n            \
                ..Default::default()\n        \
            }});\n\n",
            sampler_state.label
        )
    }

    /// Generate shader module creation code
    fn generate_shader_module_creation(&self, shader_state: &ShaderEditorState) -> String {
        format!(
            "        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {{\n            \
                label: Some(\"{}\"),\n            \
                source: wgpu::ShaderSource::Wgsl(SHADER_SOURCE.into()),\n        \
            }});\n\n",
            shader_state.label
        )
    }

    /// Generate render pipeline creation code
    fn generate_render_pipeline_creation(
        &self,
        pipeline_state: &RenderPipelinePanelState,
    ) -> String {
        format!(
            "        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {{\n            \
                label: Some(\"Pipeline Layout\"),\n            \
                bind_group_layouts: &[],\n            \
                push_constant_ranges: &[],\n        \
            }});\n\
            \n        \
            let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {{\n            \
                label: Some(\"{}\"),\n            \
                layout: Some(&pipeline_layout),\n            \
                vertex: wgpu::VertexState {{\n                \
                    module: &shader_module,\n                \
                    entry_point: Some(\"{}\"),\n                \
                    buffers: &[],\n                \
                    compilation_options: Default::default(),\n            \
                }},\n            \
                fragment: Some(wgpu::FragmentState {{\n                \
                    module: &shader_module,\n                \
                    entry_point: Some(\"{}\"),\n                \
                    targets: &[Some(wgpu::ColorTargetState {{\n                        \
                        format: config.format,\n                        \
                        blend: Some(wgpu::BlendState::REPLACE),\n                        \
                        write_mask: wgpu::ColorWrites::ALL,\n                    \
                    }})],\n                \
                    compilation_options: Default::default(),\n            \
                }}),\n            \
                primitive: wgpu::PrimitiveState {{\n                \
                    topology: wgpu::PrimitiveTopology::TriangleList,\n                \
                    ..Default::default()\n            \
                }},\n            \
                depth_stencil: None,\n            \
                multisample: wgpu::MultisampleState::default(),\n            \
                multiview: None,\n            \
                cache: None,\n        \
            }});\n\n",
            pipeline_state.label,
            pipeline_state.vertex_entry_point,
            pipeline_state.fragment_entry_point
        )
    }

    /// Generate compute pipeline creation code
    fn generate_compute_pipeline_creation(
        &self,
        compute_state: &ComputePipelinePanelState,
    ) -> String {
        format!(
            "        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {{\n            \
                label: Some(\"Compute Pipeline Layout\"),\n            \
                bind_group_layouts: &[],\n            \
                push_constant_ranges: &[],\n        \
            }});\n\
            \n        \
            let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {{\n            \
                label: Some(\"{}\"),\n            \
                layout: Some(&pipeline_layout),\n            \
                module: &shader_module,\n            \
                entry_point: Some(\"{}\"),\n            \
                compilation_options: Default::default(),\n            \
                cache: None,\n        \
            }});\n\n",
            compute_state.label, compute_state.entry_point
        )
    }

    /// Generate render method
    fn generate_render_method(&self, state: &PlaygroundState) -> String {
        let mut code = String::from(
            "\n    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {\n        \
                let output = self.surface.get_current_texture()?;\n        \
                let view = output\n            \
                    .texture\n            \
                    .create_view(&wgpu::TextureViewDescriptor::default());\n\
                \n        \
                let mut encoder = self\n            \
                    .device\n            \
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {\n                \
                        label: Some(\"Render Encoder\"),\n            \
                    });\n\n",
        );

        // Add render pass if render pipeline exists
        if state.render_pipeline_panel.is_some() {
            code.push_str(&format!(
                "        {{\n            \
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {{\n                \
                        label: Some(\"Render Pass\"),\n                \
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {{\n                    \
                            view: &view,\n                    \
                            resolve_target: None,\n                    \
                            ops: wgpu::Operations {{\n                        \
                                load: wgpu::LoadOp::Clear(wgpu::Color {{\n                            \
                                    r: {:.2},\n                            \
                                    g: {:.2},\n                            \
                                    b: {:.2},\n                            \
                                    a: {:.2},\n                        \
                                }}),\n                        \
                                store: wgpu::StoreOp::Store,\n                    \
                            }},\n                    \
                            depth_slice: None,\n                \
                        }})],\n                \
                        depth_stencil_attachment: None,\n                \
                        occlusion_query_set: None,\n                \
                        timestamp_writes: None,\n            \
                    }});\n\
                    \n            \
                    render_pass.set_pipeline(&self.render_pipeline);\n            \
                    render_pass.draw(0..3, 0..1);\n        \
                }}\n\n",
                self.config.clear_color[0],
                self.config.clear_color[1],
                self.config.clear_color[2],
                self.config.clear_color[3]
            ));
        }

        code.push_str(
            "        self.queue.submit(std::iter::once(encoder.finish()));\n        \
            output.present();\n        \
            \n        \
            Ok(())\n    \
            }\n",
        );

        code
    }

    /// Generate main function
    /// Note: playground_state is reserved for future use (e.g., window title customization)
    fn generate_main_function(&self, _playground_state: &PlaygroundState) -> String {
        format!(
            "fn main() {{\n    \
                env_logger::init();\n    \
                \n    \
                let event_loop = EventLoop::new().unwrap();\n    \
                let window_attributes = Window::default_attributes()\n        \
                    .with_title(\"{}\")\n        \
                    .with_inner_size(winit::dpi::PhysicalSize::new({}, {}));\n    \
                \n    \
                let window = Arc::new(\n        \
                    event_loop.create_window(window_attributes).unwrap()\n    \
                );\n\
                \n    \
                let mut state = pollster::block_on(State::new(window.clone()));\n\
                \n    \
                event_loop\n        \
                    .run(move |event, control_flow| match event {{\n            \
                        winit::event::Event::WindowEvent {{\n                \
                            ref event,\n                \
                            window_id,\n            \
                        }} if window_id == state.window.id() => match event {{\n                \
                            WindowEvent::CloseRequested => control_flow.exit(),\n                \
                            WindowEvent::Resized(physical_size) => {{\n                    \
                                state.resize(*physical_size);\n                \
                            }}\n                \
                            WindowEvent::RedrawRequested => {{\n                    \
                                match state.render() {{\n                        \
                                    Ok(_) => {{}},\n                        \
                                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),\n                        \
                                    Err(wgpu::SurfaceError::OutOfMemory) => control_flow.exit(),\n                        \
                                    Err(e) => eprintln!(\"{{:?}}\", e),\n                    \
                                }}\n                \
                            }}\n                \
                            _ => {{}}\n            \
                        }},\n            \
                        winit::event::Event::AboutToWait => {{\n                \
                            state.window.request_redraw();\n            \
                        }}\n            \
                        _ => {{}}\n        \
                    }})\n        \
                    .unwrap();\n\
            }}\n",
            self.config.project_name, self.config.canvas_width, self.config.canvas_height
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_code_gen_config_default() {
        let config = CodeGenConfig::default();
        assert_eq!(config.project_name, "wgpu_standalone");
        assert_eq!(config.example_type, ExampleType::Triangle);
        assert_eq!(config.canvas_width, 800);
        assert_eq!(config.canvas_height, 600);
    }

    #[test]
    fn test_code_gen_config_builder() {
        let config = CodeGenConfig::new("my_project".to_string())
            .with_canvas_size(1024, 768)
            .with_clear_color([0.5, 0.5, 0.5, 1.0])
            .with_example_type(ExampleType::Cube);

        assert_eq!(config.project_name, "my_project");
        assert_eq!(config.canvas_width, 1024);
        assert_eq!(config.canvas_height, 768);
        assert_eq!(config.clear_color, [0.5, 0.5, 0.5, 1.0]);
        assert_eq!(config.example_type, ExampleType::Cube);
    }

    #[test]
    fn test_generate_project_structure() {
        let temp_dir = std::env::temp_dir().join("wgpu_test_project");

        // Clean up if exists
        let _ = fs::remove_dir_all(&temp_dir);

        let config = CodeGenConfig::new("test_project".to_string());
        let generator = CodeGenerator::new(config);

        generator.generate(&temp_dir).unwrap();

        // Verify files were created
        assert!(temp_dir.join("Cargo.toml").exists());
        assert!(temp_dir.join("src").join("main.rs").exists());
        assert!(temp_dir.join("README.md").exists());

        // Clean up
        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_cargo_toml_generation() {
        let temp_dir = std::env::temp_dir().join("wgpu_test_cargo");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let config = CodeGenConfig::new("my_app".to_string());
        let generator = CodeGenerator::new(config);
        generator.generate_cargo_toml(&temp_dir).unwrap();

        let cargo_toml = fs::read_to_string(temp_dir.join("Cargo.toml")).unwrap();
        assert!(cargo_toml.contains("name = \"my_app\""));
        assert!(cargo_toml.contains("wgpu = \"27.0\""));
        assert!(cargo_toml.contains("winit = \"0.30\""));

        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_playground_state_export() {
        let temp_dir = std::env::temp_dir().join("wgpu_test_playground");
        let _ = fs::remove_dir_all(&temp_dir);

        // Create a playground state with shader
        let shader_code = "@vertex\nfn vs_main() -> @builtin(position) vec4<f32> {\n    return vec4<f32>(0.0, 0.0, 0.0, 1.0);\n}\n\n@fragment\nfn fs_main() -> @location(0) vec4<f32> {\n    return vec4<f32>(1.0, 0.0, 0.0, 1.0);\n}";

        let playground_state = PlaygroundState {
            version: "1.0".to_string(),
            theme: crate::state::Theme::Dark,
            shader_editor: Some(ShaderEditorState {
                source_code: shader_code.to_string(),
                label: "test_shader".to_string(),
                file_path: "shader.wgsl".to_string(),
            }),
            buffer_panel: Some(BufferPanelState {
                label: "vertex_buffer".to_string(),
                size: "1024".to_string(),
                usage_vertex: true,
                usage_index: false,
                usage_uniform: false,
                usage_storage: false,
                usage_indirect: false,
                usage_copy_src: false,
                usage_copy_dst: true,
                usage_map_read: false,
                usage_map_write: false,
                usage_query_resolve: false,
                mapped_at_creation: false,
            }),
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

        let config = CodeGenConfig::new("playground_export".to_string())
            .with_playground_state(playground_state);

        let generator = CodeGenerator::new(config);
        generator.generate(&temp_dir).unwrap();

        // Verify files were created
        assert!(temp_dir.join("Cargo.toml").exists());
        assert!(temp_dir.join("src").join("main.rs").exists());
        assert!(temp_dir.join("README.md").exists());

        // Verify main.rs contains the shader
        let main_rs = fs::read_to_string(temp_dir.join("src").join("main.rs")).unwrap();
        assert!(main_rs.contains("vs_main"));
        assert!(main_rs.contains("fs_main"));
        assert!(main_rs.contains("vertex_buffer"));

        // Clean up
        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_export_with_buffer_and_texture() {
        let temp_dir = std::env::temp_dir().join("wgpu_test_full_export");
        let _ = fs::remove_dir_all(&temp_dir);

        let playground_state = PlaygroundState {
            version: "1.0".to_string(),
            theme: crate::state::Theme::Dark,
            shader_editor: Some(ShaderEditorState {
                source_code: "@vertex\nfn main() {}".to_string(),
                label: "shader".to_string(),
                file_path: "shader.wgsl".to_string(),
            }),
            buffer_panel: Some(BufferPanelState {
                label: "my_buffer".to_string(),
                size: "256".to_string(),
                usage_vertex: true,
                usage_uniform: true,
                usage_index: false,
                usage_storage: false,
                usage_indirect: false,
                usage_copy_src: false,
                usage_copy_dst: false,
                usage_map_read: false,
                usage_map_write: false,
                usage_query_resolve: false,
                mapped_at_creation: false,
            }),
            texture_panel: Some(TexturePanelState {
                label: "my_texture".to_string(),
                width: "512".to_string(),
                height: "512".to_string(),
                depth: "1".to_string(),
                mip_levels: "1".to_string(),
                sample_count: "1".to_string(),
                format: "Rgba8Unorm".to_string(),
                dimension: "D2".to_string(),
                usage_copy_src: false,
                usage_copy_dst: true,
                usage_texture_binding: true,
                usage_storage_binding: false,
                usage_render_attachment: false,
            }),
            sampler_panel: Some(SamplerPanelState {
                label: "my_sampler".to_string(),
                address_mode_u: "Repeat".to_string(),
                address_mode_v: "Repeat".to_string(),
                address_mode_w: "Repeat".to_string(),
                mag_filter: "Linear".to_string(),
                min_filter: "Linear".to_string(),
                mipmap_filter: "Linear".to_string(),
                lod_min_clamp: "0.0".to_string(),
                lod_max_clamp: "32.0".to_string(),
                compare: None,
                max_anisotropy: "1".to_string(),
            }),
            render_pipeline_panel: None,
            compute_pipeline_panel: None,
            bind_group_panel: None,
            bind_group_layout_panel: None,
            api_coverage: None,
            tutorial_state: None,
            learning_progress: None,
        };

        let config =
            CodeGenConfig::new("full_export".to_string()).with_playground_state(playground_state);

        let generator = CodeGenerator::new(config);
        generator.generate(&temp_dir).unwrap();

        // Verify main.rs contains all components
        let main_rs = fs::read_to_string(temp_dir.join("src").join("main.rs")).unwrap();
        assert!(main_rs.contains("my_buffer"));
        assert!(main_rs.contains("my_texture"));
        assert!(main_rs.contains("my_sampler"));
        assert!(main_rs.contains("wgpu::BufferUsages::VERTEX"));
        assert!(main_rs.contains("wgpu::BufferUsages::UNIFORM"));

        // Clean up
        fs::remove_dir_all(&temp_dir).unwrap();
    }
}
