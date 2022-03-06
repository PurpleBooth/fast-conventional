use inquire::{Editor, Select, Text};
use miette::IntoDiagnostic;
use miette::Result;

use super::models::fast_conventional_config::FastConventionalConfig;
use super::models::{ConventionalBody, ConventionalSubject};

pub fn prompt_body(previous_body: &ConventionalBody) -> Result<Option<String>> {
    Ok(Editor::new("description")
        .with_predefined_text(&previous_body.0)
        .with_help_message("A body (if any)")
        .prompt_skippable()
        .into_diagnostic()?
        .filter(|breaking_message| !breaking_message.is_empty()))
}

pub fn prompt_subject(previous_subject: &ConventionalSubject) -> Result<String> {
    Text::new("subject")
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
