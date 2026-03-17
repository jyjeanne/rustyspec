use std::io::Write;
use std::time::Instant;

use anyhow::{Context, Result};

use crate::config;
use crate::core::{feature, pipeline};

#[allow(clippy::too_many_arguments)]
pub fn run(
    feature_id: Option<&str>,
    new_desc: Option<&str>,
    from: Option<&str>,
    to: Option<&str>,
    only: Option<&str>,
    force: bool,
    dry_run: bool,
    auto: bool,
) -> Result<()> {
    // Validate mutual exclusivity
    if new_desc.is_some() && feature_id.is_some() {
        anyhow::bail!("Cannot use --new with a feature ID. Use one or the other.");
    }

    let cwd = std::env::current_dir()?;
    let project_root = config::find_project_root(&cwd)
        .context("Not inside a RustySpec project. Run 'rustyspec init' first.")?;

    let root_config = config::RootConfig::load(&project_root.join("rustyspec.toml"))?;
    let default_agent = &root_config.ai.default_agent;

    // Validate pipeline config agent IDs
    let valid_agents: Vec<&str> = crate::agents::config::AGENTS.iter().map(|a| a.id).collect();
    let mut all_valid: Vec<&str> = valid_agents.clone();
    for a in crate::agents::config::AGENTS {
        for alias in a.aliases {
            all_valid.push(alias);
        }
    }
    root_config.pipeline.validate(&all_valid)?;

    // Resolve --only to --from + --to
    let (from, to) = if let Some(phase) = only {
        (Some(phase), Some(phase))
    } else {
        (from, to)
    };

    // Filter phases
    let phases = pipeline::filter_phases(from, to)?;

    // Resolve or create feature
    let is_new = new_desc.is_some();
    let mut feature_dir_name = if is_new {
        let desc = new_desc.unwrap();
        if desc.trim().is_empty() {
            anyhow::bail!("Feature description must not be empty.");
        }
        // Pre-compute name for display; specify will create the actual dir
        let specs_dir = project_root.join("specs");
        let num = feature::next_feature_number(&specs_dir)?;
        let fid = feature::format_feature_id(num);
        let short = feature::generate_branch_name(desc)?;
        format!("{fid}-{short}")
    } else {
        feature::resolve_feature(feature_id, &project_root)?
    };

    let mut feature_dir = project_root.join("specs").join(&feature_dir_name);

    println!("Pipeline: {feature_dir_name}\n");

    if dry_run {
        for (i, phase) in phases.iter().enumerate() {
            let agent = root_config.pipeline.agent_for_phase(phase, default_agent);
            let skip = pipeline::should_skip(phase, &feature_dir, force);
            let ptype = pipeline::phase_type(phase);
            let type_label = if ptype == pipeline::PhaseType::Handoff {
                " [HANDOFF]"
            } else {
                ""
            };
            let status = if skip { "○ skip" } else { "● run" };
            println!(
                "  Phase {}/{}: {} ({}){} — {}",
                i + 1,
                phases.len(),
                phase,
                agent,
                type_label,
                status
            );
        }
        println!("\n[dry-run] No files created or modified.");
        return Ok(());
    }

    // For --new, do NOT pre-create the feature dir.
    // The specify phase will create it with the correct numbering.

    let mut results: Vec<pipeline::PhaseResult> = Vec::new();

    for (i, phase) in phases.iter().enumerate() {
        let agent = root_config.pipeline.agent_for_phase(phase, default_agent);
        let ptype = pipeline::phase_type(phase);

        // Check skip
        if pipeline::should_skip(phase, &feature_dir, force) {
            let reason = skip_reason(phase, &feature_dir);
            println!(
                "  Phase {}/{}: {} ({})\n    ○ skipped — {reason}",
                i + 1,
                phases.len(),
                phase,
                agent
            );
            results.push(pipeline::PhaseResult {
                name: phase.to_string(),
                agent: agent.clone(),
                status: pipeline::PhaseStatus::Skipped,
                duration_ms: 0,
                output: reason,
            });
            continue;
        }

        // Print phase header
        let type_label = if ptype == pipeline::PhaseType::Handoff {
            " [HANDOFF]"
        } else {
            ""
        };
        println!(
            "  Phase {}/{}: {} ({}){type_label}",
            i + 1,
            phases.len(),
            phase,
            agent
        );

        // Execute phase
        let start = Instant::now();
        let result = execute_phase(
            phase,
            &feature_dir_name,
            &feature_dir,
            &project_root,
            &agent,
            new_desc,
            auto,
        );
        let elapsed_ms = start.elapsed().as_millis() as u64;

        match result {
            Ok(output) => {
                let status = if ptype == pipeline::PhaseType::Handoff {
                    pipeline::PhaseStatus::Handoff
                } else {
                    pipeline::PhaseStatus::Done
                };
                let duration_str = if status == pipeline::PhaseStatus::Handoff {
                    "user-confirmed".to_string()
                } else {
                    format!("{:.1}s", elapsed_ms as f64 / 1000.0)
                };
                println!("    ✓ {} ({duration_str})", output);
                results.push(pipeline::PhaseResult {
                    name: phase.to_string(),
                    agent: agent.clone(),
                    status,
                    duration_ms: elapsed_ms,
                    output,
                });

                // After specify with --new, re-detect the actual feature dir
                // (specify creates its own numbering which may differ)
                if *phase == "specify" && is_new {
                    feature_dir_name = feature::resolve_feature(None, &project_root)?;
                    feature_dir = project_root.join("specs").join(&feature_dir_name);
                }
            }
            Err(e) => {
                println!("    ✗ FAILED: {e}");
                results.push(pipeline::PhaseResult {
                    name: phase.to_string(),
                    agent: agent.clone(),
                    status: pipeline::PhaseStatus::Failed,
                    duration_ms: elapsed_ms,
                    output: format!("error: {e}"),
                });
                // Write partial log before stopping
                if feature_dir.exists() {
                    pipeline::write_log(&feature_dir, &feature_dir_name, &results).ok();
                }
                anyhow::bail!("Pipeline stopped at phase '{}': {e}", phase);
            }
        }
    }

    // Write pipeline log
    if feature_dir.exists() {
        pipeline::write_log(&feature_dir, &feature_dir_name, &results)?;
    }

    let agent_list: Vec<String> = {
        let mut seen = Vec::new();
        for r in &results {
            if !seen.contains(&r.agent) {
                seen.push(r.agent.clone());
            }
        }
        seen
    };

    println!(
        "\nPipeline complete: {} phases, {} agents ({})",
        results.len(),
        agent_list.len(),
        agent_list.join(", ")
    );
    println!("Log: specs/{feature_dir_name}/pipeline-log.md");
    Ok(())
}

fn execute_phase(
    phase: &str,
    feature_dir_name: &str,
    _feature_dir: &std::path::Path,
    _project_root: &std::path::Path,
    agent: &str,
    new_desc: Option<&str>,
    auto: bool,
) -> Result<String> {
    match phase {
        "specify" => {
            let desc = new_desc.unwrap_or(feature_dir_name);
            crate::cli::specify::run(desc)?;
            Ok("spec.md created".into())
        }
        "clarify" => {
            crate::cli::clarify::run(Some(feature_dir_name))?;
            Ok("clarification complete".into())
        }
        "plan" => {
            crate::cli::plan::run(Some(feature_dir_name))?;
            Ok("plan.md + supporting docs".into())
        }
        "tasks" => {
            crate::cli::tasks::run(Some(feature_dir_name))?;
            Ok("tasks.md generated".into())
        }
        "tests" => {
            crate::cli::tests_cmd::run(Some(feature_dir_name), None, None, false)?;
            Ok("test scaffolds generated".into())
        }
        "implement" => {
            // Handoff: show instructions and wait for confirmation
            println!("    → Open {} and run: /rustyspec-implement", agent);
            if auto {
                println!("    [auto] Skipping confirmation");
            } else {
                print!("    ⏳ Press Enter when done (or Ctrl+C to abort)... ");
                std::io::stdout().flush()?;
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
            }
            Ok("user-confirmed".into())
        }
        "analyze" => {
            crate::cli::analyze::run(Some(feature_dir_name))?;
            Ok("analysis complete".into())
        }
        _ => anyhow::bail!("Unknown phase: {phase}"),
    }
}

fn skip_reason(phase: &str, _feature_dir: &std::path::Path) -> String {
    match phase {
        "specify" => "spec.md already exists".into(),
        "clarify" => "no [NEEDS CLARIFICATION] markers".into(),
        "plan" => "plan.md already exists".into(),
        "tasks" => "tasks.md already exists".into(),
        "tests" => "tests/ directory exists".into(),
        "implement" => "all tasks completed".into(),
        _ => "condition met".into(),
    }
}
