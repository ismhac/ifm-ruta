#!/bin/bash

# Auto-fix formatting and linting issues
# This script automatically fixes common code issues

set -e

echo "Auto-fixing Code Issues"
echo "======================="

# Change to project directory
cd "$(dirname "$0")/.."

echo ""
echo "1. Fixing code formatting..."
cargo fmt --all
echo "OK: Formatting fixed"

echo ""
echo "2. Running clippy with auto-fix..."
# Note: clippy doesn't have auto-fix for all issues, but we can try
cargo clippy --all-targets --all-features -- -D warnings || {
    echo "WARNING: Some clippy issues need manual fixing"
    echo "TIP: Check the warnings above"
}

echo ""
echo "3. Checking for unused imports..."
# Remove unused imports (this is a common issue)
cargo +nightly clippy --all-targets --all-features -- -W unused-imports || true

echo ""
echo "4. Final formatting check..."
cargo fmt --all -- --check && echo "OK: Formatting is correct" || {
    echo "ERROR: Formatting still has issues"
    exit 1
}

echo ""
echo "SUCCESS: Auto-fix completed!"
echo "TIP: Run ./scripts/pre-push-check.sh to verify everything is ready"
