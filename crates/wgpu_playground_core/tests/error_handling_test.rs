use wgpu_playground_core::buffer::{BufferDescriptor, BufferUsages};
use wgpu_playground_core::error::{Error, ErrorFilter, ErrorType};
use wgpu_playground_core::shader::ShaderModule;

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[test]
    fn test_invalid_buffer_size() {
        // Test that zero-sized buffers are rejected
        let descriptor = BufferDescriptor::new(Some("zero_buffer"), 0, BufferUsages::UNIFORM);

        let result = descriptor.validate();
        assert!(result.is_err());

        if let Err(err) = result {
            assert!(err.to_string().contains("size"));
        }
    }

    #[test]
    fn test_invalid_buffer_empty_usage() {
        // Test that buffers with no usage flags are rejected
        let descriptor = BufferDescriptor::new(Some("no_usage"), 256, BufferUsages::empty());

        let result = descriptor.validate();
        assert!(result.is_err());

        if let Err(err) = result {
            assert!(err.to_string().contains("usage"));
        }
    }

    #[test]
    fn test_invalid_buffer_conflicting_usage() {
        // Test that MAP_READ and MAP_WRITE cannot be combined
        let descriptor =
            BufferDescriptor::new(None, 256, BufferUsages::MAP_READ | BufferUsages::MAP_WRITE);

        let result = descriptor.validate();
        assert!(result.is_err());

        if let Err(err) = result {
            assert!(err.to_string().contains("MAP_READ"));
            assert!(err.to_string().contains("MAP_WRITE"));
        }
    }

    #[test]
    fn test_empty_shader_source() {
        // Test that empty shader source is rejected
        let result = ShaderModule::from_source("", Some("empty_shader"));
        assert!(result.is_err());

        if let Err(err) = result {
            assert!(err.to_string().contains("empty"));
        }
    }

    #[test]
    fn test_whitespace_only_shader_source() {
        // Test that whitespace-only shader source is rejected
        let result = ShaderModule::from_source("   \n\t  ", Some("whitespace_shader"));
        assert!(result.is_err());

        if let Err(err) = result {
            assert!(err.to_string().contains("empty"));
        }
    }

    #[test]
    fn test_error_type_classification() {
        // Test error type creation and classification
        let validation_err = Error::validation("test validation error");
        assert_eq!(validation_err.error_type, ErrorType::Validation);
        assert!(validation_err.message.contains("test validation"));

        let oom_err = Error::out_of_memory("out of memory");
        assert_eq!(oom_err.error_type, ErrorType::OutOfMemory);
        assert!(oom_err.message.contains("out of memory"));

        let internal_err = Error::internal("internal error");
        assert_eq!(internal_err.error_type, ErrorType::Internal);
        assert!(internal_err.message.contains("internal"));

        let device_lost_err = Error::device_lost("device lost");
        assert_eq!(device_lost_err.error_type, ErrorType::DeviceLost);
        assert!(device_lost_err.message.contains("device lost"));
    }

    #[test]
    fn test_error_filter_matching() {
        // Test that error filters correctly match error types
        assert!(ErrorFilter::Validation.matches(&ErrorType::Validation));
        assert!(!ErrorFilter::Validation.matches(&ErrorType::OutOfMemory));
        assert!(!ErrorFilter::Validation.matches(&ErrorType::Internal));

        assert!(ErrorFilter::OutOfMemory.matches(&ErrorType::OutOfMemory));
        assert!(!ErrorFilter::OutOfMemory.matches(&ErrorType::Validation));
        assert!(!ErrorFilter::OutOfMemory.matches(&ErrorType::Internal));

        assert!(ErrorFilter::Internal.matches(&ErrorType::Internal));
        assert!(!ErrorFilter::Internal.matches(&ErrorType::Validation));
        assert!(!ErrorFilter::Internal.matches(&ErrorType::OutOfMemory));
    }

    #[test]
    fn test_shader_file_not_found() {
        // Test handling of non-existent shader file
        let result = ShaderModule::from_file("nonexistent_shader.wgsl", Some("test"));
        assert!(result.is_err());

        if let Err(err) = result {
            // Should be a load error (IO error)
            assert!(err.to_string().contains("Failed to load shader"));
        }
    }

    #[test]
    fn test_valid_buffer_configurations() {
        // Test valid buffer configurations pass validation
        let configs = vec![
            (BufferUsages::VERTEX, "vertex"),
            (BufferUsages::INDEX, "index"),
            (BufferUsages::UNIFORM | BufferUsages::COPY_DST, "uniform"),
            (
                BufferUsages::STORAGE | BufferUsages::COPY_SRC,
                "storage",
            ),
            (BufferUsages::MAP_READ | BufferUsages::COPY_DST, "map_read"),
            (
                BufferUsages::MAP_WRITE | BufferUsages::COPY_SRC,
                "map_write",
            ),
        ];

        for (usage, label) in configs {
            let descriptor = BufferDescriptor::new(Some(label), 256, usage);
            assert!(
                descriptor.validate().is_ok(),
                "Configuration {} should be valid",
                label
            );
        }
    }

    #[test]
    fn test_valid_shader_source() {
        // Test valid shader source is accepted
        let shader_source = r#"
            @vertex
            fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
                return vec4<f32>(0.0, 0.0, 0.0, 1.0);
            }
        "#;

        let result = ShaderModule::from_source(shader_source, Some("valid_shader"));
        assert!(result.is_ok());

        let shader = result.unwrap();
        assert_eq!(shader.label(), Some("valid_shader"));
        assert!(shader.source().contains("vs_main"));
    }

    // Integration tests that require GPU would go in a separate test file
    // These tests only validate the error handling logic without GPU access
}
