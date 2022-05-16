use std::path::PathBuf;

use git2::ObjectType as Git2ObjectType;
use git2::Oid as Git2Oid;
use git2::Repository as Git2Repository;
use git2::Sort as Git2Sort;
use git2::{Error as Git2Error, Revwalk};
use miette::Diagnostic;
use mit_commit::CommitMessage;
use thiserror::Error;

use crate::models::{GitRevisionSelection, GitShortRef};

pub struct Repository(Git2Repository);

impl Repository {
    pub(crate) fn list_commits(
        &self,
        revision_selection: Option<GitRevisionSelection>,
    ) -> Result<Vec<(GitShortRef, CommitMessage<'_>)>, CommitListError> {
        let git2_revwalk = self.build_revwalk(revision_selection)?;

        let git2_references = git2_revwalk
            .into_iter()
            .collect::<Result<Vec<Git2Oid>, _>>()?;

        let git2_commits = git2_references
            .clone()
            .into_iter()
            .map(|oid| self.0.find_commit(oid))
            .collect::<Result<Vec<_>, _>>()?;

        let mit_commits = git2_commits
            .into_iter()
            .map(|message| match message.message() {
                None => CommitMessage::default(),
                Some(message) => CommitMessage::from(message.to_string()),
            });

        let combined_commits: Vec<(GitShortRef, CommitMessage<'_>)> = git2_references
            .into_iter()
            .map(|x| x.to_string().into())
            .zip(mit_commits)
            .collect();

        Ok(combined_commits)
    }

    fn build_revwalk(
        &self,
        revision_selection: Option<GitRevisionSelection>,
    ) -> Result<Revwalk<'_>, CommitListError> {
        let mut git2_revwalk = self.0.revwalk()?;

        match revision_selection {
            None => {
                git2_revwalk.push_head()?;
            }
            Some(revision_selection) => {
                let selection = String::from(revision_selection);
                let revspec = self.0.revparse(&selection)?;
                match revspec.mode() {
                    git2::RevparseMode::SINGLE => {
                        git2_revwalk.push(revspec.from().unwrap().id())?;
                    }
                    git2::RevparseMode::RANGE => {
                        let from = revspec.from().unwrap().id();
                        let to = revspec.to().unwrap().id();
                        git2_revwalk.push(to)?;
                        git2_revwalk.hide(from)?;
                    }
                    git2::RevparseMode::MERGE_BASE => {
                        let from = revspec.from().unwrap().id();
                        let to = revspec.to().unwrap().id();
                        git2_revwalk.push(to)?;
                        let base = self.0.merge_base(from, to)?;
                        let object = self.0.find_object(base, Some(Git2ObjectType::Commit))?;
                        git2_revwalk.push(object.id())?;
                        git2_revwalk.hide(from)?;
                    }
                    _ => unimplemented!(),
                };
            }
        };
        git2_revwalk.set_sorting(Git2Sort::TOPOLOGICAL | Git2Sort::REVERSE | Git2Sort::TIME)?;

        Ok(git2_revwalk)
    }
}

impl TryFrom<PathBuf> for Repository {
    type Error = OpenError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        Ok(Self(Git2Repository::open(value)?))
    }
}

#[non_exhaustive]
#[derive(Error, Debug, Diagnostic)]
#[error(transparent)]
#[diagnostic(
    code(repositories::git::repository::repository_open_error),
    url(docsrs)
)]
pub struct OpenError(#[from] Git2Error);

#[non_exhaustive]
#[derive(Error, Debug, Diagnostic)]
#[error(transparent)]
#[diagnostic(code(repositories::git::repository::commit_list_error), url(docsrs))]
pub struct CommitListError(#[from] Git2Error);

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use git2::RepositoryInitOptions as Git2RepositoryInitOptions;
    use git2::{Repository as Git2Repository, Signature};
    use mit_commit::CommitMessage;
    use tempfile::{tempdir, TempDir};

    use super::*;

    const COMMIT_USER: &str = "user";
    const COMMIT_EMAIL: &str = "user@example.com";
    const COMMIT_MESSAGE_1: &str = "initial\n\nbody";
    const COMMIT_MESSAGE_2: &str = "Message 2";
    const COMMIT_MESSAGE_3: &str = "Message 3";

    pub fn repo_init() -> TempDir {
        let temporary_dir = tempdir().unwrap();
        let mut opts = Git2RepositoryInitOptions::new();
        opts.initial_head("main");
        let git2_repo = Git2Repository::init_opts(temporary_dir.path(), &opts).unwrap();
        {
            let mut git2_config = git2_repo.config().unwrap();
            git2_config.set_str("user.name", COMMIT_USER).unwrap();
            git2_config.set_str("user.email", COMMIT_EMAIL).unwrap();
            let mut index = git2_repo.index().unwrap();
            let id = index.write_tree().unwrap();

            let git2_tree = git2_repo.find_tree(id).unwrap();
            let sig = git2_repo.signature().unwrap();
            git2_repo
                .commit(Some("HEAD"), &sig, &sig, COMMIT_MESSAGE_1, &git2_tree, &[])
                .unwrap();
            let message_2 = COMMIT_MESSAGE_2;
            make_commit(&git2_repo, message_2);
            let message_3 = COMMIT_MESSAGE_3;
            make_commit(&git2_repo, message_3);
        }
        temporary_dir
    }

    fn make_commit(git2_repo: &Git2Repository, message: &str) {
        let author = Signature::now(COMMIT_USER, COMMIT_EMAIL).unwrap();
        let parent_commit = git2_repo.head().unwrap().peel_to_commit().unwrap();
        let commit_tree = git2_repo.head().unwrap().peel_to_tree().unwrap();
        git2_repo
            .commit(
                Some("HEAD"),
                &author,
                &author,
                message,
                &commit_tree,
                &[&parent_commit],
            )
            .unwrap();
    }

    fn take_commit_message(pair: (GitShortRef, CommitMessage<'_>)) -> CommitMessage<'_> {
        pair.1
    }

    #[test]
    fn can_be_created_from_a_git_repository() {
        let dir = repo_init();
        assert!(Repository::try_from(dir.into_path()).is_ok());
    }

    #[test]
    fn errors_on_not_a_git_repository() {
        let dir = tempdir().unwrap();
        assert!(Repository::try_from(dir.into_path()).is_err());
    }

    #[test]
    fn it_can_give_me_a_list_of_commit_messages() {
        let dir = repo_init();
        let repo = Repository::try_from(dir.into_path()).unwrap();
        let commits = repo
            .list_commits(None)
            .unwrap()
            .into_iter()
            .map(|(_, commit_message)| commit_message)
            .collect::<Vec<_>>();

        assert_eq!(
            commits,
            vec![
                CommitMessage::from(COMMIT_MESSAGE_1),
                CommitMessage::from(COMMIT_MESSAGE_2),
                CommitMessage::from(COMMIT_MESSAGE_3)
            ]
        );
    }

    #[test]
    fn it_can_give_me_a_list_of_commits_like_git_log() {
        let dir = repo_init();
        let repo = Repository::try_from(dir.into_path()).unwrap();
        let commits = repo
            .list_commits(Some(GitRevisionSelection::from_str("HEAD^").unwrap()))
            .unwrap()
            .into_iter()
            .map(|(_, commit_message)| commit_message)
            .collect::<Vec<_>>();

        assert_eq!(
            commits,
            vec![
                CommitMessage::from(COMMIT_MESSAGE_1),
                CommitMessage::from(COMMIT_MESSAGE_2)
            ]
        );
    }

    #[test]
    fn it_can_give_me_a_commit_from_a_range() {
        let dir = repo_init();
        let repo = Repository::try_from(dir.into_path()).unwrap();
        let commits = repo
            .list_commits(Some(GitRevisionSelection::from_str("HEAD^..HEAD").unwrap()))
            .unwrap()
            .into_iter()
            .map(|(_, commit_message)| commit_message)
            .collect::<Vec<_>>();

        assert_eq!(commits, vec![CommitMessage::from(COMMIT_MESSAGE_3)]);
    }

    #[test]
    fn it_can_give_me_a_commit_from_a_range_with_the_finishing_id_missing() {
        let dir = repo_init();

        let repo = Repository::try_from(dir.into_path()).unwrap();
        let commits = repo
            .list_commits(Some(GitRevisionSelection::from_str("HEAD^^..").unwrap()))
            .unwrap()
            .into_iter()
            .map(take_commit_message)
            .collect::<Vec<_>>();

        assert_eq!(
            commits,
            vec![
                CommitMessage::from(COMMIT_MESSAGE_2),
                CommitMessage::from(COMMIT_MESSAGE_3),
            ]
        );
    }
}
