use crate::cli;
use clap_complete::{generate, Shell};
use std::io;

use clap::CommandFactory;

pub fn run(shell: Shell) {
    let mut cmd = cli::Args::command();
    let bin_name = cmd.get_name().to_string();
    generate(shell, &mut cmd, bin_name, &mut io::stdout());
}
