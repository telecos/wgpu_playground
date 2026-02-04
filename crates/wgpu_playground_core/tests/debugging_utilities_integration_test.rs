/// Integration tests for GPU debugging utilities
/// 
/// These tests verify that the buffer inspector, texture inspector,
/// and pipeline debugger work correctly together with the rest of
/// the playground infrastructure.

#[cfg(test)]
mod buffer_inspector_tests {
    use wgpu_playground_core::buffer_inspector::{BufferInspector, DataFormat};

    #[test]
    fn test_buffer_inspector_integration() {
        let mut inspector = BufferInspector::new();
        
        // Test with sample data that represents GPU buffer contents
        let test_data = vec![
            0x00, 0x00, 0x80, 0x3f, // 1.0 as f32 in little-endian
            0x00, 0x00, 0x00, 0x40, // 2.0 as f32
            0x00, 0x00, 0x40, 0x40, // 3.0 as f32
            0x00, 0x00, 0x80, 0x40, // 4.0 as f32
        ];
        
        inspector.load_data(test_data.clone());
        assert_eq!(inspector.data().len(), 16);
        
        // Test different format views
        inspector.set_format(DataFormat::Hex);
        let hex_output = inspector.format_data();
        assert!(hex_output.contains("00 00 80 3f"));
        
        inspector.set_format(DataFormat::Float32);
        let float_output = inspector.format_data();
        assert!(float_output.contains("1.0"));
        assert!(float_output.contains("2.0"));
        assert!(float_output.contains("3.0"));
        assert!(float_output.contains("4.0"));
    }

    #[test]
    fn test_buffer_inspector_error_handling() {
        let mut inspector = BufferInspector::new();
        inspector.set_error("Failed to map buffer".to_string());
        assert!(inspector.has_error());
        assert_eq!(inspector.error_message(), Some("Failed to map buffer"));
    }

    #[test]
    fn test_buffer_inspector_different_formats() {
        let mut inspector = BufferInspector::new();
        
        // Test integer data
        let int_data = vec![
            0x2A, 0x00, 0x00, 0x00, // 42 as i32
            0xFF, 0xFF, 0xFF, 0xFF, // -1 as i32
        ];
        
        inspector.load_data(int_data);
        
        inspector.set_format(DataFormat::Int32);
        let output = inspector.format_data();
        assert!(output.contains("42"));
        assert!(output.contains("-1"));
        
        inspector.set_format(DataFormat::Uint32);
        let output = inspector.format_data();
        assert!(output.contains("42"));
        assert!(output.contains("4294967295")); // -1 as u32
    }
}

#[cfg(test)]
mod texture_inspector_tests {
    use wgpu_playground_core::texture_inspector::{TextureData, TextureInspector};
    use wgpu::TextureFormat;

    fn create_test_texture_data() -> TextureData {
        TextureData {
            width: 4,
            height: 4,
            format: TextureFormat::Rgba8Unorm,
            data: vec![255; 64], // 4x4 RGBA white texture
        }
    }

    #[test]
    fn test_texture_inspector_integration() {
        let mut inspector = TextureInspector::new();
        let texture_data = create_test_texture_data();
        
        inspector.load_texture(texture_data);
        assert!(inspector.texture_data().is_some());
        
        let loaded = inspector.texture_data().unwrap();
        assert_eq!(loaded.width, 4);
        assert_eq!(loaded.height, 4);
        assert_eq!(loaded.data.len(), 64);
    }

    #[test]
    fn test_texture_inspector_zoom() {
        let mut inspector = TextureInspector::new();
        
        inspector.set_zoom(2.5);
        assert_eq!(inspector.get_zoom(), 2.5);
        
        // Test zoom clamping
        inspector.set_zoom(15.0);
        assert_eq!(inspector.get_zoom(), 10.0); // Max zoom
        
        inspector.set_zoom(0.01);
        assert_eq!(inspector.get_zoom(), 0.1); // Min zoom
    }

    #[test]
    fn test_texture_inspector_mip_levels() {
        let mut inspector = TextureInspector::new();
        let texture_data = create_test_texture_data();
        inspector.load_texture(texture_data);
        
        inspector.set_mip_level(0);
        assert_eq!(inspector.get_mip_level(), 0);
        
        inspector.set_mip_level(2);
        assert_eq!(inspector.get_mip_level(), 2);
    }

    #[test]
    fn test_texture_inspector_clear() {
        let mut inspector = TextureInspector::new();
        let texture_data = create_test_texture_data();
        inspector.load_texture(texture_data);
        
        assert!(inspector.texture_data().is_some());
        
        inspector.clear();
        assert!(inspector.texture_data().is_none());
        assert_eq!(inspector.get_mip_level(), 0);
    }
}

#[cfg(test)]
mod pipeline_debugger_tests {
    use wgpu_playground_core::pipeline_debugger::{
        PipelineConfig, PipelineDebugInfo, PipelineDebugger, ShaderInfo, ShaderStage,
        ValidationMessage, ValidationSeverity,
    };

    fn create_test_pipeline_info() -> PipelineDebugInfo {
        PipelineDebugInfo {
            config: PipelineConfig {
                label: Some("Test Pipeline".to_string()),
                topology: Some("TriangleList".to_string()),
                has_depth_stencil: true,
                color_target_count: 1,
                has_blending: false,
                sample_count: 1,
            },
            shaders: vec![
                ShaderInfo {
                    source: "@vertex\nfn main() -> @builtin(position) vec4<f32> { return vec4<f32>(0.0); }".to_string(),
                    entry_point: "main".to_string(),
                    stage: ShaderStage::Vertex,
                },
            ],
            validation_messages: vec![],
        }
    }

    #[test]
    fn test_pipeline_debugger_integration() {
        let mut debugger = PipelineDebugger::new();
        let pipeline_info = create_test_pipeline_info();
        
        debugger.load_pipeline(pipeline_info);
        assert!(debugger.debug_info().is_some());
        
        let info = debugger.debug_info().unwrap();
        assert_eq!(info.shaders.len(), 1);
        assert_eq!(info.config.color_target_count, 1);
    }

    #[test]
    fn test_pipeline_debugger_validation_messages() {
        let mut debugger = PipelineDebugger::new();
        let mut pipeline_info = create_test_pipeline_info();
        
        pipeline_info.validation_messages.push(ValidationMessage {
            severity: ValidationSeverity::Warning,
            message: "Unused binding".to_string(),
        });
        pipeline_info.validation_messages.push(ValidationMessage {
            severity: ValidationSeverity::Error,
            message: "Invalid shader syntax".to_string(),
        });
        
        debugger.load_pipeline(pipeline_info);
        
        assert_eq!(debugger.count_messages_by_severity(ValidationSeverity::Warning), 1);
        assert_eq!(debugger.count_messages_by_severity(ValidationSeverity::Error), 1);
        assert_eq!(debugger.count_messages_by_severity(ValidationSeverity::Info), 0);
    }

    #[test]
    fn test_pipeline_debugger_shader_stages() {
        let mut debugger = PipelineDebugger::new();
        let mut pipeline_info = create_test_pipeline_info();
        
        // Add fragment shader
        pipeline_info.shaders.push(ShaderInfo {
            source: "@fragment\nfn main() -> @location(0) vec4<f32> { return vec4<f32>(1.0); }".to_string(),
            entry_point: "main".to_string(),
            stage: ShaderStage::Fragment,
        });
        
        debugger.load_pipeline(pipeline_info);
        
        let info = debugger.debug_info().unwrap();
        assert_eq!(info.shaders.len(), 2);
        assert_eq!(info.shaders[0].stage, ShaderStage::Vertex);
        assert_eq!(info.shaders[1].stage, ShaderStage::Fragment);
    }

    #[test]
    fn test_pipeline_debugger_clear() {
        let mut debugger = PipelineDebugger::new();
        let pipeline_info = create_test_pipeline_info();
        
        debugger.load_pipeline(pipeline_info);
        assert!(debugger.debug_info().is_some());
        
        debugger.clear();
        assert!(debugger.debug_info().is_none());
    }
}

#[cfg(test)]
mod debugging_utilities_integration_tests {
    use wgpu_playground_core::buffer_inspector::{BufferInspector, DataFormat};
    use wgpu_playground_core::pipeline_debugger::{
        PipelineConfig, PipelineDebugInfo, PipelineDebugger, ShaderInfo, ShaderStage,
    };
    use wgpu_playground_core::texture_inspector::{TextureData, TextureInspector};
    use wgpu::TextureFormat;

    #[test]
    fn test_all_debugging_utilities_together() {
        // Create all three debugging utilities
        let mut buffer_inspector = BufferInspector::new();
        let mut texture_inspector = TextureInspector::new();
        let mut pipeline_debugger = PipelineDebugger::new();

        // Load test data into buffer inspector
        let buffer_data = vec![0x00, 0x00, 0x80, 0x3f]; // 1.0 as f32
        buffer_inspector.load_data(buffer_data);
        buffer_inspector.set_format(DataFormat::Float32);

        // Load test texture into texture inspector
        let texture_data = TextureData {
            width: 2,
            height: 2,
            format: TextureFormat::Rgba8Unorm,
            data: vec![255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 0, 255],
        };
        texture_inspector.load_texture(texture_data);
        texture_inspector.set_zoom(1.5);

        // Load test pipeline into pipeline debugger
        let pipeline_info = PipelineDebugInfo {
            config: PipelineConfig {
                label: Some("Main Pipeline".to_string()),
                topology: Some("TriangleList".to_string()),
                has_depth_stencil: true,
                color_target_count: 1,
                has_blending: false,
                sample_count: 1,
            },
            shaders: vec![ShaderInfo {
                source: "@vertex\nfn main() {}".to_string(),
                entry_point: "main".to_string(),
                stage: ShaderStage::Vertex,
            }],
            validation_messages: vec![],
        };
        pipeline_debugger.load_pipeline(pipeline_info);

        // Verify all utilities are functioning
        assert!(!buffer_inspector.data().is_empty());
        assert!(texture_inspector.texture_data().is_some());
        assert!(pipeline_debugger.debug_info().is_some());

        // Verify formatted output
        let buffer_output = buffer_inspector.format_data();
        assert!(buffer_output.contains("1.0"));

        // Verify texture properties
        let texture = texture_inspector.texture_data().unwrap();
        assert_eq!(texture.width, 2);
        assert_eq!(texture.height, 2);

        // Verify pipeline properties
        let pipeline = pipeline_debugger.debug_info().unwrap();
        assert_eq!(pipeline.shaders.len(), 1);
    }

    #[test]
    fn test_debugging_utilities_error_states() {
        let mut buffer_inspector = BufferInspector::new();
        let mut texture_inspector = TextureInspector::new();
        let mut pipeline_debugger = PipelineDebugger::new();

        // Test error handling
        buffer_inspector.set_error("Buffer mapping failed".to_string());
        texture_inspector.set_error("Texture read failed".to_string());
        pipeline_debugger.set_error("Pipeline compilation failed".to_string());

        assert!(buffer_inspector.has_error());
        assert!(texture_inspector.has_error());
        assert!(pipeline_debugger.has_error());

        assert_eq!(
            buffer_inspector.error_message(),
            Some("Buffer mapping failed")
        );
        assert_eq!(
            texture_inspector.error_message(),
            Some("Texture read failed")
        );
        assert_eq!(
            pipeline_debugger.error_message(),
            Some("Pipeline compilation failed")
        );
    }
}
