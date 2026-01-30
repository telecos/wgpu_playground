# GUI Framework Evaluation for wgpu_playground

## Executive Summary

After thorough evaluation of Rust GUI frameworks suitable for WebGPU integration, **egui** has been selected for the wgpu_playground project. This document provides the detailed rationale behind this decision, comparing egui with iced and imgui-wgpu across critical criteria.

## Evaluation Criteria

The selection was based on three primary criteria:

1. **Ease of Integration with wgpu** - How well the framework integrates with wgpu and the complexity of setup
2. **WASM Support** - Compatibility with WebAssembly for browser-based deployment
3. **Feature Richness** - Available widgets, customization options, and overall capabilities

## Framework Comparison

### 1. egui (Selected)

**Version Evaluated:** 0.29.x  
**License:** MIT/Apache-2.0  
**Repository:** https://github.com/emilk/egui

#### Overview
egui is an immediate mode GUI library written in pure Rust. It's designed to be easy to use, portable, and efficient, with a focus on developer experience.

#### Strengths

**Ease of Integration with wgpu:**
- ✅ Excellent - Official `egui-wgpu` integration crate maintained by the core team
- ✅ Clean API with minimal boilerplate (~100 lines for basic setup)
- ✅ Direct wgpu surface rendering without intermediate framebuffers
- ✅ Well-documented integration examples in official repository
- ✅ Active maintenance with regular updates tracking wgpu versions
- ✅ Handles GPU resource management automatically

**WASM Support:**
- ✅ Excellent - First-class WASM support through winit's WASM backend
- ✅ No platform-specific code required - same egui/egui-wgpu/egui-winit stack works on wasm32
- ✅ Runs efficiently in browser with WebGPU support
- ✅ Active WASM examples and demos available online
- ✅ Small binary size (< 1MB gzipped for WASM)
- ✅ Works with both WebGL and WebGPU backends

**Feature Richness:**
- ✅ Comprehensive widget set (buttons, sliders, text input, tables, collapsible headers, etc.)
- ✅ Built-in plotting and graphing capabilities (egui_plot)
- ✅ Rich text formatting with fonts and colors
- ✅ Drag-and-drop support
- ✅ Context menus and popups
- ✅ Docking and multi-window support
- ✅ Extensible with custom widgets
- ✅ Immediate mode paradigm - simple state management
- ✅ Built-in theme support and customization

#### Considerations
- Immediate mode means UI is rebuilt every frame (generally not an issue for tool UIs)
- Accessibility features are still developing
- Custom styling requires more work than declarative frameworks

#### Integration Example
```rust
// Minimal setup required (simplified for illustration)
// See actual implementation in crates/wgpu_playground_gui/src/main.rs for complete details
let egui_ctx = egui::Context::default();
let mut egui_state = egui_winit::State::new(/* winit context parameters */);
let egui_renderer = egui_wgpu::Renderer::new(/* wgpu device and configuration */);
```

---

### 2. iced

**Version Evaluated:** 0.13.x  
**License:** MIT  
**Repository:** https://github.com/iced-rs/iced

#### Overview
iced is a cross-platform GUI library inspired by Elm architecture, using a declarative, type-safe approach with a focus on simplicity and reliability.

#### Strengths

**Ease of Integration with wgpu:**
- ⚠️ Moderate - Uses wgpu internally but as an abstraction layer
- ⚠️ More opinionated architecture (Elm-style architecture)
- ⚠️ Requires adapting to iced's runtime and message-passing system
- ⚠️ Less direct control over wgpu resources
- ✅ Well-structured integration once you adapt to the paradigm
- ⚠️ More boilerplate for custom rendering alongside UI

**WASM Support:**
- ✅ Good - WASM support available but less mature than egui
- ⚠️ Requires `iced_web` crate with different API
- ⚠️ Some features may not work identically between native and web
- ⚠️ Larger binary size compared to egui
- ✅ Active development of WASM support

**Feature Richness:**
- ✅ Modern widget set with good defaults
- ✅ Built-in theming system
- ✅ Responsive layout engine
- ✅ Animation support
- ✅ Good accessibility features
- ⚠️ Smaller ecosystem of third-party widgets compared to egui
- ⚠️ Custom widgets require more boilerplate
- ⚠️ Less suitable for rapid prototyping and experimentation

#### Considerations
- Better for traditional application UIs with complex layouts
- Elm architecture adds conceptual overhead for simple tools
- State management is more structured but also more rigid
- Less suitable for dynamic, tool-like interfaces with frequent state changes

---

### 3. imgui-wgpu (imgui-rs with wgpu backend)

**Version Evaluated:** imgui 0.12.x, imgui-wgpu 0.24.x  
**License:** MIT/Apache-2.0  
**Repository:** https://github.com/imgui-rs/imgui-rs

#### Overview
imgui-rs provides Rust bindings to Dear ImGui, a C++ immediate mode GUI library. imgui-wgpu provides the wgpu rendering backend.

#### Strengths

**Ease of Integration with wgpu:**
- ⚠️ Moderate - Requires managing both imgui-rs and imgui-wgpu
- ⚠️ Two-layer abstraction (C++ Dear ImGui + Rust bindings)
- ⚠️ Manual texture management for custom rendering
- ⚠️ More complex setup with multiple initialization steps
- ✅ Proven and stable rendering backend
- ⚠️ Potential FFI overhead

**WASM Support:**
- ❌ Poor - WASM support is limited and unofficial
- ❌ Dear ImGui is primarily designed for native platforms
- ❌ Requires complex build setup for WASM
- ❌ Large binary size due to C++ runtime
- ❌ Not recommended for web deployment

**Feature Richness:**
- ✅ Mature and battle-tested widget set
- ✅ Excellent documentation (Dear ImGui docs)
- ✅ Large ecosystem of extensions and addons
- ✅ Advanced features like docking and viewports
- ✅ Widely used in game development and tools
- ⚠️ API can feel less idiomatic to Rust
- ⚠️ Some features require unsafe code
- ⚠️ Dependency on C++ library

#### Considerations
- Best suited for native-only applications
- More suitable if you need compatibility with existing Dear ImGui tools
- Not ideal for WASM deployment
- Additional complexity from FFI boundary

---

## Decision Matrix

| Criterion | Weight | egui | iced | imgui-wgpu |
|-----------|--------|------|------|------------|
| **wgpu Integration** | 40% | 9/10 | 6/10 | 6/10 |
| Direct wgpu access | | ✅ Excellent | ⚠️ Abstracted | ⚠️ Moderate |
| Setup complexity | | ✅ Minimal | ⚠️ Moderate | ⚠️ High |
| Documentation | | ✅ Excellent | ✅ Good | ⚠️ Moderate |
| | | | | |
| **WASM Support** | 35% | 10/10 | 7/10 | 2/10 |
| Browser compatibility | | ✅ Excellent | ✅ Good | ❌ Limited |
| Binary size | | ✅ Small | ⚠️ Moderate | ❌ Large |
| Performance | | ✅ Excellent | ✅ Good | ⚠️ Moderate |
| | | | | |
| **Feature Richness** | 25% | 9/10 | 8/10 | 9/10 |
| Widget variety | | ✅ Comprehensive | ✅ Good | ✅ Extensive |
| Customization | | ✅ Flexible | ✅ Structured | ✅ Flexible |
| Ecosystem | | ✅ Active | ⚠️ Growing | ✅ Mature |
| | | | | |
| **Overall Score** | | **9.35/10** | **6.7/10** | **5.5/10** |

## Final Decision: egui

### Rationale

**egui** was selected as the GUI framework for wgpu_playground for the following key reasons:

#### 1. Superior wgpu Integration (Score: 9/10)
- The official `egui-wgpu` integration provides seamless wgpu support with minimal abstraction
- Setup requires approximately 100 lines of straightforward code
- Direct access to wgpu resources enables custom rendering alongside UI elements
- Regular updates ensure compatibility with latest wgpu versions
- Well-documented examples specifically for wgpu integration

#### 2. Excellent WASM Support (Score: 10/10)
- First-class WASM support is critical for potential browser-based deployment
- Small binary footprint (~1MB gzipped) makes it practical for web deployment
- No platform-specific code needed - same codebase works native and web
- Proven track record with numerous WASM deployments
- Future-proofs the project for web-based experimentation

#### 3. Rich Feature Set (Score: 9/10)
- Immediate mode paradigm is ideal for tool-like, experimental UIs
- Comprehensive widget set covers all needs for WebGPU experimentation
- Built-in plotting capabilities useful for performance visualization
- Easy to prototype and iterate quickly
- Extensible architecture for custom widgets as needed

#### 4. Additional Benefits
- **Active Development**: Frequent updates and responsive maintainers
- **Pure Rust**: No C++ dependencies or FFI overhead
- **Developer Experience**: Intuitive API, quick iteration cycle
- **Community**: Large and active community with extensive examples
- **Performance**: Efficient rendering with minimal overhead
- **Documentation**: Excellent documentation and examples

### Trade-offs Accepted

1. **Immediate Mode Paradigm**: UI is rebuilt every frame rather than retained
   - Acceptable because tool UIs are typically simple
   - Modern GPUs handle this efficiently
   - Simplifies state management for experimentation tools

2. **Styling Complexity**: Custom theming requires more code than declarative frameworks
   - Default theme is professional and sufficient for tools
   - Advanced styling can be added incrementally if needed

3. **Accessibility**: Still developing compared to mature frameworks
   - Not a primary concern for developer tools
   - Active work in progress in the egui community

### Alternative Use Cases

For reference, scenarios where alternatives might be preferred:

- **Choose iced if**: Building a traditional application with complex layouts, need strong typing guarantees, or prefer declarative UI patterns
- **Choose imgui-wgpu if**: Working on a native-only tool, need compatibility with existing Dear ImGui codebases, or require specific Dear ImGui extensions

## Implementation Status

egui (version 0.29.x) has been successfully integrated into the wgpu_playground project with the following components:

- `egui` - Core immediate mode GUI library
- `egui-wgpu` - Official wgpu rendering backend
- `egui-winit` - Window system integration

The integration provides:
- Tabbed interface for organizing features
- Device information display
- Placeholder panels for rendering and compute experiments
- Full WASM compatibility ready for future deployment

## References

### egui
- Repository: https://github.com/emilk/egui
- Documentation: https://docs.rs/egui/
- Examples: https://www.egui.rs/
- WASM Demo: https://www.egui.rs/#demo

### iced
- Repository: https://github.com/iced-rs/iced
- Documentation: https://docs.rs/iced/
- Examples: https://github.com/iced-rs/iced/tree/master/examples

### imgui-rs
- Repository: https://github.com/imgui-rs/imgui-rs
- Documentation: https://docs.rs/imgui/
- Dear ImGui: https://github.com/ocornut/imgui

### wgpu Integration Resources
- egui-wgpu: https://docs.rs/egui-wgpu/
- wgpu: https://docs.rs/wgpu/

## Conclusion

The selection of egui provides an optimal balance of ease of integration, WASM support, and feature richness for the wgpu_playground project. Its immediate mode paradigm aligns perfectly with the experimental and tool-like nature of the application, while its first-class wgpu and WASM support ensures the project can grow in both capability and reach.

The framework enables rapid development of the WebGPU experimentation features outlined in the project roadmap while maintaining the flexibility to add custom rendering and advanced UI features as needed.

---

**Document Version:** 1.0  
**Last Updated:** 2026-01-30  
**Status:** Final Decision - egui Selected and Implemented
