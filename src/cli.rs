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
            Arg::new("commit-message-source")
                .about(
                    "The commit message, and can be: message (if a -m or -F option was given to \
                     git); template (if a -t option was given or the configuration option \
                     commit.template is set in git); merge (if the commit is a merge or a \
                     .git/MERGE_MSG file exists); squash (if a .git/SQUASH_MSG file exists); or \
                     commit",
                )
                .index(2)
                .required(false),
        )
        .arg(
            Arg::new("commit-sha")
                .about("Commit SHA-1 (if a -c, -C or --amend option was given to git).")
                .index(3)
                .required(false),
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
