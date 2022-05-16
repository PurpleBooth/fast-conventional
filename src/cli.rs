use crate::models::GitRevisionSelection;
use clap::{Parser, Subcommand};
use clap_complete::Shell;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate completion for shell
    Completion {
        // The shell to generate for
        #[clap(arg_enum)]
        shell: Shell,
    },
    /// Edit a commit message
    Editor {
        /// The name of the file that contains the commit log message
        #[clap()]
        commit_message_path: PathBuf,
        /// Configuration file
        #[clap(
            short = 'c',
            long = "config",
            env = "FAST_CONVENTIONAL_CONFIG",
            default_value = ".fastconventional.yaml"
        )]
        config: PathBuf,
    },
    /// Validate a commit message is conventional
    Validate {
        /// An optional range to limit the linting
        #[clap()]
        revision_or_range: Option<GitRevisionSelection>,
        /// Git repository to search in
        #[clap(
            short = 'r',
            long = "repository",
            env = "FAST_CONVENTIONAL_GIT_REPOSITORY",
            default_value = "."
        )]
        repository: PathBuf,
    },
    /// Print an example configuration
    ExampleConfig,
}

impl Commands {}
