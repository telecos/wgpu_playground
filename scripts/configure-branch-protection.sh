#!/bin/bash
# Branch Protection Configuration Script
# This script helps configure branch protection rules for the main branch

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Branch Protection Configuration Helper${NC}"
echo ""
echo "This script helps you configure branch protection rules for the 'main' branch."
echo "You can use this with GitHub CLI or manually via the GitHub web interface."
echo ""

# Check if gh CLI is installed
if ! command -v gh &> /dev/null; then
    echo -e "${YELLOW}GitHub CLI (gh) is not installed.${NC}"
    echo "Please install it from: https://cli.github.com/"
    echo ""
    echo "Alternative: Configure manually via GitHub web interface"
    echo "Go to: Settings → Branches → Add rule"
    echo ""
    exit 1
fi

# Get repository information
REPO=$(gh repo view --json nameWithOwner -q .nameWithOwner)
echo -e "Repository: ${GREEN}${REPO}${NC}"
echo ""

# Prompt for confirmation
read -p "Do you want to configure branch protection for 'main' branch? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Cancelled."
    exit 0
fi

echo -e "${YELLOW}Configuring branch protection rules...${NC}"

# Required status checks
REQUIRED_CHECKS=(
    "PR Checks"
    "CI Success"
)

echo ""
echo "Required status checks that will be enforced:"
for check in "${REQUIRED_CHECKS[@]}"; do
    echo "  ✓ $check"
done
echo ""

# Create branch protection rule using GitHub API
# Note: This uses the GitHub API via gh CLI
gh api \
  repos/${REPO}/branches/main/protection \
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
  --field allow_deletions=false \
  > /dev/null 2>&1

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Branch protection rules configured successfully!${NC}"
    echo ""
    echo "The following settings have been applied:"
    echo "  ✓ Require pull request reviews (1 approval)"
    echo "  ✓ Dismiss stale reviews when new commits are pushed"
    echo "  ✓ Require status checks to pass before merging"
    echo "  ✓ Require branches to be up to date before merging"
    echo "  ✓ Require conversation resolution before merging"
    echo "  ✓ Include administrators in restrictions"
    echo "  ✓ Disable force pushes"
    echo "  ✓ Disable branch deletion"
    echo ""
    echo "View settings at: https://github.com/${REPO}/settings/branches"
else
    echo -e "${RED}❌ Failed to configure branch protection rules${NC}"
    echo ""
    echo "You may need to configure manually via GitHub web interface:"
    echo "Go to: https://github.com/${REPO}/settings/branches"
    echo ""
    echo "See docs/BRANCH_PROTECTION.md for detailed instructions."
    exit 1
fi

echo -e "${GREEN}Configuration complete!${NC}"
