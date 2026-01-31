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

## Future Implementation: Dawn

**Dawn** is Google's C++ implementation of WebGPU, used by Chromium/Chrome browsers.

### Status: Experimental / Placeholder

The `dawn` feature flag is currently a **placeholder** for future integration. To fully support Dawn, the following work would be needed:

1. **FFI Bindings**: Add Rust FFI bindings to Dawn's C API (e.g., using `dawn-sys` crate)
2. **Build System**: Configure the build to compile and link Dawn C++ libraries
3. **Adapter Abstraction**: Create an abstraction layer to switch between wgpu and Dawn adapters
4. **Instance Management**: Implement Dawn-specific instance creation
5. **Testing**: Extensive testing to ensure feature parity

### Why Dawn Support?

Adding Dawn support would enable:
- **Implementation Comparison**: Compare behavior between wgpu (Firefox) and Dawn (Chrome)
- **Conformance Testing**: Test against Chrome's reference implementation
- **Bug Discovery**: Find implementation-specific bugs and differences
- **Educational Value**: Learn about different WebGPU implementation approaches

### How to Add Dawn Support (Future Work)

Here's a roadmap for implementing full Dawn support:

#### Step 1: Add Dawn Dependencies

```toml
# In crates/wgpu_playground_core/Cargo.toml
[dependencies]
# Existing dependencies...

# Optional Dawn support
dawn-sys = { version = "0.x", optional = true }

[features]
dawn = ["dawn-sys"]
```

#### Step 2: Create Dawn Wrapper Module

Create `crates/wgpu_playground_core/src/dawn_wrapper.rs`:

```rust
#[cfg(feature = "dawn")]
pub mod dawn {
    use dawn_sys::*;
    
    pub struct DawnInstance {
        // Dawn-specific instance
    }
    
    pub struct DawnAdapter {
        // Dawn-specific adapter
    }
    
    // Implement Dawn-specific initialization
}
```

#### Step 3: Abstract Instance Creation

Update `src/implementation.rs` to support runtime switching:

```rust
pub enum WebGPUInstance {
    Wgpu(wgpu::Instance),
    #[cfg(feature = "dawn")]
    Dawn(DawnInstance),
}

impl WebGPUInstance {
    pub fn new(impl_type: WebGPUImplementation) -> Self {
        match impl_type {
            WebGPUImplementation::Wgpu => {
                Self::Wgpu(wgpu::Instance::new(...))
            }
            #[cfg(feature = "dawn")]
            WebGPUImplementation::Dawn => {
                Self::Dawn(DawnInstance::new())
            }
        }
    }
}
```

#### Step 4: Update Main Application

Modify initialization in `crates/wgpu_playground_gui/src/main.rs` to support selection:

```rust
// Check environment variable for implementation preference
let impl_type = std::env::var("WEBGPU_IMPL")
    .ok()
    .and_then(|s| match s.as_str() {
        "dawn" => Some(WebGPUImplementation::Dawn),
        "wgpu" => Some(WebGPUImplementation::Wgpu),
        _ => None
    })
    .unwrap_or(WebGPUImplementation::current());

let instance = WebGPUInstance::new(impl_type);
```

#### Step 5: Test and Validate

- Run test suite with both implementations
- Compare rendering output
- Benchmark performance differences
- Document any behavioral differences

## Using Different Implementations

### Current Usage (wgpu only)

```bash
# Default: uses wgpu
cargo run --release

# Select specific backend within wgpu
WGPU_BACKEND=vulkan cargo run --release
WGPU_BACKEND=metal cargo run --release
WGPU_BACKEND=dx12 cargo run --release
```

### Future Usage (with Dawn support)

```bash
# Use wgpu implementation (default)
cargo run --release

# Use Dawn implementation (when available)
cargo run --release --features dawn
WEBGPU_IMPL=dawn cargo run --release --features dawn

# Combine with backend selection
WEBGPU_IMPL=dawn WGPU_BACKEND=vulkan cargo run --release --features dawn
```

## Implementation Information in UI

The active WebGPU implementation is displayed in two places:

1. **Device Info Tab**: Shows the implementation name, description, and source URL
2. **Adapter Selection Tab**: Lists available implementations and indicates which is active

This information helps users understand which implementation they're using and what features are available.

## Architecture Design

The implementation switching is designed with these principles:

1. **Compile-time Selection**: Use Cargo features to include/exclude implementations
2. **Runtime Information**: Display which implementation is active in the UI
3. **Minimal Overhead**: When Dawn is not compiled in, it adds zero runtime cost
4. **Extensibility**: Easy to add more implementations in the future
5. **Type Safety**: Leverage Rust's type system to prevent misuse

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
