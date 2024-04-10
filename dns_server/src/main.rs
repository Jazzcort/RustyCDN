mod utils;
mod dns_server;

use utils::parse_arguments;
use dns_server::DnsServer;

#[tokio::main]
async fn main() {
    // Get the port number and CDN server name from the command line arguments
    let matches = parse_arguments();
    let port = matches.get_one::<String>("port").unwrap();
    let cdn = matches.get_one::<String>("cdn").unwrap();

    // Get the DNS server running
    let mut dns_server = DnsServer::new(&port);
    // Start the DNS server
    dns_server.start().await;
}
