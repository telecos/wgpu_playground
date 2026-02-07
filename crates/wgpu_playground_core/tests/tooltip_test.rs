//! Tests for tooltip utilities

use wgpu_playground_core::tooltip::*;

#[test]
fn test_buffer_usage_tooltips_have_descriptions() {
    assert!(!buffer_usage::VERTEX.description.is_empty());
    assert!(!buffer_usage::INDEX.description.is_empty());
    assert!(!buffer_usage::UNIFORM.description.is_empty());
    assert!(!buffer_usage::STORAGE.description.is_empty());
    assert!(!buffer_usage::INDIRECT.description.is_empty());
    assert!(!buffer_usage::COPY_SRC.description.is_empty());
    assert!(!buffer_usage::COPY_DST.description.is_empty());
    assert!(!buffer_usage::MAP_READ.description.is_empty());
    assert!(!buffer_usage::MAP_WRITE.description.is_empty());
    assert!(!buffer_usage::QUERY_RESOLVE.description.is_empty());
}

#[test]
fn test_buffer_usage_tooltips_have_spec_anchors() {
    assert!(buffer_usage::VERTEX.spec_anchor.is_some());
    assert!(buffer_usage::INDEX.spec_anchor.is_some());
    assert!(buffer_usage::UNIFORM.spec_anchor.is_some());
    assert!(buffer_usage::STORAGE.spec_anchor.is_some());
    assert!(buffer_usage::INDIRECT.spec_anchor.is_some());
    assert!(buffer_usage::COPY_SRC.spec_anchor.is_some());
    assert!(buffer_usage::COPY_DST.spec_anchor.is_some());
    assert!(buffer_usage::MAP_READ.spec_anchor.is_some());
    assert!(buffer_usage::MAP_WRITE.spec_anchor.is_some());
    assert!(buffer_usage::QUERY_RESOLVE.spec_anchor.is_some());
}

#[test]
fn test_texture_usage_tooltips_have_descriptions() {
    assert!(!texture_usage::COPY_SRC.description.is_empty());
    assert!(!texture_usage::COPY_DST.description.is_empty());
    assert!(!texture_usage::TEXTURE_BINDING.description.is_empty());
    assert!(!texture_usage::STORAGE_BINDING.description.is_empty());
    assert!(!texture_usage::RENDER_ATTACHMENT.description.is_empty());
}

#[test]
fn test_texture_usage_tooltips_have_spec_anchors() {
    assert!(texture_usage::COPY_SRC.spec_anchor.is_some());
    assert!(texture_usage::COPY_DST.spec_anchor.is_some());
    assert!(texture_usage::TEXTURE_BINDING.spec_anchor.is_some());
    assert!(texture_usage::STORAGE_BINDING.spec_anchor.is_some());
    assert!(texture_usage::RENDER_ATTACHMENT.spec_anchor.is_some());
}

#[test]
fn test_primitive_topology_tooltips() {
    assert!(!primitive_topology::POINT_LIST.description.is_empty());
    assert!(!primitive_topology::LINE_LIST.description.is_empty());
    assert!(!primitive_topology::LINE_STRIP.description.is_empty());
    assert!(!primitive_topology::TRIANGLE_LIST.description.is_empty());
    assert!(!primitive_topology::TRIANGLE_STRIP.description.is_empty());

    assert!(primitive_topology::POINT_LIST.spec_anchor.is_some());
    assert!(primitive_topology::LINE_LIST.spec_anchor.is_some());
    assert!(primitive_topology::LINE_STRIP.spec_anchor.is_some());
    assert!(primitive_topology::TRIANGLE_LIST.spec_anchor.is_some());
    assert!(primitive_topology::TRIANGLE_STRIP.spec_anchor.is_some());
}

#[test]
fn test_cull_mode_tooltips() {
    assert!(!cull_mode::NONE.description.is_empty());
    assert!(!cull_mode::FRONT.description.is_empty());
    assert!(!cull_mode::BACK.description.is_empty());

    assert!(cull_mode::NONE.spec_anchor.is_some());
    assert!(cull_mode::FRONT.spec_anchor.is_some());
    assert!(cull_mode::BACK.spec_anchor.is_some());
}

#[test]
fn test_front_face_tooltips() {
    assert!(!front_face::CCW.description.is_empty());
    assert!(!front_face::CW.description.is_empty());

    assert!(front_face::CCW.spec_anchor.is_some());
    assert!(front_face::CW.spec_anchor.is_some());
}

#[test]
fn test_compare_function_tooltips() {
    assert!(!compare_function::NEVER.description.is_empty());
    assert!(!compare_function::LESS.description.is_empty());
    assert!(!compare_function::EQUAL.description.is_empty());
    assert!(!compare_function::LESS_EQUAL.description.is_empty());
    assert!(!compare_function::GREATER.description.is_empty());
    assert!(!compare_function::NOT_EQUAL.description.is_empty());
    assert!(!compare_function::GREATER_EQUAL.description.is_empty());
    assert!(!compare_function::ALWAYS.description.is_empty());

    assert!(compare_function::NEVER.spec_anchor.is_some());
    assert!(compare_function::LESS.spec_anchor.is_some());
    assert!(compare_function::EQUAL.spec_anchor.is_some());
    assert!(compare_function::LESS_EQUAL.spec_anchor.is_some());
    assert!(compare_function::GREATER.spec_anchor.is_some());
    assert!(compare_function::NOT_EQUAL.spec_anchor.is_some());
    assert!(compare_function::GREATER_EQUAL.spec_anchor.is_some());
    assert!(compare_function::ALWAYS.spec_anchor.is_some());
}

#[test]
fn test_blend_factor_tooltips() {
    assert!(!blend_factor::ZERO.description.is_empty());
    assert!(!blend_factor::ONE.description.is_empty());
    assert!(!blend_factor::SRC.description.is_empty());
    assert!(!blend_factor::ONE_MINUS_SRC.description.is_empty());
    assert!(!blend_factor::SRC_ALPHA.description.is_empty());
    assert!(!blend_factor::ONE_MINUS_SRC_ALPHA.description.is_empty());
    assert!(!blend_factor::DST.description.is_empty());
    assert!(!blend_factor::ONE_MINUS_DST.description.is_empty());
    assert!(!blend_factor::DST_ALPHA.description.is_empty());
    assert!(!blend_factor::ONE_MINUS_DST_ALPHA.description.is_empty());

    assert!(blend_factor::ZERO.spec_anchor.is_some());
    assert!(blend_factor::ONE.spec_anchor.is_some());
    assert!(blend_factor::SRC.spec_anchor.is_some());
}

#[test]
fn test_blend_operation_tooltips() {
    assert!(!blend_operation::ADD.description.is_empty());
    assert!(!blend_operation::SUBTRACT.description.is_empty());
    assert!(!blend_operation::REVERSE_SUBTRACT.description.is_empty());
    assert!(!blend_operation::MIN.description.is_empty());
    assert!(!blend_operation::MAX.description.is_empty());

    assert!(blend_operation::ADD.spec_anchor.is_some());
    assert!(blend_operation::SUBTRACT.spec_anchor.is_some());
    assert!(blend_operation::REVERSE_SUBTRACT.spec_anchor.is_some());
    assert!(blend_operation::MIN.spec_anchor.is_some());
    assert!(blend_operation::MAX.spec_anchor.is_some());
}

#[test]
fn test_address_mode_tooltips() {
    assert!(!address_mode::CLAMP_TO_EDGE.description.is_empty());
    assert!(!address_mode::REPEAT.description.is_empty());
    assert!(!address_mode::MIRROR_REPEAT.description.is_empty());
    assert!(!address_mode::CLAMP_TO_BORDER.description.is_empty());

    assert!(address_mode::CLAMP_TO_EDGE.spec_anchor.is_some());
    assert!(address_mode::REPEAT.spec_anchor.is_some());
    assert!(address_mode::MIRROR_REPEAT.spec_anchor.is_some());
    assert!(address_mode::CLAMP_TO_BORDER.spec_anchor.is_some());
}

#[test]
fn test_filter_mode_tooltips() {
    assert!(!filter_mode::NEAREST.description.is_empty());
    assert!(!filter_mode::LINEAR.description.is_empty());

    assert!(filter_mode::NEAREST.spec_anchor.is_some());
    assert!(filter_mode::LINEAR.spec_anchor.is_some());
}

#[test]
fn test_stencil_operation_tooltips() {
    assert!(!stencil_operation::KEEP.description.is_empty());
    assert!(!stencil_operation::ZERO.description.is_empty());
    assert!(!stencil_operation::REPLACE.description.is_empty());
    assert!(!stencil_operation::INVERT.description.is_empty());
    assert!(!stencil_operation::INCREMENT_CLAMP.description.is_empty());
    assert!(!stencil_operation::DECREMENT_CLAMP.description.is_empty());
    assert!(!stencil_operation::INCREMENT_WRAP.description.is_empty());
    assert!(!stencil_operation::DECREMENT_WRAP.description.is_empty());

    assert!(stencil_operation::KEEP.spec_anchor.is_some());
    assert!(stencil_operation::ZERO.spec_anchor.is_some());
    assert!(stencil_operation::REPLACE.spec_anchor.is_some());
    assert!(stencil_operation::INVERT.spec_anchor.is_some());
}

#[test]
fn test_property_tooltips() {
    assert!(!property::BUFFER_SIZE.description.is_empty());
    assert!(!property::BUFFER_MAPPED_AT_CREATION.description.is_empty());
    assert!(!property::TEXTURE_WIDTH.description.is_empty());
    assert!(!property::TEXTURE_HEIGHT.description.is_empty());
    assert!(!property::TEXTURE_DEPTH.description.is_empty());
    assert!(!property::TEXTURE_MIP_LEVELS.description.is_empty());
    assert!(!property::TEXTURE_SAMPLE_COUNT.description.is_empty());
    assert!(!property::DEPTH_WRITE_ENABLED.description.is_empty());
    assert!(!property::ALPHA_TO_COVERAGE.description.is_empty());
    assert!(!property::SAMPLE_COUNT.description.is_empty());

    assert!(property::BUFFER_SIZE.spec_anchor.is_some());
    assert!(property::BUFFER_MAPPED_AT_CREATION.spec_anchor.is_some());
    assert!(property::TEXTURE_WIDTH.spec_anchor.is_some());
    assert!(property::TEXTURE_HEIGHT.spec_anchor.is_some());
}

#[test]
fn test_shader_visibility_tooltips() {
    assert!(!shader_visibility::VERTEX.description.is_empty());
    assert!(!shader_visibility::FRAGMENT.description.is_empty());
    assert!(!shader_visibility::COMPUTE.description.is_empty());

    assert!(shader_visibility::VERTEX.spec_anchor.is_some());
    assert!(shader_visibility::FRAGMENT.spec_anchor.is_some());
    assert!(shader_visibility::COMPUTE.spec_anchor.is_some());
}

#[test]
fn test_compute_tooltips() {
    assert!(!compute::WORKGROUP_COUNT_X.description.is_empty());
    assert!(!compute::WORKGROUP_COUNT_Y.description.is_empty());
    assert!(!compute::WORKGROUP_COUNT_Z.description.is_empty());
    assert!(!compute::ENTRY_POINT.description.is_empty());
    assert!(!compute::PIPELINE_LAYOUT.description.is_empty());

    assert!(compute::WORKGROUP_COUNT_X.spec_anchor.is_some());
    assert!(compute::WORKGROUP_COUNT_Y.spec_anchor.is_some());
    assert!(compute::WORKGROUP_COUNT_Z.spec_anchor.is_some());
}

#[test]
fn test_load_store_op_tooltips() {
    assert!(!load_store_op::LOAD_OP_CLEAR.description.is_empty());
    assert!(!load_store_op::LOAD_OP_LOAD.description.is_empty());
    assert!(!load_store_op::STORE_OP_STORE.description.is_empty());
    assert!(!load_store_op::STORE_OP_DISCARD.description.is_empty());

    assert!(load_store_op::LOAD_OP_CLEAR.spec_anchor.is_some());
    assert!(load_store_op::LOAD_OP_LOAD.spec_anchor.is_some());
    assert!(load_store_op::STORE_OP_STORE.spec_anchor.is_some());
    assert!(load_store_op::STORE_OP_DISCARD.spec_anchor.is_some());
}

#[test]
fn test_draw_tooltips() {
    assert!(!draw::VERTEX_COUNT.description.is_empty());
    assert!(!draw::INSTANCE_COUNT.description.is_empty());
    assert!(!draw::FIRST_VERTEX.description.is_empty());
    assert!(!draw::FIRST_INSTANCE.description.is_empty());

    assert!(draw::VERTEX_COUNT.spec_anchor.is_some());
    assert!(draw::INSTANCE_COUNT.spec_anchor.is_some());
    assert!(draw::FIRST_VERTEX.spec_anchor.is_some());
    assert!(draw::FIRST_INSTANCE.spec_anchor.is_some());
}

#[test]
fn test_tooltip_info_creation() {
    let tooltip = TooltipInfo::new("Test description", Some("#test-anchor"));
    assert_eq!(tooltip.description, "Test description");
    assert_eq!(tooltip.spec_anchor, Some("#test-anchor"));
}

#[test]
fn test_tooltip_info_without_anchor() {
    let tooltip = TooltipInfo::new("Test description", None);
    assert_eq!(tooltip.description, "Test description");
    assert_eq!(tooltip.spec_anchor, None);
}

#[test]
fn test_sampler_tooltips() {
    assert!(!sampler::LOD_MIN_CLAMP.description.is_empty());
    assert!(!sampler::LOD_MAX_CLAMP.description.is_empty());
    assert!(!sampler::MAX_ANISOTROPY.description.is_empty());
    assert!(!sampler::BORDER_COLOR.description.is_empty());

    assert!(sampler::LOD_MIN_CLAMP.spec_anchor.is_some());
    assert!(sampler::LOD_MAX_CLAMP.spec_anchor.is_some());
    assert!(sampler::MAX_ANISOTROPY.spec_anchor.is_some());
    assert!(sampler::BORDER_COLOR.spec_anchor.is_some());
}
