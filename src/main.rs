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
mod models;
use clap::Parser;

use std::fs::File;
use std::io::Write;

use std::path::PathBuf;

use inquire::{Editor, Select, Text};
use miette::IntoDiagnostic;
use miette::Result;
use mit_commit::CommitMessage;

use crate::models::{ConventionalChange, ConventionalCommit, ConventionalScope};
use models::fast_conventional_config::FastConventionalConfig;

fn main() -> Result<()> {
    miette::set_panic_hook();

    let args = cli::Args::parse();
    let buf: PathBuf = args.commit_message_path;
    let config: FastConventionalConfig = args.config.try_into()?;
    let existing_commit = CommitMessage::try_from(buf.clone())?;
    let result = existing_commit.try_into();
    let previous_conventional = result.ok();

    let new_conventional = ask_user(&config, previous_conventional)?;
    let commit: CommitMessage<'_> = new_conventional.into();

    let mut commit_file = File::create(&buf).into_diagnostic()?;
    writeln!(commit_file, "{}", String::from(commit.clone())).into_diagnostic()?;

    Ok(())
}

fn ask_user(
    config: &FastConventionalConfig,
    conventional_commit: Option<ConventionalCommit>,
) -> Result<ConventionalCommit> {
    let (type_index, scope_index, previous_breaking, previous_subject, previous_body) =
        conventional_commit
            .map(|conv| {
                (
                    conv.type_index(config.get_types().into_iter().collect::<Vec<_>>()),
                    conv.scope_index(config.get_scopes().into_iter().collect::<Vec<_>>()),
                    match conv.breaking {
                        ConventionalChange::BreakingWithMessage(message) => message,
                        ConventionalChange::Compatible => "".to_string(),
                        ConventionalChange::BreakingWithoutMessage => "See description".to_string(),
                    },
                    conv.subject,
                    conv.body,
                )
            })
            .unwrap_or_default();

    let commit_type: String = Select::new(
        "type",
        config.get_types().into_iter().collect::<Vec<String>>(),
    )
    .with_help_message("What type of change is this?")
    .with_starting_cursor(type_index)
    .prompt()
    .into_diagnostic()?;
    let mut scope: Option<String> = None;

    let scopes = config.get_scopes();
    if !scopes.is_empty() {
        scope = Select::new("scope", scopes.into_iter().collect())
            .with_help_message("What scope your change is within (if any)?")
            .with_starting_cursor(scope_index)
            .prompt_skippable()
            .into_diagnostic()?
            .filter(|scope| !scope.is_empty());
    }

    let mut breaking_ui =
        Text::new("breaking").with_help_message("Did the public interface change?");

    if !previous_breaking.is_empty() {
        breaking_ui = breaking_ui.with_default(&previous_breaking);
    }

    let breaking: Option<String> = breaking_ui
        .prompt_skippable()
        .into_diagnostic()?
        .filter(|breaking_message| !breaking_message.is_empty());

    let subject = Text::new("subject")
        .with_help_message("Summary of the code changes")
        .with_validator(&|subject: &str| {
            if subject.is_empty() {
                Err("subject can't be empty".to_string())
            } else {
                Ok(())
            }
        })
        .with_default(&previous_subject.0)
        .prompt()
        .into_diagnostic()?;
    let body = Editor::new("description")
        .with_predefined_text(&previous_body.0)
        .with_help_message("A body (if any)")
        .prompt_skippable()
        .into_diagnostic()?
        .filter(|breaking_message| !breaking_message.is_empty());
    Ok(ConventionalCommit {
        type_slug: commit_type.into(),
        scope: scope.map(ConventionalScope::from),
        breaking: breaking.into(),
        subject: subject.into(),
        body: body.into(),
    })
}
