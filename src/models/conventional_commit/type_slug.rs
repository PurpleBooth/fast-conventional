#[derive(Clone, PartialOrd, PartialEq, Default, Debug)]
pub struct TypeSlug(String);

impl From<String> for TypeSlug {
    fn from(contents: String) -> Self {
        Self(contents)
    }
}

impl From<&str> for TypeSlug {
    fn from(contents: &str) -> Self {
        Self(contents.to_string())
    }
}

impl From<TypeSlug> for String {
    fn from(contents: TypeSlug) -> Self {
        contents.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_be_created_from_str() {
        assert_eq!(TypeSlug::from("Hello"), TypeSlug("Hello".to_string()));
    }

    #[test]
    fn can_be_created_from_string() {
        assert_eq!(
            TypeSlug::from("Hello".to_string()),
            TypeSlug("Hello".to_string())
        );
    }
    #[test]
    fn can_create_string_from() {
        assert_eq!(
            String::from(TypeSlug("Hello".to_string())),
            "Hello".to_string()
        );
    }
}
