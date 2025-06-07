use std::fmt::{Display, Formatter};

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Clone)]
pub struct ShortRef(String);

impl From<String> for ShortRef {
    fn from(contents: String) -> Self {
        Self(contents)
    }
}

impl From<&str> for ShortRef {
    fn from(contents: &str) -> Self {
        Self(contents.to_string())
    }
}

impl Display for ShortRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_be_created_from_str() {
        assert_eq!(
            ShortRef::from("deadbeef".to_string()),
            ShortRef("deadbeef".to_string())
        );
    }

    #[test]
    fn can_be_displayed() {
        assert_eq!(
            format!("{}", ShortRef::from("deadbeef".to_string())),
            "deadbeef".to_string()
        );
    }
}
