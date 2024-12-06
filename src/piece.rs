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
pub enum Colour{
    White,
    Black
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug)]
pub struct Piece {
    colour: Colour,
    piece_type: PieceType,
    pos_x: f32, 
    pos_y: f32,
}

impl Piece {
    pub fn new (colour: Colour, piece_type: PieceType, pos_x: f32, pos_y: f32) -> Self {
        Self{
            colour: colour,
            piece_type: piece_type, 
            pos_x: pos_x,
            pos_y: pos_y,
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

    pub fn get_image(&self) -> egui::widgets::Image<'_>{
        match self.colour {
            Colour::White => {
                match self.piece_type {
                    PieceType::Bishop => egui::widgets::Image::from_bytes("../assets/Casual/Pieces/Chess_white_casual/Bishop.png", include_bytes!("../assets/Casual/Pieces/Chess_white_casual/Bishop.png")).maintain_aspect_ratio(true),
                    PieceType::King => egui::widgets::Image::from_bytes("../assets/Casual/Pieces/Chess_white_casual/King.png", include_bytes!("../assets/Casual/Pieces/Chess_white_casual/King.png")).maintain_aspect_ratio(true),
                    PieceType::Knight => egui::widgets::Image::from_bytes("../assets/Casual/Pieces/Chess_white_casual/Knight.png", include_bytes!("../assets/Casual/Pieces/Chess_white_casual/Knight.png")).maintain_aspect_ratio(true),
                    PieceType::Pawn => egui::widgets::Image::from_bytes("../assets/Casual/Pieces/Chess_white_casual/Pawn.png", include_bytes!("../assets/Casual/Pieces/Chess_white_casual/Pawn.png")).maintain_aspect_ratio(true),
                    PieceType::Queen => egui::widgets::Image::from_bytes("../assets/Casual/Pieces/Chess_white_casual/Queen.png", include_bytes!("../assets/Casual/Pieces/Chess_white_casual/Queen.png")).maintain_aspect_ratio(true),
                    PieceType::Rook => egui::widgets::Image::from_bytes("../assets/Casual/Pieces/Chess_white_casual/Rook.png", include_bytes!("../assets/Casual/Pieces/Chess_white_casual/Rook.png")).maintain_aspect_ratio(true),
                }
            },
            Colour::Black => {
                match self.piece_type {
                    PieceType::Bishop => egui::widgets::Image::from_bytes("../assets/Casual/Pieces/Chess_black_casual/Bishop.png", include_bytes!("../assets/Casual/Pieces/Chess_black_casual/Bishop.png")).maintain_aspect_ratio(true),
                    PieceType::King => egui::widgets::Image::from_bytes("../assets/Casual/Pieces/Chess_black_casual/King.png", include_bytes!("../assets/Casual/Pieces/Chess_black_casual/King.png")).maintain_aspect_ratio(true),
                    PieceType::Knight => egui::widgets::Image::from_bytes("../assets/Casual/Pieces/Chess_black_casual/Knight.png", include_bytes!("../assets/Casual/Pieces/Chess_black_casual/Knight.png")).maintain_aspect_ratio(true),
                    PieceType::Pawn => egui::widgets::Image::from_bytes("../assets/Casual/Pieces/Chess_black_casual/Pawn.png", include_bytes!("../assets/Casual/Pieces/Chess_black_casual/Pawn.png")).maintain_aspect_ratio(true),
                    PieceType::Queen => egui::widgets::Image::from_bytes("../assets/Casual/Pieces/Chess_black_casual/Queen.png", include_bytes!("../assets/Casual/Pieces/Chess_black_casual/Queen.png")).maintain_aspect_ratio(true),
                    PieceType::Rook => egui::widgets::Image::from_bytes("../assets/Casual/Pieces/Chess_black_casual/Rook.png", include_bytes!("../assets/Casual/Pieces/Chess_black_casual/Rook.png")).maintain_aspect_ratio(true),
                }
            },
        }
    }


    // const PIECE_COUNT: u8 = 8;

    // pub fn get_white_piece_init(){
    //     let ret = [Self::PIECE_COUNT; 0];
        
    // }

    // pub fn get_black_piece_init(){
        
    // }

}