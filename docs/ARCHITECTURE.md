# RustySpec Architecture

## Overview

RustySpec is a Rust CLI built with a layered architecture that separates concerns into distinct module trees. The CLI layer handles user interaction, the core layer contains domain logic, and specialized modules handle agent integration, templates, presets, and extensions.

```
                        ┌──────────────┐
                        │    main.rs   │
                        │  (entrypoint)│
                        └──────┬───────┘
                               │
                        ┌──────▼───────┐
                        │   cli/mod    │  13 subcommands
                        │   (clap)     │
                        └──┬───┬───┬───┘
                           │   │   │
              ┌────────────┘   │   └────────────┐
              ▼                ▼                 ▼
        ┌──────────┐    ┌──────────┐     ┌──────────────┐
        │  core/   │    │ agents/  │     │  templates/  │
        │ (domain) │    │ (AI)     │     │  (render)    │
        └──────────┘    └──────────┘     └──────────────┘
              │                                 │
         ┌────┴────┐                    ┌───────┴──────┐
         ▼         ▼                    ▼              ▼
   ┌──────────┐ ┌──────────┐    ┌──────────┐   ┌──────────┐
   │ presets/ │ │extensions│    │ config/  │   │ embedded │
   │          │ │  /hooks  │    │          │   │ templates│
   └──────────┘ └──────────┘    └──────────┘   └──────────┘
```

## Module Tree

### `src/main.rs`

Entrypoint. Parses CLI args, initializes logger, dispatches to `cli::run()`.

### `src/cli/` — Command Layer

Thin handlers for each CLI subcommand. Each file maps to one command. No business logic — delegates to `core/`, `agents/`, `templates/`, `presets/`, `extensions/`.

| File | Command | Responsibility |
|------|---------|---------------|
| `mod.rs` | — | Clap `Cli` struct, `Commands` enum, dispatch |
| `init.rs` | `init` | Bootstrap project, register agents, create git repo |
| `specify.rs` | `specify` | Create feature branch + spec from template |
| `clarify.rs` | `clarify` | Identify markers, generate questions |
| `plan.rs` | `plan` | Generate plan + supporting docs, constitution checks |
| `tasks.rs` | `tasks` | Generate phased task breakdown |
| `implement.rs` | `implement` | Parse tasks, fire hooks, list pending work |
| `analyze.rs` | `analyze` | Run consistency analysis, print report |
| `checklist.rs` | `checklist` | Generate/append quality checklists |
| `check.rs` | `check` | Verify prerequisites |
| `preset.rs` | `preset` | Preset CRUD subcommands |
| `extension.rs` | `extension` | Extension CRUD subcommands |
| `upgrade.rs` | `upgrade` | Refresh templates and agent commands |
| `completions.rs` | `completions` | Generate shell completions |
| `ux.rs` | — | Step tracker, status indicators (shared UI) |

### `src/core/` — Domain Logic

Pure business logic with no CLI dependency. Can be used as a library.

| File | Responsibility |
|------|---------------|
| `spec_parser.rs` | Parse `spec.md` into `ParsedSpec` (stories, requirements, markers, entities) |
| `constitution.rs` | Load constitution, parse gates, check plan compliance |
| `task_generator.rs` | Generate `TaskList` from spec + plan, organize by phases |
| `analyzer.rs` | Cross-artifact consistency validation, severity heuristic |
| `feature.rs` | Feature numbering, branch name generation, 4-level resolution |
| `git.rs` | Git operations: init, branch creation, current branch detection |
| `errors.rs` | Typed error enum `RustySpecError` with what/where/fix |
| `token.rs` | GitHub token resolution (CLI flag > env vars) |
| `vscode.rs` | Deep-merge `.vscode/settings.json` |

### `src/agents/` — AI Agent Integration

Manages 20 AI coding agents with data-driven configuration.

| File | Responsibility |
|------|---------------|
| `config.rs` | `AGENTS` const table — 20 agents with ID, dir, format, placeholder |
| `registry.rs` | Detection, registration, unregistration of commands |
| `registrar.rs` | Re-exports from registry |
| `formats.rs` | Markdown/TOML rendering, placeholder translation, path adjustment |

**Agent-specific handling** (in `registry.rs`):
- **Copilot**: `.agent.md` + companion `.prompt.md`
- **Kimi**: Directory-based skills with dot-separator
- **Gemini/Tabnine**: TOML format with `{{args}}`

### `src/templates/` — Template Engine

| File | Responsibility |
|------|---------------|
| `mod.rs` | Tera rendering (autoescape disabled), embedded template constants |
| `resolver.rs` | 4-layer resolution: overrides > presets > extensions > embedded |

**Resolution hierarchy:**
```
1. .rustyspec/templates/overrides/    (project tweaks)
2. .rustyspec/presets/<id>/templates/ (sorted by priority)
3. .rustyspec/extensions/<id>/templates/
4. Embedded in binary (include_str!)
```

### `src/presets/` — Preset System

| File | Responsibility |
|------|---------------|
| `manifest.rs` | Parse + validate `preset.yml` (schema, semver, ID regex, template types) |
| `registry.rs` | `PresetRegistry` — JSON persistence, priority sort, search |
| `manager.rs` | Add/remove/list/search/info + recursive directory copy |

### `src/extensions/` — Extension System

| File | Responsibility |
|------|---------------|
| `manifest.rs` | Parse + validate `extension.yml` (commands, hooks, dependencies) |
| `registry.rs` | `ExtensionRegistry` — enable/disable, deep-copy, name resolution |
| `manager.rs` | Install (--dev), remove, enable, disable, list, search |
| `hooks.rs` | Cross-platform hook executor (sh/PowerShell/cmd fallback) |

### `src/config/` — Configuration

| File | Responsibility |
|------|---------------|
| `mod.rs` | `RootConfig` (rustyspec.toml), `ProjectInternalConfig`, `InitOptions`, project root finder |

## Data Flow

### Specify → Plan → Tasks Pipeline

```
User description
      │
      ▼
┌─────────────┐    ┌──────────────┐    ┌───────────────┐
│   specify    │───>│    plan      │───>│    tasks      │
│             │    │              │    │               │
│ - branch    │    │ - research   │    │ - phases      │
│ - spec.md   │    │ - plan.md    │    │ - tasks.md    │
│ - checklist │    │ - data-model │    │ - after_tasks │
│             │    │ - contracts  │    │   hook        │
└─────────────┘    │ - quickstart │    └───────────────┘
                   │ - AGENT.md   │
                   └──────────────┘
```

### Template Resolution Flow

```
Command needs template
        │
        ▼
┌─ resolver::load_template() ──────────────────────┐
│                                                   │
│  1. Check overrides/  ──found──> return file      │
│  2. Check presets/ (by priority) ──found──> return │
│  3. Check extensions/ ──found──> return            │
│  4. Return embedded default                        │
│                                                   │
└───────────────────────────────────────────────────┘
        │
        ▼
  templates::render() (Tera, no HTML escaping)
        │
        ▼
  Write to specs/<feature>/
```

### Feature Resolution (4 levels)

```
resolve_feature(explicit_id, project_root)
        │
        ├── Level 1: explicit argument ──found──> return
        ├── Level 2: RUSTYSPEC_FEATURE env ──found──> return
        ├── Level 3: git branch (if \d{3}-.*) ──found──> return
        └── Level 4: latest specs/ directory ──found──> return
```

### Hook Execution Flow

```
Workflow command (tasks, implement)
        │
        ▼
fire_hooks(trigger, project_root, registry)
        │
        ├── For each enabled extension with matching trigger:
        │     │
        │     ├── Resolve hook file path
        │     ├── Check file exists
        │     └── Execute (platform-aware):
        │           ├── Windows .ps1 → powershell
        │           ├── Windows .sh  → sh, fallback cmd
        │           └── Unix         → sh
        │
        └── Failures logged as warnings (non-blocking)
```

## Key Design Decisions

### 1. Data-driven agent config

All 20 agents are defined in a single `AGENTS` const array. Adding a new agent requires only adding an `AgentConfig` entry — no new code files needed. Special behaviors (Copilot, Kimi, Cursor) are handled by ID checks in `registry.rs`.

### 2. Template auto-escaping disabled

Tera's HTML auto-escaping is explicitly turned off (`tera.autoescape_on(vec![])`) because RustySpec generates Markdown, not HTML. Without this, `&` becomes `&amp;` in all generated artifacts.

### 3. Constitution gate stripping

When checking plan compliance, the `## Constitution Check` section of the plan itself is stripped before analysis to prevent false positives from the gate checklist text.

### 4. Private registry fields

Both `PresetRegistry` and `ExtensionRegistry` keep their `entries` HashMap private. Access is via methods that return deep copies, preventing accidental mutation of internal state.

### 5. Platform-aware hooks

The hook executor detects Windows vs Unix at compile time (`cfg!(windows)`) and uses appropriate shell: PowerShell for `.ps1`, `sh` with `cmd` fallback for others on Windows, `sh` on Unix.

### 6. Branch-first specify

`specify` creates the git branch before writing any files, so artifacts land on the correct branch. If branch creation fails, files go to the current branch with a warning.

## File Counts

| Category | Files | Tests |
|----------|-------|-------|
| CLI commands | 14 | — |
| Core domain | 9 | 80+ |
| Agents | 4 | 33 |
| Templates | 2 | 20 |
| Presets | 3 | 27 |
| Extensions | 4 | 38 |
| Config | 1 | 11 |
| **Total** | **37** | **209** |

## Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` + `clap_complete` | CLI parsing + shell completions |
| `serde` + `toml` + `serde_yaml` + `serde_json` | Config serialization |
| `tera` | Template rendering |
| `git2` | Git operations (libgit2 bindings) |
| `regex` | Spec parsing, feature numbering |
| `semver` | Version validation |
| `thiserror` + `anyhow` | Error handling |
| `console` | Colored output |
| `which` | CLI tool detection |
| `chrono` | Timestamps |
| `log` + `env_logger` | Logging |
