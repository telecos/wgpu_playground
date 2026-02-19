use std::fmt;
use wgpu::{ComputePipeline, Device, PipelineLayout};

use crate::api_coverage::{ApiCategory, ApiCoverageTracker};
use crate::shader::ShaderModule;

/// Errors that can occur during compute pipeline operations
#[derive(Debug)]
pub enum ComputePipelineError {
    /// Failed to create compute pipeline
    CreationFailed(String),
    /// Invalid configuration
    InvalidConfiguration(String),
    /// Missing required shader module
    MissingShader,
    /// Missing entry point
    MissingEntryPoint,
}

impl fmt::Display for ComputePipelineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComputePipelineError::CreationFailed(msg) => {
                write!(f, "Compute pipeline creation failed: {}", msg)
            }
            ComputePipelineError::InvalidConfiguration(msg) => {
                write!(f, "Invalid pipeline configuration: {}", msg)
            }
            ComputePipelineError::MissingShader => {
                write!(
                    f,
                    "Missing shader module: compute pipeline requires a shader"
                )
            }
            ComputePipelineError::MissingEntryPoint => {
                write!(
                    f,
                    "Missing entry point: compute pipeline requires an entry point"
                )
            }
        }
    }
}

impl std::error::Error for ComputePipelineError {}

/// Descriptor for creating a compute pipeline
///
/// This descriptor follows the builder pattern to configure a compute pipeline.
/// At minimum, a shader module and entry point must be provided.
///
/// # Examples
/// ```no_run
/// use wgpu_playground_core::compute::ComputePipelineDescriptor;
/// use wgpu_playground_core::shader::ShaderModule;
/// # async fn example(device: &wgpu::Device) {
/// let shader = ShaderModule::from_source(
///     "@compute @workgroup_size(1) fn main() {}",
///     Some("compute_shader")
/// ).unwrap();
///
/// let descriptor = ComputePipelineDescriptor::new(Some("my_pipeline"))
///     .with_shader(shader)
///     .with_entry_point("main");
///
/// let pipeline = descriptor.create_pipeline(device).unwrap();
/// # }
/// ```
pub struct ComputePipelineDescriptor {
    /// Optional label for debugging
    label: Option<String>,
    /// The compute shader module
    shader: Option<ShaderModule>,
    /// Entry point function name in the shader
    entry_point: Option<String>,
    /// Optional pipeline layout (if None, will be auto-generated)
    layout: Option<PipelineLayout>,
}

impl ComputePipelineDescriptor {
    /// Create a new compute pipeline descriptor
    ///
    /// # Arguments
    /// * `label` - Optional label for debugging
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::compute::ComputePipelineDescriptor;
    ///
    /// let descriptor = ComputePipelineDescriptor::new(Some("my_compute_pipeline"));
    /// ```
    pub fn new(label: Option<&str>) -> Self {
        Self {
            label: label.map(String::from),
            shader: None,
            entry_point: None,
            layout: None,
        }
    }

    /// Set the shader module for this compute pipeline
    ///
    /// # Arguments
    /// * `shader` - The shader module containing the compute shader
    ///
    /// # Returns
    /// Self for method chaining
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::compute::ComputePipelineDescriptor;
    /// use wgpu_playground_core::shader::ShaderModule;
    ///
    /// let shader = ShaderModule::from_source(
    ///     "@compute @workgroup_size(1) fn main() {}",
    ///     Some("compute")
    /// ).unwrap();
    ///
    /// let descriptor = ComputePipelineDescriptor::new(Some("pipeline"))
    ///     .with_shader(shader);
    /// ```
    pub fn with_shader(mut self, shader: ShaderModule) -> Self {
        self.shader = Some(shader);
        self
    }

    /// Set the entry point function name
    ///
    /// # Arguments
    /// * `entry_point` - The name of the entry point function in the shader
    ///
    /// # Returns
    /// Self for method chaining
    ///
    /// # Examples
    /// ```
    /// use wgpu_playground_core::compute::ComputePipelineDescriptor;
    ///
    /// let descriptor = ComputePipelineDescriptor::new(Some("pipeline"))
    ///     .with_entry_point("main");
    /// ```
    pub fn with_entry_point(mut self, entry_point: &str) -> Self {
        self.entry_point = Some(entry_point.to_string());
        self
    }

    /// Set the pipeline layout
    ///
    /// If not provided, the pipeline layout will be automatically generated from the shader.
    ///
    /// # Arguments
    /// * `layout` - The pipeline layout defining bind groups
    ///
    /// # Returns
    /// Self for method chaining
    ///
    /// # Examples
    /// ```no_run
    /// use wgpu_playground_core::compute::ComputePipelineDescriptor;
    /// # async fn example(device: &wgpu::Device, layout: wgpu::PipelineLayout) {
    /// let descriptor = ComputePipelineDescriptor::new(Some("pipeline"))
    ///     .with_layout(layout);
    /// # }
    /// ```
    pub fn with_layout(mut self, layout: PipelineLayout) -> Self {
        self.layout = Some(layout);
        self
    }

    /// Get the label
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Get the shader module
    pub fn shader(&self) -> Option<&ShaderModule> {
        self.shader.as_ref()
    }

    /// Get the entry point
    pub fn entry_point(&self) -> Option<&str> {
        self.entry_point.as_deref()
    }

    /// Get the pipeline layout
    pub fn layout(&self) -> Option<&PipelineLayout> {
        self.layout.as_ref()
    }

    /// Validate the compute pipeline descriptor
    ///
    /// Checks for:
    /// - Shader module must be provided
    /// - Entry point must be provided
    /// - Entry point name must not be empty
    ///
    /// # Returns
    /// Ok(()) if valid, Err with ComputePipelineError if invalid
    pub fn validate(&self) -> Result<(), ComputePipelineError> {
        if self.shader.is_none() {
            return Err(ComputePipelineError::MissingShader);
        }

        if self.entry_point.is_none() {
            return Err(ComputePipelineError::MissingEntryPoint);
        }

        if let Some(entry_point) = &self.entry_point {
            if entry_point.trim().is_empty() {
                return Err(ComputePipelineError::InvalidConfiguration(
                    "Entry point name cannot be empty".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Create a wgpu compute pipeline from this descriptor
    ///
    /// This method validates the descriptor and creates the actual compute pipeline.
    /// The shader will be compiled during this process.
    ///
    /// # Arguments
    /// * `device` - The wgpu device to create the pipeline on
    ///
    /// # Returns
    /// A Result containing the ComputePipeline or a ComputePipelineError
    ///
    /// # Examples
    /// ```no_run
    /// use wgpu_playground_core::compute::ComputePipelineDescriptor;
    /// use wgpu_playground_core::shader::ShaderModule;
    /// # async fn example(device: &wgpu::Device) {
    /// let shader = ShaderModule::from_source(
    ///     "@compute @workgroup_size(1) fn main() {}",
    ///     Some("compute")
    /// ).unwrap();
    ///
    /// let descriptor = ComputePipelineDescriptor::new(Some("my_pipeline"))
    ///     .with_shader(shader)
    ///     .with_entry_point("main");
    ///
    /// let pipeline = descriptor.create_pipeline(device).unwrap();
    /// # }
    /// ```
    pub fn create_pipeline(
        &self,
        device: &Device,
    ) -> Result<ComputePipeline, ComputePipelineError> {
        self.validate()?;

        let tracker = ApiCoverageTracker::global();

        let shader = self
            .shader
            .as_ref()
            .expect("shader is Some after validate()");
        let _entry_point = self
            .entry_point
            .as_ref()
            .expect("entry_point is Some after validate()");

        tracker.record(ApiCategory::Shader, "create_shader_module");
        let shader_module = shader.create_module(device);

        tracker.record(ApiCategory::ComputePipeline, "create_compute_pipeline");
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: self.label.as_deref(),
            layout: self.layout.as_ref(),
            module: &shader_module,
            entry_point: self.entry_point.as_deref(),
            compilation_options: Default::default(),
            cache: None,
        });

        Ok(pipeline)
    }
}

impl Default for ComputePipelineDescriptor {
    fn default() -> Self {
        Self::new(None)
    }
}

/// Example compute workload types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputeExample {
    /// Simple array doubling
    ArrayDouble,
    /// Vector addition
    VectorAdd,
    /// Matrix multiplication (basic)
    MatrixMultiply,
    /// Image grayscale filter
    Grayscale,
    /// Reduction (sum)
    Reduction,
    /// Prefix sum (scan)
    PrefixSum,
}

impl ComputeExample {
    /// Get display name for the example
    pub fn name(&self) -> &'static str {
        match self {
            ComputeExample::ArrayDouble => "Array Double",
            ComputeExample::VectorAdd => "Vector Addition",
            ComputeExample::MatrixMultiply => "Matrix Multiply",
            ComputeExample::Grayscale => "Grayscale Filter",
            ComputeExample::Reduction => "Reduction (Sum)",
            ComputeExample::PrefixSum => "Prefix Sum (Scan)",
        }
    }

    /// Get description for the example
    pub fn description(&self) -> &'static str {
        match self {
            ComputeExample::ArrayDouble => {
                "Doubles each element in an array using a compute shader"
            }
            ComputeExample::VectorAdd => "Adds two vectors element-wise on the GPU",
            ComputeExample::MatrixMultiply => "Multiplies two matrices using GPU parallelism",
            ComputeExample::Grayscale => "Converts color image data to grayscale",
            ComputeExample::Reduction => "Computes the sum of all elements in an array",
            ComputeExample::PrefixSum => "Computes prefix sum (cumulative sum) of an array",
        }
    }

    /// Get the shader source for this example
    pub fn shader_source(&self) -> &'static str {
        match self {
            ComputeExample::ArrayDouble => {
                r#"@group(0) @binding(0)
var<storage, read_write> data: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if (index < arrayLength(&data)) {
        data[index] = data[index] * 2.0;
    }
}"#
            }
            ComputeExample::VectorAdd => {
                r#"@group(0) @binding(0)
var<storage, read> a: array<f32>;

@group(0) @binding(1)
var<storage, read> b: array<f32>;

@group(0) @binding(2)
var<storage, read_write> result: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if (index < arrayLength(&a)) {
        result[index] = a[index] + b[index];
    }
}"#
            }
            ComputeExample::MatrixMultiply => {
                r#"struct Dimensions {
    M: u32,  // Rows of A and C
    N: u32,  // Cols of B and C
    K: u32,  // Cols of A, Rows of B
}

@group(0) @binding(0)
var<uniform> dims: Dimensions;

@group(0) @binding(1)
var<storage, read> a: array<f32>;

@group(0) @binding(2)
var<storage, read> b: array<f32>;

@group(0) @binding(3)
var<storage, read_write> c: array<f32>;

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let row = global_id.x;
    let col = global_id.y;
    
    if (row >= dims.M || col >= dims.N) {
        return;
    }
    
    var sum: f32 = 0.0;
    for (var k: u32 = 0u; k < dims.K; k = k + 1u) {
        let a_index = row * dims.K + k;
        let b_index = k * dims.N + col;
        sum = sum + a[a_index] * b[b_index];
    }
    
    let c_index = row * dims.N + col;
    c[c_index] = sum;
}"#
            }
            ComputeExample::Grayscale => {
                r#"@group(0) @binding(0)
var<storage, read> input: array<vec4<f32>>;

@group(0) @binding(1)
var<storage, read_write> output: array<vec4<f32>>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if (index < arrayLength(&input)) {
        let pixel = input[index];
        // Luminance formula: 0.299*R + 0.587*G + 0.114*B
        let gray = 0.299 * pixel.r + 0.587 * pixel.g + 0.114 * pixel.b;
        output[index] = vec4<f32>(gray, gray, gray, pixel.a);
    }
}"#
            }
            ComputeExample::Reduction => {
                r#"@group(0) @binding(0)
var<storage, read_write> data: array<f32>;

@group(0) @binding(1)
var<storage, read_write> result: array<f32>;

var<workgroup> shared_data: array<f32, 256>;

@compute @workgroup_size(256)
fn main(
    @builtin(global_invocation_id) global_id: vec3<u32>,
    @builtin(local_invocation_id) local_id: vec3<u32>,
    @builtin(workgroup_id) workgroup_id: vec3<u32>
) {
    let tid = local_id.x;
    let gid = global_id.x;
    
    // Load data into shared memory
    if (gid < arrayLength(&data)) {
        shared_data[tid] = data[gid];
    } else {
        shared_data[tid] = 0.0;
    }
    
    workgroupBarrier();
    
    // Reduction in shared memory
    for (var stride: u32 = 128u; stride > 0u; stride = stride >> 1u) {
        if (tid < stride) {
            shared_data[tid] = shared_data[tid] + shared_data[tid + stride];
        }
        workgroupBarrier();
    }
    
    // Write result for this workgroup
    if (tid == 0u) {
        result[workgroup_id.x] = shared_data[0];
    }
}"#
            }
            ComputeExample::PrefixSum => {
                r#"@group(0) @binding(0)
var<storage, read_write> data: array<f32>;

var<workgroup> temp: array<f32, 256>;

@compute @workgroup_size(128)
fn main(
    @builtin(local_invocation_id) local_id: vec3<u32>,
    @builtin(workgroup_id) workgroup_id: vec3<u32>
) {
    let tid = local_id.x;
    let offset = workgroup_id.x * 256u;
    
    // Load input into shared memory (each thread loads 2 elements)
    let ai = tid;
    let bi = tid + 128u;
    
    if (offset + ai < arrayLength(&data)) {
        temp[ai] = data[offset + ai];
    } else {
        temp[ai] = 0.0;
    }
    if (offset + bi < arrayLength(&data)) {
        temp[bi] = data[offset + bi];
    } else {
        temp[bi] = 0.0;
    }
    
    // Up-sweep (reduce) phase
    var stride: u32 = 1u;
    for (var d: u32 = 128u; d > 0u; d = d >> 1u) {
        workgroupBarrier();
        if (tid < d) {
            let ai2 = stride * (2u * tid + 1u) - 1u;
            let bi2 = stride * (2u * tid + 2u) - 1u;
            temp[bi2] = temp[bi2] + temp[ai2];
        }
        stride = stride * 2u;
    }
    
    // Clear the last element
    if (tid == 0u) {
        temp[255] = 0.0;
    }
    
    // Down-sweep phase
    for (var d: u32 = 1u; d < 256u; d = d * 2u) {
        stride = stride >> 1u;
        workgroupBarrier();
        if (tid < d) {
            let ai2 = stride * (2u * tid + 1u) - 1u;
            let bi2 = stride * (2u * tid + 2u) - 1u;
            let t = temp[ai2];
            temp[ai2] = temp[bi2];
            temp[bi2] = temp[bi2] + t;
        }
    }
    
    workgroupBarrier();
    
    // Write results
    if (offset + ai < arrayLength(&data)) {
        data[offset + ai] = temp[ai];
    }
    if (offset + bi < arrayLength(&data)) {
        data[offset + bi] = temp[bi];
    }
}"#
            }
        }
    }
}

/// Compute & ML Panel with interactive examples
pub struct ComputePanel {
    /// Currently selected example
    selected_example: ComputeExample,
    /// Input data size
    input_size: usize,
    /// Input data (editable)
    input_data: Vec<f32>,
    /// Output data (result)
    output_data: Vec<f32>,
    /// Whether compute has been run
    has_run: bool,
    /// Status message
    status_message: Option<String>,
    /// Error message
    error_message: Option<String>,
    /// Workgroup size X
    workgroup_size_x: u32,
    /// Execution time in milliseconds (approximate)
    execution_time_ms: Option<f64>,
    /// Show shader source
    show_shader_source: bool,
}

impl Default for ComputePanel {
    fn default() -> Self {
        Self::new()
    }
}

impl ComputePanel {
    pub fn new() -> Self {
        let initial_size = 16;
        Self {
            selected_example: ComputeExample::ArrayDouble,
            input_size: initial_size,
            input_data: (1..=initial_size).map(|i| i as f32).collect(),
            output_data: Vec::new(),
            has_run: false,
            status_message: None,
            error_message: None,
            workgroup_size_x: 64,
            execution_time_ms: None,
            show_shader_source: false,
        }
    }

    /// Reset input data to default values
    fn reset_input(&mut self) {
        self.input_data = (1..=self.input_size).map(|i| i as f32).collect();
        self.output_data.clear();
        self.has_run = false;
        self.status_message = None;
        self.error_message = None;
        self.execution_time_ms = None;
    }

    /// Run the compute shader (CPU simulation for now, can be extended to GPU)
    fn run_compute_cpu(&mut self) {
        let start = std::time::Instant::now();

        match self.selected_example {
            ComputeExample::ArrayDouble => {
                self.output_data = self.input_data.iter().map(|x| x * 2.0).collect();
            }
            ComputeExample::VectorAdd => {
                // For vector add, use first half as 'a' and second half as 'b'
                let half = self.input_data.len() / 2;
                if half > 0 {
                    self.output_data = (0..half)
                        .map(|i| {
                            let a = self.input_data.get(i).copied().unwrap_or(0.0);
                            let b = self.input_data.get(i + half).copied().unwrap_or(0.0);
                            a + b
                        })
                        .collect();
                } else {
                    self.output_data = Vec::new();
                }
            }
            ComputeExample::MatrixMultiply => {
                // Simple 4x4 matrix multiply for demo
                let size = 4;
                if self.input_data.len() >= size * size * 2 {
                    self.output_data = vec![0.0; size * size];
                    for i in 0..size {
                        for j in 0..size {
                            let mut sum = 0.0;
                            for k in 0..size {
                                let a = self.input_data[i * size + k];
                                let b = self.input_data[size * size + k * size + j];
                                sum += a * b;
                            }
                            self.output_data[i * size + j] = sum;
                        }
                    }
                }
            }
            ComputeExample::Grayscale => {
                // Treat every 4 floats as RGBA, output grayscale
                self.output_data = self
                    .input_data
                    .chunks(4)
                    .flat_map(|pixel| {
                        let r = pixel.first().copied().unwrap_or(0.0);
                        let g = pixel.get(1).copied().unwrap_or(0.0);
                        let b = pixel.get(2).copied().unwrap_or(0.0);
                        let a = pixel.get(3).copied().unwrap_or(1.0);
                        let gray = 0.299 * r + 0.587 * g + 0.114 * b;
                        vec![gray, gray, gray, a]
                    })
                    .collect();
            }
            ComputeExample::Reduction => {
                let sum: f32 = self.input_data.iter().sum();
                self.output_data = vec![sum];
            }
            ComputeExample::PrefixSum => {
                self.output_data = Vec::with_capacity(self.input_data.len());
                let mut running_sum = 0.0;
                for &x in &self.input_data {
                    self.output_data.push(running_sum);
                    running_sum += x;
                }
            }
        }

        let elapsed = start.elapsed();
        self.execution_time_ms = Some(elapsed.as_secs_f64() * 1000.0);
        self.has_run = true;
        self.status_message = Some(format!(
            "✓ Compute completed ({} elements processed)",
            self.input_data.len()
        ));
        self.error_message = None;
    }

    /// Run compute on GPU
    fn run_compute_gpu(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        use wgpu::util::DeviceExt;

        let tracker = ApiCoverageTracker::global();
        let start = std::time::Instant::now();

        // Create shader module
        let shader_source = self.selected_example.shader_source();

        tracker.record(ApiCategory::Shader, "create_shader_module");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Compute Example Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // Create storage buffer with input data
        tracker.record(ApiCategory::Buffer, "create_buffer");
        let storage_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Storage Buffer"),
            contents: bytemuck::cast_slice(&self.input_data),
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_SRC
                | wgpu::BufferUsages::COPY_DST,
        });

        // Create staging buffer for reading results
        let buffer_size = (self.input_data.len() * std::mem::size_of::<f32>()) as u64;
        tracker.record(ApiCategory::Buffer, "create_buffer");
        let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Staging Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create bind group layout
        tracker.record(ApiCategory::BindGroup, "create_bind_group_layout");
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Compute Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        // Create bind group
        tracker.record(ApiCategory::BindGroup, "create_bind_group");
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Compute Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: storage_buffer.as_entire_binding(),
            }],
        });

        // Create pipeline layout
        tracker.record(ApiCategory::PipelineLayout, "create_pipeline_layout");
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Compute Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            immediate_size: 0,
        });

        // Create compute pipeline
        tracker.record(ApiCategory::ComputePipeline, "create_compute_pipeline");
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: None,
        });

        // Create command encoder and run compute pass
        tracker.record(ApiCategory::CommandEncoder, "create_command_encoder");
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Compute Encoder"),
        });

        {
            tracker.record(ApiCategory::ComputePass, "begin_compute_pass");
            tracker.record(ApiCategory::CommandEncoder, "begin_compute_pass");
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Compute Pass"),
                timestamp_writes: None,
            });

            tracker.record(ApiCategory::ComputePass, "set_pipeline");
            compute_pass.set_pipeline(&pipeline);
            tracker.record(ApiCategory::ComputePass, "set_bind_group");
            compute_pass.set_bind_group(0, &bind_group, &[]);

            let workgroup_count = (self.input_data.len() as u32).div_ceil(self.workgroup_size_x);
            tracker.record(ApiCategory::ComputePass, "dispatch_workgroups");
            compute_pass.dispatch_workgroups(workgroup_count, 1, 1);
        }

        // Copy result to staging buffer
        encoder.copy_buffer_to_buffer(&storage_buffer, 0, &staging_buffer, 0, buffer_size);

        tracker.record(ApiCategory::Queue, "submit");
        queue.submit(Some(encoder.finish()));

        // Read back results
        let buffer_slice = staging_buffer.slice(..);
        let (sender, receiver) = std::sync::mpsc::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            let _ = sender.send(result);
        });

        let _ = device.poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: None,
        });

        if let Ok(Ok(())) = receiver.recv() {
            let data = buffer_slice.get_mapped_range();
            self.output_data = bytemuck::cast_slice(&data).to_vec();
            drop(data);
            staging_buffer.unmap();

            let elapsed = start.elapsed();
            self.execution_time_ms = Some(elapsed.as_secs_f64() * 1000.0);
            self.has_run = true;
            self.status_message = Some(format!(
                "✓ GPU Compute completed ({} elements, {:.2}ms)",
                self.input_data.len(),
                self.execution_time_ms.unwrap_or(0.0)
            ));
            self.error_message = None;
        } else {
            self.error_message = Some("Failed to read compute results from GPU".to_string());
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        self.ui_with_device(ui, None, None);
    }

    pub fn ui_with_device(
        &mut self,
        ui: &mut egui::Ui,
        device: Option<&wgpu::Device>,
        queue: Option<&wgpu::Queue>,
    ) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("🧮 Compute & ML Panel");
            ui.label("Run GPU compute shaders for data-parallel operations and ML workloads.");
            ui.add_space(10.0);

            // Example selection
            ui.group(|ui| {
                ui.heading("📚 Select Example");
                ui.horizontal_wrapped(|ui| {
                    for example in [
                        ComputeExample::ArrayDouble,
                        ComputeExample::VectorAdd,
                        ComputeExample::Reduction,
                        ComputeExample::PrefixSum,
                        ComputeExample::Grayscale,
                        ComputeExample::MatrixMultiply,
                    ] {
                        if ui
                            .selectable_label(self.selected_example == example, example.name())
                            .clicked()
                        {
                            self.selected_example = example;
                            self.reset_input();
                        }
                    }
                });

                ui.add_space(5.0);
                ui.label(egui::RichText::new(self.selected_example.description()).italics());
            });

            ui.add_space(10.0);

            // Shader source toggle
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.show_shader_source, "📝 Show Shader Source");
            });

            if self.show_shader_source {
                ui.add_space(5.0);
                ui.group(|ui| {
                    ui.heading("WGSL Compute Shader");
                    egui::ScrollArea::vertical()
                        .max_height(200.0)
                        .show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::multiline(
                                    &mut self.selected_example.shader_source().to_string(),
                                )
                                .code_editor()
                                .desired_width(f32::INFINITY),
                            );
                        });
                });
            }

            ui.add_space(10.0);

            // Configuration
            ui.group(|ui| {
                ui.heading("⚙️ Configuration");

                egui::Grid::new("compute_config")
                    .num_columns(2)
                    .spacing([10.0, 5.0])
                    .show(ui, |ui| {
                        ui.label("Data Size:");
                        let mut size_str = self.input_size.to_string();
                        if ui.text_edit_singleline(&mut size_str).changed() {
                            if let Ok(new_size) = size_str.parse::<usize>() {
                                if new_size > 0 && new_size <= 1024 {
                                    self.input_size = new_size;
                                    self.reset_input();
                                }
                            }
                        }
                        ui.end_row();

                        ui.label("Workgroup Size:");
                        ui.add(egui::Slider::new(&mut self.workgroup_size_x, 1..=256));
                        ui.end_row();
                    });
            });

            ui.add_space(10.0);

            // Input data
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.heading("📥 Input Data");
                    if ui.button("🔄 Reset").clicked() {
                        self.reset_input();
                    }
                    if ui.button("🎲 Random").clicked() {
                        use std::time::{SystemTime, UNIX_EPOCH};
                        let seed = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_nanos() as u64;
                        self.input_data = (0..self.input_size)
                            .map(|i| {
                                let x = ((seed.wrapping_add(i as u64))
                                    .wrapping_mul(1103515245)
                                    .wrapping_add(12345)
                                    % 1000) as f32
                                    / 100.0;
                                (x * 10.0).round() / 10.0
                            })
                            .collect();
                        self.has_run = false;
                    }
                });

                egui::ScrollArea::horizontal()
                    .max_height(60.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let data_len = self.input_data.len();
                            for (i, val) in self.input_data.iter_mut().enumerate() {
                                let mut val_str = format!("{:.1}", val);
                                let response = ui.add(
                                    egui::TextEdit::singleline(&mut val_str).desired_width(40.0),
                                );
                                if response.changed() {
                                    if let Ok(new_val) = val_str.parse::<f32>() {
                                        *val = new_val;
                                    }
                                }
                                if i < data_len - 1 {
                                    ui.label(",");
                                }
                            }
                        });
                    });
            });

            ui.add_space(10.0);

            // Run buttons
            ui.horizontal(|ui| {
                if ui.button("▶ Run (CPU Simulation)").clicked() {
                    self.run_compute_cpu();
                }

                let gpu_available = device.is_some() && queue.is_some();
                if ui
                    .add_enabled(
                        gpu_available && self.selected_example == ComputeExample::ArrayDouble,
                        egui::Button::new("🚀 Run on GPU"),
                    )
                    .on_hover_text(if gpu_available {
                        if self.selected_example == ComputeExample::ArrayDouble {
                            "Run the compute shader on the GPU"
                        } else {
                            "GPU execution only available for Array Double example currently"
                        }
                    } else {
                        "GPU device not available"
                    })
                    .clicked()
                {
                    if let (Some(dev), Some(q)) = (device, queue) {
                        self.run_compute_gpu(dev, q);
                    }
                }
            });

            ui.add_space(10.0);

            // Status messages
            if let Some(status) = &self.status_message {
                ui.colored_label(egui::Color32::GREEN, status);
            }
            if let Some(error) = &self.error_message {
                ui.colored_label(egui::Color32::RED, format!("❌ {}", error));
            }

            // Output data
            if self.has_run && !self.output_data.is_empty() {
                ui.add_space(10.0);
                ui.group(|ui| {
                    ui.heading("📤 Output Data");

                    if let Some(time_ms) = self.execution_time_ms {
                        ui.label(format!("Execution time: {:.3} ms", time_ms));
                    }

                    egui::ScrollArea::horizontal()
                        .max_height(60.0)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                for (i, val) in self.output_data.iter().enumerate() {
                                    ui.label(format!("{:.2}", val));
                                    if i < self.output_data.len() - 1 {
                                        ui.label(",");
                                    }
                                }
                            });
                        });
                });
            }

            ui.add_space(20.0);

            // Information section
            ui.group(|ui| {
                ui.heading("ℹ️ About Compute Shaders");
                ui.label("Compute shaders run massively parallel workloads on the GPU:");
                ui.add_space(5.0);
                ui.label("• Each workgroup contains multiple invocations");
                ui.label("• Invocations within a workgroup can share memory");
                ui.label("• @workgroup_size(x, y, z) defines invocations per workgroup");
                ui.label("• dispatch_workgroups(x, y, z) launches multiple workgroups");
                ui.add_space(5.0);
                ui.label("Use cases: ML inference, physics simulation, image processing, sorting");
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_pipeline_descriptor_creation() {
        let descriptor = ComputePipelineDescriptor::new(Some("test_compute_pipeline"));
        assert_eq!(descriptor.label(), Some("test_compute_pipeline"));
        assert!(descriptor.shader().is_none());
        assert!(descriptor.entry_point().is_none());
        assert!(descriptor.layout().is_none());
    }

    #[test]
    fn test_compute_pipeline_descriptor_with_entry_point() {
        let descriptor = ComputePipelineDescriptor::new(Some("test")).with_entry_point("main");

        assert_eq!(descriptor.entry_point(), Some("main"));
    }

    #[test]
    fn test_compute_pipeline_descriptor_with_shader() {
        let shader =
            ShaderModule::from_source("@compute @workgroup_size(1) fn main() {}", Some("compute"))
                .unwrap();

        let descriptor = ComputePipelineDescriptor::new(Some("test")).with_shader(shader.clone());

        assert!(descriptor.shader().is_some());
        assert_eq!(
            descriptor.shader().unwrap().source(),
            "@compute @workgroup_size(1) fn main() {}"
        );
    }

    #[test]
    fn test_compute_pipeline_validation_missing_shader() {
        let descriptor = ComputePipelineDescriptor::new(Some("test")).with_entry_point("main");

        let result = descriptor.validate();
        assert!(result.is_err());
        match result {
            Err(ComputePipelineError::MissingShader) => {}
            _ => panic!("Expected MissingShader error"),
        }
    }

    #[test]
    fn test_compute_pipeline_validation_missing_entry_point() {
        let shader =
            ShaderModule::from_source("@compute @workgroup_size(1) fn main() {}", Some("compute"))
                .unwrap();

        let descriptor = ComputePipelineDescriptor::new(Some("test")).with_shader(shader);

        let result = descriptor.validate();
        assert!(result.is_err());
        match result {
            Err(ComputePipelineError::MissingEntryPoint) => {}
            _ => panic!("Expected MissingEntryPoint error"),
        }
    }

    #[test]
    fn test_compute_pipeline_validation_empty_entry_point() {
        let shader =
            ShaderModule::from_source("@compute @workgroup_size(1) fn main() {}", Some("compute"))
                .unwrap();

        let descriptor = ComputePipelineDescriptor::new(Some("test"))
            .with_shader(shader)
            .with_entry_point("   ");

        let result = descriptor.validate();
        assert!(result.is_err());
        match result {
            Err(ComputePipelineError::InvalidConfiguration(msg)) => {
                assert!(msg.contains("Entry point name cannot be empty"));
            }
            _ => panic!("Expected InvalidConfiguration error"),
        }
    }

    #[test]
    fn test_compute_pipeline_validation_success() {
        let shader =
            ShaderModule::from_source("@compute @workgroup_size(1) fn main() {}", Some("compute"))
                .unwrap();

        let descriptor = ComputePipelineDescriptor::new(Some("test"))
            .with_shader(shader)
            .with_entry_point("main");

        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn test_compute_pipeline_descriptor_default() {
        let descriptor = ComputePipelineDescriptor::default();
        assert_eq!(descriptor.label(), None);
        assert!(descriptor.shader().is_none());
        assert!(descriptor.entry_point().is_none());
    }

    #[test]
    fn test_compute_pipeline_error_display() {
        let err = ComputePipelineError::CreationFailed("test error".to_string());
        assert_eq!(
            err.to_string(),
            "Compute pipeline creation failed: test error"
        );

        let err = ComputePipelineError::InvalidConfiguration("config error".to_string());
        assert_eq!(
            err.to_string(),
            "Invalid pipeline configuration: config error"
        );

        let err = ComputePipelineError::MissingShader;
        assert!(err.to_string().contains("Missing shader module"));

        let err = ComputePipelineError::MissingEntryPoint;
        assert!(err.to_string().contains("Missing entry point"));
    }

    #[test]
    fn test_compute_pipeline_builder_pattern() {
        let shader =
            ShaderModule::from_source("@compute @workgroup_size(1) fn compute() {}", Some("test"))
                .unwrap();

        let descriptor = ComputePipelineDescriptor::new(Some("my_pipeline"))
            .with_shader(shader)
            .with_entry_point("compute");

        assert_eq!(descriptor.label(), Some("my_pipeline"));
        assert_eq!(descriptor.entry_point(), Some("compute"));
        assert!(descriptor.shader().is_some());
        assert!(descriptor.validate().is_ok());
    }
}
