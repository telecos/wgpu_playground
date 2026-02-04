use crate::model_loader::{load_model_from_file, ModelData};
use crate::assets;
use egui::{Color32, RichText};
use wgpu::Device;

/// UI panel for loading and managing 3D models
pub struct ModelLoaderPanel {
    // Input state
    filename_input: String,
    selected_format: ModelFormat,
    
    // Loaded model data
    current_model: Option<ModelData>,
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>,
    
    // UI feedback
    status_message: Option<StatusMessage>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ModelFormat {
    Obj,
    Gltf,
}

impl ModelFormat {
    fn name(&self) -> &'static str {
        match self {
            ModelFormat::Obj => "Wavefront OBJ (.obj)",
            ModelFormat::Gltf => "glTF 2.0 (.gltf/.glb)",
        }
    }
}

struct StatusMessage {
    text: String,
    is_error: bool,
}

impl Default for ModelLoaderPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl ModelLoaderPanel {
    pub fn new() -> Self {
        Self {
            filename_input: String::new(),
            selected_format: ModelFormat::Obj,
            current_model: None,
            vertex_buffer: None,
            index_buffer: None,
            status_message: None,
        }
    }

    /// Display the model loader panel UI
    pub fn show(&mut self, ui: &mut egui::Ui, device: &Device) {
        ui.heading("3D Model Loader");
        ui.add_space(10.0);

        ui.label("Load 3D models in glTF or OBJ format with support for materials and textures.");
        ui.add_space(10.0);

        // File selection section
        ui.group(|ui| {
            ui.label(RichText::new("Model File").strong());
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Filename:");
                ui.text_edit_singleline(&mut self.filename_input);
            });

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Format:");
                egui::ComboBox::from_id_salt("model_format")
                    .selected_text(self.selected_format.name())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.selected_format,
                            ModelFormat::Obj,
                            ModelFormat::Obj.name(),
                        );
                        ui.selectable_value(
                            &mut self.selected_format,
                            ModelFormat::Gltf,
                            ModelFormat::Gltf.name(),
                        );
                    });
            });

            ui.add_space(10.0);

            if ui.button("Load Model").clicked() {
                self.load_model(device);
            }
        });

        ui.add_space(10.0);

        // Status message
        if let Some(status) = &self.status_message {
            let color = if status.is_error {
                Color32::from_rgb(255, 100, 100)
            } else {
                Color32::from_rgb(100, 255, 100)
            };

            ui.colored_label(color, &status.text);
            ui.add_space(10.0);
        }

        // Display loaded model information
        if let Some(model) = &self.current_model {
            ui.separator();
            ui.add_space(10.0);
            self.show_model_info(ui, model);
        }

        ui.add_space(10.0);

        // Help section
        ui.collapsing("Help & Examples", |ui| {
            ui.label("Place your model files in the assets/models/ directory.");
            ui.add_space(5.0);
            
            ui.label(RichText::new("Supported formats:").strong());
            ui.label("• Wavefront OBJ (.obj) - Simple mesh format with material support");
            ui.label("• glTF 2.0 (.gltf, .glb) - Modern format with full PBR material support");
            ui.add_space(5.0);
            
            ui.label(RichText::new("Example filenames:").strong());
            ui.label("• cube.obj");
            ui.label("• character.gltf");
            ui.label("• scene.glb");
        });
    }

    /// Load a model from the specified file
    fn load_model(&mut self, device: &Device) {
        self.status_message = None;

        if self.filename_input.trim().is_empty() {
            self.status_message = Some(StatusMessage {
                text: "Please enter a filename".to_string(),
                is_error: true,
            });
            return;
        }

        // Construct the full path
        let path = assets::models_dir().join(&self.filename_input);

        // Load the model
        match load_model_from_file(&path) {
            Ok(model) => {
                // Create GPU buffers
                match model.create_buffers(device) {
                    Ok((vertex_buffer, index_buffer)) => {
                        self.status_message = Some(StatusMessage {
                            text: format!("Successfully loaded model: {}", self.filename_input),
                            is_error: false,
                        });
                        
                        self.vertex_buffer = Some(vertex_buffer);
                        self.index_buffer = Some(index_buffer);
                        self.current_model = Some(model);
                    }
                    Err(e) => {
                        self.status_message = Some(StatusMessage {
                            text: format!("Failed to create GPU buffers: {}", e),
                            is_error: true,
                        });
                    }
                }
            }
            Err(e) => {
                self.status_message = Some(StatusMessage {
                    text: format!("Failed to load model: {}", e),
                    is_error: true,
                });
            }
        }
    }

    /// Display information about the loaded model
    fn show_model_info(&self, ui: &mut egui::Ui, model: &ModelData) {
        ui.heading("Loaded Model Information");
        ui.add_space(5.0);

        ui.group(|ui| {
            ui.label(RichText::new("Geometry").strong());
            ui.label(format!("Meshes: {}", model.meshes.len()));
            ui.label(format!("Total Vertices: {}", model.vertex_count));
            ui.label(format!("Total Indices: {}", model.index_count));
            ui.label(format!(
                "Triangles: {}",
                model.index_count / 3
            ));
        });

        ui.add_space(5.0);

        if !model.materials.is_empty() {
            ui.group(|ui| {
                ui.label(RichText::new("Materials").strong());
                ui.label(format!("Material Count: {}", model.materials.len()));
                
                ui.add_space(5.0);
                
                egui::ScrollArea::vertical()
                    .max_height(200.0)
                    .show(ui, |ui| {
                        for (i, material) in model.materials.iter().enumerate() {
                            ui.collapsing(format!("Material {}: {}", i, material.name), |ui| {
                                ui.label(format!(
                                    "Diffuse Color: [{:.2}, {:.2}, {:.2}, {:.2}]",
                                    material.diffuse_color[0],
                                    material.diffuse_color[1],
                                    material.diffuse_color[2],
                                    material.diffuse_color[3]
                                ));
                                
                                if let Some(tex) = &material.diffuse_texture {
                                    ui.label(format!("Diffuse Texture: {}", tex));
                                }
                                
                                if let Some(spec) = &material.specular_color {
                                    ui.label(format!(
                                        "Specular Color: [{:.2}, {:.2}, {:.2}]",
                                        spec[0], spec[1], spec[2]
                                    ));
                                }
                                
                                if let Some(shininess) = material.shininess {
                                    ui.label(format!("Shininess: {:.2}", shininess));
                                }
                            });
                        }
                    });
            });
        }

        ui.add_space(5.0);

        // Mesh details
        if !model.meshes.is_empty() {
            ui.group(|ui| {
                ui.label(RichText::new("Mesh Details").strong());
                
                egui::ScrollArea::vertical()
                    .max_height(200.0)
                    .show(ui, |ui| {
                        for (i, mesh) in model.meshes.iter().enumerate() {
                            ui.collapsing(format!("Mesh {}", i), |ui| {
                                ui.label(format!("Vertices: {}", mesh.vertices.len()));
                                ui.label(format!("Indices: {}", mesh.indices.len()));
                                ui.label(format!("Triangles: {}", mesh.indices.len() / 3));
                                
                                if let Some(mat_idx) = mesh.material_index {
                                    if let Some(material) = model.materials.get(mat_idx) {
                                        ui.label(format!("Material: {}", material.name));
                                    }
                                }
                            });
                        }
                    });
            });
        }
    }

    /// Get the vertex buffer if a model is loaded
    pub fn vertex_buffer(&self) -> Option<&wgpu::Buffer> {
        self.vertex_buffer.as_ref()
    }

    /// Get the index buffer if a model is loaded
    pub fn index_buffer(&self) -> Option<&wgpu::Buffer> {
        self.index_buffer.as_ref()
    }

    /// Get the current model data
    pub fn current_model(&self) -> Option<&ModelData> {
        self.current_model.as_ref()
    }
}
