#![allow(dead_code)]
/// Agent configuration table — data-driven, 21 agents.
#[derive(Debug, Clone)]
pub struct AgentConfig {
    pub id: &'static str,
    pub name: &'static str,
    pub command_dir: &'static str,
    pub commands_subdir: &'static str,
    pub extension: &'static str,
    pub format: AgentFormat,
    pub arg_placeholder: &'static str,
    pub requires_cli: bool,
    pub aliases: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentFormat {
    Markdown,
    Toml,
}

pub const AGENTS: &[AgentConfig] = &[
    AgentConfig {
        id: "claude",
        name: "Claude Code",
        command_dir: ".claude",
        commands_subdir: "commands",
        extension: ".md",
        format: AgentFormat::Markdown,
        arg_placeholder: "$ARGUMENTS",
        requires_cli: true,
        aliases: &[],
    },
    AgentConfig {
        id: "gemini",
        name: "Gemini CLI",
        command_dir: ".gemini",
        commands_subdir: "commands",
        extension: ".toml",
        format: AgentFormat::Toml,
        arg_placeholder: "{{args}}",
        requires_cli: true,
        aliases: &[],
    },
    AgentConfig {
        id: "copilot",
        name: "GitHub Copilot",
        command_dir: ".github",
        commands_subdir: "agents",
        extension: ".agent.md",
        format: AgentFormat::Markdown,
        arg_placeholder: "$ARGUMENTS",
        requires_cli: false,
        aliases: &[],
    },
    AgentConfig {
        id: "cursor",
        name: "Cursor",
        command_dir: ".cursor",
        commands_subdir: "commands",
        extension: ".md",
        format: AgentFormat::Markdown,
        arg_placeholder: "$ARGUMENTS",
        requires_cli: false,
        aliases: &[],
    },
    AgentConfig {
        id: "windsurf",
        name: "Windsurf",
        command_dir: ".windsurf",
        commands_subdir: "workflows",
        extension: ".md",
        format: AgentFormat::Markdown,
        arg_placeholder: "$ARGUMENTS",
        requires_cli: false,
        aliases: &[],
    },
    AgentConfig {
        id: "codex",
        name: "Codex CLI",
        command_dir: ".codex",
        commands_subdir: "prompts",
        extension: ".md",
        format: AgentFormat::Markdown,
        arg_placeholder: "$ARGUMENTS",
        requires_cli: true,
        aliases: &[],
    },
    AgentConfig {
        id: "qwen",
        name: "Qwen Code",
        command_dir: ".qwen",
        commands_subdir: "commands",
        extension: ".md",
        format: AgentFormat::Markdown,
        arg_placeholder: "$ARGUMENTS",
        requires_cli: true,
        aliases: &[],
    },
    AgentConfig {
        id: "opencode",
        name: "opencode",
        command_dir: ".opencode",
        commands_subdir: "command",
        extension: ".md",
        format: AgentFormat::Markdown,
        arg_placeholder: "$ARGUMENTS",
        requires_cli: true,
        aliases: &[],
    },
    AgentConfig {
        id: "kilocode",
        name: "Kilo Code",
        command_dir: ".kilocode",
        commands_subdir: "workflows",
        extension: ".md",
        format: AgentFormat::Markdown,
        arg_placeholder: "$ARGUMENTS",
        requires_cli: false,
        aliases: &[],
    },
    AgentConfig {
        id: "auggie",
        name: "Auggie CLI",
        command_dir: ".augment",
        commands_subdir: "commands",
        extension: ".md",
        format: AgentFormat::Markdown,
        arg_placeholder: "$ARGUMENTS",
        requires_cli: true,
        aliases: &[],
    },
    AgentConfig {
        id: "roo",
        name: "Roo Code",
        command_dir: ".roo",
        commands_subdir: "commands",
        extension: ".md",
        format: AgentFormat::Markdown,
        arg_placeholder: "$ARGUMENTS",
        requires_cli: false,
        aliases: &[],
    },
    AgentConfig {
        id: "codebuddy",
        name: "CodeBuddy",
        command_dir: ".codebuddy",
        commands_subdir: "commands",
        extension: ".md",
        format: AgentFormat::Markdown,
        arg_placeholder: "$ARGUMENTS",
        requires_cli: true,
        aliases: &[],
    },
    AgentConfig {
        id: "qodercli",
        name: "Qoder CLI",
        command_dir: ".qoder",
        commands_subdir: "commands",
        extension: ".md",
        format: AgentFormat::Markdown,
        arg_placeholder: "$ARGUMENTS",
        requires_cli: true,
        aliases: &["qoder"],
    },
    AgentConfig {
        id: "kiro-cli",
        name: "Kiro CLI",
        command_dir: ".kiro",
        commands_subdir: "prompts",
        extension: ".md",
        format: AgentFormat::Markdown,
        arg_placeholder: "$ARGUMENTS",
        requires_cli: true,
        aliases: &["kiro"],
    },
    AgentConfig {
        id: "amp",
        name: "Amp",
        command_dir: ".agents",
        commands_subdir: "commands",
        extension: ".md",
        format: AgentFormat::Markdown,
        arg_placeholder: "$ARGUMENTS",
        requires_cli: true,
        aliases: &[],
    },
    AgentConfig {
        id: "shai",
        name: "SHAI",
        command_dir: ".shai",
        commands_subdir: "commands",
        extension: ".md",
        format: AgentFormat::Markdown,
        arg_placeholder: "$ARGUMENTS",
        requires_cli: true,
        aliases: &[],
    },
    AgentConfig {
        id: "tabnine",
        name: "Tabnine CLI",
        command_dir: ".tabnine",
        commands_subdir: "agent/commands",
        extension: ".toml",
        format: AgentFormat::Toml,
        arg_placeholder: "{{args}}",
        requires_cli: true,
        aliases: &[],
    },
    AgentConfig {
        id: "kimi",
        name: "Kimi Code",
        command_dir: ".kimi",
        commands_subdir: "skills",
        extension: "/SKILL.md",
        format: AgentFormat::Markdown,
        arg_placeholder: "$ARGUMENTS",
        requires_cli: true,
        aliases: &[],
    },
    AgentConfig {
        id: "vibe",
        name: "Mistral Vibe",
        command_dir: ".vibe",
        commands_subdir: "prompts",
        extension: ".md",
        format: AgentFormat::Markdown,
        arg_placeholder: "$ARGUMENTS",
        requires_cli: false,
        aliases: &[],
    },
    AgentConfig {
        id: "bob",
        name: "IBM Bob",
        command_dir: ".bob",
        commands_subdir: "commands",
        extension: ".md",
        format: AgentFormat::Markdown,
        arg_placeholder: "$ARGUMENTS",
        requires_cli: false,
        aliases: &[],
    },
];

pub fn find_agent(id: &str) -> Option<&'static AgentConfig> {
    // Check direct ID match
    if let Some(agent) = AGENTS.iter().find(|a| a.id == id) {
        return Some(agent);
    }
    // Check aliases
    AGENTS.iter().find(|a| a.aliases.contains(&id))
}

pub fn all_agent_ids() -> Vec<&'static str> {
    AGENTS.iter().map(|a| a.id).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn config_table_has_20_agents() {
        assert_eq!(AGENTS.len(), 20); // Generic agent handled via --ai-commands-dir flag, not in table
    }

    #[test]
    fn no_duplicate_ids() {
        let ids: HashSet<_> = AGENTS.iter().map(|a| a.id).collect();
        assert_eq!(ids.len(), AGENTS.len(), "Duplicate agent IDs found");
    }

    #[test]
    fn no_duplicate_aliases() {
        let mut all_aliases = Vec::new();
        for agent in AGENTS {
            for alias in agent.aliases {
                assert!(!all_aliases.contains(alias), "Duplicate alias: {alias}");
                all_aliases.push(*alias);
            }
        }
    }

    #[test]
    fn all_agents_have_nonempty_command_dir_and_format() {
        for agent in AGENTS {
            assert!(
                !agent.command_dir.is_empty(),
                "{} has empty command_dir",
                agent.id
            );
            assert!(
                !agent.commands_subdir.is_empty(),
                "{} has empty commands_subdir",
                agent.id
            );
            assert!(
                !agent.extension.is_empty(),
                "{} has empty extension",
                agent.id
            );
        }
    }

    #[test]
    fn cli_agents_have_requires_cli_true() {
        let cli_agents = [
            "claude",
            "gemini",
            "codex",
            "qwen",
            "opencode",
            "auggie",
            "codebuddy",
            "qodercli",
            "kiro-cli",
            "amp",
            "shai",
            "tabnine",
            "kimi",
        ];
        for id in &cli_agents {
            let agent = find_agent(id).unwrap_or_else(|| panic!("Agent {id} not found"));
            assert!(agent.requires_cli, "{id} should have requires_cli=true");
        }
    }

    #[test]
    fn ide_agents_have_requires_cli_false() {
        let ide_agents = [
            "cursor", "windsurf", "kilocode", "roo", "copilot", "bob", "vibe",
        ];
        for id in &ide_agents {
            let agent = find_agent(id).unwrap_or_else(|| panic!("Agent {id} not found"));
            assert!(!agent.requires_cli, "{id} should have requires_cli=false");
        }
    }

    #[test]
    fn alias_resolution_works() {
        assert!(find_agent("kiro").is_some());
        assert_eq!(find_agent("kiro").unwrap().id, "kiro-cli");
        assert!(find_agent("qoder").is_some());
        assert_eq!(find_agent("qoder").unwrap().id, "qodercli");
    }

    #[test]
    fn toml_agents_use_args_placeholder() {
        for agent in AGENTS {
            if agent.format == AgentFormat::Toml {
                assert_eq!(
                    agent.arg_placeholder, "{{args}}",
                    "{} should use {{{{args}}}}",
                    agent.id
                );
            }
        }
    }

    #[test]
    fn copilot_uses_agent_md_extension() {
        let copilot = find_agent("copilot").unwrap();
        assert_eq!(copilot.extension, ".agent.md");
    }

    #[test]
    fn kimi_uses_skill_md_extension() {
        let kimi = find_agent("kimi").unwrap();
        assert_eq!(kimi.extension, "/SKILL.md");
    }

    #[test]
    fn alias_bijection() {
        // Each alias must map to exactly one agent
        let mut alias_set = HashSet::new();
        for agent in AGENTS {
            for alias in agent.aliases {
                assert!(
                    alias_set.insert(*alias),
                    "Alias '{alias}' used by multiple agents"
                );
                // Alias must not conflict with any agent ID
                assert!(
                    AGENTS.iter().all(|a| a.id != *alias),
                    "Alias '{alias}' conflicts with agent ID"
                );
            }
        }
    }
}
