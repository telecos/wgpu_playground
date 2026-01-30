# GitHub Issues Creation Guide for wgpu_playground

This directory contains scripts and data files to create 80 GitHub issues for the wgpu_playground project tasks.

## Overview

The project has been broken down into 80 atomic tasks across the following categories:

- **WebGPU API Implementation (TASK-023 to TASK-032)**: 10 tasks - Core WebGPU functionality
- **GUI/UI Implementation (TASK-040 to TASK-060)**: 21 tasks - User interface components
- **Example Implementation (TASK-070 to TASK-076)**: 7 tasks - Sample applications
- **Testing Infrastructure (TASK-080 to TASK-092)**: 13 tasks - Test framework and tests
- **CI/CD Pipeline (TASK-100 to TASK-112)**: 13 tasks - Continuous integration and deployment
- **Documentation (TASK-120 to TASK-125)**: 6 tasks - Project documentation
- **Additional Enhancements (TASK-130 to TASK-139)**: 10 tasks - Nice-to-have features

**Total: 80 tasks**

## Files

- `create_issues_api.py` - Python script to create issues via GitHub REST API
- `create_all_issues.sh` - Bash script to create issues via GitHub CLI (`gh`)
- `issues_data.json` - JSON export of all task data for programmatic use

## Methods to Create Issues

### Method 1: Using Python Script with GitHub Token (Recommended)

This method uses the GitHub REST API directly and works anywhere.

```bash
# You'll need a GitHub Personal Access Token with 'repo' scope
# Get one from: https://github.com/settings/tokens

python3 create_issues_api.py --token YOUR_GITHUB_TOKEN
```

### Method 2: Using GitHub CLI

If you have `gh` CLI authenticated:

```bash
bash create_all_issues.sh
```

### Method 3: Using the JSON Data

The `issues_data.json` file contains all task data in structured format. You can use this with your own scripts or tools:

```json
{
  "repository": "telecos/wgpu_playground",
  "tasks": [
    {
      "id": "TASK-023",
      "title": "TASK-023: Implement render pass with all operations",
      "body": "...",
      "labels": ["webgpu-api", "core-functionality"]
    },
    ...
  ]
}
```

### Method 4: Manual Creation via Web Interface

If automated methods don't work, you can create issues manually:

1. Go to https://github.com/telecos/wgpu_playground/issues/new
2. Use the data from `issues_data.json` for each issue
3. Copy the title and body, add the appropriate labels

## Dry Run

To see what issues would be created without actually creating them:

```bash
python3 create_issues_api.py --dry-run
```

## Task Categories and Labels

Each task is labeled based on its category:

- `webgpu-api, core-functionality` - Core WebGPU API implementation
- `ui, gui` - User interface components
- `examples, documentation` - Example applications
- `testing, quality` - Testing infrastructure
- `ci-cd, devops` - CI/CD and automation
- `documentation` - Documentation tasks
- `enhancement, nice-to-have` - Additional features

## Enhanced Issue Description

Each issue includes:

1. **Description**: Detailed explanation of what needs to be done
2. **Task Metadata**: ID, category, and estimated time (1-4 hours)
3. **Dependencies**: Note about potential dependencies
4. **Acceptance Criteria**: Quality standards that must be met

## Priority Guidance

### High Priority (Core Functionality)
- TASK-023 through TASK-032: Core WebGPU API implementation
- TASK-040 through TASK-050: Essential GUI components
- TASK-070 through TASK-073: Basic examples

### Medium Priority (Testing and CI)
- TASK-080 through TASK-092: Testing infrastructure
- TASK-100 through TASK-112: CI/CD pipelines

### Lower Priority (Enhancements)
- TASK-051 through TASK-060: Advanced GUI features
- TASK-074 through TASK-076: Advanced examples
- TASK-120 through TASK-125: Documentation
- TASK-130 through TASK-139: Additional features

## Notes

- Tasks are designed to be atomic and independently implementable
- Most tasks can be worked on in parallel once dependencies are met
- Each task should take 1-4 hours for a single developer
- Cross-platform support (native + WASM) should be considered in all implementation tasks
- All UI tasks should ensure WebGPU API features are fully exposed to users

## Troubleshooting

### Authentication Issues with gh CLI

If `gh` says you're not authenticated:
```bash
gh auth login
```

### Permission Errors

Ensure your GitHub token has the `repo` scope which includes issue creation permissions.

### Rate Limiting

If creating many issues quickly, you might hit GitHub's rate limits. The scripts will report failures, and you can re-run them to create any missing issues.

## Support

For questions or issues with these scripts, please open an issue in the repository.
