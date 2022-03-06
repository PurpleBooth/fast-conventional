//! Quickly put together a conventional commit
#![warn(
    rust_2018_idioms,
    unused,
    rust_2021_compatibility,
    nonstandard_style,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs
)]

mod cli;
mod completion;
mod models;
mod ui;

use clap::Parser;

use std::fs::File;
use std::io::Write;

use std::path::PathBuf;

use clap::IntoApp;
use miette::IntoDiagnostic;
use miette::Result;
use mit_commit::CommitMessage;

use models::fast_conventional_config::FastConventionalConfig;

fn main() -> Result<()> {
    miette::set_panic_hook();

    let args = cli::Args::parse();

    match args.command {
        cli::Commands::Completion { shell } => {
            completion::print_completions(shell, &mut cli::Args::command());
            Ok(())
        }
        cli::Commands::Editor {
            commit_message_path,
            config,
        } => {
            let buf: PathBuf = commit_message_path;
            let config: FastConventionalConfig = config.try_into()?;
            let existing_commit = CommitMessage::try_from(buf.clone())?;
            let result = existing_commit.try_into();
            let previous_conventional = result.ok();

            let new_conventional = ui::ask_user(&config, previous_conventional)?;
            let commit: CommitMessage<'_> = new_conventional.into();

            let mut commit_file = File::create(&buf).into_diagnostic()?;
            writeln!(commit_file, "{}", String::from(commit.clone())).into_diagnostic()?;

            Ok(())
        }
    }
}
