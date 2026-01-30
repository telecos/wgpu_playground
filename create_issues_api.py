#!/usr/bin/env python3
"""
Script to create GitHub issues using the GitHub REST API.
This script can work with or without the GitHub CLI.
"""

import json
import os
import sys
import urllib.request
import urllib.error

# Task data structure
TASKS = [
    {
        "id": "TASK-023",
        "title": "Implement render pass with all operations",
        "description": "Create render pass encoder supporting color attachments, depth-stencil attachments, load/store operations, clear values. Implement draw commands (draw, drawIndexed, drawIndirect, drawIndexedIndirect).",
        "labels": ["webgpu-api", "core-functionality"]
    },
    {
        "id": "TASK-024",
        "title": "Implement compute pass operations",
        "description": "Create compute pass encoder with dispatch operations (dispatch, dispatchIndirect). Support pipeline and bind group setting.",
        "labels": ["webgpu-api", "core-functionality"]
    },
    {
        "id": "TASK-025",
        "title": "Implement buffer-to-buffer and buffer-texture copies",
        "description": "Implement copyBufferToBuffer, copyBufferToTexture, and copyTextureToBuffer operations in command encoder. Handle size and offset validation.",
        "labels": ["webgpu-api", "core-functionality"]
    },
    {
        "id": "TASK-026",
        "title": "Implement texture-to-texture copy operations",
        "description": "Implement copyTextureToTexture operations supporting different mip levels, array layers, and aspects.",
        "labels": ["webgpu-api", "core-functionality"]
    },
    {
        "id": "TASK-027",
        "title": "Implement GPU query sets for timestamps and statistics",
        "description": "Create query set creation supporting occlusion and timestamp queries. Implement query result resolution and retrieval.",
        "labels": ["webgpu-api", "core-functionality"]
    },
    {
        "id": "TASK-028",
        "title": "Implement surface/canvas context management",
        "description": "Create canvas context configuration for render targets. Handle surface creation, configuration (format, present mode, alpha mode), and getCurrentTexture operations.",
        "labels": ["webgpu-api", "core-functionality"]
    },
    {
        "id": "TASK-029",
        "title": "Implement vertex buffer binding and layouts",
        "description": "Create vertex buffer state configuration with multiple buffer slots, step modes (vertex, instance), and attribute formats. Support setVertexBuffer operations in render passes.",
        "labels": ["webgpu-api", "core-functionality"]
    },
    {
        "id": "TASK-030",
        "title": "Implement index buffer binding",
        "description": "Implement index buffer setup with uint16 and uint32 formats. Support setIndexBuffer operations in render passes.",
        "labels": ["webgpu-api", "core-functionality"]
    },
    {
        "id": "TASK-031",
        "title": "Implement render bundles for command reuse",
        "description": "Create render bundle encoder for recording reusable draw commands. Support render bundle execution in render passes for optimization.",
        "labels": ["webgpu-api", "core-functionality"]
    },
    {
        "id": "TASK-032",
        "title": "Implement comprehensive error handling",
        "description": "Set up error scopes, validation errors, out-of-memory errors, and internal errors handling. Implement error callbacks and logging throughout the API.",
        "labels": ["webgpu-api", "core-functionality"]
    },
    {
        "id": "TASK-040",
        "title": "Evaluate and select GUI framework",
        "description": "Research and select appropriate Rust GUI framework (egui, iced, or custom imgui-wgpu). Consider ease of integration with wgpu, WASM support, and feature richness. Document decision rationale.",
        "labels": ["ui", "gui"]
    },
    {
        "id": "TASK-041",
        "title": "Create main application window with GUI framework",
        "description": "Set up main application window using selected GUI framework. Integrate with winit event loop. Create basic layout with menu bar, sidebar, and main canvas area.",
        "labels": ["ui", "gui"]
    },
    {
        "id": "TASK-042",
        "title": "Create UI for GPU adapter selection",
        "description": "Build UI panel displaying available GPU adapters with their properties (name, vendor, device type). Allow user to select adapter and configure power preference options.",
        "labels": ["ui", "gui"]
    },
    {
        "id": "TASK-043",
        "title": "Create UI for device limits and features",
        "description": "Build UI panel showing available device features and limits. Allow users to enable/disable features and adjust limits before device creation.",
        "labels": ["ui", "gui"]
    },
    {
        "id": "TASK-044",
        "title": "Create UI panel for buffer configuration",
        "description": "Build interface for creating GPU buffers with controls for size, usage flags (checkboxes for each flag), label, and mapped-at-creation option. Include validation and creation button.",
        "labels": ["ui", "gui"]
    },
    {
        "id": "TASK-045",
        "title": "Create UI panel for texture configuration",
        "description": "Build interface for creating textures with controls for dimensions, format (dropdown), mip levels, sample count, usage flags, and label. Support all texture formats from the spec.",
        "labels": ["ui", "gui"]
    },
    {
        "id": "TASK-046",
        "title": "Create UI panel for sampler settings",
        "description": "Build sampler configuration interface with controls for address modes (U, V, W), filter modes (mag, min, mipmap), LOD clamp, compare function, and anisotropy.",
        "labels": ["ui", "gui"]
    },
    {
        "id": "TASK-047",
        "title": "Create WGSL shader editor with syntax highlighting",
        "description": "Implement shader editor panel with WGSL syntax highlighting, line numbers, and compilation error display. Support loading from file and inline editing. Show compilation results.",
        "labels": ["ui", "gui"]
    },
    {
        "id": "TASK-048",
        "title": "Create UI for bind group layout configuration",
        "description": "Build interface for defining bind group layouts with dynamic entry addition. For each entry: binding number, visibility (vertex/fragment/compute checkboxes), and resource type configuration.",
        "labels": ["ui", "gui"]
    },
    {
        "id": "TASK-049",
        "title": "Create UI for bind group resource binding",
        "description": "Build interface for creating bind groups by selecting layout and binding resources. Display available resources (buffers, textures, samplers) and allow assignment to binding slots.",
        "labels": ["ui", "gui"]
    },
    {
        "id": "TASK-050",
        "title": "Create UI for render pipeline configuration",
        "description": "Build comprehensive render pipeline editor with sections for vertex state, primitive state (topology, culling, front face), depth-stencil, multisample, and fragment state. Include preset configurations.",
        "labels": ["ui", "gui"]
    },
    {
        "id": "TASK-051",
        "title": "Create UI for compute pipeline configuration",
        "description": "Build compute pipeline editor with shader module selection, entry point input, and pipeline layout configuration.",
        "labels": ["ui", "gui"]
    },
    {
        "id": "TASK-052",
        "title": "Create UI for render pass configuration",
        "description": "Build interface for configuring render pass with color attachments (load/store ops, clear color), depth-stencil attachment, and timestamp writes.",
        "labels": ["ui", "gui"]
    },
    {
        "id": "TASK-053",
        "title": "Create UI for draw command parameters",
        "description": "Build interface for executing draw commands with controls for vertex count, instance count, first vertex/instance, and indexed drawing parameters.",
        "labels": ["ui", "gui"]
    },
    {
        "id": "TASK-054",
        "title": "Create UI for compute dispatch configuration",
        "description": "Build interface for compute dispatch with workgroup count inputs (X, Y, Z dimensions) and indirect dispatch buffer selection.",
        "labels": ["ui", "gui"]
    },
    {
        "id": "TASK-055",
        "title": "Create resource inspector panel",
        "description": "Build panel displaying all created resources (buffers, textures, pipelines) with their properties, current state, and memory usage. Support filtering and searching.",
        "labels": ["ui", "gui"]
    },
    {
        "id": "TASK-056",
        "title": "Create main rendering canvas with controls",
        "description": "Implement main canvas area for WebGPU rendering output. Add controls for clear color, canvas size, and screenshot capture. Support mouse interaction for camera control in 3D examples.",
        "labels": ["ui", "gui"]
    },
    {
        "id": "TASK-057",
        "title": "Create command recording and playback panel",
        "description": "Build panel showing recorded GPU commands with timeline. Support command inspection, replay, and export. Display command buffer contents.",
        "labels": ["ui", "gui"]
    },
    {
        "id": "TASK-058",
        "title": "Create example gallery and loader",
        "description": "Build UI for browsing and loading preset examples (triangle, cube, texture mapping, compute shader). Include example descriptions and source code display.",
        "labels": ["ui", "gui"]
    },
    {
        "id": "TASK-059",
        "title": "Create performance metrics panel",
        "description": "Build panel displaying FPS, frame time, GPU memory usage, and command buffer statistics. Support performance graphs and profiling data.",
        "labels": ["ui", "gui"]
    },
    {
        "id": "TASK-060",
        "title": "Create error and warning console",
        "description": "Build console panel displaying WebGPU errors, warnings, and validation messages. Support filtering by severity and clearing. Include error details and stack traces.",
        "labels": ["ui", "gui"]
    },
    {
        "id": "TASK-070",
        "title": "Create basic triangle rendering example",
        "description": "Implement classic triangle example with vertex buffer, simple shader, and render pipeline. Demonstrate basic rendering setup and draw command.",
        "labels": ["examples", "documentation"]
    },
    {
        "id": "TASK-071",
        "title": "Create texture mapping example",
        "description": "Implement textured quad example demonstrating texture creation, sampler configuration, and texture binding in shaders.",
        "labels": ["examples", "documentation"]
    },
    {
        "id": "TASK-072",
        "title": "Create rotating 3D cube example",
        "description": "Implement 3D cube with rotation using uniform buffers for transformation matrices. Demonstrate depth testing and index buffers.",
        "labels": ["examples", "documentation"]
    },
    {
        "id": "TASK-073",
        "title": "Create basic compute shader example",
        "description": "Implement compute shader example performing simple calculations (e.g., array processing). Demonstrate compute pipeline and buffer sharing between compute and render.",
        "labels": ["examples", "documentation"]
    },
    {
        "id": "TASK-074",
        "title": "Create instanced rendering example",
        "description": "Implement instanced rendering example with multiple objects. Demonstrate instance buffers and per-instance attributes.",
        "labels": ["examples", "documentation"]
    },
    {
        "id": "TASK-075",
        "title": "Create render-to-texture example",
        "description": "Implement example rendering to texture and using it in subsequent render pass. Demonstrate framebuffer usage and multi-pass rendering.",
        "labels": ["examples", "documentation"]
    },
    {
        "id": "TASK-076",
        "title": "Create multisampling example",
        "description": "Implement example using multisampling for anti-aliasing. Demonstrate MSAA render targets and resolve operations.",
        "labels": ["examples", "documentation"]
    },
    {
        "id": "TASK-080",
        "title": "Configure unit testing infrastructure",
        "description": "Set up Rust unit testing framework with test modules in each crate. Configure test organization following Rust best practices. Add test utilities and helper functions.",
        "labels": ["testing", "quality"]
    },
    {
        "id": "TASK-081",
        "title": "Create unit tests for buffer operations",
        "description": "Write unit tests for buffer creation, mapping, writing, and reading. Test all usage flag combinations and error conditions.",
        "labels": ["testing", "quality"]
    },
    {
        "id": "TASK-082",
        "title": "Create unit tests for texture operations",
        "description": "Write unit tests for texture creation, format support, dimension validation, and texture operations. Test error conditions and edge cases.",
        "labels": ["testing", "quality"]
    },
    {
        "id": "TASK-083",
        "title": "Create unit tests for pipeline creation",
        "description": "Write unit tests for render and compute pipeline creation. Test valid and invalid configurations, shader compilation, and pipeline state.",
        "labels": ["testing", "quality"]
    },
    {
        "id": "TASK-084",
        "title": "Create unit tests for command encoding",
        "description": "Write unit tests for command encoder operations, render/compute pass recording, and copy operations. Validate command sequences.",
        "labels": ["testing", "quality"]
    },
    {
        "id": "TASK-085",
        "title": "Create integration tests for complete workflows",
        "description": "Write integration tests for complete rendering workflows (setup → encode → submit). Test multiple examples end-to-end.",
        "labels": ["testing", "quality"]
    },
    {
        "id": "TASK-086",
        "title": "Configure headless GPU testing",
        "description": "Set up headless testing using software adapter or offscreen rendering. Enable tests to run in CI without display. Configure appropriate backends for testing.",
        "labels": ["testing", "quality"]
    },
    {
        "id": "TASK-087",
        "title": "Create visual regression test framework",
        "description": "Set up visual regression testing by capturing rendered output and comparing with reference images. Use image comparison libraries. Store reference images in repository.",
        "labels": ["testing", "quality"]
    },
    {
        "id": "TASK-088",
        "title": "Create GUI interaction tests",
        "description": "Write tests for GUI components and user interactions. Test UI state management, input handling, and rendering output. Mock user input events.",
        "labels": ["testing", "quality"]
    },
    {
        "id": "TASK-089",
        "title": "Create performance benchmark suite",
        "description": "Set up criterion.rs or similar benchmarking framework. Create benchmarks for critical paths (buffer operations, draw calls, pipeline creation). Configure benchmark CI jobs.",
        "labels": ["testing", "quality"]
    },
    {
        "id": "TASK-090",
        "title": "Create WASM-specific tests",
        "description": "Write tests specifically for WASM build. Test web-sys integration, wasm-bindgen exports, and browser-specific functionality. Configure wasm-pack test.",
        "labels": ["testing", "quality"]
    },
    {
        "id": "TASK-091",
        "title": "Configure code coverage tools",
        "description": "Set up tarpaulin or llvm-cov for test coverage reporting. Configure coverage thresholds and reporting format. Integrate with CI.",
        "labels": ["testing", "quality"]
    },
    {
        "id": "TASK-092",
        "title": "Create tests for error conditions",
        "description": "Write tests validating error handling for invalid operations, out-of-bounds access, device lost scenarios, and validation errors.",
        "labels": ["testing", "quality"]
    },
    {
        "id": "TASK-100",
        "title": "Create base GitHub Actions CI configuration",
        "description": "Set up .github/workflows directory with main CI workflow. Configure triggers (push, PR) and basic job structure. Set up caching for cargo dependencies.",
        "labels": ["ci-cd", "devops"]
    },
    {
        "id": "TASK-101",
        "title": "Create CI jobs for native builds",
        "description": "Configure CI jobs for building on Linux, macOS, and Windows. Set up Rust toolchain installation and build matrix. Test all native targets.",
        "labels": ["ci-cd", "devops"]
    },
    {
        "id": "TASK-102",
        "title": "Create CI jobs for WASM builds",
        "description": "Configure wasm-pack in CI. Create jobs for building and testing WASM target. Validate web bundle creation and deployment artifacts.",
        "labels": ["ci-cd", "devops"]
    },
    {
        "id": "TASK-103",
        "title": "Create CI jobs for code quality checks",
        "description": "Set up clippy for linting with strict rules. Configure rustfmt checks for code formatting. Fail CI on warnings or formatting issues.",
        "labels": ["ci-cd", "devops"]
    },
    {
        "id": "TASK-104",
        "title": "Create CI jobs for running test suite",
        "description": "Configure jobs running unit tests, integration tests, and doc tests. Set up test reporting and failure notifications. Run tests on all platforms.",
        "labels": ["ci-cd", "devops"]
    },
    {
        "id": "TASK-105",
        "title": "Create CI jobs for security scanning",
        "description": "Set up cargo-audit for dependency vulnerability scanning. Configure cargo-deny for license and security policy enforcement. Run security checks on schedule and PRs.",
        "labels": ["ci-cd", "devops"]
    },
    {
        "id": "TASK-106",
        "title": "Create CI jobs for documentation building",
        "description": "Configure cargo doc generation in CI. Build and publish documentation to GitHub Pages. Validate documentation completeness and links.",
        "labels": ["ci-cd", "devops"]
    },
    {
        "id": "TASK-107",
        "title": "Create CI jobs for performance benchmarks",
        "description": "Set up benchmark running on schedule or manual trigger. Compare results against baseline. Store and visualize benchmark history.",
        "labels": ["ci-cd", "devops"]
    },
    {
        "id": "TASK-108",
        "title": "Create CI jobs for release artifacts",
        "description": "Configure artifact creation for releases: native binaries, WASM bundles, and documentation. Set up automatic publishing to GitHub Releases.",
        "labels": ["ci-cd", "devops"]
    },
    {
        "id": "TASK-109",
        "title": "Create deployment workflow for WASM demo",
        "description": "Set up automatic deployment of WASM build to GitHub Pages or other hosting. Deploy on main branch updates. Configure custom domain if applicable.",
        "labels": ["ci-cd", "devops"]
    },
    {
        "id": "TASK-110",
        "title": "Configure Dependabot or similar for updates",
        "description": "Set up automated dependency update PRs. Configure update frequency and grouping. Add auto-merge for minor updates passing CI.",
        "labels": ["ci-cd", "devops"]
    },
    {
        "id": "TASK-111",
        "title": "Create comprehensive PR check workflow",
        "description": "Configure required status checks for PRs: builds, tests, linting, formatting. Set up PR labeling based on changes. Configure branch protection rules.",
        "labels": ["ci-cd", "devops"]
    },
    {
        "id": "TASK-112",
        "title": "Create CI jobs for coverage reporting",
        "description": "Configure coverage collection in CI. Upload results to Codecov or similar. Add coverage badges to README. Set minimum coverage thresholds.",
        "labels": ["ci-cd", "devops"]
    },
    {
        "id": "TASK-120",
        "title": "Document system architecture and design",
        "description": "Create docs/architecture.md documenting overall system design, module structure, data flow, and key design decisions. Include diagrams if applicable.",
        "labels": ["documentation"]
    },
    {
        "id": "TASK-121",
        "title": "Document public API with examples",
        "description": "Write comprehensive rustdoc comments for all public APIs. Include usage examples, parameter descriptions, and return value documentation. Document error conditions.",
        "labels": ["documentation"]
    },
    {
        "id": "TASK-122",
        "title": "Create end-user documentation",
        "description": "Write user guide covering GUI usage, example workflows, and common tasks. Include screenshots and step-by-step tutorials.",
        "labels": ["documentation"]
    },
    {
        "id": "TASK-123",
        "title": "Create developer/contributor guide",
        "description": "Write guide for developers contributing to the project. Cover development setup, coding standards, PR process, and testing requirements.",
        "labels": ["documentation"]
    },
    {
        "id": "TASK-124",
        "title": "Document WGSL shader development",
        "description": "Create guide for writing WGSL shaders in the playground. Cover shader structure, built-in functions, and debugging techniques.",
        "labels": ["documentation"]
    },
    {
        "id": "TASK-125",
        "title": "Document WebGPU API coverage",
        "description": "Create comprehensive document mapping WebGPU API features to playground implementation. Mark implemented, partial, and missing features.",
        "labels": ["documentation"]
    },
    {
        "id": "TASK-130",
        "title": "Add hot reload for shader changes",
        "description": "Implement file watching for shader files and automatic reload on changes. Update pipelines dynamically without restarting application.",
        "labels": ["enhancement", "nice-to-have"]
    },
    {
        "id": "TASK-131",
        "title": "Add saving and loading of playground state",
        "description": "Implement serialization of current playground state (resources, pipeline configs, shaders). Support loading saved states. Use JSON or binary format.",
        "labels": ["enhancement", "nice-to-have"]
    },
    {
        "id": "TASK-132",
        "title": "Add standalone code generation",
        "description": "Generate standalone Rust code from current playground configuration. Export as buildable cargo project. Include all shaders and resources.",
        "labels": ["enhancement", "nice-to-have"]
    },
    {
        "id": "TASK-133",
        "title": "Add theme switching support",
        "description": "Implement dark and light UI themes. Add theme selector in settings. Persist theme preference.",
        "labels": ["enhancement", "nice-to-have"]
    },
    {
        "id": "TASK-134",
        "title": "Add sharing and collaboration features",
        "description": "Implement URL-based state sharing (encode state in URL). Optional: Add cloud save for sharing configurations. Generate shareable links.",
        "labels": ["enhancement", "nice-to-have"]
    },
    {
        "id": "TASK-135",
        "title": "Add texture loading from files",
        "description": "Implement texture loading from image files (PNG, JPG, etc.). Support drag-and-drop. Include image decoding libraries. Allow texture export.",
        "labels": ["enhancement", "nice-to-have"]
    },
    {
        "id": "TASK-136",
        "title": "Add 3D model import support",
        "description": "Implement loading of 3D models (glTF, OBJ). Parse model data into buffers. Support materials and textures from model files.",
        "labels": ["enhancement", "nice-to-have"]
    },
    {
        "id": "TASK-137",
        "title": "Add GPU debugging utilities",
        "description": "Implement debugging tools: buffer inspector (view buffer contents), texture inspector (visualize textures), and pipeline debugger.",
        "labels": ["enhancement", "nice-to-have"]
    },
    {
        "id": "TASK-138",
        "title": "Add mobile device support",
        "description": "Optimize UI for mobile screens. Test on mobile browsers with WebGPU support. Implement touch controls and responsive layout.",
        "labels": ["enhancement", "nice-to-have"]
    },
    {
        "id": "TASK-139",
        "title": "Add accessibility improvements",
        "description": "Implement keyboard navigation for all UI elements. Add ARIA labels. Support screen readers. Ensure sufficient contrast ratios.",
        "labels": ["enhancement", "nice-to-have"]
    },
]


def create_issue_body(task):
    """Create enhanced issue body with metadata."""
    return f"""{task['description']}

## Task Metadata
- **Task ID**: {task['id']}
- **Category**: {', '.join(task['labels'])}
- **Estimated Time**: 1-4 hours

## Dependencies
This task may depend on completion of previous tasks in its category. Please check the project roadmap for dependencies.

## Acceptance Criteria
- Implementation follows Rust and WebGPU best practices
- Code includes appropriate error handling
- Changes are tested (unit/integration tests as applicable)
- Documentation is updated if needed
- Cross-platform compatibility maintained (native + WASM)
"""


def create_issues_with_api(token, repo='telecos/wgpu_playground'):
    """Create issues using GitHub REST API."""
    base_url = f"https://api.github.com/repos/{repo}/issues"
    
    created = []
    failed = []
    
    for task in TASKS:
        issue_data = {
            "title": f"{task['id']}: {task['title']}",
            "body": create_issue_body(task),
            "labels": task['labels']
        }
        
        request = urllib.request.Request(
            base_url,
            data=json.dumps(issue_data).encode('utf-8'),
            headers={
                'Authorization': f'Bearer {token}',
                'Accept': 'application/vnd.github.v3+json',
                'Content-Type': 'application/json',
            },
            method='POST'
        )
        
        try:
            with urllib.request.urlopen(request) as response:
                result = json.loads(response.read().decode('utf-8'))
                print(f"✓ Created: {task['id']} - #{result['number']}")
                created.append(task['id'])
        except urllib.error.HTTPError as e:
            error_body = e.read().decode('utf-8')
            print(f"✗ Failed: {task['id']} - {e.code} {e.reason}")
            print(f"  Error: {error_body}")
            failed.append(task['id'])
        except Exception as e:
            print(f"✗ Failed: {task['id']} - {str(e)}")
            failed.append(task['id'])
    
    print(f"\nSummary: {len(created)} created, {len(failed)} failed")
    if failed:
        print(f"Failed tasks: {', '.join(failed)}")
    
    return len(failed) == 0


def export_to_json(filename):
    """Export tasks to JSON for external use."""
    data = {
        "repository": "telecos/wgpu_playground",
        "tasks": [
            {
                "id": task["id"],
                "title": f"{task['id']}: {task['title']}",
                "body": create_issue_body(task),
                "labels": task["labels"]
            }
            for task in TASKS
        ]
    }
    
    with open(filename, 'w') as f:
        json.dump(data, f, indent=2)
    
    print(f"Exported {len(TASKS)} tasks to {filename}")


if __name__ == "__main__":
    import argparse
    
    parser = argparse.ArgumentParser(description='Create GitHub issues for wgpu_playground tasks')
    parser.add_argument('--token', help='GitHub personal access token')
    parser.add_argument('--repo', default='telecos/wgpu_playground', help='GitHub repository (owner/name)')
    parser.add_argument('--export', help='Export to JSON file instead of creating issues')
    parser.add_argument('--dry-run', action='store_true', help='Print what would be created without creating')
    
    args = parser.parse_args()
    
    if args.export:
        export_to_json(args.export)
    elif args.dry_run:
        print(f"Would create {len(TASKS)} issues:")
        for task in TASKS:
            print(f"  - {task['id']}: {task['title']} [{', '.join(task['labels'])}]")
    elif args.token:
        success = create_issues_with_api(args.token, args.repo)
        sys.exit(0 if success else 1)
    else:
        print("Error: Either provide --token for creating issues or --export for exporting to JSON")
        print("Usage:")
        print("  python3 create_issues_api.py --token YOUR_GITHUB_TOKEN [--repo owner/name]")
        print("  python3 create_issues_api.py --export issues_data.json")
        print("  python3 create_issues_api.py --dry-run")
        sys.exit(1)
