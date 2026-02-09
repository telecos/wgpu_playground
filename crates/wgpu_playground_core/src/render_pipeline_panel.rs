use crate::pipeline_preview::RenderPipelinePreviewState;
use crate::render_pipeline::{
    BlendComponent, BlendFactor, BlendOperation, BlendState, ColorTargetState, ColorWrites,
    CompareFunction, CullMode, DepthStencilState, FrontFace, MultisampleState, PrimitiveState,
    PrimitiveTopology, RenderPipelineDescriptor, StencilFaceState, StencilOperation,
};
use crate::tooltip::{
    blend_factor, blend_operation, compare_function, cull_mode, front_face, primitive_topology,
    property, stencil_operation,
};

/// UI panel for configuring render pipelines
pub struct RenderPipelinePanel {
    /// Current pipeline descriptor being configured
    descriptor: RenderPipelineDescriptor,
    /// Label input text
    label_input: String,

    // Vertex State
    /// Vertex entry point
    vertex_entry_point: String,
    /// Fragment entry point
    fragment_entry_point: String,

    // Primitive State
    /// Primitive topology
    topology: PrimitiveTopology,
    /// Cull mode
    cull_mode: CullMode,
    /// Front face
    front_face: FrontFace,

    // Depth-Stencil State
    /// Whether depth-stencil is enabled
    enable_depth_stencil: bool,
    /// Depth format
    depth_format: DepthFormat,
    /// Whether depth writes are enabled
    depth_write_enabled: bool,
    /// Depth comparison function
    depth_compare: CompareFunction,
    /// Stencil read mask input
    stencil_read_mask_input: String,
    /// Stencil write mask input
    stencil_write_mask_input: String,
    /// Stencil front compare
    stencil_front_compare: CompareFunction,
    /// Stencil front fail op
    stencil_front_fail_op: StencilOperation,
    /// Stencil front depth fail op
    stencil_front_depth_fail_op: StencilOperation,
    /// Stencil front pass op
    stencil_front_pass_op: StencilOperation,
    /// Stencil back compare
    stencil_back_compare: CompareFunction,
    /// Stencil back fail op
    stencil_back_fail_op: StencilOperation,
    /// Stencil back depth fail op
    stencil_back_depth_fail_op: StencilOperation,
    /// Stencil back pass op
    stencil_back_pass_op: StencilOperation,

    // Multisample State
    /// Sample count
    sample_count: u32,
    /// Alpha to coverage enabled
    alpha_to_coverage_enabled: bool,

    // Fragment State
    /// Target format
    target_format: TargetFormat,
    /// Blend enabled
    blend_enabled: bool,
    /// Color blend source factor
    color_blend_src: BlendFactor,
    /// Color blend destination factor
    color_blend_dst: BlendFactor,
    /// Color blend operation
    color_blend_op: BlendOperation,
    /// Alpha blend source factor
    alpha_blend_src: BlendFactor,
    /// Alpha blend destination factor
    alpha_blend_dst: BlendFactor,
    /// Alpha blend operation
    alpha_blend_op: BlendOperation,
    /// Color write red
    write_red: bool,
    /// Color write green
    write_green: bool,
    /// Color write blue
    write_blue: bool,
    /// Color write alpha
    write_alpha: bool,

    /// Validation error message
    validation_error: Option<String>,
    /// Success message
    success_message: Option<String>,

    /// Pipeline preview rendering state
    preview_state: Option<RenderPipelinePreviewState>,
    /// Whether preview is enabled
    show_preview: bool,
}

/// Depth format options for UI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DepthFormat {
    Depth24Plus,
    Depth32Float,
    Depth24PlusStencil8,
    Depth32FloatStencil8,
}

impl DepthFormat {
    fn to_wgpu(self) -> wgpu::TextureFormat {
        match self {
            DepthFormat::Depth24Plus => wgpu::TextureFormat::Depth24Plus,
            DepthFormat::Depth32Float => wgpu::TextureFormat::Depth32Float,
            DepthFormat::Depth24PlusStencil8 => wgpu::TextureFormat::Depth24PlusStencil8,
            DepthFormat::Depth32FloatStencil8 => wgpu::TextureFormat::Depth32FloatStencil8,
        }
    }

    fn all() -> Vec<Self> {
        vec![
            DepthFormat::Depth24Plus,
            DepthFormat::Depth32Float,
            DepthFormat::Depth24PlusStencil8,
            DepthFormat::Depth32FloatStencil8,
        ]
    }

    fn name(&self) -> &'static str {
        match self {
            DepthFormat::Depth24Plus => "Depth24Plus",
            DepthFormat::Depth32Float => "Depth32Float",
            DepthFormat::Depth24PlusStencil8 => "Depth24Plus + Stencil8",
            DepthFormat::Depth32FloatStencil8 => "Depth32Float + Stencil8",
        }
    }
}

/// Target format options for UI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TargetFormat {
    Bgra8UnormSrgb,
    Rgba8UnormSrgb,
    Bgra8Unorm,
    Rgba8Unorm,
    Rgba16Float,
}

impl TargetFormat {
    fn to_wgpu(self) -> wgpu::TextureFormat {
        match self {
            TargetFormat::Bgra8UnormSrgb => wgpu::TextureFormat::Bgra8UnormSrgb,
            TargetFormat::Rgba8UnormSrgb => wgpu::TextureFormat::Rgba8UnormSrgb,
            TargetFormat::Bgra8Unorm => wgpu::TextureFormat::Bgra8Unorm,
            TargetFormat::Rgba8Unorm => wgpu::TextureFormat::Rgba8Unorm,
            TargetFormat::Rgba16Float => wgpu::TextureFormat::Rgba16Float,
        }
    }

    fn all() -> Vec<Self> {
        vec![
            TargetFormat::Bgra8UnormSrgb,
            TargetFormat::Rgba8UnormSrgb,
            TargetFormat::Bgra8Unorm,
            TargetFormat::Rgba8Unorm,
            TargetFormat::Rgba16Float,
        ]
    }

    fn name(&self) -> &'static str {
        match self {
            TargetFormat::Bgra8UnormSrgb => "BGRA8 Unorm sRGB",
            TargetFormat::Rgba8UnormSrgb => "RGBA8 Unorm sRGB",
            TargetFormat::Bgra8Unorm => "BGRA8 Unorm",
            TargetFormat::Rgba8Unorm => "RGBA8 Unorm",
            TargetFormat::Rgba16Float => "RGBA16 Float",
        }
    }
}

impl Default for RenderPipelinePanel {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderPipelinePanel {
    /// Create a new render pipeline panel with default values
    pub fn new() -> Self {
        Self {
            descriptor: RenderPipelineDescriptor::new(None),
            label_input: String::new(),

            // Vertex State
            vertex_entry_point: "vs_main".to_string(),
            fragment_entry_point: "fs_main".to_string(),

            // Primitive State
            topology: PrimitiveTopology::TriangleList,
            cull_mode: CullMode::None,
            front_face: FrontFace::Ccw,

            // Depth-Stencil State
            enable_depth_stencil: false,
            depth_format: DepthFormat::Depth24Plus,
            depth_write_enabled: true,
            depth_compare: CompareFunction::Less,
            stencil_read_mask_input: "0xFFFFFFFF".to_string(),
            stencil_write_mask_input: "0xFFFFFFFF".to_string(),
            stencil_front_compare: CompareFunction::Always,
            stencil_front_fail_op: StencilOperation::Keep,
            stencil_front_depth_fail_op: StencilOperation::Keep,
            stencil_front_pass_op: StencilOperation::Keep,
            stencil_back_compare: CompareFunction::Always,
            stencil_back_fail_op: StencilOperation::Keep,
            stencil_back_depth_fail_op: StencilOperation::Keep,
            stencil_back_pass_op: StencilOperation::Keep,

            // Multisample State
            sample_count: 1,
            alpha_to_coverage_enabled: false,

            // Fragment State
            target_format: TargetFormat::Bgra8UnormSrgb,
            blend_enabled: false,
            color_blend_src: BlendFactor::One,
            color_blend_dst: BlendFactor::Zero,
            color_blend_op: BlendOperation::Add,
            alpha_blend_src: BlendFactor::One,
            alpha_blend_dst: BlendFactor::Zero,
            alpha_blend_op: BlendOperation::Add,
            write_red: true,
            write_green: true,
            write_blue: true,
            write_alpha: true,

            validation_error: None,
            success_message: None,

            preview_state: None,
            show_preview: false,
        }
    }

    /// Apply a preset configuration
    pub fn apply_preset(&mut self, preset: PipelinePreset) {
        match preset {
            PipelinePreset::Default => {
                *self = Self::new();
            }
            PipelinePreset::BasicTriangle => {
                self.topology = PrimitiveTopology::TriangleList;
                self.cull_mode = CullMode::None;
                self.front_face = FrontFace::Ccw;
                self.enable_depth_stencil = false;
                self.blend_enabled = false;
                self.sample_count = 1;
            }
            PipelinePreset::DepthTested => {
                self.topology = PrimitiveTopology::TriangleList;
                self.cull_mode = CullMode::Back;
                self.front_face = FrontFace::Ccw;
                self.enable_depth_stencil = true;
                self.depth_write_enabled = true;
                self.depth_compare = CompareFunction::Less;
                self.blend_enabled = false;
                self.sample_count = 1;
            }
            PipelinePreset::AlphaBlended => {
                self.topology = PrimitiveTopology::TriangleList;
                self.cull_mode = CullMode::None;
                self.front_face = FrontFace::Ccw;
                self.enable_depth_stencil = false;
                self.blend_enabled = true;
                // Premultiplied alpha blending
                self.color_blend_src = BlendFactor::One;
                self.color_blend_dst = BlendFactor::OneMinusSrcAlpha;
                self.color_blend_op = BlendOperation::Add;
                self.alpha_blend_src = BlendFactor::One;
                self.alpha_blend_dst = BlendFactor::OneMinusSrcAlpha;
                self.alpha_blend_op = BlendOperation::Add;
                self.sample_count = 1;
            }
            PipelinePreset::Wireframe => {
                self.topology = PrimitiveTopology::LineList;
                self.cull_mode = CullMode::None;
                self.front_face = FrontFace::Ccw;
                self.enable_depth_stencil = true;
                self.depth_write_enabled = true;
                self.depth_compare = CompareFunction::Less;
                self.blend_enabled = false;
                self.sample_count = 1;
            }
            PipelinePreset::Multisample4x => {
                self.topology = PrimitiveTopology::TriangleList;
                self.cull_mode = CullMode::Back;
                self.front_face = FrontFace::Ccw;
                self.enable_depth_stencil = true;
                self.depth_write_enabled = true;
                self.depth_compare = CompareFunction::Less;
                self.blend_enabled = false;
                self.sample_count = 4;
                self.alpha_to_coverage_enabled = false;
            }
        }

        self.update_descriptor();
        self.validation_error = None;
        self.success_message = Some(format!("✓ Preset '{}' applied", preset.name()));
    }

    /// Update the internal descriptor based on current UI state
    fn update_descriptor(&mut self) {
        let label = if self.label_input.is_empty() {
            None
        } else {
            Some(self.label_input.as_str())
        };

        let mut descriptor = RenderPipelineDescriptor::new(label)
            .with_vertex_entry_point(&self.vertex_entry_point)
            .with_fragment_entry_point(&self.fragment_entry_point)
            .with_primitive(
                PrimitiveState::new()
                    .with_topology(self.topology)
                    .with_cull_mode(self.cull_mode)
                    .with_front_face(self.front_face),
            )
            .with_multisample(
                MultisampleState::new()
                    .with_count(self.sample_count)
                    .with_alpha_to_coverage(self.alpha_to_coverage_enabled),
            );

        // Add depth-stencil state if enabled
        if self.enable_depth_stencil {
            let stencil_front = StencilFaceState {
                compare: self.stencil_front_compare,
                fail_op: self.stencil_front_fail_op,
                depth_fail_op: self.stencil_front_depth_fail_op,
                pass_op: self.stencil_front_pass_op,
            };

            let stencil_back = StencilFaceState {
                compare: self.stencil_back_compare,
                fail_op: self.stencil_back_fail_op,
                depth_fail_op: self.stencil_back_depth_fail_op,
                pass_op: self.stencil_back_pass_op,
            };

            let stencil_read_mask =
                u32::from_str_radix(self.stencil_read_mask_input.trim_start_matches("0x"), 16)
                    .unwrap_or(0xFFFFFFFF);

            let stencil_write_mask =
                u32::from_str_radix(self.stencil_write_mask_input.trim_start_matches("0x"), 16)
                    .unwrap_or(0xFFFFFFFF);

            let mut depth_stencil = DepthStencilState::new(self.depth_format.to_wgpu())
                .with_depth_write_enabled(self.depth_write_enabled)
                .with_depth_compare(self.depth_compare)
                .with_stencil_front(stencil_front)
                .with_stencil_back(stencil_back);

            depth_stencil.stencil_read_mask = stencil_read_mask;
            depth_stencil.stencil_write_mask = stencil_write_mask;

            descriptor = descriptor.with_depth_stencil(depth_stencil);
        }

        // Add fragment target
        let blend = if self.blend_enabled {
            Some(BlendState::new(
                BlendComponent::new(
                    self.color_blend_src,
                    self.color_blend_dst,
                    self.color_blend_op,
                ),
                BlendComponent::new(
                    self.alpha_blend_src,
                    self.alpha_blend_dst,
                    self.alpha_blend_op,
                ),
            ))
        } else {
            None
        };

        let mut write_mask = ColorWrites::empty();
        if self.write_red {
            write_mask = write_mask | ColorWrites::RED;
        }
        if self.write_green {
            write_mask = write_mask | ColorWrites::GREEN;
        }
        if self.write_blue {
            write_mask = write_mask | ColorWrites::BLUE;
        }
        if self.write_alpha {
            write_mask = write_mask | ColorWrites::ALPHA;
        }

        let mut target =
            ColorTargetState::new(self.target_format.to_wgpu()).with_write_mask(write_mask);

        if let Some(blend) = blend {
            target = target.with_blend(blend);
        }

        descriptor = descriptor.with_fragment_target(target);

        self.descriptor = descriptor;
    }

    /// Render the render pipeline configuration UI
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("🎨 Render Pipeline Configuration");
            ui.label("Configure comprehensive render pipeline settings with vertex, primitive, depth-stencil, multisample, and fragment states.");
            ui.add_space(10.0);

            // Display messages
            if let Some(error) = &self.validation_error {
                ui.colored_label(egui::Color32::RED, format!("❌ {}", error));
                ui.add_space(5.0);
            }
            if let Some(success) = &self.success_message {
                ui.colored_label(egui::Color32::GREEN, success);
                ui.add_space(5.0);
            }

            // Presets Section
            ui.group(|ui| {
                ui.heading("📋 Presets");
                ui.label("Quick configuration presets:");
                ui.add_space(5.0);

                ui.horizontal_wrapped(|ui| {
                    for preset in PipelinePreset::all() {
                        if ui.button(preset.name()).clicked() {
                            self.apply_preset(preset);
                        }
                    }
                });
            });

            ui.add_space(10.0);

            // Pipeline Properties
            ui.group(|ui| {
                ui.heading("Pipeline Properties");
                ui.add_space(5.0);

                egui::Grid::new("pipeline_properties")
                    .num_columns(2)
                    .spacing([10.0, 8.0])
                    .show(ui, |ui| {
                        ui.label("Label:");
                        ui.text_edit_singleline(&mut self.label_input);
                        ui.end_row();
                    });
            });

            ui.add_space(10.0);

            // Vertex State
            ui.group(|ui| {
                ui.heading("🔺 Vertex State");
                ui.label("Configure vertex shader entry point:");
                ui.add_space(5.0);

                egui::Grid::new("vertex_state")
                    .num_columns(2)
                    .spacing([10.0, 8.0])
                    .show(ui, |ui| {
                        ui.label("Vertex Entry Point:");
                        ui.text_edit_singleline(&mut self.vertex_entry_point);
                        ui.end_row();
                    });
            });

            ui.add_space(10.0);

            // Primitive State
            ui.group(|ui| {
                ui.heading("🔷 Primitive State");
                ui.label("Configure primitive topology and culling:");
                ui.add_space(5.0);

                egui::Grid::new("primitive_state")
                    .num_columns(2)
                    .spacing([10.0, 8.0])
                    .show(ui, |ui| {
                        Self::topology_tooltip(ui.label("Topology:"), self.topology);
                        Self::render_topology_combo(ui, &mut self.topology);
                        ui.end_row();

                        Self::cull_mode_tooltip(ui.label("Cull Mode:"), self.cull_mode);
                        Self::render_cull_mode_combo(ui, &mut self.cull_mode);
                        ui.end_row();

                        Self::front_face_tooltip(ui.label("Front Face:"), self.front_face);
                        Self::render_front_face_combo(ui, &mut self.front_face);
                        ui.end_row();
                    });
            });

            ui.add_space(10.0);

            // Depth-Stencil State
            ui.group(|ui| {
                ui.heading("📏 Depth-Stencil State");
                ui.checkbox(&mut self.enable_depth_stencil, "Enable Depth-Stencil Testing");
                ui.add_space(5.0);

                if self.enable_depth_stencil {
                    egui::Grid::new("depth_stencil_state")
                        .num_columns(2)
                        .spacing([10.0, 8.0])
                        .show(ui, |ui| {
                            ui.label("Depth Format:");
                            Self::render_depth_format_combo(ui, &mut self.depth_format);
                            ui.end_row();

                            property::DEPTH_WRITE_ENABLED.apply(ui.label("Depth Write:"));
                            ui.checkbox(&mut self.depth_write_enabled, "Enabled");
                            ui.end_row();

                            Self::compare_function_tooltip(ui.label("Depth Compare:"), self.depth_compare);
                            Self::render_compare_function_combo(ui, &mut self.depth_compare, "depth_compare");
                            ui.end_row();

                            ui.label("Stencil Read Mask:");
                            ui.text_edit_singleline(&mut self.stencil_read_mask_input);
                            ui.end_row();

                            ui.label("Stencil Write Mask:");
                            ui.text_edit_singleline(&mut self.stencil_write_mask_input);
                            ui.end_row();
                        });

                    ui.add_space(5.0);

                    ui.collapsing("Stencil Front Face", |ui| {
                        egui::Grid::new("stencil_front")
                            .num_columns(2)
                            .spacing([10.0, 8.0])
                            .show(ui, |ui| {
                                Self::compare_function_tooltip(ui.label("Compare:"), self.stencil_front_compare);
                                Self::render_compare_function_combo(ui, &mut self.stencil_front_compare, "stencil_front_compare");
                                ui.end_row();

                                Self::stencil_operation_tooltip(ui.label("Fail Operation:"), self.stencil_front_fail_op);
                                Self::render_stencil_operation_combo(ui, &mut self.stencil_front_fail_op, "stencil_front_fail");
                                ui.end_row();

                                Self::stencil_operation_tooltip(ui.label("Depth Fail Operation:"), self.stencil_front_depth_fail_op);
                                Self::render_stencil_operation_combo(ui, &mut self.stencil_front_depth_fail_op, "stencil_front_depth_fail");
                                ui.end_row();

                                Self::stencil_operation_tooltip(ui.label("Pass Operation:"), self.stencil_front_pass_op);
                                Self::render_stencil_operation_combo(ui, &mut self.stencil_front_pass_op, "stencil_front_pass");
                                ui.end_row();
                            });
                    });

                    ui.collapsing("Stencil Back Face", |ui| {
                        egui::Grid::new("stencil_back")
                            .num_columns(2)
                            .spacing([10.0, 8.0])
                            .show(ui, |ui| {
                                Self::compare_function_tooltip(ui.label("Compare:"), self.stencil_back_compare);
                                Self::render_compare_function_combo(ui, &mut self.stencil_back_compare, "stencil_back_compare");
                                ui.end_row();

                                Self::stencil_operation_tooltip(ui.label("Fail Operation:"), self.stencil_back_fail_op);
                                Self::render_stencil_operation_combo(ui, &mut self.stencil_back_fail_op, "stencil_back_fail");
                                ui.end_row();

                                Self::stencil_operation_tooltip(ui.label("Depth Fail Operation:"), self.stencil_back_depth_fail_op);
                                Self::render_stencil_operation_combo(ui, &mut self.stencil_back_depth_fail_op, "stencil_back_depth_fail");
                                ui.end_row();

                                Self::stencil_operation_tooltip(ui.label("Pass Operation:"), self.stencil_back_pass_op);
                                Self::render_stencil_operation_combo(ui, &mut self.stencil_back_pass_op, "stencil_back_pass");
                                ui.end_row();
                            });
                    });
                }
            });

            ui.add_space(10.0);

            // Multisample State
            ui.group(|ui| {
                ui.heading("🔬 Multisample State");
                ui.label("Configure multisampling anti-aliasing:");
                ui.add_space(5.0);

                egui::Grid::new("multisample_state")
                    .num_columns(2)
                    .spacing([10.0, 8.0])
                    .show(ui, |ui| {
                        property::SAMPLE_COUNT.apply(ui.label("Sample Count:"));
                        egui::ComboBox::from_id_salt("sample_count")
                            .selected_text(format!("{}", self.sample_count))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.sample_count, 1, "1 (No MSAA)");
                                ui.selectable_value(&mut self.sample_count, 2, "2x MSAA");
                                ui.selectable_value(&mut self.sample_count, 4, "4x MSAA");
                                ui.selectable_value(&mut self.sample_count, 8, "8x MSAA");
                            });
                        ui.end_row();

                        property::ALPHA_TO_COVERAGE.apply(ui.label("Alpha to Coverage:"));
                        ui.checkbox(&mut self.alpha_to_coverage_enabled, "Enabled");
                        ui.end_row();
                    });
            });

            ui.add_space(10.0);

            // Fragment State
            ui.group(|ui| {
                ui.heading("🎨 Fragment State");
                ui.label("Configure fragment shader and color output:");
                ui.add_space(5.0);

                egui::Grid::new("fragment_state")
                    .num_columns(2)
                    .spacing([10.0, 8.0])
                    .show(ui, |ui| {
                        ui.label("Fragment Entry Point:");
                        ui.text_edit_singleline(&mut self.fragment_entry_point);
                        ui.end_row();

                        ui.label("Target Format:");
                        Self::render_target_format_combo(ui, &mut self.target_format);
                        ui.end_row();

                        ui.label("Blending:");
                        ui.checkbox(&mut self.blend_enabled, "Enable Blending");
                        ui.end_row();
                    });

                if self.blend_enabled {
                    ui.add_space(5.0);

                    ui.collapsing("Color Blend", |ui| {
                        egui::Grid::new("color_blend")
                            .num_columns(2)
                            .spacing([10.0, 8.0])
                            .show(ui, |ui| {
                                Self::blend_factor_tooltip(ui.label("Source Factor:"), self.color_blend_src);
                                Self::render_blend_factor_combo(ui, &mut self.color_blend_src, "color_src");
                                ui.end_row();

                                Self::blend_factor_tooltip(ui.label("Destination Factor:"), self.color_blend_dst);
                                Self::render_blend_factor_combo(ui, &mut self.color_blend_dst, "color_dst");
                                ui.end_row();

                                Self::blend_operation_tooltip(ui.label("Operation:"), self.color_blend_op);
                                Self::render_blend_operation_combo(ui, &mut self.color_blend_op, "color_op");
                                ui.end_row();
                            });
                    });

                    ui.collapsing("Alpha Blend", |ui| {
                        egui::Grid::new("alpha_blend")
                            .num_columns(2)
                            .spacing([10.0, 8.0])
                            .show(ui, |ui| {
                                Self::blend_factor_tooltip(ui.label("Source Factor:"), self.alpha_blend_src);
                                Self::render_blend_factor_combo(ui, &mut self.alpha_blend_src, "alpha_src");
                                ui.end_row();

                                Self::blend_factor_tooltip(ui.label("Destination Factor:"), self.alpha_blend_dst);
                                Self::render_blend_factor_combo(ui, &mut self.alpha_blend_dst, "alpha_dst");
                                ui.end_row();

                                Self::blend_operation_tooltip(ui.label("Operation:"), self.alpha_blend_op);
                                Self::render_blend_operation_combo(ui, &mut self.alpha_blend_op, "alpha_op");
                                ui.end_row();
                            });
                    });
                }

                ui.add_space(5.0);

                ui.label("Color Write Mask:");
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.write_red, "Red");
                    ui.checkbox(&mut self.write_green, "Green");
                    ui.checkbox(&mut self.write_blue, "Blue");
                    ui.checkbox(&mut self.write_alpha, "Alpha");
                });
            });

            ui.add_space(10.0);

            // Action buttons
            ui.horizontal(|ui| {
                if ui.button("📝 Update Configuration").clicked() {
                    self.update_descriptor();
                    self.validation_error = None;
                    self.success_message = Some("✓ Configuration updated".to_string());
                }

                if ui.button("🔄 Reset to Default").clicked() {
                    *self = Self::new();
                }
            });
        });
    }

    /// UI with live pipeline preview
    pub fn ui_with_preview(
        &mut self,
        ui: &mut egui::Ui,
        device: Option<&wgpu::Device>,
        queue: Option<&wgpu::Queue>,
        renderer: Option<&mut egui_wgpu::Renderer>,
    ) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("🎨 Render Pipeline Configuration");
            ui.label("Configure comprehensive render pipeline settings with vertex, primitive, depth-stencil, multisample, and fragment states.");
            ui.add_space(10.0);

            // Display messages
            if let Some(error) = &self.validation_error {
                ui.colored_label(egui::Color32::RED, format!("❌ {}", error));
                ui.add_space(5.0);
            }
            if let Some(success) = &self.success_message {
                ui.colored_label(egui::Color32::GREEN, success);
                ui.add_space(5.0);
            }

            // Presets Section
            ui.group(|ui| {
                ui.heading("📋 Presets");
                ui.label("Quick configuration presets:");
                ui.add_space(5.0);

                ui.horizontal_wrapped(|ui| {
                    for preset in PipelinePreset::all() {
                        if ui.button(preset.name()).clicked() {
                            self.apply_preset(preset);
                        }
                    }
                });
            });

            ui.add_space(10.0);

            // Call the existing UI method to render all configuration sections
            // We need to temporarily create a new scope to prevent duplicate heading
            self.render_configuration_ui(ui);

            ui.add_space(15.0);

            // Live Preview Section
            if self.show_preview {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.heading("🎬 Pipeline Preview");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.small_button("✕").on_hover_text("Hide preview").clicked() {
                                self.show_preview = false;
                            }
                        });
                    });
                    ui.add_space(5.0);

                    ui.label("Preview shows how this pipeline configuration affects rendering of a 3D cube:");
                    ui.label("• Topology: Triangle/Line primitives");
                    ui.label("• Culling: Front/back face visibility");
                    ui.label("• Depth: Z-buffer testing effect");
                    ui.label("• Blending: Color composition");

                    ui.add_space(5.0);

                    // Initialize preview if we have device
                    if let Some(device) = device {
                        if self.preview_state.is_none() {
                            let mut preview = RenderPipelinePreviewState::new();
                            preview.initialize(device);
                            self.preview_state = Some(preview);
                        }

                        // Update descriptor before borrowing preview
                        self.update_descriptor();

                        // Update pipeline when configuration changes
                        if let Some(preview) = &mut self.preview_state {
                            // Build primitive state
                            let primitive = PrimitiveState::new()
                                .with_topology(self.topology)
                                .with_cull_mode(self.cull_mode)
                                .with_front_face(self.front_face);

                            // Build depth-stencil state
                            let depth_stencil = if self.enable_depth_stencil {
                                Some(
                                    DepthStencilState::new(self.depth_format.to_wgpu())
                                        .with_depth_write_enabled(self.depth_write_enabled)
                                        .with_depth_compare(self.depth_compare),
                                )
                            } else {
                                None
                            };

                            // Build blend state
                            let blend = if self.blend_enabled {
                                Some(BlendState::new(
                                    BlendComponent::new(
                                        self.color_blend_src,
                                        self.color_blend_dst,
                                        self.color_blend_op,
                                    ),
                                    BlendComponent::new(
                                        self.alpha_blend_src,
                                        self.alpha_blend_dst,
                                        self.alpha_blend_op,
                                    ),
                                ))
                            } else {
                                None
                            };

                            // Build multisample state
                            let multisample = MultisampleState::new()
                                .with_count(self.sample_count)
                                .with_alpha_to_coverage(self.alpha_to_coverage_enabled);

                            // Update pipeline
                            preview.update_pipeline(
                                device,
                                &primitive,
                                depth_stencil.as_ref(),
                                blend.as_ref(),
                                &multisample,
                            );
                        }
                    }

                    // Render preview
                    if let (Some(preview), Some(device), Some(queue), Some(renderer)) =
                        (&mut self.preview_state, device, queue, renderer)
                    {
                        // Render the preview
                        let delta_time = ui.input(|i| i.stable_dt);
                        preview.render(device, queue, delta_time);

                        // Display the preview texture
                        #[cfg(not(target_arch = "wasm32"))]
                        if let Some(texture_id) = preview.get_texture_id(device, renderer) {
                            let (width, height) = preview.size();
                            ui.add(egui::Image::new(egui::load::SizedTexture::new(
                                texture_id,
                                egui::vec2(width as f32, height as f32),
                            )));
                        }

                        // Always request repaint for animated preview (rotating cube)
                        ui.ctx().request_repaint();
                    } else if device.is_none() {
                        ui.colored_label(
                            egui::Color32::YELLOW,
                            "⚠ Preview requires GPU device to be initialized",
                        );
                    }
                });
            } else {
                // Show "Test Pipeline" button when preview is hidden
                ui.horizontal(|ui| {
                    if ui.button("🎬 Test Pipeline").on_hover_text("Show live preview of pipeline configuration").clicked() {
                        self.show_preview = true;
                    }
                });
            }
        });
    }

    /// Render the main configuration UI (used by both ui() and ui_with_preview())
    fn render_configuration_ui(&mut self, ui: &mut egui::Ui) {
        // Pipeline Properties
        ui.group(|ui| {
            ui.heading("Pipeline Properties");
            ui.add_space(5.0);

            egui::Grid::new("pipeline_properties")
                .num_columns(2)
                .spacing([10.0, 8.0])
                .show(ui, |ui| {
                    ui.label("Label:");
                    ui.text_edit_singleline(&mut self.label_input);
                    ui.end_row();
                });
        });

        ui.add_space(10.0);

        // Vertex State
        ui.group(|ui| {
            ui.heading("🔺 Vertex State");
            ui.label("Configure vertex shader entry point:");
            ui.add_space(5.0);

            egui::Grid::new("vertex_state")
                .num_columns(2)
                .spacing([10.0, 8.0])
                .show(ui, |ui| {
                    ui.label("Vertex Entry Point:");
                    ui.text_edit_singleline(&mut self.vertex_entry_point);
                    ui.end_row();
                });
        });

        ui.add_space(10.0);

        // Primitive State
        ui.group(|ui| {
            ui.heading("🔷 Primitive State");
            ui.label("Configure primitive topology and culling:");
            ui.add_space(5.0);

            egui::Grid::new("primitive_state")
                .num_columns(2)
                .spacing([10.0, 8.0])
                .show(ui, |ui| {
                    ui.label("Topology:")
                        .on_hover_text("How vertices are assembled into primitives");
                    Self::render_topology_combo(ui, &mut self.topology);
                    ui.end_row();

                    ui.label("Cull Mode:")
                        .on_hover_text("Which faces to cull (not render)");
                    Self::render_cull_mode_combo(ui, &mut self.cull_mode);
                    ui.end_row();

                    ui.label("Front Face:")
                        .on_hover_text("Winding order that determines front-facing");
                    Self::render_front_face_combo(ui, &mut self.front_face);
                    ui.end_row();
                });
        });

        ui.add_space(10.0);

        // Depth-Stencil State
        ui.group(|ui| {
            ui.heading("📏 Depth-Stencil State");
            ui.checkbox(
                &mut self.enable_depth_stencil,
                "Enable Depth-Stencil Testing",
            );
            ui.add_space(5.0);

            if self.enable_depth_stencil {
                egui::Grid::new("depth_stencil_state")
                    .num_columns(2)
                    .spacing([10.0, 8.0])
                    .show(ui, |ui| {
                        ui.label("Depth Format:");
                        Self::render_depth_format_combo(ui, &mut self.depth_format);
                        ui.end_row();

                        ui.label("Depth Write:");
                        ui.checkbox(&mut self.depth_write_enabled, "Enabled");
                        ui.end_row();

                        ui.label("Depth Compare:")
                            .on_hover_text("Comparison function for depth test");
                        Self::render_compare_function_combo(
                            ui,
                            &mut self.depth_compare,
                            "depth_compare",
                        );
                        ui.end_row();

                        ui.label("Stencil Read Mask:");
                        ui.text_edit_singleline(&mut self.stencil_read_mask_input);
                        ui.end_row();

                        ui.label("Stencil Write Mask:");
                        ui.text_edit_singleline(&mut self.stencil_write_mask_input);
                        ui.end_row();
                    });

                ui.add_space(5.0);

                ui.collapsing("Stencil Front Face", |ui| {
                    egui::Grid::new("stencil_front")
                        .num_columns(2)
                        .spacing([10.0, 8.0])
                        .show(ui, |ui| {
                            ui.label("Compare:");
                            Self::render_compare_function_combo(
                                ui,
                                &mut self.stencil_front_compare,
                                "stencil_front_compare",
                            );
                            ui.end_row();

                            ui.label("Fail Operation:");
                            Self::render_stencil_operation_combo(
                                ui,
                                &mut self.stencil_front_fail_op,
                                "stencil_front_fail",
                            );
                            ui.end_row();

                            ui.label("Depth Fail Operation:");
                            Self::render_stencil_operation_combo(
                                ui,
                                &mut self.stencil_front_depth_fail_op,
                                "stencil_front_depth_fail",
                            );
                            ui.end_row();

                            ui.label("Pass Operation:");
                            Self::render_stencil_operation_combo(
                                ui,
                                &mut self.stencil_front_pass_op,
                                "stencil_front_pass",
                            );
                            ui.end_row();
                        });
                });

                ui.collapsing("Stencil Back Face", |ui| {
                    egui::Grid::new("stencil_back")
                        .num_columns(2)
                        .spacing([10.0, 8.0])
                        .show(ui, |ui| {
                            ui.label("Compare:");
                            Self::render_compare_function_combo(
                                ui,
                                &mut self.stencil_back_compare,
                                "stencil_back_compare",
                            );
                            ui.end_row();

                            ui.label("Fail Operation:");
                            Self::render_stencil_operation_combo(
                                ui,
                                &mut self.stencil_back_fail_op,
                                "stencil_back_fail",
                            );
                            ui.end_row();

                            ui.label("Depth Fail Operation:");
                            Self::render_stencil_operation_combo(
                                ui,
                                &mut self.stencil_back_depth_fail_op,
                                "stencil_back_depth_fail",
                            );
                            ui.end_row();

                            ui.label("Pass Operation:");
                            Self::render_stencil_operation_combo(
                                ui,
                                &mut self.stencil_back_pass_op,
                                "stencil_back_pass",
                            );
                            ui.end_row();
                        });
                });
            }
        });

        ui.add_space(10.0);

        // Multisample State
        ui.group(|ui| {
            ui.heading("🔬 Multisample State");
            ui.label("Configure multisampling anti-aliasing:");
            ui.add_space(5.0);

            egui::Grid::new("multisample_state")
                .num_columns(2)
                .spacing([10.0, 8.0])
                .show(ui, |ui| {
                    ui.label("Sample Count:")
                        .on_hover_text("Number of samples per pixel (1, 2, 4, or 8)");
                    egui::ComboBox::from_id_salt("sample_count")
                        .selected_text(format!("{}", self.sample_count))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.sample_count, 1, "1 (No MSAA)");
                            ui.selectable_value(&mut self.sample_count, 2, "2x MSAA");
                            ui.selectable_value(&mut self.sample_count, 4, "4x MSAA");
                            ui.selectable_value(&mut self.sample_count, 8, "8x MSAA");
                        });
                    ui.end_row();

                    ui.label("Alpha to Coverage:")
                        .on_hover_text("Enable alpha to coverage for transparency");
                    ui.checkbox(&mut self.alpha_to_coverage_enabled, "Enabled");
                    ui.end_row();
                });
        });

        ui.add_space(10.0);

        // Fragment State
        ui.group(|ui| {
            ui.heading("🎨 Fragment State");
            ui.label("Configure fragment shader and color output:");
            ui.add_space(5.0);

            egui::Grid::new("fragment_state")
                .num_columns(2)
                .spacing([10.0, 8.0])
                .show(ui, |ui| {
                    ui.label("Fragment Entry Point:");
                    ui.text_edit_singleline(&mut self.fragment_entry_point);
                    ui.end_row();

                    ui.label("Target Format:");
                    Self::render_target_format_combo(ui, &mut self.target_format);
                    ui.end_row();

                    ui.label("Blending:");
                    ui.checkbox(&mut self.blend_enabled, "Enable Blending");
                    ui.end_row();
                });

            if self.blend_enabled {
                ui.add_space(5.0);

                ui.collapsing("Color Blend", |ui| {
                    egui::Grid::new("color_blend")
                        .num_columns(2)
                        .spacing([10.0, 8.0])
                        .show(ui, |ui| {
                            ui.label("Source Factor:");
                            Self::render_blend_factor_combo(
                                ui,
                                &mut self.color_blend_src,
                                "color_src",
                            );
                            ui.end_row();

                            ui.label("Destination Factor:");
                            Self::render_blend_factor_combo(
                                ui,
                                &mut self.color_blend_dst,
                                "color_dst",
                            );
                            ui.end_row();

                            ui.label("Operation:");
                            Self::render_blend_operation_combo(
                                ui,
                                &mut self.color_blend_op,
                                "color_op",
                            );
                            ui.end_row();
                        });
                });

                ui.collapsing("Alpha Blend", |ui| {
                    egui::Grid::new("alpha_blend")
                        .num_columns(2)
                        .spacing([10.0, 8.0])
                        .show(ui, |ui| {
                            ui.label("Source Factor:");
                            Self::render_blend_factor_combo(
                                ui,
                                &mut self.alpha_blend_src,
                                "alpha_src",
                            );
                            ui.end_row();

                            ui.label("Destination Factor:");
                            Self::render_blend_factor_combo(
                                ui,
                                &mut self.alpha_blend_dst,
                                "alpha_dst",
                            );
                            ui.end_row();

                            ui.label("Operation:");
                            Self::render_blend_operation_combo(
                                ui,
                                &mut self.alpha_blend_op,
                                "alpha_op",
                            );
                            ui.end_row();
                        });
                });
            }

            ui.add_space(5.0);

            ui.label("Color Write Mask:");
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.write_red, "Red");
                ui.checkbox(&mut self.write_green, "Green");
                ui.checkbox(&mut self.write_blue, "Blue");
                ui.checkbox(&mut self.write_alpha, "Alpha");
            });
        });

        ui.add_space(10.0);

        // Action buttons
        ui.horizontal(|ui| {
            if ui.button("📝 Update Configuration").clicked() {
                self.update_descriptor();
                self.validation_error = None;
                self.success_message = Some("✓ Configuration updated".to_string());
            }

            if ui.button("🔄 Reset to Default").clicked() {
                *self = Self::new();
            }
        });
    }

    // Helper methods for applying tooltips based on enum values
    fn topology_tooltip(response: egui::Response, topology: PrimitiveTopology) -> egui::Response {
        match topology {
            PrimitiveTopology::PointList => primitive_topology::POINT_LIST.apply(response),
            PrimitiveTopology::LineList => primitive_topology::LINE_LIST.apply(response),
            PrimitiveTopology::LineStrip => primitive_topology::LINE_STRIP.apply(response),
            PrimitiveTopology::TriangleList => primitive_topology::TRIANGLE_LIST.apply(response),
            PrimitiveTopology::TriangleStrip => primitive_topology::TRIANGLE_STRIP.apply(response),
        }
    }

    fn cull_mode_tooltip(response: egui::Response, mode: CullMode) -> egui::Response {
        match mode {
            CullMode::None => cull_mode::NONE.apply(response),
            CullMode::Front => cull_mode::FRONT.apply(response),
            CullMode::Back => cull_mode::BACK.apply(response),
        }
    }

    fn front_face_tooltip(response: egui::Response, face: FrontFace) -> egui::Response {
        match face {
            FrontFace::Ccw => front_face::CCW.apply(response),
            FrontFace::Cw => front_face::CW.apply(response),
        }
    }

    fn compare_function_tooltip(response: egui::Response, func: CompareFunction) -> egui::Response {
        match func {
            CompareFunction::Never => compare_function::NEVER.apply(response),
            CompareFunction::Less => compare_function::LESS.apply(response),
            CompareFunction::Equal => compare_function::EQUAL.apply(response),
            CompareFunction::LessEqual => compare_function::LESS_EQUAL.apply(response),
            CompareFunction::Greater => compare_function::GREATER.apply(response),
            CompareFunction::NotEqual => compare_function::NOT_EQUAL.apply(response),
            CompareFunction::GreaterEqual => compare_function::GREATER_EQUAL.apply(response),
            CompareFunction::Always => compare_function::ALWAYS.apply(response),
        }
    }

    fn stencil_operation_tooltip(response: egui::Response, op: StencilOperation) -> egui::Response {
        match op {
            StencilOperation::Keep => stencil_operation::KEEP.apply(response),
            StencilOperation::Zero => stencil_operation::ZERO.apply(response),
            StencilOperation::Replace => stencil_operation::REPLACE.apply(response),
            StencilOperation::Invert => stencil_operation::INVERT.apply(response),
            StencilOperation::IncrementClamp => stencil_operation::INCREMENT_CLAMP.apply(response),
            StencilOperation::DecrementClamp => stencil_operation::DECREMENT_CLAMP.apply(response),
            StencilOperation::IncrementWrap => stencil_operation::INCREMENT_WRAP.apply(response),
            StencilOperation::DecrementWrap => stencil_operation::DECREMENT_WRAP.apply(response),
        }
    }

    fn blend_factor_tooltip(response: egui::Response, factor: BlendFactor) -> egui::Response {
        match factor {
            BlendFactor::Zero => blend_factor::ZERO.apply(response),
            BlendFactor::One => blend_factor::ONE.apply(response),
            BlendFactor::Src => blend_factor::SRC.apply(response),
            BlendFactor::OneMinusSrc => blend_factor::ONE_MINUS_SRC.apply(response),
            BlendFactor::SrcAlpha => blend_factor::SRC_ALPHA.apply(response),
            BlendFactor::OneMinusSrcAlpha => blend_factor::ONE_MINUS_SRC_ALPHA.apply(response),
            BlendFactor::Dst => blend_factor::DST.apply(response),
            BlendFactor::OneMinusDst => blend_factor::ONE_MINUS_DST.apply(response),
            BlendFactor::DstAlpha => blend_factor::DST_ALPHA.apply(response),
            BlendFactor::OneMinusDstAlpha => blend_factor::ONE_MINUS_DST_ALPHA.apply(response),
            // Note: Constant and SrcAlphaSaturated are not in the tooltip module
            BlendFactor::Constant
            | BlendFactor::OneMinusConstant
            | BlendFactor::SrcAlphaSaturated => response,
        }
    }

    fn blend_operation_tooltip(response: egui::Response, op: BlendOperation) -> egui::Response {
        match op {
            BlendOperation::Add => blend_operation::ADD.apply(response),
            BlendOperation::Subtract => blend_operation::SUBTRACT.apply(response),
            BlendOperation::ReverseSubtract => blend_operation::REVERSE_SUBTRACT.apply(response),
            BlendOperation::Min => blend_operation::MIN.apply(response),
            BlendOperation::Max => blend_operation::MAX.apply(response),
        }
    }

    // Helper methods for rendering combo boxes
    fn render_topology_combo(ui: &mut egui::Ui, topology: &mut PrimitiveTopology) {
        egui::ComboBox::from_id_salt("topology")
            .selected_text(Self::topology_name(*topology))
            .show_ui(ui, |ui| {
                ui.selectable_value(topology, PrimitiveTopology::TriangleList, "Triangle List");
                ui.selectable_value(topology, PrimitiveTopology::TriangleStrip, "Triangle Strip");
                ui.selectable_value(topology, PrimitiveTopology::LineList, "Line List");
                ui.selectable_value(topology, PrimitiveTopology::LineStrip, "Line Strip");
                ui.selectable_value(topology, PrimitiveTopology::PointList, "Point List");
            });
    }

    fn topology_name(topology: PrimitiveTopology) -> &'static str {
        match topology {
            PrimitiveTopology::TriangleList => "Triangle List",
            PrimitiveTopology::TriangleStrip => "Triangle Strip",
            PrimitiveTopology::LineList => "Line List",
            PrimitiveTopology::LineStrip => "Line Strip",
            PrimitiveTopology::PointList => "Point List",
        }
    }

    fn render_cull_mode_combo(ui: &mut egui::Ui, cull_mode: &mut CullMode) {
        egui::ComboBox::from_id_salt("cull_mode")
            .selected_text(Self::cull_mode_name(*cull_mode))
            .show_ui(ui, |ui| {
                ui.selectable_value(cull_mode, CullMode::None, "None");
                ui.selectable_value(cull_mode, CullMode::Front, "Front");
                ui.selectable_value(cull_mode, CullMode::Back, "Back");
            });
    }

    fn cull_mode_name(cull_mode: CullMode) -> &'static str {
        match cull_mode {
            CullMode::None => "None",
            CullMode::Front => "Front",
            CullMode::Back => "Back",
        }
    }

    fn render_front_face_combo(ui: &mut egui::Ui, front_face: &mut FrontFace) {
        egui::ComboBox::from_id_salt("front_face")
            .selected_text(Self::front_face_name(*front_face))
            .show_ui(ui, |ui| {
                ui.selectable_value(front_face, FrontFace::Ccw, "Counter-Clockwise");
                ui.selectable_value(front_face, FrontFace::Cw, "Clockwise");
            });
    }

    fn front_face_name(front_face: FrontFace) -> &'static str {
        match front_face {
            FrontFace::Ccw => "Counter-Clockwise",
            FrontFace::Cw => "Clockwise",
        }
    }

    fn render_depth_format_combo(ui: &mut egui::Ui, format: &mut DepthFormat) {
        egui::ComboBox::from_id_salt("depth_format")
            .selected_text(format.name())
            .show_ui(ui, |ui| {
                for f in DepthFormat::all() {
                    ui.selectable_value(format, f, f.name());
                }
            });
    }

    fn render_target_format_combo(ui: &mut egui::Ui, format: &mut TargetFormat) {
        egui::ComboBox::from_id_salt("target_format")
            .selected_text(format.name())
            .show_ui(ui, |ui| {
                for f in TargetFormat::all() {
                    ui.selectable_value(format, f, f.name());
                }
            });
    }

    fn render_compare_function_combo(ui: &mut egui::Ui, compare: &mut CompareFunction, id: &str) {
        egui::ComboBox::from_id_salt(id)
            .selected_text(Self::compare_function_name(*compare))
            .show_ui(ui, |ui| {
                ui.selectable_value(compare, CompareFunction::Never, "Never");
                ui.selectable_value(compare, CompareFunction::Less, "Less");
                ui.selectable_value(compare, CompareFunction::Equal, "Equal");
                ui.selectable_value(compare, CompareFunction::LessEqual, "Less or Equal");
                ui.selectable_value(compare, CompareFunction::Greater, "Greater");
                ui.selectable_value(compare, CompareFunction::NotEqual, "Not Equal");
                ui.selectable_value(compare, CompareFunction::GreaterEqual, "Greater or Equal");
                ui.selectable_value(compare, CompareFunction::Always, "Always");
            });
    }

    fn compare_function_name(compare: CompareFunction) -> &'static str {
        match compare {
            CompareFunction::Never => "Never",
            CompareFunction::Less => "Less",
            CompareFunction::Equal => "Equal",
            CompareFunction::LessEqual => "Less or Equal",
            CompareFunction::Greater => "Greater",
            CompareFunction::NotEqual => "Not Equal",
            CompareFunction::GreaterEqual => "Greater or Equal",
            CompareFunction::Always => "Always",
        }
    }

    fn render_stencil_operation_combo(
        ui: &mut egui::Ui,
        operation: &mut StencilOperation,
        id: &str,
    ) {
        egui::ComboBox::from_id_salt(id)
            .selected_text(Self::stencil_operation_name(*operation))
            .show_ui(ui, |ui| {
                ui.selectable_value(operation, StencilOperation::Keep, "Keep");
                ui.selectable_value(operation, StencilOperation::Zero, "Zero");
                ui.selectable_value(operation, StencilOperation::Replace, "Replace");
                ui.selectable_value(
                    operation,
                    StencilOperation::IncrementClamp,
                    "Increment Clamp",
                );
                ui.selectable_value(
                    operation,
                    StencilOperation::DecrementClamp,
                    "Decrement Clamp",
                );
                ui.selectable_value(operation, StencilOperation::Invert, "Invert");
                ui.selectable_value(operation, StencilOperation::IncrementWrap, "Increment Wrap");
                ui.selectable_value(operation, StencilOperation::DecrementWrap, "Decrement Wrap");
            });
    }

    fn stencil_operation_name(operation: StencilOperation) -> &'static str {
        match operation {
            StencilOperation::Keep => "Keep",
            StencilOperation::Zero => "Zero",
            StencilOperation::Replace => "Replace",
            StencilOperation::IncrementClamp => "Increment Clamp",
            StencilOperation::DecrementClamp => "Decrement Clamp",
            StencilOperation::Invert => "Invert",
            StencilOperation::IncrementWrap => "Increment Wrap",
            StencilOperation::DecrementWrap => "Decrement Wrap",
        }
    }

    fn render_blend_factor_combo(ui: &mut egui::Ui, factor: &mut BlendFactor, id: &str) {
        egui::ComboBox::from_id_salt(id)
            .selected_text(Self::blend_factor_name(*factor))
            .show_ui(ui, |ui| {
                ui.selectable_value(factor, BlendFactor::Zero, "Zero");
                ui.selectable_value(factor, BlendFactor::One, "One");
                ui.selectable_value(factor, BlendFactor::Src, "Source");
                ui.selectable_value(factor, BlendFactor::OneMinusSrc, "One - Source");
                ui.selectable_value(factor, BlendFactor::SrcAlpha, "Source Alpha");
                ui.selectable_value(factor, BlendFactor::OneMinusSrcAlpha, "One - Source Alpha");
                ui.selectable_value(factor, BlendFactor::Dst, "Destination");
                ui.selectable_value(factor, BlendFactor::OneMinusDst, "One - Destination");
                ui.selectable_value(factor, BlendFactor::DstAlpha, "Destination Alpha");
                ui.selectable_value(
                    factor,
                    BlendFactor::OneMinusDstAlpha,
                    "One - Destination Alpha",
                );
                ui.selectable_value(factor, BlendFactor::Constant, "Constant");
                ui.selectable_value(factor, BlendFactor::OneMinusConstant, "One - Constant");
                ui.selectable_value(
                    factor,
                    BlendFactor::SrcAlphaSaturated,
                    "Source Alpha Saturated",
                );
            });
    }

    fn blend_factor_name(factor: BlendFactor) -> &'static str {
        match factor {
            BlendFactor::Zero => "Zero",
            BlendFactor::One => "One",
            BlendFactor::Src => "Source",
            BlendFactor::OneMinusSrc => "One - Source",
            BlendFactor::SrcAlpha => "Source Alpha",
            BlendFactor::OneMinusSrcAlpha => "One - Source Alpha",
            BlendFactor::Dst => "Destination",
            BlendFactor::OneMinusDst => "One - Destination",
            BlendFactor::DstAlpha => "Destination Alpha",
            BlendFactor::OneMinusDstAlpha => "One - Destination Alpha",
            BlendFactor::Constant => "Constant",
            BlendFactor::OneMinusConstant => "One - Constant",
            BlendFactor::SrcAlphaSaturated => "Source Alpha Saturated",
        }
    }

    fn render_blend_operation_combo(ui: &mut egui::Ui, operation: &mut BlendOperation, id: &str) {
        egui::ComboBox::from_id_salt(id)
            .selected_text(Self::blend_operation_name(*operation))
            .show_ui(ui, |ui| {
                ui.selectable_value(operation, BlendOperation::Add, "Add");
                ui.selectable_value(operation, BlendOperation::Subtract, "Subtract");
                ui.selectable_value(
                    operation,
                    BlendOperation::ReverseSubtract,
                    "Reverse Subtract",
                );
                ui.selectable_value(operation, BlendOperation::Min, "Min");
                ui.selectable_value(operation, BlendOperation::Max, "Max");
            });
    }

    fn blend_operation_name(operation: BlendOperation) -> &'static str {
        match operation {
            BlendOperation::Add => "Add",
            BlendOperation::Subtract => "Subtract",
            BlendOperation::ReverseSubtract => "Reverse Subtract",
            BlendOperation::Min => "Min",
            BlendOperation::Max => "Max",
        }
    }
}

/// Pipeline preset configurations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelinePreset {
    /// Default basic configuration
    Default,
    /// Basic triangle rendering (no depth, no blending)
    BasicTriangle,
    /// Depth-tested solid rendering
    DepthTested,
    /// Alpha-blended transparent rendering
    AlphaBlended,
    /// Wireframe rendering
    Wireframe,
    /// 4x multisample anti-aliasing
    Multisample4x,
}

impl PipelinePreset {
    fn all() -> Vec<Self> {
        vec![
            PipelinePreset::Default,
            PipelinePreset::BasicTriangle,
            PipelinePreset::DepthTested,
            PipelinePreset::AlphaBlended,
            PipelinePreset::Wireframe,
            PipelinePreset::Multisample4x,
        ]
    }

    fn name(&self) -> &'static str {
        match self {
            PipelinePreset::Default => "Default",
            PipelinePreset::BasicTriangle => "Basic Triangle",
            PipelinePreset::DepthTested => "Depth Tested",
            PipelinePreset::AlphaBlended => "Alpha Blended",
            PipelinePreset::Wireframe => "Wireframe",
            PipelinePreset::Multisample4x => "4x MSAA",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_pipeline_panel_creation() {
        let panel = RenderPipelinePanel::new();
        assert_eq!(panel.label_input, "");
        assert_eq!(panel.vertex_entry_point, "vs_main");
        assert_eq!(panel.fragment_entry_point, "fs_main");
        assert_eq!(panel.topology, PrimitiveTopology::TriangleList);
        assert_eq!(panel.cull_mode, CullMode::None);
        assert_eq!(panel.front_face, FrontFace::Ccw);
        assert!(!panel.enable_depth_stencil);
        assert_eq!(panel.sample_count, 1);
        assert!(!panel.alpha_to_coverage_enabled);
        assert!(!panel.blend_enabled);
        assert!(panel.write_red);
        assert!(panel.write_green);
        assert!(panel.write_blue);
        assert!(panel.write_alpha);
    }

    #[test]
    fn test_render_pipeline_panel_default() {
        let panel = RenderPipelinePanel::default();
        assert_eq!(panel.vertex_entry_point, "vs_main");
        assert_eq!(panel.fragment_entry_point, "fs_main");
    }

    #[test]
    fn test_update_descriptor() {
        let mut panel = RenderPipelinePanel::new();
        panel.label_input = "test_pipeline".to_string();
        panel.topology = PrimitiveTopology::TriangleStrip;
        panel.cull_mode = CullMode::Back;

        panel.update_descriptor();

        assert_eq!(panel.descriptor.label(), Some("test_pipeline"));
    }

    #[test]
    fn test_preset_basic_triangle() {
        let mut panel = RenderPipelinePanel::new();
        panel.apply_preset(PipelinePreset::BasicTriangle);

        assert_eq!(panel.topology, PrimitiveTopology::TriangleList);
        assert_eq!(panel.cull_mode, CullMode::None);
        assert_eq!(panel.front_face, FrontFace::Ccw);
        assert!(!panel.enable_depth_stencil);
        assert!(!panel.blend_enabled);
        assert_eq!(panel.sample_count, 1);
        assert!(panel.success_message.is_some());
    }

    #[test]
    fn test_preset_depth_tested() {
        let mut panel = RenderPipelinePanel::new();
        panel.apply_preset(PipelinePreset::DepthTested);

        assert_eq!(panel.topology, PrimitiveTopology::TriangleList);
        assert_eq!(panel.cull_mode, CullMode::Back);
        assert_eq!(panel.front_face, FrontFace::Ccw);
        assert!(panel.enable_depth_stencil);
        assert!(panel.depth_write_enabled);
        assert_eq!(panel.depth_compare, CompareFunction::Less);
        assert!(!panel.blend_enabled);
        assert_eq!(panel.sample_count, 1);
    }

    #[test]
    fn test_preset_alpha_blended() {
        let mut panel = RenderPipelinePanel::new();
        panel.apply_preset(PipelinePreset::AlphaBlended);

        assert_eq!(panel.topology, PrimitiveTopology::TriangleList);
        assert_eq!(panel.cull_mode, CullMode::None);
        assert!(!panel.enable_depth_stencil);
        assert!(panel.blend_enabled);
        assert_eq!(panel.color_blend_src, BlendFactor::One);
        assert_eq!(panel.color_blend_dst, BlendFactor::OneMinusSrcAlpha);
        assert_eq!(panel.color_blend_op, BlendOperation::Add);
        assert_eq!(panel.alpha_blend_src, BlendFactor::One);
        assert_eq!(panel.alpha_blend_dst, BlendFactor::OneMinusSrcAlpha);
    }

    #[test]
    fn test_preset_wireframe() {
        let mut panel = RenderPipelinePanel::new();
        panel.apply_preset(PipelinePreset::Wireframe);

        assert_eq!(panel.topology, PrimitiveTopology::LineList);
        assert_eq!(panel.cull_mode, CullMode::None);
        assert!(panel.enable_depth_stencil);
        assert!(!panel.blend_enabled);
    }

    #[test]
    fn test_preset_multisample_4x() {
        let mut panel = RenderPipelinePanel::new();
        panel.apply_preset(PipelinePreset::Multisample4x);

        assert_eq!(panel.topology, PrimitiveTopology::TriangleList);
        assert_eq!(panel.cull_mode, CullMode::Back);
        assert!(panel.enable_depth_stencil);
        assert_eq!(panel.sample_count, 4);
        assert!(!panel.alpha_to_coverage_enabled);
    }

    #[test]
    fn test_depth_format_conversion() {
        assert_eq!(
            DepthFormat::Depth24Plus.to_wgpu(),
            wgpu::TextureFormat::Depth24Plus
        );
        assert_eq!(
            DepthFormat::Depth32Float.to_wgpu(),
            wgpu::TextureFormat::Depth32Float
        );
    }

    #[test]
    fn test_target_format_conversion() {
        assert_eq!(
            TargetFormat::Bgra8UnormSrgb.to_wgpu(),
            wgpu::TextureFormat::Bgra8UnormSrgb
        );
        assert_eq!(
            TargetFormat::Rgba8UnormSrgb.to_wgpu(),
            wgpu::TextureFormat::Rgba8UnormSrgb
        );
    }

    #[test]
    fn test_topology_name() {
        assert_eq!(
            RenderPipelinePanel::topology_name(PrimitiveTopology::TriangleList),
            "Triangle List"
        );
        assert_eq!(
            RenderPipelinePanel::topology_name(PrimitiveTopology::LineList),
            "Line List"
        );
    }

    #[test]
    fn test_cull_mode_name() {
        assert_eq!(RenderPipelinePanel::cull_mode_name(CullMode::None), "None");
        assert_eq!(RenderPipelinePanel::cull_mode_name(CullMode::Back), "Back");
    }

    #[test]
    fn test_front_face_name() {
        assert_eq!(
            RenderPipelinePanel::front_face_name(FrontFace::Ccw),
            "Counter-Clockwise"
        );
        assert_eq!(
            RenderPipelinePanel::front_face_name(FrontFace::Cw),
            "Clockwise"
        );
    }

    #[test]
    fn test_compare_function_name() {
        assert_eq!(
            RenderPipelinePanel::compare_function_name(CompareFunction::Less),
            "Less"
        );
        assert_eq!(
            RenderPipelinePanel::compare_function_name(CompareFunction::Always),
            "Always"
        );
    }

    #[test]
    fn test_blend_factor_name() {
        assert_eq!(
            RenderPipelinePanel::blend_factor_name(BlendFactor::One),
            "One"
        );
        assert_eq!(
            RenderPipelinePanel::blend_factor_name(BlendFactor::SrcAlpha),
            "Source Alpha"
        );
    }

    #[test]
    fn test_blend_operation_name() {
        assert_eq!(
            RenderPipelinePanel::blend_operation_name(BlendOperation::Add),
            "Add"
        );
        assert_eq!(
            RenderPipelinePanel::blend_operation_name(BlendOperation::Subtract),
            "Subtract"
        );
    }

    #[test]
    fn test_stencil_operation_name() {
        assert_eq!(
            RenderPipelinePanel::stencil_operation_name(StencilOperation::Keep),
            "Keep"
        );
        assert_eq!(
            RenderPipelinePanel::stencil_operation_name(StencilOperation::Replace),
            "Replace"
        );
    }

    #[test]
    fn test_preset_all() {
        let presets = PipelinePreset::all();
        assert_eq!(presets.len(), 6);
        assert!(presets.contains(&PipelinePreset::Default));
        assert!(presets.contains(&PipelinePreset::BasicTriangle));
        assert!(presets.contains(&PipelinePreset::DepthTested));
        assert!(presets.contains(&PipelinePreset::AlphaBlended));
        assert!(presets.contains(&PipelinePreset::Wireframe));
        assert!(presets.contains(&PipelinePreset::Multisample4x));
    }

    #[test]
    fn test_preset_names() {
        assert_eq!(PipelinePreset::Default.name(), "Default");
        assert_eq!(PipelinePreset::BasicTriangle.name(), "Basic Triangle");
        assert_eq!(PipelinePreset::DepthTested.name(), "Depth Tested");
        assert_eq!(PipelinePreset::AlphaBlended.name(), "Alpha Blended");
        assert_eq!(PipelinePreset::Wireframe.name(), "Wireframe");
        assert_eq!(PipelinePreset::Multisample4x.name(), "4x MSAA");
    }

    #[test]
    fn test_color_write_mask() {
        let mut panel = RenderPipelinePanel::new();
        panel.write_red = true;
        panel.write_green = true;
        panel.write_blue = false;
        panel.write_alpha = false;

        panel.update_descriptor();
        // The test just verifies that update_descriptor doesn't panic
        // Actual color writes validation would require accessing descriptor internals
    }

    #[test]
    fn test_depth_stencil_enabled() {
        let mut panel = RenderPipelinePanel::new();
        panel.enable_depth_stencil = true;
        panel.depth_write_enabled = true;
        panel.depth_compare = CompareFunction::LessEqual;

        panel.update_descriptor();
        // The test just verifies that update_descriptor doesn't panic with depth-stencil enabled
    }

    #[test]
    fn test_blend_configuration() {
        let mut panel = RenderPipelinePanel::new();
        panel.blend_enabled = true;
        panel.color_blend_src = BlendFactor::SrcAlpha;
        panel.color_blend_dst = BlendFactor::OneMinusSrcAlpha;
        panel.color_blend_op = BlendOperation::Add;

        panel.update_descriptor();
        // The test just verifies that update_descriptor doesn't panic with blending enabled
    }
}
