# Developer Guide

Welcome to the wgpu_playground Developer Guide! This document provides comprehensive information for developers contributing to the project, including setup, architecture, workflows, and best practices.

> **Note**: For contribution guidelines and PR process, see [CONTRIBUTING.md](CONTRIBUTING.md). This guide focuses on developer workflows and technical architecture.

## Table of Contents

- [Quick Start for Developers](#quick-start-for-developers)
- [Development Environment Setup](#development-environment-setup)
- [Project Architecture](#project-architecture)
- [Development Workflows](#development-workflows)
- [Debugging and Troubleshooting](#debugging-and-troubleshooting)
- [Testing Strategy](#testing-strategy)
- [Performance Considerations](#performance-considerations)
- [Common Development Tasks](#common-development-tasks)
- [Resources and References](#resources-and-references)

## Quick Start for Developers

### First-Time Setup

```bash
# Clone the repository
git clone https://github.com/telecos/wgpu_playground.git
cd wgpu_playground

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install required components
rustup component add rustfmt clippy

# Build the project
cargo build

# Run tests to verify setup
cargo test --workspace

# Run the application
cargo run --release
```

### Essential Commands

```bash
# Development build (faster compilation, slower runtime)
cargo build

# Release build (slower compilation, optimized runtime)
cargo build --release

# Run with logging
RUST_LOG=debug cargo run

# Format code
cargo fmt --all

# Lint code
cargo clippy --workspace --all-targets -- -D warnings

# Run all tests
cargo test --workspace

# Run specific test
cargo test test_buffer_creation

# Build and view documentation
cargo doc --workspace --all-features --no-deps --open
```

## Development Environment Setup

### Recommended IDE Setup

#### Visual Studio Code

**Extensions:**
- **rust-analyzer**: Language server for Rust (essential)
- **CodeLLDB**: Debugger for Rust
- **Even Better TOML**: TOML file support
- **Error Lens**: Inline error display
- **GitLens**: Git integration

**VS Code Settings** (`.vscode/settings.json`):
```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.cargo.features": "all",
  "editor.formatOnSave": true,
  "editor.rulers": [100],
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  }
}
```

**Launch Configuration** (`.vscode/launch.json`):
```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug wgpu_playground",
      "cargo": {
        "args": ["build", "--bin=wgpu_playground_gui", "--package=wgpu_playground_gui"],
        "filter": {
          "name": "wgpu_playground_gui",
          "kind": "bin"
        }
      },
      "args": [],
      "cwd": "${workspaceFolder}",
      "env": {
        "RUST_LOG": "debug",
        "RUST_BACKTRACE": "1"
      }
    }
  ]
}
```

#### Other IDEs

- **IntelliJ IDEA / CLion**: Install Rust plugin
- **Vim/Neovim**: Use rust-analyzer with coc.nvim or native LSP
- **Emacs**: Use rust-mode and lsp-mode with rust-analyzer

### Platform-Specific Setup

#### Linux

```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install -y \
  build-essential \
  libvulkan-dev \
  libx11-dev \
  libxcb-xfixes0-dev \
  mesa-vulkan-drivers \
  vulkan-tools

# Verify Vulkan support
vulkaninfo --summary
```

#### macOS

```bash
# Install Xcode Command Line Tools
xcode-select --install

# Verify Metal support (should be built-in)
system_profiler SPDisplaysDataType | grep Metal
```

#### Windows

```powershell
# Install Visual Studio with C++ support
# Download from: https://visualstudio.microsoft.com/

# Install Vulkan SDK (optional, for advanced development)
# Download from: https://vulkan.lunarg.com/
```

### Environment Variables

Common environment variables for development:

```bash
# Logging
export RUST_LOG=debug                    # Enable debug logging
export RUST_LOG=wgpu=trace               # Trace wgpu operations
export RUST_BACKTRACE=1                  # Enable backtraces
export RUST_BACKTRACE=full               # Full backtraces

# GPU Backend Selection
export WGPU_BACKEND=vulkan               # Use Vulkan
export WGPU_BACKEND=metal                # Use Metal (macOS)
export WGPU_BACKEND=dx12                 # Use DirectX 12 (Windows)

# WebGPU Implementation
export WEBGPU_IMPL=wgpu                  # Use wgpu (default)
export WEBGPU_IMPL=dawn                  # Use Dawn (requires --features dawn)

# Testing
export WGPU_HEADLESS=1                   # Use software rendering for tests
export CI=1                              # Simulate CI environment

# Performance
export WGPU_POWER_PREF=high-performance  # Prefer discrete GPU
export WGPU_POWER_PREF=low-power         # Prefer integrated GPU
```

## Project Architecture

### Crate Organization

The project is organized as a Cargo workspace with three main crates:

```
wgpu_playground/
├── crates/
│   ├── wgpu_playground_core/      # Core library with WebGPU abstractions
│   │   ├── src/
│   │   │   ├── lib.rs             # Library root
│   │   │   ├── device_info.rs     # GPU device information panel
│   │   │   ├── rendering.rs       # Rendering panel and APIs
│   │   │   ├── compute.rs         # Compute/ML panel and APIs
│   │   │   ├── buffer.rs          # Buffer configuration panel
│   │   │   ├── texture.rs         # Texture configuration panel
│   │   │   ├── sampler.rs         # Sampler configuration panel
│   │   │   ├── shader_editor.rs   # WGSL shader editor
│   │   │   ├── assets.rs          # Asset loading system
│   │   │   └── ...
│   │   ├── tests/                 # Integration tests
│   │   └── benches/               # Performance benchmarks
│   │
│   ├── wgpu_playground_gui/       # GUI application
│   │   ├── src/
│   │   │   ├── main.rs            # Entry point and window setup
│   │   │   └── app.rs             # Main application and tab management
│   │   └── Cargo.toml
│   │
│   └── wgpu_playground_examples/  # Standalone examples
│       ├── examples/
│       │   ├── triangle.rs        # Basic triangle rendering
│       │   ├── texture_mapping.rs # Texture operations
│       │   ├── rotating_cube.rs   # 3D rendering with transforms
│       │   └── ...
│       └── Cargo.toml
│
├── assets/                         # Static resources
│   ├── shaders/                    # WGSL shader files
│   ├── textures/                   # Texture assets
│   └── models/                     # 3D model files
│
├── docs/                           # Documentation
├── tests/                          # Additional integration tests
└── Cargo.toml                      # Workspace configuration
```

### Key Design Patterns

#### Panel Architecture

Each UI panel follows a consistent pattern:

```rust
pub struct PanelState {
    // Panel-specific state
    config: PanelConfig,
    validation_errors: Vec<String>,
    // ...
}

impl PanelState {
    pub fn new() -> Self { /* ... */ }
    
    pub fn ui(&mut self, ui: &mut egui::Ui, device: Option<&wgpu::Device>) {
        // Render UI controls
        // Validate input
        // Display errors
    }
    
    pub fn create_resource(&self, device: &wgpu::Device) -> Result<Resource, Error> {
        // Create GPU resource based on panel configuration
    }
}
```

#### Error Handling

The project uses `Result<T, E>` for fallible operations:

```rust
// Custom error types for different domains
pub enum BufferError {
    InvalidSize(String),
    InvalidUsage(String),
    CreationFailed(String),
}

// User-friendly error messages
impl Display for BufferError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BufferError::InvalidSize(msg) => write!(f, "Invalid buffer size: {}", msg),
            // ...
        }
    }
}

// Usage in code
fn create_buffer(config: &BufferConfig) -> Result<wgpu::Buffer, BufferError> {
    config.validate()?;
    // Create buffer...
}
```

#### Asset Management

Assets are loaded from the `assets/` directory:

```rust
// Load shader from file
let shader_source = include_str!("../../assets/shaders/triangle.wgsl");

// Or use the assets module for runtime loading
use wgpu_playground_core::assets;
let shader = assets::load_shader("shaders/triangle.wgsl")?;
```

### Data Flow

```
User Input (egui UI)
    ↓
Panel State Update
    ↓
Validation
    ↓
GPU Resource Creation (wgpu API)
    ↓
Command Recording
    ↓
Queue Submission
    ↓
GPU Execution
    ↓
Result Display (texture/buffer readback)
```

## Development Workflows

### Adding a New Feature

1. **Check PLAN.md**: Review planned features and create/assign GitHub issue
2. **Create Feature Branch**: `git checkout -b feature/your-feature-name`
3. **Design API**: Plan the public interface before implementation
4. **Write Tests First** (TDD approach):
   ```bash
   # Create test file
   touch crates/wgpu_playground_core/tests/your_feature_test.rs
   
   # Write failing test
   # Implement feature
   # Verify test passes
   ```
5. **Implement Feature**: Follow coding standards (see CONTRIBUTING.md)
6. **Add Documentation**: Doc comments for public APIs
7. **Update Examples**: Add example if applicable
8. **Run Quality Checks**:
   ```bash
   cargo fmt --all
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace
   ```
9. **Create PR**: Follow PR guidelines in CONTRIBUTING.md

### Working with WebGPU Resources

#### Creating a Buffer

```rust
use wgpu::util::DeviceExt;

let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    label: Some("Vertex Buffer"),
    contents: bytemuck::cast_slice(&vertices),
    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
});
```

#### Creating a Texture

```rust
let texture = device.create_texture(&wgpu::TextureDescriptor {
    label: Some("Render Texture"),
    size: wgpu::Extent3d {
        width: 512,
        height: 512,
        depth_or_array_layers: 1,
    },
    mip_level_count: 1,
    sample_count: 1,
    dimension: wgpu::TextureDimension::D2,
    format: wgpu::TextureFormat::Rgba8UnormSrgb,
    usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
    view_formats: &[],
});
```

#### Loading and Compiling a Shader

```rust
let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
    label: Some("Shader"),
    source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
});
```

### Adding a New UI Panel

1. **Create Panel Module** in `crates/wgpu_playground_core/src/`:
   ```rust
   // my_panel.rs
   pub struct MyPanelState {
       // Panel state
   }
   
   impl MyPanelState {
       pub fn new() -> Self { /* ... */ }
       
       pub fn ui(&mut self, ui: &mut egui::Ui) {
           // UI implementation
       }
   }
   ```

2. **Add to Core Library** (`lib.rs`):
   ```rust
   pub mod my_panel;
   pub use my_panel::MyPanelState;
   ```

3. **Integrate in GUI App** (`crates/wgpu_playground_gui/src/app.rs`):
   ```rust
   pub struct App {
       my_panel: MyPanelState,
       // ...
   }
   
   // In the tab rendering code:
   if ui.selectable_label(self.active_tab == Tab::MyPanel, "My Panel").clicked() {
       self.active_tab = Tab::MyPanel;
   }
   
   Tab::MyPanel => {
       self.my_panel.ui(ui);
   }
   ```

### Adding a New Example

1. **Create Example File** in `crates/wgpu_playground_examples/examples/`:
   ```bash
   touch crates/wgpu_playground_examples/examples/my_example.rs
   ```

2. **Implement Example**:
   ```rust
   fn main() {
       pollster::block_on(run_example());
   }
   
   async fn run_example() {
       println!("=== My Example ===");
       
       // Setup GPU
       let instance = wgpu::Instance::default();
       let adapter = instance.request_adapter(&Default::default()).await.unwrap();
       let (device, queue) = adapter.request_device(&Default::default(), None).await.unwrap();
       
       // Example logic
       // ...
       
       println!("=== Example Complete ===");
   }
   ```

3. **Run Example**:
   ```bash
   cargo run --package wgpu_playground_examples --example my_example
   ```

4. **Document in README.md**: Add example to the examples section

## Debugging and Troubleshooting

### Enabling Debug Logging

```bash
# All debug output
RUST_LOG=debug cargo run

# Module-specific logging
RUST_LOG=wgpu_playground_core=trace cargo run

# wgpu internal logging
RUST_LOG=wgpu_core=debug,wgpu_hal=debug cargo run

# Multiple modules
RUST_LOG=wgpu_playground_core=debug,wgpu=trace cargo run
```

### Using the Debugger

#### VS Code with CodeLLDB

1. Set breakpoints in the editor (click left of line numbers)
2. Press `F5` or use "Run > Start Debugging"
3. Use debug controls to step through code

#### Command Line with LLDB/GDB

```bash
# Build with debug symbols
cargo build

# Run with lldb
lldb target/debug/wgpu_playground_gui

# Set breakpoint
(lldb) breakpoint set --name main
(lldb) run

# Or with rust-lldb wrapper
rust-lldb target/debug/wgpu_playground_gui
```

### GPU Debugging Tools

#### RenderDoc (Cross-platform)

```bash
# Install RenderDoc
# Linux: sudo apt-get install renderdoc
# Windows/macOS: Download from https://renderdoc.org/

# Run application through RenderDoc
renderdoccmd capture --wait-for-exit target/release/wgpu_playground_gui
```

#### Platform-Specific Tools

- **Windows**: PIX for Windows, NVIDIA Nsight Graphics
- **macOS**: Xcode Metal Debugger
- **Linux**: NVIDIA Nsight Graphics, Intel GPA

### Common Issues and Solutions

#### Issue: "No suitable adapter found"

**Cause**: No compatible GPU or graphics driver issue

**Solution**:
```bash
# Check available backends
RUST_LOG=wgpu=debug cargo run 2>&1 | grep -i adapter

# Try different backend
WGPU_BACKEND=vulkan cargo run  # Or metal, dx12, gl

# Use software rendering (fallback)
WGPU_BACKEND=gl cargo run
```

#### Issue: Shader compilation errors

**Cause**: Invalid WGSL syntax or unsupported features

**Solution**:
```rust
// Enable detailed shader error reporting
let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
    label: Some("Shader"),
    source: wgpu::ShaderSource::Wgsl(shader_source.into()),
});

// Check for errors in logs
// RUST_LOG=wgpu=debug cargo run
```

#### Issue: "Surface is outdated" or rendering glitches

**Cause**: Window resize or GPU state mismatch

**Solution**:
```rust
// Properly handle window resize
fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    if new_size.width > 0 && new_size.height > 0 {
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }
}
```

#### Issue: Tests fail with "No adapter found"

**Cause**: Running GPU tests in headless environment

**Solution**:
```bash
# Use software rendering for tests
WGPU_HEADLESS=1 cargo test
CI=1 cargo test

# Or skip GPU tests
cargo test --lib  # Only library tests
```

### Performance Profiling

#### CPU Profiling

```bash
# Install cargo-flamegraph
cargo install flamegraph

# Generate flamegraph
cargo flamegraph --bin wgpu_playground_gui

# View output
open flamegraph.svg
```

#### GPU Profiling

Use platform-specific tools:
- **RenderDoc**: Frame captures and GPU timings
- **Tracy**: Real-time profiling
- **NVIDIA Nsight**: Detailed GPU metrics

#### Memory Profiling

```bash
# Install valgrind (Linux)
sudo apt-get install valgrind

# Run memory check
valgrind --leak-check=full target/debug/wgpu_playground_gui

# Or use heaptrack
heaptrack target/debug/wgpu_playground_gui
heaptrack_gui heaptrack.wgpu_playground_gui.*.gz
```

## Testing Strategy

### Test Organization

```
crates/wgpu_playground_core/
├── src/
│   ├── buffer.rs
│   │   #[cfg(test)]
│   │   mod tests { /* unit tests */ }
│   └── ...
├── tests/
│   ├── common/           # Shared test utilities
│   │   └── mod.rs        # create_test_device(), etc.
│   ├── buffer_integration_test.rs
│   └── ...
└── benches/             # Performance benchmarks
    └── buffer_operations.rs
```

### Writing Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_config_validation() {
        let config = BufferConfig {
            size: 0,  // Invalid
            usage: wgpu::BufferUsages::VERTEX,
            label: None,
            mapped_at_creation: false,
        };
        
        assert!(config.validate().is_err());
    }
}
```

### Writing Integration Tests

```rust
// tests/my_integration_test.rs
mod common;
use common::create_test_device;

#[test]
fn test_buffer_creation() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };
        
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Test Buffer"),
            size: 256,
            usage: wgpu::BufferUsages::VERTEX,
            mapped_at_creation: false,
        });
        
        // Test assertions
        assert_eq!(buffer.size(), 256);
    });
}
```

### Running Tests

```bash
# All tests
cargo test --workspace

# Specific test
cargo test test_buffer_creation

# With output
cargo test -- --nocapture

# Only unit tests
cargo test --lib

# Only integration tests
cargo test --test '*'

# Specific package
cargo test -p wgpu_playground_core

# Headless mode (software rendering)
WGPU_HEADLESS=1 cargo test --workspace
```

### Benchmarking

```bash
# Run all benchmarks
cargo bench

# Specific benchmark
cargo bench --bench buffer_operations

# View results
open target/criterion/report/index.html
```

See [docs/BENCHMARKING.md](docs/BENCHMARKING.md) for details.

### Visual Regression Testing

```bash
# Run visual regression tests
cargo test visual_regression

# Update reference images
UPDATE_VISUAL_REFERENCES=1 cargo test visual_regression
```

See [docs/VISUAL_REGRESSION_TESTING.md](docs/VISUAL_REGRESSION_TESTING.md) for details.

## Performance Considerations

### GPU Resource Management

- **Reuse Resources**: Don't create new buffers/textures every frame
- **Proper Buffer Usage**: Use appropriate usage flags
  ```rust
  // Good: Specific usage
  wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
  
  // Avoid: Overly broad usage
  wgpu::BufferUsages::all()
  ```
- **Texture Mipmaps**: Generate mipmaps for sampled textures
- **Buffer Alignment**: Follow platform alignment requirements

### Render Pipeline Optimization

- **Minimize State Changes**: Batch draws with same pipeline
- **Instance Drawing**: Use instancing for repeated geometry
- **Frustum Culling**: Don't render off-screen objects
- **Level of Detail**: Use appropriate detail levels

### Shader Best Practices

- **Avoid Dynamic Branching**: Minimize `if` statements in shaders
- **Use Built-in Functions**: Prefer `dot()`, `normalize()`, etc.
- **Minimize Register Pressure**: Reduce temporary variables
- **Uniform Buffer Layout**: Follow std140/std430 layout rules

### CPU-Side Performance

- **Minimize Allocations**: Reuse vectors and buffers
- **Efficient Data Structures**: Use appropriate collections
- **Parallelization**: Use rayon for CPU-heavy tasks
- **Profile First**: Use `cargo bench` to identify bottlenecks

## Common Development Tasks

### Updating Dependencies

```bash
# Check for outdated dependencies
cargo outdated

# Update to latest compatible versions
cargo update

# Update specific dependency
cargo update -p wgpu

# Edit Cargo.toml for major updates
# Then run cargo update
```

### Running Linters

```bash
# Format code
cargo fmt --all

# Check formatting without modifying
cargo fmt --all -- --check

# Run clippy
cargo clippy --workspace --all-targets -- -D warnings

# Fix auto-fixable clippy issues
cargo clippy --fix --workspace --all-targets
```

### Generating Documentation

```bash
# Build docs
cargo doc --workspace --all-features --no-deps

# Build and open docs
cargo doc --workspace --all-features --no-deps --open

# Include private items
cargo doc --workspace --all-features --no-deps --document-private-items --open
```

### Working with Git

```bash
# Create feature branch
git checkout -b feature/my-feature

# Commit changes
git add .
git commit -m "feat: add new feature"

# Keep branch updated
git fetch origin
git rebase origin/main

# Push branch
git push origin feature/my-feature
```

### Security Auditing

```bash
# Run security audit
cargo audit

# Check dependencies
cargo deny check

# Both run automatically in CI
```

## Resources and References

### Official Documentation

- **wgpu**: https://docs.rs/wgpu/
- **WebGPU Specification**: https://www.w3.org/TR/webgpu/
- **WGSL Specification**: https://www.w3.org/TR/WGSL/
- **egui**: https://docs.rs/egui/
- **Rust Book**: https://doc.rust-lang.org/book/

### Learning Resources

- **Learn wgpu**: https://sotrh.github.io/learn-wgpu/
- **WebGPU Fundamentals**: https://webgpufundamentals.org/
- **Rust Graphics**: https://wiki.alopex.li/RustGraphics
- **GPU Gems**: https://developer.nvidia.com/gpugems/

### Project Documentation

- [CONTRIBUTING.md](CONTRIBUTING.md) - Contribution guidelines
- [README.md](README.md) - Project overview
- [PLAN.md](PLAN.md) - Development roadmap
- [docs/USER_GUIDE.md](docs/USER_GUIDE.md) - User documentation
- [docs/architecture.md](docs/architecture.md) - Detailed architecture
- [docs/VISUAL_REGRESSION_TESTING.md](docs/VISUAL_REGRESSION_TESTING.md) - Visual testing
- [docs/BENCHMARKING.md](docs/BENCHMARKING.md) - Performance benchmarking

### Community

- **GitHub Issues**: https://github.com/telecos/wgpu_playground/issues
- **wgpu Matrix**: https://matrix.to/#/#wgpu:matrix.org
- **Rust Community**: https://www.rust-lang.org/community

### Tools

- **RenderDoc**: https://renderdoc.org/ - Graphics debugger
- **rust-analyzer**: https://rust-analyzer.github.io/ - LSP server
- **cargo-watch**: Auto-rebuild on file changes
  ```bash
  cargo install cargo-watch
  cargo watch -x build
  ```
- **cargo-expand**: Expand macros
  ```bash
  cargo install cargo-expand
  cargo expand module::path
  ```

## Getting Help

If you encounter issues or have questions:

1. **Check Documentation**: Review this guide and CONTRIBUTING.md
2. **Search Issues**: Look for similar problems in GitHub issues
3. **Enable Debug Logging**: `RUST_LOG=debug cargo run` for details
4. **Ask in PR Comments**: When working on a specific PR
5. **Open Discussion Issue**: For design questions or proposals

---

**Happy coding! 🚀**

For contribution workflow and PR guidelines, see [CONTRIBUTING.md](CONTRIBUTING.md).
