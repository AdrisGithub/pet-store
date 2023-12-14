use std::io::Write;
use std::net::SocketAddr;
use std::str::FromStr;

use wbsl::server::Server;
use whdp::resp_presets::ok;

fn main() {
    let server = Server::try_from(SocketAddr::from_str("0.0.0.0:6969").unwrap());

    if server.is_ok() {
        let server = server.unwrap();
        for mut request in server {
           let _ =  request.1.write_all(ok("IT Works...".to_string()).to_string().as_bytes());
        }
    } else {
        println!("{}",server.err().unwrap())
    }
}

