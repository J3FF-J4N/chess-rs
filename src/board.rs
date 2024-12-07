use crate::piece::*;
// use crate::piece::Colour;

/*This struct is used to determine of the path is blocked in a certain direction */
#[derive(Debug)]
pub struct Direction {
    pub north: u8,
    pub north_east: u8,
    pub east: u8,
    pub south_east: u8,
    pub south: u8,
    pub south_west: u8,
    pub west: u8,
    pub north_west: u8,
}

impl Default for Direction {
    /*The default assumption is all paths are free */
    fn default() -> Self {
        Self {
            north: 7,
            north_east: 7,
            east: 7,
            south_east: 7,
            south: 7,
            south_west: 7,
            west: 7,
            north_west: 7,
        }
    }
}

const BOARD_HEIGHT: usize = 8;
const BOARD_WIDTH: usize = 8;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Board {
    pub grid: [[Option<Piece>; BOARD_HEIGHT]; BOARD_WIDTH], //Using arrays instead of Vec since arrays are on the stack and; therefore, much faster
}

impl Board {
    pub fn init_board_state() -> Self {
        let mut state: [[Option<Piece>; BOARD_HEIGHT]; BOARD_WIDTH] =
            [[None; BOARD_HEIGHT]; BOARD_WIDTH];

        //To avoid immense overhead caused by bounds checking the array is sliced into it's sides first and initialised that way

        //This is ugly af but it avoids the overhead of array boundary checking
        //Fill the board with white pieces
        let mut iter = state[BOARD_HEIGHT - 1..=BOARD_HEIGHT - 1]
            .iter_mut()
            .next()
            .unwrap()
            .iter_mut();
        *iter.next().unwrap() = Some(Piece::new(Colour::White, PieceType::Rook, 0, 7, 0));
        *iter.next().unwrap() = Some(Piece::new(Colour::White, PieceType::Knight, 1, 7, 0));
        *iter.next().unwrap() = Some(Piece::new(Colour::White, PieceType::Bishop, 2, 7, 0));
        *iter.next().unwrap() = Some(Piece::new(Colour::White, PieceType::Queen, 3, 7, 0));
        *iter.next().unwrap() = Some(Piece::new(Colour::White, PieceType::King, 4, 7, 0));
        *iter.next().unwrap() = Some(Piece::new(Colour::White, PieceType::Bishop, 5, 7, 1)); //Id must be different for duplicate pieces
        *iter.next().unwrap() = Some(Piece::new(Colour::White, PieceType::Knight, 6, 7, 1));
        *iter.next().unwrap() = Some(Piece::new(Colour::White, PieceType::Rook, 7, 7, 1));

        //Fill the board with black pieces
        let mut iter = state[0..=0].iter_mut().next().unwrap().iter_mut(); //Chess Board Row 8 Black Pieces
        *iter.next().unwrap() = Some(Piece::new(Colour::Black, PieceType::Rook, 0, 0, 0));
        *iter.next().unwrap() = Some(Piece::new(Colour::Black, PieceType::Knight, 1, 0, 0));
        *iter.next().unwrap() = Some(Piece::new(Colour::Black, PieceType::Bishop, 2, 0, 0));
        *iter.next().unwrap() = Some(Piece::new(Colour::Black, PieceType::Queen, 3, 0, 0));
        *iter.next().unwrap() = Some(Piece::new(Colour::Black, PieceType::King, 4, 0, 0));
        *iter.next().unwrap() = Some(Piece::new(Colour::Black, PieceType::Bishop, 5, 0, 1)); //Id must be different for duplicate pieces
        *iter.next().unwrap() = Some(Piece::new(Colour::Black, PieceType::Knight, 6, 0, 1));
        *iter.next().unwrap() = Some(Piece::new(Colour::Black, PieceType::Rook, 7, 0, 1));

        //This is much prettier and  does the same; however, it can only be done for pawns as they are all the same
        //Fill the board with white pawns
        let mut temp: u8 = 0;
        state[BOARD_HEIGHT - 2..=BOARD_HEIGHT - 2]
            .iter_mut()
            .for_each(|row| {
                //Chess Board Row 2 White Pawns
                row.iter_mut().for_each(|piece| {
                    *piece = Some(Piece::new(
                        Colour::White,
                        PieceType::Pawn,
                        temp,
                        (BOARD_HEIGHT - 2) as u8,
                        temp,
                    ));
                    temp += 1;
                });
            });

        //Fill the board with black pawns
        temp = 0;
        state[1..=1].iter_mut().for_each(|row| {
            //Chess Board Row 7 Black Pawns
            row.iter_mut().for_each(|piece| {
                *piece = Some(Piece::new(Colour::Black, PieceType::Pawn, temp, 1, temp));
                temp += 1;
            });
        });

        Self { grid: state }
    }

    // pub fn get_board_state(&self) {}

    pub fn update_board(&mut self, piece_to_update: Option<Piece>) {
        if let Some(piece_to_update) = piece_to_update {
            self.grid.iter_mut().enumerate().for_each(|(row_idx, row)| {
                row.iter_mut().enumerate().for_each(|(col_idx, piece)| {
                    if let Some(piece_ref) = piece {
                        if piece_to_update.get_colour() == piece_ref.get_colour()
                            && piece_to_update.get_type() == piece_ref.get_type()
                            && piece_to_update.piece_id == piece_ref.piece_id
                        {
                            //Remove the old location
                            *piece = None;
                        }
                    }
                    if piece_to_update.pos_x == col_idx as u8
                        && piece_to_update.pos_y == row_idx as u8
                    {
                        //Add to new location
                        *piece = Some(piece_to_update);
                    }
                });
            });

            // let mut state = std::fs::File::create("./boardstate").unwrap();

            // write!(state, "{}", ron::ser::to_string_pretty(&self.grid, ron::ser::PrettyConfig::default()).unwrap()).unwrap();

            // writeln!(state, "------------------------------------------------------------------------").unwrap();
        } else {
            return;
        }
    }

    pub fn get_valid_moves(&mut self, piece: &Piece) -> Vec<(u8, u8)> {
        /*These are the hypothetical valid moves that a given piece can make, these need to be filtered when the path is obstructed.
        The only piece that ignores blocked paths is the knight*/
        let moves = piece.get_valid_moves(self);

        // if piece.get_type() == PieceType::Knight { // If the piece is a knight no more filtering is necessary as it can jump over any piece
        //     return moves;
        // }

        // let blocked: Direction = Direction::default();
        // /*Filtering out moves that are blocked by friendly / opposing pieces */
        // self.grid.iter_mut().enumerate().for_each(|(row_idx, row)|{
        //     row.iter_mut().enumerate().for_each(|(col_idx, col)| {

        //         if let Some(piece_ref) = col { //A piece is present
        //             let direction = (piece_ref.pos_x as i8 - piece.pos_x as i8, piece_ref.pos_y as i8 - piece.pos_y as i8); //Determine the x and y direction the potentially blocking piece is located
        //             let step_x = direction.0.signum();
        //             let step_y = direction.1.signum();
        //         }

        //     });
        // });

        // let dir = Direction::East(5);

        // match dir {
        //     Direction::North(x) => todo!(),
        //     Direction::NorthEast(_) => todo!(),
        //     Direction::East(_) => todo!(),
        //     Direction::SouthEast(_) => todo!(),
        //     Direction::South(_) => todo!(),
        //     Direction::SouthWest(_) => todo!(),
        //     Direction::West(_) => todo!(),
        //     Direction::NorthWest(_) => todo!(),
        // }

        // let mut valid_moves = Vec::new();

        // for (target_x, target_y) in moves {
        //     let direction = (target_x as i8 - piece.pos_x as i8, target_y as i8 - piece.pos_y as i8);
        //     let step_x = direction.0.signum();
        //     let step_y = direction.1.signum();

        //     let mut current_x = piece.pos_x as i8;
        //     let mut current_y = piece.pos_y as i8;

        //     // Move in the direction until we hit a piece or the edge of the board
        //     while (current_x + step_x >= 0 && current_x + step_x < 8) && (current_y + step_y >= 0 && current_y + step_y < 8) {
        //         current_x += step_x;
        //         current_y += step_y;

        //         if let Some(piece_at_target) = self.grid[current_y as usize][current_x as usize] {
        //             // If we hit a piece
        //             if piece_at_target.get_colour() == piece.get_colour() {
        //                 // Blocked by our own piece
        //                 break;
        //             } else {
        //                 // It's an opponent's piece, we can capture it
        //                 valid_moves.push((current_x as u8, current_y as u8));
        //                 break; // Stop checking further in this direction
        //             }
        //         } else {
        //             // Empty square, add to valid moves
        //             valid_moves.push((current_x as u8, current_y as u8));
        //         }
        //     }
        // }

        moves

        // moves.iter().for_each(|to_filter|{

        //     if highlight == true { //If the highlight is already false there is no need to continue checking for equality
        //         highlight = !element.pos_x.eq(&to_filter.0) || !element.pos_y.eq(&to_filter.1); //If either of the coordinates differ the highlight is kept
        //     }

        // });

        // moves
    }

    // pub fn check_location(&self, x: u8, y: u8) {}
}

// let new_x_y = piece_to_update.get_x_y();
// .for_each(|piece| {
//     if let Some(piece) = piece {
//         if piece_to_update.get_colour() == piece.get_colour() && piece_to_update.get_type() == piece.get_type() && piece_to_update.piece_id == piece.piece_id {
//             piece.pos_x = new_x_y.0;
//             piece.pos_y = new_x_y.1;
//             dbg!(piece);
//         }
//     }
// });
