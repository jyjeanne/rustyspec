use super::spec_parser::ParsedSpec;

/// Generated task list
#[derive(Debug, Clone)]
pub struct TaskList {
    pub phases: Vec<Phase>,
}

#[derive(Debug, Clone)]
pub struct Phase {
    pub name: String,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: String,            // T001, T002...
    pub parallel: bool,        // [P] marker
    pub story: Option<String>, // [US1], [US2]... only in story phases
    pub description: String,
}

impl Task {
    pub fn format(&self) -> String {
        let p = if self.parallel { " [P]" } else { "" };
        let s = self
            .story
            .as_ref()
            .map(|s| format!(" [{s}]"))
            .unwrap_or_default();
        format!("- [ ] {}{}{} {}", self.id, p, s, self.description)
    }
}

/// Generate tasks from a parsed spec and plan content.
pub fn generate_tasks(spec: &ParsedSpec, plan_content: &str, has_data_model: bool) -> TaskList {
    let mut phases = Vec::new();
    let mut task_num: u32 = 1;

    // Phase 1: Setup
    let mut setup_tasks = vec![
        make_task(
            &mut task_num,
            false,
            None,
            "Create project structure per implementation plan",
        ),
        make_task(
            &mut task_num,
            false,
            None,
            "Initialize project with dependencies",
        ),
    ];
    if plan_content.contains("linting") || plan_content.contains("formatting") {
        setup_tasks.push(make_task(
            &mut task_num,
            true,
            None,
            "Configure linting and formatting tools",
        ));
    }
    phases.push(Phase {
        name: "Phase 1: Setup".into(),
        tasks: setup_tasks,
    });

    // Phase 2: Foundational
    let mut foundational_tasks = Vec::new();
    if has_data_model {
        foundational_tasks.push(make_task(
            &mut task_num,
            false,
            None,
            "Setup data models and schemas",
        ));
    }
    // Entity model tasks go in Foundational (created once, not per story)
    for entity in &spec.entities {
        foundational_tasks.push(make_task(
            &mut task_num,
            true,
            None,
            &format!(
                "Create {entity} model in src/models/{}",
                entity.to_lowercase()
            ),
        ));
    }
    if plan_content.contains("auth") || plan_content.contains("authentication") {
        foundational_tasks.push(make_task(
            &mut task_num,
            true,
            None,
            "Implement authentication framework",
        ));
    }
    foundational_tasks.push(make_task(
        &mut task_num,
        false,
        None,
        "Configure error handling and logging infrastructure",
    ));
    phases.push(Phase {
        name: "Phase 2: Foundational".into(),
        tasks: foundational_tasks,
    });

    // Story phases — use sequential story index (US1, US2, US3...) not priority
    for (story_idx, story) in spec.user_stories.iter().enumerate() {
        let story_label = format!("US{}", story_idx + 1);
        let phase_name = format!(
            "Phase {}: User Story {} - {} (Priority: {})",
            phases.len() + 1,
            story_idx + 1,
            story.title,
            story.priority
        );

        let mut story_tasks = Vec::new();
        story_tasks.push(make_task(
            &mut task_num,
            false,
            Some(&story_label),
            &format!("Implement {} core functionality", story.title),
        ));
        story_tasks.push(make_task(
            &mut task_num,
            true,
            Some(&story_label),
            &format!("Add validation and error handling for {}", story.title),
        ));

        phases.push(Phase {
            name: phase_name,
            tasks: story_tasks,
        });
    }

    // Polish phase
    let polish_tasks = vec![
        make_task(&mut task_num, true, None, "Documentation updates in docs/"),
        make_task(&mut task_num, false, None, "Code cleanup and refactoring"),
        make_task(&mut task_num, false, None, "Run quality validation"),
    ];
    phases.push(Phase {
        name: "Phase N: Polish & Cross-Cutting Concerns".into(),
        tasks: polish_tasks,
    });

    TaskList { phases }
}

/// Format the full task list as Markdown.
pub fn format_task_list(tasks: &TaskList, feature_name: &str, branch_name: &str) -> String {
    let mut output = format!("# Tasks: {feature_name}\n\n");
    output.push_str(&format!(
        "**Input**: Design documents from `specs/{branch_name}/`\n"
    ));
    output
        .push_str("**Prerequisites**: plan.md (required), spec.md (required for user stories)\n\n");

    for (i, phase) in tasks.phases.iter().enumerate() {
        output.push_str(&format!("## {}\n\n", phase.name));

        for task in &phase.tasks {
            output.push_str(&format!("{}\n", task.format()));
        }

        if i < tasks.phases.len() - 1 {
            output.push_str(&format!(
                "\n**Checkpoint**: {} complete\n\n---\n\n",
                phase.name
            ));
        }
    }

    // Dependencies section
    output.push_str("\n---\n\n## Dependencies & Execution Order\n\n");
    output.push_str("- **Setup**: No dependencies\n");
    output.push_str("- **Foundational**: Depends on Setup — BLOCKS all user stories\n");
    output.push_str("- **User Stories**: Depend on Foundational; can proceed in parallel\n");
    output.push_str("- **Polish**: Depends on all user stories\n");

    output
}

fn make_task(counter: &mut u32, parallel: bool, story: Option<&str>, desc: &str) -> Task {
    let task = Task {
        id: format!("T{:03}", *counter),
        parallel,
        story: story.map(String::from),
        description: desc.to_string(),
    };
    *counter += 1;
    task
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::spec_parser::{Requirement, UserStory};

    fn sample_spec() -> ParsedSpec {
        ParsedSpec {
            user_stories: vec![
                UserStory {
                    title: "Real-time messaging".into(),
                    priority: "P1".into(),
                    acceptance_scenarios: vec!["Given user sends msg".into()],
                },
                UserStory {
                    title: "Message history".into(),
                    priority: "P2".into(),
                    acceptance_scenarios: vec![],
                },
            ],
            requirements: vec![Requirement {
                id: "FR-001".into(),
                text: "deliver messages".into(),
            }],
            clarification_markers: vec![],
            entities: vec!["Message".into(), "User".into()],
            raw: String::new(),
        }
    }

    #[test]
    fn tasks_have_strict_format() {
        let spec = sample_spec();
        let tasks = generate_tasks(&spec, "simple plan", true);
        for phase in &tasks.phases {
            for task in &phase.tasks {
                let formatted = task.format();
                assert!(formatted.starts_with("- [ ] T"), "Bad format: {formatted}");
                // Check zero-padded 3 digits
                assert!(task.id.len() == 4, "ID should be T### but got {}", task.id);
            }
        }
    }

    #[test]
    fn task_ids_are_sequential_zero_padded() {
        let spec = sample_spec();
        let tasks = generate_tasks(&spec, "plan", true);
        let all_tasks: Vec<_> = tasks.phases.iter().flat_map(|p| &p.tasks).collect();
        for (i, task) in all_tasks.iter().enumerate() {
            assert_eq!(task.id, format!("T{:03}", i + 1));
        }
    }

    #[test]
    fn story_labels_only_in_story_phases() {
        let spec = sample_spec();
        let tasks = generate_tasks(&spec, "plan", true);

        // Setup and Foundational: no story labels
        for task in &tasks.phases[0].tasks {
            assert!(
                task.story.is_none(),
                "Setup task has story label: {}",
                task.format()
            );
        }
        for task in &tasks.phases[1].tasks {
            assert!(
                task.story.is_none(),
                "Foundational task has story label: {}",
                task.format()
            );
        }

        // Story phases: have labels
        for task in &tasks.phases[2].tasks {
            assert!(
                task.story.is_some(),
                "Story task missing label: {}",
                task.format()
            );
        }

        // Polish: no story labels
        let last = tasks.phases.last().unwrap();
        for task in &last.tasks {
            assert!(
                task.story.is_none(),
                "Polish task has story label: {}",
                task.format()
            );
        }
    }

    #[test]
    fn parallel_markers_present() {
        let spec = sample_spec();
        let tasks = generate_tasks(&spec, "plan", true);
        let all_tasks: Vec<_> = tasks.phases.iter().flat_map(|p| &p.tasks).collect();
        assert!(
            all_tasks.iter().any(|t| t.parallel),
            "No parallel tasks found"
        );
    }

    #[test]
    fn no_data_model_skips_schema_task() {
        let spec = sample_spec();
        let tasks = generate_tasks(&spec, "plan", false);
        let foundational = &tasks.phases[1];
        assert!(
            !foundational
                .tasks
                .iter()
                .any(|t| t.description.contains("data model"))
        );
    }

    #[test]
    fn file_paths_included_in_entity_tasks() {
        let spec = sample_spec();
        let tasks = generate_tasks(&spec, "plan", true);
        let all_tasks: Vec<_> = tasks.phases.iter().flat_map(|p| &p.tasks).collect();
        assert!(
            all_tasks
                .iter()
                .any(|t| t.description.contains("src/models/"))
        );
    }

    #[test]
    fn formatted_output_has_phases_and_checkpoints() {
        let spec = sample_spec();
        let tasks = generate_tasks(&spec, "plan", true);
        let output = format_task_list(&tasks, "Chat", "001-chat");
        assert!(output.contains("Phase 1: Setup"));
        assert!(output.contains("Phase 2: Foundational"));
        assert!(output.contains("Checkpoint"));
        assert!(output.contains("Dependencies & Execution Order"));
    }
}
