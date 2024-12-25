// #![warn(clippy::all, rust_2018_idioms)]

// use core::str;
// use std::{
//     io::{stdout, Read, Write},
//     net::IpAddr,
//     ops::Deref,
//     sync::{Arc, Condvar, Mutex},
//     thread,
// };

// const IP_ADDR: &'static str = "127.0.0.1:12345";
// const BUFF_SIZE: usize = 1024;

// pub struct Clients {
//     player_1_new_data: Option<Arc<(Mutex<bool>, Condvar, u8)>>,
//     player_2_new_data: Option<Arc<(Mutex<bool>, Condvar, u8)>>,
// }

// fn main() -> std::io::Result<()> {
//     let listener = std::net::TcpListener::bind(IP_ADDR)
//         .expect(format!("Could not establish connection to {}", IP_ADDR).as_str());

//     let mut clients = Clients {
//         player_1_new_data: None,
//         player_2_new_data: None,
//     };

//     // Arc::new((Mutex::new(false), Condvar::new()))
//     let mut connected_player: i32 = 0;
//     let shared_thread_buffer = Arc::new(Mutex::new(String::with_capacity(512)));

//     let player_condition = Arc::new((Mutex::new(false), Condvar::new()));

//     for stream in listener.incoming() {

//         connected_player = connected_player.saturating_add(1);

//         let mut input_buffer: String = String::with_capacity(BUFF_SIZE);

//         // #[cfg(feature = "dynamic_memory")]

//         match stream {
//             Ok(mut stream) => {
//                 let buffer_clone = Arc::clone(&shared_thread_buffer);
//                 let player_condition_clone = Arc::clone(&player_condition);

//                 // connected_player = connected_player.saturating_add(1);

//                 thread::spawn(move || {
//                     println!("Client connected: {}", stream.peer_addr().unwrap());

//                     // match clients.player_1 {
//                     //     Some(_) => {
//                     //         clients.player_2 = Some(stream.peer_addr().unwrap().ip().to_string())
//                     //     } //If player 1 already exists player 2 can be initiliased
//                     //     None => clients.player_1 = Some(stream.peer_addr().unwrap().ip().to_string()), //If player 1 does not exist needs to be initiliased
//                     // }

//                     let mut input_buffer: String = String::new();

//                     let (send, cvar) = &*player_condition_clone.deref();
//                     loop {

//                         if connected_player == 2 {
//                             //Player 2 waits for player 1's move before beginning to read data
//                             println!("Player 2 start");
//                             let mut send_message = send.lock().unwrap();

//                             while !*send_message {
//                                 send_message = cvar.wait(send_message).unwrap(); // player_condition_clone.1.wait(send_message).unwrap();
//                             }

//                             println!("Player 2 end");

//                             //Write to player 2's stream

//                             // player_condition_clone.1.wait(send_message).unwrap();*player_condition_clone.0.lock().unwrap() = false;
//                             *send.lock().unwrap() = false;

//                         }

//                         loop {

//                             let mut buffer: [u8; BUFF_SIZE] = [0; BUFF_SIZE];

//                             match stream.read(&mut buffer) {
//                                 // Ok(0) => {*send.lock().unwrap() = true; cvar.notify_all(); println!("{}", buffer_clone.lock().unwrap());  break}, // Connection closed
//                                 Ok(bytes_read) => {
//                                     input_buffer
//                                         .push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));
//                                     while let Some(pos) = input_buffer.find('\n') {
//                                         buffer_clone
//                                             .lock()
//                                             .unwrap()
//                                             .push_str(&input_buffer[..pos].to_string());

//                                         // println!("{}", input_buffer);

//                                         // let message = input_buffer[..pos].to_string();
//                                         input_buffer.clear();
//                                         // input_buffer = input_buffer[pos + 1..].to_string(); // Remove the processed message
//                                         // println!("{}", message);
//                                         println!(
//                                             "---------------------------------------------------"
//                                         );
//                                         println!("Player {connected_player}");
//                                     }

//                                 }
//                                 Err(err) => {
//                                     println!("Error reading from stream: {}", err);
//                                     break;
//                                 }
//                             }

//                         }

//                             //Once this point is reached the other thread is notified that there is data available
//                             if connected_player == 1 {
//                                 println!("Player 1 start");
//                                 //Player 1 waits for player 2's move after reading data
//                                 let mut send_message = send.lock().unwrap();

//                                 while !*send_message {
//                                     send_message = cvar.wait(send_message).unwrap(); //player_condition_clone.1.wait(send_message).unwrap();
//                                 }
//                                 //Write to player 1's stream
//                                 println!("Player 1 end");

//                                 *send.lock().unwrap() = false;
//                             }

//                     }
//                 });
//             }
//             Err(err_msg) => println!("Failed to accept connection from client. {}", err_msg),
//         }

//     }

//     Ok(())
// } // the stream is closed here
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

const IP_ADDR: &str = "127.0.0.1:12345";
const BUFF_SIZE: usize = 4096;

pub struct Clients {
    player_turn: Arc<(Mutex<u8>, Condvar)>, // 0 for Player 1, 1 for Player 2
    shared_client_buffer: Arc<Mutex<String>>,
}

// fn handle_client(mut stream: TcpStream, clients: Arc<Clients>, player_id: u8) {

//     let (turn_mutex, turn_cvar) = &*clients.player_turn;

//     loop {
//         let mut buffer = [0; BUFF_SIZE];
//         match stream.read(&mut buffer) {
//             Ok(0) => {
//                 println!("Client disconnected.");
//                 break; // Connection closed
//             }
//             Ok(bytes_read) => {
//                 let message = String::from_utf8_lossy(&buffer[..bytes_read]);
//                 println!("Player {}: {}", player_id + 1, message);

//                 // Signal the other player
//                 let mut turn = turn_mutex.lock().unwrap();
//                 *turn = 1 - player_id; // Switch turn
//                 turn_cvar.notify_all(); // Notify the other player
//             }
//             Err(err) => {
//                 println!("Error reading from stream: {}", err);
//                 break;
//             }
//         }

//         // Wait for the turn to switch
//         let mut turn = turn_mutex.lock().unwrap();
//         while *turn == player_id {
//             turn = turn_cvar.wait(turn).unwrap();
//         }
//     }
// }

fn handle_client(mut stream: TcpStream, clients: Arc<Clients>, player_id: u8) {
    let (turn_mutex, turn_cvar) = &*clients.player_turn;

    let mut input_buffer = String::new();
    let mut buffer = [0; BUFF_SIZE];

    loop {
        // Wait for the player's turn
        let mut turn = turn_mutex.lock().unwrap();
        while *turn != player_id {
            turn = turn_cvar.wait(turn).unwrap(); // Wait until it's this player's turn
        }

        //The thread which is not the current player sends the data to indicate the changes to the other player

        // if 1 - *turn == player_id {

        // }

                //When a play is release from the turn lock the other players thread has received data that needs to be synchronised


        // Read from the stream
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected.");
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

                    *clients.shared_client_buffer.lock().unwrap() = message;

                    // Here you would send the message to the other player
                    // send_message_to_other_player(message, player_id);
                    // println!("Player {}: sending message", player_id + 1);
                    // write!(stream, "{}\n", clients.shared_client_buffer.lock().unwrap()).unwrap();
                    // stream.flush().unwrap();
                    // println!("Player {}: sent message", player_id + 1);
                    turn_cvar.notify_all();
                }
            }
            Err(err) => {
                println!("Error reading from stream: {}", err);
                break; // Exit the loop on error
            }
        }

        // After processing the message, switch turns
        *turn = 1 - player_id; // Switch to the other player
        turn_cvar.notify_all(); // Notify the other player that it's their turn


        // The player who just received the message should send the updated state
        if *turn == 1 - player_id { // Check if it's the other player's turn
            println!("Player {}: sending updated state", 1 - player_id + 1);
            write!(stream, "{}\n", clients.shared_client_buffer.lock().unwrap()).unwrap();
            stream.flush().unwrap();
            println!("Player {}: sent updated state: {}", 1 - player_id + 1, clients.shared_client_buffer.lock().unwrap());
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(IP_ADDR)?;
    let clients = Arc::new(Clients {
        player_turn: Arc::new((Mutex::new(0), Condvar::new())), // Start with Player 1's turn
        shared_client_buffer: Arc::new(Mutex::new(String::with_capacity(BUFF_SIZE))),
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
