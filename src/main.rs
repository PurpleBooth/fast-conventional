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

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use inquire::{Editor, Select, Text};
use miette::IntoDiagnostic;
use miette::Result;
use mit_commit::{CommitMessage, Trailer};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct FastConventionalConfig {
    use_angular: Option<bool>,
    types: Option<Vec<String>>,
    scopes: Option<Vec<String>>,
}

fn main() -> Result<()> {
    miette::set_panic_hook();

    let matches = cli::app().get_matches();
    let config = matches.value_of("config").unwrap();

    let contents = fs::read_to_string(config).into_diagnostic()?;
    let config: FastConventionalConfig = serde_yaml::from_str(&contents).into_diagnostic()?;

    let mut type_options = vec![config.types.unwrap_or_default()];

    if config.use_angular == Some(true) {
        type_options.push(vec![
            "feat".to_string(),
            "fix".to_string(),
            "docs".to_string(),
            "style".to_string(),
            "refactor".to_string(),
            "perf".to_string(),
            "test".to_string(),
            "chore".to_string(),
            "build".to_string(),
            "ci".to_string(),
        ]);
    };

    let type_options: Vec<String> = type_options.iter().flatten().cloned().collect();

    let selected_type: String = Select::new("type", type_options)
        .with_help_message("What type of change is this?")
        .prompt()
        .into_diagnostic()?;
    let mut selected_scope: Option<String> = None;

    if let Some(scopes) = config.scopes {
        selected_scope = Select::new("scope", scopes)
            .with_help_message("What scope is your change within (if any)?")
            .prompt_skippable()
            .into_diagnostic()?
            .filter(|scope| !scope.is_empty());
    }

    let selected_breaking: Option<String> = Text::new("breaking")
        .with_help_message("Did the public interface change?")
        .prompt_skippable()
        .into_diagnostic()?
        .filter(|breaking_message| !breaking_message.is_empty());

    let subject = Text::new("subject")
        .with_help_message("Short summary of the code changes")
        .with_validator(&|subject: &str| {
            if subject.is_empty() {
                Err("subject can't be empty".to_string())
            } else {
                Ok(())
            }
        })
        .prompt()
        .into_diagnostic()?;
    let body = Editor::new("description")
        .with_help_message("A body (if any)")
        .prompt_skippable()
        .into_diagnostic()?
        .filter(|breaking_message| !breaking_message.is_empty());

    let buf: PathBuf = matches.value_of("commit-message-path").unwrap().into();
    let commit: CommitMessage<'_> =
        mit_commit::CommitMessage::try_from(buf.clone()).into_diagnostic()?;

    let mut subject_buffer: String = selected_type;

    if let Some(selected_scope) = selected_scope {
        subject_buffer.push('(');
        subject_buffer.push_str(&selected_scope);
        subject_buffer.push(')');
    }

    if selected_breaking.is_some() {
        subject_buffer.push('!');
    }

    subject_buffer.push_str(": ");
    subject_buffer.push_str(&subject);

    let mut commit = commit.with_subject(subject_buffer.into());

    let body_content = body.clone().unwrap_or_default();
    if body.is_some() {
        commit = commit.with_body_contents(&body_content);
    }

    if let Some(selected_breaking) = selected_breaking {
        commit = commit.add_trailer(Trailer::new(
            "BREAKING CHANGE".into(),
            selected_breaking.into(),
        ));
    }

    let mut commit_file = File::create(&buf).into_diagnostic()?;
    write!(commit_file, "{}", String::from(commit.clone())).into_diagnostic()?;

    Ok(())
}
