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
            (BufferUsages::STORAGE | BufferUsages::COPY_SRC, "storage"),
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

    #[test]
    fn test_error_handler_callback_invocation() {
        use std::sync::{Arc, Mutex};
        use wgpu_playground_core::error::ErrorHandler;

        let mut handler = ErrorHandler::new();
        let error_received = Arc::new(Mutex::new(false));
        let error_received_clone = Arc::clone(&error_received);

        handler.on_error(move |err| {
            assert_eq!(err.error_type, ErrorType::Validation);
            *error_received_clone.lock().unwrap() = true;
        });

        handler.handle_error(Error::validation("test error"));
        assert!(*error_received.lock().unwrap());
    }

    #[test]
    fn test_error_handler_multiple_callbacks() {
        use std::sync::{Arc, Mutex};
        use wgpu_playground_core::error::ErrorHandler;

        let mut handler = ErrorHandler::new();
        let counter = Arc::new(Mutex::new(0));

        let counter1 = Arc::clone(&counter);
        handler.on_error(move |_| {
            *counter1.lock().unwrap() += 1;
        });

        let counter2 = Arc::clone(&counter);
        handler.on_error(move |_| {
            *counter2.lock().unwrap() += 10;
        });

        handler.handle_error(Error::validation("test"));
        assert_eq!(*counter.lock().unwrap(), 11);
    }

    #[test]
    fn test_device_lost_reason_conversion() {
        use wgpu_playground_core::error::DeviceLostReason;

        let unknown = DeviceLostReason::from(wgpu::DeviceLostReason::Unknown);
        assert_eq!(unknown, DeviceLostReason::Unknown);
        assert_eq!(unknown.to_string(), "Unknown");

        let destroyed = DeviceLostReason::from(wgpu::DeviceLostReason::Destroyed);
        assert_eq!(destroyed, DeviceLostReason::Destroyed);
        assert_eq!(destroyed.to_string(), "Destroyed");
    }

    #[test]
    fn test_out_of_bounds_buffer_size() {
        // Test that buffer size larger than reasonable limits would be caught
        // This tests the validation logic for extreme buffer sizes
        let u64_max_size = u64::MAX;
        let descriptor =
            BufferDescriptor::new(Some("huge_buffer"), u64_max_size, BufferUsages::UNIFORM);

        // The validation should pass (size validation only checks for zero)
        // The actual GPU would reject this during creation
        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn test_buffer_usage_combinations() {
        // Test various valid usage combinations
        let valid_usages = vec![
            (
                BufferUsages::VERTEX | BufferUsages::COPY_DST,
                "vertex+copy_dst",
            ),
            (
                BufferUsages::INDEX | BufferUsages::COPY_DST,
                "index+copy_dst",
            ),
            (
                BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                "uniform+copy_dst",
            ),
            (
                BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
                "storage+copy",
            ),
            (
                BufferUsages::INDIRECT | BufferUsages::COPY_DST,
                "indirect+copy_dst",
            ),
            (
                BufferUsages::MAP_READ | BufferUsages::COPY_DST,
                "map_read+copy_dst",
            ),
            (
                BufferUsages::MAP_WRITE | BufferUsages::COPY_SRC,
                "map_write+copy_src",
            ),
        ];

        for (usage, label) in valid_usages {
            let descriptor = BufferDescriptor::new(Some(label), 256, usage);
            assert!(
                descriptor.validate().is_ok(),
                "Usage combination {} should be valid",
                label
            );
        }
    }

    #[test]
    fn test_error_filter_to_wgpu_conversion() {
        // Test that our error filters convert correctly to wgpu error filters
        assert_eq!(
            ErrorFilter::Validation.to_wgpu(),
            wgpu::ErrorFilter::Validation
        );
        assert_eq!(
            ErrorFilter::OutOfMemory.to_wgpu(),
            wgpu::ErrorFilter::OutOfMemory
        );
        assert_eq!(ErrorFilter::Internal.to_wgpu(), wgpu::ErrorFilter::Internal);
    }

    #[test]
    fn test_error_conversion_functionality() {
        // Test that our error conversion preserves error types correctly
        // This tests the From<wgpu::Error> implementation indirectly
        // by verifying the error type classification works
        let validation_err = Error::validation("test validation");
        assert_eq!(validation_err.error_type, ErrorType::Validation);
        assert!(validation_err.to_string().contains("Validation error"));

        let oom_err = Error::out_of_memory("test oom");
        assert_eq!(oom_err.error_type, ErrorType::OutOfMemory);
        assert!(oom_err.to_string().contains("OutOfMemory error"));

        let internal_err = Error::internal("test internal");
        assert_eq!(internal_err.error_type, ErrorType::Internal);
        assert!(internal_err.to_string().contains("Internal error"));
    }

    #[test]
    fn test_buffer_descriptor_validation_edge_cases() {
        // Test edge case: buffer with size 1 (minimum valid size)
        let descriptor = BufferDescriptor::new(Some("tiny"), 1, BufferUsages::UNIFORM);
        assert!(descriptor.validate().is_ok());

        // Test edge case: all valid single usage flags
        let single_usages = vec![
            BufferUsages::VERTEX,
            BufferUsages::INDEX,
            BufferUsages::UNIFORM,
            BufferUsages::STORAGE,
            BufferUsages::INDIRECT,
            BufferUsages::COPY_SRC,
            BufferUsages::COPY_DST,
            BufferUsages::MAP_READ,
            BufferUsages::MAP_WRITE,
            BufferUsages::QUERY_RESOLVE,
        ];

        for usage in single_usages {
            let descriptor = BufferDescriptor::new(Some("test"), 64, usage);
            assert!(
                descriptor.validate().is_ok(),
                "Single usage flag should be valid"
            );
        }
    }

    #[test]
    fn test_shader_error_messages() {
        // Test that shader errors contain helpful information
        let result = ShaderModule::from_source("", Some("empty"));
        assert!(result.is_err());
        if let Err(err) = result {
            let msg = err.to_string();
            // Error message should be informative
            assert!(
                msg.contains("empty") || msg.contains("source"),
                "Error message should mention empty or source"
            );
        }

        let result = ShaderModule::from_file("/nonexistent/path/shader.wgsl", Some("missing"));
        assert!(result.is_err());
        if let Err(err) = result {
            let msg = err.to_string();
            // Error message should mention loading or file issue
            assert!(
                msg.contains("Failed to load") || msg.contains("shader"),
                "Error message should mention loading failure"
            );
        }
    }

    #[test]
    fn test_error_type_equality() {
        // Test ErrorType equality comparisons
        assert_eq!(ErrorType::Validation, ErrorType::Validation);
        assert_eq!(ErrorType::OutOfMemory, ErrorType::OutOfMemory);
        assert_eq!(ErrorType::Internal, ErrorType::Internal);
        assert_eq!(ErrorType::DeviceLost, ErrorType::DeviceLost);

        assert_ne!(ErrorType::Validation, ErrorType::OutOfMemory);
        assert_ne!(ErrorType::Internal, ErrorType::DeviceLost);
    }

    #[test]
    fn test_error_filter_non_matching() {
        // Test that error filters correctly reject non-matching types
        assert!(!ErrorFilter::Validation.matches(&ErrorType::DeviceLost));
        assert!(!ErrorFilter::OutOfMemory.matches(&ErrorType::DeviceLost));
        assert!(!ErrorFilter::Internal.matches(&ErrorType::DeviceLost));
    }

    // Integration tests that require GPU would go in a separate test file
    // These tests only validate the error handling logic without GPU access
}
