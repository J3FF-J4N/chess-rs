use crate::board::{Board, Direction};

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Eq)]
pub enum PieceType {
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Eq)]
pub enum Colour {
    White,
    Black,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Piece {
    //Information about the piece
    pub colour: Colour,
    pub piece_type: PieceType,
    pub pos_x: u8,
    pub pos_y: u8,
    pub piece_id: u8, //This is needed as there are several pieces that are the same

    //State of the piece
    pub is_moving: bool,

    /*
        There are three pieces where the first move matters:
        Pawn: The pawn is allowed to move two fields if it is its first move
        Rook and King: If neither the Rook nor the King moved, you are allowed to castle
    */
    pub first_move: bool,
}

const BOARD_MAX: u8 = 7;
const BOARD_MIN: u8 = 0;

impl Piece {
    pub fn new(colour: Colour, piece_type: PieceType, pos_x: u8, pos_y: u8, piece_id: u8) -> Self {
        Self {
            colour: colour,
            piece_type: piece_type,
            pos_x: pos_x,
            pos_y: pos_y,
            piece_id: piece_id,
            is_moving: false,
            first_move: true,
        }
    }

    // const BW: &'static str = "../assets/Casual/Pieces/Chess_white_casual/Bishop.png";
    // const KIW: &'static str = "../assets/Casual/Pieces/Chess_white_casual/King.png";
    // const KW: &'static str = "../assets/Casual/Pieces/Chess_white_casual/Knight.png";
    // const PW: &'static str = "../assets/Casual/Pieces/Chess_white_casual/Pawn.png";
    // const QW: &'static str = "../assets/Casual/Pieces/Chess_white_casual/Queen.png";
    // const RW: &'static str = "../assets/Casual/Pieces/Chess_white_casual/Rook.png";

    // const BWB: &'static str = "../assets/Casual/Pieces/Chess_black_casual/Bishop.png";
    // const KIB: &'static str = "../assets/Casual/Pieces/Chess_black_casual/King.png";
    // const KB: &'static str = "../assets/Casual/Pieces/Chess_black_casual/Knight.png";
    // const PB: &'static str = "../assets/Casual/Pieces/Chess_black_casual/Pawn.png";
    // const QB: &'static str = "../assets/Casual/Pieces/Chess_black_casual/Queen.png";
    // const RB: &'static str = "../assets/Casual/Pieces/Chess_black_casual/Rook.png";

    /*Unfortunately, "include_bytes!" needs a string literal and a constant variable does not work. This would make the code much prettier and more concise */
    pub fn get_image(&self) -> egui::widgets::Image<'_> {
        match self.colour {
            Colour::White => match self.piece_type {
                PieceType::Bishop => egui::widgets::Image::from_bytes(
                    "../assets/Casual/Pieces/Chess_white_casual/Bishop.png",
                    include_bytes!("../assets/Casual/Pieces/Chess_white_casual/Bishop.png"),
                )
                .maintain_aspect_ratio(true),
                PieceType::King => egui::widgets::Image::from_bytes(
                    "../assets/Casual/Pieces/Chess_white_casual/King.png",
                    include_bytes!("../assets/Casual/Pieces/Chess_white_casual/King.png"),
                )
                .maintain_aspect_ratio(true),
                PieceType::Knight => egui::widgets::Image::from_bytes(
                    "../assets/Casual/Pieces/Chess_white_casual/Knight.png",
                    include_bytes!("../assets/Casual/Pieces/Chess_white_casual/Knight.png"),
                )
                .maintain_aspect_ratio(true),
                PieceType::Pawn => egui::widgets::Image::from_bytes(
                    "../assets/Casual/Pieces/Chess_white_casual/Pawn.png",
                    include_bytes!("../assets/Casual/Pieces/Chess_white_casual/Pawn.png"),
                )
                .maintain_aspect_ratio(true),
                PieceType::Queen => egui::widgets::Image::from_bytes(
                    "../assets/Casual/Pieces/Chess_white_casual/Queen.png",
                    include_bytes!("../assets/Casual/Pieces/Chess_white_casual/Queen.png"),
                )
                .maintain_aspect_ratio(true),
                PieceType::Rook => egui::widgets::Image::from_bytes(
                    "../assets/Casual/Pieces/Chess_white_casual/Rook.png",
                    include_bytes!("../assets/Casual/Pieces/Chess_white_casual/Rook.png"),
                )
                .maintain_aspect_ratio(true),
            },
            Colour::Black => match self.piece_type {
                PieceType::Bishop => egui::widgets::Image::from_bytes(
                    "../assets/Casual/Pieces/Chess_black_casual/Bishop.png",
                    include_bytes!("../assets/Casual/Pieces/Chess_black_casual/Bishop.png"),
                )
                .maintain_aspect_ratio(true),
                PieceType::King => egui::widgets::Image::from_bytes(
                    "../assets/Casual/Pieces/Chess_black_casual/King.png",
                    include_bytes!("../assets/Casual/Pieces/Chess_black_casual/King.png"),
                )
                .maintain_aspect_ratio(true),
                PieceType::Knight => egui::widgets::Image::from_bytes(
                    "../assets/Casual/Pieces/Chess_black_casual/Knight.png",
                    include_bytes!("../assets/Casual/Pieces/Chess_black_casual/Knight.png"),
                )
                .maintain_aspect_ratio(true),
                PieceType::Pawn => egui::widgets::Image::from_bytes(
                    "../assets/Casual/Pieces/Chess_black_casual/Pawn.png",
                    include_bytes!("../assets/Casual/Pieces/Chess_black_casual/Pawn.png"),
                )
                .maintain_aspect_ratio(true),
                PieceType::Queen => egui::widgets::Image::from_bytes(
                    "../assets/Casual/Pieces/Chess_black_casual/Queen.png",
                    include_bytes!("../assets/Casual/Pieces/Chess_black_casual/Queen.png"),
                )
                .maintain_aspect_ratio(true),
                PieceType::Rook => egui::widgets::Image::from_bytes(
                    "../assets/Casual/Pieces/Chess_black_casual/Rook.png",
                    include_bytes!("../assets/Casual/Pieces/Chess_black_casual/Rook.png"),
                )
                .maintain_aspect_ratio(true),
            },
        }
    }

    pub fn get_name(&self) -> &str {
        match self.piece_type {
            PieceType::Bishop => "Bishop",
            PieceType::King => "King",
            PieceType::Knight => "Knight",
            PieceType::Pawn => "Pawn",
            PieceType::Queen => "Queen",
            PieceType::Rook => "Rook",
        }
    }

    /*A vector is much more sensible as it is not gueranteed that a piece moves at all. If it moves the space is allocated. */
    pub fn get_valid_moves(&self, boardstate: &Board) -> Vec<(u8, u8)> {
        match self.piece_type {
            PieceType::Bishop => self.get_bishop_moves(boardstate),
            PieceType::King => self.get_king_moves(boardstate),
            PieceType::Knight => self.get_knight_moves(boardstate),
            PieceType::Pawn => self.get_pawn_moves(boardstate),
            PieceType::Queen => self.get_queen_moves(boardstate),
            PieceType::Rook => self.get_rook_moves(boardstate),
        }
    }

    fn get_bishop_moves(&self, boardstate: &Board) -> Vec<(u8, u8)> {
        let mut blocking = Direction::default();

        let mut ret = vec![];

        for idx in 1..=BOARD_MAX {
            //There is a max of 8 possible fields at once the bishop can move

            // println!("X:{} Y:{}", self.pos_x, self.pos_y);
            //North East
            let new_x = self.pos_x.overflowing_add(idx);
            let new_y = self.pos_y.overflowing_sub(idx);
            if new_x.1 == false && new_y.1 == false {
                //Only when moves are within bounds should it be valid
                if idx <= blocking.north_east && new_x.0 <= BOARD_MAX && new_y.0 <= BOARD_MAX {
                    //Check if there is a piece blocking the path

                    if let Some(pot_blocking) = &boardstate.grid[new_y.0 as usize][new_x.0 as usize]
                    {
                        //If there is a piece some conditions need to be checked
                        if pot_blocking.colour == self.colour {
                            //If the piece at the given location is of the same colour the movement is blocked before
                            blocking.north_east = idx;
                        } else {
                            //If the piece at the given location is of the opposing colour the move is allowed and everything afterwards is blocked
                            blocking.north_east = idx;
                            ret.push((new_x.0, new_y.0)); //Thhis is then the last valid move in the given direction
                        }
                    } else {
                        //Otherwise the move is valid
                        ret.push((new_x.0, new_y.0));
                    }
                }
            }

            //South East
            let new_x = self.pos_x.overflowing_add(idx);
            let new_y = self.pos_y.overflowing_add(idx);
            if new_x.1 == false && new_y.1 == false {
                //Only when moves are within bounds should it be valid
                if idx <= blocking.south_east && new_x.0 <= BOARD_MAX && new_y.0 <= BOARD_MAX {
                    //Remain in bounds of the array
                    if let Some(pot_blocking) = &boardstate.grid[new_y.0 as usize][new_x.0 as usize]
                    {
                        //If there is a piece some conditions need to be checked
                        if pot_blocking.colour == self.colour {
                            //If the piece at the given location is of the same colour the movement is blocked before
                            blocking.south_east = idx;
                        } else {
                            //If the piece at the given location is of the opposing colour the move is allowed and everything afterwards is blocked
                            blocking.south_east = idx;
                            ret.push((new_x.0, new_y.0)); //Thhis is then the last valid move in the given direction
                        }
                    } else {
                        //Otherwise the move is valid
                        ret.push((new_x.0, new_y.0));
                    }
                }
            }
            //South West
            let new_x = self.pos_x.overflowing_sub(idx);
            let new_y = self.pos_y.overflowing_add(idx);
            if new_x.1 == false && new_y.1 == false {
                //Only when moves are within bounds should it be valid
                if idx <= blocking.south_west && new_x.0 <= BOARD_MAX && new_y.0 <= BOARD_MAX {
                    //Remain in bounds of the array
                    if let Some(pot_blocking) = &boardstate.grid[new_y.0 as usize][new_x.0 as usize]
                    {
                        //If there is a piece some conditions need to be checked
                        if pot_blocking.colour == self.colour {
                            //If the piece at the given location is of the same colour the movement is blocked before
                            blocking.south_west = idx;
                        } else {
                            //If the piece at the given location is of the opposing colour the move is allowed and everything afterwards is blocked
                            blocking.south_west = idx;
                            ret.push((new_x.0, new_y.0)); //Thhis is then the last valid move in the given direction
                        }
                    } else {
                        //Otherwise the move is valid
                        ret.push((new_x.0, new_y.0));
                    }
                }
            }

            //The first piece that appears in the top right corner blocks any other piece from blocking the path
            //North West
            let new_x = self.pos_x.overflowing_sub(idx);
            let new_y = self.pos_y.overflowing_sub(idx);
            if new_x.1 == false && new_y.1 == false && new_x.0 <= BOARD_MAX && new_y.0 <= BOARD_MAX
            {
                //Only when moves are within bounds should it be valid
                // if new_x.0 <= blocking.north_west && new_y.0 <= blocking.north_west { //Remain in bounds of the array
                if idx <= blocking.north_west {
                    //Remain in bounds of the array
                    if let Some(pot_blocking) = &boardstate.grid[new_y.0 as usize][new_x.0 as usize]
                    {
                        //If there is a piece some conditions need to be checked

                        if pot_blocking.colour == self.colour {
                            //If the piece at the given location is of the same colour the movement is blocked before

                            blocking.north_west = idx;
                        } else {
                            //If the piece at the given location is of the opposing colour the move is allowed and everything afterwards is blocked

                            blocking.north_west = idx;

                            ret.push((new_x.0, new_y.0)); //Thhis is then the last valid move in the given direction
                        }
                    } else {
                        //Otherwise the move is valid
                        ret.push((new_x.0, new_y.0));
                    }
                }
            }
        }

        ret
    }

    fn get_king_moves(&self, boardstate: &Board) -> Vec<(u8, u8)> {
        let mut ret = vec![];

        if self.first_move == true {
            //Castle
        }

        //North
        let new_y = self.pos_y.overflowing_sub(1);

        if new_y.1 == false {
            if new_y.0 <= BOARD_MAX && new_y.0 <= BOARD_MAX {
                if let Some(pot_blocking) = &boardstate.grid[new_y.0 as usize][self.pos_x as usize]
                {
                    if pot_blocking.colour != self.colour {
                        //If the piece is of the opposing colour the move is valid
                        ret.push((self.pos_x, new_y.0));
                    }
                } else {
                    //Otherwise the move is valid
                    ret.push((self.pos_x, new_y.0));
                }

                //North East
                let new_x = self.pos_x.overflowing_add(1);

                if new_x.1 == false {
                    if new_x.0 <= BOARD_MAX && new_x.0 <= BOARD_MAX {
                        if let Some(pot_blocking) =
                            &boardstate.grid[new_y.0 as usize][new_x.0 as usize]
                        {
                            if pot_blocking.colour != self.colour {
                                //If the piece is of the opposing colour the move is valid
                                ret.push((new_x.0, new_y.0));
                            }
                        } else {
                            //Otherwise the move is valid
                            ret.push((new_x.0, new_y.0));
                        }
                    }
                }

                //North West
                let new_x = self.pos_x.overflowing_sub(1);

                if new_x.1 == false {
                    if new_x.0 <= BOARD_MAX && new_x.0 <= BOARD_MAX {
                        if let Some(pot_blocking) =
                            &boardstate.grid[new_y.0 as usize][new_x.0 as usize]
                        {
                            if pot_blocking.colour != self.colour {
                                //If the piece is of the opposing colour the move is valid
                                ret.push((new_x.0, new_y.0));
                            }
                        } else {
                            //Otherwise the move is valid
                            ret.push((new_x.0, new_y.0));
                        }
                    }
                }
            }
        }

        //East
        let new_x = self.pos_x.overflowing_add(1);

        if new_x.1 == false {
            if new_x.0 <= BOARD_MAX && new_x.0 <= BOARD_MAX {
                if let Some(pot_blocking) = &boardstate.grid[self.pos_y as usize][new_x.0 as usize]
                {
                    if pot_blocking.colour != self.colour {
                        //If the piece is of the opposing colour the move is valid
                        ret.push((new_x.0, self.pos_y));
                    }
                } else {
                    //Otherwise the move is valid
                    ret.push((new_x.0, self.pos_y));
                }
            }
        }

        //West
        let new_x = self.pos_x.overflowing_sub(1);

        if new_x.1 == false {
            if new_x.0 <= BOARD_MAX && new_x.0 <= BOARD_MAX {
                if let Some(pot_blocking) = &boardstate.grid[self.pos_y as usize][new_x.0 as usize]
                {
                    if pot_blocking.colour != self.colour {
                        //If the piece is of the opposing colour the move is valid
                        ret.push((new_x.0, self.pos_y));
                    }
                } else {
                    //Otherwise the move is valid
                    ret.push((new_x.0, self.pos_y));
                }
            }
        }

        //South
        let new_y = self.pos_y.overflowing_add(1);

        if new_y.1 == false {
            if new_y.0 <= BOARD_MAX && new_y.0 <= BOARD_MAX {
                if let Some(pot_blocking) = &boardstate.grid[new_y.0 as usize][self.pos_x as usize]
                {
                    if pot_blocking.colour != self.colour {
                        //If the piece is of the opposing colour the move is valid
                        ret.push((self.pos_x, new_y.0));
                    }
                } else {
                    //Otherwise the move is valid
                    ret.push((self.pos_x, new_y.0));
                }

                //South East
                let new_x = self.pos_x.overflowing_add(1);

                if new_x.1 == false {
                    if new_x.0 <= BOARD_MAX && new_x.0 <= BOARD_MAX {
                        if let Some(pot_blocking) =
                            &boardstate.grid[new_y.0 as usize][new_x.0 as usize]
                        {
                            if pot_blocking.colour != self.colour {
                                //If the piece is of the opposing colour the move is valid
                                ret.push((new_x.0, new_y.0));
                            }
                        } else {
                            //Otherwise the move is valid
                            ret.push((new_x.0, new_y.0));
                        }
                    }
                }

                //South West
                let new_x = self.pos_x.overflowing_sub(1);

                if new_x.1 == false {
                    if new_x.0 <= BOARD_MAX && new_x.0 <= BOARD_MAX {
                        if let Some(pot_blocking) =
                            &boardstate.grid[new_y.0 as usize][new_x.0 as usize]
                        {
                            if pot_blocking.colour != self.colour {
                                //If the piece is of the opposing colour the move is valid
                                ret.push((new_x.0, new_y.0));
                            }
                        } else {
                            //Otherwise the move is valid
                            ret.push((new_x.0, new_y.0));
                        }
                    }
                }
            }
        }

        ret
    }

    fn get_knight_moves(&self, boardstate: &Board) -> Vec<(u8, u8)> {
        let mut ret = vec![];

        //North
        let new_y = self.pos_y.overflowing_sub(2);
        if new_y.1 == false {
            //Only when moves are within bounds should it be valid
            if new_y.0 <= BOARD_MAX {
                let new_x = self.pos_x.overflowing_add(1); //3 North 1 East

                if new_x.1 == false && new_x.0 <= BOARD_MAX {
                    //If there is a piece some conditions need to be checked
                    if let Some(pot_blocking) = &boardstate.grid[new_y.0 as usize][new_x.0 as usize]
                    {
                        if pot_blocking.colour != self.colour {
                            //If the piece is of the opposing colour the move is valid
                            ret.push((new_x.0, new_y.0));
                        }
                    } else {
                        //Otherwise the move is valid
                        ret.push((new_x.0, new_y.0));
                    }
                }

                let new_x = self.pos_x.overflowing_sub(1); //3 North 1 West

                if new_x.1 == false && new_x.0 <= BOARD_MAX {
                    //If there is a piece some conditions need to be checked
                    if let Some(pot_blocking) = &boardstate.grid[new_y.0 as usize][new_x.0 as usize]
                    {
                        if pot_blocking.colour != self.colour {
                            //If the piece is of the opposing colour the move is valid
                            ret.push((new_x.0, new_y.0));
                        }
                    } else {
                        //Otherwise the move is valid
                        ret.push((new_x.0, new_y.0));
                    }
                }
            }
        }

        //East
        let new_x = self.pos_x.overflowing_sub(2);
        if new_x.1 == false {
            //Only when moves are within bounds should it be valid
            if new_x.0 <= BOARD_MAX {
                let new_y = self.pos_y.overflowing_add(1); //3 East 1 South

                if new_y.1 == false && new_y.0 <= BOARD_MAX {
                    //If there is a piece some conditions need to be checked
                    if let Some(pot_blocking) = &boardstate.grid[new_y.0 as usize][new_x.0 as usize]
                    {
                        if pot_blocking.colour != self.colour {
                            //If the piece is of the opposing colour the move is valid
                            ret.push((new_x.0, new_y.0));
                        }
                    } else {
                        //Otherwise the move is valid
                        ret.push((new_x.0, new_y.0));
                    }
                }

                let new_y = self.pos_y.overflowing_sub(1); //3 East 1 North

                if new_y.1 == false && new_y.0 <= BOARD_MAX {
                    //If there is a piece some conditions need to be checked
                    if let Some(pot_blocking) = &boardstate.grid[new_y.0 as usize][new_x.0 as usize]
                    {
                        if pot_blocking.colour != self.colour {
                            //If the piece is of the opposing colour the move is valid
                            ret.push((new_x.0, new_y.0));
                        }
                    } else {
                        //Otherwise the move is valid
                        ret.push((new_x.0, new_y.0));
                    }
                }
            }
        }
        //South
        let new_y = self.pos_y.overflowing_add(2);
        if new_y.1 == false {
            //Only when moves are within bounds should it be valid
            if new_y.0 <= BOARD_MAX {
                let new_x = self.pos_x.overflowing_add(1); //3 South 1 East

                if new_x.1 == false && new_x.0 <= BOARD_MAX {
                    //If there is a piece some conditions need to be checked
                    if let Some(pot_blocking) = &boardstate.grid[new_y.0 as usize][new_x.0 as usize]
                    {
                        if pot_blocking.colour != self.colour {
                            //If the piece is of the opposing colour the move is valid
                            ret.push((new_x.0, new_y.0));
                        }
                    } else {
                        //Otherwise the move is valid
                        ret.push((new_x.0, new_y.0));
                    }
                }

                let new_x = self.pos_x.overflowing_sub(1); //3 South 1 West

                if new_x.1 == false && new_x.0 <= BOARD_MAX {
                    //If there is a piece some conditions need to be checked
                    if let Some(pot_blocking) = &boardstate.grid[new_y.0 as usize][new_x.0 as usize]
                    {
                        if pot_blocking.colour != self.colour {
                            //If the piece is of the opposing colour the move is valid
                            ret.push((new_x.0, new_y.0));
                        }
                    } else {
                        //Otherwise the move is valid
                        ret.push((new_x.0, new_y.0));
                    };
                }
            }
        }
        //West
        let new_x = self.pos_x.overflowing_add(2);
        if new_x.1 == false {
            //Only when moves are within bounds should it be valid
            if new_x.0 <= BOARD_MAX {
                let new_y = self.pos_y.overflowing_add(1); //3 East 1 South

                if new_y.1 == false && new_y.0 <= BOARD_MAX {
                    //If there is a piece some conditions need to be checked
                    if let Some(pot_blocking) = &boardstate.grid[new_y.0 as usize][new_x.0 as usize]
                    {
                        if pot_blocking.colour != self.colour {
                            //If the piece is of the opposing colour the move is valid
                            ret.push((new_x.0, new_y.0));
                        }
                    } else {
                        //Otherwise the move is valid
                        ret.push((new_x.0, new_y.0));
                    }
                }

                let new_y = self.pos_y.overflowing_sub(1); //3 East 1 North

                if new_y.1 == false && new_y.0 <= BOARD_MAX {
                    //If there is a piece some conditions need to be checked
                    if let Some(pot_blocking) = &boardstate.grid[new_y.0 as usize][new_x.0 as usize]
                    {
                        if pot_blocking.colour != self.colour {
                            //If the piece is of the opposing colour the move is valid
                            ret.push((new_x.0, new_y.0));
                        }
                    } else {
                        //Otherwise the move is valid
                        ret.push((new_x.0, new_y.0));
                    }
                }
            }
        }

        ret
    }

    fn get_pawn_moves(&self, boardstate: &Board) -> Vec<(u8, u8)> {
        let mut ret: Vec<(u8, u8)> = Vec::with_capacity(16); //64 Is the amount of available spaces on a chess board; it is impossible to have more moves than available spaces

        //TODO En Passant

        //Pawn is the only piece where the colour matters as it's the only piece that has a direction dependency (e.g. Black can only go Down and White up)
        let times_to_loop = if self.first_move == true {
            2
        } else {
            1
            // let new_y = match self.colour {
            //     Colour::White => self.pos_y.saturating_sub(1), //For White the row index decreases
            //     Colour::Black => self.pos_y.saturating_add(1), //For Black the row index increases
            // };

            // let new_x_right = self.pos_x.saturating_add(1);
            // let new_x_left = self.pos_x.saturating_sub(1);

            // if let None = &boardstate.grid[new_y as usize][self.pos_x as usize] { //If there is a piece of any colour the move is blocked since pawns can't capture vertically
            //     ret.push((self.pos_x, new_y));
            // }
        };

        for y in 1..=times_to_loop {
            let new_y = match self.colour {
                Colour::White => self.pos_y.saturating_sub(y), //For White the row index decreases
                Colour::Black => self.pos_y.saturating_add(y), //For Black the row index increases
            };

            if self.pos_x == BOARD_MAX {
                //The right most pawn can't capture diagonally in the positive direction
                let new_x = self.pos_x.saturating_sub(1);
                if let Some(_) = &boardstate.grid[new_y as usize][new_x as usize] {
                    ret.push((new_x, new_y));
                }
            } else if self.pos_y == BOARD_MIN {
                //The left most pawn can't capture diagonally in the negative direction
                let new_x = self.pos_x.saturating_add(1);
                if let Some(_) = &boardstate.grid[new_y as usize][new_x as usize] {
                    ret.push((new_x, new_y));
                }
            } else {
                let new_x_neg = self.pos_x.saturating_sub(1);
                let new_x_pos = self.pos_x.saturating_add(1);
                if let Some(_) = &boardstate.grid[new_y as usize][new_x_neg as usize] {
                    ret.push((new_x_neg, new_y));
                }

                if let Some(_) = &boardstate.grid[new_y as usize][new_x_pos as usize] {
                    ret.push((new_x_pos, new_y));
                }
            }

            // let new_x_right = self.pos_x.saturating_add(1);
            // let new_x_left = self.pos_x.saturating_add(1);

            if let None = &boardstate.grid[new_y as usize][self.pos_x as usize] {
                //If there is a piece of any colour the move is blocked since pawns can't capture vertically
                ret.push((self.pos_x, new_y));
            } else {
                break;
            }
        }

        ret
    }

    fn get_queen_moves(&self, boardstate: &Board) -> Vec<(u8, u8)> {
        let mut ret = vec![];

        //Queen has the moveset of bishop + rook
        ret.append(&mut self.get_bishop_moves(boardstate));

        ret.append(&mut self.get_rook_moves(boardstate));

        ret
    }

    fn get_rook_moves(&self, boardstate: &Board) -> Vec<(u8, u8)> {
        let mut ret = vec![];

        let mut blocking = Direction::default();

        for idx in 1..=BOARD_MAX {
            //There is a max of 8 possible fields at once the bishop can move

            //North
            let new_y = self.pos_y.overflowing_sub(idx);
            if new_y.1 == false {
                //Only when moves are within bounds should it be valid
                if new_y.0 <= BOARD_MAX && idx <= blocking.north {
                    //If there is a piece some conditions need to be checked
                    if let Some(pot_blocking) =
                        &boardstate.grid[new_y.0 as usize][self.pos_x as usize]
                    {
                        if pot_blocking.colour == self.colour {
                            //If the piece at the given location is of the same colour the movement is blocked before

                            blocking.north = idx;
                        } else {
                            //If the piece at the given location is of the opposing colour the move is allowed and everything afterwards is blocked

                            blocking.north = idx;

                            ret.push((self.pos_x, new_y.0)); //Thhis is then the last valid move in the given direction
                        }
                    } else {
                        //Otherwise the move is valid
                        ret.push((self.pos_x, new_y.0));
                    }
                }
            }

            //East
            let new_x = self.pos_x.overflowing_add(idx);
            if new_x.1 == false {
                //Only when moves are within bounds should it be valid
                if new_x.0 <= BOARD_MAX && idx <= blocking.east {
                    //If there is a piece some conditions need to be checked
                    if let Some(pot_blocking) =
                        &boardstate.grid[self.pos_y as usize][new_x.0 as usize]
                    {
                        if pot_blocking.colour == self.colour {
                            //If the piece at the given location is of the same colour the movement is blocked before

                            blocking.east = idx;
                        } else {
                            //If the piece at the given location is of the opposing colour the move is allowed and everything afterwards is blocked

                            blocking.east = idx;

                            ret.push((new_x.0, self.pos_y)); //Thhis is then the last valid move in the given direction
                        }
                    } else {
                        //Otherwise the move is valid
                        ret.push((new_x.0, self.pos_y));
                    }
                }
            }
            //South
            let new_y = self.pos_y.overflowing_add(idx);
            if new_y.1 == false {
                //Only when moves are within bounds should it be valid
                if new_y.0 <= BOARD_MAX && idx <= blocking.south {
                    //If there is a piece some conditions need to be checked
                    if let Some(pot_blocking) =
                        &boardstate.grid[new_y.0 as usize][self.pos_x as usize]
                    {
                        if pot_blocking.colour == self.colour {
                            //If the piece at the given location is of the same colour the movement is blocked before

                            blocking.south = idx;
                        } else {
                            //If the piece at the given location is of the opposing colour the move is allowed and everything afterwards is blocked

                            blocking.south = idx;

                            ret.push((self.pos_x, new_y.0)); //Thhis is then the last valid move in the given direction
                        }
                    } else {
                        //Otherwise the move is valid
                        ret.push((self.pos_x, new_y.0));
                    }
                }
            }
            //West
            let new_x = self.pos_x.overflowing_sub(idx);
            if new_x.1 == false {
                //Only when moves are within bounds should it be valid
                if new_x.0 <= BOARD_MAX && idx <= blocking.west {
                    //If there is a piece some conditions need to be checked
                    if let Some(pot_blocking) =
                        &boardstate.grid[self.pos_y as usize][new_x.0 as usize]
                    {
                        if pot_blocking.colour == self.colour {
                            //If the piece at the given location is of the same colour the movement is blocked before

                            blocking.west = idx;
                        } else {
                            //If the piece at the given location is of the opposing colour the move is allowed and everything afterwards is blocked

                            blocking.west = idx;

                            ret.push((new_x.0, self.pos_y)); //Thhis is then the last valid move in the given direction
                        }
                    } else {
                        //Otherwise the move is valid
                        ret.push((new_x.0, self.pos_y));
                    }
                }
            }
        }

        //A player may not castle out of, through, or into check.

        //when in double check the only legal moves are king moves

        ret
    }

    pub fn move_piece(&mut self, new_x: u8, new_y: u8) {
        if self.first_move == true {
            self.first_move = false;
        }

        self.pos_x = new_x;
        self.pos_y = new_y;
    }

    // pub fn get_id(&self) -> egui::Id {
    //     egui::Id::new((self.get_name(), self.pos_x, self.pos_y))
    // }

    // const PIECE_COUNT: u8 = 8;

    // pub fn get_white_piece_init(){
    //     let ret = [Self::PIECE_COUNT; 0];

    // }

    // pub fn get_black_piece_init(){

    // }
}
