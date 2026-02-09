# Developer Guide

Welcome to the wgpu_playground Developer Guide! This document provides comprehensive information for developers contributing to the project.

## Table of Contents

- [Quick Start](#quick-start)
- [Development Environment](#development-environment)
- [Project Structure](#project-structure)
- [Development Workflow](#development-workflow)
- [Testing](#testing)
- [Debugging](#debugging)
- [Code Style](#code-style)
- [Common Tasks](#common-tasks)

## Quick Start

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
# Development build
cargo build

# Release build
cargo build --release

# Run with logging
RUST_LOG=debug cargo run

# Format code
cargo fmt --all

# Lint code
cargo clippy --workspace --all-targets

# Run all tests
cargo test --workspace

# Build documentation
cargo doc --workspace --all-features --no-deps --open
```

## Development Environment

### Recommended: VS Code

**Extensions:**
- **rust-analyzer**: Language server (essential)
- **CodeLLDB**: Debugger for Rust
- **Even Better TOML**: TOML file support
- **Error Lens**: Inline error display

**VS Code Settings** (`.vscode/settings.json`):
```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.cargo.features": "all",
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  }
}
```

### Debugging

A `.vscode/launch.json` is included with configurations for:
- **Debug wgpu_playground**: Debug the main GUI application
- **Debug wgpu_playground (Release)**: Debug with release optimizations
- **Debug Unit Tests**: Debug tests in wgpu_playground_core

## Project Structure

```
wgpu_playground/
├── crates/
│   ├── wgpu_playground_core/     # Core library
│   │   ├── src/                  # Source code
│   │   ├── benches/              # Benchmarks
│   │   └── tests/                # Integration tests
│   ├── wgpu_playground_gui/      # GUI application
│   │   └── src/                  # main.rs, app.rs
│   └── wgpu_playground_examples/ # Standalone examples
│       └── examples/             # Example programs
├── assets/                       # Shaders, textures, models
├── docs/                         # Documentation
├── tests/                        # Workspace-level tests
├── web/                          # WASM build files
├── Cargo.toml                    # Workspace configuration
├── README.md                     # Project overview
├── CONTRIBUTING.md               # Contribution guidelines
├── TASKS.md                      # Task tracking
└── PLAN.md                       # Roadmap
```

## Development Workflow

### Adding a New Panel

1. Create a new module in `crates/wgpu_playground_core/src/`:
   ```rust
   // my_panel.rs
   pub struct MyPanel {
       // Panel state
   }

   impl MyPanel {
       pub fn new() -> Self {
           Self { /* ... */ }
       }

       pub fn ui(&mut self, ui: &mut egui::Ui) {
           // Panel UI code
       }
   }
   ```

2. Export in `lib.rs`:
   ```rust
   mod my_panel;
   pub use my_panel::MyPanel;
   ```

3. Add to `PlaygroundApp` in `crates/wgpu_playground_gui/src/app.rs`:
   ```rust
   // Add to Tab enum
   pub enum Tab {
       // ...
       MyPanel,
   }

   // Add panel instance
   my_panel: MyPanel,

   // Add routing
   Tab::MyPanel => self.my_panel.ui(ui),
   ```

### Adding API Tracking

When exercising WebGPU APIs, record them for coverage tracking:

```rust
use crate::api_coverage::{ApiCategory, ApiCoverageTracker};

fn create_buffer(&self, device: &Device) {
    let tracker = ApiCoverageTracker::global();
    tracker.record(ApiCategory::Buffer, "create_buffer");
    
    let buffer = device.create_buffer(&BufferDescriptor {
        // ...
    });
}
```

### Adding Examples

1. Create example in `crates/wgpu_playground_examples/examples/`:
   ```rust
   // my_example.rs
   fn main() {
       // Example code
   }
   ```

2. Run with: `cargo run --example my_example`

## Testing

### Running Tests

```bash
# All tests
cargo test --workspace

# Specific crate
cargo test -p wgpu_playground_core

# Specific test
cargo test test_buffer_creation

# With output
cargo test -- --nocapture
```

### Test Categories

- **Unit tests**: In each module with `#[cfg(test)]`
- **Integration tests**: In `crates/*/tests/`
- **Examples**: Validate example programs compile and run

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Test code
    }
}
```

## Debugging

### Logging

```bash
# General debug
RUST_LOG=debug cargo run

# wgpu-specific
RUST_LOG=wgpu=debug cargo run

# Specific module
RUST_LOG=wgpu_playground_core::rendering=trace cargo run
```

### Common Issues

| Issue | Solution |
|-------|----------|
| "No GPU adapters" | Update drivers, try `WGPU_BACKEND=vulkan` |
| Build fails | Run `cargo update`, check Rust version |
| Shader errors | Check console panel for details |

### GPU Debugging

For detailed GPU debugging:
```bash
# Vulkan validation layers
VK_INSTANCE_LAYERS=VK_LAYER_KHRONOS_validation cargo run

# DirectX debug layer (Windows)
DXGI_DEBUG=1 cargo run
```

## Code Style

### Formatting

- Use `cargo fmt` before committing
- Configuration in `rustfmt.toml`

### Linting

- Use `cargo clippy` before committing
- Fix all warnings
- Configuration in `clippy.toml`

### Documentation

- Document all public items
- Use `///` for item documentation
- Use `//!` for module documentation
- Include examples where helpful

### Naming Conventions

- Types: `PascalCase`
- Functions/variables: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
- Modules: `snake_case`

## Common Tasks

### Updating Dependencies

```bash
cargo update
cargo test --workspace  # Verify nothing broke
```

### Building for Release

```bash
cargo build --release
```

### Building WASM

```bash
# Install target
rustup target add wasm32-unknown-unknown

# Build
cargo build --release --target wasm32-unknown-unknown
```

### Generating Documentation

```bash
cargo doc --workspace --all-features --no-deps --open
```

### Running Benchmarks

```bash
cd crates/wgpu_playground_core
cargo bench
```

## Pull Request Checklist

Before submitting a PR:

- [ ] Code compiles without warnings
- [ ] `cargo fmt --all` applied
- [ ] `cargo clippy --workspace --all-targets` passes
- [ ] Tests pass: `cargo test --workspace`
- [ ] Documentation updated if needed
- [ ] Commit messages are clear and descriptive

## Resources

- [wgpu Documentation](https://docs.rs/wgpu)
- [WebGPU Specification](https://www.w3.org/TR/webgpu/)
- [WGSL Specification](https://www.w3.org/TR/WGSL/)
- [egui Documentation](https://docs.rs/egui)
- [Rust Book](https://doc.rust-lang.org/book/)
