# WASM Demo Deployment

This document describes the WebGPU demo deployment setup for wgpu_playground.

## Overview

The wgpu_playground project includes a WebAssembly demo that showcases WebGPU capabilities directly in the browser. The demo is automatically built and deployed to GitHub Pages on every push to the main branch.

## Prerequisites

### Enabling GitHub Pages (Required)

**IMPORTANT**: Before the automated deployment can work, GitHub Pages must be enabled in the repository settings. This is a one-time setup step that requires repository admin access.

#### Setup Steps:

1. Go to your repository on GitHub
2. Click on **Settings** tab
3. Navigate to **Pages** in the left sidebar
4. Under **Source**, select:
   - Source: **GitHub Actions** (not "Deploy from a branch")
5. Click **Save**

After completing these steps, the workflow will be able to deploy successfully.

#### Verification:

You can verify GitHub Pages is properly configured by checking:
- The Pages section shows "Your site is ready to be published"
- The workflow runs complete successfully without 404 errors
- The deployment URL is accessible at `https://<username>.github.io/<repository>/`

## Live Demo

🚀 **[Try the Demo](https://telecos.github.io/wgpu_playground/demo/)**

The demo is hosted at: https://telecos.github.io/wgpu_playground/demo/

## Architecture

The deployment consists of:

1. **WASM Module** (`wgpu_playground_core`) - Core WebGPU functionality compiled to WebAssembly
2. **HTML Demo Page** (`web/index.html`) - Interactive web interface that loads and runs the WASM module
3. **Documentation** - API documentation at the root level
4. **Landing Page** - Main entry point that links to both demo and docs

### Directory Structure

```
GitHub Pages Root (https://telecos.github.io/wgpu_playground/)
├── index.html              # Landing page
├── demo/                   # WASM demo
│   ├── index.html         # Demo page
│   └── pkg/               # WASM package
│       ├── wgpu_playground_core_bg.wasm
│       ├── wgpu_playground_core.js
│       └── wgpu_playground_core.d.ts
└── wgpu_playground_core/  # API documentation
    └── ...
```

## Deployment Workflow

The deployment is handled by the `.github/workflows/docs.yml` workflow, which:

1. **Builds Documentation** - Generates rustdoc API documentation
2. **Builds WASM Demo** - Compiles the core crate to WebAssembly using wasm-pack
3. **Combines Artifacts** - Merges docs and demo into a single deployment
4. **Deploys to GitHub Pages** - Publishes to https://telecos.github.io/wgpu_playground/

### Workflow Jobs

#### build-docs
- Builds API documentation with `cargo doc`
- Creates documentation artifacts

#### build-wasm-demo
- Installs Rust toolchain with `wasm32-unknown-unknown` target
- Installs `wasm-pack` for building WebAssembly
- Builds the core crate: `wasm-pack build --target web --release`
- Prepares demo artifacts with HTML page and WASM package

#### prepare-deployment
- Downloads both documentation and WASM demo artifacts
- Creates a combined deployment directory
- Generates a landing page with links to demo and docs

#### deploy-pages
- Deploys the combined artifacts to GitHub Pages
- Only runs on main branch pushes

## Browser Requirements

The demo requires a browser with WebGPU support:

| Browser | Minimum Version | Status |
|---------|----------------|--------|
| Chrome | 113+ | ✅ Full support |
| Edge | 113+ | ✅ Full support |
| Safari | Technology Preview | 🟡 Requires enabling WebGPU |
| Firefox | Nightly | 🟡 Experimental support |

### Enabling WebGPU

**Chrome/Edge**: WebGPU is enabled by default in version 113+

**Safari**: 
1. Open Safari Technology Preview
2. Go to Develop → Experimental Features
3. Enable "WebGPU" and "WebGPU via Metal"

**Firefox Nightly**:
1. Open about:config
2. Set `dom.webgpu.enabled` to `true`

## Local Development

To test the demo locally:

### 1. Build the WASM module

```bash
cd crates/wgpu_playground_core
wasm-pack build --target web --release
```

### 2. Prepare the demo directory

```bash
cd ../../web
cp -r ../crates/wgpu_playground_core/pkg .
```

### 3. Start a local HTTP server

```bash
# Using Python
python3 -m http.server 8000

# Or using Node.js
npx http-server -p 8000

# Or using Rust
cargo install simple-http-server
simple-http-server -p 8000
```

### 4. Open in browser

Navigate to http://localhost:8000

**Note**: You must use an HTTP server - opening the HTML file directly (`file://`) will not work due to CORS restrictions on WASM modules.

## Customizing the Demo

### Updating the HTML Page

Edit `web/index.html` to customize:
- Page styling and layout
- Loading behavior
- Error handling
- Integration with the WASM module

### Adding Custom Domain

To configure a custom domain for the demo:

1. Create a `CNAME` file in the `web/` directory:
   ```
   demo.yourproject.com
   ```

2. Update the workflow to include the CNAME file in the deployment:
   ```yaml
   - name: Add custom domain
     run: |
       echo "demo.yourproject.com" > deployment/CNAME
   ```

3. Configure your DNS provider to point to GitHub Pages:
   ```
   CNAME: youruser.github.io
   ```

4. Enable HTTPS in GitHub repository settings under Pages

## Troubleshooting

### Deployment Fails with 404 Error

**Problem**: GitHub Actions deployment step fails with "Not Found" or "Failed to create deployment (status: 404)"

**Error Message**:
```
Error: Failed to create deployment (status: 404)
Ensure GitHub Pages has been enabled: https://github.com/<owner>/<repo>/settings/pages
```

**Solutions**:
1. **Enable GitHub Pages** (most common cause):
   - Go to repository Settings → Pages
   - Under "Source", select **GitHub Actions**
   - Click Save
   - Wait a few minutes for GitHub to process the change
   - Re-run the failed workflow

2. **Verify Permissions**:
   - Ensure the workflow has correct permissions (already configured in `.github/workflows/docs.yml`)
   - Check that Actions have permission to deploy to Pages in repository settings

3. **Check Repository Settings**:
   - Ensure Pages is not disabled in repository settings
   - Verify the repository is public (or has GitHub Pro/Enterprise for private repos)

### Demo Not Loading

**Problem**: Demo page loads but shows loading spinner indefinitely

**Solutions**:
1. Check browser console for JavaScript errors
2. Verify WASM files are being served correctly (check Network tab)
3. Ensure your browser supports WebGPU
4. Try clearing browser cache

### WebGPU Not Supported Error

**Problem**: "WebGPU is not supported in your browser"

**Solutions**:
1. Update browser to latest version
2. Use Chrome 113+ or Edge 113+ for best compatibility
3. Enable WebGPU in experimental features (Safari/Firefox)

### CORS Errors

**Problem**: Cannot load WASM module due to CORS

**Solutions**:
1. Must use HTTP server, not file:// protocol
2. Ensure server sends correct MIME types for .wasm files
3. Check that server doesn't block WASM with CSP headers

### Build Failures

**Problem**: WASM build fails in CI

**Solutions**:
1. Check that wasm-pack version is compatible
2. Verify wasm32-unknown-unknown target is installed
3. Review build logs for missing dependencies
4. Ensure Cargo.toml has correct WASM dependencies

## Performance Optimization

### WASM File Size

The workflow includes monitoring for WASM file size. If the WASM module exceeds 10MB, consider:

1. **Enable wasm-opt**: In `Cargo.toml`:
   ```toml
   [package.metadata.wasm-pack.profile.release]
   wasm-opt = ["-O4"]
   ```

2. **Remove unused features**: Only include necessary wgpu features

3. **Use release profile optimizations**: In `Cargo.toml`:
   ```toml
   [profile.release]
   opt-level = "z"
   lto = true
   ```

### Loading Performance

To improve demo loading time:

1. Enable WASM streaming compilation in the HTML
2. Add loading progress indicators
3. Lazy load non-critical resources
4. Use CDN for static assets

## Security Considerations

### Content Security Policy

The demo page should include appropriate CSP headers:

```html
<meta http-equiv="Content-Security-Policy" 
      content="default-src 'self'; 
               script-src 'self' 'wasm-eval'; 
               style-src 'self' 'unsafe-inline';">
```

### HTTPS Enforcement

GitHub Pages automatically enforces HTTPS, which is required for WebGPU.

## Monitoring

### Deployment Status

Check deployment status at:
- GitHub Actions: https://github.com/telecos/wgpu_playground/actions
- GitHub Pages Deployments: https://github.com/telecos/wgpu_playground/deployments

### Analytics

To add analytics to track demo usage:

1. Add analytics script to `web/index.html`
2. Track key events:
   - Demo page loads
   - WASM initialization success/failure
   - WebGPU feature usage
   - Browser compatibility issues

## Related Documentation

- [WASM Testing](WASM_TESTING.md) - Testing WebAssembly builds
- [CI Testing](CI_TESTING.md) - Continuous integration setup
- [User Guide](USER_GUIDE.md) - End-user documentation
- [Quick Start](QUICK_START.md) - Getting started guide
