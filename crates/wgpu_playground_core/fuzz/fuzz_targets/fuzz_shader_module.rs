//! Fuzz tests for `wgpu_playground_core::shader` public interface.
//!
//! Target: `ShaderModule::new()` with inline WGSL source.
//!
//! The fuzzer converts arbitrary byte sequences into strings and passes them as
//! inline WGSL source to `ShaderModule::new()`.  The goal is to verify that the
//! function never panics regardless of the content of the input – it may return
//! `Ok` or a well-formed `Err`, but must not crash or exhibit undefined behaviour.
#![no_main]

use libfuzzer_sys::fuzz_target;
use wgpu_playground_core::shader::{ShaderModule, ShaderSource};

fuzz_target!(|data: &[u8]| {
    // Convert bytes to a (possibly non-UTF-8) string via lossy conversion so the
    // fuzzer can explore the full byte space while still producing a valid Rust
    // `String` to hand to the public API.
    let source_code = String::from_utf8_lossy(data).into_owned();

    // ShaderModule::new() must never panic. It returns Err for empty input and
    // Ok for any non-empty source string (validation of WGSL syntax happens later
    // when a wgpu Device compiles the module, not here).
    let _ = ShaderModule::new(ShaderSource::Inline(source_code), None);
});
