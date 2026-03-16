mod agents;
mod cli;
mod config;
mod core;
mod extensions;
mod presets;
mod templates;

use anyhow::Result;
use clap::Parser;
use cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set log level before initializing logger — no unsafe needed
    if cli.debug {
        env_logger::Builder::new()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::init();
    }

    cli::run(cli)
}
