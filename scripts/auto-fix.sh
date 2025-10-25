#!/bin/bash

# Auto-fix formatting and linting issues
# This script automatically fixes common code issues

set -e

echo "🔧 Auto-fixing Code Issues"
echo "=========================="

# Change to project directory
cd "$(dirname "$0")/.."

echo ""
echo "1️⃣ Fixing code formatting..."
cargo fmt --all
echo "✅ Formatting fixed"

echo ""
echo "2️⃣ Running clippy with auto-fix..."
# Note: clippy doesn't have auto-fix for all issues, but we can try
cargo clippy --all-targets --all-features -- -D warnings || {
    echo "⚠️  Some clippy issues need manual fixing"
    echo "💡 Check the warnings above"
}

echo ""
echo "3️⃣ Checking for unused imports..."
# Remove unused imports (this is a common issue)
cargo +nightly clippy --all-targets --all-features -- -W unused-imports || true

echo ""
echo "4️⃣ Final formatting check..."
cargo fmt --all -- --check && echo "✅ Formatting is correct" || {
    echo "❌ Formatting still has issues"
    exit 1
}

echo ""
echo "🎉 Auto-fix completed!"
echo "💡 Run ./scripts/pre-push-check.sh to verify everything is ready"
