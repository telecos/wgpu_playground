# Branch Protection Rules Configuration

This document describes the recommended branch protection rules for the `main` branch to ensure code quality and prevent accidental changes.

## Overview

Branch protection rules enforce quality standards by requiring specific checks to pass before code can be merged into protected branches. This repository uses comprehensive PR checks to validate all changes.

## Recommended Branch Protection Settings

### Required Status Checks

The following status checks should be required to pass before merging:

#### From `PR Checks` workflow (`.github/workflows/pr-checks.yml`):
- **PR Checks** - Combined check that runs all validation in a single job (format, lint, build, tests, security)

#### From `CI` workflow (`.github/workflows/ci.yml`):
- **CI Success** - Overall CI pipeline success check

### Branch Protection Configuration

Apply the following settings to the `main` branch:

#### 1. Require Pull Request Reviews
- **Required approving reviews**: 1
- **Dismiss stale pull request approvals when new commits are pushed**: ✅ Enabled
- **Require review from Code Owners**: ✅ Enabled (if CODEOWNERS file exists)

#### 2. Require Status Checks to Pass
- **Require branches to be up to date before merging**: ✅ Enabled
- **Required status checks**:
  - `PR Checks`
  - `CI Success`

#### 3. Require Conversation Resolution
- **Require conversation resolution before merging**: ✅ Enabled

#### 4. Require Linear History
- **Require linear history**: ⚠️ Optional (enforces squash or rebase merges)

#### 5. Restrictions
- **Restrict who can push to matching branches**: ⚠️ Optional (for private repos with teams)
- **Allow force pushes**: ❌ Disabled
- **Allow deletions**: ❌ Disabled

#### 6. Rules Applied to Administrators
- **Include administrators**: ✅ Enabled (ensures admins follow same rules)

## How to Configure Branch Protection

### Via GitHub Web Interface

1. Go to repository **Settings** → **Branches**
2. Click **Add rule** under "Branch protection rules"
3. Enter `main` as the branch name pattern
4. Configure the settings as described above
5. Click **Create** or **Save changes**

### Via GitHub CLI

```bash
# Install GitHub CLI if not already installed
# https://cli.github.com/

# Set branch protection for main branch
gh api repos/{owner}/{repo}/branches/main/protection \
  --method PUT \
  --field required_status_checks[strict]=true \
  --field required_status_checks[contexts][]="PR Checks" \
  --field required_status_checks[contexts][]="CI Success" \
  --field enforce_admins=true \
  --field required_pull_request_reviews[dismiss_stale_reviews]=true \
  --field required_pull_request_reviews[required_approving_review_count]=1 \
  --field required_conversation_resolution=true \
  --field restrictions=null \
  --field allow_force_pushes=false \
  --field allow_deletions=false
```

### Via Terraform (Infrastructure as Code)

```hcl
resource "github_branch_protection" "main" {
  repository_id = github_repository.repo.node_id
  pattern       = "main"

  required_status_checks {
    strict = true
    contexts = [
      "PR Checks",
      "CI Success"
    ]
  }

  required_pull_request_reviews {
    dismiss_stale_reviews           = true
    require_code_owner_reviews      = true
    required_approving_review_count = 1
  }

  enforce_admins              = true
  require_conversation_resolution = true
  allow_force_pushes          = false
  allow_deletions             = false
}
```

## PR Workflow

### For Contributors

1. **Create a feature branch** from `main`
2. **Make your changes** following the coding standards
3. **Run checks locally** (recommended before pushing):
   ```bash
   # Format code
   cargo fmt --all

   # Run lints
   cargo clippy --workspace --all-targets -- -D warnings

   # Run tests
   cargo test --workspace

   # Check security
   cargo deny check
   ```
4. **Push your branch** and create a pull request
5. **Wait for PR checks** to complete (automated)
6. **Address any failures** and push updates
7. **Request review** once all checks pass
8. **Merge** after approval and passing all checks

### For Reviewers

1. **Review the code** for correctness, clarity, and adherence to standards
2. **Check PR labels** for context (size, type of changes)
3. **Verify all status checks pass** before approving
4. **Leave comments** and request changes if needed
5. **Approve the PR** when satisfied
6. **Merge** using the preferred merge strategy (squash, rebase, or merge commit)

## Automated PR Labels

PRs are automatically labeled based on:

### Size Labels
- `size/xs` - Extra small changes (≤10 lines)
- `size/s` - Small changes (≤100 lines)
- `size/m` - Medium changes (≤500 lines)
- `size/l` - Large changes (≤1000 lines)
- `size/xl` - Extra large changes (>1000 lines)

### Type Labels
- `documentation` - Documentation changes
- `dependencies` - Dependency updates
- `ci/cd` - CI/CD workflow changes
- `configuration` - Configuration file changes
- `core` - Core library changes
- `gui` - GUI changes
- `examples` - Example code changes
- `tests` - Test changes
- `benchmarks` - Benchmark changes
- `shaders` - Shader changes
- `assets` - Asset changes
- `security` - Security-related changes

## Status Check Details

### Format Check
Ensures all Rust code is formatted according to `rustfmt.toml`:
- **Command**: `cargo fmt --all -- --check`
- **Fix**: Run `cargo fmt --all` locally

### Lint Check (Clippy)
Ensures code passes Clippy lints according to `clippy.toml`:
- **Command**: `cargo clippy --workspace --all-targets -- -D warnings`
- **Fix**: Address Clippy warnings in your code

### Build Check
Ensures the project builds successfully:
- **Command**: `cargo build --workspace --all-targets`
- **Fix**: Fix compilation errors

### Test Check
Ensures all tests pass:
- **Command**: `cargo nextest run --workspace --all-targets && cargo test --workspace --doc`
- **Fix**: Fix failing tests

### Security Check
Ensures dependencies meet security and license policies:
- **Commands**: `cargo deny check` and `cargo audit`
- **Fix**: Update dependencies or adjust `deny.toml` for exceptions

## Troubleshooting

### PR Checks Failing

If PR checks fail:

1. **View the workflow logs** in the "Checks" tab of your PR
2. **Identify the failing check** from the summary
3. **Run the failing check locally** using the commands above
4. **Fix the issues** and push your changes
5. **Wait for checks to re-run** automatically

### Merge Conflicts

If your branch has merge conflicts with `main`:

1. **Fetch latest changes**: `git fetch origin`
2. **Rebase your branch**: `git rebase origin/main` or merge: `git merge origin/main`
3. **Resolve conflicts** in your editor
4. **Continue the rebase**: `git rebase --continue` or commit the merge: `git commit`
5. **Force push** (if rebased): `git push --force-with-lease`

### Status Checks Not Running

If status checks don't run:

1. **Check workflow triggers** in `.github/workflows/pr-checks.yml`
2. **Ensure PR targets `main` branch**
3. **Check GitHub Actions** are enabled for the repository
4. **Verify workflow files** have no syntax errors

## Additional Resources

- [GitHub Branch Protection Documentation](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches)
- [GitHub Required Status Checks](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches#require-status-checks-before-merging)
- [Contributing Guide](../CONTRIBUTING.md)
- [CI Testing Documentation](CI_TESTING.md)
