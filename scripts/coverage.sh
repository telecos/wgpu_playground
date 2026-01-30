#!/usr/bin/env bash
# Script to generate code coverage reports
# Usage: ./scripts/coverage.sh [html|lcov|summary]

set -e

# Check if cargo-llvm-cov is installed
if ! command -v cargo-llvm-cov &> /dev/null; then
    echo "cargo-llvm-cov not found. Installing..."
    cargo install cargo-llvm-cov
fi

# Default to HTML if no argument provided
REPORT_TYPE="${1:-html}"

case "$REPORT_TYPE" in
    html)
        echo "Generating HTML coverage report..."
        cargo llvm-cov --all-features --workspace --html
        echo ""
        echo "Coverage report generated at: target/llvm-cov/html/index.html"
        echo "Open it in your browser to view the report."
        ;;
    lcov)
        echo "Generating LCOV coverage report..."
        cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info
        echo ""
        echo "LCOV report generated at: lcov.info"
        ;;
    summary)
        echo "Generating coverage summary..."
        cargo llvm-cov --all-features --workspace --summary-only
        ;;
    *)
        echo "Unknown report type: $REPORT_TYPE"
        echo "Usage: $0 [html|lcov|summary]"
        exit 1
        ;;
esac
