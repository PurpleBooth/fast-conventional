use super::body::Body;
use super::change::Change;
use super::scope::Scope;
use super::subject::Subject;
use super::type_slug::TypeSlug;
use miette::{ErrReport, IntoDiagnostic, Result};
use mit_commit::CommitMessage;
use mit_commit::Subject as CommitSubject;
use mit_commit::Trailer;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until1;
use nom::combinator::{opt, rest};
use nom::sequence::{delimited, pair, preceded, terminated, tuple};

#[derive(Clone, PartialOrd, PartialEq, Default, Debug)]
pub struct Commit {
    pub(crate) subject: Subject,
    pub(crate) body: Body,
    pub(crate) breaking: Change,
    pub(crate) type_slug: TypeSlug,
    pub(crate) scope: Option<Scope>,
}

impl Commit {
    pub fn type_index(&self, option: Vec<String>) -> usize {
        option
            .into_iter()
            .position(|option| self.type_slug == option.into())
            .unwrap_or_default()
    }

    pub fn scope_index(&self, option: Vec<String>) -> usize {
        self.scope.as_ref().map_or(0, |scope| {
            option
                .into_iter()
                .position(|option| scope.0 == option)
                .unwrap_or_default()
        })
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
        let result = tuple((
            alt((take_until1("("), take_until1("!"), rest)),
            opt(alt((
                terminated(delimited(tag("("), take_until1(")"), tag(")")), tag("!")),
                delimited(tag("("), take_until1(")"), tag(")")),
            ))),
        ))(scope_plus)
        .map_err(nom::Err::<nom::error::Error<&str>>::to_owned)
        .into_diagnostic();
        let split: (&str, (&str, Option<&str>)) = result?;
        Ok((split.1 .0, split.1 .1))
    }
}

impl From<Commit> for CommitMessage<'_> {
    fn from(conventional_commit: Commit) -> Self {
        let commit = CommitMessage::default();
        let mut subject_buffer: String = conventional_commit.type_slug.into();

        if let Some(Scope(selected_scope)) = conventional_commit.scope {
            subject_buffer.push('(');
            subject_buffer.push_str(&selected_scope);
            subject_buffer.push(')');
        }

        if match conventional_commit.breaking {
            Change::BreakingWithMessage(_) | Change::BreakingWithoutMessage => true,
            Change::Compatible => false,
        } {
            subject_buffer.push('!');
        }

        subject_buffer.push_str(": ");
        let subject = String::from(conventional_commit.subject);
        subject_buffer.push_str(&subject);

        let mut commit = commit.with_subject(subject_buffer.into());

        if !conventional_commit.body.is_empty() {
            let existing_subject: CommitSubject<'_> = commit.get_subject();
            let body = format!("Unused\n\n{}", conventional_commit.body.0);
            let edited_commit = CommitMessage::from(body);

            commit = edited_commit.with_subject(existing_subject);
        }

        if let Change::BreakingWithMessage(message) = conventional_commit.breaking {
            commit = commit.add_trailer(Trailer::new("BREAKING CHANGE".into(), message.into()));
        }

        commit
    }
}

impl TryFrom<CommitMessage<'_>> for Commit {
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
            subject: description.into(),
            body: value.get_body().into(),
            breaking,
            scope: scope.map(Into::into),
            type_slug: type_slug.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mit_commit::CommitMessage;

    #[test]
    fn can_be_created_from_string() {
        assert_eq!(
            Commit::try_from(CommitMessage::from("fix: example")).unwrap(),
            Commit {
                type_slug: "fix".into(),
                subject: "example".into(),
                ..Commit::default()
            }
        );
    }

    #[test]
    fn captures_the_body() {
        assert_eq!(
            Commit::try_from(CommitMessage::from("fix: example\n\nDemonstration")).unwrap(),
            Commit {
                type_slug: "fix".into(),
                subject: "example".into(),
                body: "Demonstration".into(),
                ..Commit::default()
            }
        );
    }

    #[test]
    fn can_capture_when_colon_is_next_to_subject() {
        assert_eq!(
            Commit::try_from(CommitMessage::from("fix:example")).unwrap(),
            Commit {
                type_slug: "fix".into(),
                subject: "example".into(),
                ..Commit::default()
            }
        );
    }

    #[test]
    fn it_knows_when_something_is_a_bc_break() {
        assert_eq!(
            Commit::try_from(CommitMessage::from("fix!: example")).unwrap(),
            Commit {
                type_slug: "fix".into(),
                subject: "example".into(),
                breaking: Change::BreakingWithoutMessage,
                ..Commit::default()
            }
        );
    }

    #[test]
    fn break_with_message_and_bang() {
        assert_eq!(
            Commit::try_from(CommitMessage::from(
                "fix!: example\n\nBREAKING CHANGE: Some text"
            ))
            .unwrap(),
            Commit {
                type_slug: "fix".into(),
                subject: "example".into(),
                breaking: "Some text".into(),
                ..Commit::default()
            }
        );
    }

    #[test]
    fn break_with_message() {
        assert_eq!(
            Commit::try_from(CommitMessage::from(
                "fix: example\n\nBREAKING CHANGE: Some text"
            ))
            .unwrap(),
            Commit {
                type_slug: "fix".into(),
                subject: "example".into(),
                breaking: "Some text".into(),
                ..Commit::default()
            }
        );
    }

    #[test]
    fn gets_scope() {
        assert_eq!(
            Commit::try_from(CommitMessage::from("fix(something): example")).unwrap(),
            Commit {
                type_slug: "fix".into(),
                subject: "example".into(),
                scope: Some("something".into()),
                ..Commit::default()
            }
        );
    }

    #[test]
    fn can_get_scope_cursor() {
        assert_eq!(
            Commit::try_from(CommitMessage::from("fix(something): example"))
                .unwrap()
                .scope_index(vec![
                    "some".into(),
                    "something".into(),
                    "somethingelse".into()
                ]),
            1_usize
        );
    }

    #[test]
    fn scope_cursor_is_0_not_set() {
        assert_eq!(
            Commit::try_from(CommitMessage::from("fix: example"))
                .unwrap()
                .scope_index(vec!["some".into(),]),
            0_usize
        );
    }

    #[test]
    fn scope_cursor_is_0_when_not_found_in_config() {
        assert_eq!(
            Commit::try_from(CommitMessage::from("fix(other): example"))
                .unwrap()
                .scope_index(vec!["some".into(),]),
            0_usize
        );
    }

    #[test]
    fn can_get_type_cursor() {
        assert_eq!(
            Commit::try_from(CommitMessage::from("fix(something): example"))
                .unwrap()
                .type_index(vec!["ci".into(), "fix".into(), "tests".into()]),
            1_usize
        );
    }
    #[test]
    fn type_is_0_when_not_found() {
        assert_eq!(
            Commit::try_from(CommitMessage::from("fix: example"))
                .unwrap()
                .type_index(vec!["ci".into(), "feat".into(), "tests".into()]),
            0_usize
        );
    }

    #[test]
    fn convert_to_commit_message_simple() {
        assert_eq!(
            CommitMessage::from(Commit::try_from(CommitMessage::from("fix: example")).unwrap()),
            CommitMessage::default().with_subject("fix: example".into())
        );
    }

    #[test]
    fn convert_to_commit_message_breaking_no_contents() {
        assert_eq!(
            CommitMessage::from(Commit::try_from(CommitMessage::from("fix!: example")).unwrap()),
            CommitMessage::default().with_subject("fix!: example".into())
        );
    }

    #[test]
    fn convert_to_commit_message_breaking_with_tag_contents() {
        assert_eq!(
            CommitMessage::from(
                Commit::try_from(CommitMessage::from(
                    "fix: example\n\nBREAKING CHANGE: Something that changed"
                ))
                .unwrap()
            ),
            CommitMessage::default()
                .with_subject("fix!: example".into())
                .add_trailer(Trailer::new(
                    "BREAKING CHANGE".into(),
                    "Something that changed".into()
                ))
        );
    }

    #[test]
    fn convert_to_commit_message_scope() {
        assert_eq!(
            CommitMessage::from(
                Commit::try_from(CommitMessage::from("fix(example): subject")).unwrap()
            ),
            CommitMessage::default().with_subject("fix(example): subject".into())
        );
    }
    #[test]
    fn convert_to_commit_message_scope_breaking_with_message() {
        assert_eq!(
            CommitMessage::from(
                Commit::try_from(CommitMessage::from(
                    "fix(example): subject\n\nBREAKING CHANGE: Something that changed"
                ))
                .unwrap()
            ),
            CommitMessage::default()
                .with_subject("fix(example)!: subject".into())
                .add_trailer(Trailer::new(
                    "BREAKING CHANGE".into(),
                    "Something that changed".into()
                ))
        );
    }

    #[test]
    fn convert_to_commit_message_scope_breaking_without_message() {
        assert_eq!(
            CommitMessage::from(
                Commit::try_from(CommitMessage::from("fix(example)!: subject")).unwrap()
            ),
            CommitMessage::default().with_subject("fix(example)!: subject".into())
        );
    }
}
