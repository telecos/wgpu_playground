# Code Coverage

[![Coverage](https://codecov.io/gh/telecos/wgpu_playground/branch/main/graph/badge.svg)](https://codecov.io/gh/telecos/wgpu_playground)

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
- `.github/workflows/coverage.yml` - CI workflow for coverage

## Excluded Paths

The following paths are excluded from coverage reports:

- `crates/wgpu_playground_examples/**` - Example code
- `tests/**` - Test files
- `benches/**` - Benchmark files

## Viewing Coverage in CI

### Codecov Dashboard

The easiest way to view coverage is through the [Codecov dashboard](https://codecov.io/gh/telecos/wgpu_playground):

- View overall project coverage trends
- See coverage by file and directory
- Compare coverage between branches
- View coverage on pull requests with inline annotations

### CI Artifacts

You can also download HTML coverage reports from GitHub Actions:

1. Go to the "Actions" tab in the GitHub repository
2. Click on a workflow run
3. Scroll to "Artifacts" section
4. Download "coverage-report" artifact
5. Extract and open `index.html` in a browser

## Codecov Setup

### For Repository Maintainers

To enable Codecov uploads, the `CODECOV_TOKEN` secret must be configured in the repository:

1. Sign up at [codecov.io](https://codecov.io) (free for public repositories)
2. Add the repository to Codecov
3. Copy the upload token from the Codecov repository settings
4. Add the token to GitHub repository secrets:
   - Go to repository Settings → Secrets and variables → Actions
   - Create a new secret named `CODECOV_TOKEN`
   - Paste the token value

The coverage workflow will automatically upload results to Codecov on every push to `main` and on pull requests.

### Coverage Badge

The coverage badge is automatically updated by Codecov and displayed in the README:

```markdown
[![Coverage](https://codecov.io/gh/telecos/wgpu_playground/branch/main/graph/badge.svg)](https://codecov.io/gh/telecos/wgpu_playground)
```
