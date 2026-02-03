use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use wgpu::{Device, RenderPipeline};

/// Type alias for the pipeline cache map
///
/// Maps pipeline names (String) to compiled render pipeline instances wrapped in Arc.
/// Uses Arc<Mutex<...>> for thread-safe access and sharing of the cache across multiple
/// parts of the application. The inner Arc allows multiple references to the same pipeline.
type PipelineCacheMap = Arc<Mutex<HashMap<String, Arc<RenderPipeline>>>>;

/// Errors that can occur during render pipeline operations
#[derive(Debug)]
pub enum RenderPipelineError {
    /// Failed to create render pipeline
    CreationFailed(String),
    /// Invalid configuration
    InvalidConfiguration(String),
    /// Missing required shader
    MissingShader(String),
    /// Vertex buffer layout validation error
    InvalidVertexBufferLayout(String),
}

impl fmt::Display for RenderPipelineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderPipelineError::CreationFailed(msg) => {
                write!(f, "Render pipeline creation failed: {}", msg)
            }
            RenderPipelineError::InvalidConfiguration(msg) => {
                write!(f, "Invalid pipeline configuration: {}", msg)
            }
            RenderPipelineError::MissingShader(msg) => write!(f, "Missing shader: {}", msg),
            RenderPipelineError::InvalidVertexBufferLayout(msg) => {
                write!(f, "Invalid vertex buffer layout: {}", msg)
            }
        }
    }
}

impl std::error::Error for RenderPipelineError {}

/// Vertex format types for vertex attributes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexFormat {
    /// Single 32-bit unsigned integer
    Uint32,
    /// Single 32-bit signed integer
    Sint32,
    /// Single 32-bit float
    Float32,
    /// Two 32-bit floats (vec2)
    Float32x2,
    /// Three 32-bit floats (vec3)
    Float32x3,
    /// Four 32-bit floats (vec4)
    Float32x4,
    /// Two 32-bit unsigned integers
    Uint32x2,
    /// Three 32-bit unsigned integers
    Uint32x3,
    /// Four 32-bit unsigned integers
    Uint32x4,
    /// Two 32-bit signed integers
    Sint32x2,
    /// Three 32-bit signed integers
    Sint32x3,
    /// Four 32-bit signed integers
    Sint32x4,
}

impl VertexFormat {
    /// Convert to wgpu::VertexFormat
    pub fn to_wgpu(&self) -> wgpu::VertexFormat {
        match self {
            VertexFormat::Uint32 => wgpu::VertexFormat::Uint32,
            VertexFormat::Sint32 => wgpu::VertexFormat::Sint32,
            VertexFormat::Float32 => wgpu::VertexFormat::Float32,
            VertexFormat::Float32x2 => wgpu::VertexFormat::Float32x2,
            VertexFormat::Float32x3 => wgpu::VertexFormat::Float32x3,
            VertexFormat::Float32x4 => wgpu::VertexFormat::Float32x4,
            VertexFormat::Uint32x2 => wgpu::VertexFormat::Uint32x2,
            VertexFormat::Uint32x3 => wgpu::VertexFormat::Uint32x3,
            VertexFormat::Uint32x4 => wgpu::VertexFormat::Uint32x4,
            VertexFormat::Sint32x2 => wgpu::VertexFormat::Sint32x2,
            VertexFormat::Sint32x3 => wgpu::VertexFormat::Sint32x3,
            VertexFormat::Sint32x4 => wgpu::VertexFormat::Sint32x4,
        }
    }

    /// Get the size in bytes of this vertex format
    pub fn size(&self) -> u64 {
        match self {
            VertexFormat::Uint32 | VertexFormat::Sint32 | VertexFormat::Float32 => 4,
            VertexFormat::Float32x2 | VertexFormat::Uint32x2 | VertexFormat::Sint32x2 => 8,
            VertexFormat::Float32x3 | VertexFormat::Uint32x3 | VertexFormat::Sint32x3 => 12,
            VertexFormat::Float32x4 | VertexFormat::Uint32x4 | VertexFormat::Sint32x4 => 16,
        }
    }
}

/// Vertex step mode determines how vertex data is read
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexStepMode {
    /// Each vertex gets the next value
    Vertex,
    /// Each instance gets the next value
    Instance,
}

impl VertexStepMode {
    /// Convert to wgpu::VertexStepMode
    pub fn to_wgpu(&self) -> wgpu::VertexStepMode {
        match self {
            VertexStepMode::Vertex => wgpu::VertexStepMode::Vertex,
            VertexStepMode::Instance => wgpu::VertexStepMode::Instance,
        }
    }
}

/// Description of a single vertex attribute
#[derive(Debug, Clone)]
pub struct VertexAttribute {
    /// Format of the attribute
    pub format: VertexFormat,
    /// Byte offset from the start of the vertex
    pub offset: u64,
    /// Shader location for this attribute
    pub shader_location: u32,
}

impl VertexAttribute {
    /// Create a new vertex attribute
    ///
    /// # Arguments
    /// * `shader_location` - The shader location for this attribute
    /// * `format` - The format of the attribute data
    /// * `offset` - Byte offset from the start of the vertex
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::render_pipeline::{VertexAttribute, VertexFormat};
    ///
    /// // Position attribute at location 0
    /// let position = VertexAttribute::new(0, VertexFormat::Float32x3, 0);
    ///
    /// // Color attribute at location 1, offset by 12 bytes (after position)
    /// let color = VertexAttribute::new(1, VertexFormat::Float32x4, 12);
    /// ```
    pub fn new(shader_location: u32, format: VertexFormat, offset: u64) -> Self {
        Self {
            format,
            offset,
            shader_location,
        }
    }

    /// Convert to wgpu::VertexAttribute
    pub fn to_wgpu(&self) -> wgpu::VertexAttribute {
        wgpu::VertexAttribute {
            format: self.format.to_wgpu(),
            offset: self.offset,
            shader_location: self.shader_location,
        }
    }
}

/// Description of a vertex buffer layout
#[derive(Debug, Clone)]
pub struct VertexBufferLayout {
    /// The stride in bytes between consecutive vertices or instances
    pub array_stride: u64,
    /// How the vertex data is read
    pub step_mode: VertexStepMode,
    /// Descriptions of vertex attributes
    pub attributes: Vec<VertexAttribute>,
}

impl VertexBufferLayout {
    /// Create a new vertex buffer layout
    ///
    /// # Arguments
    /// * `array_stride` - The stride in bytes between consecutive elements
    /// * `step_mode` - How the vertex data is read
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::render_pipeline::{VertexBufferLayout, VertexStepMode};
    ///
    /// let layout = VertexBufferLayout::new(32, VertexStepMode::Vertex);
    /// ```
    pub fn new(array_stride: u64, step_mode: VertexStepMode) -> Self {
        Self {
            array_stride,
            step_mode,
            attributes: Vec::new(),
        }
    }

    /// Add a vertex attribute to this layout
    ///
    /// # Arguments
    /// * `attribute` - The vertex attribute to add
    ///
    /// # Returns
    /// Self for method chaining
    pub fn with_attribute(mut self, attribute: VertexAttribute) -> Self {
        self.attributes.push(attribute);
        self
    }

    /// Add multiple vertex attributes to this layout
    ///
    /// # Arguments
    /// * `attributes` - The vertex attributes to add
    ///
    /// # Returns
    /// Self for method chaining
    pub fn with_attributes(mut self, attributes: &[VertexAttribute]) -> Self {
        self.attributes.extend_from_slice(attributes);
        self
    }

    /// Validate this vertex buffer layout
    pub fn validate(&self) -> Result<(), RenderPipelineError> {
        if self.array_stride == 0 {
            return Err(RenderPipelineError::InvalidVertexBufferLayout(
                "Array stride cannot be zero".to_string(),
            ));
        }

        // Check that offsets don't exceed stride
        for attr in &self.attributes {
            if attr.offset + attr.format.size() > self.array_stride {
                return Err(RenderPipelineError::InvalidVertexBufferLayout(format!(
                    "Attribute at location {} extends beyond array stride",
                    attr.shader_location
                )));
            }
        }

        Ok(())
    }

    /// Convert to wgpu::VertexBufferLayout attributes vector
    pub fn to_wgpu_attributes(&self) -> Vec<wgpu::VertexAttribute> {
        self.attributes.iter().map(|attr| attr.to_wgpu()).collect()
    }
}

/// Primitive topology type
///
/// Defines how vertices are assembled into geometric primitives.
///
/// # Examples
///
/// Standard triangle rendering:
/// ```
/// use wgpu_playground_core::render_pipeline::PrimitiveTopology;
///
/// let topology = PrimitiveTopology::TriangleList;
/// // Vertices [0,1,2,3,4,5] form triangles: (0,1,2) and (3,4,5)
/// ```
///
/// Efficient triangle strip (shares vertices):
/// ```
/// use wgpu_playground_core::render_pipeline::PrimitiveTopology;
///
/// let topology = PrimitiveTopology::TriangleStrip;
/// // Vertices [0,1,2,3,4] form triangles: (0,1,2), (1,2,3), (2,3,4)
/// ```
///
/// Line rendering:
/// ```
/// use wgpu_playground_core::render_pipeline::PrimitiveTopology;
///
/// let topology = PrimitiveTopology::LineList;
/// // Vertices [0,1,2,3] form lines: (0,1) and (2,3)
/// ```
///
/// Point cloud rendering:
/// ```
/// use wgpu_playground_core::render_pipeline::PrimitiveTopology;
///
/// let topology = PrimitiveTopology::PointList;
/// // Each vertex is rendered as a point
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveTopology {
    /// Each group of 3 vertices forms a triangle
    TriangleList,
    /// Each vertex (after the first two) forms a triangle with the previous two
    TriangleStrip,
    /// Each group of 2 vertices forms a line
    LineList,
    /// Each vertex (after the first) forms a line with the previous vertex
    LineStrip,
    /// Each vertex is a point
    PointList,
}

impl PrimitiveTopology {
    /// Convert to wgpu::PrimitiveTopology
    pub fn to_wgpu(&self) -> wgpu::PrimitiveTopology {
        match self {
            PrimitiveTopology::TriangleList => wgpu::PrimitiveTopology::TriangleList,
            PrimitiveTopology::TriangleStrip => wgpu::PrimitiveTopology::TriangleStrip,
            PrimitiveTopology::LineList => wgpu::PrimitiveTopology::LineList,
            PrimitiveTopology::LineStrip => wgpu::PrimitiveTopology::LineStrip,
            PrimitiveTopology::PointList => wgpu::PrimitiveTopology::PointList,
        }
    }
}

/// Face culling mode
///
/// Determines which triangle faces are culled (discarded) during rasterization.
/// Culling improves performance by not rendering invisible back faces.
///
/// # Examples
///
/// No culling (render both sides):
/// ```
/// use wgpu_playground_core::render_pipeline::CullMode;
///
/// let cull_mode = CullMode::None;
/// // Useful for: foliage, glass, two-sided materials
/// ```
///
/// Back-face culling (standard for solid objects):
/// ```
/// use wgpu_playground_core::render_pipeline::CullMode;
///
/// let cull_mode = CullMode::Back;
/// // Improves performance by ~50% for closed meshes
/// // Most common setting for 3D models
/// ```
///
/// Front-face culling (for inverted geometry):
/// ```
/// use wgpu_playground_core::render_pipeline::CullMode;
///
/// let cull_mode = CullMode::Front;
/// // Useful for: skyboxes, inside-out rendering
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CullMode {
    /// No culling
    None,
    /// Cull front faces
    Front,
    /// Cull back faces
    Back,
}

impl CullMode {
    /// Convert to wgpu::Face option
    pub fn to_wgpu(&self) -> Option<wgpu::Face> {
        match self {
            CullMode::None => None,
            CullMode::Front => Some(wgpu::Face::Front),
            CullMode::Back => Some(wgpu::Face::Back),
        }
    }
}

/// Front face winding order
///
/// Defines which winding order (clockwise or counter-clockwise) is considered
/// the front face of a triangle. Used with face culling to determine visibility.
///
/// # Examples
///
/// Counter-clockwise (OpenGL/WebGPU default):
/// ```
/// use wgpu_playground_core::render_pipeline::FrontFace;
///
/// let front_face = FrontFace::Ccw;
/// // Vertices wound counter-clockwise are front-facing
/// // Most common for models exported from 3D software
/// ```
///
/// Clockwise (DirectX convention):
/// ```
/// use wgpu_playground_core::render_pipeline::FrontFace;
///
/// let front_face = FrontFace::Cw;
/// // Vertices wound clockwise are front-facing
/// ```
///
/// # Notes
///
/// This interacts with CullMode:
/// - CullMode::Back + FrontFace::Ccw = culls clockwise triangles
/// - CullMode::Front + FrontFace::Ccw = culls counter-clockwise triangles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrontFace {
    /// Clockwise winding
    Cw,
    /// Counter-clockwise winding
    Ccw,
}

impl FrontFace {
    /// Convert to wgpu::FrontFace
    pub fn to_wgpu(&self) -> wgpu::FrontFace {
        match self {
            FrontFace::Cw => wgpu::FrontFace::Cw,
            FrontFace::Ccw => wgpu::FrontFace::Ccw,
        }
    }
}

/// Primitive state configuration
#[derive(Debug, Clone, Copy)]
pub struct PrimitiveState {
    /// The primitive topology
    pub topology: PrimitiveTopology,
    /// Face culling mode
    pub cull_mode: CullMode,
    /// Front face winding order
    pub front_face: FrontFace,
}

impl PrimitiveState {
    /// Create a new primitive state with default values
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::render_pipeline::PrimitiveState;
    ///
    /// let state = PrimitiveState::new();
    /// ```
    pub fn new() -> Self {
        Self {
            topology: PrimitiveTopology::TriangleList,
            cull_mode: CullMode::None,
            front_face: FrontFace::Ccw,
        }
    }

    /// Set the primitive topology
    pub fn with_topology(mut self, topology: PrimitiveTopology) -> Self {
        self.topology = topology;
        self
    }

    /// Set the cull mode
    pub fn with_cull_mode(mut self, cull_mode: CullMode) -> Self {
        self.cull_mode = cull_mode;
        self
    }

    /// Set the front face winding order
    pub fn with_front_face(mut self, front_face: FrontFace) -> Self {
        self.front_face = front_face;
        self
    }

    /// Convert to wgpu::PrimitiveState
    pub fn to_wgpu(&self) -> wgpu::PrimitiveState {
        wgpu::PrimitiveState {
            topology: self.topology.to_wgpu(),
            strip_index_format: None,
            front_face: self.front_face.to_wgpu(),
            cull_mode: self.cull_mode.to_wgpu(),
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        }
    }
}

impl Default for PrimitiveState {
    fn default() -> Self {
        Self::new()
    }
}

/// Depth/stencil comparison function
///
/// Comparison function used for depth testing and stencil testing.
/// Returns whether the test passes based on comparing incoming value with stored value.
///
/// # Examples
///
/// Standard depth testing (closer objects win):
/// ```
/// use wgpu_playground_core::render_pipeline::CompareFunction;
///
/// let depth_compare = CompareFunction::Less;
/// // Fragment passes if its depth < stored depth
/// // Renders front-to-back correctly
/// ```
///
/// Reverse depth testing (for reverse-Z):
/// ```
/// use wgpu_playground_core::render_pipeline::CompareFunction;
///
/// let depth_compare = CompareFunction::Greater;
/// // Used with reversed depth buffer for better precision
/// ```
///
/// Equality testing:
/// ```
/// use wgpu_playground_core::render_pipeline::CompareFunction;
///
/// let stencil_compare = CompareFunction::Equal;
/// // Pass only if stencil value exactly matches reference
/// // Useful for masking specific regions
/// ```
///
/// Always pass (disable testing):
/// ```
/// use wgpu_playground_core::render_pipeline::CompareFunction;
///
/// let compare = CompareFunction::Always;
/// // All fragments pass (effectively disables the test)
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareFunction {
    /// Never pass
    Never,
    /// Pass if value is less than stored
    Less,
    /// Pass if value equals stored
    Equal,
    /// Pass if value is less than or equal to stored
    LessEqual,
    /// Pass if value is greater than stored
    Greater,
    /// Pass if value does not equal stored
    NotEqual,
    /// Pass if value is greater than or equal to stored
    GreaterEqual,
    /// Always pass
    Always,
}

impl CompareFunction {
    /// Convert to wgpu::CompareFunction
    pub fn to_wgpu(&self) -> wgpu::CompareFunction {
        match self {
            CompareFunction::Never => wgpu::CompareFunction::Never,
            CompareFunction::Less => wgpu::CompareFunction::Less,
            CompareFunction::Equal => wgpu::CompareFunction::Equal,
            CompareFunction::LessEqual => wgpu::CompareFunction::LessEqual,
            CompareFunction::Greater => wgpu::CompareFunction::Greater,
            CompareFunction::NotEqual => wgpu::CompareFunction::NotEqual,
            CompareFunction::GreaterEqual => wgpu::CompareFunction::GreaterEqual,
            CompareFunction::Always => wgpu::CompareFunction::Always,
        }
    }
}

/// Stencil operation
///
/// Defines what happens to the stencil buffer value when a stencil test passes or fails.
///
/// # Examples
///
/// Stencil masking (mark rendered areas):
/// ```
/// use wgpu_playground_core::render_pipeline::StencilOperation;
///
/// let operation = StencilOperation::Replace;
/// // Write reference value to stencil buffer
/// // Common for creating stencil masks
/// ```
///
/// Count overlapping draws (with wrapping):
/// ```
/// use wgpu_playground_core::render_pipeline::StencilOperation;
///
/// let operation = StencilOperation::IncrementWrap;
/// // Increment stencil, wrapping at max value
/// // Useful for counting overlapping geometry
/// ```
///
/// Invert stencil bits:
/// ```
/// use wgpu_playground_core::render_pipeline::StencilOperation;
///
/// let operation = StencilOperation::Invert;
/// // Bitwise NOT operation on stencil value
/// // Useful for toggling stencil regions
/// ```
///
/// Preserve existing stencil:
/// ```
/// use wgpu_playground_core::render_pipeline::StencilOperation;
///
/// let operation = StencilOperation::Keep;
/// // Don't modify stencil buffer
/// // Use when you only want to test, not write
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StencilOperation {
    /// Keep the current stencil value
    Keep,
    /// Set stencil value to zero
    Zero,
    /// Replace stencil value with reference value
    Replace,
    /// Increment and clamp
    IncrementClamp,
    /// Decrement and clamp
    DecrementClamp,
    /// Bitwise invert
    Invert,
    /// Increment and wrap
    IncrementWrap,
    /// Decrement and wrap
    DecrementWrap,
}

impl StencilOperation {
    /// Convert to wgpu::StencilOperation
    pub fn to_wgpu(&self) -> wgpu::StencilOperation {
        match self {
            StencilOperation::Keep => wgpu::StencilOperation::Keep,
            StencilOperation::Zero => wgpu::StencilOperation::Zero,
            StencilOperation::Replace => wgpu::StencilOperation::Replace,
            StencilOperation::IncrementClamp => wgpu::StencilOperation::IncrementClamp,
            StencilOperation::DecrementClamp => wgpu::StencilOperation::DecrementClamp,
            StencilOperation::Invert => wgpu::StencilOperation::Invert,
            StencilOperation::IncrementWrap => wgpu::StencilOperation::IncrementWrap,
            StencilOperation::DecrementWrap => wgpu::StencilOperation::DecrementWrap,
        }
    }
}

/// Stencil face state
#[derive(Debug, Clone, Copy)]
pub struct StencilFaceState {
    /// Comparison function
    pub compare: CompareFunction,
    /// Operation when stencil test fails
    pub fail_op: StencilOperation,
    /// Operation when depth test fails
    pub depth_fail_op: StencilOperation,
    /// Operation when both tests pass
    pub pass_op: StencilOperation,
}

impl StencilFaceState {
    /// Create a new stencil face state with default values
    pub fn new() -> Self {
        Self {
            compare: CompareFunction::Always,
            fail_op: StencilOperation::Keep,
            depth_fail_op: StencilOperation::Keep,
            pass_op: StencilOperation::Keep,
        }
    }

    /// Convert to wgpu::StencilFaceState
    pub fn to_wgpu(&self) -> wgpu::StencilFaceState {
        wgpu::StencilFaceState {
            compare: self.compare.to_wgpu(),
            fail_op: self.fail_op.to_wgpu(),
            depth_fail_op: self.depth_fail_op.to_wgpu(),
            pass_op: self.pass_op.to_wgpu(),
        }
    }
}

impl Default for StencilFaceState {
    fn default() -> Self {
        Self::new()
    }
}

/// Depth/stencil state configuration
///
/// Configures depth testing, depth writing, and stencil operations for a render pipeline.
///
/// # Examples
///
/// Basic depth testing (most common):
/// ```
/// use wgpu_playground_core::render_pipeline::DepthStencilState;
///
/// let depth_state = DepthStencilState::new(wgpu::TextureFormat::Depth24Plus)
///     .with_depth_write_enabled(true);
/// // Enables standard depth testing with Less comparison
/// ```
///
/// Read-only depth testing (for transparency):
/// ```
/// use wgpu_playground_core::render_pipeline::{DepthStencilState, CompareFunction};
///
/// let depth_state = DepthStencilState::new(wgpu::TextureFormat::Depth24Plus)
///     .with_depth_write_enabled(false)
///     .with_depth_compare(CompareFunction::LessEqual);
/// // Test against depth but don't write (for transparent objects)
/// ```
///
/// Stencil masking:
/// ```
/// use wgpu_playground_core::render_pipeline::{DepthStencilState, StencilFaceState, StencilOperation, CompareFunction};
///
/// let stencil = StencilFaceState {
///     compare: CompareFunction::Always,
///     fail_op: StencilOperation::Keep,
///     depth_fail_op: StencilOperation::Keep,
///     pass_op: StencilOperation::Replace,
/// };
///
/// let depth_state = DepthStencilState::new(wgpu::TextureFormat::Depth24PlusStencil8)
///     .with_stencil_front(stencil)
///     .with_stencil_back(stencil);
/// // Write to stencil buffer for masking
/// ```
#[derive(Debug, Clone)]
pub struct DepthStencilState {
    /// Texture format for depth/stencil
    pub format: wgpu::TextureFormat,
    /// Whether depth writes are enabled
    pub depth_write_enabled: bool,
    /// Depth comparison function
    pub depth_compare: CompareFunction,
    /// Stencil state for front faces
    pub stencil_front: StencilFaceState,
    /// Stencil state for back faces
    pub stencil_back: StencilFaceState,
    /// Stencil read mask
    pub stencil_read_mask: u32,
    /// Stencil write mask
    pub stencil_write_mask: u32,
    /// Depth bias constant factor
    pub depth_bias: i32,
    /// Depth bias slope factor
    pub depth_bias_slope_scale: f32,
    /// Depth bias clamp
    pub depth_bias_clamp: f32,
}

impl DepthStencilState {
    /// Create a new depth/stencil state
    ///
    /// # Arguments
    /// * `format` - The texture format for depth/stencil
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::render_pipeline::DepthStencilState;
    ///
    /// let state = DepthStencilState::new(wgpu::TextureFormat::Depth24Plus);
    /// ```
    pub fn new(format: wgpu::TextureFormat) -> Self {
        Self {
            format,
            depth_write_enabled: true,
            depth_compare: CompareFunction::Less,
            stencil_front: StencilFaceState::default(),
            stencil_back: StencilFaceState::default(),
            stencil_read_mask: 0xFFFFFFFF,
            stencil_write_mask: 0xFFFFFFFF,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
        }
    }

    /// Set whether depth writes are enabled
    pub fn with_depth_write_enabled(mut self, enabled: bool) -> Self {
        self.depth_write_enabled = enabled;
        self
    }

    /// Set the depth comparison function
    pub fn with_depth_compare(mut self, compare: CompareFunction) -> Self {
        self.depth_compare = compare;
        self
    }

    /// Set the stencil state for front faces
    pub fn with_stencil_front(mut self, stencil_front: StencilFaceState) -> Self {
        self.stencil_front = stencil_front;
        self
    }

    /// Set the stencil state for back faces
    pub fn with_stencil_back(mut self, stencil_back: StencilFaceState) -> Self {
        self.stencil_back = stencil_back;
        self
    }

    /// Convert to wgpu::DepthStencilState
    pub fn to_wgpu(&self) -> wgpu::DepthStencilState {
        wgpu::DepthStencilState {
            format: self.format,
            depth_write_enabled: self.depth_write_enabled,
            depth_compare: self.depth_compare.to_wgpu(),
            stencil: wgpu::StencilState {
                front: self.stencil_front.to_wgpu(),
                back: self.stencil_back.to_wgpu(),
                read_mask: self.stencil_read_mask,
                write_mask: self.stencil_write_mask,
            },
            bias: wgpu::DepthBiasState {
                constant: self.depth_bias,
                slope_scale: self.depth_bias_slope_scale,
                clamp: self.depth_bias_clamp,
            },
        }
    }
}

/// Multisample state configuration
///
/// Configures multisampling anti-aliasing (MSAA) for smoother edges.
///
/// # Examples
///
/// No multisampling (default):
/// ```
/// use wgpu_playground_core::render_pipeline::MultisampleState;
///
/// let msaa = MultisampleState::new();
/// // 1 sample per pixel (no MSAA)
/// ```
///
/// 4x MSAA (balanced quality/performance):
/// ```
/// use wgpu_playground_core::render_pipeline::MultisampleState;
///
/// let msaa = MultisampleState::new()
///     .with_count(4);
/// // 4 samples per pixel, common for games
/// ```
///
/// 4x MSAA with alpha to coverage (for foliage):
/// ```
/// use wgpu_playground_core::render_pipeline::MultisampleState;
///
/// let msaa = MultisampleState::new()
///     .with_count(4)
///     .with_alpha_to_coverage_enabled(true);
/// // Converts alpha values to sample coverage
/// // Useful for rendering vegetation, fences, etc.
/// ```
///
/// Sample masking (custom sample pattern):
/// ```
/// use wgpu_playground_core::render_pipeline::MultisampleState;
///
/// let msaa = MultisampleState::new()
///     .with_count(4)
///     .with_mask(0b1010); // Use samples 1 and 3 only
/// // Advanced: control which samples are used
/// ```
///
/// # Notes
///
/// - `count` must be 1, 2, 4, or 8 (device-dependent)
/// - Requires a multisampled texture as render target
/// - Higher counts improve quality but reduce performance
#[derive(Debug, Clone, Copy)]
pub struct MultisampleState {
    /// Number of samples per pixel
    pub count: u32,
    /// Sample mask
    pub mask: u64,
    /// Whether alpha to coverage is enabled
    pub alpha_to_coverage_enabled: bool,
}

impl MultisampleState {
    /// Create a new multisample state with default values (no multisampling)
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::render_pipeline::MultisampleState;
    ///
    /// let state = MultisampleState::new();
    /// ```
    pub fn new() -> Self {
        Self {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        }
    }

    /// Set the sample count
    ///
    /// # Arguments
    /// * `count` - Number of samples per pixel (must be 1, 2, 4, or 8)
    pub fn with_count(mut self, count: u32) -> Self {
        self.count = count;
        self
    }

    /// Set whether alpha to coverage is enabled
    pub fn with_alpha_to_coverage(mut self, enabled: bool) -> Self {
        self.alpha_to_coverage_enabled = enabled;
        self
    }

    /// Convert to wgpu::MultisampleState
    pub fn to_wgpu(&self) -> wgpu::MultisampleState {
        wgpu::MultisampleState {
            count: self.count,
            mask: self.mask,
            alpha_to_coverage_enabled: self.alpha_to_coverage_enabled,
        }
    }
}

impl Default for MultisampleState {
    fn default() -> Self {
        Self::new()
    }
}

/// Blend factor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendFactor {
    /// 0
    Zero,
    /// 1
    One,
    /// Source color
    Src,
    /// 1 - source color
    OneMinusSrc,
    /// Source alpha
    SrcAlpha,
    /// 1 - source alpha
    OneMinusSrcAlpha,
    /// Destination color
    Dst,
    /// 1 - destination color
    OneMinusDst,
    /// Destination alpha
    DstAlpha,
    /// 1 - destination alpha
    OneMinusDstAlpha,
    /// Constant color
    Constant,
    /// 1 - constant color
    OneMinusConstant,
    /// Source alpha saturated
    SrcAlphaSaturated,
}

impl BlendFactor {
    /// Convert to wgpu::BlendFactor
    pub fn to_wgpu(&self) -> wgpu::BlendFactor {
        match self {
            BlendFactor::Zero => wgpu::BlendFactor::Zero,
            BlendFactor::One => wgpu::BlendFactor::One,
            BlendFactor::Src => wgpu::BlendFactor::Src,
            BlendFactor::OneMinusSrc => wgpu::BlendFactor::OneMinusSrc,
            BlendFactor::SrcAlpha => wgpu::BlendFactor::SrcAlpha,
            BlendFactor::OneMinusSrcAlpha => wgpu::BlendFactor::OneMinusSrcAlpha,
            BlendFactor::Dst => wgpu::BlendFactor::Dst,
            BlendFactor::OneMinusDst => wgpu::BlendFactor::OneMinusDst,
            BlendFactor::DstAlpha => wgpu::BlendFactor::DstAlpha,
            BlendFactor::OneMinusDstAlpha => wgpu::BlendFactor::OneMinusDstAlpha,
            BlendFactor::Constant => wgpu::BlendFactor::Constant,
            BlendFactor::OneMinusConstant => wgpu::BlendFactor::OneMinusConstant,
            BlendFactor::SrcAlphaSaturated => wgpu::BlendFactor::SrcAlphaSaturated,
        }
    }
}

/// Blend operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendOperation {
    /// Add source and destination
    Add,
    /// Subtract destination from source
    Subtract,
    /// Subtract source from destination
    ReverseSubtract,
    /// Minimum of source and destination
    Min,
    /// Maximum of source and destination
    Max,
}

impl BlendOperation {
    /// Convert to wgpu::BlendOperation
    pub fn to_wgpu(&self) -> wgpu::BlendOperation {
        match self {
            BlendOperation::Add => wgpu::BlendOperation::Add,
            BlendOperation::Subtract => wgpu::BlendOperation::Subtract,
            BlendOperation::ReverseSubtract => wgpu::BlendOperation::ReverseSubtract,
            BlendOperation::Min => wgpu::BlendOperation::Min,
            BlendOperation::Max => wgpu::BlendOperation::Max,
        }
    }
}

/// Blend component configuration
#[derive(Debug, Clone, Copy)]
pub struct BlendComponent {
    /// Source blend factor
    pub src_factor: BlendFactor,
    /// Destination blend factor
    pub dst_factor: BlendFactor,
    /// Blend operation
    pub operation: BlendOperation,
}

impl BlendComponent {
    /// Create a new blend component
    pub fn new(
        src_factor: BlendFactor,
        dst_factor: BlendFactor,
        operation: BlendOperation,
    ) -> Self {
        Self {
            src_factor,
            dst_factor,
            operation,
        }
    }

    /// Create a blend component that replaces destination with source
    pub fn replace() -> Self {
        Self {
            src_factor: BlendFactor::One,
            dst_factor: BlendFactor::Zero,
            operation: BlendOperation::Add,
        }
    }

    /// Convert to wgpu::BlendComponent
    pub fn to_wgpu(&self) -> wgpu::BlendComponent {
        wgpu::BlendComponent {
            src_factor: self.src_factor.to_wgpu(),
            dst_factor: self.dst_factor.to_wgpu(),
            operation: self.operation.to_wgpu(),
        }
    }
}

impl Default for BlendComponent {
    fn default() -> Self {
        Self::replace()
    }
}

/// Blend state configuration
#[derive(Debug, Clone, Copy)]
pub struct BlendState {
    /// Color blend component
    pub color: BlendComponent,
    /// Alpha blend component
    pub alpha: BlendComponent,
}

impl BlendState {
    /// Create a new blend state
    pub fn new(color: BlendComponent, alpha: BlendComponent) -> Self {
        Self { color, alpha }
    }

    /// Create a blend state that replaces destination with source
    pub fn replace() -> Self {
        Self {
            color: BlendComponent::replace(),
            alpha: BlendComponent::replace(),
        }
    }

    /// Create alpha blending blend state (premultiplied alpha)
    pub fn alpha_blending() -> Self {
        Self {
            color: BlendComponent {
                src_factor: BlendFactor::One,
                dst_factor: BlendFactor::OneMinusSrcAlpha,
                operation: BlendOperation::Add,
            },
            alpha: BlendComponent {
                src_factor: BlendFactor::One,
                dst_factor: BlendFactor::OneMinusSrcAlpha,
                operation: BlendOperation::Add,
            },
        }
    }

    /// Convert to wgpu::BlendState
    pub fn to_wgpu(&self) -> wgpu::BlendState {
        wgpu::BlendState {
            color: self.color.to_wgpu(),
            alpha: self.alpha.to_wgpu(),
        }
    }
}

/// Color write mask
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorWrites {
    bits: u32,
}

impl ColorWrites {
    /// Empty color writes
    pub const NONE: Self = Self { bits: 0 };
    /// Write red component
    pub const RED: Self = Self { bits: 1 << 0 };
    /// Write green component
    pub const GREEN: Self = Self { bits: 1 << 1 };
    /// Write blue component
    pub const BLUE: Self = Self { bits: 1 << 2 };
    /// Write alpha component
    pub const ALPHA: Self = Self { bits: 1 << 3 };
    /// Write all components
    pub const ALL: Self = Self {
        bits: Self::RED.bits | Self::GREEN.bits | Self::BLUE.bits | Self::ALPHA.bits,
    };

    /// Create empty color writes
    pub const fn empty() -> Self {
        Self::NONE
    }

    /// Create color writes for all components
    pub const fn all() -> Self {
        Self::ALL
    }

    /// Combine with another color writes
    pub const fn union(self, other: Self) -> Self {
        Self {
            bits: self.bits | other.bits,
        }
    }

    /// Convert to wgpu::ColorWrites
    pub fn to_wgpu(&self) -> wgpu::ColorWrites {
        let mut writes = wgpu::ColorWrites::empty();

        if (self.bits & Self::RED.bits) != 0 {
            writes |= wgpu::ColorWrites::RED;
        }
        if (self.bits & Self::GREEN.bits) != 0 {
            writes |= wgpu::ColorWrites::GREEN;
        }
        if (self.bits & Self::BLUE.bits) != 0 {
            writes |= wgpu::ColorWrites::BLUE;
        }
        if (self.bits & Self::ALPHA.bits) != 0 {
            writes |= wgpu::ColorWrites::ALPHA;
        }

        writes
    }
}

impl std::ops::BitOr for ColorWrites {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl Default for ColorWrites {
    fn default() -> Self {
        Self::ALL
    }
}

/// Color target state
#[derive(Debug, Clone)]
pub struct ColorTargetState {
    /// Texture format of the target
    pub format: wgpu::TextureFormat,
    /// Blend state (None for no blending)
    pub blend: Option<BlendState>,
    /// Color write mask
    pub write_mask: ColorWrites,
}

impl ColorTargetState {
    /// Create a new color target state
    ///
    /// # Arguments
    /// * `format` - The texture format of the target
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::render_pipeline::ColorTargetState;
    ///
    /// let target = ColorTargetState::new(wgpu::TextureFormat::Bgra8UnormSrgb);
    /// ```
    pub fn new(format: wgpu::TextureFormat) -> Self {
        Self {
            format,
            blend: None,
            write_mask: ColorWrites::ALL,
        }
    }

    /// Set the blend state
    pub fn with_blend(mut self, blend: BlendState) -> Self {
        self.blend = Some(blend);
        self
    }

    /// Set the color write mask
    pub fn with_write_mask(mut self, write_mask: ColorWrites) -> Self {
        self.write_mask = write_mask;
        self
    }

    /// Convert to wgpu::ColorTargetState
    pub fn to_wgpu(&self) -> wgpu::ColorTargetState {
        wgpu::ColorTargetState {
            format: self.format,
            blend: self.blend.map(|b| b.to_wgpu()),
            write_mask: self.write_mask.to_wgpu(),
        }
    }
}

/// Render pipeline descriptor
#[derive(Clone)]
pub struct RenderPipelineDescriptor {
    /// Optional label for debugging
    label: Option<String>,
    /// Vertex buffer layouts
    vertex_buffers: Vec<VertexBufferLayout>,
    /// Primitive state
    primitive: PrimitiveState,
    /// Depth/stencil state
    depth_stencil: Option<DepthStencilState>,
    /// Multisample state
    multisample: MultisampleState,
    /// Fragment targets
    fragment_targets: Vec<ColorTargetState>,
    /// Vertex shader entry point
    vertex_entry_point: String,
    /// Fragment shader entry point
    fragment_entry_point: String,
}

impl RenderPipelineDescriptor {
    /// Create a new render pipeline descriptor
    ///
    /// # Arguments
    /// * `label` - Optional label for debugging
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::render_pipeline::RenderPipelineDescriptor;
    ///
    /// let descriptor = RenderPipelineDescriptor::new(Some("my_pipeline"));
    /// ```
    pub fn new(label: Option<&str>) -> Self {
        Self {
            label: label.map(String::from),
            vertex_buffers: Vec::new(),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            fragment_targets: Vec::new(),
            vertex_entry_point: "main".to_string(),
            fragment_entry_point: "main".to_string(),
        }
    }

    /// Add a vertex buffer layout
    ///
    /// # Arguments
    ///
    /// * `layout` - The vertex buffer layout to add
    ///
    /// # Returns
    ///
    /// Self for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use wgpu_playground_core::render_pipeline::{RenderPipelineDescriptor, VertexBufferLayout, VertexStepMode, VertexAttribute, VertexFormat};
    ///
    /// let layout = VertexBufferLayout::new(20, VertexStepMode::Vertex)
    ///     .with_attribute(VertexAttribute::new(0, VertexFormat::Float32x3, 0))
    ///     .with_attribute(VertexAttribute::new(1, VertexFormat::Float32x2, 12));
    ///
    /// let descriptor = RenderPipelineDescriptor::new(Some("pipeline"))
    ///     .with_vertex_buffer(layout);
    /// ```
    pub fn with_vertex_buffer(mut self, layout: VertexBufferLayout) -> Self {
        self.vertex_buffers.push(layout);
        self
    }

    /// Add multiple vertex buffer layouts
    ///
    /// # Arguments
    ///
    /// * `layouts` - Slice of vertex buffer layouts to add
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_vertex_buffers(mut self, layouts: &[VertexBufferLayout]) -> Self {
        self.vertex_buffers.extend_from_slice(layouts);
        self
    }

    /// Set the primitive state
    ///
    /// # Arguments
    ///
    /// * `primitive` - The primitive state configuration
    ///
    /// # Returns
    ///
    /// Self for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use wgpu_playground_core::render_pipeline::{RenderPipelineDescriptor, PrimitiveState, PrimitiveTopology, CullMode};
    ///
    /// let primitive = PrimitiveState::new()
    ///     .with_topology(PrimitiveTopology::TriangleList)
    ///     .with_cull_mode(CullMode::Back);
    ///
    /// let descriptor = RenderPipelineDescriptor::new(Some("pipeline"))
    ///     .with_primitive(primitive);
    /// ```
    pub fn with_primitive(mut self, primitive: PrimitiveState) -> Self {
        self.primitive = primitive;
        self
    }

    /// Set the depth/stencil state
    ///
    /// # Arguments
    ///
    /// * `depth_stencil` - The depth/stencil state configuration
    ///
    /// # Returns
    ///
    /// Self for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use wgpu_playground_core::render_pipeline::{RenderPipelineDescriptor, DepthStencilState};
    ///
    /// let depth = DepthStencilState::new(wgpu::TextureFormat::Depth24Plus)
    ///     .with_depth_write_enabled(true);
    ///
    /// let descriptor = RenderPipelineDescriptor::new(Some("pipeline"))
    ///     .with_depth_stencil(depth);
    /// ```
    pub fn with_depth_stencil(mut self, depth_stencil: DepthStencilState) -> Self {
        self.depth_stencil = Some(depth_stencil);
        self
    }

    /// Set the multisample state
    ///
    /// # Arguments
    ///
    /// * `multisample` - The multisample state configuration
    ///
    /// # Returns
    ///
    /// Self for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use wgpu_playground_core::render_pipeline::{RenderPipelineDescriptor, MultisampleState};
    ///
    /// let msaa = MultisampleState::new().with_count(4);
    ///
    /// let descriptor = RenderPipelineDescriptor::new(Some("pipeline"))
    ///     .with_multisample(msaa);
    /// ```
    pub fn with_multisample(mut self, multisample: MultisampleState) -> Self {
        self.multisample = multisample;
        self
    }

    /// Add a fragment target
    ///
    /// # Arguments
    ///
    /// * `target` - The color target state for a render target
    ///
    /// # Returns
    ///
    /// Self for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use wgpu_playground_core::render_pipeline::{RenderPipelineDescriptor, ColorTargetState, BlendState};
    ///
    /// let target = ColorTargetState::new(wgpu::TextureFormat::Bgra8UnormSrgb)
    ///     .with_blend(BlendState::alpha_blending());
    ///
    /// let descriptor = RenderPipelineDescriptor::new(Some("pipeline"))
    ///     .with_fragment_target(target);
    /// ```
    pub fn with_fragment_target(mut self, target: ColorTargetState) -> Self {
        self.fragment_targets.push(target);
        self
    }

    /// Add multiple fragment targets
    ///
    /// # Arguments
    ///
    /// * `targets` - Slice of color target states for multiple render targets (MRT)
    ///
    /// # Returns
    ///
    /// Self for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use wgpu_playground_core::render_pipeline::{RenderPipelineDescriptor, ColorTargetState};
    ///
    /// let targets = vec![
    ///     ColorTargetState::new(wgpu::TextureFormat::Rgba16Float), // Albedo
    ///     ColorTargetState::new(wgpu::TextureFormat::Rgba16Float), // Normal
    ///     ColorTargetState::new(wgpu::TextureFormat::Rgba16Float), // Position
    /// ];
    ///
    /// let descriptor = RenderPipelineDescriptor::new(Some("gbuffer"))
    ///     .with_fragment_targets(&targets);
    /// // Multi-render target for deferred rendering
    /// ```
    pub fn with_fragment_targets(mut self, targets: &[ColorTargetState]) -> Self {
        self.fragment_targets.extend_from_slice(targets);
        self
    }

    /// Set the vertex shader entry point
    ///
    /// # Arguments
    /// * `entry_point` - The name of the vertex shader entry point function
    ///
    /// # Returns
    /// Self for method chaining
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::render_pipeline::RenderPipelineDescriptor;
    ///
    /// let descriptor = RenderPipelineDescriptor::new(Some("my_pipeline"))
    ///     .with_vertex_entry_point("vs_main");
    /// ```
    pub fn with_vertex_entry_point(mut self, entry_point: &str) -> Self {
        self.vertex_entry_point = entry_point.to_string();
        self
    }

    /// Set the fragment shader entry point
    ///
    /// # Arguments
    /// * `entry_point` - The name of the fragment shader entry point function
    ///
    /// # Returns
    /// Self for method chaining
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::render_pipeline::RenderPipelineDescriptor;
    ///
    /// let descriptor = RenderPipelineDescriptor::new(Some("my_pipeline"))
    ///     .with_fragment_entry_point("fs_main");
    /// ```
    pub fn with_fragment_entry_point(mut self, entry_point: &str) -> Self {
        self.fragment_entry_point = entry_point.to_string();
        self
    }

    /// Get the label
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Get vertex buffer layouts
    pub fn vertex_buffers(&self) -> &[VertexBufferLayout] {
        &self.vertex_buffers
    }

    /// Get primitive state
    pub fn primitive(&self) -> &PrimitiveState {
        &self.primitive
    }

    /// Get depth/stencil state
    pub fn depth_stencil(&self) -> Option<&DepthStencilState> {
        self.depth_stencil.as_ref()
    }

    /// Get multisample state
    pub fn multisample(&self) -> &MultisampleState {
        &self.multisample
    }

    /// Get fragment targets
    pub fn fragment_targets(&self) -> &[ColorTargetState] {
        &self.fragment_targets
    }

    /// Validate the pipeline descriptor
    pub fn validate(&self) -> Result<(), RenderPipelineError> {
        // Validate vertex buffer layouts
        for layout in &self.vertex_buffers {
            layout.validate()?;
        }

        // Validate multisample count
        if self.multisample.count == 0 {
            return Err(RenderPipelineError::InvalidConfiguration(
                "Multisample count cannot be 0".to_string(),
            ));
        }
        if ![1, 2, 4, 8].contains(&self.multisample.count) {
            return Err(RenderPipelineError::InvalidConfiguration(
                "Multisample count must be 1, 2, 4, or 8".to_string(),
            ));
        }

        Ok(())
    }

    /// Create a render pipeline from this descriptor
    ///
    /// # Arguments
    /// * `device` - The wgpu device
    /// * `layout` - The pipeline layout
    /// * `vertex_shader` - The vertex shader module
    /// * `fragment_shader` - Optional fragment shader module
    ///
    /// # Returns
    /// A Result containing the RenderPipeline or an error
    ///
    /// # Examples
    /// ```no_run
    /// use wgpu_playground_core::render_pipeline::{RenderPipelineDescriptor, ColorTargetState};
    /// use wgpu_playground_core::shader::ShaderModule;
    /// # async fn example(device: &wgpu::Device, layout: &wgpu::PipelineLayout) -> Result<(), Box<dyn std::error::Error>> {
    /// let vertex_shader = ShaderModule::from_source(
    ///     "@vertex fn main() -> @builtin(position) vec4<f32> { return vec4<f32>(0.0); }",
    ///     Some("vertex")
    /// )?;
    ///
    /// let fragment_shader = ShaderModule::from_source(
    ///     "@fragment fn main() -> @location(0) vec4<f32> { return vec4<f32>(1.0); }",
    ///     Some("fragment")
    /// )?;
    ///
    /// let descriptor = RenderPipelineDescriptor::new(Some("pipeline"))
    ///     .with_fragment_target(ColorTargetState::new(wgpu::TextureFormat::Bgra8UnormSrgb));
    ///
    /// let pipeline = descriptor.create_pipeline(
    ///     device,
    ///     layout,
    ///     &vertex_shader,
    ///     Some(&fragment_shader)
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_pipeline(
        &self,
        device: &Device,
        layout: &wgpu::PipelineLayout,
        vertex_shader: &crate::shader::ShaderModule,
        fragment_shader: Option<&crate::shader::ShaderModule>,
    ) -> Result<RenderPipeline, RenderPipelineError> {
        log::debug!(
            "Creating render pipeline: label={:?}, vertex_entry={}, fragment_entry={}",
            self.label,
            self.vertex_entry_point,
            self.fragment_entry_point
        );

        // Validate the descriptor
        self.validate()?;

        // Create shader modules
        log::trace!("Creating vertex shader module");
        let vertex_module = vertex_shader.create_module(device);
        let fragment_module = fragment_shader.map(|shader| {
            log::trace!("Creating fragment shader module");
            shader.create_module(device)
        });

        // Convert vertex buffer layouts with their attributes
        let vertex_buffer_attrs: Vec<Vec<wgpu::VertexAttribute>> = self
            .vertex_buffers
            .iter()
            .map(|layout| layout.to_wgpu_attributes())
            .collect();

        let vertex_buffer_layouts: Vec<wgpu::VertexBufferLayout> = vertex_buffer_attrs
            .iter()
            .enumerate()
            .map(|(i, attrs)| wgpu::VertexBufferLayout {
                array_stride: self.vertex_buffers[i].array_stride,
                step_mode: self.vertex_buffers[i].step_mode.to_wgpu(),
                attributes: attrs,
            })
            .collect();

        log::trace!(
            "Configured {} vertex buffer layouts",
            vertex_buffer_layouts.len()
        );

        // Convert fragment targets
        let fragment_targets: Vec<Option<wgpu::ColorTargetState>> = self
            .fragment_targets
            .iter()
            .map(|target| Some(target.to_wgpu()))
            .collect();

        log::trace!("Configured {} fragment targets", fragment_targets.len());

        // Build the pipeline descriptor
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: self.label.as_deref(),
            layout: Some(layout),
            vertex: wgpu::VertexState {
                module: &vertex_module,
                entry_point: Some(&self.vertex_entry_point),
                compilation_options: Default::default(),
                buffers: &vertex_buffer_layouts,
            },
            primitive: self.primitive.to_wgpu(),
            depth_stencil: self.depth_stencil.as_ref().map(|ds| ds.to_wgpu()),
            multisample: self.multisample.to_wgpu(),
            fragment: fragment_module.as_ref().map(|module| wgpu::FragmentState {
                module,
                entry_point: Some(&self.fragment_entry_point),
                compilation_options: Default::default(),
                targets: &fragment_targets,
            }),
            multiview: None,
            cache: None,
        });

        log::info!(
            "Render pipeline created successfully: label={:?}",
            self.label
        );
        Ok(pipeline)
    }
}

/// Pipeline cache for storing compiled pipelines
pub struct PipelineCache {
    cache: PipelineCacheMap,
}

impl PipelineCache {
    /// Create a new pipeline cache
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::render_pipeline::PipelineCache;
    ///
    /// let cache = PipelineCache::new();
    /// ```
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Get a pipeline from the cache
    ///
    /// # Arguments
    /// * `key` - The cache key for the pipeline
    ///
    /// # Returns
    /// An Option containing an Arc to the pipeline if found
    pub fn get(&self, key: &str) -> Option<Arc<RenderPipeline>> {
        let cache = self.cache.lock().unwrap();
        cache.get(key).cloned()
    }

    /// Insert a pipeline into the cache
    ///
    /// # Arguments
    /// * `key` - The cache key for the pipeline
    /// * `pipeline` - The pipeline to cache
    pub fn insert(&self, key: String, pipeline: RenderPipeline) {
        let mut cache = self.cache.lock().unwrap();
        cache.insert(key, Arc::new(pipeline));
    }

    /// Check if a pipeline exists in the cache
    ///
    /// # Arguments
    /// * `key` - The cache key to check
    pub fn contains(&self, key: &str) -> bool {
        let cache = self.cache.lock().unwrap();
        cache.contains_key(key)
    }

    /// Clear all pipelines from the cache
    pub fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }

    /// Get the number of cached pipelines
    pub fn len(&self) -> usize {
        let cache = self.cache.lock().unwrap();
        cache.len()
    }

    /// Check if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for PipelineCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_format_size() {
        assert_eq!(VertexFormat::Float32.size(), 4);
        assert_eq!(VertexFormat::Float32x2.size(), 8);
        assert_eq!(VertexFormat::Float32x3.size(), 12);
        assert_eq!(VertexFormat::Float32x4.size(), 16);
    }

    #[test]
    fn test_vertex_attribute_creation() {
        let attr = VertexAttribute::new(0, VertexFormat::Float32x3, 0);
        assert_eq!(attr.shader_location, 0);
        assert_eq!(attr.format, VertexFormat::Float32x3);
        assert_eq!(attr.offset, 0);
    }

    #[test]
    fn test_vertex_buffer_layout() {
        let layout = VertexBufferLayout::new(32, VertexStepMode::Vertex)
            .with_attribute(VertexAttribute::new(0, VertexFormat::Float32x3, 0))
            .with_attribute(VertexAttribute::new(1, VertexFormat::Float32x4, 12));

        assert_eq!(layout.array_stride, 32);
        assert_eq!(layout.attributes.len(), 2);
        assert!(layout.validate().is_ok());
    }

    #[test]
    fn test_vertex_buffer_layout_validation_error() {
        // Create a layout where an attribute extends beyond the stride
        let layout = VertexBufferLayout::new(8, VertexStepMode::Vertex)
            .with_attribute(VertexAttribute::new(0, VertexFormat::Float32x3, 0));

        // This should fail because Float32x3 is 12 bytes but stride is only 8
        assert!(layout.validate().is_err());
    }

    #[test]
    fn test_primitive_state_defaults() {
        let state = PrimitiveState::default();
        assert_eq!(state.topology, PrimitiveTopology::TriangleList);
        assert_eq!(state.cull_mode, CullMode::None);
        assert_eq!(state.front_face, FrontFace::Ccw);
    }

    #[test]
    fn test_primitive_state_builder() {
        let state = PrimitiveState::new()
            .with_topology(PrimitiveTopology::LineList)
            .with_cull_mode(CullMode::Back)
            .with_front_face(FrontFace::Cw);

        assert_eq!(state.topology, PrimitiveTopology::LineList);
        assert_eq!(state.cull_mode, CullMode::Back);
        assert_eq!(state.front_face, FrontFace::Cw);
    }

    #[test]
    fn test_depth_stencil_state() {
        let state = DepthStencilState::new(wgpu::TextureFormat::Depth24Plus)
            .with_depth_write_enabled(false)
            .with_depth_compare(CompareFunction::Always);

        assert_eq!(state.format, wgpu::TextureFormat::Depth24Plus);
        assert!(!state.depth_write_enabled);
        assert_eq!(state.depth_compare, CompareFunction::Always);
    }

    #[test]
    fn test_multisample_state_defaults() {
        let state = MultisampleState::default();
        assert_eq!(state.count, 1);
        assert_eq!(state.mask, !0);
        assert!(!state.alpha_to_coverage_enabled);
    }

    #[test]
    fn test_multisample_state_builder() {
        let state = MultisampleState::new()
            .with_count(4)
            .with_alpha_to_coverage(true);

        assert_eq!(state.count, 4);
        assert!(state.alpha_to_coverage_enabled);
    }

    #[test]
    fn test_blend_state_replace() {
        let blend = BlendState::replace();
        assert_eq!(blend.color.src_factor, BlendFactor::One);
        assert_eq!(blend.color.dst_factor, BlendFactor::Zero);
        assert_eq!(blend.alpha.src_factor, BlendFactor::One);
        assert_eq!(blend.alpha.dst_factor, BlendFactor::Zero);
    }

    #[test]
    fn test_blend_state_alpha_blending() {
        let blend = BlendState::alpha_blending();
        assert_eq!(blend.color.src_factor, BlendFactor::One);
        assert_eq!(blend.color.dst_factor, BlendFactor::OneMinusSrcAlpha);
    }

    #[test]
    fn test_color_writes() {
        let writes = ColorWrites::RED | ColorWrites::GREEN;
        let all = ColorWrites::ALL;

        assert_ne!(writes.bits, 0);
        assert_eq!(
            all.bits,
            ColorWrites::RED.bits
                | ColorWrites::GREEN.bits
                | ColorWrites::BLUE.bits
                | ColorWrites::ALPHA.bits
        );
    }

    #[test]
    fn test_color_target_state() {
        let target = ColorTargetState::new(wgpu::TextureFormat::Bgra8UnormSrgb)
            .with_blend(BlendState::alpha_blending())
            .with_write_mask(ColorWrites::ALL);

        assert_eq!(target.format, wgpu::TextureFormat::Bgra8UnormSrgb);
        assert!(target.blend.is_some());
    }

    #[test]
    fn test_render_pipeline_descriptor() {
        let descriptor = RenderPipelineDescriptor::new(Some("test_pipeline"))
            .with_fragment_target(ColorTargetState::new(wgpu::TextureFormat::Bgra8UnormSrgb))
            .with_primitive(PrimitiveState::default())
            .with_multisample(MultisampleState::default());

        assert_eq!(descriptor.label(), Some("test_pipeline"));
        assert_eq!(descriptor.fragment_targets().len(), 1);
        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn test_render_pipeline_descriptor_validation_invalid_multisample() {
        let descriptor = RenderPipelineDescriptor::new(Some("test"))
            .with_multisample(MultisampleState::new().with_count(3)); // Invalid count

        assert!(descriptor.validate().is_err());
    }

    #[test]
    fn test_render_pipeline_descriptor_validation_zero_multisample() {
        let descriptor = RenderPipelineDescriptor::new(Some("test"))
            .with_multisample(MultisampleState::new().with_count(0)); // Zero count

        assert!(descriptor.validate().is_err());
    }

    #[test]
    fn test_pipeline_cache() {
        let cache = PipelineCache::new();
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
        assert!(!cache.contains("test"));
    }
}
