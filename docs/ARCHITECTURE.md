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
                        │   cli/mod    │  15 subcommands
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
| `tests_cmd.rs` | `tests` | Generate test scaffolds from acceptance scenarios |
| `analyze.rs` | `analyze` | Run consistency analysis, print report |
| `checklist.rs` | `checklist` | Generate/append quality checklists |
| `pipeline.rs` | `pipeline` | Multi-agent pipeline orchestrator (7 phases) with agent CLI invocation |
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
| `test_generator.rs` | Extract Given/When/Then scenarios, detect framework, generate test scaffolds |
| `pipeline.rs` | Pipeline phase definitions, skip conditions, filtering, log generation |
| `analyzer.rs` | Cross-artifact consistency validation, severity heuristic |
| `feature.rs` | Feature numbering, branch name generation, 4-level resolution |
| `git.rs` | Git operations: init, branch creation, current branch detection |
| `errors.rs` | Typed error enum `RustySpecError` with what/where/fix |
| `token.rs` | GitHub token resolution (CLI flag > env vars) |
| `vscode.rs` | Deep-merge `.vscode/settings.json` |

### `src/agents/` — AI Agent Integration

Manages 20 AI coding agents with data-driven configuration and CLI invocation.

| File | Responsibility |
|------|---------------|
| `config.rs` | `AGENTS` const table — 20 agents with ID, dir, format, placeholder, CLI binary/flags |
| `registry.rs` | Detection, registration, unregistration of commands (phase-specific prompts) |
| `registrar.rs` | Re-exports from registry |
| `formats.rs` | Markdown/TOML/Vibe-skill rendering, placeholder translation, path adjustment |
| `invoker.rs` | Non-interactive CLI invocation of AI agents, phase-specific prompt generation |

**Agent-specific handling** (in `registry.rs`):
- **Copilot**: `.agent.md` + companion `.prompt.md`
- **Kimi**: Directory-based skills with dot-separator (`.kimi/skills/rustyspec.specify/SKILL.md`)
- **Vibe**: Directory-based skills with hyphen-separator (`.vibe/skills/rustyspec-specify/SKILL.md`), `user-invocable: true` frontmatter
- **Gemini/Tabnine**: TOML format with `{{args}}`

**CLI invocation** (in `invoker.rs`):
- Builds detailed, phase-specific prompts (not generic "execute the workflow")
- Invokes agent CLI via `std::process::Command` with non-interactive flags
- Returns `Success`/`NotAvailable`/`Failed` — pipeline falls back to handoff on failure
- Agent-specific invocation: `claude -p`, `vibe -p`, `codex exec`, `kimi --yolo`, etc.

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
| `mod.rs` | `RootConfig` (rustyspec.toml), `PipelineConfig` (per-phase agent mapping), `ProjectInternalConfig`, `InitOptions`, project root finder |

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

### Pipeline Execution Flow

```
rustyspec pipeline [--new "desc"] [--from X] [--to Y] [--auto] [--no-agent]
        │
        ▼
┌─ Resolve feature ──────────────────────────────┐
│  --new → next_feature_number() + branch name    │
│  else  → 4-level feature resolution             │
└─────────────────────┬───────────────────────────┘
                      │
        ▼─────────────┘
┌─ Check agent availability ─────────────────────┐
│  For each phase, check if agent CLI is in PATH  │
│  → AllCli (fully automated)                     │
│  → Mixed (some handoff)                         │
│  → Disabled (--no-agent, scaffold only)         │
└─────────────────────┬──────────────────────────┘
                      │
        ▼─────────────┘
┌─ For each phase in [specify→clarify→plan→tasks→tests→implement→analyze] ─┐
│                                                                           │
│  1. Resolve agent (per-phase config > default_agent)                      │
│  2. Check skip condition (artifact exists? force?)                        │
│  3. Generate scaffold:                                                    │
│     └── call cli::{phase}::run() → creates template files                │
│  4. Invoke AI agent (unless --no-agent):                                  │
│     ├── Auto phases → invoker::invoke_agent() with phase-specific prompt  │
│     │   ├── Success → agent fills templates with real content             │
│     │   ├── NotAvailable → fall back to handoff (show manual command)     │
│     │   └── Failed → fall back to handoff (log warning)                   │
│     └── Handoff phases (implement) → prompt user, wait for Enter          │
│  5. After specify with --new → re-detect feature dir                      │
│  6. Record PhaseResult (status, duration, output)                         │
│                                                                           │
└───────────────────────────────────────────────────────────────────────────┘
        │
        ▼
  Write specs/<feature>/pipeline-log.md
```

### Agent CLI Invocation Flow

```
Pipeline Auto phase (specify, plan, tasks, tests, analyze)
        │
        ▼
┌─ invoker::build_phase_prompt() ────────────────┐
│  Generate phase-specific instructions:          │
│  - specify: "Fill spec.md with user stories..." │
│  - plan: "Fill plan.md, research.md, ..."       │
│  - tasks: "Fill tasks.md with actionable..."    │
│  - tests: "Enhance test scaffolds..."           │
│  - analyze: "Validate consistency..."           │
└─────────────────────┬──────────────────────────┘
                      │
        ▼─────────────┘
┌─ invoker::invoke_agent() ──────────────────────┐
│  1. Look up AgentConfig for agent ID            │
│  2. Check cli_binary is non-empty               │
│  3. Check binary exists in PATH (which::which)  │
│  4. Build Command:                              │
│     ├── claude: claude -p "prompt" --allowedTools│
│     ├── vibe: vibe -p "prompt" --max-turns 25   │
│     ├── codex: codex exec "prompt"              │
│     ├── kimi: kimi --yolo "prompt"              │
│     └── others: {binary} {flag} "prompt"        │
│  5. Execute with current_dir = project_root     │
│  6. Return Success/NotAvailable/Failed          │
└─────────────────────────────────────────────────┘
```

### Spec-to-Test Generation Flow

```
rustyspec tests [feature-id] [--framework X] [--output-dir Y]
        │
        ▼
┌─ test_generator::extract_scenarios(spec_text) ─┐
│  Parse Given/When/Then blocks from spec.md      │
│  Group by user story index                      │
└─────────────────────┬──────────────────────────┘
                      │
        ▼─────────────┘
┌─ Detect framework ──────────────────────────────┐
│  --framework flag > Cargo.toml/package.json/etc  │
│  Fallback: generic                               │
└─────────────────────┬───────────────────────────┘
                      │
        ▼─────────────┘
┌─ Render test scaffolds ─────────────────────────┐
│  Jest (.test.js) │ pytest (.py) │ Cargo (.rs)    │
│  Go (_test.go)   │ generic (.test.txt)           │
│  One file per user story                         │
└──────────────────────────────────────────────────┘
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

### 7. Pipeline re-detection after specify

When `pipeline --new` runs, the specify phase creates its own feature directory with independent numbering. After specify completes, the pipeline re-resolves the feature directory to pick up the actual name, avoiding mismatches between pre-computed and actual directory names.

### 8. Given/When/Then extraction

Test generation parses acceptance scenarios from spec.md using regex-based extraction of Given/When/Then blocks. Scenarios are grouped by user story index (sorted) to produce one test file per story, ensuring deterministic output regardless of HashMap iteration order.

### 9. Phase-specific agent prompts

Each pipeline phase has a detailed, unique prompt (in `invoker.rs`) telling the AI agent exactly what to fill in. The `implement` command in `registry.rs` has enriched 7-step instructions. Non-implement commands have per-phase instructions (not generic "execute the workflow"). This ensures agents produce useful content when invoked programmatically via CLI.

### 10. Graceful CLI fallback

Agent CLI invocation returns a three-variant enum (`Success`/`NotAvailable`/`Failed`). When an agent's CLI is not installed or fails, the pipeline falls back to handoff mode (shows the manual `/rustyspec-*` command to run). This means the pipeline never crashes due to a missing agent — it degrades gracefully.

### 11. Vibe directory-based skills

Mistral Vibe uses a skills system with directory-based discovery (`.vibe/skills/<name>/SKILL.md`), unlike most agents that use flat command files. The `SKILL.md` requires `user-invocable: true` and `allowed-tools:` in YAML frontmatter. RustySpec generates these with the correct format so skills appear in Vibe's slash command list.

## File Counts

| Category | Files | Tests |
|----------|-------|-------|
| CLI commands | 17 | 19 |
| Core domain | 12 | 103 |
| Agents | 6 | 47 |
| Templates | 2 | 22 |
| Presets | 4 | 28 |
| Extensions | 5 | 40 |
| Config | 1 | 9 |
| main.rs | 1 | — |
| **Total** | **48** | **268** |

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
