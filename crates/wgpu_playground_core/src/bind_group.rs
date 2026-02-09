use std::fmt;
use std::num::{NonZeroU32, NonZeroU64};
use wgpu::{BindGroup, BindGroupLayout, Buffer, Device, Sampler, ShaderStages, TextureView};

use crate::api_coverage::{ApiCategory, ApiCoverageTracker};

/// Errors that can occur during bind group layout operations
#[derive(Debug)]
pub enum BindGroupError {
    /// Invalid binding configuration
    InvalidBinding(String),
    /// Binding number conflict
    DuplicateBinding(u32),
}

impl fmt::Display for BindGroupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BindGroupError::InvalidBinding(msg) => write!(f, "Invalid binding: {}", msg),
            BindGroupError::DuplicateBinding(binding) => {
                write!(f, "Duplicate binding number: {}", binding)
            }
        }
    }
}

impl std::error::Error for BindGroupError {}

/// Describes the type of resource that can be bound
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingType {
    /// Uniform buffer binding
    UniformBuffer {
        /// Whether this binding has dynamic offsets
        has_dynamic_offset: bool,
        /// Minimum size of the binding (optional, must be non-zero)
        min_binding_size: Option<NonZeroU64>,
    },
    /// Storage buffer binding
    StorageBuffer {
        /// Whether this binding has dynamic offsets
        has_dynamic_offset: bool,
        /// Minimum size of the binding (optional, must be non-zero)
        min_binding_size: Option<NonZeroU64>,
        /// Whether the buffer is read-only
        read_only: bool,
    },
    /// Sampled texture binding
    Texture {
        /// Sample type (float, signed int, unsigned int, depth)
        sample_type: TextureSampleType,
        /// View dimension (1D, 2D, 2D Array, Cube, Cube Array, 3D)
        view_dimension: TextureViewDimension,
        /// Whether the texture is multisampled
        multisampled: bool,
    },
    /// Sampler binding
    Sampler {
        /// Sampler binding type (filtering, non-filtering, comparison)
        sampler_type: SamplerBindingType,
    },
    /// Storage texture binding
    StorageTexture {
        /// Access mode (write-only, read-only, read-write)
        access: StorageTextureAccess,
        /// Texture format
        format: wgpu::TextureFormat,
        /// View dimension (1D, 2D, 2D Array, Cube, Cube Array, 3D)
        view_dimension: TextureViewDimension,
    },
}

/// Texture sample type for sampled textures
///
/// Defines how texture data is interpreted when sampled in a shader.
///
/// # Examples
///
/// Filterable float texture (most common):
/// ```
/// use wgpu_playground_core::bind_group::TextureSampleType;
///
/// let sample_type = TextureSampleType::Float { filterable: true };
/// // Use for color textures with linear/trilinear filtering
/// // Formats: Rgba8Unorm, Rgba16Float, etc.
/// ```
///
/// Non-filterable float texture (HDR formats):
/// ```
/// use wgpu_playground_core::bind_group::TextureSampleType;
///
/// let sample_type = TextureSampleType::Float { filterable: false };
/// // Use for high-precision formats that don't support filtering
/// // Formats: Rgba32Float, R32Float
/// ```
///
/// Signed integer texture:
/// ```
/// use wgpu_playground_core::bind_group::TextureSampleType;
///
/// let sample_type = TextureSampleType::Sint;
/// // Use for signed integer textures
/// // Formats: Rgba32Sint, R32Sint
/// // Sampled as ivec4 in shaders
/// ```
///
/// Unsigned integer texture:
/// ```
/// use wgpu_playground_core::bind_group::TextureSampleType;
///
/// let sample_type = TextureSampleType::Uint;
/// // Use for unsigned integer textures
/// // Formats: Rgba32Uint, R32Uint
/// // Sampled as uvec4 in shaders
/// ```
///
/// Depth texture:
/// ```
/// use wgpu_playground_core::bind_group::TextureSampleType;
///
/// let sample_type = TextureSampleType::Depth;
/// // Use for depth textures
/// // Formats: Depth32Float, Depth24Plus
/// // Sampled as float in shaders
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureSampleType {
    /// Floating point texture
    Float { filterable: bool },
    /// Signed integer texture
    Sint,
    /// Unsigned integer texture
    Uint,
    /// Depth texture
    Depth,
}

/// Texture view dimension
///
/// Defines the dimensionality of a texture view for binding.
///
/// # Examples
///
/// 2D texture (most common):
/// ```
/// use wgpu_playground_core::bind_group::TextureViewDimension;
///
/// let dimension = TextureViewDimension::D2;
/// // Standard 2D textures (albedo, normal maps, etc.)
/// ```
///
/// 1D texture (gradients, lookup tables):
/// ```
/// use wgpu_playground_core::bind_group::TextureViewDimension;
///
/// let dimension = TextureViewDimension::D1;
/// // One-dimensional textures (color ramps, 1D noise)
/// ```
///
/// 2D array texture:
/// ```
/// use wgpu_playground_core::bind_group::TextureViewDimension;
///
/// let dimension = TextureViewDimension::D2Array;
/// // Array of 2D textures (texture atlases, animation frames)
/// ```
///
/// Cube map texture:
/// ```
/// use wgpu_playground_core::bind_group::TextureViewDimension;
///
/// let dimension = TextureViewDimension::Cube;
/// // Cube maps for skyboxes, environment mapping
/// ```
///
/// 3D texture (volume data):
/// ```
/// use wgpu_playground_core::bind_group::TextureViewDimension;
///
/// let dimension = TextureViewDimension::D3;
/// // 3D textures for volumes, voxels, 3D noise
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureViewDimension {
    D1,
    D2,
    D2Array,
    Cube,
    CubeArray,
    D3,
}

/// Sampler binding type
///
/// Defines the type of sampler for texture sampling operations.
///
/// # Examples
///
/// Filtering sampler (linear/trilinear):
/// ```
/// use wgpu_playground_core::bind_group::SamplerBindingType;
///
/// let sampler_type = SamplerBindingType::Filtering;
/// // Use for smooth texture sampling with interpolation
/// // Supports minFilter/magFilter/mipmapFilter
/// ```
///
/// Non-filtering sampler (nearest neighbor):
/// ```
/// use wgpu_playground_core::bind_group::SamplerBindingType;
///
/// let sampler_type = SamplerBindingType::NonFiltering;
/// // Samples exact texel values without interpolation
/// // Use for pixel-perfect rendering, data textures
/// ```
///
/// Comparison sampler (for shadow mapping):
/// ```
/// use wgpu_playground_core::bind_group::SamplerBindingType;
///
/// let sampler_type = SamplerBindingType::Comparison;
/// // Compares sampled depth value with reference
/// // Used for percentage-closer filtering (PCF) shadows
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SamplerBindingType {
    /// Filtering sampler
    Filtering,
    /// Non-filtering sampler
    NonFiltering,
    /// Comparison sampler
    Comparison,
}

/// Storage texture access mode
///
/// Defines how a storage texture can be accessed in compute or fragment shaders.
///
/// # Examples
///
/// Write-only storage texture (compute output):
/// ```
/// use wgpu_playground_core::bind_group::StorageTextureAccess;
///
/// let access = StorageTextureAccess::WriteOnly;
/// // Common for compute shader outputs
/// // Use image2D in GLSL, texture_storage_2d in WGSL
/// ```
///
/// Read-only storage texture (data input):
/// ```
/// use wgpu_playground_core::bind_group::StorageTextureAccess;
///
/// let access = StorageTextureAccess::ReadOnly;
/// // Read texture data without sampling
/// // Direct texel access in compute shaders
/// ```
///
/// Read-write storage texture (in-place modification):
/// ```
/// use wgpu_playground_core::bind_group::StorageTextureAccess;
///
/// let access = StorageTextureAccess::ReadWrite;
/// // Both read and write in same shader
/// // Use for image processing, accumulation
/// // Note: Device and format support required
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageTextureAccess {
    /// Write-only access
    WriteOnly,
    /// Read-only access
    ReadOnly,
    /// Read-write access
    ReadWrite,
}

/// A single binding entry in a bind group layout
#[derive(Debug, Clone)]
pub struct BindGroupLayoutEntry {
    /// Binding number
    pub binding: u32,
    /// Shader stages this binding is visible to
    pub visibility: ShaderStages,
    /// Type of binding
    pub ty: BindingType,
    /// Optional count for binding arrays (None for non-array bindings, must be non-zero)
    pub count: Option<NonZeroU32>,
}

impl BindGroupLayoutEntry {
    /// Create a new bind group layout entry
    ///
    /// # Arguments
    /// * `binding` - The binding number
    /// * `visibility` - Shader stages where this binding is visible
    /// * `ty` - Type of binding
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::bind_group::{BindGroupLayoutEntry, BindingType};
    /// use wgpu::ShaderStages;
    ///
    /// let entry = BindGroupLayoutEntry::new(
    ///     0,
    ///     ShaderStages::VERTEX,
    ///     BindingType::UniformBuffer {
    ///         has_dynamic_offset: false,
    ///         min_binding_size: None,
    ///     }
    /// );
    /// ```
    pub fn new(binding: u32, visibility: ShaderStages, ty: BindingType) -> Self {
        Self {
            binding,
            visibility,
            ty,
            count: None,
        }
    }

    /// Set the count for binding arrays
    ///
    /// # Arguments
    /// * `count` - The number of array elements (must be non-zero)
    pub fn with_count(mut self, count: NonZeroU32) -> Self {
        self.count = Some(count);
        self
    }

    /// Convert to wgpu::BindGroupLayoutEntry
    pub fn to_wgpu(&self) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: self.binding,
            visibility: self.visibility,
            ty: self.ty.to_wgpu(),
            count: self.count,
        }
    }
}

impl BindingType {
    /// Convert to wgpu::BindingType
    pub fn to_wgpu(&self) -> wgpu::BindingType {
        match self {
            BindingType::UniformBuffer {
                has_dynamic_offset,
                min_binding_size,
            } => wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: *has_dynamic_offset,
                min_binding_size: *min_binding_size,
            },
            BindingType::StorageBuffer {
                has_dynamic_offset,
                min_binding_size,
                read_only,
            } => wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage {
                    read_only: *read_only,
                },
                has_dynamic_offset: *has_dynamic_offset,
                min_binding_size: *min_binding_size,
            },
            BindingType::Texture {
                sample_type,
                view_dimension,
                multisampled,
            } => wgpu::BindingType::Texture {
                sample_type: sample_type.to_wgpu(),
                view_dimension: view_dimension.to_wgpu(),
                multisampled: *multisampled,
            },
            BindingType::Sampler { sampler_type } => {
                wgpu::BindingType::Sampler(sampler_type.to_wgpu())
            }
            BindingType::StorageTexture {
                access,
                format,
                view_dimension,
            } => wgpu::BindingType::StorageTexture {
                access: access.to_wgpu(),
                format: *format,
                view_dimension: view_dimension.to_wgpu(),
            },
        }
    }
}

impl TextureSampleType {
    /// Convert to wgpu::TextureSampleType
    pub fn to_wgpu(&self) -> wgpu::TextureSampleType {
        match self {
            TextureSampleType::Float { filterable } => wgpu::TextureSampleType::Float {
                filterable: *filterable,
            },
            TextureSampleType::Sint => wgpu::TextureSampleType::Sint,
            TextureSampleType::Uint => wgpu::TextureSampleType::Uint,
            TextureSampleType::Depth => wgpu::TextureSampleType::Depth,
        }
    }
}

impl TextureViewDimension {
    /// Convert to wgpu::TextureViewDimension
    pub fn to_wgpu(&self) -> wgpu::TextureViewDimension {
        match self {
            TextureViewDimension::D1 => wgpu::TextureViewDimension::D1,
            TextureViewDimension::D2 => wgpu::TextureViewDimension::D2,
            TextureViewDimension::D2Array => wgpu::TextureViewDimension::D2Array,
            TextureViewDimension::Cube => wgpu::TextureViewDimension::Cube,
            TextureViewDimension::CubeArray => wgpu::TextureViewDimension::CubeArray,
            TextureViewDimension::D3 => wgpu::TextureViewDimension::D3,
        }
    }
}

impl SamplerBindingType {
    /// Convert to wgpu::SamplerBindingType
    pub fn to_wgpu(&self) -> wgpu::SamplerBindingType {
        match self {
            SamplerBindingType::Filtering => wgpu::SamplerBindingType::Filtering,
            SamplerBindingType::NonFiltering => wgpu::SamplerBindingType::NonFiltering,
            SamplerBindingType::Comparison => wgpu::SamplerBindingType::Comparison,
        }
    }
}

impl StorageTextureAccess {
    /// Convert to wgpu::StorageTextureAccess
    pub fn to_wgpu(&self) -> wgpu::StorageTextureAccess {
        match self {
            StorageTextureAccess::WriteOnly => wgpu::StorageTextureAccess::WriteOnly,
            StorageTextureAccess::ReadOnly => wgpu::StorageTextureAccess::ReadOnly,
            StorageTextureAccess::ReadWrite => wgpu::StorageTextureAccess::ReadWrite,
        }
    }
}

/// Descriptor for creating a bind group layout
#[derive(Debug, Clone)]
pub struct BindGroupLayoutDescriptor {
    /// Optional label for debugging
    label: Option<String>,
    /// Entries in the bind group layout
    entries: Vec<BindGroupLayoutEntry>,
}

impl BindGroupLayoutDescriptor {
    /// Create a new bind group layout descriptor
    ///
    /// # Arguments
    /// * `label` - Optional label for debugging
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::bind_group::BindGroupLayoutDescriptor;
    ///
    /// let descriptor = BindGroupLayoutDescriptor::new(Some("my_layout"));
    /// ```
    pub fn new(label: Option<&str>) -> Self {
        Self {
            label: label.map(String::from),
            entries: Vec::new(),
        }
    }

    /// Add a binding entry to the layout
    ///
    /// # Arguments
    /// * `entry` - The bind group layout entry to add
    ///
    /// # Returns
    /// Self for method chaining
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::bind_group::{BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType};
    /// use wgpu::ShaderStages;
    ///
    /// let descriptor = BindGroupLayoutDescriptor::new(Some("my_layout"))
    ///     .with_entry(BindGroupLayoutEntry::new(
    ///         0,
    ///         ShaderStages::VERTEX,
    ///         BindingType::UniformBuffer {
    ///             has_dynamic_offset: false,
    ///             min_binding_size: None,
    ///         }
    ///     ));
    /// ```
    pub fn with_entry(mut self, entry: BindGroupLayoutEntry) -> Self {
        self.entries.push(entry);
        self
    }

    /// Add multiple binding entries to the layout
    ///
    /// # Arguments
    /// * `entries` - A slice of bind group layout entries to add
    ///
    /// # Returns
    /// Self for method chaining
    pub fn with_entries(mut self, entries: &[BindGroupLayoutEntry]) -> Self {
        self.entries.extend_from_slice(entries);
        self
    }

    /// Get the label
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Get the entries
    pub fn entries(&self) -> &[BindGroupLayoutEntry] {
        &self.entries
    }

    /// Validate the layout descriptor
    ///
    /// Checks for:
    /// - Duplicate binding numbers
    /// - Empty entry list
    ///
    /// # Returns
    /// Ok(()) if valid, Err with BindGroupError if invalid
    pub fn validate(&self) -> Result<(), BindGroupError> {
        if self.entries.is_empty() {
            return Err(BindGroupError::InvalidBinding(
                "Bind group layout must have at least one entry".to_string(),
            ));
        }

        // Check for duplicate binding numbers
        let mut bindings = std::collections::HashSet::new();
        for entry in &self.entries {
            if !bindings.insert(entry.binding) {
                return Err(BindGroupError::DuplicateBinding(entry.binding));
            }
        }

        Ok(())
    }

    /// Create a wgpu bind group layout from this descriptor
    ///
    /// This method validates the descriptor and creates the actual bind group layout.
    ///
    /// # Arguments
    /// * `device` - The wgpu device to create the layout on
    ///
    /// # Returns
    /// A Result containing the BindGroupLayout or a BindGroupError
    ///
    /// # Examples
    /// ```no_run
    /// use wgpu_playground_core::bind_group::{BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType};
    /// use wgpu::ShaderStages;
    /// # async fn example(device: &wgpu::Device) {
    /// let descriptor = BindGroupLayoutDescriptor::new(Some("my_layout"))
    ///     .with_entry(BindGroupLayoutEntry::new(
    ///         0,
    ///         ShaderStages::VERTEX | ShaderStages::FRAGMENT,
    ///         BindingType::UniformBuffer {
    ///             has_dynamic_offset: false,
    ///             min_binding_size: None,
    ///         }
    ///     ));
    ///
    /// let layout = descriptor.create_layout(device).unwrap();
    /// # }
    /// ```
    pub fn create_layout(&self, device: &Device) -> Result<BindGroupLayout, BindGroupError> {
        self.validate()?;

        let tracker = ApiCoverageTracker::global();
        tracker.record(ApiCategory::BindGroup, "create_bind_group_layout");

        let wgpu_entries: Vec<wgpu::BindGroupLayoutEntry> =
            self.entries.iter().map(|e| e.to_wgpu()).collect();

        Ok(
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: self.label.as_deref(),
                entries: &wgpu_entries,
            }),
        )
    }
}

impl Default for BindGroupLayoutDescriptor {
    fn default() -> Self {
        Self::new(None)
    }
}

/// A resource that can be bound in a bind group
#[derive(Debug, Clone)]
pub enum BindingResource<'a> {
    /// A buffer binding
    Buffer(BufferBinding<'a>),
    /// A sampler binding
    Sampler(&'a Sampler),
    /// A texture view binding
    TextureView(&'a TextureView),
}

impl<'a> BindingResource<'a> {
    /// Convert to wgpu::BindingResource
    pub fn to_wgpu(&self) -> wgpu::BindingResource<'a> {
        match self {
            BindingResource::Buffer(binding) => {
                wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: binding.buffer,
                    offset: binding.offset,
                    size: binding.size,
                })
            }
            BindingResource::Sampler(sampler) => wgpu::BindingResource::Sampler(sampler),
            BindingResource::TextureView(view) => wgpu::BindingResource::TextureView(view),
        }
    }
}

/// A buffer binding for use in a bind group
#[derive(Debug, Clone, Copy)]
pub struct BufferBinding<'a> {
    /// The buffer to bind
    pub buffer: &'a Buffer,
    /// Offset in bytes from the start of the buffer
    pub offset: u64,
    /// Size of the binding in bytes (None means use the rest of the buffer)
    pub size: Option<NonZeroU64>,
}

impl<'a> BufferBinding<'a> {
    /// Create a new buffer binding that binds the entire buffer
    ///
    /// # Arguments
    /// * `buffer` - The buffer to bind
    ///
    /// # Examples
    /// ```no_run
    /// use wgpu_playground_core::bind_group::BufferBinding;
    /// # fn example(buffer: &wgpu::Buffer) {
    /// let binding = BufferBinding::entire(buffer);
    /// # }
    /// ```
    pub fn entire(buffer: &'a Buffer) -> Self {
        Self {
            buffer,
            offset: 0,
            size: None,
        }
    }

    /// Create a new buffer binding with offset and size
    ///
    /// # Arguments
    /// * `buffer` - The buffer to bind
    /// * `offset` - Offset in bytes from the start of the buffer
    /// * `size` - Size of the binding in bytes (None means use the rest of the buffer)
    ///
    /// # Examples
    /// ```no_run
    /// use wgpu_playground_core::bind_group::BufferBinding;
    /// use std::num::NonZeroU64;
    /// # fn example(buffer: &wgpu::Buffer) {
    /// let binding = BufferBinding::new(buffer, 0, Some(NonZeroU64::new(256).unwrap()));
    /// # }
    /// ```
    pub fn new(buffer: &'a Buffer, offset: u64, size: Option<NonZeroU64>) -> Self {
        Self {
            buffer,
            offset,
            size,
        }
    }
}

/// A single binding entry in a bind group
#[derive(Debug, Clone)]
pub struct BindGroupEntry<'a> {
    /// Binding number (must match the layout)
    pub binding: u32,
    /// The resource to bind
    pub resource: BindingResource<'a>,
}

impl<'a> BindGroupEntry<'a> {
    /// Create a new bind group entry
    ///
    /// # Arguments
    /// * `binding` - The binding number
    /// * `resource` - The resource to bind
    ///
    /// # Examples
    /// ```no_run
    /// use wgpu_playground_core::bind_group::{BindGroupEntry, BufferBinding, BindingResource};
    /// # fn example(buffer: &wgpu::Buffer) {
    /// let entry = BindGroupEntry::new(
    ///     0,
    ///     BindingResource::Buffer(BufferBinding::entire(buffer))
    /// );
    /// # }
    /// ```
    pub fn new(binding: u32, resource: BindingResource<'a>) -> Self {
        Self { binding, resource }
    }

    /// Convert to wgpu::BindGroupEntry
    pub fn to_wgpu(&self) -> wgpu::BindGroupEntry<'a> {
        wgpu::BindGroupEntry {
            binding: self.binding,
            resource: self.resource.to_wgpu(),
        }
    }
}

/// Descriptor for creating a bind group
#[derive(Debug, Clone)]
pub struct BindGroupDescriptor<'a> {
    /// Optional label for debugging
    label: Option<String>,
    /// The bind group layout
    layout: &'a BindGroupLayout,
    /// The entries in the bind group
    entries: Vec<BindGroupEntry<'a>>,
}

impl<'a> BindGroupDescriptor<'a> {
    /// Create a new bind group descriptor
    ///
    /// # Arguments
    /// * `label` - Optional label for debugging
    /// * `layout` - The bind group layout
    ///
    /// # Examples
    /// ```no_run
    /// use wgpu_playground_core::bind_group::BindGroupDescriptor;
    /// # fn example(layout: &wgpu::BindGroupLayout) {
    /// let descriptor = BindGroupDescriptor::new(Some("my_bind_group"), layout);
    /// # }
    /// ```
    pub fn new(label: Option<&str>, layout: &'a BindGroupLayout) -> Self {
        Self {
            label: label.map(String::from),
            layout,
            entries: Vec::new(),
        }
    }

    /// Add a binding entry to the bind group
    ///
    /// # Arguments
    /// * `entry` - The bind group entry to add
    ///
    /// # Returns
    /// Self for method chaining
    ///
    /// # Examples
    /// ```no_run
    /// use wgpu_playground_core::bind_group::{BindGroupDescriptor, BindGroupEntry, BufferBinding, BindingResource};
    /// # fn example(layout: &wgpu::BindGroupLayout, buffer: &wgpu::Buffer) {
    /// let descriptor = BindGroupDescriptor::new(Some("my_bind_group"), layout)
    ///     .with_entry(BindGroupEntry::new(
    ///         0,
    ///         BindingResource::Buffer(BufferBinding::entire(buffer))
    ///     ));
    /// # }
    /// ```
    pub fn with_entry(mut self, entry: BindGroupEntry<'a>) -> Self {
        self.entries.push(entry);
        self
    }

    /// Add multiple binding entries to the bind group
    ///
    /// # Arguments
    ///
    /// * `entries` - A slice of bind group entries to add
    ///
    /// # Returns
    ///
    /// Self for method chaining
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use wgpu_playground_core::bind_group::{BindGroupDescriptor, BindGroupEntry, BufferBinding, BindingResource};
    /// # fn example(layout: &wgpu::BindGroupLayout, uniform_buf: &wgpu::Buffer, storage_buf: &wgpu::Buffer) {
    /// let entries = vec![
    ///     BindGroupEntry::new(0, BindingResource::Buffer(BufferBinding::entire(uniform_buf))),
    ///     BindGroupEntry::new(1, BindingResource::Buffer(BufferBinding::entire(storage_buf))),
    /// ];
    ///
    /// let descriptor = BindGroupDescriptor::new(Some("my_bind_group"), layout)
    ///     .with_entries(&entries);
    /// # }
    /// ```
    pub fn with_entries(mut self, entries: &[BindGroupEntry<'a>]) -> Self
    where
        BindGroupEntry<'a>: Clone,
    {
        self.entries.extend_from_slice(entries);
        self
    }

    /// Get the label
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Get the layout
    pub fn layout(&self) -> &BindGroupLayout {
        self.layout
    }

    /// Get the entries
    pub fn entries(&self) -> &[BindGroupEntry<'a>] {
        &self.entries
    }

    /// Validate the bind group descriptor
    ///
    /// Checks for:
    /// - Duplicate binding numbers
    /// - Empty entry list
    ///
    /// # Returns
    ///
    /// Ok(()) if valid, Err with BindGroupError if invalid
    ///
    /// # Errors
    ///
    /// - `BindGroupError::InvalidBinding` if entry list is empty
    /// - `BindGroupError::DuplicateBinding` if multiple entries use the same binding number
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use wgpu_playground_core::bind_group::{BindGroupDescriptor, BindGroupEntry, BufferBinding, BindingResource};
    /// # fn example(layout: &wgpu::BindGroupLayout, buffer: &wgpu::Buffer) {
    /// let descriptor = BindGroupDescriptor::new(Some("bind_group"), layout)
    ///     .with_entry(BindGroupEntry::new(0, BindingResource::Buffer(BufferBinding::entire(buffer))));
    ///
    /// assert!(descriptor.validate().is_ok());
    /// # }
    /// ```
    pub fn validate(&self) -> Result<(), BindGroupError> {
        if self.entries.is_empty() {
            return Err(BindGroupError::InvalidBinding(
                "Bind group must have at least one entry".to_string(),
            ));
        }

        // Check for duplicate binding numbers
        let mut bindings = std::collections::HashSet::new();
        for entry in &self.entries {
            if !bindings.insert(entry.binding) {
                return Err(BindGroupError::DuplicateBinding(entry.binding));
            }
        }

        Ok(())
    }

    /// Create a wgpu bind group from this descriptor
    ///
    /// This method validates the descriptor and creates the actual bind group.
    ///
    /// # Arguments
    ///
    /// * `device` - The wgpu device to create the bind group on
    ///
    /// # Returns
    ///
    /// A Result containing the BindGroup or a BindGroupError
    ///
    /// # Errors
    ///
    /// Returns a `BindGroupError` if validation fails (see `validate()` for details)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use wgpu_playground_core::bind_group::{BindGroupDescriptor, BindGroupEntry, BufferBinding, BindingResource};
    /// # async fn example(device: &wgpu::Device, layout: &wgpu::BindGroupLayout, buffer: &wgpu::Buffer) {
    /// let descriptor = BindGroupDescriptor::new(Some("my_bind_group"), layout)
    ///     .with_entry(BindGroupEntry::new(
    ///         0,
    ///         BindingResource::Buffer(BufferBinding::entire(buffer))
    ///     ));
    ///
    /// let bind_group = descriptor.create(&device).unwrap();
    /// # }
    /// ```
    pub fn create(&self, device: &Device) -> Result<BindGroup, BindGroupError> {
        self.validate()?;

        let tracker = ApiCoverageTracker::global();
        tracker.record(ApiCategory::BindGroup, "create_bind_group");

        let wgpu_entries: Vec<wgpu::BindGroupEntry> =
            self.entries.iter().map(|e| e.to_wgpu()).collect();

        Ok(device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: self.label.as_deref(),
            layout: self.layout,
            entries: &wgpu_entries,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::{NonZeroU32, NonZeroU64};

    #[test]
    fn test_bind_group_layout_entry_creation() {
        let entry = BindGroupLayoutEntry::new(
            0,
            ShaderStages::VERTEX,
            BindingType::UniformBuffer {
                has_dynamic_offset: false,
                min_binding_size: None,
            },
        );

        assert_eq!(entry.binding, 0);
        assert_eq!(entry.visibility, ShaderStages::VERTEX);
        assert_eq!(entry.count, None);
    }

    #[test]
    fn test_bind_group_layout_entry_with_count() {
        let entry = BindGroupLayoutEntry::new(
            1,
            ShaderStages::FRAGMENT,
            BindingType::Texture {
                sample_type: TextureSampleType::Float { filterable: true },
                view_dimension: TextureViewDimension::D2,
                multisampled: false,
            },
        )
        .with_count(NonZeroU32::new(4).unwrap());

        assert_eq!(entry.binding, 1);
        assert_eq!(entry.count, Some(NonZeroU32::new(4).unwrap()));
    }

    #[test]
    fn test_binding_type_uniform_buffer() {
        let binding = BindingType::UniformBuffer {
            has_dynamic_offset: false,
            min_binding_size: NonZeroU64::new(64),
        };

        match binding {
            BindingType::UniformBuffer {
                has_dynamic_offset,
                min_binding_size,
            } => {
                assert!(!has_dynamic_offset);
                assert_eq!(min_binding_size, NonZeroU64::new(64));
            }
            _ => panic!("Expected uniform buffer"),
        }
    }

    #[test]
    fn test_binding_type_storage_buffer() {
        let binding = BindingType::StorageBuffer {
            has_dynamic_offset: true,
            min_binding_size: None,
            read_only: true,
        };

        match binding {
            BindingType::StorageBuffer {
                has_dynamic_offset,
                min_binding_size,
                read_only,
            } => {
                assert!(has_dynamic_offset);
                assert_eq!(min_binding_size, None);
                assert!(read_only);
            }
            _ => panic!("Expected storage buffer"),
        }
    }

    #[test]
    fn test_binding_type_texture() {
        let binding = BindingType::Texture {
            sample_type: TextureSampleType::Float { filterable: true },
            view_dimension: TextureViewDimension::D2,
            multisampled: false,
        };

        match binding {
            BindingType::Texture {
                sample_type,
                view_dimension,
                multisampled,
            } => {
                assert_eq!(sample_type, TextureSampleType::Float { filterable: true });
                assert_eq!(view_dimension, TextureViewDimension::D2);
                assert!(!multisampled);
            }
            _ => panic!("Expected texture"),
        }
    }

    #[test]
    fn test_binding_type_sampler() {
        let binding = BindingType::Sampler {
            sampler_type: SamplerBindingType::Filtering,
        };

        match binding {
            BindingType::Sampler { sampler_type } => {
                assert_eq!(sampler_type, SamplerBindingType::Filtering);
            }
            _ => panic!("Expected sampler"),
        }
    }

    #[test]
    fn test_binding_type_storage_texture() {
        let binding = BindingType::StorageTexture {
            access: StorageTextureAccess::WriteOnly,
            format: wgpu::TextureFormat::Rgba8Unorm,
            view_dimension: TextureViewDimension::D2,
        };

        match binding {
            BindingType::StorageTexture {
                access,
                format,
                view_dimension,
            } => {
                assert_eq!(access, StorageTextureAccess::WriteOnly);
                assert_eq!(format, wgpu::TextureFormat::Rgba8Unorm);
                assert_eq!(view_dimension, TextureViewDimension::D2);
            }
            _ => panic!("Expected storage texture"),
        }
    }

    #[test]
    fn test_descriptor_creation() {
        let descriptor = BindGroupLayoutDescriptor::new(Some("test_layout"));
        assert_eq!(descriptor.label(), Some("test_layout"));
        assert_eq!(descriptor.entries().len(), 0);
    }

    #[test]
    fn test_descriptor_with_entry() {
        let entry = BindGroupLayoutEntry::new(
            0,
            ShaderStages::VERTEX | ShaderStages::FRAGMENT,
            BindingType::UniformBuffer {
                has_dynamic_offset: false,
                min_binding_size: None,
            },
        );

        let descriptor = BindGroupLayoutDescriptor::new(Some("test")).with_entry(entry.clone());

        assert_eq!(descriptor.entries().len(), 1);
        assert_eq!(descriptor.entries()[0].binding, 0);
    }

    #[test]
    fn test_descriptor_with_multiple_entries() {
        let entries = vec![
            BindGroupLayoutEntry::new(
                0,
                ShaderStages::VERTEX,
                BindingType::UniformBuffer {
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
            ),
            BindGroupLayoutEntry::new(
                1,
                ShaderStages::FRAGMENT,
                BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
            ),
            BindGroupLayoutEntry::new(
                2,
                ShaderStages::FRAGMENT,
                BindingType::Sampler {
                    sampler_type: SamplerBindingType::Filtering,
                },
            ),
        ];

        let descriptor = BindGroupLayoutDescriptor::new(Some("multi_entry")).with_entries(&entries);

        assert_eq!(descriptor.entries().len(), 3);
        assert_eq!(descriptor.entries()[0].binding, 0);
        assert_eq!(descriptor.entries()[1].binding, 1);
        assert_eq!(descriptor.entries()[2].binding, 2);
    }

    #[test]
    fn test_descriptor_validation_empty() {
        let descriptor = BindGroupLayoutDescriptor::new(None);
        let result = descriptor.validate();

        assert!(result.is_err());
        match result {
            Err(BindGroupError::InvalidBinding(msg)) => {
                assert!(msg.contains("at least one entry"));
            }
            _ => panic!("Expected InvalidBinding error"),
        }
    }

    #[test]
    fn test_descriptor_validation_duplicate_binding() {
        let descriptor = BindGroupLayoutDescriptor::new(None)
            .with_entry(BindGroupLayoutEntry::new(
                0,
                ShaderStages::VERTEX,
                BindingType::UniformBuffer {
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
            ))
            .with_entry(BindGroupLayoutEntry::new(
                0, // Duplicate binding number
                ShaderStages::FRAGMENT,
                BindingType::UniformBuffer {
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
            ));

        let result = descriptor.validate();
        assert!(result.is_err());
        match result {
            Err(BindGroupError::DuplicateBinding(binding)) => {
                assert_eq!(binding, 0);
            }
            _ => panic!("Expected DuplicateBinding error"),
        }
    }

    #[test]
    fn test_descriptor_validation_success() {
        let descriptor = BindGroupLayoutDescriptor::new(None)
            .with_entry(BindGroupLayoutEntry::new(
                0,
                ShaderStages::VERTEX,
                BindingType::UniformBuffer {
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
            ))
            .with_entry(BindGroupLayoutEntry::new(
                1,
                ShaderStages::FRAGMENT,
                BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
            ));

        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn test_shader_stages_combination() {
        let entry = BindGroupLayoutEntry::new(
            0,
            ShaderStages::VERTEX | ShaderStages::FRAGMENT | ShaderStages::COMPUTE,
            BindingType::UniformBuffer {
                has_dynamic_offset: false,
                min_binding_size: None,
            },
        );

        assert!(entry.visibility.contains(ShaderStages::VERTEX));
        assert!(entry.visibility.contains(ShaderStages::FRAGMENT));
        assert!(entry.visibility.contains(ShaderStages::COMPUTE));
    }

    #[test]
    fn test_error_display() {
        let err = BindGroupError::InvalidBinding("test error".to_string());
        assert_eq!(err.to_string(), "Invalid binding: test error");

        let err = BindGroupError::DuplicateBinding(5);
        assert_eq!(err.to_string(), "Duplicate binding number: 5");
    }

    #[test]
    fn test_texture_sample_types() {
        let float_filterable = TextureSampleType::Float { filterable: true };
        let float_non_filterable = TextureSampleType::Float { filterable: false };
        let sint = TextureSampleType::Sint;
        let uint = TextureSampleType::Uint;
        let depth = TextureSampleType::Depth;

        assert_eq!(
            float_filterable,
            TextureSampleType::Float { filterable: true }
        );
        assert_eq!(
            float_non_filterable,
            TextureSampleType::Float { filterable: false }
        );
        assert_eq!(sint, TextureSampleType::Sint);
        assert_eq!(uint, TextureSampleType::Uint);
        assert_eq!(depth, TextureSampleType::Depth);
    }

    #[test]
    fn test_texture_view_dimensions() {
        assert_eq!(TextureViewDimension::D1, TextureViewDimension::D1);
        assert_eq!(TextureViewDimension::D2, TextureViewDimension::D2);
        assert_eq!(TextureViewDimension::D2Array, TextureViewDimension::D2Array);
        assert_eq!(TextureViewDimension::Cube, TextureViewDimension::Cube);
        assert_eq!(
            TextureViewDimension::CubeArray,
            TextureViewDimension::CubeArray
        );
        assert_eq!(TextureViewDimension::D3, TextureViewDimension::D3);
    }

    #[test]
    fn test_sampler_binding_types() {
        assert_eq!(SamplerBindingType::Filtering, SamplerBindingType::Filtering);
        assert_eq!(
            SamplerBindingType::NonFiltering,
            SamplerBindingType::NonFiltering
        );
        assert_eq!(
            SamplerBindingType::Comparison,
            SamplerBindingType::Comparison
        );
    }

    #[test]
    fn test_storage_texture_access() {
        assert_eq!(
            StorageTextureAccess::WriteOnly,
            StorageTextureAccess::WriteOnly
        );
        assert_eq!(
            StorageTextureAccess::ReadOnly,
            StorageTextureAccess::ReadOnly
        );
        assert_eq!(
            StorageTextureAccess::ReadWrite,
            StorageTextureAccess::ReadWrite
        );
    }
}
