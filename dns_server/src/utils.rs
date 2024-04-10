use clap::{Arg, Command};

// This function is used to parse the command line arguments
pub fn parse_arguments() -> clap::ArgMatches {
    let matches = Command::new("DNS Server").
        arg(
            Arg::new("port")
                .short('p')
                .default_value("20310")
        )
        .arg(
            Arg::new("cdn")
                .short('n')
                .default_value("cs5700cdn.example.com")
        )
        .get_matches();

    matches
}