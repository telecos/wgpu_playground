use wgpu_playground_core::adapter::{
    create_instance, create_instance_with_options, get_adapter_features, get_adapter_limits,
    request_adapter, AdapterOptions,
};

#[test]
fn test_request_adapter_with_default_options() {
    pollster::block_on(async {
        let options = AdapterOptions::default();
        let instance = create_instance_with_options(&options);

        let Ok(adapter) = request_adapter(&instance, &options, None).await else {
            eprintln!("Skipping test: No GPU adapter available");
            return;
        };

        // The returned adapter must be usable
        assert!(!adapter.get_info().name.is_empty());
        assert!(get_adapter_limits(&adapter).max_texture_dimension_2d > 0);
        let _features = get_adapter_features(&adapter);
    });
}

#[test]
fn test_request_adapter_with_fallback_options() {
    pollster::block_on(async {
        let options = AdapterOptions::fallback();
        let instance = create_instance(options.backends);

        match request_adapter(&instance, &options, None).await {
            Ok(adapter) => assert!(!adapter.get_info().name.is_empty()),
            Err(err) => eprintln!("Skipping test: no fallback adapter available ({})", err),
        }
    });
}
