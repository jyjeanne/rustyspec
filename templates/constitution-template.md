# Project Constitution

**Project**: {{ project_name }}
**Established**: {{ date }}

## Core Principles

### Article I: Library-First
Every feature MUST begin as a standalone component with clear boundaries and minimal dependencies.

### Article II: CLI Interface
All functionality MUST be accessible via text input/output (stdin/stdout/JSON).

### Article III: Test-First
No implementation code shall be written before tests are written, validated, and confirmed to fail.

### Article VII: Simplicity
- Maximum 3 projects for initial implementation
- Additional projects require documented justification
- No speculative or "might need" features

### Article VIII: Anti-Abstraction
- Use framework features directly rather than wrapping them
- Single model representation (no duplicate DTOs)

### Article IX: Integration-First
- Prefer real databases/services over mocks
- Contract tests mandatory before implementation

## Governance

### Amendment Process
Modifications to this constitution require:
- Explicit documentation of the rationale for change
- Review and approval by project maintainers
- Backwards compatibility assessment
