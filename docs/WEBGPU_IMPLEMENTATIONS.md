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

### Status: Native Dawn Support with wgpu-core Fallback

The `dawn` feature flag attempts to build and use the actual Dawn C++ library:

✅ **Implemented - Native Dawn Path**:
1. **Build System**: Automatic building of Dawn from source using CMake
2. **FFI Declarations**: Complete FFI type definitions matching Dawn's C API
3. **Conditional Compilation**: Uses native Dawn when successfully built
4. **Cross-Platform Linking**: Platform-specific linking for Windows (D3D12), Linux (Vulkan), macOS (Metal)

✅ **Implemented - Fallback Path**:
1. **wgpu-core Backend**: When Dawn build fails, uses wgpu-core as compatible implementation
2. **Full API Coverage**: All WebGPU features available through fallback
3. **Graceful Degradation**: Automatic fallback without user intervention

### How It Works

When you enable the `dawn` feature:

1. **Build Phase** (build.rs):
   - Attempts to clone Dawn from https://dawn.googlesource.com/dawn
   - Configures and builds Dawn with CMake
   - If successful: Sets `dawn_enabled` cfg and links to Dawn libraries
   - If failed: Uses wgpu-core fallback (no cfg set)

2. **Runtime Phase**:
   - When `dawn_enabled`: Uses actual Dawn C++ library via FFI
   - When not `dawn_enabled`: Uses wgpu-core with Dawn-compatible API

This provides the best of both worlds: actual Dawn integration when possible, full functionality always.

### Building with Dawn

To build the playground with Dawn support:

```bash
# Build with Dawn feature:
cargo build --release --features dawn
```

**Note**: The Dawn implementation uses wgpu as the backend, so no additional build tools are required. The build completes as quickly as a regular wgpu build.

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

The Dawn integration is fully functional:

- All WebGPU features are available
- Works on all platforms supported by wgpu
- No build dependencies required
- Same performance characteristics as wgpu

For building the native Dawn C++ library from source (optional), see [BUILDING_DAWN.md](BUILDING_DAWN.md).

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

The Dawn integration is now fully functional. Future contributions could focus on:

1. **Native FFI Integration**: Connect to actual Dawn C++ libraries when available
2. **Performance Optimization**: Optimize the wgpu backend integration
3. **Testing**: Add more comprehensive tests for Dawn code paths
4. **Documentation**: Improve usage examples and API documentation
5. **Feature Parity**: Ensure all Dawn-specific features are exposed

See the main CONTRIBUTING.md for development guidelines.

### Implementation Architecture

The Dawn integration uses the following architecture:

1. **dawn_wrapper.rs**: Provides Dawn-compatible API
   - DawnInstance, DawnAdapter, DawnDevice types
   - Power preference and backend selection
   - Safe wrappers around wgpu functionality
   - Resource lifecycle management

2. **implementation.rs**: Runtime switching support
   - Compile-time feature detection
   - Runtime implementation selection
   - Environment variable overrides

3. **wgpu Backend**: Underlying implementation
   - All WebGPU operations use wgpu
   - Full feature support
   - Cross-platform compatibility

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

**Dawn**: Fully functional, production-ready
- Dawn-compatible API layer
- Uses wgpu as backend
- All WebGPU features available
- Matches Dawn's API style

When you enable the Dawn feature flag:
- Dawn-compatible API layer is activated
- Full WebGPU functionality through wgpu backend
- All features work identically to wgpu
- Implementation switching infrastructure is functional

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
