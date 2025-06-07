//! Quickly put together a conventional commit
#![warn(clippy::nursery)]
#![deny(
    unused,
    nonstandard_style,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::pedantic,
    clippy::cargo,
    clippy::complexity,
    clippy::correctness,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    non_fmt_panics
)]
#![allow(clippy::multiple_crate_versions)]
mod cli;
mod commands;
mod models;
mod repositories;
mod service;
mod ui;

use clap::Parser;

use miette::Result;

use crate::models::ConventionalCommit;
use models::fast_conventional_config::FastConventionalConfig;

fn main() -> Result<()> {
    miette::set_panic_hook();

    let args = cli::Args::parse();

    match args.command {
        cli::Commands::Completion { shell } => {
            commands::completion(shell);
            Ok(())
        }
        cli::Commands::ExampleConfig => commands::example(),
        cli::Commands::Editor {
            commit_message_path,
            config_path,
        } => commands::editor(commit_message_path, config_path),
        cli::Commands::Validate {
            repository_path,
            revision_selection,
            config_path,
        } => commands::validate(repository_path, revision_selection, config_path),
    }
}
