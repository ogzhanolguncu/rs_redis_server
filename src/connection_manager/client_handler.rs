use std::{
    io::{Read, Write},
    net::{Shutdown, TcpStream},
};

use super::command_handler::handle_command;

pub fn handle_stream(mut stream: TcpStream) {
    let mut data = [0 as u8; 60];
    loop {
        match stream.read(&mut data) {
            Ok(size) => {
                if size == 0 {
                    println!("Connection closed by {}", stream.peer_addr().unwrap());
                    break;
                } else {
                    let human_readable = String::from_utf8_lossy(&data[0..size]);
                    let serialized_repsonse = handle_command(human_readable);
                    stream.write(serialized_repsonse.as_bytes()).unwrap();
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
