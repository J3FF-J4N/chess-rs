use std::fs::File;
//     Ok(())
// } // the stream is closed here
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

use chrono::Local;

const IP_ADDR: &str = "127.0.0.1:12345";
const BUFF_SIZE: usize = 4096;

pub struct Clients {
    player_turn: Arc<(Mutex<u8>, Condvar)>, // 0 for Player 1, 1 for Player 2
}

fn handle_client(mut stream: TcpStream, clients: Arc<Clients>, player_id: u8) {
    let (_turn_mutex, turn_cvar) = &*clients.player_turn;

    let mut input_buffer = String::new();
    let mut buffer = [0; BUFF_SIZE];

    let mut file = File::create(format!("gamedata{}.txt", Local::now().format("%d_%m_%Y_%H_%M"))).unwrap();

    loop {

        // Read from the stream
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected.");
                write!(file, "{}\n", "------------------------------------").unwrap();
                break; // Connection closed
            }
            Ok(bytes_read) => {
                input_buffer.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));

                // Check for the end of the message
                while let Some(pos) = input_buffer.find('\n') {
                    let message = input_buffer[..pos].to_string();
                    input_buffer.drain(..=pos); // Remove the processed message from the buffer

                    // Process the message (e.g., send it to the other player)
                    println!("Player {}: received message", player_id + 1);

                    write!(file, "{}\n", message).unwrap();

                    turn_cvar.notify_all();
                }
            }
            Err(err) => {
                println!("Error reading from stream: {}", err);
                break; // Exit the loop on error
            }
        }

    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(IP_ADDR)?;
    let clients = Arc::new(Clients {
        player_turn: Arc::new((Mutex::new(0), Condvar::new())), // Start with Player 1's turn
        // shared_client_buffer: Arc::new(Mutex::new(String::with_capacity(BUFF_SIZE))),
    });

    for (player_id, stream) in listener.incoming().enumerate() {
        match stream {
            Ok(stream) => {
                let clients_clone = Arc::clone(&clients);
                thread::spawn(move || {
                    handle_client(stream, clients_clone, player_id as u8);
                });
            }
            Err(err) => println!("Failed to accept connection: {}", err),
        }
    }

    Ok(())
}

