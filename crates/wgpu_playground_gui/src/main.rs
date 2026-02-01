use egui_wgpu::ScreenDescriptor;
use pollster::FutureExt;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

mod app;

use app::PlaygroundApp;

struct AppState {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,
    egui_renderer: egui_wgpu::Renderer,
    egui_state: egui_winit::State,
    egui_ctx: egui::Context,
    playground_app: PlaygroundApp,
}

impl AppState {
    async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        // Check for WGPU_BACKEND environment variable to select backend
        let backends = std::env::var("WGPU_BACKEND")
            .ok()
            .and_then(|backend_str| {
                log::info!("WGPU_BACKEND environment variable set to: {}", backend_str);
                #[allow(deprecated)]
                wgpu_playground_core::adapter::parse_backend(&backend_str)
            })
            .unwrap_or_else(|| {
                // On Windows, prefer Vulkan to avoid DirectX 12 resource state validation errors
                // that are common on AMD GPUs. These errors are cosmetic but spam the console.
                // See: https://github.com/gfx-rs/wgpu/issues/3959, https://github.com/gfx-rs/wgpu/issues/4247
                #[cfg(target_os = "windows")]
                {
                    log::info!("No WGPU_BACKEND specified. On Windows, preferring Vulkan to avoid DirectX 12 validation errors.");
                    log::info!("Set WGPU_BACKEND=dx12 to force DirectX 12 if needed.");
                    wgpu::Backends::VULKAN | wgpu::Backends::DX12
                }
                #[cfg(not(target_os = "windows"))]
                {
                    log::info!("Using all available backends");
                    wgpu::Backends::all()
                }
            });

        let instance = wgpu_playground_core::adapter::create_instance(backends);

        let surface = instance
            .create_surface(window.clone())
            .expect("Failed to create surface");

        // Use the adapter module for better error handling and configurability
        let adapter_options =
            wgpu_playground_core::adapter::AdapterOptions::default().with_backends(backends);
        let adapter = wgpu_playground_core::adapter::request_adapter(
            &instance,
            &adapter_options,
            Some(&surface),
        )
        .await
        .expect("Failed to find a suitable GPU adapter");

        log::info!(
            "Using adapter: {} (Backend: {})",
            adapter.get_info().name,
            wgpu_playground_core::adapter::backend_to_str(&adapter.get_info().backend)
        );

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: Some("WebGPU Playground Device"),
                    memory_hints: Default::default(),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        // Set up comprehensive error handling for the device
        // This configures callbacks for device loss and uncaptured errors
        wgpu_playground_core::error::setup_device_error_handling(&device);

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &surface_config);

        let egui_ctx = egui::Context::default();
        let egui_state = egui_winit::State::new(
            egui_ctx.clone(),
            egui::ViewportId::ROOT,
            &window,
            None,
            None,
            None,
        );

        let egui_renderer = egui_wgpu::Renderer::new(&device, surface_config.format, None, 1, true);

        let playground_app = PlaygroundApp::new(&adapter, &device, &queue);

        Self {
            window,
            surface,
            device,
            queue,
            surface_config,
            egui_renderer,
            egui_state,
            egui_ctx,
            playground_app,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let surface_texture = self.surface.get_current_texture()?;
        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // Clear the screen
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Clear Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.1,
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
        }

        // Run egui
        let raw_input = self.egui_state.take_egui_input(&self.window);
        let egui_output = self.egui_ctx.run(raw_input, |ctx| {
            self.playground_app.ui(ctx, &self.device, &self.queue);
        });

        self.egui_state
            .handle_platform_output(&self.window, egui_output.platform_output);

        let clipped_primitives = self
            .egui_ctx
            .tessellate(egui_output.shapes, egui_output.pixels_per_point);

        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [self.surface_config.width, self.surface_config.height],
            pixels_per_point: self.window.scale_factor() as f32,
        };

        for (id, image_delta) in &egui_output.textures_delta.set {
            self.egui_renderer
                .update_texture(&self.device, &self.queue, *id, image_delta);
        }

        self.egui_renderer.update_buffers(
            &self.device,
            &self.queue,
            &mut encoder,
            &clipped_primitives,
            &screen_descriptor,
        );

        // Render egui
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("UI Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // SAFETY: This is safe because we're extending the lifetime of the render pass
            // only for the duration of the render call, and we drop it immediately after.
            // The render pass doesn't escape this scope.
            let render_pass_static: &mut wgpu::RenderPass<'static> =
                unsafe { std::mem::transmute(&mut render_pass) };

            self.egui_renderer
                .render(render_pass_static, &clipped_primitives, &screen_descriptor);
        }

        for id in &egui_output.textures_delta.free {
            self.egui_renderer.free_texture(id);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        surface_texture.present();

        Ok(())
    }
}

struct App {
    state: Option<AppState>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_none() {
            let window_attributes = Window::default_attributes()
                .with_title("WebGPU Playground")
                .with_inner_size(winit::dpi::LogicalSize::new(1280, 720));

            let window = Arc::new(
                event_loop
                    .create_window(window_attributes)
                    .expect("Failed to create window"),
            );
            self.state = Some(AppState::new(window).block_on());
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let state = match &mut self.state {
            Some(state) => state,
            None => return,
        };

        let response = state.egui_state.on_window_event(&state.window, &event);

        if response.consumed {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                state.resize(physical_size);
            }
            WindowEvent::RedrawRequested => match state.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => state.resize(state.window.inner_size()),
                Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                Err(e) => eprintln!("Surface error: {:?}", e),
            },
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(state) = &self.state {
            state.window.request_redraw();
        }
    }
}

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().expect("Failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App { state: None };
    event_loop
        .run_app(&mut app)
        .expect("Failed to run event loop");
}
