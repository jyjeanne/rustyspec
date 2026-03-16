#!/usr/bin/env bash
# Regenerate .rustyspec/AGENT.md from constitution and current specs
source "$(dirname "$0")/common.sh"

root="$(get_repo_root)"
agent_file="$root/.rustyspec/AGENT.md"
constitution="$root/.rustyspec/constitution.md"
date="$(date +%Y-%m-%d)"

# Read project name from rustyspec.toml
project_name="$(sed -n 's/^name\s*=\s*"\([^"]*\)".*/\1/p' "$root/rustyspec.toml" 2>/dev/null | head -1)"
project_name="${project_name:-unknown}"

cat > "$agent_file" <<EOF
# RustySpec Agent Context

**Project**: $project_name
**Updated**: $date

## Constitution Principles

EOF

# Append constitution summary if exists
if [ -f "$constitution" ]; then
    # Extract article headers
    grep -E "^### Article" "$constitution" >> "$agent_file" 2>/dev/null || true
    echo "" >> "$agent_file"
fi

cat >> "$agent_file" <<EOF
## Current Features

EOF

# List features
for dir in "$root"/specs/[0-9][0-9][0-9]-*/; do
    [ -d "$dir" ] || continue
    feature="$(basename "$dir")"
    status="draft"
    if [ -f "$dir/tasks.md" ]; then
        done_count=$(grep -c -i '\- \[x\] T' "$dir/tasks.md" 2>/dev/null || echo 0)
        total_count=$(grep -c '\- \[.\] T' "$dir/tasks.md" 2>/dev/null || echo 0)
        if [ "$total_count" -gt 0 ] && [ "$done_count" -eq "$total_count" ]; then
            status="complete"
        elif [ "$done_count" -gt 0 ]; then
            status="in-progress ($done_count/$total_count)"
        else
            status="planned"
        fi
    fi
    echo "- **$feature**: $status" >> "$agent_file"
done

cat >> "$agent_file" <<EOF

## Available Commands

- \`/rustyspec-specify\` — Create feature specification
- \`/rustyspec-clarify\` — Resolve spec ambiguities
- \`/rustyspec-plan\` — Generate architecture plan
- \`/rustyspec-tasks\` — Generate task breakdown
- \`/rustyspec-implement\` — Execute tasks
- \`/rustyspec-analyze\` — Validate consistency
- \`/rustyspec-checklist\` — Quality validation
EOF

echo "Updated $agent_file"
