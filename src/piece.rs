#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug)]
pub enum PieceType {
    Bishop,
    King,
    Knight,
    Pawn,
    Queen,
    Rook,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug)]
pub enum Colour {
    White,
    Black,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug)]
pub struct Piece {
    //Information about the piece
    colour: Colour,
    piece_type: PieceType,
    pos_x: u8,
    pos_y: u8,

    //State of the piece
    pub is_moving: bool,

    /*
        There are three pieces where the first move matters:
        Pawn: The pawn is allowed to move two fields if it is its first move
        Rook and King: If neither the Rook nor the King moved, you are allowed to castle
    */
    pub first_move: bool,
}

impl Piece {
    pub fn new(colour: Colour, piece_type: PieceType, pos_x: u8, pos_y: u8) -> Self {
        Self {
            colour: colour,
            piece_type: piece_type,
            pos_x: pos_x,
            pos_y: pos_y,
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
    pub fn get_valid_moves(&self) -> Vec<(u8, u8)> {
        match self.piece_type {
            PieceType::Bishop => self.get_bishop_moves(),
            PieceType::King => self.get_king_moves(),
            PieceType::Knight => self.get_knight_moves(),
            PieceType::Pawn => self.get_pawn_moves(),
            PieceType::Queen => self.get_queen_moves(),
            PieceType::Rook => self.get_rook_moves(),
        }
    }

    fn get_bishop_moves(&self) -> Vec<(u8, u8)> {
        let mut ret = vec![];

        for idx in 0..=7{//There is a max of 8 possible fields at once the bishop can move

            //North East
            let new_x = self.pos_x.overflowing_add(idx);
            let new_y = self.pos_y.overflowing_sub(idx);
            if new_x.1 == false && new_y.1 == false { //Only when moves are within bounds should it be valid
                if new_x.0 <= 7 && new_y.0 <= 7{
                    ret.push((new_x.0, new_y.0));
                }
            }

            //South East
            let new_x = self.pos_x.overflowing_add(idx);
            let new_y = self.pos_y.overflowing_add(idx);
            if new_x.1 == false && new_y.1 == false { //Only when moves are within bounds should it be valid
                if new_x.0 <= 7 && new_y.0 <= 7{
                    ret.push((new_x.0, new_y.0));
                }
            }
            //South West
            let new_x = self.pos_x.overflowing_sub(idx);
            let new_y = self.pos_y.overflowing_add(idx);
            if new_x.1 == false && new_y.1 == false { //Only when moves are within bounds should it be valid
                if new_x.0 <= 7 && new_y.0 <= 7{
                    ret.push((new_x.0, new_y.0));
                }
            }
            //North West
            let new_x = self.pos_x.overflowing_sub(idx);
            let new_y = self.pos_y.overflowing_sub(idx);
            if new_x.1 == false && new_y.1 == false { //Only when moves are within bounds should it be valid
                if new_x.0 <= 7 && new_y.0 <= 7{
                    ret.push((new_x.0, new_y.0));
                }
            }

        }

        ret
    }

    fn get_king_moves(&self) -> Vec<(u8, u8)> {
        let mut ret = vec![];

        ret
    }

    fn get_knight_moves(&self) -> Vec<(u8, u8)> {
        let mut ret = vec![];


                    //North
                    let new_y = self.pos_y.overflowing_sub(2);
                    if new_y.1 == false { //Only when moves are within bounds should it be valid
                        if new_y.0 <= 7{
                            let new_x = self.pos_x.overflowing_add(1); //3 North 1 East

                            if new_x.1 == false {
                                ret.push((new_x.0, new_y.0));
                            }
                            
                            let new_x = self.pos_x.overflowing_sub(1); //3 North 1 West

                            if new_x.1 == false {
                                ret.push((new_x.0, new_y.0));
                            }
                        }
                    }
        
                    //East
                    let new_x = self.pos_x.overflowing_sub(2);
                    if new_x.1 == false { //Only when moves are within bounds should it be valid
                        if new_x.0 <= 7{
                            let new_y = self.pos_y.overflowing_add(1); //3 East 1 South

                            if new_y.1 == false {
                                ret.push((new_x.0, new_y.0));
                            }
                            
                            let new_y = self.pos_y.overflowing_sub(1); //3 East 1 North

                            if new_y.1 == false {
                                ret.push((new_x.0, new_y.0));
                            }
                        }
                    }
                    //South
                    let new_y = self.pos_y.overflowing_add(2);
                    if new_y.1 == false { //Only when moves are within bounds should it be valid
                        if new_y.0 <= 7{
                            let new_x = self.pos_x.overflowing_add(1); //3 South 1 East

                            if new_x.1 == false {
                                ret.push((new_x.0, new_y.0));
                            }
                            
                            let new_x = self.pos_x.overflowing_sub(1); //3 South 1 West

                            if new_x.1 == false {
                                ret.push((new_x.0, new_y.0));
                            }
                        }
                    }
                    //West
                    let new_x = self.pos_x.overflowing_add(2);
                    if new_x.1 == false { //Only when moves are within bounds should it be valid
                        if new_x.0 <= 7{
                            let new_y = self.pos_y.overflowing_add(1); //3 East 1 South

                            if new_y.1 == false {
                                ret.push((new_x.0, new_y.0));
                            }
                            
                            let new_y = self.pos_y.overflowing_sub(1); //3 East 1 North

                            if new_y.1 == false {
                                ret.push((new_x.0, new_y.0));
                            }
                        }
                    }

        ret
    }

    fn get_pawn_moves(&self) -> Vec<(u8, u8)> {
        let mut ret: Vec<(u8, u8)> = Vec::with_capacity(64); //64 Is the amount of available spaces on a chess board; it is impossible to have more moves than available spaces

        //Pawn is the only piece where the colour matters as it's the only piece that has a direction dependency (e.g. Black can only go Down and White up)
        if self.first_move == true {
            match self.colour {
                Colour::White => {
                    //For White the row index decreases
                    ret.push((self.pos_x, self.pos_y - 2)); //If it is the pawns first move it may move two fields or one field in the y direction
                    ret.push((self.pos_x, self.pos_y - 1));
                }
                Colour::Black => {
                    //For Black the row index increases
                    ret.push((self.pos_x, self.pos_y + 2)); //If it is the pawns first move it may move two fields or one field in the y direction
                    ret.push((self.pos_x, self.pos_y + 1));
                }
            }
        } else {
            match self.colour {
                Colour::White => {
                    //For White the row index decreases
                    ret.push((self.pos_x, self.pos_y - 1)); //Otherwise it may only move one field
                }
                Colour::Black => {
                    //For Black the row index increases
                    ret.push((self.pos_x, self.pos_y + 1)); //Otherwise it may only move one field
                }
            }
        }

        ret
    }

    fn get_queen_moves(&self) -> Vec<(u8, u8)> {
        let mut ret = vec![];

        //Queen has the moveset of bishop + rook
        ret.append(&mut self.get_bishop_moves());

        ret.append(&mut self.get_rook_moves());

        ret
    }

    fn get_rook_moves(&self) -> Vec<(u8, u8)> {
        let mut ret = vec![];

        for idx in 0..=7{//There is a max of 8 possible fields at once the bishop can move

            //North
            let new_y = self.pos_y.overflowing_sub(idx);
            if new_y.1 == false { //Only when moves are within bounds should it be valid
                if new_y.0 <= 7{
                    ret.push((self.pos_x, new_y.0));
                }
            }

            //East
            let new_x = self.pos_x.overflowing_add(idx);
            if new_x.1 == false { //Only when moves are within bounds should it be valid
                if new_x.0 <= 7 {
                    ret.push((new_x.0, self.pos_y));
                }
            }
            //South
            let new_y = self.pos_y.overflowing_add(idx);
            if new_y.1 == false { //Only when moves are within bounds should it be valid
                if new_y.0 <= 7{
                    ret.push((self.pos_x, new_y.0));
                }
            }
            //West
            let new_x = self.pos_x.overflowing_sub(idx);
            if new_x.1 == false { //Only when moves are within bounds should it be valid
                if new_x.0 <= 7 {
                    ret.push((new_x.0, self.pos_y));
                }
            }

        }

        ret
    }

    pub fn get_id(&self) -> egui::Id {
        egui::Id::new((self.get_name(), self.pos_x, self.pos_y))
    }

    // const PIECE_COUNT: u8 = 8;

    // pub fn get_white_piece_init(){
    //     let ret = [Self::PIECE_COUNT; 0];

    // }

    // pub fn get_black_piece_init(){

    // }
}
