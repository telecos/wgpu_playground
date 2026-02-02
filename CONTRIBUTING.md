# Contributing to WebGPU Playground

Thank you for your interest in contributing to WebGPU Playground! This document provides guidelines and information for contributors.

## Development Setup

### Prerequisites
- Rust (latest stable version)
- A GPU with WebGPU support (Vulkan, Metal, or DirectX 12)
- Display/window system (X11, Wayland, or native windowing)

### Building
```bash
cargo build
```

### Running
```bash
cargo run
```

### Testing
```bash
cargo test
cargo clippy
cargo fmt -- --check
```

## Project Structure

```
wgpu_playground/
├── src/
│   ├── main.rs          # Application entry point and window setup
│   ├── app.rs           # Main UI application and tab management
│   ├── device_info.rs   # GPU device information display
│   ├── rendering.rs     # Rendering APIs experimentation
│   └── compute.rs       # Compute/ML APIs experimentation
├── Cargo.toml           # Project dependencies
├── README.md            # Project overview
├── PLAN.md              # Development roadmap with GitHub issues
└── CONTRIBUTING.md      # This file
```

## Development Workflow

1. **Choose an Issue**: Refer to [PLAN.md](PLAN.md) for planned features
2. **Create a Branch**: `git checkout -b feature/issue-name`
3. **Implement**: Follow the coding standards below
4. **Test**: Ensure all tests pass and add new tests as needed
5. **Format**: Run `cargo fmt` before committing
6. **Lint**: Run `cargo clippy` and fix any warnings
7. **Commit**: Write clear, descriptive commit messages
8. **Push**: Push your branch and create a Pull Request

## Coding Standards

### Rust Style
- Follow the official [Rust style guide](https://doc.rust-lang.org/1.0.0/style/)
- Use `cargo fmt` for consistent formatting
- Address all `cargo clippy` warnings

### Code Organization
- Keep modules focused and single-purpose
- Use descriptive names for functions, variables, and types
- Add comments for complex logic
- Document public APIs with doc comments

### Error Handling
- Use `Result<T, E>` for operations that can fail
- Provide user-friendly error messages
- Don't panic in production code (except for truly unrecoverable errors)

### GPU Code
- Check for feature support before using advanced GPU features
- Handle device loss gracefully
- Optimize for performance but prioritize correctness
- Document GPU resource usage

## Adding New Features

When implementing features from PLAN.md:

1. **Read the Issue Description**: Understand the goals and acceptance criteria
2. **Design First**: Plan your implementation before coding
3. **Implement Incrementally**: Build features step by step
4. **Test Thoroughly**: Verify correctness on different GPUs/platforms
5. **Document**: Add examples and documentation
6. **Performance**: Profile and optimize if needed

## Shader Development

When adding shaders:
- Use WGSL (WebGPU Shading Language)
- Include comments explaining the shader's purpose
- Provide example usage in the UI
- Test on multiple GPU vendors if possible
- Document any limitations or requirements

## UI Guidelines

When extending the UI:
- Keep the interface intuitive and beginner-friendly
- Provide tooltips for complex options
- Show error messages clearly
- Include visual feedback for operations
- Test UI responsiveness

## Performance Considerations

- Profile before optimizing
- Use appropriate buffer usage flags
- Minimize state changes
- Batch operations when possible
- Document performance characteristics

## Testing

The project uses Rust's built-in testing framework with a focus on modularity and reusability.

### Test Organization

Tests are organized following Rust best practices:

1. **Unit Tests**: Located in each source file within a `#[cfg(test)]` module
   - Test individual functions and methods
   - Keep tests close to the code they test
   - Example: See `crates/wgpu_playground_core/src/buffer.rs`

2. **Integration Tests**: Located in `tests/` directories
   - Test interactions between modules
   - Use GPU resources for end-to-end testing
   - Example: See `crates/wgpu_playground_core/tests/`

3. **Common Test Utilities**: Shared helpers in `tests/common/mod.rs`
   - Reusable test setup functions
   - GPU device creation helpers
   - Test shader sources
   - Reduces code duplication across integration tests

### Writing Tests

When adding new tests:

1. **Unit Tests** - Add to the same file as the code:
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_feature() {
           // Test code here
       }
   }
   ```

2. **Integration Tests** - Use common utilities:
   ```rust
   mod common;
   use common::create_test_device;

   #[test]
   fn test_integration() {
       pollster::block_on(async {
           let Some((device, queue)) = create_test_device().await else {
               eprintln!("Skipping test: No GPU adapter available");
               return;
           };
           // Test code here
       });
   }
   ```

3. **GPU Tests** - Handle missing GPU gracefully:
   - Tests should skip (not fail) when no GPU is available
   - Use the `create_test_device()` helper from `common` module
   - Always check for `None` and skip the test in headless environments

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run only unit tests
cargo test --workspace --lib

# Run only integration tests
cargo test --workspace --test '*'

# Run tests for a specific crate
cargo test -p wgpu_playground_core

# Run a specific test
cargo test test_buffer_creation
```

### Test Coverage

- Add unit tests for new functionality
- Add integration tests for GPU operations
- Verify visual output for rendering features
- Test on different platforms (Windows, macOS, Linux)
- Test with different GPU vendors (NVIDIA, AMD, Intel, Apple)
- Check for memory leaks and resource cleanup

## Documentation

- Update README.md if changing user-facing features
- Update PLAN.md to track progress
- Add doc comments for public APIs
- Include examples in code comments
- Create tutorials for complex features

## Pull Request Guidelines

### Before Submitting
- [ ] Code compiles without errors
- [ ] All tests pass
- [ ] Code is formatted with `cargo fmt`
- [ ] No clippy warnings
- [ ] Documentation is updated
- [ ] Commit messages are clear

### PR Description Should Include
- What feature/fix is implemented
- How it was tested
- Screenshots/videos for UI changes
- Performance impact (if applicable)
- Breaking changes (if any)

## Getting Help

- Check existing issues and documentation
- Ask questions in pull request comments
- Open a discussion issue for design questions

## Code of Conduct

- Be respectful and constructive
- Welcome newcomers
- Focus on the code, not the person
- Assume good intentions

## Release Process

Releases are automated through GitHub Actions. When a version tag is pushed, the CI pipeline automatically builds and publishes release artifacts.

### Creating a Release

1. **Update Version Numbers**: Update version in workspace `Cargo.toml`
   ```bash
   # Update version in Cargo.toml
   version = "0.2.0"
   ```

2. **Commit Changes**:
   ```bash
   git commit -am "Bump version to 0.2.0"
   git push
   ```

3. **Create and Push Tag**:
   ```bash
   git tag v0.2.0
   git push origin v0.2.0
   ```

4. **Automated Build**: The release workflow automatically:
   - Builds native binaries for:
     - Linux (x86_64)
     - Windows (x86_64)
     - macOS (Intel and Apple Silicon)
   - Creates WASM bundle for web deployment
   - Generates documentation bundle
   - Creates GitHub Release with all artifacts
   - Publishes to GitHub Releases page

### Release Artifacts

The release workflow produces:
- **Native Binaries**: Platform-specific executables (.tar.gz for Unix, .zip for Windows)
- **WASM Bundle**: WebAssembly build with JavaScript bindings for web deployment
- **Documentation**: Complete API documentation in HTML format

### Pre-release Versions

For beta or release candidate versions, use a tag with a suffix:
```bash
git tag v0.2.0-beta.1
git push origin v0.2.0-beta.1
```

The release will be marked as a pre-release automatically.

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (MIT).

## Recognition

Contributors will be recognized in the project's README and release notes.

Thank you for contributing to WebGPU Playground! 🎮
