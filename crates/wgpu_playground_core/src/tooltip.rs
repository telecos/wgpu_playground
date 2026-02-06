//! Tooltip utilities for WebGPU UI controls with specification links
//!
//! This module provides helpers for creating rich tooltips that explain WebGPU concepts
//! and link to relevant sections of the WebGPU specification.

use egui::{Response, RichText};

/// WebGPU specification base URL
const WEBGPU_SPEC_BASE: &str = "https://www.w3.org/TR/webgpu";

/// Helper trait to add rich tooltips with spec links to egui responses
pub trait TooltipExt {
    /// Add a tooltip with description and optional spec link
    ///
    /// # Arguments
    /// * `description` - Human-readable description of the WebGPU concept
    /// * `spec_anchor` - Optional anchor to the WebGPU spec (e.g., "#gpu-buffer-usage")
    fn webgpu_tooltip(self, description: &str, spec_anchor: Option<&str>) -> Self;
}

impl TooltipExt for Response {
    fn webgpu_tooltip(self, description: &str, spec_anchor: Option<&str>) -> Self {
        self.on_hover_ui(|ui| {
            ui.label(description);
            if let Some(anchor) = spec_anchor {
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new("📄").size(12.0));
                    let url = format!("{}{}", WEBGPU_SPEC_BASE, anchor);
                    if ui
                        .hyperlink_to("WebGPU Spec", &url)
                        .on_hover_text("Click to open specification")
                        .clicked()
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = webbrowser::open(&url);
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            if let Some(window) = web_sys::window() {
                                let _ = window.open_with_url(&url);
                            }
                        }
                    }
                });
            }
        })
    }
}

/// Tooltip information for common WebGPU concepts
pub struct TooltipInfo {
    /// Human-readable description
    pub description: &'static str,
    /// Optional WebGPU specification anchor
    pub spec_anchor: Option<&'static str>,
}

impl TooltipInfo {
    /// Create a new tooltip info
    pub const fn new(description: &'static str, spec_anchor: Option<&'static str>) -> Self {
        Self {
            description,
            spec_anchor,
        }
    }

    /// Apply this tooltip to a response
    pub fn apply(&self, response: Response) -> Response {
        response.webgpu_tooltip(self.description, self.spec_anchor)
    }
}

/// Buffer usage flag tooltips
pub mod buffer_usage {
    use super::TooltipInfo;

    pub const VERTEX: TooltipInfo = TooltipInfo::new(
        "Buffer can be used as a vertex buffer in draw commands. Contains per-vertex attributes like position, normal, UV coordinates, etc.",
        Some("#dom-gpubufferusage-vertex"),
    );

    pub const INDEX: TooltipInfo = TooltipInfo::new(
        "Buffer can be used as an index buffer in indexed draw commands. Contains indices that reference vertices in vertex buffers.",
        Some("#dom-gpubufferusage-index"),
    );

    pub const UNIFORM: TooltipInfo = TooltipInfo::new(
        "Buffer can be used as a uniform buffer. Contains read-only data accessible to shaders, typically for global parameters like transformation matrices.",
        Some("#dom-gpubufferusage-uniform"),
    );

    pub const STORAGE: TooltipInfo = TooltipInfo::new(
        "Buffer can be used as a storage buffer. Allows shaders to read and write arbitrary data, useful for compute operations and large datasets.",
        Some("#dom-gpubufferusage-storage"),
    );

    pub const INDIRECT: TooltipInfo = TooltipInfo::new(
        "Buffer can be used for indirect draw commands. Contains draw parameters that can be dynamically determined on the GPU.",
        Some("#dom-gpubufferusage-indirect"),
    );

    pub const COPY_SRC: TooltipInfo = TooltipInfo::new(
        "Buffer can be used as the source of a copy operation. Allows reading data from this buffer to copy to another buffer or texture.",
        Some("#dom-gpubufferusage-copy_src"),
    );

    pub const COPY_DST: TooltipInfo = TooltipInfo::new(
        "Buffer can be used as the destination of a copy operation. Allows writing data to this buffer from another buffer, texture, or the CPU.",
        Some("#dom-gpubufferusage-copy_dst"),
    );

    pub const MAP_READ: TooltipInfo = TooltipInfo::new(
        "Buffer can be mapped for reading on the CPU. Allows reading GPU data back to the CPU, but cannot be used with MAP_WRITE.",
        Some("#dom-gpubufferusage-map_read"),
    );

    pub const MAP_WRITE: TooltipInfo = TooltipInfo::new(
        "Buffer can be mapped for writing from the CPU. Allows uploading data to the GPU, but cannot be used with MAP_READ.",
        Some("#dom-gpubufferusage-map_write"),
    );

    pub const QUERY_RESOLVE: TooltipInfo = TooltipInfo::new(
        "Buffer can be used to resolve query results. Stores query data like timestamps or occlusion test results.",
        Some("#dom-gpubufferusage-query_resolve"),
    );
}

/// Texture usage flag tooltips
pub mod texture_usage {
    use super::TooltipInfo;

    pub const COPY_SRC: TooltipInfo = TooltipInfo::new(
        "Texture can be used as the source of a copy operation. Allows reading texture data to copy to another texture or buffer.",
        Some("#dom-gputextureusage-copy_src"),
    );

    pub const COPY_DST: TooltipInfo = TooltipInfo::new(
        "Texture can be used as the destination of a copy operation. Allows writing data to this texture from a buffer, another texture, or image uploads.",
        Some("#dom-gputextureusage-copy_dst"),
    );

    pub const TEXTURE_BINDING: TooltipInfo = TooltipInfo::new(
        "Texture can be bound to shaders for sampling. Allows shaders to read and sample texture data using samplers.",
        Some("#dom-gputextureusage-texture_binding"),
    );

    pub const STORAGE_BINDING: TooltipInfo = TooltipInfo::new(
        "Texture can be bound as a storage texture. Allows shaders to read and write texture data directly without sampling, useful for compute operations.",
        Some("#dom-gputextureusage-storage_binding"),
    );

    pub const RENDER_ATTACHMENT: TooltipInfo = TooltipInfo::new(
        "Texture can be used as a render target. Allows rendering operations to write to this texture as a color, depth, or stencil attachment.",
        Some("#dom-gputextureusage-render_attachment"),
    );
}

/// Primitive topology tooltips
pub mod primitive_topology {
    use super::TooltipInfo;

    pub const POINT_LIST: TooltipInfo = TooltipInfo::new(
        "Each vertex represents a point. Useful for particle systems and point clouds.",
        Some("#dom-gpuprimitivetopology-point-list"),
    );

    pub const LINE_LIST: TooltipInfo = TooltipInfo::new(
        "Every two vertices form an independent line segment. Useful for wireframe rendering and debug visualization.",
        Some("#dom-gpuprimitivetopology-line-list"),
    );

    pub const LINE_STRIP: TooltipInfo = TooltipInfo::new(
        "Vertices form a connected sequence of lines. Each vertex connects to the next, creating a polyline.",
        Some("#dom-gpuprimitivetopology-line-strip"),
    );

    pub const TRIANGLE_LIST: TooltipInfo = TooltipInfo::new(
        "Every three vertices form an independent triangle. The most common topology for 3D rendering.",
        Some("#dom-gpuprimitivetopology-triangle-list"),
    );

    pub const TRIANGLE_STRIP: TooltipInfo = TooltipInfo::new(
        "Vertices form a connected strip of triangles. More memory-efficient for certain mesh types like grids.",
        Some("#dom-gpuprimitivetopology-triangle-strip"),
    );
}

/// Cull mode tooltips
pub mod cull_mode {
    use super::TooltipInfo;

    pub const NONE: TooltipInfo = TooltipInfo::new(
        "No face culling. Both front and back faces are rendered. Useful for 2D objects or double-sided geometry.",
        Some("#dom-gpucullmode-none"),
    );

    pub const FRONT: TooltipInfo = TooltipInfo::new(
        "Cull front-facing triangles. Only back faces are rendered. Rarely used in practice.",
        Some("#dom-gpucullmode-front"),
    );

    pub const BACK: TooltipInfo = TooltipInfo::new(
        "Cull back-facing triangles. Only front faces are rendered. The most common culling mode for 3D objects.",
        Some("#dom-gpucullmode-back"),
    );
}

/// Front face winding order tooltips
pub mod front_face {
    use super::TooltipInfo;

    pub const CCW: TooltipInfo = TooltipInfo::new(
        "Counter-clockwise winding order defines front faces. Vertices ordered counter-clockwise (when viewed from the camera) are front-facing. This is the most common convention.",
        Some("#dom-gpufrontface-ccw"),
    );

    pub const CW: TooltipInfo = TooltipInfo::new(
        "Clockwise winding order defines front faces. Vertices ordered clockwise (when viewed from the camera) are front-facing.",
        Some("#dom-gpufrontface-cw"),
    );
}

/// Compare function tooltips (for depth/stencil testing)
pub mod compare_function {
    use super::TooltipInfo;

    pub const NEVER: TooltipInfo = TooltipInfo::new(
        "Comparison always fails. No fragments pass the test.",
        Some("#dom-gpucomparefunction-never"),
    );

    pub const LESS: TooltipInfo = TooltipInfo::new(
        "Comparison passes if the new value is less than the stored value. Most common for depth testing.",
        Some("#dom-gpucomparefunction-less"),
    );

    pub const EQUAL: TooltipInfo = TooltipInfo::new(
        "Comparison passes if the new value equals the stored value.",
        Some("#dom-gpucomparefunction-equal"),
    );

    pub const LESS_EQUAL: TooltipInfo = TooltipInfo::new(
        "Comparison passes if the new value is less than or equal to the stored value.",
        Some("#dom-gpucomparefunction-less-equal"),
    );

    pub const GREATER: TooltipInfo = TooltipInfo::new(
        "Comparison passes if the new value is greater than the stored value. Used for reverse-Z depth testing.",
        Some("#dom-gpucomparefunction-greater"),
    );

    pub const NOT_EQUAL: TooltipInfo = TooltipInfo::new(
        "Comparison passes if the new value does not equal the stored value.",
        Some("#dom-gpucomparefunction-not-equal"),
    );

    pub const GREATER_EQUAL: TooltipInfo = TooltipInfo::new(
        "Comparison passes if the new value is greater than or equal to the stored value.",
        Some("#dom-gpucomparefunction-greater-equal"),
    );

    pub const ALWAYS: TooltipInfo = TooltipInfo::new(
        "Comparison always passes. All fragments pass the test.",
        Some("#dom-gpucomparefunction-always"),
    );
}

/// Blend factor tooltips
pub mod blend_factor {
    use super::TooltipInfo;

    pub const ZERO: TooltipInfo = TooltipInfo::new(
        "Blend factor is 0. The source or destination color is multiplied by zero.",
        Some("#dom-gpublendfactor-zero"),
    );

    pub const ONE: TooltipInfo = TooltipInfo::new(
        "Blend factor is 1. The source or destination color is used as-is.",
        Some("#dom-gpublendfactor-one"),
    );

    pub const SRC: TooltipInfo = TooltipInfo::new(
        "Blend factor is the source color.",
        Some("#dom-gpublendfactor-src"),
    );

    pub const ONE_MINUS_SRC: TooltipInfo = TooltipInfo::new(
        "Blend factor is 1 minus the source color.",
        Some("#dom-gpublendfactor-one-minus-src"),
    );

    pub const SRC_ALPHA: TooltipInfo = TooltipInfo::new(
        "Blend factor is the source alpha.",
        Some("#dom-gpublendfactor-src-alpha"),
    );

    pub const ONE_MINUS_SRC_ALPHA: TooltipInfo = TooltipInfo::new(
        "Blend factor is 1 minus the source alpha. Common for alpha blending.",
        Some("#dom-gpublendfactor-one-minus-src-alpha"),
    );

    pub const DST: TooltipInfo = TooltipInfo::new(
        "Blend factor is the destination color.",
        Some("#dom-gpublendfactor-dst"),
    );

    pub const ONE_MINUS_DST: TooltipInfo = TooltipInfo::new(
        "Blend factor is 1 minus the destination color.",
        Some("#dom-gpublendfactor-one-minus-dst"),
    );

    pub const DST_ALPHA: TooltipInfo = TooltipInfo::new(
        "Blend factor is the destination alpha.",
        Some("#dom-gpublendfactor-dst-alpha"),
    );

    pub const ONE_MINUS_DST_ALPHA: TooltipInfo = TooltipInfo::new(
        "Blend factor is 1 minus the destination alpha.",
        Some("#dom-gpublendfactor-one-minus-dst-alpha"),
    );
}

/// Blend operation tooltips
pub mod blend_operation {
    use super::TooltipInfo;

    pub const ADD: TooltipInfo = TooltipInfo::new(
        "Add source and destination colors. Most common blend operation.",
        Some("#dom-gpublendoperation-add"),
    );

    pub const SUBTRACT: TooltipInfo = TooltipInfo::new(
        "Subtract destination from source color.",
        Some("#dom-gpublendoperation-subtract"),
    );

    pub const REVERSE_SUBTRACT: TooltipInfo = TooltipInfo::new(
        "Subtract source from destination color.",
        Some("#dom-gpublendoperation-reverse-subtract"),
    );

    pub const MIN: TooltipInfo = TooltipInfo::new(
        "Use the minimum of source and destination colors.",
        Some("#dom-gpublendoperation-min"),
    );

    pub const MAX: TooltipInfo = TooltipInfo::new(
        "Use the maximum of source and destination colors.",
        Some("#dom-gpublendoperation-max"),
    );
}

/// Sampler address mode tooltips
pub mod address_mode {
    use super::TooltipInfo;

    pub const CLAMP_TO_EDGE: TooltipInfo = TooltipInfo::new(
        "Texture coordinates are clamped to [0, 1]. Coordinates outside this range use the edge color.",
        Some("#dom-gpuaddressmode-clamp-to-edge"),
    );

    pub const REPEAT: TooltipInfo = TooltipInfo::new(
        "Texture coordinates wrap around. Creates a tiling effect where the texture repeats.",
        Some("#dom-gpuaddressmode-repeat"),
    );

    pub const MIRROR_REPEAT: TooltipInfo = TooltipInfo::new(
        "Texture coordinates wrap around with mirroring. The texture alternates between normal and mirrored on each repetition.",
        Some("#dom-gpuaddressmode-mirror-repeat"),
    );

    pub const CLAMP_TO_BORDER: TooltipInfo = TooltipInfo::new(
        "Texture coordinates outside [0, 1] use the border color.",
        Some("#dom-gpuaddressmode-clamp-to-border"),
    );
}

/// Sampler filter mode tooltips
pub mod filter_mode {
    use super::TooltipInfo;

    pub const NEAREST: TooltipInfo = TooltipInfo::new(
        "Nearest neighbor filtering. Fast but can appear pixelated. Uses the color of the closest texel.",
        Some("#dom-gpufiltermode-nearest"),
    );

    pub const LINEAR: TooltipInfo = TooltipInfo::new(
        "Linear (bilinear) filtering. Smoother appearance. Interpolates between nearby texels.",
        Some("#dom-gpufiltermode-linear"),
    );
}

/// Sampler property tooltips
pub mod sampler {
    use super::TooltipInfo;

    pub const LOD_MIN_CLAMP: TooltipInfo = TooltipInfo::new(
        "Minimum level of detail (LOD) to use when sampling. Lower values use higher resolution mipmap levels. Typically 0.0 to use the highest resolution.",
        Some("#dom-gpusamplerdescriptor-lodminclamp"),
    );

    pub const LOD_MAX_CLAMP: TooltipInfo = TooltipInfo::new(
        "Maximum level of detail (LOD) to use when sampling. Higher values allow using lower resolution mipmap levels. Common values are 32.0 or the actual mip level count.",
        Some("#dom-gpusamplerdescriptor-lodmaxclamp"),
    );

    pub const MAX_ANISOTROPY: TooltipInfo = TooltipInfo::new(
        "Maximum anisotropy level for anisotropic filtering. Higher values (up to 16) improve texture quality at oblique viewing angles but may reduce performance. A value of 1 disables anisotropic filtering.",
        Some("#dom-gpusamplerdescriptor-maxanisotropy"),
    );

    pub const BORDER_COLOR: TooltipInfo = TooltipInfo::new(
        "The color to use when the address mode is ClampToBorder and texture coordinates fall outside [0, 1]. Choose from predefined colors: transparent black, opaque black, opaque white, or zero.",
        Some("#dom-gpusamplerdescriptor-bordercolor"),
    );
}

/// Stencil operation tooltips
pub mod stencil_operation {
    use super::TooltipInfo;

    pub const KEEP: TooltipInfo = TooltipInfo::new(
        "Keep the current stencil value unchanged.",
        Some("#dom-gpustenciloperation-keep"),
    );

    pub const ZERO: TooltipInfo = TooltipInfo::new(
        "Set the stencil value to 0.",
        Some("#dom-gpustenciloperation-zero"),
    );

    pub const REPLACE: TooltipInfo = TooltipInfo::new(
        "Replace the stencil value with the reference value.",
        Some("#dom-gpustenciloperation-replace"),
    );

    pub const INVERT: TooltipInfo = TooltipInfo::new(
        "Bitwise invert the stencil value.",
        Some("#dom-gpustenciloperation-invert"),
    );

    pub const INCREMENT_CLAMP: TooltipInfo = TooltipInfo::new(
        "Increment the stencil value, clamping to the maximum value.",
        Some("#dom-gpustenciloperation-increment-clamp"),
    );

    pub const DECREMENT_CLAMP: TooltipInfo = TooltipInfo::new(
        "Decrement the stencil value, clamping to 0.",
        Some("#dom-gpustenciloperation-decrement-clamp"),
    );

    pub const INCREMENT_WRAP: TooltipInfo = TooltipInfo::new(
        "Increment the stencil value, wrapping to 0 on overflow.",
        Some("#dom-gpustenciloperation-increment-wrap"),
    );

    pub const DECREMENT_WRAP: TooltipInfo = TooltipInfo::new(
        "Decrement the stencil value, wrapping to the maximum value on underflow.",
        Some("#dom-gpustenciloperation-decrement-wrap"),
    );
}

/// General property tooltips
pub mod property {
    use super::TooltipInfo;

    pub const BUFFER_SIZE: TooltipInfo = TooltipInfo::new(
        "Size of the buffer in bytes. Must be greater than 0 and aligned to the usage requirements.",
        Some("#dom-gpubufferdescriptor-size"),
    );

    pub const BUFFER_MAPPED_AT_CREATION: TooltipInfo = TooltipInfo::new(
        "Whether the buffer should be mapped immediately after creation. Allows writing initial data without a separate map operation.",
        Some("#dom-gpubufferdescriptor-mappedatcreation"),
    );

    pub const TEXTURE_WIDTH: TooltipInfo = TooltipInfo::new(
        "Width of the texture in pixels. Must be greater than 0.",
        Some("#dom-gpuextent3ddict-width"),
    );

    pub const TEXTURE_HEIGHT: TooltipInfo = TooltipInfo::new(
        "Height of the texture in pixels. Must be greater than 0.",
        Some("#dom-gpuextent3ddict-height"),
    );

    pub const TEXTURE_DEPTH: TooltipInfo = TooltipInfo::new(
        "Depth or array layer count. For 2D textures, this is the number of array layers. For 3D textures, this is the depth.",
        Some("#dom-gpuextent3ddict-depthorarraylayers"),
    );

    pub const TEXTURE_MIP_LEVELS: TooltipInfo = TooltipInfo::new(
        "Number of mip levels. Mip levels are progressively smaller versions of the texture used for improved rendering quality at different distances.",
        Some("#dom-gputexturedescriptor-miplevelcount"),
    );

    pub const TEXTURE_SAMPLE_COUNT: TooltipInfo = TooltipInfo::new(
        "Number of samples per pixel for multisampling. Must be 1 (no multisampling) or a power of 2 up to the device's maximum.",
        Some("#dom-gputexturedescriptor-samplecount"),
    );

    pub const DEPTH_WRITE_ENABLED: TooltipInfo = TooltipInfo::new(
        "Whether fragments can write to the depth buffer. Usually enabled for opaque objects and disabled for transparent ones.",
        Some("#dom-gpudepthstencilstate-depthwriteenabled"),
    );

    pub const ALPHA_TO_COVERAGE: TooltipInfo = TooltipInfo::new(
        "Whether to use the alpha channel to determine coverage for multisampled rendering. Useful for foliage and transparent textures.",
        Some("#dom-gpumultisamplestate-alphatocoverageenabled"),
    );

    pub const SAMPLE_COUNT: TooltipInfo = TooltipInfo::new(
        "Number of samples per pixel for multisampling anti-aliasing (MSAA). Higher values provide better quality but require more memory.",
        Some("#dom-gpumultisamplestate-count"),
    );
}

/// Shader visibility tooltips
pub mod shader_visibility {
    use super::TooltipInfo;

    pub const VERTEX: TooltipInfo = TooltipInfo::new(
        "Resource is visible to vertex shaders. Vertex shaders process each vertex of geometry.",
        Some("#dom-gpushadervisibility-vertex"),
    );

    pub const FRAGMENT: TooltipInfo = TooltipInfo::new(
        "Resource is visible to fragment shaders. Fragment shaders determine the color of each pixel.",
        Some("#dom-gpushadervisibility-fragment"),
    );

    pub const COMPUTE: TooltipInfo = TooltipInfo::new(
        "Resource is visible to compute shaders. Compute shaders perform general-purpose GPU computation.",
        Some("#dom-gpushadervisibility-compute"),
    );
}

/// Compute dispatch tooltips
pub mod compute {
    use super::TooltipInfo;

    pub const WORKGROUP_COUNT_X: TooltipInfo = TooltipInfo::new(
        "Number of workgroups dispatched in the X dimension. Each workgroup executes the compute shader with the workgroup size defined in the shader. Total invocations in X = workgroup_count_x * workgroup_size_x.",
        Some("#dom-gpucomputepassencoder-dispatchworkgroups"),
    );

    pub const WORKGROUP_COUNT_Y: TooltipInfo = TooltipInfo::new(
        "Number of workgroups dispatched in the Y dimension. Each workgroup executes the compute shader with the workgroup size defined in the shader. Total invocations in Y = workgroup_count_y * workgroup_size_y.",
        Some("#dom-gpucomputepassencoder-dispatchworkgroups"),
    );

    pub const WORKGROUP_COUNT_Z: TooltipInfo = TooltipInfo::new(
        "Number of workgroups dispatched in the Z dimension. Each workgroup executes the compute shader with the workgroup size defined in the shader. Total invocations in Z = workgroup_count_z * workgroup_size_z.",
        Some("#dom-gpucomputepassencoder-dispatchworkgroups"),
    );
}
