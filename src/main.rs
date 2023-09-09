mod connection_manager;
mod resp;
mod store;

#[macro_use(concat_string)]
extern crate concat_string;
use std::net::{SocketAddrV4, TcpListener};

use connection_manager::client_handler::handle_stream;
use resp::deserialize::deserialize;
use resp::serialize::serialize;
use store::db::Cache;

const ADDR: std::net::Ipv4Addr = std::net::Ipv4Addr::new(127, 0, 0, 1);
const PORT: u16 = 6379; //Redis PORT

fn main() {
    let listener = TcpListener::bind(SocketAddrV4::new(ADDR, PORT)).unwrap();
    let cache = Cache::new();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                handle_stream(stream, &cache)
            }
            Err(err) => println!("Connection failed due to {:?}", err),
        }
    }
}
