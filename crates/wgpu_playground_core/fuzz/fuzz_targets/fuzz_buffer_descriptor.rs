//! Fuzz tests for `wgpu_playground_core::buffer` public interface.
//!
//! Targets: `BufferDescriptor` construction and validation.
//!
//! The fuzzer feeds arbitrary size values and usage-bit combinations to
//! `BufferDescriptor::new()` followed by `validate()` and verifies that neither
//! call ever panics for any input.
#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use wgpu_playground_core::buffer::{BufferDescriptor, BufferUsages};

/// Structured input for `BufferDescriptor`.
#[derive(Arbitrary, Debug)]
struct BufferInput {
    /// Arbitrary buffer size in bytes.
    size: u64,
    /// Raw usage bits – the fuzzer explores all flag combinations including
    /// invalid ones (e.g. MAP_READ | MAP_WRITE) to verify that `validate()`
    /// returns an `Err` rather than panicking.
    usage_bits: u32,
    /// Whether the buffer should be mapped at creation.
    mapped_at_creation: bool,
    /// Optional label for debugging.
    label: Option<String>,
}

fuzz_target!(|input: BufferInput| {
    // Use `from_bits_truncate` (a bitflags built-in) to create a wgpu usage value
    // from raw bits, then convert to our wrapper type.  This exercises every possible
    // flag combination including invalid ones such as MAP_READ | MAP_WRITE.
    let wgpu_usage = wgpu::BufferUsages::from_bits_truncate(input.usage_bits);
    let usage = BufferUsages::from_wgpu(wgpu_usage);
    let label = input.label.as_deref();

    let desc = BufferDescriptor::new(label, input.size, usage)
        .with_mapped_at_creation(input.mapped_at_creation);

    // validate() must never panic; it may return Ok or Err.
    let _ = desc.validate();
});
