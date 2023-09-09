use std::{
    io::{Read, Write},
    net::{Shutdown, TcpStream},
};

use super::command_handler::handle_command;

pub fn handle_stream(mut stream: TcpStream) {
    let mut data = [0_u8; 60];
    loop {
        match stream.read(&mut data) {
            Ok(size) => {
                if size == 0 {
                    println!("Connection closed by {}", stream.peer_addr().unwrap());
                    break;
                } else {
                    let human_readable = String::from_utf8_lossy(&data[0..size]);
                    let serialized_response = handle_command(human_readable);
                    match stream.write(serialized_response.as_bytes()) {
                        Ok(written) => {
                            if written < serialized_response.len() {
                                println!(
                                    "Warning: Not all bytes written to {}: only {}/{} bytes written",
                                    stream.peer_addr().unwrap(),
                                    written,
                                    serialized_response.len()
                                );
                            }
                        }
                        Err(err) => {
                            println!(
                                "An error occurred while writing to {}: {}",
                                stream.peer_addr().unwrap(),
                                err
                            );
                        }
                    }
                }
            }
            Err(_) => {
                println!(
                    "An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                stream.shutdown(Shutdown::Both).unwrap();
                break;
            }
        }
    }
}
