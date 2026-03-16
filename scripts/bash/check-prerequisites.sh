#!/usr/bin/env bash
# RustySpec prerequisites check
source "$(dirname "$0")/common.sh"

echo "RustySpec Prerequisites Check"
echo "=============================="

errors=0

# Check git
if command -v git &>/dev/null; then
    echo "[OK] git: $(git --version)"
else
    echo "[!!] git: not found"
    errors=$((errors + 1))
fi

# Check project structure
root="$(get_repo_root 2>/dev/null || true)"
if [ -n "$root" ]; then
    echo "[OK] Project root: $root"

    if [ -f "$root/.rustyspec/constitution.md" ]; then
        echo "[OK] Constitution file present"
    else
        echo "[!!] Constitution file missing"
        errors=$((errors + 1))
    fi

    if [ -f "$root/rustyspec.toml" ]; then
        echo "[OK] rustyspec.toml found"
    else
        echo "[!!] rustyspec.toml missing"
        errors=$((errors + 1))
    fi
else
    echo "[!!] Not inside a RustySpec project"
    errors=$((errors + 1))
fi

echo ""
if [ "$errors" -eq 0 ]; then
    echo "All checks passed."
else
    echo "$errors issue(s) found."
    exit 1
fi
