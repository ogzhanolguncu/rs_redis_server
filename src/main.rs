mod connection_manager;
mod resp;
mod store;

use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};
use std::sync::Arc;
use std::thread;

use connection_manager::client_handler::handle_stream;
use resp::deserialize::deserialize;
use store::db::Cache;

#[macro_use(concat_string)]
extern crate concat_string;

const ADDR: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const PORT: u16 = 6379; //Redis PORT

fn main() {
    let listener = TcpListener::bind(SocketAddrV4::new(ADDR, PORT)).unwrap_or_else(|err| {
        println!("Failed to bind to address: {}", err);
        std::process::exit(1);
    });
    let cache = Arc::new(Cache::new());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let cache_clone = cache.clone();
                thread::spawn(move || handle_stream(stream, &cache_clone));
            }
            Err(err) => println!("Connection failed due to {:?}", err),
        }
    }
}
