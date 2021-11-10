use clap::{crate_authors, crate_version, App, Arg};

pub fn app() -> App<'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .bin_name(String::from(env!("CARGO_PKG_NAME")))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("commit-message-path")
                .about("The name of the file that contains the commit log message")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .env("FAST_CONVENTIONAL_CONFIG")
                .default_value(".fastconventional.yaml")
                .about("Configuration file")
                .required(false),
        )
}
