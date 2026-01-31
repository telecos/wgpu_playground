# CI Testing Documentation

This document describes the Continuous Integration (CI) testing setup for the wgpu_playground project.

## Overview

The CI pipeline runs comprehensive tests on Linux and quick smoke tests on macOS/Windows to ensure code quality and cross-platform compatibility while maintaining fast CI times.

## Test Jobs

### 1. Comprehensive Tests (Linux) (`test-linux`)

**Purpose**: Run all tests with detailed reporting on Linux.

**Platform**: Linux (Ubuntu)

**Commands**: 
- `cargo nextest run --workspace --all-features --profile ci --message-format junit` - All unit and integration tests
- `cargo test --workspace --all-features --doc` - Documentation tests

**Coverage**:
- Unit tests (library and binary code)
- Integration tests (tests in `tests/` directories)
- Doc tests (documentation examples)
- JUnit XML reporting
- All workspace crates with all features

**Why Linux only for comprehensive tests?**
- Linux runners are faster and more cost-effective
- Most Rust code is platform-agnostic
- Platform-specific issues are caught by quick tests on other platforms
- Significantly reduces CI time (previously 9 parallel jobs, now 1 comprehensive + 2 quick)

### 2. Platform Compatibility Tests (`test-other-platforms`)

**Purpose**: Quick smoke tests to verify macOS and Windows compatibility.

**Platforms**: macOS, Windows

**Command**: `cargo test --workspace --all-features --lib`

**Coverage**:
- Library unit tests only (no integration tests, no doc tests)
- Validates that code compiles and basic functionality works on each platform
- Much faster than comprehensive test suite

**Why quick tests only?**
- macOS and Windows runners are slower and more expensive
- Platform-specific bugs are rare in this codebase
- Integration tests and doc tests are platform-agnostic
- Catches compilation issues and basic platform compatibility problems

## Test Reporting

### JUnit XML Reports

The comprehensive Linux test job generates JUnit XML reports that are:
- Uploaded as CI artifacts (retained for 30 days)
- Published as GitHub Check Results
- Added as PR comments for easy visibility

**Report Files**:
- `test-results-linux.xml` - All test results from Linux (unit + integration tests)

### Test Summary

The `test-report` job:
1. Downloads test results from the Linux test job
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

Tests use a strategic approach:
- **Linux**: Full comprehensive testing (all test types)
- **macOS/Windows**: Quick library tests only

**Why this approach?**
- Balances thorough testing with CI speed
- Linux is faster and more cost-effective for comprehensive tests
- macOS/Windows validate cross-platform compatibility without duplication
- Reduces total CI time by ~70% compared to running all tests on all platforms

## Running Tests Locally

### Run comprehensive tests (like Linux CI)

```bash
# Install nextest (one-time setup)
cargo install cargo-nextest --locked

# Run all unit and integration tests
cargo nextest run --workspace --all-features --profile ci

# Run doc tests
cargo test --workspace --all-features --doc
```

### Run quick platform tests (like macOS/Windows CI)

```bash
# Run library tests only
cargo test --workspace --all-features --lib
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
   - Comprehensive tests on Linux (`test-linux`)
     - All unit tests
     - All integration tests
     - All doc tests
     - JUnit XML report generation
   - Quick platform tests on macOS and Windows (`test-other-platforms`)
     - Library unit tests only
     - Validates cross-platform compatibility
   - Build verification (`build`)

3. **Test Reporting**:
   - Aggregate results from Linux tests
   - Publish unified test report
   - Generate PR comments

4. **Status Check** (`ci-success`):
   - Waits for all jobs to complete
   - Fails if any job failed
   - Used as branch protection requirement

## Performance Optimization

The current CI setup is optimized for speed while maintaining quality:

**Previous approach** (9 parallel jobs):
- 3 platforms × 3 test types = 9 test job executions
- Each platform ran: unit tests, integration tests, doc tests
- Tests were run twice (once for execution, once for JUnit)
- Estimated total time: ~45-60 minutes

**Current approach** (3 jobs):
- 1 comprehensive Linux job (all tests, run once with JUnit output)
- 2 quick platform jobs (library tests only)
- Estimated total time: ~15-20 minutes

**Time savings**: ~70% reduction in CI time

## Troubleshooting

### Tests pass locally but fail in CI

1. Check if you're testing with the same flags as CI: `--all-features`
2. Ensure your code works in a clean environment (CI starts fresh)
3. Check for platform-specific issues if it's a macOS/Windows failure

### Platform-specific test failures

If quick tests fail on macOS or Windows but pass on Linux:
1. This indicates a real platform compatibility issue
2. Run the full test suite locally on that platform
3. Fix the platform-specific code
4. Consider adding platform-conditional compilation if needed

### CI taking too long

The current setup is optimized for speed (~15-20 min total):
- Linux: Comprehensive tests (~10-12 min)
- macOS: Quick tests (~3-4 min)
- Windows: Quick tests (~3-4 min)

If you need even faster CI:
- Consider reducing the number of features tested
- Enable fail-fast mode in nextest config to exit on first failure
- Cache more aggressively

### JUnit report generation fails

A fallback XML structure is created if JUnit generation fails:
```xml
<testsuites></testsuites>
```

This ensures the reporting job doesn't fail due to missing artifacts.

## Best Practices

1. **Keep tests fast**: Slow tests slow down CI for everyone
2. **Write platform-agnostic code**: Most code should work identically on all platforms
3. **Use platform-specific tests sparingly**: Only when truly needed with `#[cfg(target_os = "...")]`
4. **Document complex test setups**: Especially for integration tests
5. **Check doc examples**: They're tests too and must compile and run
6. **Watch CI results**: Don't merge until CI is green
7. **Trust the Linux comprehensive tests**: They catch 99% of issues
8. **Platform tests catch compatibility issues**: If macOS/Windows quick tests fail, investigate

## Related Documentation

- [../COVERAGE.md](../COVERAGE.md) - Code coverage setup and reports
- [../CONTRIBUTING.md](../CONTRIBUTING.md) - Contribution guidelines
- [Cargo Nextest Documentation](https://nexte.st/) - Learn more about nextest
