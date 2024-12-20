#![warn(clippy::all, rust_2018_idioms)]

use core::str;
use std::{io::{stdout, Read, Write}, net::IpAddr};

const IP_ADDR: &'static str = "127.0.0.1:12345";

pub struct Clients{
    player_1: Option<String>,
    player_2: Option<String>
}

fn main() -> std::io::Result<()> {

    let listener = std::net::TcpListener::bind(IP_ADDR)
        .expect(format!("Could not establish connection to {}", IP_ADDR).as_str()); 

    let mut clients = Clients{
        player_1: None,
        player_2: None,
    };

    for stream in listener.incoming() {

        let mut input_buffer: String = String::with_capacity(512);

        // #[cfg(feature = "dynamic_memory")]
        match stream {
            Ok(mut stream) => {   

                println!("Client connected: {}", stream.peer_addr().unwrap()); 

                match clients.player_1 {
                    Some(_) => clients.player_2 = Some(stream.peer_addr().unwrap().ip().to_string()), //If player 1 already exists player 2 can be initiliased
                    None => clients.player_1 = Some(stream.peer_addr().unwrap().ip().to_string()), //If player 1 does not exist needs to be initiliased
                }

                while let Ok(bytes_read) = stream.read_to_string(&mut input_buffer){
                    if bytes_read == 0 {//This signals there is no more data to be read
                        break;
                    }

                    println!("{}", input_buffer);

                }

            }
            Err(err_msg) => println!("Failed to accept connection from client. {}", err_msg),
        }

    }

    Ok(())
} // the stream is closed here


