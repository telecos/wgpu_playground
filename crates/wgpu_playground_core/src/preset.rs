/// Configuration presets for common rendering scenarios
///
/// This module provides preset configurations that users can load and customize
/// for common rendering scenarios like PBR materials, shadow mapping, and post-processing.
use crate::state::{
    BufferPanelState, PlaygroundState, RenderPipelinePanelState, SamplerPanelState,
    ShaderEditorState, TexturePanelState,
};
use serde::{Deserialize, Serialize};

/// A configuration preset with metadata and state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigPreset {
    /// Unique identifier for the preset
    pub id: &'static str,
    /// Display name for the preset
    pub name: &'static str,
    /// Category of the preset
    pub category: PresetCategory,
    /// Detailed description of what the preset demonstrates
    pub description: &'static str,
    /// Tags for searchability (not serialized, for in-memory use only)
    #[serde(skip)]
    pub tags: &'static [&'static str],
    /// The playground state containing the configuration
    pub state: PlaygroundState,
}

/// Category of configuration preset
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PresetCategory {
    /// Material and shading presets
    Material,
    /// Lighting and shadow presets
    Lighting,
    /// Post-processing effects
    PostProcessing,
    /// General rendering techniques
    Rendering,
}

impl ConfigPreset {
    /// Create a new configuration preset
    pub fn new(
        id: &'static str,
        name: &'static str,
        category: PresetCategory,
        description: &'static str,
        tags: &'static [&'static str],
        state: PlaygroundState,
    ) -> Self {
        Self {
            id,
            name,
            category,
            description,
            tags,
            state,
        }
    }
}

/// Get all available configuration presets
pub fn get_all_presets() -> Vec<ConfigPreset> {
    vec![
        create_pbr_material_preset(),
        create_shadow_mapping_preset(),
        create_post_processing_preset(),
    ]
}

/// Create PBR (Physically Based Rendering) material preset
fn create_pbr_material_preset() -> ConfigPreset {
    let mut state = PlaygroundState::new();

    // Shader for PBR material
    state.shader_editor = Some(ShaderEditorState {
        source_code: r#"// PBR Material Shader
// Physically Based Rendering with Cook-Torrance BRDF

struct Material {
    albedo: vec3<f32>,
    metallic: f32,
    roughness: f32,
    ao: f32,
}

struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
    intensity: f32,
}

@group(0) @binding(0)
var<uniform> material: Material;

@group(0) @binding(1)
var<uniform> light: Light;

@group(0) @binding(2)
var<uniform> camera_pos: vec3<f32>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(in.position, 1.0);
    out.world_position = in.position;
    out.normal = normalize(in.normal);
    out.uv = in.uv;
    return out;
}

// Normal Distribution Function (GGX/Trowbridge-Reitz)
fn distribution_ggx(N: vec3<f32>, H: vec3<f32>, roughness: f32) -> f32 {
    let a = roughness * roughness;
    let a2 = a * a;
    let NdotH = max(dot(N, H), 0.0);
    let NdotH2 = NdotH * NdotH;
    
    let num = a2;
    var denom = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = 3.14159265359 * denom * denom;
    
    return num / denom;
}

// Geometry Function (Schlick-GGX)
fn geometry_schlick_ggx(NdotV: f32, roughness: f32) -> f32 {
    let r = (roughness + 1.0);
    let k = (r * r) / 8.0;
    
    let num = NdotV;
    let denom = NdotV * (1.0 - k) + k;
    
    return num / denom;
}

fn geometry_smith(N: vec3<f32>, V: vec3<f32>, L: vec3<f32>, roughness: f32) -> f32 {
    let NdotV = max(dot(N, V), 0.0);
    let NdotL = max(dot(N, L), 0.0);
    let ggx2 = geometry_schlick_ggx(NdotV, roughness);
    let ggx1 = geometry_schlick_ggx(NdotL, roughness);
    
    return ggx1 * ggx2;
}

// Fresnel-Schlick approximation
fn fresnel_schlick(cos_theta: f32, F0: vec3<f32>) -> vec3<f32> {
    return F0 + (1.0 - F0) * pow(clamp(1.0 - cos_theta, 0.0, 1.0), 5.0);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let N = normalize(in.normal);
    let V = normalize(camera_pos - in.world_position);
    
    // Calculate reflectance at normal incidence
    var F0 = vec3<f32>(0.04);
    F0 = mix(F0, material.albedo, material.metallic);
    
    // Reflectance equation
    var Lo = vec3<f32>(0.0);
    
    // Calculate per-light radiance
    let L = normalize(light.position - in.world_position);
    let H = normalize(V + L);
    let distance = length(light.position - in.world_position);
    let attenuation = 1.0 / (distance * distance);
    let radiance = light.color * light.intensity * attenuation;
    
    // Cook-Torrance BRDF
    let NDF = distribution_ggx(N, H, material.roughness);
    let G = geometry_smith(N, V, L, material.roughness);
    let F = fresnel_schlick(max(dot(H, V), 0.0), F0);
    
    let kS = F;
    var kD = vec3<f32>(1.0) - kS;
    kD *= 1.0 - material.metallic;
    
    let NdotL = max(dot(N, L), 0.0);
    let numerator = NDF * G * F;
    let denominator = 4.0 * max(dot(N, V), 0.0) * NdotL + 0.0001;
    let specular = numerator / denominator;
    
    Lo += (kD * material.albedo / 3.14159265359 + specular) * radiance * NdotL;
    
    // Ambient lighting (simple ambient occlusion)
    let ambient = vec3<f32>(0.03) * material.albedo * material.ao;
    var color = ambient + Lo;
    
    // HDR tonemapping (Reinhard)
    color = color / (color + vec3<f32>(1.0));
    // Gamma correction
    color = pow(color, vec3<f32>(1.0/2.2));
    
    return vec4<f32>(color, 1.0);
}
"#.to_string(),
        label: "pbr_material".to_string(),
        file_path: String::new(),
    });

    // Uniform buffer for material properties
    state.buffer_panel = Some(BufferPanelState {
        label: "material_uniforms".to_string(),
        size: "48".to_string(), // vec3 + f32 + f32 + f32 = 48 bytes with padding
        usage_vertex: false,
        usage_index: false,
        usage_uniform: true,
        usage_storage: false,
        usage_indirect: false,
        usage_copy_src: false,
        usage_copy_dst: true,
        usage_map_read: false,
        usage_map_write: false,
        usage_query_resolve: false,
        mapped_at_creation: false,
    });

    // Render pipeline configuration for PBR
    state.render_pipeline_panel = Some(RenderPipelinePanelState {
        label: "pbr_pipeline".to_string(),
        vertex_entry_point: "vs_main".to_string(),
        fragment_entry_point: "fs_main".to_string(),
        topology: "TriangleList".to_string(),
        cull_mode: "Back".to_string(),
        front_face: "Ccw".to_string(),
        enable_depth_stencil: true,
        depth_format: "Depth24Plus".to_string(),
        depth_write_enabled: true,
        depth_compare: "Less".to_string(),
        stencil_read_mask: "0xFF".to_string(),
        stencil_write_mask: "0xFF".to_string(),
        stencil_front_compare: "Always".to_string(),
        stencil_front_fail_op: "Keep".to_string(),
        stencil_front_depth_fail_op: "Keep".to_string(),
        stencil_front_pass_op: "Keep".to_string(),
        stencil_back_compare: "Always".to_string(),
        stencil_back_fail_op: "Keep".to_string(),
        stencil_back_depth_fail_op: "Keep".to_string(),
        stencil_back_pass_op: "Keep".to_string(),
        sample_count: 1,
        alpha_to_coverage_enabled: false,
        target_format: "Bgra8UnormSrgb".to_string(),
        blend_enabled: false,
        color_blend_src: "One".to_string(),
        color_blend_dst: "Zero".to_string(),
        color_blend_op: "Add".to_string(),
        alpha_blend_src: "One".to_string(),
        alpha_blend_dst: "Zero".to_string(),
        alpha_blend_op: "Add".to_string(),
        write_red: true,
        write_green: true,
        write_blue: true,
        write_alpha: true,
    });

    ConfigPreset::new(
        "pbr_material",
        "PBR Material",
        PresetCategory::Material,
        "Physically Based Rendering material using Cook-Torrance BRDF. \
         Demonstrates realistic material properties including albedo, metallic, \
         roughness, and ambient occlusion. Uses proper energy conservation and \
         Fresnel effects for photorealistic rendering.",
        &["pbr", "material", "lighting", "brdf", "realistic"],
        state,
    )
}

/// Create shadow mapping preset
fn create_shadow_mapping_preset() -> ConfigPreset {
    let mut state = PlaygroundState::new();

    // Shader for shadow mapping
    state.shader_editor = Some(ShaderEditorState {
        source_code: r#"// Shadow Mapping Shader
// Two-pass shadow mapping with PCF filtering

// Pass 1: Shadow map generation
struct ShadowVertexInput {
    @location(0) position: vec3<f32>,
}

struct LightSpaceUniforms {
    light_view_proj: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> light_space: LightSpaceUniforms;

@vertex
fn shadow_vs_main(in: ShadowVertexInput) -> @builtin(position) vec4<f32> {
    return light_space.light_view_proj * vec4<f32>(in.position, 1.0);
}

// No fragment shader needed for depth-only pass

// Pass 2: Scene rendering with shadows
struct SceneUniforms {
    view_proj: mat4x4<f32>,
    light_view_proj: mat4x4<f32>,
    light_pos: vec3<f32>,
}

@group(0) @binding(0)
var<uniform> scene: SceneUniforms;

@group(0) @binding(1)
var shadow_map: texture_depth_2d;

@group(0) @binding(2)
var shadow_sampler: sampler_comparison;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) light_space_position: vec4<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let world_pos = vec4<f32>(in.position, 1.0);
    out.clip_position = scene.view_proj * world_pos;
    out.world_position = in.position;
    out.normal = normalize(in.normal);
    out.light_space_position = scene.light_view_proj * world_pos;
    return out;
}

// PCF (Percentage Closer Filtering) for soft shadows
fn calculate_shadow(light_space_pos: vec4<f32>) -> f32 {
    // Perspective divide
    var proj_coords = light_space_pos.xyz / light_space_pos.w;
    // Transform to [0,1] range
    proj_coords = proj_coords * 0.5 + 0.5;
    
    // Get depth of current fragment from light's perspective
    let current_depth = proj_coords.z;
    
    // PCF with 3x3 kernel
    var shadow = 0.0;
    let texel_size = 1.0 / 2048.0; // Assume 2048x2048 shadow map
    
    for (var x = -1; x <= 1; x++) {
        for (var y = -1; y <= 1; y++) {
            let offset = vec2<f32>(f32(x), f32(y)) * texel_size;
            let sample_coords = proj_coords.xy + offset;
            shadow += textureSampleCompare(
                shadow_map,
                shadow_sampler,
                sample_coords,
                current_depth - 0.005 // Bias to prevent shadow acne
            );
        }
    }
    shadow /= 9.0;
    
    // Keep the shadow if we're outside the light's frustum
    if (proj_coords.x < 0.0 || proj_coords.x > 1.0 ||
        proj_coords.y < 0.0 || proj_coords.y > 1.0 ||
        proj_coords.z < 0.0 || proj_coords.z > 1.0) {
        shadow = 1.0;
    }
    
    return shadow;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let N = normalize(in.normal);
    let L = normalize(scene.light_pos - in.world_position);
    
    // Calculate diffuse lighting
    let diffuse = max(dot(N, L), 0.0);
    
    // Calculate shadow factor
    let shadow = calculate_shadow(in.light_space_position);
    
    // Combine lighting with shadow
    let ambient = 0.2;
    let lighting = ambient + (1.0 - ambient) * diffuse * shadow;
    
    let color = vec3<f32>(0.8, 0.8, 0.8);
    return vec4<f32>(color * lighting, 1.0);
}
"#.to_string(),
        label: "shadow_mapping".to_string(),
        file_path: String::new(),
    });

    // Shadow map texture (depth texture)
    state.texture_panel = Some(TexturePanelState {
        label: "shadow_map".to_string(),
        width: "2048".to_string(),
        height: "2048".to_string(),
        depth: "1".to_string(),
        mip_levels: "1".to_string(),
        sample_count: "1".to_string(),
        format: "Depth24Plus".to_string(),
        dimension: "D2".to_string(),
        usage_copy_src: false,
        usage_copy_dst: false,
        usage_texture_binding: true,
        usage_storage_binding: false,
        usage_render_attachment: true,
    });

    // Comparison sampler for shadow map
    state.sampler_panel = Some(SamplerPanelState {
        label: "shadow_sampler".to_string(),
        address_mode_u: "ClampToEdge".to_string(),
        address_mode_v: "ClampToEdge".to_string(),
        address_mode_w: "ClampToEdge".to_string(),
        mag_filter: "Linear".to_string(),
        min_filter: "Linear".to_string(),
        mipmap_filter: "Nearest".to_string(),
        lod_min_clamp: "0.0".to_string(),
        lod_max_clamp: "32.0".to_string(),
        compare: Some("Less".to_string()),
        max_anisotropy: "1".to_string(),
    });

    // Uniform buffer for light space transformation
    state.buffer_panel = Some(BufferPanelState {
        label: "light_space_uniforms".to_string(),
        size: "128".to_string(), // Two 4x4 matrices + vec3 + padding
        usage_vertex: false,
        usage_index: false,
        usage_uniform: true,
        usage_storage: false,
        usage_indirect: false,
        usage_copy_src: false,
        usage_copy_dst: true,
        usage_map_read: false,
        usage_map_write: false,
        usage_query_resolve: false,
        mapped_at_creation: false,
    });

    // Render pipeline for shadow rendering
    state.render_pipeline_panel = Some(RenderPipelinePanelState {
        label: "shadow_pipeline".to_string(),
        vertex_entry_point: "vs_main".to_string(),
        fragment_entry_point: "fs_main".to_string(),
        topology: "TriangleList".to_string(),
        cull_mode: "Back".to_string(),
        front_face: "Ccw".to_string(),
        enable_depth_stencil: true,
        depth_format: "Depth24Plus".to_string(),
        depth_write_enabled: true,
        depth_compare: "Less".to_string(),
        stencil_read_mask: "0xFF".to_string(),
        stencil_write_mask: "0xFF".to_string(),
        stencil_front_compare: "Always".to_string(),
        stencil_front_fail_op: "Keep".to_string(),
        stencil_front_depth_fail_op: "Keep".to_string(),
        stencil_front_pass_op: "Keep".to_string(),
        stencil_back_compare: "Always".to_string(),
        stencil_back_fail_op: "Keep".to_string(),
        stencil_back_depth_fail_op: "Keep".to_string(),
        stencil_back_pass_op: "Keep".to_string(),
        sample_count: 1,
        alpha_to_coverage_enabled: false,
        target_format: "Bgra8UnormSrgb".to_string(),
        blend_enabled: false,
        color_blend_src: "One".to_string(),
        color_blend_dst: "Zero".to_string(),
        color_blend_op: "Add".to_string(),
        alpha_blend_src: "One".to_string(),
        alpha_blend_dst: "Zero".to_string(),
        alpha_blend_op: "Add".to_string(),
        write_red: true,
        write_green: true,
        write_blue: true,
        write_alpha: true,
    });

    ConfigPreset::new(
        "shadow_mapping",
        "Shadow Mapping",
        PresetCategory::Lighting,
        "Two-pass shadow mapping with PCF (Percentage Closer Filtering) for soft shadows. \
         First pass renders the scene from the light's perspective to generate a depth map. \
         Second pass renders the scene normally and samples the shadow map to determine \
         which fragments are in shadow. Includes bias to prevent shadow acne.",
        &["shadows", "lighting", "depth", "pcf", "filtering"],
        state,
    )
}

/// Create post-processing preset
fn create_post_processing_preset() -> ConfigPreset {
    let mut state = PlaygroundState::new();

    // Shader for post-processing effects
    state.shader_editor = Some(ShaderEditorState {
        source_code: r#"// Post-Processing Shader
// Full-screen quad with multiple effects

struct PostProcessUniforms {
    time: f32,
    vignette_intensity: f32,
    chromatic_aberration: f32,
    bloom_threshold: f32,
}

@group(0) @binding(0)
var input_texture: texture_2d<f32>;

@group(0) @binding(1)
var input_sampler: sampler;

@group(0) @binding(2)
var<uniform> uniforms: PostProcessUniforms;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

// Full-screen quad vertex shader
@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    
    // Generate full-screen triangle
    let x = f32((vertex_index & 1u) << 2u);
    let y = f32((vertex_index & 2u) << 1u);
    
    out.position = vec4<f32>(x - 1.0, 1.0 - y, 0.0, 1.0);
    out.uv = vec2<f32>(x * 0.5, y * 0.5);
    
    return out;
}

// Vignette effect
fn apply_vignette(color: vec3<f32>, uv: vec2<f32>, intensity: f32) -> vec3<f32> {
    let center_dist = length(uv - vec2<f32>(0.5, 0.5));
    let vignette = smoothstep(0.8, 0.2, center_dist * intensity);
    return color * vignette;
}

// Chromatic aberration
fn apply_chromatic_aberration(uv: vec2<f32>, amount: f32) -> vec3<f32> {
    let direction = uv - vec2<f32>(0.5, 0.5);
    let r_offset = uv + direction * amount;
    let g_offset = uv;
    let b_offset = uv - direction * amount;
    
    let r = textureSample(input_texture, input_sampler, r_offset).r;
    let g = textureSample(input_texture, input_sampler, g_offset).g;
    let b = textureSample(input_texture, input_sampler, b_offset).b;
    
    return vec3<f32>(r, g, b);
}

// Simple bloom (bright pass)
fn extract_bright_areas(color: vec3<f32>, threshold: f32) -> vec3<f32> {
    let brightness = dot(color, vec3<f32>(0.2126, 0.7152, 0.0722));
    if (brightness > threshold) {
        return color * (brightness - threshold) / (1.0 - threshold);
    }
    return vec3<f32>(0.0);
}

// Gaussian blur (simple 3x3 kernel)
fn gaussian_blur(uv: vec2<f32>) -> vec3<f32> {
    let texel_size = 1.0 / vec2<f32>(textureDimensions(input_texture));
    
    var result = vec3<f32>(0.0);
    
    // 3x3 Gaussian kernel
    result += textureSample(input_texture, input_sampler, uv + vec2<f32>(-1.0, -1.0) * texel_size).rgb * 0.077847;
    result += textureSample(input_texture, input_sampler, uv + vec2<f32>(0.0, -1.0) * texel_size).rgb * 0.123317;
    result += textureSample(input_texture, input_sampler, uv + vec2<f32>(1.0, -1.0) * texel_size).rgb * 0.077847;
    
    result += textureSample(input_texture, input_sampler, uv + vec2<f32>(-1.0, 0.0) * texel_size).rgb * 0.123317;
    result += textureSample(input_texture, input_sampler, uv).rgb * 0.195346;
    result += textureSample(input_texture, input_sampler, uv + vec2<f32>(1.0, 0.0) * texel_size).rgb * 0.123317;
    
    result += textureSample(input_texture, input_sampler, uv + vec2<f32>(-1.0, 1.0) * texel_size).rgb * 0.077847;
    result += textureSample(input_texture, input_sampler, uv + vec2<f32>(0.0, 1.0) * texel_size).rgb * 0.123317;
    result += textureSample(input_texture, input_sampler, uv + vec2<f32>(1.0, 1.0) * texel_size).rgb * 0.077847;
    
    return result;
}

// Tone mapping (ACES approximation)
fn aces_tonemap(color: vec3<f32>) -> vec3<f32> {
    let a = 2.51;
    let b = 0.03;
    let c = 2.43;
    let d = 0.59;
    let e = 0.14;
    return clamp((color * (a * color + b)) / (color * (c * color + d) + e), vec3<f32>(0.0), vec3<f32>(1.0));
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var color: vec3<f32>;
    
    // Apply chromatic aberration
    if (uniforms.chromatic_aberration > 0.0) {
        color = apply_chromatic_aberration(in.uv, uniforms.chromatic_aberration);
    } else {
        color = textureSample(input_texture, input_sampler, in.uv).rgb;
    }
    
    // Extract bright areas for bloom
    let bright = extract_bright_areas(color, uniforms.bloom_threshold);
    
    // Apply tone mapping
    color = aces_tonemap(color);
    
    // Apply vignette
    if (uniforms.vignette_intensity > 0.0) {
        color = apply_vignette(color, in.uv, uniforms.vignette_intensity);
    }
    
    // Gamma correction
    color = pow(color, vec3<f32>(1.0 / 2.2));
    
    return vec4<f32>(color, 1.0);
}
"#.to_string(),
        label: "post_processing".to_string(),
        file_path: String::new(),
    });

    // Input texture from previous render pass
    state.texture_panel = Some(TexturePanelState {
        label: "scene_texture".to_string(),
        width: "1920".to_string(),
        height: "1080".to_string(),
        depth: "1".to_string(),
        mip_levels: "1".to_string(),
        sample_count: "1".to_string(),
        format: "Rgba16Float".to_string(),
        dimension: "D2".to_string(),
        usage_copy_src: false,
        usage_copy_dst: false,
        usage_texture_binding: true,
        usage_storage_binding: false,
        usage_render_attachment: true,
    });

    // Linear sampler for post-processing
    state.sampler_panel = Some(SamplerPanelState {
        label: "post_process_sampler".to_string(),
        address_mode_u: "ClampToEdge".to_string(),
        address_mode_v: "ClampToEdge".to_string(),
        address_mode_w: "ClampToEdge".to_string(),
        mag_filter: "Linear".to_string(),
        min_filter: "Linear".to_string(),
        mipmap_filter: "Linear".to_string(),
        lod_min_clamp: "0.0".to_string(),
        lod_max_clamp: "1.0".to_string(),
        compare: None,
        max_anisotropy: "1".to_string(),
    });

    // Uniform buffer for post-processing parameters
    state.buffer_panel = Some(BufferPanelState {
        label: "post_process_uniforms".to_string(),
        size: "16".to_string(), // 4 floats
        usage_vertex: false,
        usage_index: false,
        usage_uniform: true,
        usage_storage: false,
        usage_indirect: false,
        usage_copy_src: false,
        usage_copy_dst: true,
        usage_map_read: false,
        usage_map_write: false,
        usage_query_resolve: false,
        mapped_at_creation: false,
    });

    // Render pipeline for post-processing
    state.render_pipeline_panel = Some(RenderPipelinePanelState {
        label: "post_process_pipeline".to_string(),
        vertex_entry_point: "vs_main".to_string(),
        fragment_entry_point: "fs_main".to_string(),
        topology: "TriangleList".to_string(),
        cull_mode: "None".to_string(),
        front_face: "Ccw".to_string(),
        enable_depth_stencil: false,
        depth_format: "Depth24Plus".to_string(),
        depth_write_enabled: false,
        depth_compare: "Always".to_string(),
        stencil_read_mask: "0xFF".to_string(),
        stencil_write_mask: "0xFF".to_string(),
        stencil_front_compare: "Always".to_string(),
        stencil_front_fail_op: "Keep".to_string(),
        stencil_front_depth_fail_op: "Keep".to_string(),
        stencil_front_pass_op: "Keep".to_string(),
        stencil_back_compare: "Always".to_string(),
        stencil_back_fail_op: "Keep".to_string(),
        stencil_back_depth_fail_op: "Keep".to_string(),
        stencil_back_pass_op: "Keep".to_string(),
        sample_count: 1,
        alpha_to_coverage_enabled: false,
        target_format: "Bgra8UnormSrgb".to_string(),
        blend_enabled: false,
        color_blend_src: "One".to_string(),
        color_blend_dst: "Zero".to_string(),
        color_blend_op: "Add".to_string(),
        alpha_blend_src: "One".to_string(),
        alpha_blend_dst: "Zero".to_string(),
        alpha_blend_op: "Add".to_string(),
        write_red: true,
        write_green: true,
        write_blue: true,
        write_alpha: true,
    });

    ConfigPreset::new(
        "post_processing",
        "Post-Processing Effects",
        PresetCategory::PostProcessing,
        "Full-screen post-processing pipeline with multiple effects. Includes vignette, \
         chromatic aberration, bloom (bright pass extraction), ACES tone mapping, and \
         gamma correction. Demonstrates how to chain multiple image effects and work \
         with full-screen quads for image-based rendering techniques.",
        &[
            "post-processing",
            "effects",
            "vignette",
            "bloom",
            "tone-mapping",
            "chromatic-aberration",
        ],
        state,
    )
}
