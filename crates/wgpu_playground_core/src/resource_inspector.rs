use crate::buffer::BufferUsages;
use wgpu::{TextureDimension, TextureFormat, TextureUsages};

/// Type alias for unique resource identifiers
pub type ResourceId = u64;

/// Represents a tracked GPU buffer resource
#[derive(Debug, Clone)]
pub struct BufferInfo {
    /// Unique identifier for this resource
    pub id: ResourceId,
    /// Optional label for the buffer
    pub label: Option<String>,
    /// Size of the buffer in bytes
    pub size: u64,
    /// Usage flags for the buffer
    pub usage: BufferUsages,
    /// Whether the buffer was mapped at creation
    pub mapped_at_creation: bool,
    /// Current state of the buffer
    pub state: ResourceState,
}

/// Represents a tracked GPU texture resource
#[derive(Debug, Clone)]
pub struct TextureInfo {
    /// Unique identifier for this resource
    pub id: ResourceId,
    /// Optional label for the texture
    pub label: Option<String>,
    /// Width of the texture
    pub width: u32,
    /// Height of the texture
    pub height: u32,
    /// Depth or array layers
    pub depth_or_array_layers: u32,
    /// Texture dimension (1D, 2D, 3D)
    pub dimension: TextureDimension,
    /// Texture format
    pub format: TextureFormat,
    /// Mip level count
    pub mip_level_count: u32,
    /// Sample count
    pub sample_count: u32,
    /// Usage flags for the texture
    pub usage: TextureUsages,
    /// Current state of the texture
    pub state: ResourceState,
}

/// Represents a tracked render pipeline resource
#[derive(Debug, Clone)]
pub struct RenderPipelineInfo {
    /// Unique identifier for this resource
    pub id: ResourceId,
    /// Optional label for the pipeline
    pub label: Option<String>,
    /// Vertex entry point
    pub vertex_entry_point: String,
    /// Fragment entry point (if any)
    pub fragment_entry_point: Option<String>,
    /// Current state of the pipeline
    pub state: ResourceState,
}

/// Represents a tracked compute pipeline resource
#[derive(Debug, Clone)]
pub struct ComputePipelineInfo {
    /// Unique identifier for this resource
    pub id: ResourceId,
    /// Optional label for the pipeline
    pub label: Option<String>,
    /// Compute entry point
    pub entry_point: String,
    /// Current state of the pipeline
    pub state: ResourceState,
}

/// Current state of a resource
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceState {
    /// Resource is active and valid
    Active,
    /// Resource is being used
    InUse,
    /// Resource has been destroyed
    Destroyed,
}

impl ResourceState {
    /// Get a human-readable string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            ResourceState::Active => "Active",
            ResourceState::InUse => "In Use",
            ResourceState::Destroyed => "Destroyed",
        }
    }

    /// Get an emoji representation for UI
    pub fn emoji(&self) -> &'static str {
        match self {
            ResourceState::Active => "✓",
            ResourceState::InUse => "🔄",
            ResourceState::Destroyed => "❌",
        }
    }
}

/// Enum representing different types of tracked resources
#[derive(Debug, Clone)]
pub enum ResourceInfo {
    Buffer(BufferInfo),
    Texture(TextureInfo),
    RenderPipeline(RenderPipelineInfo),
    ComputePipeline(ComputePipelineInfo),
}

impl ResourceInfo {
    /// Get the label of the resource
    pub fn label(&self) -> Option<&str> {
        match self {
            ResourceInfo::Buffer(info) => info.label.as_deref(),
            ResourceInfo::Texture(info) => info.label.as_deref(),
            ResourceInfo::RenderPipeline(info) => info.label.as_deref(),
            ResourceInfo::ComputePipeline(info) => info.label.as_deref(),
        }
    }

    /// Get the state of the resource
    pub fn state(&self) -> ResourceState {
        match self {
            ResourceInfo::Buffer(info) => info.state,
            ResourceInfo::Texture(info) => info.state,
            ResourceInfo::RenderPipeline(info) => info.state,
            ResourceInfo::ComputePipeline(info) => info.state,
        }
    }

    /// Get the type name of the resource
    pub fn type_name(&self) -> &'static str {
        match self {
            ResourceInfo::Buffer(_) => "Buffer",
            ResourceInfo::Texture(_) => "Texture",
            ResourceInfo::RenderPipeline(_) => "Render Pipeline",
            ResourceInfo::ComputePipeline(_) => "Compute Pipeline",
        }
    }

    /// Calculate approximate memory usage in bytes
    pub fn memory_usage(&self) -> u64 {
        match self {
            ResourceInfo::Buffer(info) => info.size,
            ResourceInfo::Texture(info) => {
                // Calculate texture memory based on format, dimensions, and mip levels
                let bytes_per_pixel = Self::bytes_per_pixel(info.format);
                let mut total_bytes = 0u64;

                for mip in 0..info.mip_level_count {
                    let mip_width = (info.width >> mip).max(1);
                    let mip_height = (info.height >> mip).max(1);
                    let mip_depth = info.depth_or_array_layers;

                    total_bytes += (mip_width as u64)
                        * (mip_height as u64)
                        * (mip_depth as u64)
                        * bytes_per_pixel
                        * (info.sample_count as u64);
                }

                total_bytes
            }
            ResourceInfo::RenderPipeline(_) | ResourceInfo::ComputePipeline(_) => {
                // Pipelines have minimal CPU memory, mostly GPU shader code
                1024 // Estimate 1KB for pipeline state
            }
        }
    }

    /// Get bytes per pixel for a texture format (approximate)
    fn bytes_per_pixel(format: TextureFormat) -> u64 {
        match format {
            TextureFormat::R8Unorm
            | TextureFormat::R8Snorm
            | TextureFormat::R8Uint
            | TextureFormat::R8Sint => 1,
            TextureFormat::R16Uint | TextureFormat::R16Sint | TextureFormat::R16Float => 2,
            TextureFormat::Rg8Unorm
            | TextureFormat::Rg8Snorm
            | TextureFormat::Rg8Uint
            | TextureFormat::Rg8Sint => 2,
            TextureFormat::R32Uint | TextureFormat::R32Sint | TextureFormat::R32Float => 4,
            TextureFormat::Rg16Uint | TextureFormat::Rg16Sint | TextureFormat::Rg16Float => 4,
            TextureFormat::Rgba8Unorm
            | TextureFormat::Rgba8UnormSrgb
            | TextureFormat::Rgba8Snorm
            | TextureFormat::Rgba8Uint
            | TextureFormat::Rgba8Sint => 4,
            TextureFormat::Bgra8Unorm | TextureFormat::Bgra8UnormSrgb => 4,
            TextureFormat::Rgb10a2Unorm => 4,
            TextureFormat::Rg32Uint | TextureFormat::Rg32Sint | TextureFormat::Rg32Float => 8,
            TextureFormat::Rgba16Uint | TextureFormat::Rgba16Sint | TextureFormat::Rgba16Float => 8,
            TextureFormat::Rgba32Uint | TextureFormat::Rgba32Sint | TextureFormat::Rgba32Float => {
                16
            }
            TextureFormat::Depth32Float => 4,
            TextureFormat::Depth24Plus => 4,
            TextureFormat::Depth24PlusStencil8 => 4,
            TextureFormat::Stencil8 => 1,
            // Compressed formats - approximate sizes
            TextureFormat::Bc1RgbaUnorm | TextureFormat::Bc1RgbaUnormSrgb => 1, // 0.5 bytes per pixel average
            TextureFormat::Bc2RgbaUnorm | TextureFormat::Bc2RgbaUnormSrgb => 1,
            TextureFormat::Bc3RgbaUnorm | TextureFormat::Bc3RgbaUnormSrgb => 1,
            TextureFormat::Bc4RUnorm | TextureFormat::Bc4RSnorm => 1,
            TextureFormat::Bc5RgUnorm | TextureFormat::Bc5RgSnorm => 1,
            TextureFormat::Bc6hRgbUfloat | TextureFormat::Bc6hRgbFloat => 1,
            TextureFormat::Bc7RgbaUnorm | TextureFormat::Bc7RgbaUnormSrgb => 1,
            _ => 4, // Default estimate
        }
    }
}

/// Filter options for the resource inspector
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceFilter {
    All,
    Buffers,
    Textures,
    Pipelines,
}

impl ResourceFilter {
    /// Check if a resource matches this filter
    pub fn matches(&self, resource: &ResourceInfo) -> bool {
        match self {
            ResourceFilter::All => true,
            ResourceFilter::Buffers => matches!(resource, ResourceInfo::Buffer(_)),
            ResourceFilter::Textures => matches!(resource, ResourceInfo::Texture(_)),
            ResourceFilter::Pipelines => matches!(
                resource,
                ResourceInfo::RenderPipeline(_) | ResourceInfo::ComputePipeline(_)
            ),
        }
    }
}

/// UI panel for inspecting created GPU resources
pub struct ResourceInspectorPanel {
    /// List of tracked resources
    resources: Vec<ResourceInfo>,
    /// Current filter selection
    filter: ResourceFilter,
    /// Search query
    search_query: String,
    /// Whether to show destroyed resources
    show_destroyed: bool,
    /// Next available resource ID
    next_id: ResourceId,
}

impl Default for ResourceInspectorPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl ResourceInspectorPanel {
    /// Create a new resource inspector panel
    pub fn new() -> Self {
        Self {
            resources: Vec::new(),
            filter: ResourceFilter::All,
            search_query: String::new(),
            show_destroyed: false,
            next_id: 1,
        }
    }

    /// Get the next available resource ID and increment the counter
    fn get_next_id(&mut self) -> ResourceId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Add a buffer to the tracked resources
    pub fn add_buffer(&mut self, mut info: BufferInfo) {
        if info.id == 0 {
            info.id = self.get_next_id();
        }
        self.resources.push(ResourceInfo::Buffer(info));
    }

    /// Add a texture to the tracked resources
    pub fn add_texture(&mut self, mut info: TextureInfo) {
        if info.id == 0 {
            info.id = self.get_next_id();
        }
        self.resources.push(ResourceInfo::Texture(info));
    }

    /// Add a render pipeline to the tracked resources
    pub fn add_render_pipeline(&mut self, mut info: RenderPipelineInfo) {
        if info.id == 0 {
            info.id = self.get_next_id();
        }
        self.resources.push(ResourceInfo::RenderPipeline(info));
    }

    /// Add a compute pipeline to the tracked resources
    pub fn add_compute_pipeline(&mut self, mut info: ComputePipelineInfo) {
        if info.id == 0 {
            info.id = self.get_next_id();
        }
        self.resources.push(ResourceInfo::ComputePipeline(info));
    }

    /// Populate with demo resources for testing/demonstration
    pub fn add_demo_resources(&mut self) {
        // Add sample buffers
        self.add_buffer(BufferInfo {
            id: 0,
            label: Some("Vertex Buffer".to_string()),
            size: 4096,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
            state: ResourceState::Active,
        });

        self.add_buffer(BufferInfo {
            id: 0,
            label: Some("Index Buffer".to_string()),
            size: 2048,
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
            state: ResourceState::Active,
        });

        self.add_buffer(BufferInfo {
            id: 0,
            label: Some("Uniform Buffer".to_string()),
            size: 256,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
            state: ResourceState::InUse,
        });

        // Add sample textures
        self.add_texture(TextureInfo {
            id: 0,
            label: Some("Color Texture".to_string()),
            width: 1024,
            height: 1024,
            depth_or_array_layers: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            state: ResourceState::Active,
        });

        self.add_texture(TextureInfo {
            id: 0,
            label: Some("Depth Texture".to_string()),
            width: 1024,
            height: 1024,
            depth_or_array_layers: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Depth32Float,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::RENDER_ATTACHMENT,
            state: ResourceState::InUse,
        });

        // Add sample pipelines
        self.add_render_pipeline(RenderPipelineInfo {
            id: 0,
            label: Some("Main Render Pipeline".to_string()),
            vertex_entry_point: "vs_main".to_string(),
            fragment_entry_point: Some("fs_main".to_string()),
            state: ResourceState::Active,
        });

        self.add_compute_pipeline(ComputePipelineInfo {
            id: 0,
            label: Some("Compute Shader".to_string()),
            entry_point: "cs_main".to_string(),
            state: ResourceState::Active,
        });
    }

    /// Clear all tracked resources
    pub fn clear(&mut self) {
        self.resources.clear();
    }

    /// Get the number of tracked resources
    pub fn resource_count(&self) -> usize {
        self.resources.len()
    }

    /// Get all tracked resources (for testing)
    pub fn resources(&self) -> &[ResourceInfo] {
        &self.resources
    }

    /// Get filtered resources based on current filter and search
    fn filtered_resources(&self) -> Vec<&ResourceInfo> {
        self.resources
            .iter()
            .filter(|r| {
                // Apply type filter
                if !self.filter.matches(r) {
                    return false;
                }

                // Apply state filter
                if !self.show_destroyed && r.state() == ResourceState::Destroyed {
                    return false;
                }

                // Apply search filter
                if !self.search_query.is_empty() {
                    let query = self.search_query.to_lowercase();
                    let label_match = r
                        .label()
                        .map(|l| l.to_lowercase().contains(&query))
                        .unwrap_or(false);
                    let type_match = r.type_name().to_lowercase().contains(&query);

                    if !label_match && !type_match {
                        return false;
                    }
                }

                true
            })
            .collect()
    }

    /// Calculate total memory usage of filtered resources
    fn total_memory_usage(&self) -> u64 {
        self.filtered_resources()
            .iter()
            .map(|r| r.memory_usage())
            .sum()
    }

    /// Format bytes into human-readable string
    fn format_bytes(bytes: u64) -> String {
        if bytes < 1024 {
            format!("{} B", bytes)
        } else if bytes < 1024 * 1024 {
            format!("{:.2} KB", bytes as f64 / 1024.0)
        } else if bytes < 1024 * 1024 * 1024 {
            format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }

    /// Render the resource inspector UI
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("🔍 Resource Inspector");
        ui.label("View and manage all created GPU resources");
        ui.add_space(10.0);

        // Summary statistics
        ui.group(|ui| {
            ui.heading("Summary");
            ui.add_space(5.0);

            let filtered = self.filtered_resources();
            let total_memory = self.total_memory_usage();

            egui::Grid::new("resource_summary")
                .num_columns(2)
                .spacing([10.0, 5.0])
                .show(ui, |ui| {
                    ui.label("Total Resources:");
                    ui.label(format!("{}", self.resources.len()));
                    ui.end_row();

                    ui.label("Filtered Resources:");
                    ui.label(format!("{}", filtered.len()));
                    ui.end_row();

                    ui.label("Total Memory Usage:");
                    ui.label(Self::format_bytes(total_memory));
                    ui.end_row();
                });
        });

        ui.add_space(10.0);

        // Filter controls
        ui.group(|ui| {
            ui.heading("Filters");
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Type:");
                ui.radio_value(&mut self.filter, ResourceFilter::All, "All");
                ui.radio_value(&mut self.filter, ResourceFilter::Buffers, "Buffers");
                ui.radio_value(&mut self.filter, ResourceFilter::Textures, "Textures");
                ui.radio_value(&mut self.filter, ResourceFilter::Pipelines, "Pipelines");
            });

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Search:");
                ui.text_edit_singleline(&mut self.search_query);
                if ui.button("Clear").clicked() {
                    self.search_query.clear();
                }
            });

            ui.add_space(5.0);

            ui.checkbox(&mut self.show_destroyed, "Show destroyed resources");
        });

        ui.add_space(10.0);

        // Resource list
        ui.group(|ui| {
            ui.heading("Resources");
            ui.add_space(5.0);

            egui::ScrollArea::vertical()
                .max_height(400.0)
                .show(ui, |ui| {
                    let filtered = self.filtered_resources();

                    if filtered.is_empty() {
                        ui.label("No resources to display");
                    } else {
                        for resource in filtered {
                            self.render_resource_item(ui, resource);
                            ui.separator();
                        }
                    }
                });
        });

        ui.add_space(10.0);

        // Actions
        ui.horizontal(|ui| {
            if ui.button("🔄 Refresh").clicked() {
                // Refresh is automatic, this is just for user feedback
            }

            if ui.button("📝 Load Demo Data").clicked() {
                self.add_demo_resources();
            }

            if ui.button("🗑️ Clear All").clicked() {
                self.clear();
            }
        });
    }

    /// Render a single resource item
    fn render_resource_item(&self, ui: &mut egui::Ui, resource: &ResourceInfo) {
        ui.group(|ui| {
            // Header with type, label, and state
            ui.horizontal(|ui| {
                ui.strong(resource.type_name());
                ui.label("|");
                ui.label(resource.label().unwrap_or("<unlabeled>"));
                ui.label("|");
                ui.label(format!(
                    "{} {}",
                    resource.state().emoji(),
                    resource.state().as_str()
                ));
            });

            ui.add_space(5.0);

            // Resource-specific details
            match resource {
                ResourceInfo::Buffer(info) => self.render_buffer_details(ui, info),
                ResourceInfo::Texture(info) => self.render_texture_details(ui, info),
                ResourceInfo::RenderPipeline(info) => self.render_render_pipeline_details(ui, info),
                ResourceInfo::ComputePipeline(info) => {
                    self.render_compute_pipeline_details(ui, info)
                }
            }

            ui.add_space(5.0);

            // Memory usage
            ui.horizontal(|ui| {
                ui.label("Memory:");
                ui.monospace(Self::format_bytes(resource.memory_usage()));
            });
        });
    }

    /// Render buffer-specific details
    fn render_buffer_details(&self, ui: &mut egui::Ui, info: &BufferInfo) {
        egui::Grid::new(format!("buffer_{}", info.id))
            .num_columns(2)
            .spacing([10.0, 3.0])
            .show(ui, |ui| {
                ui.label("Size:");
                ui.monospace(Self::format_bytes(info.size));
                ui.end_row();

                ui.label("Mapped at creation:");
                ui.label(if info.mapped_at_creation { "Yes" } else { "No" });
                ui.end_row();

                ui.label("Usage:");
                ui.end_row();
            });

        // Display usage flags
        ui.indent(format!("usage_{}", info.id), |ui| {
            if info.usage.contains(BufferUsages::VERTEX) {
                ui.label("• VERTEX");
            }
            if info.usage.contains(BufferUsages::INDEX) {
                ui.label("• INDEX");
            }
            if info.usage.contains(BufferUsages::UNIFORM) {
                ui.label("• UNIFORM");
            }
            if info.usage.contains(BufferUsages::STORAGE) {
                ui.label("• STORAGE");
            }
            if info.usage.contains(BufferUsages::INDIRECT) {
                ui.label("• INDIRECT");
            }
            if info.usage.contains(BufferUsages::COPY_SRC) {
                ui.label("• COPY_SRC");
            }
            if info.usage.contains(BufferUsages::COPY_DST) {
                ui.label("• COPY_DST");
            }
            if info.usage.contains(BufferUsages::MAP_READ) {
                ui.label("• MAP_READ");
            }
            if info.usage.contains(BufferUsages::MAP_WRITE) {
                ui.label("• MAP_WRITE");
            }
            if info.usage.contains(BufferUsages::QUERY_RESOLVE) {
                ui.label("• QUERY_RESOLVE");
            }
        });
    }

    /// Render texture-specific details
    fn render_texture_details(&self, ui: &mut egui::Ui, info: &TextureInfo) {
        egui::Grid::new(format!("texture_{}", info.id))
            .num_columns(2)
            .spacing([10.0, 3.0])
            .show(ui, |ui| {
                ui.label("Dimension:");
                ui.monospace(format!("{:?}", info.dimension));
                ui.end_row();

                ui.label("Size:");
                ui.monospace(format!(
                    "{}x{}x{}",
                    info.width, info.height, info.depth_or_array_layers
                ));
                ui.end_row();

                ui.label("Format:");
                ui.monospace(format!("{:?}", info.format));
                ui.end_row();

                ui.label("Mip Levels:");
                ui.monospace(format!("{}", info.mip_level_count));
                ui.end_row();

                ui.label("Sample Count:");
                ui.monospace(format!("{}", info.sample_count));
                ui.end_row();

                ui.label("Usage:");
                ui.end_row();
            });

        // Display usage flags
        ui.indent(format!("usage_{}", info.id), |ui| {
            if info.usage.contains(TextureUsages::COPY_SRC) {
                ui.label("• COPY_SRC");
            }
            if info.usage.contains(TextureUsages::COPY_DST) {
                ui.label("• COPY_DST");
            }
            if info.usage.contains(TextureUsages::TEXTURE_BINDING) {
                ui.label("• TEXTURE_BINDING");
            }
            if info.usage.contains(TextureUsages::STORAGE_BINDING) {
                ui.label("• STORAGE_BINDING");
            }
            if info.usage.contains(TextureUsages::RENDER_ATTACHMENT) {
                ui.label("• RENDER_ATTACHMENT");
            }
        });
    }

    /// Render render pipeline-specific details
    fn render_render_pipeline_details(&self, ui: &mut egui::Ui, info: &RenderPipelineInfo) {
        egui::Grid::new(format!("render_pipeline_{}", info.id))
            .num_columns(2)
            .spacing([10.0, 3.0])
            .show(ui, |ui| {
                ui.label("Vertex Entry Point:");
                ui.monospace(&info.vertex_entry_point);
                ui.end_row();

                ui.label("Fragment Entry Point:");
                ui.monospace(info.fragment_entry_point.as_deref().unwrap_or("<none>"));
                ui.end_row();
            });
    }

    /// Render compute pipeline-specific details
    fn render_compute_pipeline_details(&self, ui: &mut egui::Ui, info: &ComputePipelineInfo) {
        egui::Grid::new(format!("compute_pipeline_{}", info.id))
            .num_columns(2)
            .spacing([10.0, 3.0])
            .show(ui, |ui| {
                ui.label("Entry Point:");
                ui.monospace(&info.entry_point);
                ui.end_row();
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_inspector_creation() {
        let panel = ResourceInspectorPanel::new();
        assert_eq!(panel.resource_count(), 0);
        assert_eq!(panel.filter, ResourceFilter::All);
        assert!(panel.search_query.is_empty());
        assert!(!panel.show_destroyed);
    }

    #[test]
    fn test_add_buffer() {
        let mut panel = ResourceInspectorPanel::new();
        let buffer_info = BufferInfo {
            id: 0,
            label: Some("test_buffer".to_string()),
            size: 1024,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
            state: ResourceState::Active,
        };

        panel.add_buffer(buffer_info);
        assert_eq!(panel.resource_count(), 1);
    }

    #[test]
    fn test_add_texture() {
        let mut panel = ResourceInspectorPanel::new();
        let texture_info = TextureInfo {
            id: 0,
            label: Some("test_texture".to_string()),
            width: 256,
            height: 256,
            depth_or_array_layers: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            state: ResourceState::Active,
        };

        panel.add_texture(texture_info);
        assert_eq!(panel.resource_count(), 1);
    }

    #[test]
    fn test_filter_buffers() {
        let mut panel = ResourceInspectorPanel::new();

        panel.add_buffer(BufferInfo {
            id: 0,
            label: Some("buffer1".to_string()),
            size: 1024,
            usage: BufferUsages::VERTEX,
            mapped_at_creation: false,
            state: ResourceState::Active,
        });

        panel.add_texture(TextureInfo {
            id: 0,
            label: Some("texture1".to_string()),
            width: 256,
            height: 256,
            depth_or_array_layers: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING,
            state: ResourceState::Active,
        });

        panel.filter = ResourceFilter::Buffers;
        let filtered = panel.filtered_resources();
        assert_eq!(filtered.len(), 1);
        assert!(matches!(filtered[0], ResourceInfo::Buffer(_)));
    }

    #[test]
    fn test_search_filter() {
        let mut panel = ResourceInspectorPanel::new();

        panel.add_buffer(BufferInfo {
            id: 0,
            label: Some("vertex_buffer".to_string()),
            size: 1024,
            usage: BufferUsages::VERTEX,
            mapped_at_creation: false,
            state: ResourceState::Active,
        });

        panel.add_buffer(BufferInfo {
            id: 0,
            label: Some("index_buffer".to_string()),
            size: 512,
            usage: BufferUsages::INDEX,
            mapped_at_creation: false,
            state: ResourceState::Active,
        });

        panel.search_query = "vertex".to_string();
        let filtered = panel.filtered_resources();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].label(), Some("vertex_buffer"));
    }

    #[test]
    fn test_memory_calculation_buffer() {
        let buffer_info = BufferInfo {
            id: 0,
            label: Some("test".to_string()),
            size: 2048,
            usage: BufferUsages::VERTEX,
            mapped_at_creation: false,
            state: ResourceState::Active,
        };

        let resource = ResourceInfo::Buffer(buffer_info);
        assert_eq!(resource.memory_usage(), 2048);
    }

    #[test]
    fn test_memory_calculation_texture() {
        let texture_info = TextureInfo {
            id: 0,
            label: Some("test".to_string()),
            width: 256,
            height: 256,
            depth_or_array_layers: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING,
            state: ResourceState::Active,
        };

        let resource = ResourceInfo::Texture(texture_info);
        // 256 * 256 * 1 * 4 bytes (RGBA8) * 1 sample = 262144 bytes
        assert_eq!(resource.memory_usage(), 262144);
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(ResourceInspectorPanel::format_bytes(512), "512 B");
        assert_eq!(ResourceInspectorPanel::format_bytes(1024), "1.00 KB");
        assert_eq!(ResourceInspectorPanel::format_bytes(1024 * 1024), "1.00 MB");
        assert_eq!(
            ResourceInspectorPanel::format_bytes(1024 * 1024 * 1024),
            "1.00 GB"
        );
    }

    #[test]
    fn test_resource_state() {
        assert_eq!(ResourceState::Active.as_str(), "Active");
        assert_eq!(ResourceState::InUse.as_str(), "In Use");
        assert_eq!(ResourceState::Destroyed.as_str(), "Destroyed");
    }

    #[test]
    fn test_clear_resources() {
        let mut panel = ResourceInspectorPanel::new();

        panel.add_buffer(BufferInfo {
            id: 0,
            label: Some("test".to_string()),
            size: 1024,
            usage: BufferUsages::VERTEX,
            mapped_at_creation: false,
            state: ResourceState::Active,
        });

        assert_eq!(panel.resource_count(), 1);
        panel.clear();
        assert_eq!(panel.resource_count(), 0);
    }

    #[test]
    fn test_show_destroyed_filter() {
        let mut panel = ResourceInspectorPanel::new();

        panel.add_buffer(BufferInfo {
            id: 0,
            label: Some("active".to_string()),
            size: 1024,
            usage: BufferUsages::VERTEX,
            mapped_at_creation: false,
            state: ResourceState::Active,
        });

        panel.add_buffer(BufferInfo {
            id: 0,
            label: Some("destroyed".to_string()),
            size: 1024,
            usage: BufferUsages::VERTEX,
            mapped_at_creation: false,
            state: ResourceState::Destroyed,
        });

        // By default, destroyed resources are hidden
        panel.show_destroyed = false;
        let filtered = panel.filtered_resources();
        assert_eq!(filtered.len(), 1);

        // When enabled, destroyed resources are shown
        panel.show_destroyed = true;
        let filtered = panel.filtered_resources();
        assert_eq!(filtered.len(), 2);
    }

    // GUI Interaction Tests - Simulating User Workflows

    #[test]
    fn test_gui_interaction_resource_lifecycle() {
        let mut panel = ResourceInspectorPanel::new();
        
        // User creates a buffer
        panel.add_buffer(BufferInfo {
            id: 1,
            label: Some("vertex_buffer".to_string()),
            size: 1024,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
            state: ResourceState::Active,
        });
        assert_eq!(panel.resources.len(), 1);
        
        // User can view the resource
        let resources = panel.filtered_resources();
        assert_eq!(resources.len(), 1);
        assert_eq!(resources[0].state(), ResourceState::Active);
    }

    #[test]
    fn test_gui_interaction_filter_by_resource_type() {
        let mut panel = ResourceInspectorPanel::new();
        
        // User creates different resources
        panel.add_buffer(BufferInfo {
            id: 1,
            label: Some("buffer".to_string()),
            size: 256,
            usage: BufferUsages::VERTEX,
            mapped_at_creation: false,
            state: ResourceState::Active,
        });
        
        panel.add_texture(TextureInfo {
            id: 1,
            label: Some("texture".to_string()),
            width: 256,
            height: 256,
            depth_or_array_layers: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING,
            state: ResourceState::Active,
        });
        
        assert_eq!(panel.resources.len(), 2);
        
        // User can filter resources
        panel.filter = ResourceFilter::Buffers;
        let filtered = panel.filtered_resources();
        assert_eq!(filtered.len(), 1);
        assert!(matches!(filtered[0], ResourceInfo::Buffer(_)));
        
        panel.filter = ResourceFilter::Textures;
        let filtered = panel.filtered_resources();
        assert_eq!(filtered.len(), 1);
        assert!(matches!(filtered[0], ResourceInfo::Texture(_)));
    }

    #[test]
    fn test_gui_interaction_search_workflow() {
        let mut panel = ResourceInspectorPanel::new();
        
        // User creates multiple buffers
        panel.add_buffer(BufferInfo {
            id: 1,
            label: Some("vertex_buffer".to_string()),
            size: 1024,
            usage: BufferUsages::VERTEX,
            mapped_at_creation: false,
            state: ResourceState::Active,
        });
        
        panel.add_buffer(BufferInfo {
            id: 2,
            label: Some("index_buffer".to_string()),
            size: 512,
            usage: BufferUsages::INDEX,
            mapped_at_creation: false,
            state: ResourceState::Active,
        });
        
        panel.add_buffer(BufferInfo {
            id: 3,
            label: Some("uniform_data".to_string()),
            size: 256,
            usage: BufferUsages::UNIFORM,
            mapped_at_creation: false,
            state: ResourceState::Active,
        });
        
        // User searches for "vertex" (in label)
        panel.search_query = "vertex".to_string();
        let filtered = panel.filtered_resources();
        assert_eq!(filtered.len(), 1); // vertex_buffer
        
        // User searches for "uniform"
        panel.search_query = "uniform".to_string();
        let filtered = panel.filtered_resources();
        assert_eq!(filtered.len(), 1); // uniform_data
        
        // User searches for "buffer" (matches type name)
        panel.search_query = "buffer".to_string();
        let filtered = panel.filtered_resources();
        assert_eq!(filtered.len(), 3); // All buffers (type name match)
        
        // User clears search
        panel.search_query = "".to_string();
        let filtered = panel.filtered_resources();
        assert_eq!(filtered.len(), 3); // All buffers
    }

    #[test]
    fn test_gui_interaction_clear_all() {
        let mut panel = ResourceInspectorPanel::new();
        
        // User creates various resources
        panel.add_buffer(BufferInfo {
            id: 1,
            label: Some("buffer".to_string()),
            size: 256,
            usage: BufferUsages::VERTEX,
            mapped_at_creation: false,
            state: ResourceState::Active,
        });
        
        panel.add_texture(TextureInfo {
            id: 1,
            label: Some("texture".to_string()),
            width: 256,
            height: 256,
            depth_or_array_layers: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING,
            state: ResourceState::Active,
        });
        
        panel.add_render_pipeline(RenderPipelineInfo {
            id: 1,
            label: Some("pipeline".to_string()),
            vertex_entry_point: "vs_main".to_string(),
            fragment_entry_point: Some("fs_main".to_string()),
            state: ResourceState::Active,
        });
        
        assert_eq!(panel.resources.len(), 3);
        
        // User clicks Clear All
        panel.clear();
        
        // All resources should be gone
        assert_eq!(panel.resources.len(), 0);
    }

    #[test]
    fn test_gui_interaction_memory_tracking() {
        let mut panel = ResourceInspectorPanel::new();
        
        // User creates a buffer and checks memory usage
        panel.add_buffer(BufferInfo {
            id: 1,
            label: Some("large_buffer".to_string()),
            size: 1048576, // 1 MB
            usage: BufferUsages::STORAGE,
            mapped_at_creation: false,
            state: ResourceState::Active,
        });
        
        let total_memory = panel.total_memory_usage();
        assert_eq!(total_memory, 1048576);
        
        // User creates a texture
        panel.add_texture(TextureInfo {
            id: 1,
            label: Some("texture".to_string()),
            width: 512,
            height: 512,
            depth_or_array_layers: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm, // 4 bytes per pixel
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING,
            state: ResourceState::Active,
        });
        
        // Memory should include both resources
        let new_total = panel.total_memory_usage();
        assert!(new_total > 1048576); // Should be buffer + texture
    }

    #[test]
    fn test_gui_interaction_hide_show_destroyed() {
        let mut panel = ResourceInspectorPanel::new();
        
        // User creates buffers
        panel.add_buffer(BufferInfo {
            id: 1,
            label: Some("active_buffer".to_string()),
            size: 256,
            usage: BufferUsages::VERTEX,
            mapped_at_creation: false,
            state: ResourceState::Active,
        });
        
        panel.add_buffer(BufferInfo {
            id: 2,
            label: Some("destroyed_buffer".to_string()),
            size: 256,
            usage: BufferUsages::VERTEX,
            mapped_at_creation: false,
            state: ResourceState::Destroyed,
        });
        
        // By default, destroyed resources hidden
        panel.show_destroyed = false;
        let filtered = panel.filtered_resources();
        assert_eq!(filtered.len(), 1);
        
        // User toggles show destroyed
        panel.show_destroyed = true;
        let filtered = panel.filtered_resources();
        assert_eq!(filtered.len(), 2);
        
        // User toggles it back off
        panel.show_destroyed = false;
        let filtered = panel.filtered_resources();
        assert_eq!(filtered.len(), 1);
    }
}
