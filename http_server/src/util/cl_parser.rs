use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, long_about = None)]

// Command line parser
pub struct Cli {
    /// Port number that this HTTP server is binded to
    #[arg(short, default_value_t = 80, value_parser = check_port)]
    pub port: u16,

    /// Origin domain/IP address where this server fetch the contents
    #[arg(short, default_value_t = {"cs5700cdnorigin.ccs.neu.edu".to_string()})]
    pub origin: String,
}

// This funtion is used to check if the given port number is valid.
fn check_port(s: &str) -> Result<u16, String> {
    let port: usize = s.parse().map_err(|_| format!("{s} isn't a port number "))?;

    if port <= 0 || port > u16::MAX as usize {
        Err(format!("port number is out of the range"))
    } else {
        Ok(port as u16)
    }
}
