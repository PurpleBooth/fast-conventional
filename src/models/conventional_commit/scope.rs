#[derive(Clone, PartialOrd, PartialEq, Default, Debug)]
pub struct Scope(pub String);

impl From<String> for Scope {
    fn from(contents: String) -> Self {
        Self(contents)
    }
}

impl From<&str> for Scope {
    fn from(contents: &str) -> Self {
        Self(contents.into())
    }
}

impl From<Scope> for String {
    fn from(contents: Scope) -> Self {
        contents.0
    }
}

impl PartialEq<String> for Scope {
    fn eq(&self, other: &String) -> bool {
        &self.clone().0 == other
    }
}

impl PartialEq<Scope> for String {
    fn eq(&self, other: &Scope) -> bool {
        &other.0 == self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_be_compared_to_a_string() {
        assert!(*"Hello" == *"Hello",);
        assert!(*"Other" != *"Hello",);
    }

    #[test]
    fn can_be_created_from_str() {
        assert_eq!(Scope::from("Hello"), Scope("Hello".to_string()));
    }

    #[test]
    fn can_be_created_from_string() {
        assert_eq!(Scope::from("Hello".to_string()), Scope("Hello".to_string()));
    }

    #[test]
    fn can_create_string_from() {
        assert_eq!(
            String::from(Scope("Hello".to_string())),
            "Hello".to_string()
        );
    }
}
