//! Fuzz tests for `wgpu_playground_core::math_utils` public interface.
//!
//! Targets: `normalize`, `cross`, `dot`
//!
//! The fuzzer feeds arbitrary byte sequences to these pure vector-math functions
//! and verifies that they never panic for any floating-point input, including NaN
//! and infinity.
#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use wgpu_playground_core::math_utils::{cross, dot, normalize};

/// Structured input for the three math utility functions.
#[derive(Arbitrary, Debug)]
struct MathInput {
    a: [f32; 3],
    b: [f32; 3],
}

fuzz_target!(|input: MathInput| {
    // normalize must never panic, even for zero-length or NaN vectors.
    let _ = normalize(input.a);
    let _ = normalize(input.b);

    // cross product must never panic.
    let _ = cross(input.a, input.b);

    // dot product must never panic.
    let _ = dot(input.a, input.b);
});
