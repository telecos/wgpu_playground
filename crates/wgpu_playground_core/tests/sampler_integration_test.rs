use wgpu_playground_core::sampler::{
    AddressMode, CompareFunction, FilterMode, MipmapFilterMode, SamplerDescriptor,
};

#[test]
fn test_basic_sampler_descriptor() {
    let descriptor = SamplerDescriptor::new(Some("basic_sampler"));
    assert_eq!(descriptor.label(), Some("basic_sampler"));
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_repeat_sampler() {
    let descriptor = SamplerDescriptor::new(Some("repeat_sampler"))
        .with_address_mode(AddressMode::Repeat)
        .with_filter(FilterMode::Linear);

    assert_eq!(descriptor.address_mode_u(), AddressMode::Repeat);
    assert_eq!(descriptor.address_mode_v(), AddressMode::Repeat);
    assert_eq!(descriptor.address_mode_w(), AddressMode::Repeat);
    assert_eq!(descriptor.mag_filter(), FilterMode::Linear);
    assert_eq!(descriptor.min_filter(), FilterMode::Linear);
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_mirror_sampler() {
    let descriptor = SamplerDescriptor::new(Some("mirror_sampler"))
        .with_address_mode(AddressMode::MirrorRepeat)
        .with_filter(FilterMode::Nearest);

    assert_eq!(descriptor.address_mode_u(), AddressMode::MirrorRepeat);
    assert_eq!(descriptor.address_mode_v(), AddressMode::MirrorRepeat);
    assert_eq!(descriptor.address_mode_w(), AddressMode::MirrorRepeat);
    assert_eq!(descriptor.mag_filter(), FilterMode::Nearest);
    assert_eq!(descriptor.min_filter(), FilterMode::Nearest);
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_clamp_sampler() {
    let descriptor = SamplerDescriptor::new(Some("clamp_sampler"))
        .with_address_mode(AddressMode::ClampToEdge)
        .with_filter(FilterMode::Linear);

    assert_eq!(descriptor.address_mode_u(), AddressMode::ClampToEdge);
    assert_eq!(descriptor.address_mode_v(), AddressMode::ClampToEdge);
    assert_eq!(descriptor.address_mode_w(), AddressMode::ClampToEdge);
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_mixed_address_modes() {
    let descriptor = SamplerDescriptor::new(Some("mixed_sampler"))
        .with_address_mode_u(AddressMode::Repeat)
        .with_address_mode_v(AddressMode::ClampToEdge)
        .with_address_mode_w(AddressMode::MirrorRepeat);

    assert_eq!(descriptor.address_mode_u(), AddressMode::Repeat);
    assert_eq!(descriptor.address_mode_v(), AddressMode::ClampToEdge);
    assert_eq!(descriptor.address_mode_w(), AddressMode::MirrorRepeat);
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_linear_filtering() {
    let descriptor = SamplerDescriptor::new(Some("linear_sampler"))
        .with_mag_filter(FilterMode::Linear)
        .with_min_filter(FilterMode::Linear)
        .with_mipmap_filter(MipmapFilterMode::Linear);

    assert_eq!(descriptor.mag_filter(), FilterMode::Linear);
    assert_eq!(descriptor.min_filter(), FilterMode::Linear);
    assert_eq!(descriptor.mipmap_filter(), MipmapFilterMode::Linear);
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_nearest_filtering() {
    let descriptor = SamplerDescriptor::new(Some("nearest_sampler"))
        .with_mag_filter(FilterMode::Nearest)
        .with_min_filter(FilterMode::Nearest)
        .with_mipmap_filter(MipmapFilterMode::Nearest);

    assert_eq!(descriptor.mag_filter(), FilterMode::Nearest);
    assert_eq!(descriptor.min_filter(), FilterMode::Nearest);
    assert_eq!(descriptor.mipmap_filter(), MipmapFilterMode::Nearest);
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_mixed_filtering() {
    let descriptor = SamplerDescriptor::new(Some("mixed_filter_sampler"))
        .with_mag_filter(FilterMode::Linear)
        .with_min_filter(FilterMode::Nearest)
        .with_mipmap_filter(MipmapFilterMode::Linear);

    assert_eq!(descriptor.mag_filter(), FilterMode::Linear);
    assert_eq!(descriptor.min_filter(), FilterMode::Nearest);
    assert_eq!(descriptor.mipmap_filter(), MipmapFilterMode::Linear);
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_lod_configuration() {
    let descriptor = SamplerDescriptor::new(Some("lod_sampler"))
        .with_lod_min_clamp(1.0)
        .with_lod_max_clamp(5.0);

    assert_eq!(descriptor.lod_min_clamp(), 1.0);
    assert_eq!(descriptor.lod_max_clamp(), 5.0);
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_lod_clamp_helper() {
    let descriptor = SamplerDescriptor::new(Some("lod_sampler")).with_lod_clamp(2.0, 8.0);

    assert_eq!(descriptor.lod_min_clamp(), 2.0);
    assert_eq!(descriptor.lod_max_clamp(), 8.0);
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_anisotropic_filtering() {
    let descriptor = SamplerDescriptor::new(Some("anisotropic_sampler"))
        .with_filter(FilterMode::Linear)
        .with_anisotropy(16);

    assert_eq!(descriptor.anisotropy_clamp(), 16);
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_comparison_never() {
    let descriptor =
        SamplerDescriptor::new(Some("comparison_never")).with_compare(CompareFunction::Never);

    assert_eq!(descriptor.compare(), Some(CompareFunction::Never));
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_comparison_less() {
    let descriptor =
        SamplerDescriptor::new(Some("comparison_less")).with_compare(CompareFunction::Less);

    assert_eq!(descriptor.compare(), Some(CompareFunction::Less));
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_comparison_equal() {
    let descriptor =
        SamplerDescriptor::new(Some("comparison_equal")).with_compare(CompareFunction::Equal);

    assert_eq!(descriptor.compare(), Some(CompareFunction::Equal));
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_comparison_less_equal() {
    let descriptor = SamplerDescriptor::new(Some("comparison_less_equal"))
        .with_compare(CompareFunction::LessEqual);

    assert_eq!(descriptor.compare(), Some(CompareFunction::LessEqual));
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_comparison_greater() {
    let descriptor =
        SamplerDescriptor::new(Some("comparison_greater")).with_compare(CompareFunction::Greater);

    assert_eq!(descriptor.compare(), Some(CompareFunction::Greater));
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_comparison_not_equal() {
    let descriptor = SamplerDescriptor::new(Some("comparison_not_equal"))
        .with_compare(CompareFunction::NotEqual);

    assert_eq!(descriptor.compare(), Some(CompareFunction::NotEqual));
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_comparison_greater_equal() {
    let descriptor = SamplerDescriptor::new(Some("comparison_greater_equal"))
        .with_compare(CompareFunction::GreaterEqual);

    assert_eq!(descriptor.compare(), Some(CompareFunction::GreaterEqual));
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_comparison_always() {
    let descriptor =
        SamplerDescriptor::new(Some("comparison_always")).with_compare(CompareFunction::Always);

    assert_eq!(descriptor.compare(), Some(CompareFunction::Always));
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_depth_comparison_sampler() {
    // Typical depth/shadow map sampler configuration
    let descriptor = SamplerDescriptor::new(Some("shadow_sampler"))
        .with_address_mode(AddressMode::ClampToEdge)
        .with_filter(FilterMode::Linear)
        .with_compare(CompareFunction::LessEqual);

    assert_eq!(descriptor.address_mode_u(), AddressMode::ClampToEdge);
    assert_eq!(descriptor.mag_filter(), FilterMode::Linear);
    assert_eq!(descriptor.min_filter(), FilterMode::Linear);
    assert_eq!(descriptor.compare(), Some(CompareFunction::LessEqual));
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_texture_repeat_sampler() {
    // Typical texture repeat sampler for tiling
    let descriptor = SamplerDescriptor::new(Some("texture_repeat"))
        .with_address_mode(AddressMode::Repeat)
        .with_filter(FilterMode::Linear)
        .with_mipmap_filter(MipmapFilterMode::Linear)
        .with_anisotropy(16);

    assert_eq!(descriptor.address_mode_u(), AddressMode::Repeat);
    assert_eq!(descriptor.mag_filter(), FilterMode::Linear);
    assert_eq!(descriptor.mipmap_filter(), MipmapFilterMode::Linear);
    assert_eq!(descriptor.anisotropy_clamp(), 16);
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_ui_texture_sampler() {
    // Typical UI texture sampler (clamped, linear)
    let descriptor = SamplerDescriptor::new(Some("ui_sampler"))
        .with_address_mode(AddressMode::ClampToEdge)
        .with_filter(FilterMode::Linear)
        .with_mipmap_filter(MipmapFilterMode::Nearest);

    assert_eq!(descriptor.address_mode_u(), AddressMode::ClampToEdge);
    assert_eq!(descriptor.mag_filter(), FilterMode::Linear);
    assert_eq!(descriptor.min_filter(), FilterMode::Linear);
    assert_eq!(descriptor.mipmap_filter(), MipmapFilterMode::Nearest);
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_pixel_art_sampler() {
    // Typical pixel art sampler (nearest, no mipmaps)
    let descriptor = SamplerDescriptor::new(Some("pixel_art"))
        .with_address_mode(AddressMode::ClampToEdge)
        .with_filter(FilterMode::Nearest)
        .with_mipmap_filter(MipmapFilterMode::Nearest)
        .with_anisotropy(1);

    assert_eq!(descriptor.mag_filter(), FilterMode::Nearest);
    assert_eq!(descriptor.min_filter(), FilterMode::Nearest);
    assert_eq!(descriptor.mipmap_filter(), MipmapFilterMode::Nearest);
    assert_eq!(descriptor.anisotropy_clamp(), 1);
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_border_clamp_sampler() {
    let descriptor = SamplerDescriptor::new(Some("border_sampler"))
        .with_address_mode(AddressMode::ClampToBorder)
        .with_border_color(wgpu::SamplerBorderColor::TransparentBlack);

    assert_eq!(descriptor.address_mode_u(), AddressMode::ClampToBorder);
    assert_eq!(
        descriptor.border_color(),
        Some(wgpu::SamplerBorderColor::TransparentBlack)
    );
    assert!(descriptor.validate().is_ok());
}

#[test]
fn test_all_address_modes() {
    // Test all four address modes
    let clamp_edge = SamplerDescriptor::new(None).with_address_mode(AddressMode::ClampToEdge);
    assert_eq!(clamp_edge.address_mode_u(), AddressMode::ClampToEdge);

    let repeat = SamplerDescriptor::new(None).with_address_mode(AddressMode::Repeat);
    assert_eq!(repeat.address_mode_u(), AddressMode::Repeat);

    let mirror = SamplerDescriptor::new(None).with_address_mode(AddressMode::MirrorRepeat);
    assert_eq!(mirror.address_mode_u(), AddressMode::MirrorRepeat);

    let clamp_border = SamplerDescriptor::new(None)
        .with_address_mode(AddressMode::ClampToBorder)
        .with_border_color(wgpu::SamplerBorderColor::TransparentBlack);
    assert_eq!(clamp_border.address_mode_u(), AddressMode::ClampToBorder);
    assert!(clamp_border.validate().is_ok());
}

#[test]
fn test_all_filter_modes() {
    // Test both filter modes
    let nearest = SamplerDescriptor::new(None).with_filter(FilterMode::Nearest);
    assert_eq!(nearest.mag_filter(), FilterMode::Nearest);
    assert_eq!(nearest.min_filter(), FilterMode::Nearest);

    let linear = SamplerDescriptor::new(None).with_filter(FilterMode::Linear);
    assert_eq!(linear.mag_filter(), FilterMode::Linear);
    assert_eq!(linear.min_filter(), FilterMode::Linear);
}

#[test]
fn test_all_compare_functions() {
    // Test all eight compare functions
    let functions = vec![
        CompareFunction::Never,
        CompareFunction::Less,
        CompareFunction::Equal,
        CompareFunction::LessEqual,
        CompareFunction::Greater,
        CompareFunction::NotEqual,
        CompareFunction::GreaterEqual,
        CompareFunction::Always,
    ];

    for func in functions {
        let descriptor = SamplerDescriptor::new(None).with_compare(func);
        assert_eq!(descriptor.compare(), Some(func));
        assert!(descriptor.validate().is_ok());
    }
}

#[test]
fn test_comprehensive_sampler_configuration() {
    // Test a fully configured sampler with all options
    let descriptor = SamplerDescriptor::new(Some("comprehensive_sampler"))
        .with_address_mode_u(AddressMode::Repeat)
        .with_address_mode_v(AddressMode::MirrorRepeat)
        .with_address_mode_w(AddressMode::ClampToEdge)
        .with_mag_filter(FilterMode::Linear)
        .with_min_filter(FilterMode::Linear)
        .with_mipmap_filter(MipmapFilterMode::Linear)
        .with_lod_clamp(0.0, 12.0)
        .with_anisotropy(8)
        .with_compare(CompareFunction::Less);

    assert_eq!(descriptor.label(), Some("comprehensive_sampler"));
    assert_eq!(descriptor.address_mode_u(), AddressMode::Repeat);
    assert_eq!(descriptor.address_mode_v(), AddressMode::MirrorRepeat);
    assert_eq!(descriptor.address_mode_w(), AddressMode::ClampToEdge);
    assert_eq!(descriptor.mag_filter(), FilterMode::Linear);
    assert_eq!(descriptor.min_filter(), FilterMode::Linear);
    assert_eq!(descriptor.mipmap_filter(), MipmapFilterMode::Linear);
    assert_eq!(descriptor.lod_min_clamp(), 0.0);
    assert_eq!(descriptor.lod_max_clamp(), 12.0);
    assert_eq!(descriptor.anisotropy_clamp(), 8);
    assert_eq!(descriptor.compare(), Some(CompareFunction::Less));
    assert!(descriptor.validate().is_ok());
}
