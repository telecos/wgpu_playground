/// Integration tests for the API Reference panel
use wgpu_playground_core::api_reference_panel::{ApiReferenceCategory, ApiReferencePanel};

#[test]
fn test_api_reference_panel_creation() {
    let panel = ApiReferencePanel::new();
    // Panel should start with no category selected
    assert!(matches!(panel, ApiReferencePanel { .. }));
}

#[test]
fn test_all_api_categories_are_accessible() {
    // Verify all categories have proper metadata
    let categories = ApiReferenceCategory::all();

    assert!(!categories.is_empty(), "Should have at least one category");

    for category in categories {
        // Each category should have a name
        let name = category.name();
        assert!(!name.is_empty(), "Category {:?} has empty name", category);

        // Each category should have a description
        let description = category.description();
        assert!(!description.is_empty(), "Category {:?} has empty description", category);

        // Each category should have a spec URL
        let spec_url = category.spec_url();
        assert!(spec_url.starts_with("https://"), "Category {:?} has invalid spec URL", category);
        assert!(spec_url.contains("webgpu"), "Category {:?} spec URL doesn't contain 'webgpu'", category);
    }
}

#[test]
fn test_major_categories_are_present() {
    let categories = ApiReferenceCategory::all();
    let category_names: Vec<&str> = categories.iter().map(|c| c.name()).collect();

    // Verify major WebGPU objects are documented
    assert!(category_names.contains(&"Device"), "Device category should be present");
    assert!(category_names.contains(&"Queue"), "Queue category should be present");
    assert!(category_names.contains(&"Buffer"), "Buffer category should be present");
    assert!(category_names.contains(&"Texture"), "Texture category should be present");
    assert!(category_names.contains(&"Render Pipeline"), "Render Pipeline category should be present");
    assert!(category_names.contains(&"Compute Pipeline"), "Compute Pipeline category should be present");
}

#[test]
fn test_category_descriptions_are_informative() {
    let categories = ApiReferenceCategory::all();

    for category in categories {
        let description = category.description();

        // Descriptions should be reasonably informative (at least 20 characters)
        assert!(
            description.len() >= 20,
            "Category {:?} has too short description: '{}'",
            category,
            description
        );

        // Descriptions should be capitalized
        assert!(
            description.chars().next().unwrap().is_uppercase(),
            "Category {:?} description should start with uppercase",
            category
        );
    }
}

#[test]
fn test_spec_urls_point_to_webgpu_spec() {
    let categories = ApiReferenceCategory::all();

    for category in categories {
        let url = category.spec_url();

        // All URLs should point to W3C WebGPU spec
        assert!(
            url.contains("w3.org/TR/webgpu"),
            "Category {:?} URL doesn't point to WebGPU spec: {}",
            category,
            url
        );
    }
}
