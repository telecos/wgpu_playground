/// Code generation module for exporting playground configuration as standalone Rust project
use std::path::Path;

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

        let content = match self.config.example_type {
            ExampleType::Triangle => self.generate_triangle_main(),
            ExampleType::Cube => self.generate_cube_main(),
            ExampleType::Custom => self.generate_custom_main(),
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
                let window = Arc::new(\n        \
                    winit::window::WindowBuilder::new()\n            \
                        .with_title(\"{}\")\n            \
                        .with_inner_size(winit::dpi::PhysicalSize::new({}, {}))\n            \
                        .build(&event_loop)\n            \
                        .unwrap(),\n    \
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
}
