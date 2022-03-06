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
use std::fs;

use std::fs::File;
use std::io::Write;

use std::path::PathBuf;

use clap::IntoApp;
use miette::IntoDiagnostic;
use miette::Result;
use mit_commit::CommitMessage;

use crate::models::ConventionalCommit;
use models::fast_conventional_config::FastConventionalConfig;

fn main() -> Result<()> {
    miette::set_panic_hook();

    let args = cli::Args::parse();

    match args.command {
        cli::Commands::Completion { shell } => {
            completion::print_completions(shell, &mut cli::Args::command());
            Ok(())
        }
        cli::Commands::ExampleConfig => {
            let config = FastConventionalConfig {
                use_angular: Some(true),
                types: Some(vec!["custom_type".to_string()]),
                scopes: Some(vec![
                    "src".to_string(),
                    "actions".to_string(),
                    "manpages".to_string(),
                    "readme".to_string(),
                    "e2e".to_string(),
                    "unit".to_string(),
                ]),
            };

            let example: String = config.try_into()?;

            println!("{}", example);

            Ok(())
        }
        cli::Commands::Editor {
            commit_message_path,
            config,
        } => {
            let buf: PathBuf = commit_message_path;
            let config: FastConventionalConfig = config.try_into()?;
            let existing_contents = fs::read_to_string(buf.clone()).into_diagnostic()?;
            let existing_commit = CommitMessage::from(existing_contents.clone());
            let has_bodies = existing_commit
                .get_body()
                .iter()
                .filter(|body| !body.is_empty())
                .count();

            let commit = match ConventionalCommit::try_from(existing_commit.clone()) {
                Ok(previous_conventional) => {
                    ui::ask_user(&config, Some(previous_conventional))?.into()
                }
                Err(_) if has_bodies == 0 => ui::ask_user(&config, None)?.into(),
                Err(_) => ui::ask_fallback(&existing_contents)?,
            };

            let mut commit_file = File::create(&buf).into_diagnostic()?;
            writeln!(commit_file, "{}", String::from(commit.clone())).into_diagnostic()?;

            Ok(())
        }
    }
}
