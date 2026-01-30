pub struct DeviceInfo {
    adapter_name: String,
    backend: String,
    adapter_info: String,
    device_limits: String,
    device_features: String,
}

impl DeviceInfo {
    pub fn new(adapter: &wgpu::Adapter, device: &wgpu::Device) -> Self {
        let info = adapter.get_info();
        let backend = crate::adapter::backend_to_str(&info.backend).to_string();
        let adapter_name = info.name.clone();

        let adapter_info = format!(
            "Name: {}\nVendor: {}\nDevice: {}\nDevice Type: {:?}\nDriver: {}\nDriver Info: {}\nBackend: {}",
            info.name, info.vendor, info.device, info.device_type, info.driver, info.driver_info, backend
        );

        let limits = device.limits();
        let device_limits = format!(
            "Max Texture Dimension 1D: {}\n\
            Max Texture Dimension 2D: {}\n\
            Max Texture Dimension 3D: {}\n\
            Max Texture Array Layers: {}\n\
            Max Bind Groups: {}\n\
            Max Bindings Per Bind Group: {}\n\
            Max Dynamic Uniform Buffers Per Pipeline Layout: {}\n\
            Max Dynamic Storage Buffers Per Pipeline Layout: {}\n\
            Max Sampled Textures Per Shader Stage: {}\n\
            Max Samplers Per Shader Stage: {}\n\
            Max Storage Buffers Per Shader Stage: {}\n\
            Max Storage Textures Per Shader Stage: {}\n\
            Max Uniform Buffers Per Shader Stage: {}\n\
            Max Uniform Buffer Binding Size: {}\n\
            Max Storage Buffer Binding Size: {}\n\
            Max Vertex Buffers: {}\n\
            Max Buffer Size: {}\n\
            Max Vertex Attributes: {}\n\
            Max Vertex Buffer Array Stride: {}\n\
            Min Uniform Buffer Offset Alignment: {}\n\
            Min Storage Buffer Offset Alignment: {}\n\
            Max Inter Stage Shader Components: {}\n\
            Max Compute Workgroup Storage Size: {}\n\
            Max Compute Invocations Per Workgroup: {}\n\
            Max Compute Workgroup Size X: {}\n\
            Max Compute Workgroup Size Y: {}\n\
            Max Compute Workgroup Size Z: {}\n\
            Max Compute Workgroups Per Dimension: {}",
            limits.max_texture_dimension_1d,
            limits.max_texture_dimension_2d,
            limits.max_texture_dimension_3d,
            limits.max_texture_array_layers,
            limits.max_bind_groups,
            limits.max_bindings_per_bind_group,
            limits.max_dynamic_uniform_buffers_per_pipeline_layout,
            limits.max_dynamic_storage_buffers_per_pipeline_layout,
            limits.max_sampled_textures_per_shader_stage,
            limits.max_samplers_per_shader_stage,
            limits.max_storage_buffers_per_shader_stage,
            limits.max_storage_textures_per_shader_stage,
            limits.max_uniform_buffers_per_shader_stage,
            limits.max_uniform_buffer_binding_size,
            limits.max_storage_buffer_binding_size,
            limits.max_vertex_buffers,
            limits.max_buffer_size,
            limits.max_vertex_attributes,
            limits.max_vertex_buffer_array_stride,
            limits.min_uniform_buffer_offset_alignment,
            limits.min_storage_buffer_offset_alignment,
            limits.max_inter_stage_shader_components,
            limits.max_compute_workgroup_storage_size,
            limits.max_compute_invocations_per_workgroup,
            limits.max_compute_workgroup_size_x,
            limits.max_compute_workgroup_size_y,
            limits.max_compute_workgroup_size_z,
            limits.max_compute_workgroups_per_dimension,
        );

        let features = device.features();
        let device_features = format!("{:?}", features);

        Self {
            adapter_name,
            backend,
            adapter_info,
            device_limits,
            device_features,
        }
    }

    pub fn ui(&self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // Highlight the backend being used
            ui.heading("🖥️ Active WebGPU Backend");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Backend Implementation:");
                ui.strong(&self.backend);
            });
            ui.horizontal(|ui| {
                ui.label("Adapter:");
                ui.strong(&self.adapter_name);
            });
            ui.add_space(10.0);
            ui.label(
                "💡 Tip: Set the WGPU_BACKEND environment variable to select a specific backend.",
            );
            ui.label("   Available: vulkan, metal, dx12, gl, primary, all");
            ui.add_space(20.0);

            ui.heading("Adapter Information");
            ui.separator();
            ui.label(&self.adapter_info);
            ui.add_space(20.0);

            ui.heading("Device Limits");
            ui.separator();
            ui.label(&self.device_limits);
            ui.add_space(20.0);

            ui.heading("Device Features");
            ui.separator();
            ui.label(&self.device_features);
        });
    }
}
