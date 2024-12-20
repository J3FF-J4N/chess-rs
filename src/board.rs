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
    pub current_player: Colour
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

        //White will always start
        Self { grid: state, current_player: Colour::White }
    }

    // pub fn get_board_state(&self) {}

    pub fn update_board(&mut self, piece_to_update: &mut Option<Piece>) -> bool {

        let mut ret: bool = false;

        if let Some(mut piece_to_update) = piece_to_update {

            let piece_clone = piece_to_update.clone(); //This is necessary as the new location may be updated before the old piece is removed which would change the pieces properties

            self.grid.iter_mut().enumerate().for_each(|(row_idx, row)| {
                row.iter_mut().enumerate().for_each(|(col_idx, piece)| {
                    if let Some(piece_ref) = piece {
                        if piece_clone.get_colour() == piece_ref.get_colour() //Find the piece that needs to be moved
                            && piece_clone.get_type() == piece_ref.get_type()
                            && piece_clone.piece_id == piece_ref.piece_id
                        {
                            //Remove the old location
                            *piece = None;
                        }
                    }
                    if piece_to_update.pos_x == col_idx as u8
                        && piece_to_update.pos_y == row_idx as u8
                    {

                        if piece_to_update.get_type() == PieceType::Pawn {//If the piece is a pawn I need to check if it should be promoted to a queen

                            if piece_to_update.pos_y == BOARD_HEIGHT as u8 - 1 || piece_to_update.pos_y == BOARD_HEIGHT as u8 - 8 { //If the pawn reached the max rank (black) / min rank (white) it get's promoted
                                piece_to_update.piece_type = PieceType::Queen;
                            }   

                        }

                        piece_to_update.is_moving = false; //Reset the piece's movement state after a successful move
                        //Add to new location
                        *piece = Some(piece_to_update);
                        ret = true;
                    }
                });
            });

            ret
            // let mut state = std::fs::File::create("./boardstate").unwrap();

            // write!(state, "{}", ron::ser::to_string_pretty(&self.grid, ron::ser::PrettyConfig::default()).unwrap()).unwrap();

            // writeln!(state, "------------------------------------------------------------------------").unwrap();
        } else {
            ret
        }
    }

    pub fn get_valid_moves(&mut self, piece: &Piece) -> Vec<(u8, u8)> {
        /*These are the hypothetical valid moves that a given piece can make, these need to be filtered when the path is obstructed.
        The only piece that ignores blocked paths is the knight*/
        let moves = piece.get_valid_moves(self);

        moves
    }

    // pub fn check_location(&self, x: u8, y: u8) {}
}
