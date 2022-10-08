use std::str::FromStr;

use miette::Diagnostic;
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RevisionSelection(String);

impl RevisionSelection {
    pub fn is_single_commit(&self) -> bool {
        !self.0.contains("..")
    }
}

impl FromStr for RevisionSelection {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl From<RevisionSelection> for String {
    fn from(r: RevisionSelection) -> Self {
        r.0
    }
}

#[non_exhaustive]
#[derive(Error, Debug, Diagnostic, Copy, Clone)]
#[error("This does not look like a valid git revision or range")]
#[diagnostic(
    code(models::git_access::revision_or_range::revision_or_range_parse_error),
    url("https://git-scm.com/book/en/v2/Git-Tools-Revision-Selection")
)]
pub struct ParseError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_be_created_from_str() {
        assert_eq!(
            RevisionSelection::from_str("HEAD").unwrap(),
            RevisionSelection("HEAD".to_string())
        );
    }

    #[test]
    fn can_be_converted_to_a_str() {
        let selection = RevisionSelection::from_str("HEAD").unwrap();
        let actual: String = selection.into();
        assert_eq!(&actual, "HEAD");
    }

    #[test]
    fn can_tell_me_it_is_a_single_commit() {
        let selection = RevisionSelection::from_str("HEAD").unwrap();
        assert!(&selection.is_single_commit());
    }

    #[test]
    fn can_tell_me_it_is_a_range_of_commits() {
        let selection = RevisionSelection::from_str("..HEAD").unwrap();
        assert!(!&selection.is_single_commit());
    }
}
