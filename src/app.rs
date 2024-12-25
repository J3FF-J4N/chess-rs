use std::{
    io::Write,
    net::TcpStream,
    ops::Deref,
    sync::{Arc, Condvar, Mutex},
    thread::{self},
};

use crate::{board::Board, piece::Piece};

// #[serde(default)]
pub struct Game {
    // server_coneection: TcpStream,
    send_condition: Arc<(Mutex<bool>, Condvar)>,
    board: Arc<Mutex<Board>>,
    currently_moving: Option<Piece>,
    moves: Vec<(u8, u8)>, //By giving this vector to the struct I can avoid a syscall every loop that would allocate memory
    // update_board: Arc<Mutex<bool>>,
    state_changed: bool,
}

const IP_ADDR: &'static str = "127.0.0.1:12345";
// const BUFF_SIZE: usize = 4096;

impl Game {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        let board = Arc::new(Mutex::new(Board::init_board_state()));

        let board_clone = Arc::clone(&board);

        let send_condition = Arc::new((Mutex::new(false), Condvar::new()));

        let send_condition_clone = Arc::clone(&send_condition);

        // let update_board = Arc::new(Mutex::new(false));
        // let update_board_clone = Arc::clone(&update_board);

        thread::spawn(move || {
            let mut server = TcpStream::connect(IP_ADDR).unwrap();

            let (send, cvar) = &*send_condition_clone;

            loop {
                let mut send_message = send.lock().unwrap();

                while !*send_message {
                    send_message = cvar.wait(send_message).unwrap();
                }

                let message_out = format!("{}\n", ron::ser::to_string(&board_clone.deref()).unwrap());
                // let message = ron::ser::to_string_pretty(, PrettyConfig::default().).unwrap();

                write!(server, "{}", message_out).unwrap();

                *send_message = false;

                server.flush().unwrap();

            }
        });

        Self {
            // server_coneection: TcpStream::connect(IP_ADDR).unwrap(),
            send_condition: send_condition,
            board: board,
            currently_moving: None,
            moves: Vec::with_capacity(64), //64 is the max amount of possible moves
            // update_board: Arc::new(Mutex::new(false)),
            state_changed: false
        }

        // Default::default()
    }
}

const BOARD_COL: f32 = 8.0;

impl eframe::App for Game {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::TopDown),
                |ui| {
                    let width = ui.available_width() / BOARD_COL as f32 * 0.95;
                    let height = ui.available_height() / BOARD_COL as f32* 0.95;

                    let cell_size = egui::vec2(
                        width.min(height),
                        height.min(width), //Ensuring the aspect ratio is always maintained
                    ); //Scaling the grid to be 10% less than max to avoid clipping

                    if self.state_changed { //Update the board only when there was a change in the boardstate

                        let board_updated = self.board.lock().unwrap().update_board(&mut self.currently_moving);

                        if board_updated {//When the board was updated it's the other players turn

                            self.currently_moving = None;
                            let player = self.board.lock().unwrap().current_player.clone();
                            match player {
                                crate::piece::Colour::White => self.board.lock().unwrap().current_player = crate::piece::Colour::Black,
                                crate::piece::Colour::Black => self.board.lock().unwrap().current_player = crate::piece::Colour::White,
                            }
                            println!("State changed");
                        }
                        self.state_changed = false;
                        
                        *self.send_condition.0.lock().unwrap() = true;

                        self.send_condition.1.notify_all();
                        println!("Notifying");

                    }


                    egui::Grid::new("board")
                        .num_columns(BOARD_COL as usize)
                        .show(ui, |ui| {
                            
                            self.moves.clear();
                            
                            //TODO Filter moves that are blocked by other pieces
                            if let Some(moving_piece) = &self.currently_moving {
                                // moves.append(&mut moving_piece.get_valid_moves());
                                
                                self.moves.append(&mut self.board.lock().unwrap().get_valid_moves(&moving_piece));
                            }

                            self.board.lock().unwrap()
                                .grid
                                .iter_mut()
                                .enumerate()
                                .for_each(|(row_idx, row)| {
                                    row.iter_mut().enumerate().for_each(|(col_idx, element)| {

                                        let highlight =
                                            self.moves.contains(&(col_idx as u8, row_idx as u8));

                                        if let Some(element) = element {


                                            /*When there is a piece present in the path of the moving piece the position from the current piece is removed as available movement spot.
                                            When the piece is of the opposing colour the field is highligted as possible capture point.*/
                                            // if highlight == true { //Only if the highlight is true does this code need to be executed

                                            //     // moves.iter().for_each(|to_filter|{

                                            //     //     if highlight == true { //If the highlight is already false there is no need to continue checking for equality
                                            //     //         highlight = !element.pos_x.eq(&to_filter.0) || !element.pos_y.eq(&to_filter.1); //If either of the coordinates differ the highlight is kept
                                            //     //     }
                                        
                                                    
                                            //     // });

                                            // }


                                            let image = element.get_image().sense(egui::Sense {
                                                click: true,
                                                drag: true,
                                                focusable: true,
                                            });

                                            

                                            let piece_response = ui.add_sized(cell_size, image);

                                            /*This code will always mean that a piece is currently selected. When a piece is selected and another piece is clicked
                                            it should capture the piece. The validity of the move is already verified, it must only be verified if the move is in the 
                                            list of valid moves. */
                                            if piece_response.clicked() { //Select a piece for movement

                                                //If a piece is already selcted for movement it means the clicked piece was captured
                                                if self.moves.contains(&(col_idx as u8, row_idx as u8)) && self.currently_moving.is_some() {//When a piece is moving set the destination when it is valid
                                                    self.currently_moving.as_mut().unwrap().move_piece(col_idx as u8, row_idx as u8);
                                                    // #[cfg(debug_assertions)]
                                                    // println!("Moving {} to {} {}", self.currently_moving.as_ref().unwrap().get_name(), col_idx, row_idx);

                                                    self.state_changed = true;
                                                    //TODO Add the currently selected element to a list of already captured elements
                                                } else { //If this code is reached it means either no piece is currently selected or the move was not valid

                                                    element.is_moving = !element.is_moving; //This allows the play to deselect a chosen piece

                                                    //TODO Find a correct way to capture a piece
    
                                                    if element.is_moving == true {
                                                        self.currently_moving = Some(element.to_owned())
                                                    } else {
                                                        self.currently_moving = None;
                                                    }

                                                }

                                            }


                                            #[cfg(debug_assertions)]
                                            if piece_response.secondary_clicked() {
                                                println!("{:#?}", element);
                                            }

                                            if highlight == true {//Highlight fields valid for movement
                                                ui.painter().rect_stroke(
                                                    piece_response.rect,
                                                    0.0,
                                                    egui::Stroke::new(3.0, egui::Color32::GOLD),
                                                );
                                            } else {
                                                ui.painter().rect_stroke(
                                                    piece_response.rect,
                                                    0.0,
                                                    egui::Stroke::new(1.0, egui::Color32::WHITE),
                                                );
                                            }
                                         } else {                                            
                                            
                                            let (rect, field_response) = ui.allocate_exact_size(
                                                cell_size,
                                                egui::Sense {
                                                    click: true,
                                                    drag: true,
                                                    focusable: true,
                                                },
                                            );

                                            if field_response.clicked() { 

                                                if self.moves.contains(&(col_idx as u8, row_idx as u8)) && self.currently_moving.is_some() {//When a piece is moving set the destination when it is valid
                                                    self.currently_moving.as_mut().unwrap().move_piece(col_idx as u8, row_idx as u8);
                                                    #[cfg(debug_assertions)]
                                                    println!("Moving {} to {} {}", self.currently_moving.as_ref().unwrap().get_name(), col_idx, row_idx);
                                                    self.state_changed = true;
                                                }

                                            }

                                            #[cfg(debug_assertions)]
                                            if field_response.secondary_clicked() {
                                                println!("{:#?}", element);
                                            }

                                            if highlight == true { //Highlight fields valid for movement
                                                ui.painter().rect_stroke(
                                                    rect,
                                                    0.0,
                                                    egui::Stroke::new(3.0, egui::Color32::GOLD),
                                                );
                                            } else {
                                                ui.painter().rect_stroke(
                                                    rect,
                                                    0.0,
                                                    egui::Stroke::new(1.0, egui::Color32::WHITE),
                                                );
                                            }
                                        }
                                    });

                                    ui.end_row();

                                });

                                
                        })
                },
            );
        });
    }
}
