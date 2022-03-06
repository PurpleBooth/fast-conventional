#[derive(Clone, PartialOrd, PartialEq, Default, Debug)]
pub struct Subject(pub(crate) String);

impl From<&str> for Subject {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for Subject {
    fn from(contents: String) -> Self {
        Self(contents)
    }
}

impl From<Subject> for String {
    fn from(contents: Subject) -> Self {
        contents.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_be_created_from_string() {
        assert_eq!(
            Subject::from("Hello".to_string()),
            Subject("Hello".to_string())
        );
    }

    #[test]
    fn can_be_created_from_str() {
        assert_eq!(Subject::from("Hello"), Subject("Hello".to_string()));
    }

    #[test]
    fn can_create_string_from() {
        assert_eq!(
            String::from(Subject("Hello".to_string())),
            "Hello".to_string()
        );
    }
}
