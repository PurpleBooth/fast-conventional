use clap::{crate_authors, crate_version, Arg, Command};

pub fn cli() -> Command<'static> {
    Command::new(env!("CARGO_PKG_NAME"))
        .bin_name(String::from(env!("CARGO_PKG_NAME")))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("commit-message-path")
                .help("The name of the file that contains the commit log message")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .env("FAST_CONVENTIONAL_CONFIG")
                .default_value(".fastconventional.yaml")
                .help("Configuration file")
                .required(false),
        )
}
