use clap::ArgMatches;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeSet;
use std::fs;

use miette::Diagnostic;
use thiserror::Error;

const ANGULAR_TYPES: [&str; 10] = [
    "feat", "fix", "docs", "style", "refactor", "perf", "test", "chore", "build", "ci",
];

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FastConventionalConfig {
    pub(crate) use_angular: Option<bool>,
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
    type Error = YamlParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(serde_yaml::from_str::<Self>(value)?)
    }
}

impl TryFrom<ArgMatches> for FastConventionalConfig {
    type Error = ConfigReadError;

    fn try_from(matches: ArgMatches) -> Result<Self, Self::Error> {
        let config = matches
            .value_of("config")
            .expect("No config parameter found");
        let contents = fs::read_to_string(config)?;

        Ok(contents.as_str().try_into()?)
    }
}

#[derive(Error, Debug, Diagnostic)]
#[error(transparent)]
#[diagnostic(code(models::fast_conventional_config::config_read_error), url(docsrs))]
pub enum ConfigReadError {
    Io(#[from] std::io::Error),
    Yaml(#[from] YamlParseError),
}

#[derive(Error, Debug, Diagnostic)]
#[error(transparent)]
#[diagnostic(code(models::fast_conventional_config::yaml_parse_error), url(docsrs))]
pub struct YamlParseError {
    #[from]
    inner: serde_yaml::Error,
}

#[cfg(test)]
mod tests {

    use std::io::Write;

    use super::*;
    use crate::cli::cli;

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

        let input = cli()
            .try_get_matches_from_mut(vec![
                "myapp",
                "--config",
                path.to_str().unwrap_or_default(),
                "unused",
            ])
            .unwrap();

        let actual: FastConventionalConfig = input.try_into().expect("Yaml unexpectedly invalid");

        assert_eq!(actual.get_types(), BTreeSet::from(["ci".to_string()]));
        assert_eq!(actual.get_scopes(), BTreeSet::new());
    }
}
