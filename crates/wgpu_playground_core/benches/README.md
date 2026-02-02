# Performance Benchmarks

This directory contains performance benchmarks for the wgpu_playground_core crate using [Criterion.rs](https://github.com/bheisler/criterion.rs).

## Available Benchmarks

### buffer_operations.rs
Benchmarks for buffer descriptor creation and validation:
- `buffer_descriptor_new`: Creating new buffer descriptors
- `buffer_descriptor_with_usage`: Creating buffers with combined usage flags
- `buffer_validation`: Validating buffer descriptors
- `buffer_usage_to_wgpu`: Converting usage flags to wgpu format
- `buffer_usage_from_wgpu`: Converting from wgpu format to internal format

### shader_compilation.rs
Benchmarks for shader editor operations:
- `shader_editor_new`: Creating new shader editor instances
- `shader_set_source_simple`: Setting simple shader source code
- `shader_set_source_complex`: Setting complex shader source code
- `shader_source_code`: Retrieving shader source code

## Running Benchmarks

### Run all benchmarks
```bash
cargo bench
```

### Run specific benchmark
```bash
cargo bench --bench buffer_operations
cargo bench --bench shader_compilation
```

### Run specific benchmark function
```bash
cargo bench --bench buffer_operations -- buffer_descriptor_new
```

## CI Integration

Benchmarks are automatically run by GitHub Actions:
- **Weekly**: Every Monday at 00:00 UTC
- **Manual**: Can be triggered from the Actions tab
- **PR Changes**: When benchmark files are modified

### Baseline Comparison
The CI workflow compares benchmark results against the `main` branch baseline to detect performance regressions.

### Artifacts
Benchmark results are stored as artifacts for 90 days:
- HTML reports with detailed statistics
- Comparison results when run against baseline
- Historical trend data

## Viewing Results

After running benchmarks locally, open the HTML reports:
```bash
open target/criterion/report/index.html
```

Or for a specific benchmark:
```bash
open target/criterion/buffer_descriptor_new/report/index.html
```

## Adding New Benchmarks

1. Create a new benchmark file in this directory (e.g., `my_benchmark.rs`)
2. Add it to `Cargo.toml`:
   ```toml
   [[bench]]
   name = "my_benchmark"
   harness = false
   ```
3. Use the criterion template:
   ```rust
   use criterion::{black_box, criterion_group, criterion_main, Criterion};
   
   fn my_benchmark_function(c: &mut Criterion) {
       c.bench_function("my_benchmark", |b| {
           b.iter(|| {
               // Code to benchmark
               black_box(some_operation())
           })
       });
   }
   
   criterion_group!(benches, my_benchmark_function);
   criterion_main!(benches);
   ```

## Performance Guidelines

- Benchmarks should focus on CPU-bound operations (buffer config, validation, etc.)
- GPU operations are not suitable for criterion benchmarks (use integration tests instead)
- Keep benchmark functions focused and isolated
- Use `black_box()` to prevent compiler optimizations from skewing results
- Avoid I/O operations in benchmarks when possible

## Interpreting Results

Criterion provides:
- **Mean**: Average execution time
- **Std. Dev.**: Standard deviation
- **Median**: Middle value of all measurements
- **MAD**: Median Absolute Deviation

Look for:
- Consistent performance across runs (low std. dev.)
- Significant changes when compared to baseline (>10% regression)
- Unexpected outliers that might indicate issues
