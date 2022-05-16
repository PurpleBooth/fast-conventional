use crate::repositories::GitRepository;
use crate::ConventionalCommit;
use miette::{IntoDiagnostic, Result};
use std::path::PathBuf;

use crate::models::{GitRevisionSelection, GitShortRef};
use miette::Diagnostic;
use thiserror::Error;

pub fn run(repository: PathBuf, revision_selection: Option<GitRevisionSelection>) -> Result<()> {
    let repository = GitRepository::try_from(repository)?;
    let commits = repository.list_commits(revision_selection)?;
    let mut failed_commits = vec![];

    for (commit_id, commit_message) in &commits {
        match ConventionalCommit::try_from(commit_message.clone()) {
            Err(_) => {
                failed_commits.push(commit_id.clone());
                eprintln!("[✘] {}", commit_message.get_subject());
            }
            Ok(_) => {
                println!("[✔] {}", commit_message.get_subject());
            }
        }
    }

    if failed_commits.is_empty() {
        Ok(())
    } else {
        Err(Failed { failed_commits }).into_diagnostic()
    }
}

#[non_exhaustive]
#[derive(Error, Debug, Diagnostic)]
#[error("Some commits failed validation")]
#[diagnostic(code(commands::validate::failed), url(docsrs), help("You need to amend, then the following commits {:#?}", self.failed_commits))]
pub struct Failed {
    failed_commits: Vec<GitShortRef>,
}
