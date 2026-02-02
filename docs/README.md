# wgpu_playground Documentation

This directory contains design documents and implementation notes for the wgpu_playground project.

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

- UI mockups and design specifications
- Implementation notes and summaries
- Architecture documentation
- Integration guides

These documents complement the API documentation and provide high-level context for the project's design decisions.
