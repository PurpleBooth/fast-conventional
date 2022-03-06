use miette::{ErrReport, IntoDiagnostic, Result};
use mit_commit::{CommitMessage, Trailer};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until1;

use nom::combinator::{opt, rest};

use nom::sequence::{delimited, pair, preceded, terminated, tuple};

#[derive(Clone, PartialOrd, PartialEq, Debug)]
pub enum Change {
    BreakingWithMessage(String),
    Compatible,
    BreakingWithoutMessage,
}

impl Default for Change {
    fn default() -> Self {
        Self::Compatible
    }
}

#[derive(Clone, PartialOrd, PartialEq, Default, Debug)]
pub struct ConventionalCommit {
    pub(crate) subject: String,
    pub(crate) body: String,
    pub(crate) breaking: Change,
    pub(crate) type_slug: String,
    pub(crate) scope: Option<String>,
}

impl ConventionalCommit {
    pub fn type_index(&self, option: Vec<String>) -> usize {
        option
            .into_iter()
            .position(|option| self.type_slug == option)
            .unwrap_or_default()
    }

    pub fn scope_index(&self, option: Vec<String>) -> usize {
        option
            .into_iter()
            .position(|option| {
                self.scope
                    .clone()
                    .filter(|scope| scope == &option)
                    .is_some()
            })
            .unwrap_or_default()
    }

    fn split_description<'a>(subject: &'a str) -> Result<(&'a str, &'a str)> {
        let split: Result<(_, (_, _)), nom::Err<nom::error::Error<&'a str>>> =
            pair(take_until1(":"), preceded(alt((tag(": "), tag(":"))), rest))(subject);

        split
            .map_err(nom::Err::<nom::error::Error<&str>>::to_owned)
            .map(|(_, scope_plus_and_description)| scope_plus_and_description)
            .into_diagnostic()
    }

    fn split_type_and_scope<'a>(scope_plus: &'a str) -> Result<(&str, Option<&'a str>)> {
        let split: (&str, (&str, Option<&str>)) = tuple((
            alt((take_until1("("), take_until1("!"), rest)),
            opt(alt((
                terminated(delimited(tag("("), take_until1(")"), tag(")")), tag("!")),
                delimited(tag("("), take_until1(")"), tag(")")),
            ))),
        ))(scope_plus)
        .map_err(nom::Err::<nom::error::Error<&str>>::to_owned)
        .into_diagnostic()?;
        Ok((split.1 .0, split.1 .1))
    }
}

impl TryFrom<CommitMessage<'_>> for ConventionalCommit {
    type Error = ErrReport;

    fn try_from(value: CommitMessage<'_>) -> Result<Self, Self::Error> {
        let commit_header = value.get_subject().to_string();
        let (scope_plus, description) = Self::split_description(&commit_header)?;
        let (type_slug, scope) = Self::split_type_and_scope(scope_plus)?;
        let breaking = value
            .get_trailers()
            .iter()
            .find(|x| x.get_key() == "BREAKING CHANGE")
            .map(Trailer::get_value)
            .map(|x| x.trim().to_string())
            .map_or(
                if scope_plus.ends_with('!') {
                    Change::BreakingWithoutMessage
                } else {
                    Change::Compatible
                },
                Change::BreakingWithMessage,
            );

        Ok(Self {
            subject: description.trim().to_string(),
            body: String::from(value.get_body()).trim().to_string(),
            breaking,
            scope: scope.map(str::trim).map(Into::into),
            type_slug: type_slug.trim().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_be_created_from_string() {
        assert_eq!(
            ConventionalCommit::try_from(CommitMessage::from("fix: example")).unwrap(),
            ConventionalCommit {
                type_slug: "fix".to_string(),
                subject: "example".to_string(),
                ..ConventionalCommit::default()
            }
        );
    }

    #[test]
    fn captures_the_body() {
        assert_eq!(
            ConventionalCommit::try_from(CommitMessage::from("fix: example\n\nDemonstration"))
                .unwrap(),
            ConventionalCommit {
                type_slug: "fix".to_string(),
                subject: "example".to_string(),
                body: "Demonstration".to_string(),
                ..ConventionalCommit::default()
            }
        );
    }

    #[test]
    fn can_capture_when_colon_is_next_to_subject() {
        assert_eq!(
            ConventionalCommit::try_from(CommitMessage::from("fix:example")).unwrap(),
            ConventionalCommit {
                type_slug: "fix".to_string(),
                subject: "example".to_string(),
                ..ConventionalCommit::default()
            }
        );
    }

    #[test]
    fn it_knows_when_something_is_a_bc_break() {
        assert_eq!(
            ConventionalCommit::try_from(CommitMessage::from("fix!: example")).unwrap(),
            ConventionalCommit {
                type_slug: "fix".to_string(),
                subject: "example".to_string(),
                breaking: Change::BreakingWithoutMessage,
                ..ConventionalCommit::default()
            }
        );
    }

    #[test]
    fn break_with_message_and_bang() {
        assert_eq!(
            ConventionalCommit::try_from(CommitMessage::from(
                "fix!: example\n\nBREAKING CHANGE: Some text"
            ))
            .unwrap(),
            ConventionalCommit {
                type_slug: "fix".to_string(),
                subject: "example".to_string(),
                breaking: Change::BreakingWithMessage("Some text".into()),
                ..ConventionalCommit::default()
            }
        );
    }

    #[test]
    fn break_with_message() {
        assert_eq!(
            ConventionalCommit::try_from(CommitMessage::from(
                "fix: example\n\nBREAKING CHANGE: Some text"
            ))
            .unwrap(),
            ConventionalCommit {
                type_slug: "fix".into(),
                subject: "example".to_string(),
                breaking: Change::BreakingWithMessage("Some text".into()),
                ..ConventionalCommit::default()
            }
        );
    }

    #[test]
    fn gets_scope() {
        assert_eq!(
            ConventionalCommit::try_from(CommitMessage::from("fix(something): example")).unwrap(),
            ConventionalCommit {
                type_slug: "fix".to_string(),
                subject: "example".to_string(),
                scope: Some("something".to_string()),
                ..ConventionalCommit::default()
            }
        );
    }

    #[test]
    fn can_get_scope_cursor() {
        assert_eq!(
            ConventionalCommit::try_from(CommitMessage::from("fix(something): example"))
                .unwrap()
                .scope_index(vec![
                    "some".to_string(),
                    "something".to_string(),
                    "somethingelse".to_string()
                ]),
            1_usize
        );
    }

    #[test]
    fn scope_cursor_is_0_not_set() {
        assert_eq!(
            ConventionalCommit::try_from(CommitMessage::from("fix: example"))
                .unwrap()
                .scope_index(vec!["some".to_string(),]),
            0_usize
        );
    }

    #[test]
    fn scope_cursor_is_0_when_not_found_in_config() {
        assert_eq!(
            ConventionalCommit::try_from(CommitMessage::from("fix(other): example"))
                .unwrap()
                .scope_index(vec!["some".to_string(),]),
            0_usize
        );
    }

    #[test]
    fn can_get_type_cursor() {
        assert_eq!(
            ConventionalCommit::try_from(CommitMessage::from("fix(something): example"))
                .unwrap()
                .type_index(vec![
                    "ci".to_string(),
                    "fix".to_string(),
                    "tests".to_string()
                ]),
            1_usize
        );
    }
    #[test]
    fn type_is_0_when_not_found() {
        assert_eq!(
            ConventionalCommit::try_from(CommitMessage::from("fix: example"))
                .unwrap()
                .type_index(vec![
                    "ci".to_string(),
                    "feat".to_string(),
                    "tests".to_string()
                ]),
            0_usize
        );
    }
}
