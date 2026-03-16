#!/usr/bin/env bash
# Setup plan supporting files for a feature
source "$(dirname "$0")/common.sh"

feature="${1:-}"
root="$(get_repo_root)"

if [ -z "$feature" ]; then
    feature="$(get_current_branch)"
fi

feature_dir="$root/specs/$feature"
if [ ! -d "$feature_dir" ]; then
    echo "Error: feature directory not found: $feature_dir" >&2
    exit 1
fi

date="$(date +%Y-%m-%d)"

# Create research.md if missing
if [ ! -f "$feature_dir/research.md" ]; then
    cat > "$feature_dir/research.md" <<EOF
# Research: $feature

**Date**: $date

## Technology Investigation

[Research findings to be filled]
EOF
    echo "Created research.md"
fi

# Create data-model.md if missing
if [ ! -f "$feature_dir/data-model.md" ]; then
    cat > "$feature_dir/data-model.md" <<EOF
# Data Model: $feature

## Entities

[Entities to be defined based on spec]
EOF
    echo "Created data-model.md"
fi

# Create quickstart.md if missing
if [ ! -f "$feature_dir/quickstart.md" ]; then
    cat > "$feature_dir/quickstart.md" <<EOF
# Quickstart: $feature

## Key Validation Scenarios

[Validation scenarios to be defined]
EOF
    echo "Created quickstart.md"
fi

# Create contracts directory
mkdir -p "$feature_dir/contracts"
if [ ! -f "$feature_dir/contracts/api.md" ]; then
    cat > "$feature_dir/contracts/api.md" <<EOF
# API Contracts: $feature

[To be defined based on plan]
EOF
    echo "Created contracts/api.md"
fi

echo "Plan setup complete for $feature"
