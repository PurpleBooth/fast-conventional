use mit_commit::Bodies;

#[derive(Clone, PartialOrd, PartialEq, Eq, Default, Debug)]
pub struct Body(pub(crate) String);

impl Body {
    pub(crate) const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<String> for Body {
    fn from(contents: String) -> Self {
        Self(contents.trim().to_string())
    }
}

impl From<Option<String>> for Body {
    fn from(contents: Option<String>) -> Self {
        contents.map_or_else(|| Self::from(""), Self::from)
    }
}

impl From<&str> for Body {
    fn from(contents: &str) -> Self {
        Self::from(contents.to_string())
    }
}

impl<'a> From<&'a Body> for &'a str {
    fn from(contents: &'a Body) -> Self {
        &contents.0
    }
}

impl From<Bodies<'_>> for Body {
    fn from(contents: Bodies<'_>) -> Self {
        Self::from(contents.to_string())
    }
}

impl From<Body> for String {
    fn from(contents: Body) -> Self {
        contents.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_be_created_from_string() {
        assert_eq!(Body::from("Hello".to_string()), Body("Hello".to_string()));
    }

    #[test]
    fn it_can_tell_me_if_it_is_empty() {
        assert!(Body::from(String::new()).is_empty());
    }

    #[test]
    fn it_can_tell_me_if_it_is_not_empty() {
        assert!(!Body::from("Something".to_string()).is_empty());
    }

    #[test]
    fn can_be_created_from_str() {
        assert_eq!(Body::from("Hello"), Body("Hello".to_string()));
    }

    #[test]
    fn can_be_created_from_option_string() {
        assert_eq!(
            Body::from(Some("Hello".to_string())),
            Body("Hello".to_string())
        );
    }

    #[test]
    fn can_be_created_from_empty_option_string() {
        assert_eq!(Body::from(None), Body(String::new()));
    }

    #[test]
    fn can_be_created_from_commit_message_bodies() {
        let input: Vec<mit_commit::Body<'_>> = vec!["Hello".to_string().into()];
        assert_eq!(Body::from(Bodies::from(input)), Body("Hello".to_string()));
    }

    #[test]
    fn can_create_string_from() {
        assert_eq!(String::from(Body("Hello".to_string())), "Hello".to_string());
    }

    #[test]
    fn trailing_whitespace_is_trimmed() {
        assert_eq!(
            Body::from("       Hello         "),
            Body("Hello".to_string())
        );
    }
}
