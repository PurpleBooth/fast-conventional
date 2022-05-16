use crate::repositories::GitRepository;
use crate::ConventionalCommit;
use miette::{IntoDiagnostic, Result};
use std::path::PathBuf;

use crate::models::GitRevisionSelection;
use miette::Diagnostic;
use thiserror::Error;

pub fn run(repository: PathBuf, revision_selection: Option<GitRevisionSelection>) -> Result<()> {
    let repository = GitRepository::try_from(repository)?;
    let commits = repository.list_commits(revision_selection)?;
    let mut result = Ok(());

    for (_, commit_message) in commits {
        match ConventionalCommit::try_from(commit_message.clone()) {
            Err(_) => {
                result = Err(Failed).into_diagnostic();
                eprintln!("[✘] {}", commit_message.get_subject());
            }
            Ok(_) => {
                println!("[✔] {}", commit_message.get_subject());
            }
        }
    }

    result
}

#[non_exhaustive]
#[derive(Error, Debug, Diagnostic)]
#[error("Some commits failed validation")]
#[diagnostic(code(commands::validate::failed), url(docsrs))]
pub struct Failed;
