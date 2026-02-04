use std::path::Path;
use wgpu::{Buffer, Device};
use wgpu::util::DeviceExt;

/// Errors that can occur during model loading
#[derive(Debug)]
pub enum ModelLoadError {
    /// Failed to load file
    IoError(std::io::Error),
    /// Failed to parse model data
    ParseError(String),
    /// Unsupported model format
    UnsupportedFormat(String),
    /// Missing required data (e.g., normals, UVs)
    MissingData(String),
}

impl std::fmt::Display for ModelLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelLoadError::IoError(err) => write!(f, "I/O error: {}", err),
            ModelLoadError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ModelLoadError::UnsupportedFormat(msg) => write!(f, "Unsupported format: {}", msg),
            ModelLoadError::MissingData(msg) => write!(f, "Missing data: {}", msg),
        }
    }
}

impl std::error::Error for ModelLoadError {}

impl From<std::io::Error> for ModelLoadError {
    fn from(err: std::io::Error) -> Self {
        ModelLoadError::IoError(err)
    }
}

/// Vertex data structure for 3D models
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ModelVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
}

unsafe impl bytemuck::Pod for ModelVertex {}
unsafe impl bytemuck::Zeroable for ModelVertex {}

impl ModelVertex {
    pub fn new(position: [f32; 3], normal: [f32; 3], tex_coords: [f32; 2]) -> Self {
        Self {
            position,
            normal,
            tex_coords,
        }
    }
}

/// Material information from the model
#[derive(Debug, Clone)]
pub struct Material {
    pub name: String,
    pub diffuse_color: [f32; 4],
    pub diffuse_texture: Option<String>,
    pub specular_color: Option<[f32; 3]>,
    pub shininess: Option<f32>,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            diffuse_color: [1.0, 1.0, 1.0, 1.0],
            diffuse_texture: None,
            specular_color: None,
            shininess: None,
        }
    }
}

/// A mesh within a 3D model
#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<ModelVertex>,
    pub indices: Vec<u32>,
    pub material_index: Option<usize>,
}

/// Loaded 3D model data
pub struct ModelData {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
    pub vertex_count: u32,
    pub index_count: u32,
}

impl ModelData {
    /// Create GPU buffers from the model data
    pub fn create_buffers(&self, device: &Device) -> Result<(Buffer, Buffer), ModelLoadError> {
        // Flatten all vertices from all meshes
        let mut all_vertices = Vec::new();
        let mut all_indices = Vec::new();
        
        for mesh in &self.meshes {
            let vertex_offset = all_vertices.len() as u32;
            all_vertices.extend_from_slice(&mesh.vertices);
            
            // Adjust indices for the current vertex offset
            for &index in &mesh.indices {
                all_indices.push(index + vertex_offset);
            }
        }
        
        if all_vertices.is_empty() {
            return Err(ModelLoadError::MissingData("No vertices in model".to_string()));
        }
        
        // Create vertex buffer
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Model Vertex Buffer"),
            contents: bytemuck::cast_slice(&all_vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });
        
        // Create index buffer
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Model Index Buffer"),
            contents: bytemuck::cast_slice(&all_indices),
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        });
        
        Ok((vertex_buffer, index_buffer))
    }
}

/// Load a 3D model from a file
pub fn load_model_from_file(path: &Path) -> Result<ModelData, ModelLoadError> {
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .ok_or_else(|| ModelLoadError::UnsupportedFormat("No file extension".to_string()))?;
    
    match extension.to_lowercase().as_str() {
        "obj" => load_obj(path),
        "gltf" | "glb" => load_gltf(path),
        _ => Err(ModelLoadError::UnsupportedFormat(format!(
            "Unsupported file extension: {}",
            extension
        ))),
    }
}

/// Load an OBJ model
fn load_obj(path: &Path) -> Result<ModelData, ModelLoadError> {
    let (models, materials) = tobj::load_obj(
        path,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
    )
    .map_err(|e| ModelLoadError::ParseError(format!("Failed to load OBJ: {}", e)))?;
    
    let mut meshes = Vec::new();
    let mut model_materials = Vec::new();
    
    // Load materials
    if let Ok(mats) = materials {
        for mat in mats {
            model_materials.push(Material {
                name: mat.name.clone(),
                diffuse_color: [
                    mat.diffuse.unwrap_or([1.0, 1.0, 1.0])[0],
                    mat.diffuse.unwrap_or([1.0, 1.0, 1.0])[1],
                    mat.diffuse.unwrap_or([1.0, 1.0, 1.0])[2],
                    mat.dissolve.unwrap_or(1.0),
                ],
                diffuse_texture: mat.diffuse_texture,
                specular_color: mat.specular,
                shininess: mat.shininess,
            });
        }
    }
    
    let mut total_vertices = 0;
    let mut total_indices = 0;
    
    for model in models {
        let mesh = &model.mesh;
        let mut vertices = Vec::new();
        
        // Build vertices
        for i in 0..mesh.positions.len() / 3 {
            let position = [
                mesh.positions[i * 3],
                mesh.positions[i * 3 + 1],
                mesh.positions[i * 3 + 2],
            ];
            
            let normal = if !mesh.normals.is_empty() {
                [
                    mesh.normals[i * 3],
                    mesh.normals[i * 3 + 1],
                    mesh.normals[i * 3 + 2],
                ]
            } else {
                [0.0, 1.0, 0.0] // Default normal
            };
            
            let tex_coords = if !mesh.texcoords.is_empty() {
                [mesh.texcoords[i * 2], mesh.texcoords[i * 2 + 1]]
            } else {
                [0.0, 0.0] // Default UVs
            };
            
            vertices.push(ModelVertex::new(position, normal, tex_coords));
        }
        
        total_vertices += vertices.len() as u32;
        total_indices += mesh.indices.len() as u32;
        
        meshes.push(Mesh {
            vertices,
            indices: mesh.indices.clone(),
            material_index: mesh.material_id,
        });
    }
    
    Ok(ModelData {
        meshes,
        materials: model_materials,
        vertex_count: total_vertices,
        index_count: total_indices,
    })
}

/// Load a glTF/GLB model
fn load_gltf(path: &Path) -> Result<ModelData, ModelLoadError> {
    let (document, buffers, _images) = gltf::import(path)
        .map_err(|e| ModelLoadError::ParseError(format!("Failed to load glTF: {}", e)))?;
    
    let mut meshes = Vec::new();
    let mut materials = Vec::new();
    
    // Load materials
    for material in document.materials() {
        let pbr = material.pbr_metallic_roughness();
        let base_color = pbr.base_color_factor();
        
        materials.push(Material {
            name: "material".to_string(), // gltf 1.4 doesn't expose name directly
            diffuse_color: base_color,
            diffuse_texture: pbr
                .base_color_texture()
                .map(|tex| format!("texture_{}", tex.texture().index())),
            specular_color: None,
            shininess: None,
        });
    }
    
    let mut total_vertices = 0;
    let mut total_indices = 0;
    
    // Load meshes
    for mesh in document.meshes() {
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            
            // Read positions
            let positions: Vec<[f32; 3]> = reader
                .read_positions()
                .ok_or_else(|| ModelLoadError::MissingData("Missing positions".to_string()))?
                .collect();
            
            // Read normals (or generate default)
            let normals: Vec<[f32; 3]> = reader
                .read_normals()
                .map(|iter| iter.collect())
                .unwrap_or_else(|| vec![[0.0, 1.0, 0.0]; positions.len()]);
            
            // Read texture coordinates (or generate default)
            let tex_coords: Vec<[f32; 2]> = reader
                .read_tex_coords(0)
                .map(|iter| iter.into_f32().collect())
                .unwrap_or_else(|| vec![[0.0, 0.0]; positions.len()]);
            
            // Build vertices
            // Note: normals and tex_coords are guaranteed to be the same length as positions
            // due to the unwrap_or_else fallbacks above that create default arrays
            let mut vertices = Vec::new();
            for i in 0..positions.len() {
                vertices.push(ModelVertex::new(
                    positions[i],
                    normals[i],
                    tex_coords[i],
                ));
            }
            
            // Read indices
            let indices: Vec<u32> = reader
                .read_indices()
                .ok_or_else(|| ModelLoadError::MissingData("Missing indices".to_string()))?
                .into_u32()
                .collect();
            
            total_vertices += vertices.len() as u32;
            total_indices += indices.len() as u32;
            
            meshes.push(Mesh {
                vertices,
                indices,
                material_index: primitive.material().index(),
            });
        }
    }
    
    Ok(ModelData {
        meshes,
        materials,
        vertex_count: total_vertices,
        index_count: total_indices,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_vertex_size() {
        assert_eq!(std::mem::size_of::<ModelVertex>(), 32);
    }

    #[test]
    fn test_material_default() {
        let material = Material::default();
        assert_eq!(material.name, "default");
        assert_eq!(material.diffuse_color, [1.0, 1.0, 1.0, 1.0]);
    }
}
