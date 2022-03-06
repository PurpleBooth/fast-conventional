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

impl From<Option<String>> for Change {
    fn from(change: Option<String>) -> Self {
        match change {
            None => Self::Compatible,
            Some(text) if text.is_empty() => Self::BreakingWithoutMessage,
            Some(text) => Self::BreakingWithMessage(text),
        }
    }
}

impl From<String> for Change {
    fn from(value: String) -> Self {
        if value.is_empty() {
            Self::BreakingWithoutMessage
        } else {
            Self::BreakingWithMessage(value)
        }
    }
}

impl From<&str> for Change {
    fn from(value: &str) -> Self {
        if value.is_empty() {
            Self::BreakingWithoutMessage
        } else {
            Self::BreakingWithMessage(value.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_compatible() {
        assert_eq!(Change::default(), Change::Compatible);
    }

    #[test]
    fn from_option_string_with_message() {
        assert_eq!(
            Change::from(Some("Abc".to_string())),
            Change::BreakingWithMessage("Abc".into())
        );
    }

    #[test]
    fn from_option_string_with_out_message() {
        assert_eq!(
            Change::from(Some("".to_string())),
            Change::BreakingWithoutMessage
        );
    }

    #[test]
    fn from_option_string_with_none() {
        assert_eq!(Change::from(None), Change::Compatible);
    }

    #[test]
    fn from_string_with_message() {
        assert_eq!(
            Change::from("Abc".to_string()),
            Change::BreakingWithMessage("Abc".into())
        );
    }

    #[test]
    fn from_string_without_message() {
        assert_eq!(Change::from("".to_string()), Change::BreakingWithoutMessage);
    }
    #[test]
    fn from_str_with_message() {
        assert_eq!(
            Change::from("Abc"),
            Change::BreakingWithMessage("Abc".into())
        );
    }

    #[test]
    fn from_str_without_message() {
        assert_eq!(Change::from(""), Change::BreakingWithoutMessage);
    }
}
