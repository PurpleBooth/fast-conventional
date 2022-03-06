use clap::Command;

use std::io;

use clap_complete::{generate, Generator};

pub fn print_completions<G: Generator>(gen: G, command: &mut Command<'_>) {
    generate(
        gen,
        command,
        command.get_name().to_string(),
        &mut io::stdout(),
    );
}
