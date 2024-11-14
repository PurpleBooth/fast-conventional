use crate::repositories::GitRepository;
use crate::FastConventionalConfig;
use miette::{IntoDiagnostic, Result};
use std::path::PathBuf;

use crate::models::{GitRevisionSelection, GitShortRef};
use crate::service::commit_validatator;
use miette::Diagnostic;
use thiserror::Error;

pub fn run(
    repository_path: PathBuf,
    revision_selection: Option<GitRevisionSelection>,
    config_path: PathBuf,
) -> Result<()> {
    let config: FastConventionalConfig = config_path.try_into()?;
    let repository = GitRepository::try_from(repository_path)?;
    let commits = repository.list_commits(revision_selection)?;
    let (_, failed) = commit_validatator::run(&config, commits.clone());

    for pair in &commits {
        if failed.contains_key(&pair.0) {
            eprintln!("[✘] {}", pair.1.get_subject());
        } else {
            println!("[✔] {}", pair.1.get_subject());
        }
    }

    if failed.is_empty() {
        Ok(())
    } else {
        Err(Failed::new(failed.keys().cloned().collect())).into_diagnostic()
    }
}

#[non_exhaustive]
#[derive(Error, Debug, Diagnostic)]
#[error("Some commits failed validation")]
#[diagnostic(code(commands::validate::failed), url(docsrs), help("You need to amend, then the following commits {:#?}", self.failed_commits))]
pub struct Failed {
    failed_commits: Vec<GitShortRef>,
}

impl Failed {
    const fn new(failed_commits: Vec<GitShortRef>) -> Self {
        Self { failed_commits }
    }
}
