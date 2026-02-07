//! Learning Path System for WebGPU Concepts
//!
//! This module provides a structured learning path through WebGPU concepts,
//! mapping them to tutorials and examples with progress tracking.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Represents a node in the learning path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningNode {
    /// Unique identifier for the node
    pub id: String,
    /// Display name of the concept
    pub name: String,
    /// Brief description of what this concept covers
    pub description: String,
    /// Difficulty level
    pub difficulty: NodeDifficulty,
    /// IDs of prerequisite nodes that should be completed first
    pub prerequisites: Vec<String>,
    /// Associated tutorial IDs that teach this concept
    pub tutorials: Vec<String>,
    /// Associated example IDs that demonstrate this concept
    pub examples: Vec<String>,
    /// Category for grouping nodes
    pub category: NodeCategory,
}

/// Difficulty level of a learning node
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeDifficulty {
    Beginner,
    Intermediate,
    Advanced,
}

/// Category of WebGPU concept
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeCategory {
    /// Foundation concepts (device, queue, adapter)
    Foundation,
    /// Resource management (buffers, textures, samplers)
    Resources,
    /// Shader programming
    Shaders,
    /// Rendering pipeline
    Rendering,
    /// Compute pipeline
    Compute,
    /// Advanced topics (optimization, debugging)
    Advanced,
}

impl NodeCategory {
    pub fn name(&self) -> &'static str {
        match self {
            NodeCategory::Foundation => "Foundation",
            NodeCategory::Resources => "Resources",
            NodeCategory::Shaders => "Shaders",
            NodeCategory::Rendering => "Rendering",
            NodeCategory::Compute => "Compute",
            NodeCategory::Advanced => "Advanced",
        }
    }

    pub fn color(&self) -> egui::Color32 {
        match self {
            NodeCategory::Foundation => egui::Color32::from_rgb(100, 150, 255),
            NodeCategory::Resources => egui::Color32::from_rgb(150, 255, 150),
            NodeCategory::Shaders => egui::Color32::from_rgb(255, 200, 100),
            NodeCategory::Rendering => egui::Color32::from_rgb(255, 150, 150),
            NodeCategory::Compute => egui::Color32::from_rgb(200, 150, 255),
            NodeCategory::Advanced => egui::Color32::from_rgb(150, 150, 150),
        }
    }
}

/// Tracks user's progress through the learning path
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LearningProgress {
    /// IDs of completed tutorials
    pub completed_tutorials: HashSet<String>,
    /// IDs of tried examples
    pub tried_examples: HashSet<String>,
    /// IDs of visited nodes (for tracking exploration)
    pub visited_nodes: HashSet<String>,
}

impl LearningProgress {
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if a node is completed (all associated tutorials/examples done)
    pub fn is_node_completed(&self, node: &LearningNode) -> bool {
        // If node has no content, check if it's been explicitly visited
        if node.tutorials.is_empty() && node.examples.is_empty() {
            return self.visited_nodes.contains(&node.id);
        }

        // Node is completed if at least one tutorial is done OR one example is tried
        let has_completed_tutorial = node
            .tutorials
            .iter()
            .any(|t| self.completed_tutorials.contains(t));
        let has_tried_example = node
            .examples
            .iter()
            .any(|e| self.tried_examples.contains(e));

        has_completed_tutorial || has_tried_example
    }

    /// Check if a node is in progress (some but not all content completed)
    pub fn is_node_in_progress(&self, node: &LearningNode) -> bool {
        if self.is_node_completed(node) {
            return false;
        }

        let has_some_tutorial = node
            .tutorials
            .iter()
            .any(|t| self.completed_tutorials.contains(t));
        let has_some_example = node
            .examples
            .iter()
            .any(|e| self.tried_examples.contains(e));

        has_some_tutorial || has_some_example
    }

    /// Check if prerequisites for a node are met
    pub fn are_prerequisites_met(&self, node: &LearningNode, all_nodes: &[LearningNode]) -> bool {
        node.prerequisites.iter().all(|prereq_id| {
            all_nodes
                .iter()
                .find(|n| n.id == *prereq_id)
                .map(|prereq_node| self.is_node_completed(prereq_node))
                .unwrap_or(false)
        })
    }

    /// Mark a tutorial as completed
    pub fn complete_tutorial(&mut self, tutorial_id: String) {
        self.completed_tutorials.insert(tutorial_id);
    }

    /// Mark an example as tried
    pub fn try_example(&mut self, example_id: String) {
        self.tried_examples.insert(example_id);
    }

    /// Mark a node as visited (for exploration tracking)
    pub fn visit_node(&mut self, node_id: String) {
        self.visited_nodes.insert(node_id);
    }
}

/// Get the complete WebGPU learning path
pub fn get_learning_path() -> Vec<LearningNode> {
    vec![
        // Foundation
        LearningNode {
            id: "device_setup".to_string(),
            name: "Device Setup".to_string(),
            description: "Understanding GPU adapters, devices, and queues".to_string(),
            difficulty: NodeDifficulty::Beginner,
            prerequisites: vec![],
            tutorials: vec![],
            examples: vec![],
            category: NodeCategory::Foundation,
        },
        // Resources - Buffers
        LearningNode {
            id: "buffers".to_string(),
            name: "Buffers".to_string(),
            description: "Creating and managing GPU buffer resources".to_string(),
            difficulty: NodeDifficulty::Beginner,
            prerequisites: vec!["device_setup".to_string()],
            tutorials: vec!["hello_triangle".to_string()],
            examples: vec!["triangle".to_string()],
            category: NodeCategory::Resources,
        },
        // Shaders
        LearningNode {
            id: "shaders_basic".to_string(),
            name: "Basic Shaders".to_string(),
            description: "Writing vertex and fragment shaders in WGSL".to_string(),
            difficulty: NodeDifficulty::Beginner,
            prerequisites: vec!["device_setup".to_string()],
            tutorials: vec!["hello_triangle".to_string()],
            examples: vec!["triangle".to_string()],
            category: NodeCategory::Shaders,
        },
        // Rendering - Basic
        LearningNode {
            id: "render_pipeline".to_string(),
            name: "Render Pipeline".to_string(),
            description: "Configuring the rendering pipeline and render passes".to_string(),
            difficulty: NodeDifficulty::Beginner,
            prerequisites: vec!["buffers".to_string(), "shaders_basic".to_string()],
            tutorials: vec!["hello_triangle".to_string()],
            examples: vec!["triangle".to_string()],
            category: NodeCategory::Rendering,
        },
        // Resources - Textures
        LearningNode {
            id: "textures".to_string(),
            name: "Textures & Samplers".to_string(),
            description: "Working with texture resources and sampling".to_string(),
            difficulty: NodeDifficulty::Beginner,
            prerequisites: vec!["buffers".to_string()],
            tutorials: vec!["adding_textures".to_string()],
            examples: vec!["texture_mapping".to_string()],
            category: NodeCategory::Resources,
        },
        // Resources - Bind Groups
        LearningNode {
            id: "bind_groups".to_string(),
            name: "Bind Groups".to_string(),
            description: "Organizing resources with bind groups and layouts".to_string(),
            difficulty: NodeDifficulty::Intermediate,
            prerequisites: vec!["textures".to_string(), "shaders_basic".to_string()],
            tutorials: vec!["adding_textures".to_string()],
            examples: vec!["texture_mapping".to_string(), "cube".to_string()],
            category: NodeCategory::Resources,
        },
        // Rendering - 3D
        LearningNode {
            id: "3d_rendering".to_string(),
            name: "3D Rendering".to_string(),
            description: "Depth testing, transformations, and 3D graphics".to_string(),
            difficulty: NodeDifficulty::Intermediate,
            prerequisites: vec!["render_pipeline".to_string(), "bind_groups".to_string()],
            tutorials: vec!["3d_with_depth".to_string()],
            examples: vec!["cube".to_string(), "rotating_cube".to_string()],
            category: NodeCategory::Rendering,
        },
        // Compute
        LearningNode {
            id: "compute_shaders".to_string(),
            name: "Compute Shaders".to_string(),
            description: "GPU compute operations and compute pipelines".to_string(),
            difficulty: NodeDifficulty::Intermediate,
            prerequisites: vec!["buffers".to_string(), "shaders_basic".to_string()],
            tutorials: vec!["gpu_compute".to_string()],
            examples: vec!["compute_shader".to_string(), "compute_pass".to_string()],
            category: NodeCategory::Compute,
        },
        // Advanced Rendering
        LearningNode {
            id: "instancing".to_string(),
            name: "Instanced Rendering".to_string(),
            description: "Efficiently rendering multiple objects".to_string(),
            difficulty: NodeDifficulty::Intermediate,
            prerequisites: vec!["3d_rendering".to_string()],
            tutorials: vec![],
            examples: vec!["instanced_rendering".to_string()],
            category: NodeCategory::Rendering,
        },
        LearningNode {
            id: "multisampling".to_string(),
            name: "Multisampling".to_string(),
            description: "Anti-aliasing with MSAA".to_string(),
            difficulty: NodeDifficulty::Intermediate,
            prerequisites: vec!["render_pipeline".to_string()],
            tutorials: vec![],
            examples: vec!["multisampling".to_string()],
            category: NodeCategory::Rendering,
        },
        LearningNode {
            id: "render_to_texture".to_string(),
            name: "Render to Texture".to_string(),
            description: "Off-screen rendering and texture targets".to_string(),
            difficulty: NodeDifficulty::Intermediate,
            prerequisites: vec!["textures".to_string(), "render_pipeline".to_string()],
            tutorials: vec![],
            examples: vec!["render_to_texture".to_string()],
            category: NodeCategory::Rendering,
        },
        // Advanced Compute
        LearningNode {
            id: "compute_render_sharing".to_string(),
            name: "Compute-Render Integration".to_string(),
            description: "Combining compute and rendering workflows".to_string(),
            difficulty: NodeDifficulty::Advanced,
            prerequisites: vec!["compute_shaders".to_string(), "3d_rendering".to_string()],
            tutorials: vec![],
            examples: vec!["compute_render_sharing".to_string(), "particle_system".to_string()],
            category: NodeCategory::Compute,
        },
        // Advanced Topics
        LearningNode {
            id: "indirect_drawing".to_string(),
            name: "Indirect Drawing".to_string(),
            description: "GPU-driven rendering with indirect commands".to_string(),
            difficulty: NodeDifficulty::Advanced,
            prerequisites: vec!["3d_rendering".to_string(), "compute_shaders".to_string()],
            tutorials: vec![],
            examples: vec!["indirect_drawing".to_string()],
            category: NodeCategory::Advanced,
        },
        LearningNode {
            id: "lighting_shadows".to_string(),
            name: "Lighting & Shadows".to_string(),
            description: "Advanced lighting techniques and shadow mapping".to_string(),
            difficulty: NodeDifficulty::Advanced,
            prerequisites: vec!["3d_rendering".to_string(), "render_to_texture".to_string()],
            tutorials: vec![],
            examples: vec!["lighting_shadows".to_string()],
            category: NodeCategory::Advanced,
        },
        LearningNode {
            id: "post_processing".to_string(),
            name: "Post Processing".to_string(),
            description: "Screen-space effects and post-processing pipelines".to_string(),
            difficulty: NodeDifficulty::Advanced,
            prerequisites: vec!["render_to_texture".to_string()],
            tutorials: vec![],
            examples: vec!["post_processing".to_string()],
            category: NodeCategory::Advanced,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_learning_path_has_nodes() {
        let path = get_learning_path();
        assert!(!path.is_empty());
        assert!(path.len() > 10); // Should have substantial content
    }

    #[test]
    fn test_all_nodes_have_unique_ids() {
        let path = get_learning_path();
        let mut ids = HashSet::new();
        for node in &path {
            assert!(
                ids.insert(node.id.clone()),
                "Duplicate node ID: {}",
                node.id
            );
        }
    }

    #[test]
    fn test_prerequisites_exist() {
        let path = get_learning_path();
        let ids: HashSet<_> = path.iter().map(|n| n.id.as_str()).collect();

        for node in &path {
            for prereq in &node.prerequisites {
                assert!(
                    ids.contains(prereq.as_str()),
                    "Node '{}' has non-existent prerequisite '{}'",
                    node.id,
                    prereq
                );
            }
        }
    }

    #[test]
    fn test_no_circular_dependencies() {
        let path = get_learning_path();
        // Simple check: no node should have itself as a prerequisite (direct cycle)
        for node in &path {
            assert!(
                !node.prerequisites.contains(&node.id),
                "Node '{}' has itself as prerequisite",
                node.id
            );
        }
    }

    #[test]
    fn test_progress_tracking() {
        let mut progress = LearningProgress::new();

        // Initially, nothing completed
        assert!(progress.completed_tutorials.is_empty());
        assert!(progress.tried_examples.is_empty());

        // Complete a tutorial
        progress.complete_tutorial("hello_triangle".to_string());
        assert!(progress.completed_tutorials.contains("hello_triangle"));

        // Try an example
        progress.try_example("triangle".to_string());
        assert!(progress.tried_examples.contains("triangle"));
    }

    #[test]
    fn test_node_completion_status() {
        let mut progress = LearningProgress::new();
        let nodes = get_learning_path();
        let buffer_node = nodes.iter().find(|n| n.id == "buffers").unwrap();

        // Initially not completed
        assert!(!progress.is_node_completed(buffer_node));

        // Try an associated example
        progress.try_example("triangle".to_string());
        assert!(progress.is_node_completed(buffer_node));
    }

    #[test]
    fn test_prerequisites_check() {
        let mut progress = LearningProgress::new();
        let nodes = get_learning_path();

        let device_node = nodes.iter().find(|n| n.id == "device_setup").unwrap();
        let buffer_node = nodes.iter().find(|n| n.id == "buffers").unwrap();

        // Buffer node requires device setup
        assert!(!progress.are_prerequisites_met(buffer_node, &nodes));

        // Complete device setup
        progress.visit_node("device_setup".to_string());
        // Mark it as completed by trying an example (device_setup has no examples, so manually complete)
        // For nodes without tutorials/examples, they're considered complete by default
        assert!(progress.is_node_completed(device_node));
        assert!(progress.are_prerequisites_met(buffer_node, &nodes));
    }

    #[test]
    fn test_category_names() {
        assert_eq!(NodeCategory::Foundation.name(), "Foundation");
        assert_eq!(NodeCategory::Resources.name(), "Resources");
        assert_eq!(NodeCategory::Shaders.name(), "Shaders");
    }
}
