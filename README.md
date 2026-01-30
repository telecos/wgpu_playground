# wgpu_playground

Repository for experimenting WebGPU capabilities in Rust

## Project Setup

This project has been broken down into 80 well-defined tasks ready for implementation. To create GitHub issues for all tasks:

**Quick Start:**
```bash
python3 create_issues_api.py --token YOUR_GITHUB_TOKEN
```

See [USAGE_INSTRUCTIONS.md](USAGE_INSTRUCTIONS.md) for detailed instructions and alternative methods.

## Available Scripts and Documentation

- **[USAGE_INSTRUCTIONS.md](USAGE_INSTRUCTIONS.md)** - Step-by-step guide to create all GitHub issues
- **[CREATE_ISSUES_README.md](CREATE_ISSUES_README.md)** - Detailed documentation about the tasks and scripts
- **create_issues_api.py** - Python script for creating issues via GitHub REST API
- **create_all_issues.sh** - Bash script for creating issues via GitHub CLI
- **issues_data.json** - JSON export of all 80 tasks

## Task Categories

The project is organized into the following categories:

| Category | Tasks | Count | Labels |
|----------|-------|-------|---------|
| WebGPU API Implementation | TASK-023 to TASK-032 | 10 | `webgpu-api`, `core-functionality` |
| GUI/UI Implementation | TASK-040 to TASK-060 | 21 | `ui`, `gui` |
| Examples | TASK-070 to TASK-076 | 7 | `examples`, `documentation` |
| Testing Infrastructure | TASK-080 to TASK-092 | 13 | `testing`, `quality` |
| CI/CD Pipeline | TASK-100 to TASK-112 | 13 | `ci-cd`, `devops` |
| Documentation | TASK-120 to TASK-125 | 6 | `documentation` |
| Enhancements | TASK-130 to TASK-139 | 10 | `enhancement`, `nice-to-have` |

**Total: 80 tasks**

## Contributing

Once the issues are created, you can start contributing:

1. Pick an issue from the [Issues page](https://github.com/telecos/wgpu_playground/issues)
2. Review the task description and acceptance criteria
3. Implement the feature following Rust and WebGPU best practices
4. Ensure cross-platform compatibility (native + WASM)
5. Write tests as applicable
6. Submit a pull request

## License

[Add license information here]
