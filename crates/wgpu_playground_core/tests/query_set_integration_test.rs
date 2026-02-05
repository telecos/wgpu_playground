mod common;

use common::{create_test_device, create_test_device_with_features};
use wgpu_playground_core::buffer::{BufferDescriptor, BufferOps, BufferUsages};
use wgpu_playground_core::query_set::{QuerySetDescriptor, QuerySetOps, QueryType};

// Helper function to create a test device and queue with timestamp query support
async fn create_test_device_with_timestamp() -> Option<(wgpu::Device, wgpu::Queue)> {
    create_test_device_with_features(wgpu::Features::TIMESTAMP_QUERY).await
}

// Helper function to create a test device with support for writing timestamps inside encoders
async fn create_test_device_with_timestamp_writes() -> Option<(wgpu::Device, wgpu::Queue)> {
    // TIMESTAMP_QUERY_INSIDE_ENCODERS implies TIMESTAMP_QUERY
    // We need this feature to call write_timestamp on command encoders
    create_test_device_with_features(wgpu::Features::TIMESTAMP_QUERY_INSIDE_ENCODERS).await
}

#[test]
fn test_timestamp_query_set_creation() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device_with_timestamp().await else {
            eprintln!(
                "Skipping test: No GPU adapter available or TIMESTAMP_QUERY feature not supported"
            );
            return;
        };

        let descriptor =
            QuerySetDescriptor::new(Some("timestamp_queries"), QueryType::Timestamp, 4);

        let query_set = descriptor.create_query_set(&device).unwrap();
        // The query set is successfully created
        assert_eq!(descriptor.count(), 4);
        drop(query_set);
    });
}

#[test]
fn test_occlusion_query_set_creation() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let descriptor =
            QuerySetDescriptor::new(Some("occlusion_queries"), QueryType::Occlusion, 2);

        let query_set = descriptor.create_query_set(&device).unwrap();
        assert_eq!(descriptor.count(), 2);
        drop(query_set);
    });
}

#[test]
fn test_query_set_creation_with_zero_count() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device().await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        let descriptor = QuerySetDescriptor::new(Some("invalid_queries"), QueryType::Timestamp, 0);

        let result = descriptor.create_query_set(&device);
        assert!(result.is_err());
    });
}

#[test]
#[cfg_attr(
    all(target_os = "linux", target_env = "gnu"),
    ignore = "TIMESTAMP_QUERY_INSIDE_ENCODERS not supported in CI with lavapipe software rendering"
)]
fn test_timestamp_write() {
    pollster::block_on(async {
        let Some((device, _queue)) = create_test_device_with_timestamp_writes().await else {
            eprintln!(
                "Skipping test: No GPU adapter available or TIMESTAMP_QUERY_INSIDE_ENCODERS feature not supported"
            );
            return;
        };

        // Create query set
        let query_set_desc = QuerySetDescriptor::new(Some("timestamps"), QueryType::Timestamp, 2);
        let query_set = query_set_desc.create_query_set(&device).unwrap();

        // Create command encoder
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("timestamp_encoder"),
        });

        // Write timestamps
        QuerySetOps::write_timestamp(&mut encoder, &query_set, 0);
        QuerySetOps::write_timestamp(&mut encoder, &query_set, 1);

        encoder.finish();
    });
}

#[test]
#[cfg_attr(
    all(target_os = "linux", target_env = "gnu"),
    ignore = "Hangs in CI with lavapipe software rendering"
)]
fn test_query_set_resolution() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device_with_timestamp().await else {
            eprintln!(
                "Skipping test: No GPU adapter available or TIMESTAMP_QUERY feature not supported"
            );
            return;
        };

        // Create query set
        let query_set_desc = QuerySetDescriptor::new(Some("queries"), QueryType::Timestamp, 2);
        let query_set = query_set_desc.create_query_set(&device).unwrap();

        // Create buffer to receive query results (8 bytes per timestamp)
        let buffer_desc = BufferDescriptor::new(
            Some("query_results"),
            16, // 2 queries * 8 bytes
            BufferUsages::QUERY_RESOLVE | BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        );
        let buffer = buffer_desc.create_buffer(&device).unwrap();

        // Create command encoder
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("query_encoder"),
        });

        // Write timestamps
        QuerySetOps::write_timestamp(&mut encoder, &query_set, 0);
        QuerySetOps::write_timestamp(&mut encoder, &query_set, 1);

        // Resolve queries to buffer
        QuerySetOps::resolve_query_set(&mut encoder, &query_set, 0..2, &buffer, 0);

        let command_buffer = encoder.finish();
        queue.submit(Some(command_buffer));

        // Map buffer and verify we can read it
        BufferOps::map_read(&buffer).await.unwrap();
        let view = BufferOps::get_mapped_range(&buffer);

        // Verify the buffer has the expected size
        assert_eq!(view.len(), 16);

        drop(view);
        BufferOps::unmap(&buffer);
    });
}

#[test]
#[cfg_attr(
    all(target_os = "linux", target_env = "gnu"),
    ignore = "Hangs in CI with lavapipe software rendering"
)]
fn test_query_set_multiple_resolutions() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device_with_timestamp().await else {
            eprintln!(
                "Skipping test: No GPU adapter available or TIMESTAMP_QUERY feature not supported"
            );
            return;
        };

        // Create query set with 4 queries
        let query_set_desc = QuerySetDescriptor::new(Some("queries"), QueryType::Timestamp, 4);
        let query_set = query_set_desc.create_query_set(&device).unwrap();

        // Create buffer to receive query results (8 bytes per timestamp)
        let buffer_desc = BufferDescriptor::new(
            Some("query_results"),
            32, // 4 queries * 8 bytes
            BufferUsages::QUERY_RESOLVE | BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        );
        let buffer = buffer_desc.create_buffer(&device).unwrap();

        // Create command encoder
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("query_encoder"),
        });

        // Write timestamps
        QuerySetOps::write_timestamp(&mut encoder, &query_set, 0);
        QuerySetOps::write_timestamp(&mut encoder, &query_set, 1);
        QuerySetOps::write_timestamp(&mut encoder, &query_set, 2);
        QuerySetOps::write_timestamp(&mut encoder, &query_set, 3);

        // Resolve queries to buffer
        QuerySetOps::resolve_query_set(&mut encoder, &query_set, 0..4, &buffer, 0);

        let command_buffer = encoder.finish();
        queue.submit(Some(command_buffer));

        // Map buffer and verify we can read it
        BufferOps::map_read(&buffer).await.unwrap();
        let view = BufferOps::get_mapped_range(&buffer);

        // Verify the buffer has the expected size
        assert_eq!(view.len(), 32);

        drop(view);
        BufferOps::unmap(&buffer);
    });
}

#[test]
#[cfg_attr(
    all(target_os = "linux", target_env = "gnu"),
    ignore = "Hangs in CI with lavapipe software rendering"
)]
fn test_query_set_partial_resolution() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device_with_timestamp().await else {
            eprintln!(
                "Skipping test: No GPU adapter available or TIMESTAMP_QUERY feature not supported"
            );
            return;
        };

        // Create query set with 4 queries
        let query_set_desc = QuerySetDescriptor::new(Some("queries"), QueryType::Timestamp, 4);
        let query_set = query_set_desc.create_query_set(&device).unwrap();

        // Create buffer to receive query results (8 bytes per timestamp, only 2 queries)
        let buffer_desc = BufferDescriptor::new(
            Some("query_results"),
            16, // 2 queries * 8 bytes
            BufferUsages::QUERY_RESOLVE | BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        );
        let buffer = buffer_desc.create_buffer(&device).unwrap();

        // Create command encoder
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("query_encoder"),
        });

        // Write timestamps to all 4 queries
        QuerySetOps::write_timestamp(&mut encoder, &query_set, 0);
        QuerySetOps::write_timestamp(&mut encoder, &query_set, 1);
        QuerySetOps::write_timestamp(&mut encoder, &query_set, 2);
        QuerySetOps::write_timestamp(&mut encoder, &query_set, 3);

        // Resolve only queries 1 and 2 to buffer
        QuerySetOps::resolve_query_set(&mut encoder, &query_set, 1..3, &buffer, 0);

        let command_buffer = encoder.finish();
        queue.submit(Some(command_buffer));

        // Map buffer and verify we can read it
        BufferOps::map_read(&buffer).await.unwrap();
        let view = BufferOps::get_mapped_range(&buffer);

        // Verify the buffer has the expected size
        assert_eq!(view.len(), 16);

        drop(view);
        BufferOps::unmap(&buffer);
    });
}

#[test]
#[cfg_attr(
    all(target_os = "linux", target_env = "gnu"),
    ignore = "Hangs in CI with lavapipe software rendering"
)]
fn test_query_set_buffer_offset() {
    pollster::block_on(async {
        let Some((device, queue)) = create_test_device_with_timestamp().await else {
            eprintln!(
                "Skipping test: No GPU adapter available or TIMESTAMP_QUERY feature not supported"
            );
            return;
        };

        // Create query set
        let query_set_desc = QuerySetDescriptor::new(Some("queries"), QueryType::Timestamp, 2);
        let query_set = query_set_desc.create_query_set(&device).unwrap();

        // Create larger buffer to test offset
        let buffer_desc = BufferDescriptor::new(
            Some("query_results"),
            32, // Extra space for testing offset
            BufferUsages::QUERY_RESOLVE | BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        );
        let buffer = buffer_desc.create_buffer(&device).unwrap();

        // Create command encoder
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("query_encoder"),
        });

        // Write timestamps
        QuerySetOps::write_timestamp(&mut encoder, &query_set, 0);
        QuerySetOps::write_timestamp(&mut encoder, &query_set, 1);

        // Resolve queries to buffer with offset of 8 bytes
        QuerySetOps::resolve_query_set(&mut encoder, &query_set, 0..2, &buffer, 8);

        let command_buffer = encoder.finish();
        queue.submit(Some(command_buffer));

        // Map buffer and verify we can read it
        BufferOps::map_read(&buffer).await.unwrap();
        let view = BufferOps::get_mapped_range(&buffer);

        // Verify the buffer has the expected size
        assert_eq!(view.len(), 32);

        drop(view);
        BufferOps::unmap(&buffer);
    });
}

#[test]
fn test_default_query_set_descriptor() {
    let descriptor = QuerySetDescriptor::default();
    assert_eq!(descriptor.count(), 2);
    assert_eq!(descriptor.query_type(), QueryType::Timestamp);
}
