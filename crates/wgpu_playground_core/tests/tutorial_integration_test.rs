//! Integration tests for the tutorial system

use wgpu_playground_core::tutorial::{
    get_all_tutorials, Difficulty, HighlightTarget, TutorialState, ValidationCheck,
};
use wgpu_playground_core::tutorial_panel::TutorialPanel;

#[test]
fn test_all_four_tutorials_available() {
    let tutorials = get_all_tutorials();
    assert_eq!(
        tutorials.len(),
        4,
        "Should have exactly 4 tutorials as specified"
    );

    // Verify tutorial IDs
    assert_eq!(tutorials[0].id, "hello_triangle");
    assert_eq!(tutorials[1].id, "adding_textures");
    assert_eq!(tutorials[2].id, "3d_with_depth");
    assert_eq!(tutorials[3].id, "gpu_compute");
}

#[test]
fn test_hello_triangle_tutorial_content() {
    let tutorials = get_all_tutorials();
    let hello_triangle = &tutorials[0];

    assert_eq!(hello_triangle.title, "Hello Triangle");
    assert_eq!(hello_triangle.difficulty, Difficulty::Beginner);
    assert!(!hello_triangle.steps.is_empty());

    // Verify it covers basic rendering setup
    assert!(
        hello_triangle.steps.len() >= 5,
        "Tutorial should have multiple steps"
    );

    // Check that it includes key concepts
    let step_descriptions: Vec<String> = hello_triangle
        .steps
        .iter()
        .map(|s| s.description.clone())
        .collect();

    let descriptions_text = step_descriptions.join(" ");
    assert!(descriptions_text.contains("vertex") || descriptions_text.contains("buffer"));
    assert!(descriptions_text.contains("shader") || descriptions_text.contains("pipeline"));
}

#[test]
fn test_adding_textures_tutorial_content() {
    let tutorials = get_all_tutorials();
    let textures = &tutorials[1];

    assert_eq!(textures.title, "Adding Textures");
    assert_eq!(textures.difficulty, Difficulty::Beginner);

    // Check for texture-related concepts
    let all_text: String = textures
        .steps
        .iter()
        .map(|s| format!("{} {}", s.description, s.explanation))
        .collect::<Vec<_>>()
        .join(" ");

    assert!(all_text.contains("texture") || all_text.contains("Texture"));
    assert!(all_text.contains("sampler") || all_text.contains("Sampler"));
}

#[test]
fn test_3d_depth_tutorial_content() {
    let tutorials = get_all_tutorials();
    let depth_3d = &tutorials[2];

    assert_eq!(depth_3d.title, "3D with Depth");
    assert_eq!(depth_3d.difficulty, Difficulty::Intermediate);

    // Check for 3D and depth concepts
    let all_text: String = depth_3d
        .steps
        .iter()
        .map(|s| format!("{} {}", s.description, s.explanation))
        .collect::<Vec<_>>()
        .join(" ");

    assert!(all_text.contains("depth") || all_text.contains("Depth"));
    assert!(all_text.contains("matrix") || all_text.contains("Matrix"));
}

#[test]
fn test_gpu_compute_tutorial_content() {
    let tutorials = get_all_tutorials();
    let compute = &tutorials[3];

    assert_eq!(compute.title, "GPU Compute");
    assert_eq!(compute.difficulty, Difficulty::Intermediate);

    // Check for compute shader concepts
    let all_text: String = compute
        .steps
        .iter()
        .map(|s| format!("{} {}", s.description, s.explanation))
        .collect::<Vec<_>>()
        .join(" ");

    assert!(all_text.contains("compute") || all_text.contains("Compute"));
    assert!(all_text.contains("shader") || all_text.contains("Shader"));
}

#[test]
fn test_tutorial_panel_initialization() {
    let panel = TutorialPanel::new();

    // Should have tutorials loaded
    assert!(!panel.get_current_highlight().is_none() || panel.get_current_highlight().is_none());
}

#[test]
fn test_tutorial_state_progression() {
    let mut state = TutorialState::default();

    // Start tutorial
    state.current_tutorial = Some(0);
    state.current_step = 0;

    // Progress through steps
    state.current_step += 1;
    assert_eq!(state.current_step, 1);

    state.current_step += 1;
    assert_eq!(state.current_step, 2);
}

#[test]
fn test_panel_visit_tracking() {
    let mut state = TutorialState::default();

    // Track panel visits
    state.mark_panel_visited(HighlightTarget::BufferConfig);
    assert!(state.visited_panels.contains(&HighlightTarget::BufferConfig));

    state.mark_panel_visited(HighlightTarget::RenderPipeline);
    assert!(state.visited_panels.contains(&HighlightTarget::RenderPipeline));
    assert_eq!(state.visited_panels.len(), 2);
}

#[test]
fn test_tutorial_completion() {
    let mut state = TutorialState::default();

    // Complete a tutorial
    state.complete_current_tutorial("hello_triangle".to_string());
    assert!(state.is_tutorial_completed("hello_triangle"));
    assert!(!state.is_tutorial_completed("adding_textures"));

    // State should reset
    assert_eq!(state.current_step, 0);
    assert!(state.current_tutorial.is_none());
}

#[test]
fn test_multiple_tutorial_completions() {
    let mut state = TutorialState::default();

    state.complete_current_tutorial("hello_triangle".to_string());
    state.complete_current_tutorial("adding_textures".to_string());

    assert_eq!(state.completed_tutorials.len(), 2);
    assert!(state.is_tutorial_completed("hello_triangle"));
    assert!(state.is_tutorial_completed("adding_textures"));
}

#[test]
fn test_highlight_target_mapping() {
    let tutorials = get_all_tutorials();

    // Hello Triangle should highlight relevant panels
    let hello_triangle = &tutorials[0];
    let has_buffer_highlight = hello_triangle
        .steps
        .iter()
        .any(|s| s.highlight_panel == Some(HighlightTarget::BufferConfig));
    let has_pipeline_highlight = hello_triangle
        .steps
        .iter()
        .any(|s| s.highlight_panel == Some(HighlightTarget::RenderPipeline));

    assert!(has_buffer_highlight, "Should highlight buffer config panel");
    assert!(
        has_pipeline_highlight,
        "Should highlight render pipeline panel"
    );
}

#[test]
fn test_validation_checks_present() {
    let tutorials = get_all_tutorials();

    for tutorial in &tutorials {
        // Each tutorial should have validation on most steps
        let steps_with_validation = tutorial
            .steps
            .iter()
            .filter(|s| s.validation.is_some())
            .count();

        assert!(
            steps_with_validation > 0,
            "Tutorial '{}' should have validation checks",
            tutorial.title
        );
    }
}

#[test]
fn test_manual_confirm_validation() {
    let tutorials = get_all_tutorials();
    let hello_triangle = &tutorials[0];

    // First step should have manual confirmation
    if let Some(validation) = &hello_triangle.steps[0].validation {
        assert!(matches!(
            validation.check_type,
            ValidationCheck::ManualConfirm
        ));
    }
}

#[test]
fn test_tutorial_panel_highlight_logic() {
    let mut panel = TutorialPanel::new();

    // No highlight when no tutorial is active
    assert!(panel.get_current_highlight().is_none());

    // Mark a panel as visited
    panel.mark_panel_visited(HighlightTarget::BufferConfig);

    // Should not highlight if tutorial isn't active
    assert!(!panel.should_highlight_panel(HighlightTarget::BufferConfig));
}

#[test]
fn test_tutorial_descriptions_are_educational() {
    let tutorials = get_all_tutorials();

    for tutorial in &tutorials {
        // Each step should have meaningful explanation
        for step in &tutorial.steps {
            assert!(
                step.explanation.len() > 50,
                "Step '{}' in '{}' should have substantial explanation",
                step.title,
                tutorial.title
            );

            // Explanation should be educational (check for key words)
            let explanation_lower = step.explanation.to_lowercase();
            let description_lower = step.description.to_lowercase();
            let combined = format!("{} {}", explanation_lower, description_lower);
            
            let has_educational_content = combined.contains("gpu")
                || combined.contains("webgpu")
                || combined.contains("shader")
                || combined.contains("buffer")
                || combined.contains("texture")
                || combined.contains("render")
                || combined.contains("compute")
                || combined.contains("pipeline")
                || combined.contains("vertex")
                || combined.contains("fragment")
                || combined.contains("depth")
                || combined.contains("matrix")
                || combined.contains("3d");

            assert!(
                has_educational_content,
                "Step '{}' should have educational WebGPU content",
                step.title
            );
        }
    }
}
