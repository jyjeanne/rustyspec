#![allow(dead_code)]
use std::path::Path;

use anyhow::Result;

use super::config::{AGENTS, AgentConfig, find_agent};
use super::formats;

/// Detected agent in a repository.
#[derive(Debug, Clone)]
pub struct DetectedAgent {
    pub config: &'static AgentConfig,
    pub dir_exists: bool,
    pub cli_available: bool,
}

/// RustySpec commands to register with agents.
const COMMANDS: &[(&str, &str)] = &[
    ("specify", "Create a new feature specification"),
    ("clarify", "Resolve ambiguities in a specification"),
    ("plan", "Generate an architecture plan from a specification"),
    (
        "tasks",
        "Generate a story-driven task breakdown from the plan",
    ),
    ("implement", "Execute tasks from the task breakdown"),
    ("analyze", "Validate cross-artifact consistency"),
    ("checklist", "Generate a quality validation checklist"),
];

/// Detect all agents present in the repository.
pub fn detect_agents(project_root: &Path) -> Vec<DetectedAgent> {
    AGENTS
        .iter()
        .map(|agent| {
            let agent_path = project_root.join(agent.command_dir);
            let dir_exists = agent_path.exists();
            let cli_available = if agent.requires_cli {
                check_cli_available(agent.id)
            } else {
                true // IDE agents don't need CLI
            };
            DetectedAgent {
                config: agent,
                dir_exists,
                cli_available,
            }
        })
        .collect()
}

/// Register RustySpec commands for a specific agent.
pub fn register_commands(project_root: &Path, agent: &AgentConfig) -> Result<()> {
    let cmd_dir = project_root
        .join(agent.command_dir)
        .join(agent.commands_subdir);
    std::fs::create_dir_all(&cmd_dir)?;

    for (cmd_name, description) in COMMANDS {
        let body = format!(
            "Read the project context from .rustyspec/AGENT.md, then execute the '{}' workflow for the feature specified by {}.",
            cmd_name, agent.arg_placeholder
        );

        let body = formats::translate_placeholder(&body, agent.arg_placeholder);
        let content = formats::render_command(agent.format, description, &body);
        let content = formats::adjust_script_paths(&content);

        write_command_file(project_root, agent, cmd_name, &content)?;
    }

    Ok(())
}

/// Write a single command file, handling agent-specific paths.
fn write_command_file(
    project_root: &Path,
    agent: &AgentConfig,
    cmd_name: &str,
    content: &str,
) -> Result<()> {
    let cmd_dir = project_root
        .join(agent.command_dir)
        .join(agent.commands_subdir);

    if agent.id == "kimi" {
        // Kimi: directory-based skills with dot-separator
        let skill_name = formats::kimi_command_name(cmd_name);
        let skill_dir = cmd_dir.join(&skill_name);
        std::fs::create_dir_all(&skill_dir)?;
        std::fs::write(skill_dir.join("SKILL.md"), content)?;
    } else if agent.id == "copilot" {
        // Copilot: .agent.md + companion .prompt.md
        let file_name = format!("rustyspec-{cmd_name}{}", agent.extension);
        std::fs::write(cmd_dir.join(&file_name), content)?;

        // Companion .prompt.md in .github/prompts/
        let prompts_dir = project_root.join(".github/prompts");
        std::fs::create_dir_all(&prompts_dir)?;
        let prompt_name = format!("rustyspec-{cmd_name}.prompt.md");
        std::fs::write(prompts_dir.join(&prompt_name), content)?;
    } else {
        // Standard: flat file with hyphen-separator
        let file_name = format!(
            "{}{}",
            formats::standard_command_name(cmd_name),
            agent.extension
        );
        std::fs::write(cmd_dir.join(&file_name), content)?;
    }

    Ok(())
}

/// Unregister all RustySpec commands for a specific agent.
pub fn unregister_commands(project_root: &Path, agent: &AgentConfig) -> Result<()> {
    let cmd_dir = project_root
        .join(agent.command_dir)
        .join(agent.commands_subdir);

    if !cmd_dir.exists() {
        return Ok(());
    }

    for (cmd_name, _) in COMMANDS {
        if agent.id == "kimi" {
            let skill_name = formats::kimi_command_name(cmd_name);
            let skill_dir = cmd_dir.join(&skill_name);
            if skill_dir.exists() {
                std::fs::remove_dir_all(&skill_dir)?;
            }
        } else if agent.id == "copilot" {
            let file_name = format!("rustyspec-{cmd_name}{}", agent.extension);
            let path = cmd_dir.join(&file_name);
            if path.exists() {
                std::fs::remove_file(&path)?;
            }

            // Remove companion .prompt.md
            let prompt = project_root
                .join(".github/prompts")
                .join(format!("rustyspec-{cmd_name}.prompt.md"));
            if prompt.exists() {
                std::fs::remove_file(&prompt)?;
            }
        } else {
            let file_name = format!(
                "{}{}",
                formats::standard_command_name(cmd_name),
                agent.extension
            );
            let path = cmd_dir.join(&file_name);
            if path.exists() {
                std::fs::remove_file(&path)?;
            }
        }
    }

    Ok(())
}

/// Register commands for all detected agents.
pub fn register_all(project_root: &Path, target_agent: Option<&str>) -> Result<Vec<String>> {
    let mut registered = Vec::new();

    if let Some(agent_id) = target_agent {
        // Register for a specific agent
        let agent = find_agent(agent_id).ok_or_else(|| {
            anyhow::anyhow!(
                "Unknown agent '{}'. Available: {}",
                agent_id,
                AGENTS.iter().map(|a| a.id).collect::<Vec<_>>().join(", ")
            )
        })?;

        let cmd_dir = project_root.join(agent.command_dir);
        std::fs::create_dir_all(cmd_dir.join(agent.commands_subdir))?;
        register_commands(project_root, agent)?;
        registered.push(agent.id.to_string());
    } else {
        // Auto-detect and register for all present agents
        let detected = detect_agents(project_root);
        for det in &detected {
            if det.dir_exists {
                register_commands(project_root, det.config)?;
                registered.push(det.config.id.to_string());
            }
        }
    }

    Ok(registered)
}

fn check_cli_available(agent_id: &str) -> bool {
    let exe_name = match agent_id {
        "kiro-cli" => "kiro",
        "qodercli" => "qodercli",
        _ => agent_id,
    };

    which::which(exe_name).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn detect_claude_when_dir_exists() {
        let dir = TempDir::new().unwrap();
        std::fs::create_dir_all(dir.path().join(".claude")).unwrap();
        let detected = detect_agents(dir.path());
        let claude = detected.iter().find(|d| d.config.id == "claude").unwrap();
        assert!(claude.dir_exists);
    }

    #[test]
    fn detect_multiple_agents() {
        let dir = TempDir::new().unwrap();
        std::fs::create_dir_all(dir.path().join(".claude")).unwrap();
        std::fs::create_dir_all(dir.path().join(".cursor")).unwrap();
        let detected = detect_agents(dir.path());
        let present: Vec<_> = detected
            .iter()
            .filter(|d| d.dir_exists)
            .map(|d| d.config.id)
            .collect();
        assert!(present.contains(&"claude"));
        assert!(present.contains(&"cursor"));
    }

    #[test]
    fn empty_repo_detects_nothing() {
        let dir = TempDir::new().unwrap();
        let detected = detect_agents(dir.path());
        assert!(detected.iter().all(|d| !d.dir_exists));
    }

    #[test]
    fn register_markdown_agent_creates_md_files() {
        let dir = TempDir::new().unwrap();
        let claude = find_agent("claude").unwrap();
        register_commands(dir.path(), claude).unwrap();

        let cmd_dir = dir.path().join(".claude/commands");
        assert!(cmd_dir.exists());

        let specify = cmd_dir.join("rustyspec-specify.md");
        assert!(specify.exists());
        let content = std::fs::read_to_string(&specify).unwrap();
        assert!(content.starts_with("---\n"));
        assert!(content.contains("description:"));
        assert!(content.contains("$ARGUMENTS"));
    }

    #[test]
    fn register_toml_agent_creates_toml_files() {
        let dir = TempDir::new().unwrap();
        let gemini = find_agent("gemini").unwrap();
        register_commands(dir.path(), gemini).unwrap();

        let cmd_dir = dir.path().join(".gemini/commands");
        let specify = cmd_dir.join("rustyspec-specify.toml");
        assert!(specify.exists());
        let content = std::fs::read_to_string(&specify).unwrap();
        assert!(content.contains("description = "));
        assert!(content.contains("prompt = \"\"\""));
        assert!(content.contains("{{args}}"));
        assert!(!content.contains("$ARGUMENTS"));
    }

    #[test]
    fn copilot_creates_agent_md_and_prompt_md() {
        let dir = TempDir::new().unwrap();
        let copilot = find_agent("copilot").unwrap();
        register_commands(dir.path(), copilot).unwrap();

        // .agent.md in .github/agents/
        let agent_file = dir.path().join(".github/agents/rustyspec-specify.agent.md");
        assert!(agent_file.exists());

        // .prompt.md in .github/prompts/
        let prompt_file = dir
            .path()
            .join(".github/prompts/rustyspec-specify.prompt.md");
        assert!(prompt_file.exists());
    }

    #[test]
    fn kimi_creates_directory_based_skills() {
        let dir = TempDir::new().unwrap();
        let kimi = find_agent("kimi").unwrap();
        register_commands(dir.path(), kimi).unwrap();

        // Directory-based: .kimi/skills/rustyspec.specify/SKILL.md
        let skill = dir.path().join(".kimi/skills/rustyspec.specify/SKILL.md");
        assert!(
            skill.exists(),
            "Kimi skill not found at {}",
            skill.display()
        );
    }

    #[test]
    fn unregister_removes_copilot_files() {
        let dir = TempDir::new().unwrap();
        let copilot = find_agent("copilot").unwrap();
        register_commands(dir.path(), copilot).unwrap();

        let agent_file = dir.path().join(".github/agents/rustyspec-specify.agent.md");
        let prompt_file = dir
            .path()
            .join(".github/prompts/rustyspec-specify.prompt.md");
        assert!(agent_file.exists());
        assert!(prompt_file.exists());

        unregister_commands(dir.path(), copilot).unwrap();
        assert!(!agent_file.exists());
        assert!(!prompt_file.exists());
    }

    #[test]
    fn unregister_removes_kimi_dirs() {
        let dir = TempDir::new().unwrap();
        let kimi = find_agent("kimi").unwrap();
        register_commands(dir.path(), kimi).unwrap();
        unregister_commands(dir.path(), kimi).unwrap();

        let skill = dir.path().join(".kimi/skills/rustyspec.specify");
        assert!(!skill.exists());
    }

    #[test]
    fn register_all_with_specific_agent() {
        let dir = TempDir::new().unwrap();
        let registered = register_all(dir.path(), Some("claude")).unwrap();
        assert_eq!(registered, vec!["claude"]);
        assert!(
            dir.path()
                .join(".claude/commands/rustyspec-specify.md")
                .exists()
        );
    }

    #[test]
    fn register_all_with_invalid_agent_returns_error() {
        let dir = TempDir::new().unwrap();
        let result = register_all(dir.path(), Some("nonexistent"));
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Unknown agent"));
        assert!(err.contains("Available:"));
    }

    #[test]
    fn register_all_auto_detect() {
        let dir = TempDir::new().unwrap();
        std::fs::create_dir_all(dir.path().join(".claude")).unwrap();
        std::fs::create_dir_all(dir.path().join(".cursor")).unwrap();

        let registered = register_all(dir.path(), None).unwrap();
        assert!(registered.contains(&"claude".to_string()));
        assert!(registered.contains(&"cursor".to_string()));
    }

    #[test]
    fn kimi_uses_dot_separator_others_use_hyphen() {
        let dir = TempDir::new().unwrap();

        // Kimi: dot separator
        let kimi = find_agent("kimi").unwrap();
        register_commands(dir.path(), kimi).unwrap();
        assert!(
            dir.path()
                .join(".kimi/skills/rustyspec.specify/SKILL.md")
                .exists()
        );

        // Claude: hyphen separator
        let claude = find_agent("claude").unwrap();
        register_commands(dir.path(), claude).unwrap();
        assert!(
            dir.path()
                .join(".claude/commands/rustyspec-specify.md")
                .exists()
        );
    }
}
