use anyhow::Result;

use crate::core::git;

pub fn run() -> Result<()> {
    println!("RustySpec System Check");
    println!("======================\n");

    // Check Git
    let cwd = std::env::current_dir()?;
    if git::is_git_repo(&cwd) {
        println!("  [OK] Git repository detected");
    } else {
        println!("  [--] Git not available in current directory");
    }

    // Check project structure
    let rustyspec_dir = cwd.join(".rustyspec");
    if rustyspec_dir.exists() {
        println!("  [OK] .rustyspec/ directory found");

        let constitution = rustyspec_dir.join("constitution.md");
        if constitution.exists() {
            println!("  [OK] Constitution file present");
        } else {
            println!("  [!!] Constitution file missing");
        }
    } else {
        println!("  [--] Not a RustySpec project (no .rustyspec/ directory)");
    }

    let config_path = cwd.join("rustyspec.toml");
    if config_path.exists() {
        println!("  [OK] rustyspec.toml found");
    } else {
        println!("  [--] rustyspec.toml not found");
    }

    println!("\nRustySpec v{}", env!("CARGO_PKG_VERSION"));
    Ok(())
}
