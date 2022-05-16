use std::collections::BTreeMap;

use crate::models::GitShortRef;
use crate::{ConventionalCommit, FastConventionalConfig};
use mit_commit::CommitMessage;

pub fn run<'a, 'b>(
    config: &'a FastConventionalConfig,
    commit_messages: Vec<(GitShortRef, CommitMessage<'b>)>,
) -> (
    BTreeMap<GitShortRef, CommitMessage<'b>>,
    BTreeMap<GitShortRef, CommitMessage<'b>>,
) {
    commit_messages.into_iter().partition(is_valid_with(config))
}

fn is_valid_with(
    config: &FastConventionalConfig,
) -> impl Fn(&(GitShortRef, CommitMessage<'_>)) -> bool + '_ {
    |message: &(GitShortRef, CommitMessage<'_>)| -> bool {
        match ConventionalCommit::try_from(message.clone().1) {
            Ok(conventional_commit) => {
                let expected_type: String = conventional_commit.type_slug.into();
                config.get_types().contains(&expected_type)
            }
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::GitShortRef;
    use crate::FastConventionalConfig;
    use mit_commit::CommitMessage;

    #[test]
    fn fails_if_commit_is_not_conventional() {
        let actual = run(
            &FastConventionalConfig {
                use_angular: Some(true),
                types: None,
                scopes: None,
            },
            vec![
                ("cafebabe".into(), CommitMessage::from("Not Conventional")),
                (
                    "deadbeef".into(),
                    CommitMessage::from("fix: Example commit"),
                ),
            ],
        );

        let failed_commits: BTreeMap<GitShortRef, CommitMessage<'_>> =
            vec![("cafebabe".into(), CommitMessage::from("Not Conventional"))]
                .into_iter()
                .collect();
        let valid_commits: BTreeMap<GitShortRef, CommitMessage<'_>> = vec![(
            "deadbeef".into(),
            CommitMessage::from("fix: Example commit"),
        )]
        .into_iter()
        .collect();
        assert_eq!(actual.0, valid_commits);
        assert_eq!(actual.1, failed_commits);
    }

    #[test]
    fn fails_if_commit_has_a_type_that_is_not_in_the_types_list() {
        let actual = run(
            &FastConventionalConfig {
                use_angular: Some(true),
                types: None,
                scopes: None,
            },
            vec![
                (
                    "deadbeef".into(),
                    CommitMessage::from("fix: Example commit"),
                ),
                (
                    "cafebabe".into(),
                    CommitMessage::from("missing: Example commit"),
                ),
            ],
        );

        let failed_commits: BTreeMap<GitShortRef, CommitMessage<'_>> = vec![(
            "cafebabe".into(),
            CommitMessage::from("missing: Example commit"),
        )]
        .into_iter()
        .collect();
        let valid_commits: BTreeMap<GitShortRef, CommitMessage<'_>> = vec![(
            "deadbeef".into(),
            CommitMessage::from("fix: Example commit"),
        )]
        .into_iter()
        .collect();
        assert_eq!(actual.0, valid_commits);
        assert_eq!(actual.1, failed_commits);
    }
}
