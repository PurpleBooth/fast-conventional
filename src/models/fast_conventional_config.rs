use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeSet;
use std::fs::File;
use std::path::PathBuf;

use miette::Diagnostic;
use thiserror::Error;

const ANGULAR_TYPES: [&str; 10] = [
    "feat", "fix", "docs", "style", "refactor", "perf", "test", "chore", "build", "ci",
];

#[non_exhaustive]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FastConventionalConfig {
    use_angular: Option<bool>,
    types: Option<Vec<String>>,
    scopes: Option<Vec<String>>,
}

impl FastConventionalConfig {
    pub(crate) fn get_scopes(&self) -> BTreeSet<String> {
        self.scopes
            .clone()
            .unwrap_or_default()
            .into_iter()
            .collect()
    }

    pub(crate) fn get_types(&self) -> BTreeSet<String> {
        let user_types: BTreeSet<String> =
            self.types.clone().unwrap_or_default().into_iter().collect();

        if self.use_angular == Some(true) {
            let angular_types: BTreeSet<String> = ANGULAR_TYPES
                .into_iter()
                .map(ToString::to_string)
                .collect::<_>();

            return angular_types
                .union(&user_types)
                .map(ToString::to_string)
                .collect::<_>();
        };

        user_types
    }
}

impl TryFrom<&str> for FastConventionalConfig {
    type Error = YamlReadError;

    fn try_from(filename: &str) -> Result<Self, Self::Error> {
        Ok(serde_yaml::from_str(filename)?)
    }
}

impl TryFrom<PathBuf> for FastConventionalConfig {
    type Error = YamlReadError;

    fn try_from(filename: PathBuf) -> Result<Self, Self::Error> {
        let file = File::open(filename)?;
        Ok(serde_yaml::from_reader(file)?)
    }
}

#[non_exhaustive]
#[derive(Error, Debug, Diagnostic)]
#[error(transparent)]
#[diagnostic(code(models::fast_conventional_config::config_read_error), url(docsrs))]
pub enum ConfigReadError {
    Io(#[from] std::io::Error),
    Yaml(#[from] YamlReadError),
}

#[non_exhaustive]
#[derive(Error, Debug, Diagnostic)]
#[error(transparent)]
#[diagnostic(code(models::fast_conventional_config::yaml_parse_error), url(docsrs))]
pub enum YamlReadError {
    Io(#[from] std::io::Error),
    Yaml(#[from] serde_yaml::Error),
}

#[cfg(test)]
mod tests {

    use std::io::Write;

    use super::*;

    #[test]
    fn can_be_created_from_string() {
        let input = r#"types: [ci]
scopes: ["mergify", "just", "github"]"#;

        let actual: FastConventionalConfig = input.try_into().expect("Yaml unexpectedly invalid");
        let expected_types = BTreeSet::from(["ci".to_string()]);
        let expected_scopes = BTreeSet::from([
            "mergify".to_string(),
            "just".to_string(),
            "github".to_string(),
        ]);

        assert_eq!(actual.get_types(), expected_types);
        assert_eq!(actual.get_scopes(), expected_scopes);
    }

    #[test]
    fn adds_angular_types_on_flag() {
        let input = r#"use_angular: true
types: [additional]
scopes: ["mergify", "just", "github"]"#;

        let actual: FastConventionalConfig = input.try_into().expect("Yaml unexpectedly invalid");
        let expected_types = [
            "feat",
            "fix",
            "docs",
            "style",
            "refactor",
            "perf",
            "test",
            "chore",
            "build",
            "ci",
            "additional",
        ]
        .into_iter()
        .map(String::from)
        .collect();
        let expected_scopes = ["mergify", "just", "github"]
            .into_iter()
            .map(String::from)
            .collect();

        assert_eq!(actual.get_types(), expected_types);
        assert_eq!(actual.get_scopes(), expected_scopes);
    }

    #[test]
    fn it_can_be_created_from_an_arg_matches() {
        let mut temp_file =
            tempfile::NamedTempFile::new().expect("failed to create temporary file");
        let path = temp_file.path().to_path_buf();

        write!(temp_file, r#"types: [ci]"#).expect("failed to write test config");

        let actual: FastConventionalConfig = path.try_into().expect("Yaml unexpectedly invalid");

        assert_eq!(actual.get_types(), BTreeSet::from(["ci".to_string()]));
        assert_eq!(actual.get_scopes(), BTreeSet::new());
    }
}
