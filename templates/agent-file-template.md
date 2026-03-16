# RustySpec Agent Context

**Project**: {{ project_name }}
**Updated**: {{ date }}

## Constitution Principles

This project follows Specification-Driven Development (SDD). Key governance:

- **Library-First**: Features begin as standalone components
- **Test-First**: Tests before implementation
- **Simplicity**: Max 3 projects, no future-proofing
- **Anti-Abstraction**: Use frameworks directly
- **Integration-First**: Real services over mocks

## Available Commands

- `rustyspec specify <name>` — Create feature specification
- `rustyspec clarify <id>` — Resolve spec ambiguities
- `rustyspec plan <id>` — Generate architecture plan
- `rustyspec tasks <id>` — Generate task breakdown
- `rustyspec implement <id>` — Execute tasks
- `rustyspec analyze <id>` — Validate consistency
- `rustyspec checklist <id>` — Quality validation
