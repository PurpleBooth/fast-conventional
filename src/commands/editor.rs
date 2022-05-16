use miette::{IntoDiagnostic, Result};
use mit_commit::CommitMessage;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

use std::io::Write;

use crate::{ui, ConventionalCommit, FastConventionalConfig};

pub fn run(commit_message_path: PathBuf, config_path: PathBuf) -> Result<()> {
    let buf: PathBuf = commit_message_path;
    let config: FastConventionalConfig = config_path.try_into()?;
    let existing_contents = fs::read_to_string(buf.clone()).into_diagnostic()?;
    let existing_commit = CommitMessage::from(existing_contents.clone());
    let has_bodies = existing_commit
        .get_body()
        .iter()
        .filter(|body| !body.is_empty())
        .count();

    let commit = match ConventionalCommit::try_from(existing_commit.clone()) {
        Ok(previous_conventional) => ui::ask_user(&config, Some(previous_conventional))?.into(),
        Err(_) if has_bodies == 0 => ui::ask_user(&config, None)?.into(),
        Err(_) => ui::ask_fallback(&existing_contents)?,
    };

    let mut commit_file = File::create(&buf).into_diagnostic()?;
    writeln!(commit_file, "{}", String::from(commit.clone())).into_diagnostic()?;

    Ok(())
}
