use array2d::Array2D;

use crate::piece::*;
// use crate::piece::Colour;


const BOARD_HEIGHT: usize = 8;
const BOARD_WIDTH: usize = 8;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Board {
    pub grid: [[Option<Piece>; BOARD_HEIGHT]; BOARD_WIDTH] //Using arrays instead of Vec since arrays are on the stack and; therefore, much faster
}



impl Board {


    pub fn init_board_state() -> Self{ 

        let mut state: [[Option<Piece>; BOARD_HEIGHT]; BOARD_WIDTH] = [[None; BOARD_HEIGHT]; BOARD_WIDTH];

        //To avoid immense overhead caused by bounds checking the array is sliced into it's sides first and initialised that way

        
        //This is ugly af but it avoids the overhead of array boundary checking
        let mut iter = state[BOARD_HEIGHT-1..=BOARD_HEIGHT-1].iter_mut().next().unwrap().iter_mut();
        *iter.next().unwrap() = Some(Piece::new(Colour::White, PieceType::Rook, 0.0, 7.0));
        *iter.next().unwrap() = Some(Piece::new(Colour::White, PieceType::Knight, 1.0, 7.0));
        *iter.next().unwrap() = Some(Piece::new(Colour::White, PieceType::Bishop, 2.0, 7.0));
        *iter.next().unwrap() = Some(Piece::new(Colour::White, PieceType::Queen, 3.0, 7.0));
        *iter.next().unwrap() = Some(Piece::new(Colour::White, PieceType::King, 4.0, 7.0));
        *iter.next().unwrap() = Some(Piece::new(Colour::White, PieceType::Bishop, 5.0, 7.0));
        *iter.next().unwrap() = Some(Piece::new(Colour::White, PieceType::Knight, 6.0, 7.0));
        *iter.next().unwrap() = Some(Piece::new(Colour::White, PieceType::Rook, 7.0, 7.0));

       
        let mut iter = state[0..=0].iter_mut().next().unwrap().iter_mut(); //Chess Board Row 8 Black Pieces
        *iter.next().unwrap() = Some(Piece::new(Colour::Black, PieceType::Rook, 0.0, 0.0));
        *iter.next().unwrap() = Some(Piece::new(Colour::Black, PieceType::Knight, 1.0, 0.0));
        *iter.next().unwrap() = Some(Piece::new(Colour::Black, PieceType::Bishop, 2.0, 0.0));
        *iter.next().unwrap() = Some(Piece::new(Colour::Black, PieceType::Queen, 3.0, 0.0));
        *iter.next().unwrap() = Some(Piece::new(Colour::Black, PieceType::King, 4.0, 0.0));
        *iter.next().unwrap() = Some(Piece::new(Colour::Black, PieceType::Bishop, 5.0, 0.0));
        *iter.next().unwrap() = Some(Piece::new(Colour::Black, PieceType::Knight, 6.0, 0.0));
        *iter.next().unwrap() = Some(Piece::new(Colour::Black, PieceType::Rook, 7.0, 0.0));

        

        //This is much prettier and does the same; however, it can only be done for pawns as they are all the same
        let mut temp: f32 = 0.0;
        state[BOARD_HEIGHT - 2..=BOARD_HEIGHT - 2].iter_mut().for_each(|row|{//Chess Board Row 2 White Pawns
            row.iter_mut().for_each(|piece|{
                *piece = Some(Piece::new(Colour::White, PieceType::Pawn, temp as f32, (BOARD_HEIGHT - 2) as f32));
                temp += 1.0;
            });
        });

        temp = 0.0;
        state[1..=1].iter_mut().for_each(|row|{ //Chess Board Row 7 Black Pawns
            row.iter_mut().for_each(|piece|{
                *piece = Some(Piece::new(Colour::Black, PieceType::Pawn, temp as f32, 1 as f32));
                temp += 1.0;
            });
        });


    





        // state.iter().for_each(|&row| {
        //     row.iter().for_each(|element|{
        //         print!("{:#?}", element);
        //     });
        //     println!("");
        // });

        Self {  
            grid: state
        }
    }



    pub fn get_board_state (&self){
        
    }
}