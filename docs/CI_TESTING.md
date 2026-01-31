# CI Testing Documentation

This document describes the Continuous Integration (CI) testing setup for the wgpu_playground project.

## Overview

The CI pipeline runs comprehensive tests on every push and pull request to ensure code quality and prevent regressions. Tests are executed across multiple platforms and test types.

## Test Jobs

### 1. Unit Tests (`unit-tests`)

**Purpose**: Tests individual functions and modules within the source code.

**Platforms**: Linux (Ubuntu), macOS, Windows

**Command**: `cargo nextest run --workspace --all-features --lib --bins --profile ci`

**Coverage**:
- Library code (`--lib`)
- Binary targets (`--bins`)
- All workspace crates

### 2. Integration Tests (`integration-tests`)

**Purpose**: Tests interactions between different modules and components.

**Platforms**: Linux (Ubuntu), macOS, Windows

**Command**: `cargo nextest run --workspace --all-features --tests --profile ci`

**Coverage**:
- All integration tests in `tests/` directories
- Full workspace coverage
- All features enabled

### 3. Doc Tests (`doc-tests`)

**Purpose**: Validates code examples in documentation comments.

**Platforms**: Linux (Ubuntu), macOS, Windows

**Command**: `cargo test --workspace --all-features --doc`

**Coverage**:
- All documentation examples (`///` and `//!` comments with code blocks)
- Ensures documentation stays up-to-date with code

## Test Reporting

### JUnit XML Reports

Each test job generates JUnit XML reports that are:
- Uploaded as CI artifacts (retained for 30 days)
- Aggregated in the `test-report` job
- Published as GitHub Check Results
- Added as PR comments for easy visibility

**Report Files**:
- `test-results-unit-<platform>.xml` - Unit test results
- `test-results-integration-<platform>.xml` - Integration test results

### Test Summary

The `test-report` job:
1. Downloads all test results from all platforms
2. Publishes unified test results using `EnricoMi/publish-unit-test-result-action`
3. Creates a test summary in GitHub Step Summary
4. Adds PR comments comparing results to previous commits

## Failure Notifications

Test failures are reported through multiple channels:

1. **GitHub Actions UI**: Failed jobs are clearly marked
2. **PR Comments**: Test results are automatically commented on PRs
3. **Check Results**: Detailed test results appear in the "Checks" tab
4. **Error Annotations**: Failed tests create file annotations in the GitHub UI
5. **CI Status Check**: The `ci-success` job fails if any test job fails

## Configuration

### Nextest Configuration

Test behavior is configured in `.config/nextest.toml`:

**Default Profile**:
- Runs tests with all available CPU cores
- Shows output for failed tests only
- Retries failed tests once (handles flaky tests)

**CI Profile** (`--profile ci`):
- Optimized for CI environments
- Fail-fast disabled (all tests run even after failures)
- 60-second slow test timeout
- Configured for JUnit report generation

### Matrix Strategy

Tests run on a matrix of platforms:
```yaml
matrix:
  os: [ubuntu-latest, macos-latest, windows-latest]
```

**fail-fast: false** ensures all platforms complete even if one fails.

## Running Tests Locally

### Run all tests (like CI)

```bash
# Install nextest (one-time setup)
cargo install cargo-nextest --locked

# Run unit tests
cargo nextest run --workspace --all-features --lib --bins --profile ci

# Run integration tests
cargo nextest run --workspace --all-features --tests --profile ci

# Run doc tests
cargo test --workspace --all-features --doc
```

### Run tests for a specific crate

```bash
cargo nextest run -p wgpu_playground_core --all-features
```

### Generate JUnit report locally

```bash
cargo nextest run --workspace --all-features --message-format junit > test-results.xml
```

## CI Pipeline Flow

1. **Fast Checks** (run first):
   - Format check (`fmt`)
   - Clippy lints (`clippy`)

2. **Parallel Test Execution**:
   - Unit tests on all platforms
   - Integration tests on all platforms
   - Doc tests on all platforms
   - Build verification

3. **Test Reporting**:
   - Aggregate results from all platforms
   - Publish unified test report
   - Generate PR comments

4. **Status Check** (`ci-success`):
   - Waits for all jobs to complete
   - Fails if any job failed
   - Used as branch protection requirement

## Troubleshooting

### Test failures only on specific platforms

1. Check the platform-specific test result artifact
2. Review the test logs in the failed job
3. Run tests locally on that platform if possible

### Flaky tests

Tests are automatically retried once. If a test fails intermittently:
1. Check if it's a timing issue
2. Add appropriate waits or synchronization
3. Consider marking truly non-deterministic tests as `#[ignore]`

### JUnit report generation fails

A fallback XML structure is created if JUnit generation fails:
```xml
<testsuites></testsuites>
```

This ensures the reporting job doesn't fail due to missing artifacts.

## Best Practices

1. **Keep tests fast**: Slow tests slow down CI for everyone
2. **Avoid platform-specific tests**: Use `#[cfg(target_os = "...")]` when necessary
3. **Document complex test setups**: Especially for integration tests
4. **Check doc examples**: They're tests too and must compile and run
5. **Watch CI results**: Don't merge until CI is green

## Related Documentation

- [../COVERAGE.md](../COVERAGE.md) - Code coverage setup and reports
- [../CONTRIBUTING.md](../CONTRIBUTING.md) - Contribution guidelines
- [Cargo Nextest Documentation](https://nexte.st/) - Learn more about nextest
