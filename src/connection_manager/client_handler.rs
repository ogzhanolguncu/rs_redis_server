use std::{
    io::{Read, Write},
    net::{Shutdown, TcpStream},
};

use crate::store::db::Cache;

use super::command_handler::handle_command;

pub fn handle_stream(mut stream: TcpStream, cache: &Cache) {
    let mut data = [0_u8; 128];
    loop {
        match stream.read(&mut data) {
            Ok(size) => {
                if size == 0 {
                    match stream.peer_addr() {
                        Ok(addr) => println!("Connection closed by {}", addr),
                        Err(_) => println!("Connection closed but could not get peer address."),
                    }
                    break;
                } else {
                    let human_readable = String::from_utf8_lossy(&data);
                    let serialized_response = handle_command(human_readable, cache);
                    match stream.write_all(serialized_response.as_bytes()) {
                        Ok(_) => {}
                        Err(err) => {
                            match stream.peer_addr() {
                                Ok(addr) => println!("An error occurred while writing to {}: {}", addr, err),
                                Err(_) => println!("An error occurred while writing and could not get peer address: {}", err),
                            }
                        }
                    }
                }
            }
            Err(err) => {
                match stream.peer_addr() {
                    Ok(addr) => println!(
                        "An error occurred, terminating connection with {}: {}",
                        addr, err
                    ),
                    Err(_) => println!("An error occurred and could not get peer address: {}", err),
                }
                if let Err(err) = stream.shutdown(Shutdown::Both) {
                    println!("An error occurred while shutting down the stream: {}", err);
                }
                break;
            }
        }
    }
}
