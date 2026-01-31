# WebGPU Implementations

This document explains the WebGPU implementation support in the wgpu_playground and how to switch between different implementations.

## Overview

WebGPU is a modern GPU API that can be implemented in different ways. This playground supports:

1. **wgpu** (default) - Pure Rust implementation used by Firefox
2. **Dawn** (experimental) - C++ implementation used by Chromium

## Current Implementation: wgpu

By default, the playground uses the **wgpu** crate, which is a safe, portable, and performant implementation of the WebGPU API written in Rust.

### Features
- **Safety**: Written in Rust with memory safety guarantees
- **Cross-platform**: Works on Windows, Linux, macOS, iOS, Android, and Web
- **Performance**: Compiles to efficient native code
- **Active development**: Maintained by the gfx-rs team
- **Used by**: Firefox, Deno, Bevy game engine

### Backend Support
wgpu supports multiple graphics API backends:
- **Vulkan** - Windows, Linux, Android, macOS (via MoltenVK)
- **Metal** - macOS, iOS
- **DirectX 12** - Windows 10+
- **OpenGL/GLES** - Fallback for older platforms
- **WebGPU** - Browser environments (via WASM)

## Dawn Implementation

**Dawn** is Google's C++ implementation of WebGPU, used by Chromium/Chrome browsers.

### Status: Build Infrastructure Complete

The `dawn` feature flag now provides a complete build infrastructure for Dawn integration:

✅ **Implemented**:
1. **Build System**: Automatic building of Dawn from source using CMake
2. **FFI Foundation**: Custom FFI type definitions for Dawn's C API
3. **Cross-Platform Support**: Platform-specific linking for Windows (D3D12), Linux (Vulkan), macOS (Metal)
4. **Wrapper Module**: Safe Rust abstractions over Dawn's C API
5. **Instance Management**: Structure for Dawn instance and adapter management

🚧 **In Progress**:
- **Runtime Integration**: Connecting FFI stubs to built Dawn libraries
- **Adapter Enumeration**: Implementing full adapter discovery
- **Device Creation**: Complete device initialization flow

### Building Dawn from Source

When you enable the `dawn` feature, the build system automatically:

1. Clones the Dawn repository from `https://dawn.googlesource.com/dawn`
2. Configures the build using CMake with dependency fetching
3. Builds Dawn in Release mode with parallel compilation
4. Installs Dawn libraries and headers to the build output directory
5. Sets up platform-specific linking

**Build Requirements**:
- Git (for cloning Dawn repository)
- CMake 3.16+ (for build configuration)
- C++ compiler with C++20 support (MSVC on Windows, GCC/Clang on Linux/macOS)
- Python 3 (for Dawn's dependency management scripts)

**Build Time**: First build with Dawn takes 10-30 minutes depending on your system.

### Why Dawn Support?

Adding Dawn support enables:
- **Implementation Comparison**: Compare behavior between wgpu (Firefox) and Dawn (Chrome)
- **Conformance Testing**: Test against Chrome's reference implementation
- **Bug Discovery**: Find implementation-specific bugs and differences
- **Educational Value**: Learn about different WebGPU implementation approaches
- **Production Parity**: Match the WebGPU implementation used in Chromium browsers

### Platform-Specific Backends

Dawn automatically selects the appropriate backend for each platform:

- **Windows**: Direct3D 12 (D3D12) - Primary backend for Windows 10+
- **Linux**: Vulkan - Native high-performance graphics API
- **macOS**: Metal - Apple's native graphics API

The build system automatically configures platform-specific linking requirements.

### Building with Dawn

To build the playground with Dawn support:

```bash
# Install required tools first
# On Ubuntu/Debian:
sudo apt-get install git cmake build-essential python3

# On macOS (with Homebrew):
brew install git cmake python3

# On Windows:
# Install Visual Studio with C++ support, CMake, Git, and Python 3

# Then build with Dawn feature:
cargo build --release --features dawn
```

**Note**: The first build will clone and compile Dawn, which can take 10-30 minutes.
Subsequent builds will be much faster as Dawn is cached in the build directory.

### Running with Dawn

Once built, you can run the playground with Dawn:

```bash
# Using environment variable (recommended)
WEBGPU_IMPL=dawn cargo run --release --features dawn

# Or let it use the compile-time default (Dawn when feature is enabled)
cargo run --release --features dawn
```

### Implementation Architecture

The Dawn integration uses the following architecture:

1. **build.rs**: Handles Dawn compilation and linking
   - Clones Dawn repository if not present
   - Configures CMake build
   - Compiles Dawn in Release mode
   - Sets up library search paths

2. **dawn_wrapper.rs**: Provides safe Rust abstractions
   - FFI type definitions matching webgpu.h
   - Safe wrappers around unsafe FFI calls
   - Resource lifecycle management
   - Error handling

3. **implementation.rs**: Runtime switching support
   - Compile-time feature detection
   - Runtime implementation selection
   - Environment variable overrides

### Current Limitations

While the build infrastructure is complete, full runtime integration is still in progress:

- FFI function declarations are defined but not yet linked to compiled Dawn libraries
- Instance creation returns a placeholder error
- Adapter enumeration is not yet implemented
- The build warns about the placeholder status

These will be addressed in future updates as the FFI integration is completed.

### Troubleshooting Dawn Builds

**"Git not found"**:
- Install Git: https://git-scm.com/downloads
- Ensure `git` is in your PATH

**"CMake not found"**:
- Install CMake 3.16 or later: https://cmake.org/download/
- Ensure `cmake` is in your PATH

**"Build failed" on Linux**:
- Install build essentials: `sudo apt-get install build-essential`
- Install Vulkan development files: `sudo apt-get install libvulkan-dev`

**"Build failed" on Windows**:
- Install Visual Studio 2019 or later with C++ Desktop Development
- Ensure you have Windows SDK installed

**"Python not found"**:
- Install Python 3: https://www.python.org/downloads/
- Ensure `python3` (or `python` on Windows) is in your PATH

### Contributing to Dawn Integration

The Dawn integration is functional but not yet complete. Contributions are welcome in these areas:

1. **FFI Runtime Integration**: Connect FFI stubs to built libraries
2. **Adapter Enumeration**: Implement adapter discovery and selection
3. **Device Creation**: Complete device initialization flow
4. **Testing**: Add comprehensive tests for Dawn code paths
5. **Documentation**: Improve build and usage documentation
6. **CI/CD**: Add Dawn builds to continuous integration

See the main CONTRIBUTING.md for development guidelines.

### How Dawn Integration Works (Technical Details)

#### Build Process

When `--features dawn` is enabled:

1. **Clone Dawn**: build.rs checks if Dawn source exists, clones if needed
2. **CMake Configure**: Runs `cmake -S dawn -B build -DDAWN_FETCH_DEPENDENCIES=ON`
3. **Build Dawn**: Runs `cmake --build build --parallel`
4. **Install**: Runs `cmake --install build` to output directory
5. **Link**: Sets rustc link flags for Dawn libraries and platform dependencies

#### FFI Layer

The FFI layer (`dawn_wrapper.rs`) provides:

- Type definitions matching Dawn's webgpu.h API
- Safe wrappers for `DawnInstance`, `DawnAdapter`, `DawnDevice`
- Rust enums for power preferences and backends
- Error types for Dawn-specific errors
- Resource lifecycle management

Currently, FFI functions are declared but not yet linked. Full runtime integration is in progress.

## Using Different Implementations

### Current Usage

The playground supports runtime switching between implementations using environment variables:

```bash
# Default: uses wgpu implementation
cargo run --release

# Explicitly select wgpu
WEBGPU_IMPL=wgpu cargo run --release

# Select Dawn (requires compilation with --features dawn)
cargo run --release --features dawn
WEBGPU_IMPL=dawn cargo run --release --features dawn

# Select specific backend within wgpu
WGPU_BACKEND=vulkan cargo run --release
WGPU_BACKEND=metal cargo run --release
WGPU_BACKEND=dx12 cargo run --release

# Combine implementation and backend selection
WEBGPU_IMPL=wgpu WGPU_BACKEND=vulkan cargo run --release
```

### Implementation Status

**wgpu**: Fully functional, production-ready
- Native Rust implementation
- All features supported
- Used by Firefox

**Dawn**: Build infrastructure complete, runtime integration in progress
- Builds from source automatically
- FFI types and wrappers defined
- Runtime calls pending
- Will match Chromium's implementation when complete

When you enable the Dawn feature flag:
- Build system automatically builds Dawn from source
- FFI infrastructure is in place
- Runtime calls return placeholder errors until integration is complete
- All the switching infrastructure is functional

## Implementation Information in UI

The active WebGPU implementation is displayed in two places:

1. **Device Info Tab**: Shows the implementation name, description, source URL, and status
2. **Adapter Selection Tab**: Lists available implementations and indicates which is active

This information helps users understand which implementation they're using and what features are available.

## Architecture Design

The implementation switching is designed with these principles:

1. **Compile-time Selection**: Use Cargo features to include/exclude implementations
2. **Runtime Information**: Display which implementation is active in the UI
3. **Minimal Overhead**: When Dawn is not compiled in, it adds zero runtime cost
4. **Extensibility**: Easy to add more implementations in the future
5. **Type Safety**: Leverage Rust's type system to prevent misuse
6. **Build from Source**: Dawn is built automatically from source for maximum compatibility

## References

- **wgpu**: https://github.com/gfx-rs/wgpu
- **Dawn**: https://dawn.googlesource.com/dawn
- **WebGPU Specification**: https://www.w3.org/TR/webgpu/
- **dawn-sys (Rust bindings)**: https://docs.rs/dawn-sys/

## Contributing

If you're interested in implementing full Dawn support, please:

1. Open an issue to discuss the approach
2. Review this document and the roadmap
3. Start with small, incremental PRs
4. Ensure tests pass with both implementations
5. Document any implementation-specific behaviors

## FAQ

### Q: Why not use Dawn by default?

**A**: wgpu is written in Rust, providing memory safety and better integration with the Rust ecosystem. It's also the implementation used by Firefox, making it a good default choice for a Rust project.

### Q: Can I use both implementations at the same time?

**A**: Not in the same application instance. You must choose one implementation at compile time via feature flags, and the application uses that implementation exclusively.

### Q: What about other WebGPU implementations?

**A**: The architecture could be extended to support other implementations like:
- **wgpu-native** (C FFI bindings to wgpu)
- Browser-native WebGPU (via WASM)
- Custom implementations

### Q: Will the API be the same with different implementations?

**A**: Yes, all implementations follow the WebGPU specification. However, there may be minor behavioral differences, bugs, or performance characteristics that vary between implementations.

### Q: How can I contribute to Dawn support?

**A**: Start by familiarizing yourself with:
1. Dawn's C API (`webgpu.h`)
2. Rust FFI (Foreign Function Interface)
3. The `dawn-sys` crate if available
4. The architecture outlined in this document

Then open an issue to discuss your implementation plan before starting work.
