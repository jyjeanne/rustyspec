pub mod resolver;

use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;
use tera::{Context, Tera};

use crate::core::errors::RustySpecError;

/// Embedded default templates
pub mod embedded {
    pub const SPEC_TEMPLATE: &str = include_str!("../../templates/spec-template.md");
    pub const PLAN_TEMPLATE: &str = include_str!("../../templates/plan-template.md");
    pub const TASKS_TEMPLATE: &str = include_str!("../../templates/tasks-template.md");
    pub const CHECKLIST_TEMPLATE: &str = include_str!("../../templates/checklist-template.md");
    pub const CONSTITUTION_TEMPLATE: &str =
        include_str!("../../templates/constitution-template.md");
    pub const AGENT_FILE_TEMPLATE: &str = include_str!("../../templates/agent-file-template.md");

    pub fn all() -> Vec<(&'static str, &'static str)> {
        vec![
            ("spec-template.md", SPEC_TEMPLATE),
            ("plan-template.md", PLAN_TEMPLATE),
            ("tasks-template.md", TASKS_TEMPLATE),
            ("checklist-template.md", CHECKLIST_TEMPLATE),
            ("constitution-template.md", CONSTITUTION_TEMPLATE),
            ("agent-file-template.md", AGENT_FILE_TEMPLATE),
        ]
    }
}

/// Render a template string with the given variables.
pub fn render(template_str: &str, vars: &HashMap<String, String>) -> Result<String> {
    let mut tera = Tera::default();
    tera.autoescape_on(vec![]); // Disable HTML auto-escaping — we generate markdown, not HTML
    tera.add_raw_template("template", template_str)
        .map_err(|e| RustySpecError::Template {
            template: "inline".into(),
            message: format!("Failed to parse template: {e}"),
            fix: "Check template syntax (Tera/Jinja2 format).".into(),
        })?;

    let mut context = Context::new();
    for (key, value) in vars {
        context.insert(key.as_str(), value);
    }

    tera.render("template", &context)
        .map_err(|e| RustySpecError::Template {
            template: "inline".into(),
            message: format!("Failed to render template: {e}"),
            fix: "Ensure all required variables are provided.".into(),
        })
        .map_err(Into::into)
}

/// Copy all embedded templates to a target directory.
pub fn copy_embedded_templates(target_dir: &Path) -> Result<()> {
    std::fs::create_dir_all(target_dir)?;
    for (name, content) in embedded::all() {
        let path = target_dir.join(name);
        if !path.exists() {
            std::fs::write(&path, content)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_with_all_variables() {
        let template = "# {{ feature_name }}\nID: {{ feature_id }}\nBranch: {{ branch_name }}\nDate: {{ date }}\nProject: {{ project_name }}";
        let mut vars = HashMap::new();
        vars.insert("feature_name".into(), "Auth System".into());
        vars.insert("feature_id".into(), "001".into());
        vars.insert("branch_name".into(), "001-auth-system".into());
        vars.insert("date".into(), "2026-03-14".into());
        vars.insert("project_name".into(), "myapp".into());

        let result = render(template, &vars).unwrap();
        assert!(result.contains("Auth System"));
        assert!(result.contains("001"));
        assert!(result.contains("001-auth-system"));
        assert!(result.contains("2026-03-14"));
        assert!(result.contains("myapp"));
    }

    #[test]
    fn render_missing_variable_returns_error() {
        let template = "Hello {{ name }}";
        let vars = HashMap::new(); // no variables
        assert!(render(template, &vars).is_err());
    }

    #[test]
    fn render_empty_arguments_handled() {
        let template = "Args: {{ arguments }}";
        let mut vars = HashMap::new();
        vars.insert("arguments".into(), "".into());
        let result = render(template, &vars).unwrap();
        assert_eq!(result, "Args: ");
    }

    #[test]
    fn render_preserves_special_characters_in_markdown() {
        let template = "Name: {{ feature_name }}";
        let mut vars = HashMap::new();
        vars.insert("feature_name".into(), "auth & payments <v2>".into());
        let result = render(template, &vars).unwrap();
        // Markdown output must NOT be HTML-escaped
        assert!(result.contains("auth & payments <v2>"), "Got: {result}");
        assert!(
            !result.contains("&amp;"),
            "HTML escaping detected — markdown corrupted: {result}"
        );
    }

    #[test]
    fn all_embedded_templates_are_nonempty() {
        for (name, content) in embedded::all() {
            assert!(!content.is_empty(), "Template {name} is empty");
        }
    }

    #[test]
    fn embedded_templates_contain_expected_markers() {
        assert!(embedded::SPEC_TEMPLATE.contains("Feature Specification"));
        assert!(embedded::PLAN_TEMPLATE.contains("Implementation Plan"));
        assert!(embedded::TASKS_TEMPLATE.contains("Tasks"));
        assert!(embedded::CONSTITUTION_TEMPLATE.contains("Constitution"));
    }

    #[test]
    fn copy_embedded_templates_creates_files() {
        let dir = tempfile::TempDir::new().unwrap();
        let target = dir.path().join("templates");
        copy_embedded_templates(&target).unwrap();

        for (name, _) in embedded::all() {
            assert!(target.join(name).exists(), "Missing template: {name}");
        }
    }

    #[test]
    fn copy_embedded_templates_preserves_existing() {
        let dir = tempfile::TempDir::new().unwrap();
        let target = dir.path().join("templates");
        std::fs::create_dir_all(&target).unwrap();

        // Write a custom spec template
        let custom = "CUSTOM CONTENT";
        std::fs::write(target.join("spec-template.md"), custom).unwrap();

        // Copy should NOT overwrite
        copy_embedded_templates(&target).unwrap();

        let content = std::fs::read_to_string(target.join("spec-template.md")).unwrap();
        assert_eq!(content, custom);
    }
}
