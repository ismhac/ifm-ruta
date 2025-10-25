#!/bin/bash

# Pre-push CI check script
# This script runs all CI checks locally before pushing code

set -e

echo "🔍 Running Pre-Push CI Checks..."
echo "================================"

# Change to project directory
cd "$(dirname "$0")/.."

echo ""
echo "1️⃣ Checking code formatting..."
if cargo fmt --all -- --check; then
    echo "✅ Formatting check passed"
else
    echo "❌ Formatting check failed"
    echo "💡 Run: cargo fmt --all"
    exit 1
fi

echo ""
echo "2️⃣ Running clippy lints..."
if cargo clippy --all-targets --all-features -- -D warnings; then
    echo "✅ Clippy check passed"
else
    echo "❌ Clippy check failed"
    echo "💡 Fix the warnings above"
    exit 1
fi

echo ""
echo "3️⃣ Running tests..."
if cargo test --workspace; then
    echo "✅ Tests passed"
else
    echo "❌ Tests failed"
    echo "💡 Fix the failing tests"
    exit 1
fi

echo ""
echo "4️⃣ Building project..."
if cargo build --release --workspace; then
    echo "✅ Build successful"
else
    echo "❌ Build failed"
    echo "💡 Fix the compilation errors"
    exit 1
fi

echo ""
echo "5️⃣ Testing MCP server..."
if ./scripts/test-mcp.sh; then
    echo "✅ MCP server test passed"
else
    echo "❌ MCP server test failed"
    echo "💡 Check MCP server configuration"
    exit 1
fi

echo ""
echo "🎉 All CI checks passed!"
echo "✅ Ready to push code"
echo ""
echo "Next steps:"
echo "  git add ."
echo "  git commit -m \"Your commit message\""
echo "  git push origin master"
echo ""
echo "Or run: ./scripts/push-with-checks.sh"
