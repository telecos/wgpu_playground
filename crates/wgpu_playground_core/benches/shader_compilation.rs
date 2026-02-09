use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use wgpu_playground_core::shader_editor::ShaderEditor;

const SIMPLE_SHADER: &str = r#"
@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    let x = f32(i32(in_vertex_index) - 1);
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
    return vec4<f32>(x, y, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
"#;

const COMPLEX_SHADER: &str = r#"
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

struct Camera {
    view_proj: mat4x4<f32>,
    position: vec3<f32>,
}

@group(0) @binding(0)
var<uniform> camera: Camera;

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.world_position = vertex.position;
    out.world_normal = vertex.normal;
    out.uv = vertex.uv;
    out.clip_position = camera.view_proj * vec4<f32>(vertex.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let light_dir = normalize(vec3<f32>(1.0, 1.0, 1.0));
    let diffuse = max(dot(in.world_normal, light_dir), 0.0);
    return vec4<f32>(vec3<f32>(diffuse), 1.0);
}
"#;

fn shader_editor_creation(c: &mut Criterion) {
    c.bench_function("shader_editor_new", |b| {
        b.iter(|| {
            let editor = ShaderEditor::new();
            black_box(editor)
        })
    });
}

fn shader_content_update(c: &mut Criterion) {
    c.bench_function("shader_set_source_simple", |b| {
        b.iter(|| {
            let mut editor = ShaderEditor::new();
            editor.set_source_code(black_box(SIMPLE_SHADER.to_string()));
            black_box(editor)
        })
    });

    c.bench_function("shader_set_source_complex", |b| {
        b.iter(|| {
            let mut editor = ShaderEditor::new();
            editor.set_source_code(black_box(COMPLEX_SHADER.to_string()));
            black_box(editor)
        })
    });
}

fn shader_get_content(c: &mut Criterion) {
    c.bench_function("shader_source_code", |b| {
        let mut editor = ShaderEditor::new();
        editor.set_source_code(SIMPLE_SHADER.to_string());

        b.iter(|| {
            let content = editor.source_code();
            black_box(content)
        })
    });
}

criterion_group!(
    benches,
    shader_editor_creation,
    shader_content_update,
    shader_get_content
);
criterion_main!(benches);
