#!/bin/bash

set -e

echo "Installing git hooks..."

# Build the pre-commit hook
cd .hooks/pre-commit
cargo build --release

# Ensure git hooks directory exists
git_hooks_dir="$(git rev-parse --git-dir)/hooks"
mkdir -p "$git_hooks_dir"

# Install the pre-commit hook
cp target/release/pre-commit "$git_hooks_dir/"
chmod +x "$git_hooks_dir/pre-commit"

echo "Git hooks installed successfully!"