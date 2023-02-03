use crate::{piece::{PieceKind, Color, ChessPiece}, Move};
use std::fmt::{Display};
impl Board {
    pub fn move_piece(&mut self, chess_move: Move) {
        self.squares[chess_move.y2 as usize][chess_move.x2 as usize] = 
            self.squares[chess_move.y1 as usize][chess_move.x1 as usize];
        self.squares[chess_move.y1 as usize][chess_move.x1 as usize] = None;
    }
}

pub struct Board {
    squares: [[Option<ChessPiece>;8];8],
}
impl Default for Board {
    fn default() -> Self {
        let firstrank = [
            Some(ChessPiece { kind: PieceKind::Rook, color: Color::White }),
            Some(ChessPiece { kind: PieceKind::Knight, color: Color::White }),
            Some(ChessPiece { kind: PieceKind::Bishop, color: Color::White }),
            Some(ChessPiece { kind: PieceKind::Queen, color: Color::White }), 
            Some(ChessPiece { kind: PieceKind::King, color: Color::White }), 
            Some(ChessPiece { kind: PieceKind::Bishop, color: Color::White }),
            Some(ChessPiece { kind: PieceKind::Knight, color: Color::White }),
            Some(ChessPiece { kind: PieceKind::Rook, color: Color::White })];
        let eighthrank = [
            Some(ChessPiece { kind: PieceKind::Rook, color: Color::Black }),
            Some(ChessPiece { kind: PieceKind::Knight, color: Color::Black }),
            Some(ChessPiece { kind: PieceKind::Bishop, color: Color::Black }),
            Some(ChessPiece { kind: PieceKind::Queen, color: Color::Black }), 
            Some(ChessPiece { kind: PieceKind::King, color: Color::Black }), 
            Some(ChessPiece { kind: PieceKind::Bishop, color: Color::Black }),
            Some(ChessPiece { kind: PieceKind::Knight, color: Color::Black }),
            Some(ChessPiece { kind: PieceKind::Rook, color: Color::Black })];
        let secondrank = [Some(ChessPiece { kind: PieceKind::Pawn, color: Color::White }); 8];
        let seventhrank = [Some(ChessPiece { kind: PieceKind::Pawn, color: Color::Black }); 8];
        let middlerank = [None; 8];
        Self{
            squares: [firstrank, secondrank, middlerank, middlerank, middlerank, middlerank, seventhrank, eighthrank]
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..8).rev() {
            for x in 0..8 {
                // Creates chess piece in the correct spot
                let piece = self.squares[y][x];
                // If both row and column are either even or odd, then it is a light square
                let is_light_square = x % 2 != y % 2;
                //Assigns color
                if let Some(chess_piece) = piece {
                    match (is_light_square, chess_piece.color) {
                        (true, Color::White) => write!(f, "\x1b[31;47m")?,
                        (true, Color::Black) => write!(f, "\x1b[30;47m")?,
                        (false, Color::White) => write!(f,"\x1b[31;42m")?,
                        (false, Color::Black) => write!(f,"\x1b[30;42m")?,
                    }
                    write!(f," {} ", chess_piece.kind.emoji())?;
                } else {
                    match is_light_square {
                        true => write!(f,"\x1b[47m")?,
                        false => write!(f,"\x1b[42m")?,
                    }
                    write!(f,"   ")?;
                }
            }
            writeln!(f, "\x1b[39;49m")?;
        }
        Ok(())
    }
}

// ♟︎ ♞ ♝ ♜ ♛ ♚