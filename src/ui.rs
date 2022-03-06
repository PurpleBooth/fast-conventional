use crate::models::ConventionalChange;
use crate::models::ConventionalCommit;
use crate::models::ConventionalScope;
use inquire::{Editor, Select, Text};
use miette::IntoDiagnostic;
use miette::Result;
use mit_commit::CommitMessage;

use super::models::fast_conventional_config::FastConventionalConfig;
use super::models::{ConventionalBody, ConventionalSubject};

pub fn prompt_body(previous_body: &ConventionalBody) -> Result<Option<String>> {
    let mut body_ui = Editor::new("description")
        .with_predefined_text(&previous_body.0)
        .with_help_message("A body (if any)");

    body_ui = if previous_body.is_empty() {
        body_ui
    } else {
        body_ui.with_predefined_text(&previous_body.0)
    };

    Ok(body_ui
        .prompt_skippable()
        .into_diagnostic()?
        .filter(|breaking_message| !breaking_message.is_empty()))
}

pub fn prompt_subject(previous_subject: &ConventionalSubject) -> Result<String> {
    let mut subject_ui = Text::new("subject")
        .with_help_message("Summary of the code changes")
        .with_validator(&|subject: &str| {
            if subject.is_empty() {
                Err("subject can't be empty".to_string())
            } else {
                Ok(())
            }
        });

    subject_ui = if previous_subject.is_empty() {
        subject_ui
    } else {
        subject_ui.with_default(&previous_subject.0)
    };

    subject_ui
        .with_default(&previous_subject.0)
        .prompt()
        .into_diagnostic()
}

pub fn prompt_breaking(previous_breaking: &str) -> Result<Option<String>> {
    let mut breaking_ui =
        Text::new("breaking").with_help_message("Did the public interface change?");

    if !previous_breaking.is_empty() {
        breaking_ui = breaking_ui.with_default(previous_breaking);
    }

    let breaking: Option<String> = breaking_ui
        .prompt_skippable()
        .into_diagnostic()?
        .filter(|breaking_message| !breaking_message.is_empty());
    Ok(breaking)
}

pub fn prompt_commit_scope(
    config: &FastConventionalConfig,
    scope_index: usize,
) -> Result<Option<String>> {
    if config.get_scopes().is_empty() {
        Ok(None)
    } else {
        let scopes = config.get_scopes();
        Ok(Select::new("scope", scopes.into_iter().collect())
            .with_help_message("What scope your change is within (if any)?")
            .with_starting_cursor(scope_index)
            .prompt_skippable()
            .into_diagnostic()?
            .filter(|scope| !scope.is_empty()))
    }
}

pub fn prompt_commit_type(config: &FastConventionalConfig, type_index: usize) -> Result<String> {
    Select::new(
        "type",
        config.get_types().into_iter().collect::<Vec<String>>(),
    )
    .with_help_message("What type of change is this?")
    .with_starting_cursor(type_index)
    .prompt()
    .into_diagnostic()
}

pub fn ask_user(
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

pub fn ask_fallback(previous_text: &'_ str) -> Result<CommitMessage<'_>> {
    Ok(Editor::new("Non-conventional editor")
        .with_predefined_text(previous_text)
        .with_help_message("This commit isn't conventional")
        .prompt()
        .into_diagnostic()?
        .into())
}
