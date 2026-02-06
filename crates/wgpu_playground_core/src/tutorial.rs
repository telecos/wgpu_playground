//! Tutorial system for guided learning of WebGPU concepts
//!
//! This module provides an interactive tutorial system that guides users through
//! creating rendering examples step-by-step, with explanations of WebGPU concepts.

use serde::{Deserialize, Serialize};

/// Represents a complete tutorial with multiple steps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tutorial {
    pub id: String,
    pub title: String,
    pub description: String,
    pub steps: Vec<TutorialStep>,
    pub difficulty: Difficulty,
}

/// Difficulty level of a tutorial
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

/// A single step in a tutorial
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TutorialStep {
    pub title: String,
    pub description: String,
    pub explanation: String,
    pub highlight_panel: Option<HighlightTarget>,
    pub action: StepAction,
    pub validation: Option<StepValidation>,
}

/// Specifies which UI panel should be highlighted
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HighlightTarget {
    RenderPipeline,
    BufferConfig,
    TextureConfig,
    BindGroup,
    RenderPass,
    DrawCommand,
    ComputePipeline,
    ComputeDispatch,
    Rendering,
}

/// Action required to complete a step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StepAction {
    ReadAndUnderstand,
    NavigateToPanel(HighlightTarget),
    ConfigureShader { expected_contains: Option<String> },
    CreateBuffer { buffer_type: String },
    CreateTexture,
    CreateBindGroup,
    ConfigurePipeline,
    ExecuteRender,
    ExecuteCompute,
}

/// Validation criteria for step completion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepValidation {
    pub check_type: ValidationCheck,
    pub hint: String,
}

/// Types of validation checks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationCheck {
    ManualConfirm,
    PanelVisited(HighlightTarget),
    ShaderCompiled,
    BufferCreated,
    TextureCreated,
    BindGroupCreated,
    RenderExecuted,
}

/// State tracking for tutorial progress
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TutorialState {
    pub current_tutorial: Option<usize>,
    pub current_step: usize,
    pub completed_tutorials: Vec<String>,
    pub visited_panels: Vec<HighlightTarget>,
}

impl TutorialState {
    pub fn mark_panel_visited(&mut self, panel: HighlightTarget) {
        if !self.visited_panels.contains(&panel) {
            self.visited_panels.push(panel);
        }
    }

    pub fn complete_current_tutorial(&mut self, tutorial_id: String) {
        if !self.completed_tutorials.contains(&tutorial_id) {
            self.completed_tutorials.push(tutorial_id);
        }
        self.current_tutorial = None;
        self.current_step = 0;
        self.visited_panels.clear();
    }

    pub fn is_tutorial_completed(&self, tutorial_id: &str) -> bool {
        self.completed_tutorials.contains(&tutorial_id.to_string())
    }
}

/// Get all available tutorials
pub fn get_all_tutorials() -> Vec<Tutorial> {
    vec![
        create_hello_triangle_tutorial(),
        create_adding_textures_tutorial(),
        create_3d_with_depth_tutorial(),
        create_gpu_compute_tutorial(),
    ]
}

/// Tutorial 1: Hello Triangle - Basic rendering setup
fn create_hello_triangle_tutorial() -> Tutorial {
    Tutorial {
        id: "hello_triangle".to_string(),
        title: "Hello Triangle".to_string(),
        description: "Learn the basics of WebGPU rendering by creating your first triangle".to_string(),
        difficulty: Difficulty::Beginner,
        steps: vec![
            TutorialStep {
                title: "Introduction to WebGPU Rendering".to_string(),
                description: "Welcome! In this tutorial, you'll learn the fundamentals of WebGPU rendering by creating a simple colored triangle.".to_string(),
                explanation: "WebGPU is a modern graphics API that provides low-level access to GPU capabilities. To render anything, we need: vertices (points in space), a shader (GPU program), and a render pipeline (configuration for drawing).".to_string(),
                highlight_panel: None,
                action: StepAction::ReadAndUnderstand,
                validation: Some(StepValidation {
                    check_type: ValidationCheck::ManualConfirm,
                    hint: "Click 'Next Step' when you're ready to continue".to_string(),
                }),
            },
            TutorialStep {
                title: "Create Vertex Buffer".to_string(),
                description: "Navigate to the Buffer Config panel to create a buffer for triangle vertices.".to_string(),
                explanation: "A vertex buffer stores the positions and attributes of each vertex. For a triangle, we need 3 vertices. Each vertex has a position (x, y) and a color (r, g, b). The GPU will read from this buffer during rendering.".to_string(),
                highlight_panel: Some(HighlightTarget::BufferConfig),
                action: StepAction::NavigateToPanel(HighlightTarget::BufferConfig),
                validation: Some(StepValidation {
                    check_type: ValidationCheck::PanelVisited(HighlightTarget::BufferConfig),
                    hint: "Click on 'Buffers' in the Resources section".to_string(),
                }),
            },
            TutorialStep {
                title: "Configure the Vertex Shader".to_string(),
                description: "Go to the Render Pipeline panel to set up the vertex shader.".to_string(),
                explanation: "The vertex shader runs once per vertex and transforms vertex positions. It receives vertex data from buffers and outputs positions in clip space (-1 to 1 range). The vertex shader is the first stage of the rendering pipeline.".to_string(),
                highlight_panel: Some(HighlightTarget::RenderPipeline),
                action: StepAction::NavigateToPanel(HighlightTarget::RenderPipeline),
                validation: Some(StepValidation {
                    check_type: ValidationCheck::PanelVisited(HighlightTarget::RenderPipeline),
                    hint: "Navigate to 'Render Pipeline' under Rendering & Graphics".to_string(),
                }),
            },
            TutorialStep {
                title: "Configure the Fragment Shader".to_string(),
                description: "Set up the fragment shader to colorize pixels.".to_string(),
                explanation: "The fragment shader runs once per pixel and determines the final color. It receives interpolated data from the vertex shader (like colors) and outputs the pixel color. This is where you control how your geometry looks.".to_string(),
                highlight_panel: Some(HighlightTarget::RenderPipeline),
                action: StepAction::ConfigureShader {
                    expected_contains: None,
                },
                validation: Some(StepValidation {
                    check_type: ValidationCheck::ShaderCompiled,
                    hint: "Ensure your shader code compiles without errors".to_string(),
                }),
            },
            TutorialStep {
                title: "Configure Render Pass".to_string(),
                description: "Set up the render pass to define how rendering occurs.".to_string(),
                explanation: "A render pass describes a rendering operation: what to draw to, how to clear it, and what to preserve. The load operation clears the screen, and the store operation saves the result. This encapsulates a complete rendering operation.".to_string(),
                highlight_panel: Some(HighlightTarget::RenderPass),
                action: StepAction::NavigateToPanel(HighlightTarget::RenderPass),
                validation: Some(StepValidation {
                    check_type: ValidationCheck::PanelVisited(HighlightTarget::RenderPass),
                    hint: "Go to 'Render Pass' in the Rendering & Graphics section".to_string(),
                }),
            },
            TutorialStep {
                title: "Execute Draw Command".to_string(),
                description: "Issue the draw command to render your triangle!".to_string(),
                explanation: "The draw command tells the GPU to process your vertices through the pipeline. draw(3, 1, 0, 0) means: draw 3 vertices, 1 instance, starting at vertex 0, with instance 0. The GPU will run your vertex shader 3 times and the fragment shader for each pixel covered by the triangle.".to_string(),
                highlight_panel: Some(HighlightTarget::DrawCommand),
                action: StepAction::ExecuteRender,
                validation: Some(StepValidation {
                    check_type: ValidationCheck::RenderExecuted,
                    hint: "Click the draw button to render your triangle".to_string(),
                }),
            },
            TutorialStep {
                title: "Congratulations!".to_string(),
                description: "You've successfully rendered your first triangle with WebGPU!".to_string(),
                explanation: "You've learned the core concepts: vertex buffers store geometry data, shaders are GPU programs (vertex transforms positions, fragment colors pixels), render passes define rendering operations, and draw commands execute the pipeline. This is the foundation for all GPU rendering!".to_string(),
                highlight_panel: Some(HighlightTarget::Rendering),
                action: StepAction::ReadAndUnderstand,
                validation: Some(StepValidation {
                    check_type: ValidationCheck::ManualConfirm,
                    hint: "Review your work and mark the tutorial as complete".to_string(),
                }),
            },
        ],
    }
}

/// Tutorial 2: Adding Textures - Texture pipeline
fn create_adding_textures_tutorial() -> Tutorial {
    Tutorial {
        id: "adding_textures".to_string(),
        title: "Adding Textures".to_string(),
        description: "Learn how to load and apply textures to 3D geometry".to_string(),
        difficulty: Difficulty::Beginner,
        steps: vec![
            TutorialStep {
                title: "Introduction to Textures".to_string(),
                description: "Textures add visual detail to 3D geometry by mapping images onto surfaces.".to_string(),
                explanation: "A texture is an image stored in GPU memory. To use textures, you need: the texture itself (image data), a sampler (how to read the texture), texture coordinates (UV mapping), and bind groups (connecting resources to shaders). Textures are essential for realistic rendering.".to_string(),
                highlight_panel: None,
                action: StepAction::ReadAndUnderstand,
                validation: Some(StepValidation {
                    check_type: ValidationCheck::ManualConfirm,
                    hint: "Click 'Next Step' to start learning about textures".to_string(),
                }),
            },
            TutorialStep {
                title: "Create a Texture".to_string(),
                description: "Load an image as a texture in the Texture Config panel.".to_string(),
                explanation: "Textures can be loaded from image files or created procedurally. The texture format (like RGBA8) determines how color data is stored. Dimensions must be powers of 2 for mipmapping. The GPU can efficiently read texture data during rendering.".to_string(),
                highlight_panel: Some(HighlightTarget::TextureConfig),
                action: StepAction::CreateTexture,
                validation: Some(StepValidation {
                    check_type: ValidationCheck::TextureCreated,
                    hint: "Navigate to Textures and create a texture".to_string(),
                }),
            },
            TutorialStep {
                title: "Configure Texture Sampling".to_string(),
                description: "Set up how the texture will be sampled in shaders.".to_string(),
                explanation: "A sampler controls how textures are read: filtering (linear vs nearest), addressing (repeat, clamp, mirror), and mipmapping. Linear filtering provides smooth results, while nearest gives a pixelated look. The sampler is separate from the texture so one texture can be sampled different ways.".to_string(),
                highlight_panel: Some(HighlightTarget::TextureConfig),
                action: StepAction::ReadAndUnderstand,
                validation: Some(StepValidation {
                    check_type: ValidationCheck::ManualConfirm,
                    hint: "Continue when you understand texture sampling".to_string(),
                }),
            },
            TutorialStep {
                title: "Create Bind Group".to_string(),
                description: "Create a bind group to connect the texture and sampler to your shader.".to_string(),
                explanation: "Bind groups bundle resources (textures, samplers, buffers) for shader access. Each binding in the group has an index matching the shader's binding declarations. Bind groups are the mechanism for passing data to shaders in WebGPU.".to_string(),
                highlight_panel: Some(HighlightTarget::BindGroup),
                action: StepAction::CreateBindGroup,
                validation: Some(StepValidation {
                    check_type: ValidationCheck::BindGroupCreated,
                    hint: "Navigate to Bind Groups and create a bind group with your texture".to_string(),
                }),
            },
            TutorialStep {
                title: "Add Texture Coordinates".to_string(),
                description: "Update vertex buffer to include UV coordinates for texture mapping.".to_string(),
                explanation: "UV coordinates (0-1 range) map vertices to texture locations. (0,0) is typically the top-left corner, (1,1) is bottom-right. The GPU interpolates UVs between vertices, so each pixel knows where to sample the texture. This creates the appearance of the texture being 'wrapped' on the geometry.".to_string(),
                highlight_panel: Some(HighlightTarget::BufferConfig),
                action: StepAction::NavigateToPanel(HighlightTarget::BufferConfig),
                validation: Some(StepValidation {
                    check_type: ValidationCheck::PanelVisited(HighlightTarget::BufferConfig),
                    hint: "Go to Buffers to add UV coordinates to vertices".to_string(),
                }),
            },
            TutorialStep {
                title: "Update Shaders for Texturing".to_string(),
                description: "Modify shaders to sample and display the texture.".to_string(),
                explanation: "In the vertex shader, pass UV coordinates to the fragment shader. In the fragment shader, use textureSample() with the texture, sampler, and UV coordinates to read the texture color. The GPU hardware efficiently performs this sampling operation.".to_string(),
                highlight_panel: Some(HighlightTarget::RenderPipeline),
                action: StepAction::ConfigureShader {
                    expected_contains: Some("textureSample".to_string()),
                },
                validation: Some(StepValidation {
                    check_type: ValidationCheck::ShaderCompiled,
                    hint: "Update your fragment shader to use textureSample()".to_string(),
                }),
            },
            TutorialStep {
                title: "Render Textured Geometry".to_string(),
                description: "Execute the render to see your textured result!".to_string(),
                explanation: "When you draw, the pipeline now includes texture sampling. For each pixel, the GPU interpolates UV coordinates, samples the texture at that location, and uses the color in the fragment shader. This creates richly detailed surfaces from simple geometry.".to_string(),
                highlight_panel: Some(HighlightTarget::Rendering),
                action: StepAction::ExecuteRender,
                validation: Some(StepValidation {
                    check_type: ValidationCheck::RenderExecuted,
                    hint: "Draw to see your textured geometry".to_string(),
                }),
            },
            TutorialStep {
                title: "Texture Tutorial Complete!".to_string(),
                description: "You've mastered texture mapping in WebGPU!".to_string(),
                explanation: "You've learned: textures store image data, samplers control how textures are read, UV coordinates map geometry to textures, and bind groups connect resources to shaders. Textures are fundamental to creating visually compelling graphics!".to_string(),
                highlight_panel: None,
                action: StepAction::ReadAndUnderstand,
                validation: Some(StepValidation {
                    check_type: ValidationCheck::ManualConfirm,
                    hint: "Mark tutorial complete to continue".to_string(),
                }),
            },
        ],
    }
}

/// Tutorial 3: 3D with Depth - Depth testing and matrices
fn create_3d_with_depth_tutorial() -> Tutorial {
    Tutorial {
        id: "3d_with_depth".to_string(),
        title: "3D with Depth".to_string(),
        description: "Learn depth testing and transformation matrices for proper 3D rendering".to_string(),
        difficulty: Difficulty::Intermediate,
        steps: vec![
            TutorialStep {
                title: "Introduction to 3D Rendering".to_string(),
                description: "3D rendering requires depth testing and transformation matrices to display geometry correctly.".to_string(),
                explanation: "In 3D, objects at different distances need proper visibility handling. Depth testing ensures closer objects appear in front. Transformation matrices (model, view, projection) convert 3D world coordinates to 2D screen space. This tutorial covers both concepts.".to_string(),
                highlight_panel: None,
                action: StepAction::ReadAndUnderstand,
                validation: Some(StepValidation {
                    check_type: ValidationCheck::ManualConfirm,
                    hint: "Ready to learn 3D rendering? Click 'Next Step'".to_string(),
                }),
            },
            TutorialStep {
                title: "Understanding Transformation Matrices".to_string(),
                description: "Learn how matrices transform 3D coordinates to screen space.".to_string(),
                explanation: "Three matrices work together: Model matrix positions/rotates/scales objects in the world. View matrix positions the camera. Projection matrix creates perspective (things get smaller with distance). Combined as MVP matrix, they transform vertices from object space to clip space for rendering.".to_string(),
                highlight_panel: None,
                action: StepAction::ReadAndUnderstand,
                validation: Some(StepValidation {
                    check_type: ValidationCheck::ManualConfirm,
                    hint: "Continue when you understand the matrix transformations".to_string(),
                }),
            },
            TutorialStep {
                title: "Create Uniform Buffer for Matrices".to_string(),
                description: "Create a uniform buffer to store transformation matrices.".to_string(),
                explanation: "Uniform buffers hold data that's constant across all vertices in a draw call. Store your MVP matrices here. The vertex shader reads these to transform each vertex position. Uniform buffers are updated from the CPU before each frame.".to_string(),
                highlight_panel: Some(HighlightTarget::BufferConfig),
                action: StepAction::CreateBuffer {
                    buffer_type: "uniform".to_string(),
                },
                validation: Some(StepValidation {
                    check_type: ValidationCheck::BufferCreated,
                    hint: "Create a uniform buffer for transformation matrices".to_string(),
                }),
            },
            TutorialStep {
                title: "Create 3D Vertex Data".to_string(),
                description: "Define vertices for a 3D cube with x, y, z coordinates.".to_string(),
                explanation: "3D vertices have three position components (x, y, z) instead of two. For a cube, you need 8 corner vertices. Using an index buffer, you can reference these vertices to create 12 triangles (2 per face × 6 faces). This is more efficient than duplicating vertices.".to_string(),
                highlight_panel: Some(HighlightTarget::BufferConfig),
                action: StepAction::NavigateToPanel(HighlightTarget::BufferConfig),
                validation: Some(StepValidation {
                    check_type: ValidationCheck::PanelVisited(HighlightTarget::BufferConfig),
                    hint: "Go to Buffers to create 3D vertex data".to_string(),
                }),
            },
            TutorialStep {
                title: "Configure Depth Testing".to_string(),
                description: "Enable depth testing in the render pipeline to handle 3D visibility.".to_string(),
                explanation: "Depth testing compares each fragment's depth with the depth buffer value. If closer, the fragment is drawn and depth updated; otherwise, it's discarded. This ensures objects are drawn in correct depth order, solving the 'which object is in front' problem automatically.".to_string(),
                highlight_panel: Some(HighlightTarget::RenderPipeline),
                action: StepAction::ConfigurePipeline,
                validation: Some(StepValidation {
                    check_type: ValidationCheck::PanelVisited(HighlightTarget::RenderPipeline),
                    hint: "Navigate to Render Pipeline and enable depth testing".to_string(),
                }),
            },
            TutorialStep {
                title: "Update Vertex Shader for 3D".to_string(),
                description: "Modify the vertex shader to apply transformation matrices.".to_string(),
                explanation: "The vertex shader now multiplies the vertex position by the MVP matrix: position_clip = mvp * position_3d. This transforms from object space through world and view space to clip space. The GPU uses clip space positions to determine screen locations and depth values.".to_string(),
                highlight_panel: Some(HighlightTarget::RenderPipeline),
                action: StepAction::ConfigureShader {
                    expected_contains: None,
                },
                validation: Some(StepValidation {
                    check_type: ValidationCheck::ShaderCompiled,
                    hint: "Update vertex shader to multiply position by MVP matrix".to_string(),
                }),
            },
            TutorialStep {
                title: "Render Your 3D Scene".to_string(),
                description: "Execute the render pass with depth testing enabled.".to_string(),
                explanation: "The render pass now uses a depth attachment in addition to the color attachment. The depth buffer is cleared at the start and updated as fragments are processed. Watch as your 3D geometry renders with correct depth ordering, even as objects overlap!".to_string(),
                highlight_panel: Some(HighlightTarget::Rendering),
                action: StepAction::ExecuteRender,
                validation: Some(StepValidation {
                    check_type: ValidationCheck::RenderExecuted,
                    hint: "Execute render to see 3D with proper depth".to_string(),
                }),
            },
            TutorialStep {
                title: "3D Rendering Mastered!".to_string(),
                description: "You now understand the fundamentals of 3D rendering in WebGPU!".to_string(),
                explanation: "You've learned: transformation matrices (MVP) convert 3D to screen space, uniform buffers hold per-draw data, depth testing ensures correct visibility, and 3D coordinates extend to x, y, z. These concepts are essential for any 3D application!".to_string(),
                highlight_panel: None,
                action: StepAction::ReadAndUnderstand,
                validation: Some(StepValidation {
                    check_type: ValidationCheck::ManualConfirm,
                    hint: "Complete the tutorial when ready".to_string(),
                }),
            },
        ],
    }
}

/// Tutorial 4: GPU Compute - Compute shader basics
fn create_gpu_compute_tutorial() -> Tutorial {
    Tutorial {
        id: "gpu_compute".to_string(),
        title: "GPU Compute".to_string(),
        description: "Learn compute shaders for general-purpose GPU computation".to_string(),
        difficulty: Difficulty::Intermediate,
        steps: vec![
            TutorialStep {
                title: "Introduction to Compute Shaders".to_string(),
                description: "Compute shaders enable general-purpose parallel computation on the GPU.".to_string(),
                explanation: "Unlike rendering, compute shaders perform arbitrary calculations. They're organized in workgroups executing in parallel. Common uses: physics simulation, image processing, particle systems, and data transformations. GPUs excel at parallel tasks with thousands of simultaneous operations.".to_string(),
                highlight_panel: None,
                action: StepAction::ReadAndUnderstand,
                validation: Some(StepValidation {
                    check_type: ValidationCheck::ManualConfirm,
                    hint: "Click 'Next Step' to learn compute shaders".to_string(),
                }),
            },
            TutorialStep {
                title: "Create Input Buffer".to_string(),
                description: "Create a storage buffer to hold input data for computation.".to_string(),
                explanation: "Storage buffers allow read/write access from compute shaders. They can be much larger than uniform buffers and support arbitrary data structures. Mark the buffer with STORAGE usage for compute shader access. This is where your input data lives.".to_string(),
                highlight_panel: Some(HighlightTarget::BufferConfig),
                action: StepAction::CreateBuffer {
                    buffer_type: "storage".to_string(),
                },
                validation: Some(StepValidation {
                    check_type: ValidationCheck::BufferCreated,
                    hint: "Create a storage buffer for compute input".to_string(),
                }),
            },
            TutorialStep {
                title: "Create Output Buffer".to_string(),
                description: "Create another storage buffer for computation results.".to_string(),
                explanation: "Compute shaders can write to multiple buffers. The output buffer will receive results. To read results back to CPU, add MAP_READ usage. This enables asynchronous data transfer from GPU to CPU memory after computation completes.".to_string(),
                highlight_panel: Some(HighlightTarget::BufferConfig),
                action: StepAction::CreateBuffer {
                    buffer_type: "storage".to_string(),
                },
                validation: Some(StepValidation {
                    check_type: ValidationCheck::BufferCreated,
                    hint: "Create an output storage buffer".to_string(),
                }),
            },
            TutorialStep {
                title: "Write Compute Shader".to_string(),
                description: "Create the compute shader that will process your data.".to_string(),
                explanation: "Compute shaders use @workgroup_size to define execution parallelism. @builtin(global_invocation_id) identifies each parallel execution. Use @group/@binding to access buffers. Write your algorithm to process data in parallel - each invocation handles a portion of the data independently.".to_string(),
                highlight_panel: Some(HighlightTarget::ComputePipeline),
                action: StepAction::ConfigureShader {
                    expected_contains: Some("@compute".to_string()),
                },
                validation: Some(StepValidation {
                    check_type: ValidationCheck::ShaderCompiled,
                    hint: "Write a compute shader with @compute annotation".to_string(),
                }),
            },
            TutorialStep {
                title: "Create Compute Bind Group".to_string(),
                description: "Bind your buffers to the compute shader.".to_string(),
                explanation: "The bind group connects input and output buffers to shader bindings. The layout must match shader declarations. Index 0 might be input, index 1 output. Bind groups are how compute shaders access external resources.".to_string(),
                highlight_panel: Some(HighlightTarget::BindGroup),
                action: StepAction::CreateBindGroup,
                validation: Some(StepValidation {
                    check_type: ValidationCheck::BindGroupCreated,
                    hint: "Create bind group for compute pipeline".to_string(),
                }),
            },
            TutorialStep {
                title: "Configure Compute Pipeline".to_string(),
                description: "Set up the compute pipeline with your shader.".to_string(),
                explanation: "The compute pipeline combines your shader with its bind group layout. It defines the complete compute operation. Unlike render pipelines, compute pipelines are simpler: just shader and resource bindings, no vertex formats or blending.".to_string(),
                highlight_panel: Some(HighlightTarget::ComputePipeline),
                action: StepAction::ConfigurePipeline,
                validation: Some(StepValidation {
                    check_type: ValidationCheck::PanelVisited(HighlightTarget::ComputePipeline),
                    hint: "Configure the compute pipeline".to_string(),
                }),
            },
            TutorialStep {
                title: "Dispatch Compute Work".to_string(),
                description: "Execute your compute shader on the GPU!".to_string(),
                explanation: "Dispatch specifies workgroup counts (x, y, z). Total invocations = dispatch × workgroup_size. For example, dispatch(100, 1, 1) with workgroup_size(64, 1, 1) runs 6400 invocations. The GPU schedules these efficiently across its compute units.".to_string(),
                highlight_panel: Some(HighlightTarget::ComputeDispatch),
                action: StepAction::ExecuteCompute,
                validation: Some(StepValidation {
                    check_type: ValidationCheck::ManualConfirm,
                    hint: "Dispatch compute work to execute shader".to_string(),
                }),
            },
            TutorialStep {
                title: "GPU Compute Complete!".to_string(),
                description: "You've learned the fundamentals of GPU compute with WebGPU!".to_string(),
                explanation: "You've mastered: compute shaders for parallel processing, storage buffers for large data sets, workgroups for parallel organization, and dispatch for execution control. Compute shaders unlock GPU power for non-graphics tasks!".to_string(),
                highlight_panel: None,
                action: StepAction::ReadAndUnderstand,
                validation: Some(StepValidation {
                    check_type: ValidationCheck::ManualConfirm,
                    hint: "Congratulations on completing the compute tutorial!".to_string(),
                }),
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_tutorials_created() {
        let tutorials = get_all_tutorials();
        assert_eq!(tutorials.len(), 4);
        assert_eq!(tutorials[0].id, "hello_triangle");
        assert_eq!(tutorials[1].id, "adding_textures");
        assert_eq!(tutorials[2].id, "3d_with_depth");
        assert_eq!(tutorials[3].id, "gpu_compute");
    }

    #[test]
    fn test_tutorial_state_tracking() {
        let mut state = TutorialState::default();
        assert_eq!(state.current_step, 0);
        assert!(state.completed_tutorials.is_empty());

        state.mark_panel_visited(HighlightTarget::BufferConfig);
        assert_eq!(state.visited_panels.len(), 1);

        state.complete_current_tutorial("hello_triangle".to_string());
        assert!(state.is_tutorial_completed("hello_triangle"));
        assert_eq!(state.current_step, 0);
    }

    #[test]
    fn test_hello_triangle_tutorial_structure() {
        let tutorial = create_hello_triangle_tutorial();
        assert_eq!(tutorial.difficulty, Difficulty::Beginner);
        assert!(!tutorial.steps.is_empty());
        assert!(tutorial.steps[0].description.contains("Welcome"));
    }

    #[test]
    fn test_panel_visited_deduplication() {
        let mut state = TutorialState::default();
        state.mark_panel_visited(HighlightTarget::BufferConfig);
        state.mark_panel_visited(HighlightTarget::BufferConfig);
        assert_eq!(state.visited_panels.len(), 1);
    }
}
