use clap::{Arg, ArgMatches, Command};

pub fn get_args() -> ArgMatches {
    Command::new("rid3")
        .version("0.1")
        .arg(Arg::new("path"))
        .get_matches()
}
