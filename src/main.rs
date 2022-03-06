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
mod ui;

use clap::Parser;

use std::fs::File;
use std::io::Write;

use std::path::PathBuf;

use miette::IntoDiagnostic;
use miette::Result;
use mit_commit::CommitMessage;

use models::fast_conventional_config::FastConventionalConfig;
use models::{ConventionalChange, ConventionalCommit, ConventionalScope};
use ui::{prompt_body, prompt_breaking, prompt_commit_scope, prompt_commit_type, prompt_subject};

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

    let commit_type: String = prompt_commit_type(config, type_index)?;
    let scope: Option<String> = prompt_commit_scope(config, scope_index)?;
    let breaking = prompt_breaking(&previous_breaking)?;
    let subject = prompt_subject(&previous_subject)?;
    let body = prompt_body(&previous_body)?;

    Ok(ConventionalCommit {
        type_slug: commit_type.into(),
        scope: scope.map(ConventionalScope::from),
        breaking: breaking.into(),
        subject: subject.into(),
        body: body.into(),
    })
}
