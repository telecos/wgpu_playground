# How to Create GitHub Issues for wgpu_playground Tasks

This guide provides step-by-step instructions to create all 80 GitHub issues for the project tasks.

## Quick Start (Recommended)

### Option 1: Using Python Script (Works Anywhere)

1. **Get a GitHub Personal Access Token:**
   - Go to https://github.com/settings/tokens
   - Click "Generate new token" → "Generate new token (classic)"
   - Give it a name like "wgpu_playground_issues"
   - Select the `repo` scope (which includes issue creation)
   - Click "Generate token"
   - **Copy the token immediately** (you won't see it again)

2. **Run the script:**
   ```bash
   python3 create_issues_api.py --token YOUR_GITHUB_TOKEN_HERE
   ```

3. **Wait for completion:**
   - The script will create all 80 issues
   - You'll see progress messages for each issue
   - Successful creations show a ✓
   - Failed creations show a ✗ with error details

### Option 2: Using GitHub CLI

1. **Authenticate GitHub CLI:**
   ```bash
   gh auth login
   ```
   Follow the prompts to authenticate

2. **Run the script:**
   ```bash
   bash create_all_issues.sh
   ```

## Verification

After running either script, verify the issues were created:

```bash
# List recent issues
gh issue list --repo telecos/wgpu_playground --limit 100

# Or visit: https://github.com/telecos/wgpu_playground/issues
```

## What Gets Created

Each of the 80 tasks will be created as a GitHub issue with:

- **Title**: `TASK-XXX: Description`
- **Body**: Including:
  - Detailed description
  - Task metadata (ID, category, estimated time)
  - Dependencies information
  - Acceptance criteria
- **Labels**: Appropriate category labels

### Task Breakdown

| Category | Task Range | Count | Labels |
|----------|------------|-------|---------|
| WebGPU API Implementation | TASK-023 to TASK-032 | 10 | `webgpu-api`, `core-functionality` |
| GUI/UI Implementation | TASK-040 to TASK-060 | 21 | `ui`, `gui` |
| Examples | TASK-070 to TASK-076 | 7 | `examples`, `documentation` |
| Testing Infrastructure | TASK-080 to TASK-092 | 13 | `testing`, `quality` |
| CI/CD Pipeline | TASK-100 to TASK-112 | 13 | `ci-cd`, `devops` |
| Documentation | TASK-120 to TASK-125 | 6 | `documentation` |
| Enhancements | TASK-130 to TASK-139 | 10 | `enhancement`, `nice-to-have` |

## Dry Run

To see what would be created without actually creating issues:

```bash
python3 create_issues_api.py --dry-run
```

## Troubleshooting

### Issue: "Authentication failed"

**Solution**: Ensure your GitHub token has the `repo` scope. Generate a new token if needed.

### Issue: "Rate limit exceeded"

**Solution**: GitHub has rate limits. Wait a few minutes and run the script again. It will skip already-created issues.

### Issue: Labels don't exist

**Solution**: The script will create issues even if labels don't exist. You can create the labels manually:
- Go to https://github.com/telecos/wgpu_playground/labels
- Create the following labels:
  - `webgpu-api` (color: #0052cc)
  - `core-functionality` (color: #d73a4a)
  - `ui` (color: #1d76db)
  - `gui` (color: #1d76db)
  - `examples` (color: #0e8a16)
  - `documentation` (color: #0075ca)
  - `testing` (color: #fbca04)
  - `quality` (color: #fbca04)
  - `ci-cd` (color: #5319e7)
  - `devops` (color: #5319e7)
  - `enhancement` (color: #a2eeef)
  - `nice-to-have` (color: #d4c5f9)

## Alternative: Manual Creation

If automated methods fail, you can use `issues_data.json`:

1. Open the file to see all task data
2. Go to https://github.com/telecos/wgpu_playground/issues/new
3. For each task in the JSON:
   - Copy the `title`
   - Copy the `body`
   - Add the `labels`
   - Click "Submit new issue"

## Files Reference

- **create_issues_api.py** - Python script using GitHub REST API
- **create_all_issues.sh** - Bash script using GitHub CLI
- **issues_data.json** - JSON export of all task data
- **CREATE_ISSUES_README.md** - Detailed documentation
- **USAGE_INSTRUCTIONS.md** - This file

## Security Notes

- Never commit your GitHub token to the repository
- Use a token with minimal required permissions (`repo` scope only)
- Revoke the token after use if it was created specifically for this task
- The scripts don't store or transmit your token anywhere except to GitHub's API

## Success Criteria

✅ All 80 issues created successfully
✅ Each issue has correct title, body, and labels
✅ Issues are numbered/organized properly
✅ No duplicate issues

## Next Steps

After creating the issues:

1. Review the issues at https://github.com/telecos/wgpu_playground/issues
2. Organize them into milestones if desired
3. Assign issues to team members
4. Set up a project board for tracking progress
5. Prioritize issues based on the guidance in CREATE_ISSUES_README.md

## Support

If you encounter any problems, please:
1. Check this documentation thoroughly
2. Verify your GitHub token has correct permissions
3. Check GitHub's status page: https://www.githubstatus.com/
4. Open an issue with the error details if problems persist
