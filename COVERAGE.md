# Code Coverage

This project uses [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) for code coverage reporting.

## Running Coverage Locally

### Prerequisites

Install `cargo-llvm-cov`:

```bash
cargo install cargo-llvm-cov
```

### Generate Coverage Report

#### Using the Convenience Script

For ease of use, you can use the provided script:

```bash
# Generate HTML report (default)
./scripts/coverage.sh

# Generate LCOV report
./scripts/coverage.sh lcov

# Display summary in terminal
./scripts/coverage.sh summary
```

#### Manual Commands

Generate an HTML coverage report:

```bash
cargo llvm-cov --all-features --workspace --html
```

The HTML report will be available at `target/llvm-cov/html/index.html`.

Generate coverage in LCOV format:

```bash
cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
```

Display coverage summary in terminal:

```bash
cargo llvm-cov --all-features --workspace --summary-only
```

## Coverage Thresholds

The project aims for the following coverage targets:

- **Overall Project Coverage**: 70% (with 2% threshold)
- **New Code Coverage**: 60% (with 5% threshold)

These thresholds are configured in `.codecov.yml`.

## CI Integration

Code coverage is automatically generated and reported for:

- All pushes to the `main` branch
- All pull requests

Coverage reports are:

1. Uploaded to Codecov (if `CODECOV_TOKEN` is configured)
2. Available as CI artifacts (HTML report)
3. Displayed in the CI logs (summary)

## Configuration Files

- `.codecov.yml` - Codecov configuration and thresholds
- `.cargo/config.toml` - Cargo configuration for coverage
- `.github/workflows/coverage.yml` - CI workflow for coverage

## Excluded Paths

The following paths are excluded from coverage reports:

- `crates/wgpu_playground_examples/**` - Example code
- `tests/**` - Test files
- `benches/**` - Benchmark files

## Viewing Coverage in CI

1. Go to the "Actions" tab in the GitHub repository
2. Click on a workflow run
3. Scroll to "Artifacts" section
4. Download "coverage-report" artifact
5. Extract and open `index.html` in a browser

Alternatively, view coverage on [Codecov](https://codecov.io) (if configured).
