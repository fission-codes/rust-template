#!/bin/bash

# Script to copy workspace Cargo.lock to all example template directories
# This ensures templates have up-to-date lock files for cargo generate

set -e

# Define paths
WORKSPACE_LOCK="examples/Cargo.lock"
EXAMPLES_DIR="examples"

# Check if workspace lock file exists
if [[ ! -f "$WORKSPACE_LOCK" ]]; then
    echo "Error: Workspace Cargo.lock not found at $WORKSPACE_LOCK"
    exit 1
fi

# Check if examples directory exists
if [[ ! -d "$EXAMPLES_DIR" ]]; then
    echo "Error: Examples directory not found at $EXAMPLES_DIR"
    exit 1
fi

# Find all subdirectories in examples/ that contain a Cargo.toml (excluding target)
example_dirs=()
while IFS= read -r -d '' dir; do
    # Skip the target directory
    if [[ "$(basename "$dir")" == "target" ]]; then
        continue
    fi

    # Check if the directory contains a Cargo.toml file
    if [[ -f "$dir/Cargo.toml" ]]; then
        example_dirs+=("$dir")
    fi
done < <(find "$EXAMPLES_DIR" -mindepth 1 -maxdepth 1 -type d -print0)

# Check if we found any example directories
if [[ ${#example_dirs[@]} -eq 0 ]]; then
    echo "No example directories with Cargo.toml found in $EXAMPLES_DIR"
    exit 0
fi

# Copy workspace lock to each example directory
copied_count=0
for example_dir in "${example_dirs[@]}"; do
    target_lock="$example_dir/Cargo.lock"
    cp "$WORKSPACE_LOCK" "$target_lock"
    echo "Copied $WORKSPACE_LOCK to $target_lock"
    ((copied_count++))
done

echo "Successfully copied Cargo.lock to $copied_count example template(s)"
