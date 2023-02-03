#[allow(unused)]
#[derive(Copy, Clone, Debug)]
pub enum PieceKind {
    Pawn, Knight, Bishop, Rook, Queen, King
}

impl PieceKind {
    pub fn emoji(self) -> &'static str {
        match self {
            PieceKind::Pawn => "♟︎",
            PieceKind::Knight => "♞",
            PieceKind::Bishop => "♝",
            PieceKind::Rook => "♜", 
            PieceKind::Queen => "♛", 
            PieceKind::King => "♚"
        }

    }
}
#[derive(Copy, Clone, Debug)]
pub enum Color {
    Black, White
}
#[derive(Copy, Clone, Debug)]
pub struct ChessPiece {
    pub kind: PieceKind,
    pub color: Color
}