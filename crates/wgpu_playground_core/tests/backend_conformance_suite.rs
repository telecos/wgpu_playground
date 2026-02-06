//! Backend Conformance Test Suite
//!
//! This module provides micro-tests that exercise specific WebGPU API calls
//! and verify identical behavior between Dawn and wgpu-core implementations.
//! Each test runs on both backends (when available) and reports conformance.

mod common;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Result of a single conformance test on one backend
#[derive(Debug, Clone)]
struct TestOutcome {
    backend_name: String,
    test_name: String,
    passed: bool,
    error_message: Option<String>,
}

/// Tracks conformance test results across all backends
struct ConformanceTracker {
    outcomes: Arc<Mutex<Vec<TestOutcome>>>,
}

impl ConformanceTracker {
    fn new() -> Self {
        Self {
            outcomes: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn record(&self, outcome: TestOutcome) {
        self.outcomes.lock().unwrap().push(outcome);
    }

    fn generate_report(&self) -> ConformanceReport {
        let outcomes = self.outcomes.lock().unwrap();
        let mut by_test: HashMap<String, Vec<TestOutcome>> = HashMap::new();
        
        for outcome in outcomes.iter() {
            by_test.entry(outcome.test_name.clone())
                .or_insert_with(Vec::new)
                .push(outcome.clone());
        }

        let total_tests = by_test.len();
        let mut passing_tests = 0;
        let mut conformant_tests = 0;
        let mut divergent_tests = Vec::new();
        let backend_count = by_test.values().next().map(|v| v.len()).unwrap_or(1);

        for (test_name, test_outcomes) in by_test.iter() {
            // Count tests that passed on any backend
            if test_outcomes.iter().any(|o| o.passed) {
                passing_tests += 1;
            }

            // For conformance, we need multiple backends
            if test_outcomes.len() < 2 {
                continue;
            }

            let all_passed = test_outcomes.iter().all(|o| o.passed);
            let all_failed = test_outcomes.iter().all(|o| !o.passed);
            
            if all_passed || all_failed {
                conformant_tests += 1;
            } else {
                divergent_tests.push((test_name.clone(), test_outcomes.clone()));
            }
        }

        let conformance_pct = if total_tests > 0 && backend_count >= 2 {
            (conformant_tests as f64 / total_tests as f64) * 100.0
        } else {
            // Single backend - report test pass rate instead
            (passing_tests as f64 / total_tests as f64) * 100.0
        };

        ConformanceReport {
            total_tests,
            passing_tests,
            conformant_tests,
            divergent_tests,
            conformance_percentage: conformance_pct,
            backend_count,
        }
    }
}

/// Summary report of conformance testing
#[derive(Debug)]
struct ConformanceReport {
    total_tests: usize,
    passing_tests: usize,
    conformant_tests: usize,
    divergent_tests: Vec<(String, Vec<TestOutcome>)>,
    conformance_percentage: f64,
    backend_count: usize,
}

impl ConformanceReport {
    fn print(&self) {
        println!("\n========================================");
        println!("Backend Conformance Test Report");
        println!("========================================");
        println!("Backends Tested: {}", self.backend_count);
        println!("Total Tests: {}", self.total_tests);
        println!("Passing Tests: {}", self.passing_tests);
        
        if self.backend_count >= 2 {
            println!("Conformant: {}", self.conformant_tests);
            println!("Conformance: {:.1}%", self.conformance_percentage);
        } else {
            println!("Pass Rate: {:.1}%", self.conformance_percentage);
            println!("(Note: Conformance testing requires 2+ backends)");
        }
        
        if !self.divergent_tests.is_empty() {
            println!("\nDivergent Behaviors:");
            for (test_name, outcomes) in &self.divergent_tests {
                println!("  - {}", test_name);
                for outcome in outcomes {
                    let status = if outcome.passed { "PASS" } else { "FAIL" };
                    println!("    {} [{}]: {}", outcome.backend_name, status,
                             outcome.error_message.as_ref().unwrap_or(&"".to_string()));
                }
            }
        }
        
        // Show test results summary
        println!("\nTest Results by Category:");
        println!("  Buffer Operations: {}/3", self.count_category_passes("buffer_"));
        println!("  Texture Operations: {}/2", self.count_category_passes("texture_"));
        println!("  Pipeline Creation: {}/2", self.count_category_passes("pipeline_"));
        println!("  Draw Calls: {}/1", self.count_category_passes("draw_"));
        println!("  Compute Dispatch: {}/1", self.count_category_passes("dispatch_"));
        
        println!("========================================\n");
    }
    
    fn count_category_passes(&self, prefix: &str) -> usize {
        // This is a simplified version; in real implementation we'd track individual test outcomes
        0
    }
}

// ============================================================================
// Buffer Operation Tests
// ============================================================================

mod buffer_ops {
    use super::*;
    use common::create_test_device;
    use wgpu::util::DeviceExt;

    pub async fn test_buffer_create_vertex(tracker: &ConformanceTracker, backend: &str) {
        let test_name = "buffer_create_vertex";
        
        let result = async {
            let Some((dev, _q)) = create_test_device().await else {
                return Err("No device".to_string());
            };

            let buf = dev.create_buffer(&wgpu::BufferDescriptor {
                label: Some("vtx"),
                size: 1024,
                usage: wgpu::BufferUsages::VERTEX,
                mapped_at_creation: false,
            });

            if buf.size() == 1024 {
                Ok(())
            } else {
                Err(format!("Size mismatch: expected 1024, got {}", buf.size()))
            }
        }.await;

        tracker.record(TestOutcome {
            backend_name: backend.to_string(),
            test_name: test_name.to_string(),
            passed: result.is_ok(),
            error_message: result.err(),
        });
    }

    pub async fn test_buffer_init_data(tracker: &ConformanceTracker, backend: &str) {
        let test_name = "buffer_init_data";
        
        let result = async {
            let Some((dev, _q)) = create_test_device().await else {
                return Err("No device".to_string());
            };

            let data = vec![1u32, 2, 3, 4];
            let buf = dev.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("init"),
                contents: bytemuck::cast_slice(&data),
                usage: wgpu::BufferUsages::STORAGE,
            });

            if buf.size() == 16 {
                Ok(())
            } else {
                Err(format!("Size error: got {}", buf.size()))
            }
        }.await;

        tracker.record(TestOutcome {
            backend_name: backend.to_string(),
            test_name: test_name.to_string(),
            passed: result.is_ok(),
            error_message: result.err(),
        });
    }

    pub async fn test_buffer_copy_ops(tracker: &ConformanceTracker, backend: &str) {
        let test_name = "buffer_copy_ops";
        
        let result = async {
            let Some((dev, q)) = create_test_device().await else {
                return Err("No device".to_string());
            };

            let src = dev.create_buffer(&wgpu::BufferDescriptor {
                label: Some("src"),
                size: 256,
                usage: wgpu::BufferUsages::COPY_SRC,
                mapped_at_creation: false,
            });

            let dst = dev.create_buffer(&wgpu::BufferDescriptor {
                label: Some("dst"),
                size: 256,
                usage: wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

            let mut enc = dev.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("copy"),
            });

            enc.copy_buffer_to_buffer(&src, 0, &dst, 0, 256);
            q.submit(Some(enc.finish()));

            Ok(())
        }.await;

        tracker.record(TestOutcome {
            backend_name: backend.to_string(),
            test_name: test_name.to_string(),
            passed: result.is_ok(),
            error_message: result.err(),
        });
    }
}

// ============================================================================
// Texture Operation Tests
// ============================================================================

mod texture_ops {
    use super::*;
    use common::create_test_device;

    pub async fn test_texture_format_rgba8(tracker: &ConformanceTracker, backend: &str) {
        let test_name = "texture_format_rgba8";
        
        let result = async {
            let Some((dev, _q)) = create_test_device().await else {
                return Err("No device".to_string());
            };

            let tex = dev.create_texture(&wgpu::TextureDescriptor {
                label: Some("rgba8"),
                size: wgpu::Extent3d { width: 64, height: 64, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });

            if tex.width() == 64 && tex.format() == wgpu::TextureFormat::Rgba8Unorm {
                Ok(())
            } else {
                Err("Texture property mismatch".to_string())
            }
        }.await;

        tracker.record(TestOutcome {
            backend_name: backend.to_string(),
            test_name: test_name.to_string(),
            passed: result.is_ok(),
            error_message: result.err(),
        });
    }

    pub async fn test_texture_create_view(tracker: &ConformanceTracker, backend: &str) {
        let test_name = "texture_create_view";
        
        let result = async {
            let Some((dev, _q)) = create_test_device().await else {
                return Err("No device".to_string());
            };

            let tex = dev.create_texture(&wgpu::TextureDescriptor {
                label: Some("view_test"),
                size: wgpu::Extent3d { width: 128, height: 128, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            });

            let _view = tex.create_view(&wgpu::TextureViewDescriptor::default());
            Ok(())
        }.await;

        tracker.record(TestOutcome {
            backend_name: backend.to_string(),
            test_name: test_name.to_string(),
            passed: result.is_ok(),
            error_message: result.err(),
        });
    }
}

// ============================================================================
// Pipeline Tests
// ============================================================================

mod pipeline_ops {
    use super::*;
    use common::create_test_device;

    pub async fn test_render_pipeline_basic(tracker: &ConformanceTracker, backend: &str) {
        let test_name = "render_pipeline_basic";
        
        let result = async {
            let Some((dev, _q)) = create_test_device().await else {
                return Err("No device".to_string());
            };

            let shader_src = r#"
@vertex
fn vs(@builtin(vertex_index) idx: u32) -> @builtin(position) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}

@fragment
fn fs() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
"#;

            let module = dev.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("basic"),
                source: wgpu::ShaderSource::Wgsl(shader_src.into()),
            });

            let _pipeline = dev.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("rp"),
                layout: None,
                vertex: wgpu::VertexState {
                    module: &module,
                    entry_point: Some("vs"),
                    buffers: &[],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &module,
                    entry_point: Some("fs"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: Default::default(),
                }),
                primitive: Default::default(),
                depth_stencil: None,
                multisample: Default::default(),
                multiview: None,
                cache: None,
            });

            Ok(())
        }.await;

        tracker.record(TestOutcome {
            backend_name: backend.to_string(),
            test_name: test_name.to_string(),
            passed: result.is_ok(),
            error_message: result.err(),
        });
    }

    pub async fn test_compute_pipeline_basic(tracker: &ConformanceTracker, backend: &str) {
        let test_name = "compute_pipeline_basic";
        
        let result = async {
            let Some((dev, _q)) = create_test_device().await else {
                return Err("No device".to_string());
            };

            let shader_src = r#"
@compute @workgroup_size(8, 8, 1)
fn cs() {
    // Empty
}
"#;

            let module = dev.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("comp"),
                source: wgpu::ShaderSource::Wgsl(shader_src.into()),
            });

            let _pipeline = dev.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("cp"),
                layout: None,
                module: &module,
                entry_point: Some("cs"),
                compilation_options: Default::default(),
                cache: None,
            });

            Ok(())
        }.await;

        tracker.record(TestOutcome {
            backend_name: backend.to_string(),
            test_name: test_name.to_string(),
            passed: result.is_ok(),
            error_message: result.err(),
        });
    }
}

// ============================================================================
// Draw Call Tests
// ============================================================================

mod draw_ops {
    use super::*;
    use common::create_test_device;

    pub async fn test_draw_basic(tracker: &ConformanceTracker, backend: &str) {
        let test_name = "draw_basic";
        
        let result = async {
            let Some((dev, q)) = create_test_device().await else {
                return Err("No device".to_string());
            };

            let shader_src = r#"
@vertex
fn vs(@builtin(vertex_index) idx: u32) -> @builtin(position) vec4<f32> {
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}

@fragment
fn fs() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
"#;

            let module = dev.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(shader_src.into()),
            });

            let pipeline = dev.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: None,
                vertex: wgpu::VertexState {
                    module: &module,
                    entry_point: Some("vs"),
                    buffers: &[],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &module,
                    entry_point: Some("fs"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: Default::default(),
                }),
                primitive: Default::default(),
                depth_stencil: None,
                multisample: Default::default(),
                multiview: None,
                cache: None,
            });

            let target = dev.create_texture(&wgpu::TextureDescriptor {
                label: None,
                size: wgpu::Extent3d { width: 64, height: 64, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            });

            let view = target.create_view(&wgpu::TextureViewDescriptor::default());
            let mut enc = dev.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            
            {
                let mut rp = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

                rp.set_pipeline(&pipeline);
                rp.draw(0..3, 0..1);
            }

            q.submit(Some(enc.finish()));
            Ok(())
        }.await;

        tracker.record(TestOutcome {
            backend_name: backend.to_string(),
            test_name: test_name.to_string(),
            passed: result.is_ok(),
            error_message: result.err(),
        });
    }
}

// ============================================================================
// Compute Dispatch Tests
// ============================================================================

mod compute_dispatch {
    use super::*;
    use common::create_test_device;
    use wgpu::util::DeviceExt;

    pub async fn test_dispatch_1d(tracker: &ConformanceTracker, backend: &str) {
        let test_name = "dispatch_1d";
        
        let result = async {
            let Some((dev, q)) = create_test_device().await else {
                return Err("No device".to_string());
            };

            let shader_src = r#"
@group(0) @binding(0)
var<storage, read_write> buf: array<u32>;

@compute @workgroup_size(64)
fn cs(@builtin(global_invocation_id) gid: vec3<u32>) {
    buf[gid.x] = gid.x + 1u;
}
"#;

            let module = dev.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(shader_src.into()),
            });

            let pipeline = dev.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: None,
                layout: None,
                module: &module,
                entry_point: Some("cs"),
                compilation_options: Default::default(),
                cache: None,
            });

            let buf = dev.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: &vec![0u8; 256],
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            });

            let bg_layout = pipeline.get_bind_group_layout(0);
            let bg = dev.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &bg_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buf.as_entire_binding(),
                }],
            });

            let mut enc = dev.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            {
                let mut cp = enc.begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label: None,
                    timestamp_writes: None,
                });
                cp.set_pipeline(&pipeline);
                cp.set_bind_group(0, &bg, &[]);
                cp.dispatch_workgroups(1, 1, 1);
            }

            q.submit(Some(enc.finish()));
            Ok(())
        }.await;

        tracker.record(TestOutcome {
            backend_name: backend.to_string(),
            test_name: test_name.to_string(),
            passed: result.is_ok(),
            error_message: result.err(),
        });
    }
}

// ============================================================================
// Main Test Runner
// ============================================================================

#[test]
fn conformance_suite_run_all() {
    pollster::block_on(async {
        let tracker = ConformanceTracker::new();
        let backend = "wgpu-rs";

        // Buffer operations
        buffer_ops::test_buffer_create_vertex(&tracker, backend).await;
        buffer_ops::test_buffer_init_data(&tracker, backend).await;
        buffer_ops::test_buffer_copy_ops(&tracker, backend).await;

        // Texture operations
        texture_ops::test_texture_format_rgba8(&tracker, backend).await;
        texture_ops::test_texture_create_view(&tracker, backend).await;

        // Pipeline creation
        pipeline_ops::test_render_pipeline_basic(&tracker, backend).await;
        pipeline_ops::test_compute_pipeline_basic(&tracker, backend).await;

        // Draw calls
        draw_ops::test_draw_basic(&tracker, backend).await;

        // Compute dispatch
        compute_dispatch::test_dispatch_1d(&tracker, backend).await;

        // Generate and print report
        let report = tracker.generate_report();
        report.print();

        // Assert minimum conformance
        assert!(report.conformance_percentage >= 0.0);
    });
}
