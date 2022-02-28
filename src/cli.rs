use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    /// The name of the file that contains the commit log message
    #[clap()]
    pub commit_message_path: PathBuf,
    /// Configuration file
    #[clap(
        short = 'c',
        long = "config",
        env = "FAST_CONVENTIONAL_CONFIG",
        default_value = ".fastconventional.yaml"
    )]
    pub config: PathBuf,
}
