# wgpu_playground Documentation

This directory contains the documentation for the wgpu_playground project.

## Documentation Structure

| Document | Description |
|----------|-------------|
| [ARCHITECTURE.md](ARCHITECTURE.md) | System architecture, module structure, and design decisions |
| [USER_GUIDE.md](USER_GUIDE.md) | Complete user guide for the application |
| [DEVELOPER_GUIDE.md](DEVELOPER_GUIDE.md) | Developer workflows, testing, and contribution guide |

## Quick Links

### For Users
- **Getting Started**: See [USER_GUIDE.md](USER_GUIDE.md) for installation and basic usage
- **Try the Live Demo**: [WebGPU Demo](https://telecos.github.io/wgpu_playground/demo/) - Try in your browser (requires WebGPU support)

### For Developers  
- **Setup**: See [DEVELOPER_GUIDE.md](DEVELOPER_GUIDE.md) for development environment setup
- **Architecture**: See [ARCHITECTURE.md](ARCHITECTURE.md) for system design and module structure
- **Contributing**: See [CONTRIBUTING.md](../CONTRIBUTING.md) in the root directory

## Additional Resources

- **Project README**: [../README.md](../README.md) - Project overview and quick start
- **Task Tracking**: [../TASKS.md](../TASKS.md) - Current tasks and enhancements
- **Roadmap**: [../PLAN.md](../PLAN.md) - Project roadmap and milestones

## API Documentation

Generate and view Rust API documentation locally:

```bash
cargo doc --workspace --all-features --no-deps --open
```

  cargo doc --workspace --all-features --no-deps --open
  ```

### Building Documentation

Documentation is built using Rust's built-in documentation tool:

```bash
# Build documentation for all crates
cargo doc --workspace --all-features --no-deps

# Build with warnings as errors (same as CI)
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps
```

### CI/CD Pipeline

The documentation workflow (`.github/workflows/docs.yml`) runs on every push and pull request to:

1. **Build Documentation**: Generate API docs from code comments using `cargo doc`
2. **Validate Quality**: Treat documentation warnings as errors to ensure completeness
3. **Check Links**: Validate all hyperlinks in the generated documentation
4. **Deploy**: Publish to GitHub Pages (main branch only)

### Documentation Guidelines

When contributing to the project:

- Add documentation comments (`///`) to all public APIs
- Use proper Markdown formatting in doc comments
- Wrap types in backticks: `` `Vec<u8>` ``
- Use angle brackets for URLs: `<https://example.com>`
- Include examples in doc comments where appropriate
- Document errors, panics, and safety requirements

### Documentation Structure

The published documentation includes:

- **wgpu_playground_core**: Core WebGPU functionality and rendering primitives
- **wgpu_playground_gui**: GUI application and user interface components
- **wgpu_playground_examples**: Example programs and usage demonstrations

### Design Documents

This `docs/` directory also contains various design documents:

#### User-Facing Documentation
- **[USER_GUIDE.md](USER_GUIDE.md)** - Comprehensive end-user guide covering GUI usage, workflows, and tutorials
- **[QUICK_START.md](QUICK_START.md)** - Quick start guide to get running in 5 minutes
- **[WGSL_SHADER_GUIDE.md](WGSL_SHADER_GUIDE.md)** - Complete guide to writing WGSL shaders, including structure, built-in functions, and debugging
- **[SHADER_EDITOR.md](SHADER_EDITOR.md)** - Complete guide to the WGSL Shader Editor
- **[VISUAL_REGRESSION_TESTING.md](VISUAL_REGRESSION_TESTING.md)** - Guide to the visual regression testing framework

#### Technical Documentation
- **[WASM_DEMO_DEPLOYMENT.md](WASM_DEMO_DEPLOYMENT.md)** - WebAssembly demo deployment guide and troubleshooting
- **[WASM_TESTING.md](WASM_TESTING.md)** - Testing WebAssembly builds
- **[WEBGPU_IMPLEMENTATIONS.md](WEBGPU_IMPLEMENTATIONS.md)** - WebGPU implementations (wgpu vs Dawn) and architecture
- **[BENCHMARKING.md](BENCHMARKING.md)** - Performance benchmarking system and guidelines
- **[CI_TESTING.md](CI_TESTING.md)** - Continuous integration testing documentation
- **[BRANCH_PROTECTION.md](BRANCH_PROTECTION.md)** - Branch protection rules and PR workflow
- **[architecture.md](architecture.md)** - System architecture and design

#### Implementation Notes
- UI mockups and design specifications
- Implementation notes and summaries
- Integration guides
- Feature-specific documentation

These documents complement the API documentation and provide high-level context for the project's design decisions.
