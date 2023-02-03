use std::{io::stdin};
use board::Board;
mod board;
mod piece;

// TODO!
// (hard) Implement move validation for moves.
// (hard) Use actual pgn notation where you can leave out information

pub struct Move{x1:u8, y1:u8, x2:u8, y2:u8}
impl Move {
    fn read_move() -> Self {
        let mut input = String::default();
        loop {
            stdin().read_line(&mut input).unwrap();
            let chess_move = Move::parse_move(&input);
            match chess_move {
                Ok(chess_move) => return chess_move,
                Err(error_message) => println!("{}", error_message)
            }
            input.clear()
        }
    }
    fn parse_move(input: &str) -> Result<Move, &'static str> {
        let mut chars = input.chars();
        let x1 = {
            match chars.next() {
                Some(char) => char,
                None => return Err("Oopsie: Must provide a move".into())
            }
        };

        let y1 = chars.next().ok_or("Oopsie daisies! Your move should have more characters. Come on, do better. Not cool.")?;
        let x2 = chars.next().ok_or("Oopsie daisies! Your move should have more characters. Come on, do better. Not cool.")?;
        let y2 = chars.next().ok_or("Oopsie daisies! Your move should have more characters. Come on, do better. Not cool.")?;
// x1 = b, y1 = l, x2 = e, y2 = e
        let x1 = x1 as i8 - 'a' as i8;
        let y1 = y1 as i8 - '1' as i8;
        let x2 = x2 as i8 - 'a' as i8;
        let y2 = y2 as i8 - '1' as i8 ;
        if 0 > x1 || x1 > 7 || 0 > x2 || x2 > 7 {
            return Err("Oopsie: You screwed up. Provide a valid file")
        }
        if 0 > y1 || y1 > 7 || 0 > y2 || y2 > 7 {
            return Err("Oopsie: You screwed up. Provide a valid rank")
        }


        
        Ok(Self {
            x1:x1 as u8, y1:y1 as u8, x2:x2 as u8, y2:y2 as u8
        })
    }
}

fn main() {
    // println!("♟︎ ♞ {} ♜ ♛ ♚", ChessPiece::Bishop.emoji());
    let mut board = Board::default();
    println!("{}",board);
    loop {
        let chess_move = Move::read_move();
        board.move_piece(chess_move);
        println!("{}",board);
    }
        // loop {
        //     if let Some(char ) = chars.next() {
        //         println!("{}", char);
        //     }
        //     else {
        //         break;
        //     }
        // }
//     print!("\n{}", board);
//     board.move_piece('e', '7', 'e', '5');
//     print!("\n{}", board);
//     board.move_piece('g', '1', 'f', '3');
//     print!("\n{}", board); 
}
// Regular expression syntax: Find Some\(PieceKind::([a-zA-Z]*)\), Replace Some(ChessPiece { kind: PieceKind::$1, color: Color::White })
