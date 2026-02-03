# wgpu_playground Documentation

This directory contains design documents, implementation notes, and user guides for the wgpu_playground project.

## User Documentation

New to wgpu_playground? Start here:

- **[QUICK_START.md](QUICK_START.md)** - Get up and running in 5 minutes
- **[USER_GUIDE.md](USER_GUIDE.md)** - Comprehensive user guide with tutorials, workflows, and troubleshooting

These guides cover:
- Installation and setup
- GUI navigation and usage
- Creating buffers, textures, and samplers
- Writing and compiling shaders
- Step-by-step tutorials for common tasks
- Troubleshooting common issues

## API Documentation

The project's API documentation is automatically generated from source code using `cargo doc` and published to GitHub Pages.

### Viewing Documentation

- **Online**: The latest documentation is available at the project's GitHub Pages site (published automatically on every push to `main`)
- **Local**: Generate and view documentation locally:
  ```bash
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
- **[WEBGPU_IMPLEMENTATIONS.md](WEBGPU_IMPLEMENTATIONS.md)** - WebGPU implementations (wgpu vs Dawn) and architecture
- **[BENCHMARKING.md](BENCHMARKING.md)** - Performance benchmarking system and guidelines
- **[CI_TESTING.md](CI_TESTING.md)** - Continuous integration testing documentation
- **[architecture.md](architecture.md)** - System architecture and design

#### Implementation Notes
- UI mockups and design specifications
- Implementation notes and summaries
- Integration guides
- Feature-specific documentation

These documents complement the API documentation and provide high-level context for the project's design decisions.
