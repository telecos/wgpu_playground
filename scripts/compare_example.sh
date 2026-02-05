#!/bin/bash
# Simple script to run examples with both backends and compare

EXAMPLE=$1
OUTPUT_DIR="backend_comparison_output"

mkdir -p "$OUTPUT_DIR"

echo "Running $EXAMPLE with wgpu-rs..."
WEBGPU_IMPL=wgpu cargo run --package wgpu_playground_examples --example "$EXAMPLE" 2>&1 | tee "$OUTPUT_DIR/${EXAMPLE}_wgpu.log"

echo "Running $EXAMPLE with Dawn..."
WEBGPU_IMPL=dawn cargo run --package wgpu_playground_examples --example "$EXAMPLE" --features dawn 2>&1 | tee "$OUTPUT_DIR/${EXAMPLE}_dawn.log"

echo "Logs saved to $OUTPUT_DIR"
echo "Compare with: diff $OUTPUT_DIR/${EXAMPLE}_wgpu.log $OUTPUT_DIR/${EXAMPLE}_dawn.log"
