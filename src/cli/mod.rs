pub mod analyze;
pub mod check;
pub mod checklist;
pub mod clarify;
pub mod completions;
pub mod extension;
pub mod implement;
pub mod init;
pub mod plan;
pub mod preset;
pub mod specify;
pub mod tasks;
pub mod upgrade;
pub mod ux;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "rustyspec",
    version,
    about = "Specification-Driven Development CLI"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Verbose debug output
    #[arg(long, global = true)]
    pub debug: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new RustySpec project
    Init {
        /// Project name (initializes in current directory if omitted)
        name: Option<String>,

        /// Initialize in current directory
        #[arg(long)]
        here: bool,

        /// Skip Git repository initialization
        #[arg(long)]
        no_git: bool,

        /// Skip confirmation prompts
        #[arg(long)]
        force: bool,
    },

    /// Create a new feature specification
    Specify {
        /// Feature description
        #[arg(name = "feature-name")]
        feature_name: String,
    },

    /// Resolve ambiguities in a specification
    Clarify {
        /// Feature ID (e.g., 001) — auto-detected if omitted
        feature_id: Option<String>,
    },

    /// Generate an architecture plan from a specification
    Plan {
        /// Feature ID (e.g., 001) — auto-detected if omitted
        feature_id: Option<String>,
    },

    /// Generate a story-driven task breakdown from the plan
    Tasks {
        /// Feature ID (e.g., 001) — auto-detected if omitted
        feature_id: Option<String>,
    },

    /// Execute tasks from the task breakdown
    Implement {
        /// Feature ID (e.g., 001) — auto-detected if omitted
        feature_id: Option<String>,

        /// Multi-pass implementation (for iterative refinement)
        #[arg(long)]
        pass: Option<u32>,
    },

    /// Validate cross-artifact consistency (read-only)
    Analyze {
        /// Feature ID (e.g., 001) — auto-detected if omitted
        feature_id: Option<String>,
    },

    /// Generate a quality validation checklist
    Checklist {
        /// Feature ID (e.g., 001) — auto-detected if omitted
        feature_id: Option<String>,

        /// Append to existing checklist instead of creating new
        #[arg(long)]
        append: bool,
    },

    /// Manage workflow presets
    Preset {
        #[command(subcommand)]
        command: preset::PresetCommands,
    },

    /// Manage extensions
    Extension {
        #[command(subcommand)]
        command: extension::ExtensionCommands,
    },

    /// Refresh templates and scripts after a RustySpec update
    Upgrade {
        /// Skip confirmation prompts
        #[arg(long)]
        force: bool,
    },

    /// Generate shell completions
    Completions {
        /// Shell type: bash, zsh, fish, powershell
        shell: String,
    },

    /// Verify system prerequisites
    Check,
}

pub fn run(cli: Cli) -> Result<()> {
    // Logger already initialized in main() based on --debug flag

    match cli.command {
        Commands::Init {
            name,
            here,
            no_git,
            force,
        } => init::run(name, here, no_git, force),
        Commands::Specify { feature_name } => specify::run(&feature_name),
        Commands::Clarify { feature_id } => clarify::run(feature_id.as_deref()),
        Commands::Plan { feature_id } => plan::run(feature_id.as_deref()),
        Commands::Tasks { feature_id } => tasks::run(feature_id.as_deref()),
        Commands::Implement { feature_id, pass } => implement::run(feature_id.as_deref(), pass),
        Commands::Analyze { feature_id } => analyze::run(feature_id.as_deref()),
        Commands::Checklist { feature_id, append } => checklist::run(feature_id.as_deref(), append),
        Commands::Preset { command } => preset::run(command),
        Commands::Extension { command } => extension::run(command),
        Commands::Upgrade { force } => upgrade::run(force),
        Commands::Completions { shell } => completions::run(&shell),
        Commands::Check => check::run(),
    }
}
