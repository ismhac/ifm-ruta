#!/bin/bash

# Pre-push CI check script
# This script runs all CI checks locally before pushing code

set -e

echo "Running Pre-Push CI Checks..."
echo "=============================="

# Change to project directory
cd "$(dirname "$0")/.."

echo ""
echo "1. Checking code formatting..."
if cargo fmt --all -- --check; then
    echo "OK: Formatting check passed"
else
    echo "ERROR: Formatting check failed"
    echo "TIP: Run: cargo fmt --all"
    exit 1
fi

echo ""
echo "2. Running clippy lints..."
if cargo clippy --all-targets --all-features -- -D warnings; then
    echo "OK: Clippy check passed"
else
    echo "ERROR: Clippy check failed"
    echo "TIP: Fix the warnings above"
    exit 1
fi

echo ""
echo "3. Running tests..."
if cargo test --workspace; then
    echo "OK: Tests passed"
else
    echo "ERROR: Tests failed"
    echo "TIP: Fix the failing tests"
    exit 1
fi

echo ""
echo "4. Building project..."
if cargo build --release --workspace; then
    echo "OK: Build successful"
else
    echo "ERROR: Build failed"
    echo "TIP: Fix the compilation errors"
    exit 1
fi

echo ""
echo "5. Testing MCP server..."
if ./scripts/test-mcp.sh; then
    echo "OK: MCP server test passed"
else
    echo "ERROR: MCP server test failed"
    echo "TIP: Check MCP server configuration"
    exit 1
fi

echo ""
echo "SUCCESS: All CI checks passed!"
echo "Ready to push code"
echo ""
echo "Next steps:"
echo "  git add ."
echo "  git commit -m \"Your commit message\""
echo "  git push origin master"
echo ""
echo "Or run: ./scripts/push-with-checks.sh"
