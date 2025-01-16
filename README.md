# chess-rs

This project is a result of an implementation of chess in Rust. Overall the program can be split into two major parts. The **server** allows one client to connect and it logs all the moves that happened. The **client** connects to the server and it automatically transmits all the moves. 

## How to run everything

The server can be started by calling: cargo run --bin server --release 
The client can be started by calling: cargo run --bin client --release 

## Client

The client binary itself first constructs a window that is handled by egui. It also handles the rendering by itself. The window can be resized but the chess board itself will not stretch. The default window size is set for 1000 x 1000 pixels and the program may not launch or crash on resolutions below that (I personaly don't know how egui handles this). The connection to the server is handled by seperate thread and it waits for a change in the baord state that is signaled by a cvar. If I were to do this simply all in one thread the game would have been much easier and probably finished properly, but it would be way too easy and not really intresting. If the connection to the server can't be established the thread will panick; however, the game can still be played but it won't be recorded. 

## Server

The Server itself is rather simple. It will accept connections on port 12345 and will start to record any data that is transmitted. I though it would be smart to keep all tcp streams alive for the hole duration of the game but that may have been the factor that made it next to impossible for me to synchronise all the game data. In hindsight it would have been much smarter to just immediately reply with the next state and await a new connection. Once again using a single thread and using async would have been much easier but boring.  


## Game

This part describes how the game itself is built and how most of the rules are enforced. 

### Board

The board is responsible for loading all the pieces into their initial position. It is also responsible for updating the board when a move occurs. 

### Piece

The piece is the final component in the chess game. Pieces are spawned from the board. And the type of piece is determined by an enum called *PieceType*. A further enum is used for the colour. The corresponding images are included in the binary code since they are included with the *include_bytes!()* macro. The pieces calculate their valid moves individually by checking the state of the board. I won't go into much more detail here since it is incredibly complex. 

# Assets

Chess piece assets:

https://wildlifestudios.itch.io/chess-set-pixel-art

https://peterellisjones.com/posts/generating-legal-chess-moves-efficiently/

# What works

* All pieces have their valid moves
* Friendly pieces block moves
* The knight movement is correct
* Pawns promote to queens upon reaching the other side
* Moves that are valid are highlighted
* Opposing colours can capture pieces
* Pawn movement is 2 initial and then only one spot


# Unimplemented

## King related

* Find a way to get valid moves that don't leave the king in check
* Implement check & checkmate
* Implement castle

To implement more complex king mvoes it is required to generate a threat map to show which pieces are attacked by the opposing colour. 

## Pawn related

* ~~Implement pawn promotion to queen~~
* Implement en passant
* ~~Implement the correct attack pattern for pawns~~


# The game will not be finished since the complexity increases greatly ...

Calculating the legal king moves in the late game is incredibly difficult. The game will be handled like a board game and the player simply has to know when they won. Furthermore, the server will only act as a record for already played games.