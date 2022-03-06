use clap::{Parser, Subcommand};
use clap_complete::Shell;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
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
}
