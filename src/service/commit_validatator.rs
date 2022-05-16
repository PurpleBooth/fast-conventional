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
            Ok(conventional_commit) => uses_configured_values(config, &conventional_commit),
            Err(_) => false,
        }
    }
}

fn uses_configured_values(
    config: &FastConventionalConfig,
    conventional_commit: &ConventionalCommit,
) -> bool {
    is_type_slug_in_config(config, conventional_commit)
        && is_scope_in_config(config, conventional_commit)
}

fn is_type_slug_in_config(
    config: &FastConventionalConfig,
    conventional_commit: &ConventionalCommit,
) -> bool {
    let type_slug: String = conventional_commit.type_slug.clone().into();
    config.get_types().contains(&type_slug)
}

fn is_scope_in_config(
    config: &FastConventionalConfig,
    conventional_commit: &ConventionalCommit,
) -> bool {
    match &conventional_commit.scope {
        None => true,
        Some(scope) => {
            let expected_scope: String = scope.clone().into();
            config.get_scopes().contains(&expected_scope)
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

    #[test]
    fn fails_if_commit_has_a_scope_that_is_not_in_the_scopes_list() {
        let actual = run(
            &FastConventionalConfig {
                use_angular: Some(true),
                types: None,
                scopes: Some(vec!["FastConventional".into()]),
            },
            vec![
                (
                    "deadbeef".into(),
                    CommitMessage::from("fix(FastConventional): Example commit"),
                ),
                (
                    "cafebabe".into(),
                    CommitMessage::from("fix(missing): Example commit"),
                ),
            ],
        );

        let failed_commits: BTreeMap<GitShortRef, CommitMessage<'_>> = vec![(
            "cafebabe".into(),
            CommitMessage::from("fix(missing): Example commit"),
        )]
        .into_iter()
        .collect();
        let valid_commits: BTreeMap<GitShortRef, CommitMessage<'_>> = vec![(
            "deadbeef".into(),
            CommitMessage::from("fix(FastConventional): Example commit"),
        )]
        .into_iter()
        .collect();
        assert_eq!(actual.0, valid_commits);
        assert_eq!(actual.1, failed_commits);
    }
}
