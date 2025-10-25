#!/bin/bash

# Push with CI checks script
# This script runs CI checks and then pushes code

set -e

echo "🚀 Push with CI Checks"
echo "======================"

# Change to project directory
cd "$(dirname "$0")/.."

# Run pre-push checks
echo "Running pre-push checks..."
./scripts/pre-push-check.sh

echo ""
echo "📝 Checking git status..."

# Check if there are changes to commit
if git diff --quiet && git diff --cached --quiet; then
    echo "ℹ️  No changes to commit"
    exit 0
fi

# Show git status
git status

echo ""
echo "📦 Staging changes..."
git add .

echo ""
echo "💬 Commit message:"
if [ -n "$1" ]; then
    COMMIT_MSG="$1"
    echo "Using provided message: $COMMIT_MSG"
else
    echo "Please provide a commit message:"
    read -r COMMIT_MSG
fi

echo ""
echo "💾 Committing changes..."
git commit -m "$COMMIT_MSG"

echo ""
echo "🚀 Pushing to remote..."
git push origin master

echo ""
echo "✅ Code pushed successfully!"
echo "🔗 Check CI status: https://github.com/ismhac/ifm-ruta/actions"
