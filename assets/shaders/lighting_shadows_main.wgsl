// Main Pass Shader
// Renders the scene with Phong lighting and shadow mapping

// Bind Group 0: Camera
struct CameraUniforms {
    view_proj: mat4x4<f32>,
    camera_pos: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraUniforms;

// Bind Group 1: Lights
struct LightUniforms {
    dir_light_direction: vec4<f32>,
    dir_light_color: vec4<f32>,
    point_light_position: vec4<f32>,
    point_light_color: vec4<f32>,
}

@group(1) @binding(0)
var<uniform> lights: LightUniforms;

// Bind Group 2: Shadow Map
@group(2) @binding(0)
var shadow_map: texture_depth_2d;

@group(2) @binding(1)
var shadow_sampler: sampler_comparison;

struct ShadowUniforms {
    light_view_proj: mat4x4<f32>,
}

@group(2) @binding(2)
var<uniform> shadow_uniforms: ShadowUniforms;

// Vertex Input/Output
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) shadow_pos: vec4<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    
    // Transform position to clip space
    output.clip_position = camera.view_proj * vec4<f32>(input.position, 1.0);
    
    // Pass world position and normal to fragment shader
    output.world_position = input.position;
    output.world_normal = input.normal;
    
    // Transform position to light space for shadow mapping
    output.shadow_pos = shadow_uniforms.light_view_proj * vec4<f32>(input.position, 1.0);
    
    return output;
}

// Shadow mapping with hardware PCF (Percentage Closer Filtering)
fn calculate_shadow(shadow_pos: vec4<f32>) -> f32 {
    // Perspective divide to get NDC coordinates
    var proj_coords = shadow_pos.xyz / shadow_pos.w;
    
    // Transform from NDC [-1, 1] to texture coordinates [0, 1]
    proj_coords = proj_coords * 0.5 + 0.5;
    
    // Flip Y coordinate (WebGPU texture coordinates have origin at top-left)
    proj_coords.y = 1.0 - proj_coords.y;
    
    // Check if outside shadow map bounds
    if (proj_coords.x < 0.0 || proj_coords.x > 1.0 ||
        proj_coords.y < 0.0 || proj_coords.y > 1.0 ||
        proj_coords.z < 0.0 || proj_coords.z > 1.0) {
        return 1.0; // Not in shadow if outside bounds
    }
    
    // Use comparison sampler for hardware PCF
    // Returns 1.0 if depth test passes (not in shadow), 0.0 if fails (in shadow)
    // Bias of 0.005 helps prevent shadow acne (self-shadowing artifacts)
    let shadow = textureSampleCompare(
        shadow_map,
        shadow_sampler,
        proj_coords.xy,
        proj_coords.z - 0.005
    );
    
    return shadow;
}

// Phong lighting model
fn calculate_phong_lighting(
    world_pos: vec3<f32>,
    world_normal: vec3<f32>,
    view_dir: vec3<f32>,
    light_dir: vec3<f32>,
    light_color: vec3<f32>,
    light_intensity: f32,
    shadow: f32
) -> vec3<f32> {
    let normal = normalize(world_normal);
    let light_direction = normalize(light_dir);
    
    // Ambient lighting (10% contribution, always present)
    let ambient_strength = 0.1;
    let ambient = ambient_strength * light_color;
    
    // Diffuse lighting (Lambert's cosine law)
    let diff = max(dot(normal, light_direction), 0.0);
    let diffuse = diff * light_color;
    
    // Specular lighting (Phong model)
    // Shininess of 32 creates moderately shiny surface
    let reflect_dir = reflect(-light_direction, normal);
    let spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32.0);
    let specular_strength = 0.5; // 50% specular contribution
    let specular = specular_strength * spec * light_color;
    
    // Combine with shadow (ambient is not affected by shadows)
    return (ambient + shadow * (diffuse + specular)) * light_intensity;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Base material color (light gray)
    let material_color = vec3<f32>(0.8, 0.8, 0.8);
    
    // Calculate view direction
    let view_dir = normalize(camera.camera_pos.xyz - input.world_position);
    
    // Calculate shadow factor
    let shadow = calculate_shadow(input.shadow_pos);
    
    // Directional light contribution
    let dir_light_dir = -lights.dir_light_direction.xyz;
    let dir_light_contrib = calculate_phong_lighting(
        input.world_position,
        input.world_normal,
        view_dir,
        dir_light_dir,
        lights.dir_light_color.xyz,
        lights.dir_light_color.w,
        shadow
    );
    
    // Point light contribution
    let point_light_dir = lights.point_light_position.xyz - input.world_position;
    let point_light_dist = length(point_light_dir);
    // Quadratic attenuation: 1 / (1 + linear*d + quadratic*d^2)
    // Linear term: 0.09, Quadratic term: 0.032 (suitable for medium-range light)
    let point_light_attenuation = 1.0 / (1.0 + 0.09 * point_light_dist + 0.032 * point_light_dist * point_light_dist);
    
    let point_light_contrib = calculate_phong_lighting(
        input.world_position,
        input.world_normal,
        view_dir,
        point_light_dir,
        lights.point_light_color.xyz,
        lights.point_light_color.w * point_light_attenuation,
        1.0 // Point light not affected by directional light shadow
    );
    
    // Combine all lighting
    let final_color = material_color * (dir_light_contrib + point_light_contrib);
    
    return vec4<f32>(final_color, 1.0);
}
