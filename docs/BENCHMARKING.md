# Performance Benchmarking Guide

This guide explains how to use the performance benchmarking system in wgpu_playground.

## Overview

The project uses [Criterion.rs](https://github.com/bheisler/criterion.rs) for performance benchmarking, with automated CI jobs that run benchmarks on a schedule and compare results against a baseline.

## Running Benchmarks Locally

### Run all benchmarks
```bash
cargo bench
```

### Run specific benchmark suite
```bash
cargo bench --bench buffer_operations
cargo bench --bench shader_compilation
```

### Run specific benchmark function
```bash
cargo bench -- buffer_descriptor_new
```

### View results
After running benchmarks, open the HTML report:
```bash
open target/criterion/report/index.html
```

## CI Integration

### Automated Runs

Benchmarks run automatically:
- **Weekly**: Every Monday at 00:00 UTC
- **On PR**: When benchmark files are modified
- **Manual**: Via GitHub Actions workflow dispatch

### Manual Trigger

To manually run benchmarks:

1. Go to the [Actions tab](../../actions)
2. Select "Performance Benchmarks" workflow
3. Click "Run workflow"
4. Choose whether to compare against baseline
5. Click the green "Run workflow" button

### Workflow Features

The benchmark CI workflow provides:

1. **Baseline Comparison**
   - Compares current results against main branch
   - Detects performance regressions (>10% slower)
   - Posts results as PR comments

2. **Artifact Storage**
   - Stores benchmark results for 90 days
   - Keeps baseline results for 365 days
   - Includes HTML reports and raw data

3. **Trend Visualization**
   - Collects historical benchmark data
   - Available in workflow artifacts
   - Helps identify long-term performance trends

4. **PR Integration**
   - Automatically comments on PRs with benchmark results
   - Shows comparison with main branch
   - Highlights performance regressions

## Understanding Results

Criterion provides detailed statistics for each benchmark:

- **Mean**: Average execution time
- **Std. Dev.**: Standard deviation (consistency)
- **Median**: Middle value of measurements
- **MAD**: Median Absolute Deviation

### What to look for:

✅ **Good**:
- Low standard deviation (< 5%)
- Consistent results across runs
- Similar or better performance vs baseline

⚠️ **Needs attention**:
- High standard deviation (> 10%)
- Significant regression (> 10% slower)
- Unexpected performance changes

## Adding New Benchmarks

1. Create a new benchmark file in `crates/wgpu_playground_core/benches/`
2. Add the benchmark to `Cargo.toml`:
   ```toml
   [[bench]]
   name = "your_benchmark"
   harness = false
   ```
3. Use the Criterion template:
   ```rust
   use criterion::{black_box, criterion_group, criterion_main, Criterion};

   fn your_benchmark(c: &mut Criterion) {
       c.bench_function("benchmark_name", |b| {
           b.iter(|| {
               // Code to benchmark
               black_box(your_function())
           })
       });
   }

   criterion_group!(benches, your_benchmark);
   criterion_main!(benches);
   ```

See [benches/README.md](../crates/wgpu_playground_core/benches/README.md) for more details.

## Best Practices

### What to benchmark:
- CPU-bound operations (buffer config, validation)
- Data structure operations
- Algorithm performance
- Parsing and serialization

### What NOT to benchmark:
- GPU operations (use integration tests)
- I/O operations
- Network calls
- External dependencies

### Writing good benchmarks:
- Keep benchmarks focused and isolated
- Use `black_box()` to prevent compiler optimizations
- Avoid setup code in the benchmark loop
- Test realistic workloads
- Document what you're measuring

## Interpreting Performance Changes

When benchmarks show performance differences:

1. **Verify consistency**
   - Run benchmarks multiple times
   - Check standard deviation

2. **Understand the cause**
   - Review code changes
   - Check for algorithmic differences
   - Look for caching effects

3. **Assess impact**
   - Is it a critical path?
   - What's the real-world impact?
   - Is the tradeoff worth it?

4. **Document findings**
   - Add comments to PR
   - Update benchmark documentation
   - Consider adding regression tests

## Troubleshooting

### Benchmarks fail to compile
- Check that your code compiles with `cargo build`
- Ensure benchmark dependencies are in `[dev-dependencies]`
- Verify `harness = false` in `Cargo.toml`

### Inconsistent results
- Close other applications
- Run benchmarks multiple times
- Check for system load
- Consider increasing sample size

### CI workflow fails
- Check workflow logs in GitHub Actions
- Verify YAML syntax is correct
- Ensure benchmarks pass locally first

## Resources

- [Criterion.rs Documentation](https://bheisler.github.io/criterion.rs/book/)
- [Benchmark User Guide](https://doc.rust-lang.org/cargo/commands/cargo-bench.html)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
