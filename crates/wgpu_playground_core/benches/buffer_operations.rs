use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wgpu_playground_core::buffer::{BufferDescriptor, BufferUsages};

fn buffer_descriptor_creation(c: &mut Criterion) {
    c.bench_function("buffer_descriptor_new", |b| {
        b.iter(|| {
            let desc =
                BufferDescriptor::new(Some("test_buffer"), black_box(1024), BufferUsages::VERTEX);
            black_box(desc)
        })
    });
}

fn buffer_descriptor_with_usage(c: &mut Criterion) {
    c.bench_function("buffer_descriptor_with_usage", |b| {
        b.iter(|| {
            let usage = BufferUsages::VERTEX | BufferUsages::INDEX | BufferUsages::COPY_DST;
            let desc = BufferDescriptor::new(Some("test_buffer"), black_box(1024), usage);
            black_box(desc)
        })
    });
}

fn buffer_validation(c: &mut Criterion) {
    c.bench_function("buffer_validation", |b| {
        b.iter(|| {
            let desc =
                BufferDescriptor::new(Some("test_buffer"), black_box(1024), BufferUsages::VERTEX);
            let result = desc.validate();
            black_box(result)
        })
    });
}

fn buffer_usage_operations(c: &mut Criterion) {
    c.bench_function("buffer_usage_to_wgpu", |b| {
        b.iter(|| {
            let usage = BufferUsages::VERTEX | BufferUsages::INDEX;
            let wgpu_usage = usage.to_wgpu();
            black_box(wgpu_usage)
        })
    });

    c.bench_function("buffer_usage_from_wgpu", |b| {
        let wgpu_usage = wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::INDEX;
        b.iter(|| {
            let usage = BufferUsages::from_wgpu(black_box(wgpu_usage));
            black_box(usage)
        })
    });
}

criterion_group!(
    benches,
    buffer_descriptor_creation,
    buffer_descriptor_with_usage,
    buffer_validation,
    buffer_usage_operations
);
criterion_main!(benches);
