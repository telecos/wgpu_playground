# GitHub Actions Workflows

This directory contains automated workflows for the wgpu_playground project.

## Workflows Overview

### CI (`ci.yml`)
Main continuous integration workflow that runs on all pull requests and main branch pushes.
- Builds and tests the project
- Runs clippy linting
- Validates code quality

### Documentation & WASM Demo (`docs.yml`)
Builds and deploys documentation and WebAssembly demo to GitHub Pages.
- Builds rustdoc API documentation
- Compiles WASM demo
- Deploys to https://telecos.github.io/wgpu_playground/

**Prerequisites**: Requires GitHub Pages to be enabled in repository settings.

### WASM Builds (`wasm-builds.yml`)
Tests WebAssembly compilation and validates WASM bundles.
- Builds WASM modules
- Runs WASM tests in headless browser
- Validates bundle size and structure

### Coverage (`coverage.yml`)
Generates and uploads code coverage reports to Codecov.
- Uses cargo-tarpaulin for coverage collection
- Uploads to codecov.io

### Security Audit (`security-audit.yml`)
Scans dependencies for known security vulnerabilities.
- Uses cargo-audit (RustSec Advisory Database)
- Creates GitHub issues for vulnerabilities

### Security Deny (`security-deny.yml`)
Enforces security and licensing policies.
- Checks for banned/unmaintained dependencies
- Validates license compatibility

### Native Builds (`native-builds.yml`)
Tests builds across multiple platforms (Linux, macOS, Windows).

### Benchmarks (`benchmarks.yml`)
Runs performance benchmarks and tracks results over time.

### Dawn CI (`dawn-ci.yml`)
Tests against Google Dawn WebGPU implementation (when available).

### Release (`release.yml`)
Automated release workflow for creating new versions.

## Setup Requirements

### GitHub Pages Deployment

For the `docs.yml` workflow to successfully deploy, GitHub Pages must be manually enabled:

1. Go to repository **Settings**
2. Navigate to **Pages** (left sidebar)
3. Under **Source**, select **GitHub Actions**
4. Click **Save**

Without this step, the deployment will fail with a 404 error.

See [WASM_DEMO_DEPLOYMENT.md](../../docs/WASM_DEMO_DEPLOYMENT.md) for detailed deployment documentation.

### Secrets Configuration

Some workflows may require repository secrets:

- `CODECOV_TOKEN`: For coverage uploads (optional, public repos work without it)
- Any custom deployment tokens (if configured)

### Permissions

The workflows are configured with minimal required permissions following the principle of least privilege. Each workflow specifies its required permissions in the `permissions:` section.

## Local Testing

You can test workflow changes locally using [act](https://github.com/nektos/act):

```bash
# Install act
brew install act  # macOS
# or download from https://github.com/nektos/act

# Run a specific workflow
act -W .github/workflows/ci.yml

# Run a specific job
act -W .github/workflows/ci.yml -j test
```

Note: Some workflows (like deploy-pages) cannot be fully tested locally due to GitHub-specific APIs.

## Troubleshooting

### Deployment Fails with 404
- **Cause**: GitHub Pages not enabled
- **Solution**: Enable Pages in repository settings (see Setup Requirements above)

### Permission Errors
- **Cause**: Insufficient workflow permissions
- **Solution**: Check `permissions:` section in workflow file matches requirements

### Workflow Not Running
- **Cause**: Workflow file syntax error or branch protection
- **Solution**: Validate YAML syntax and check branch protection rules

For more help, see:
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Workflow Syntax](https://docs.github.com/en/actions/reference/workflow-syntax-for-github-actions)
