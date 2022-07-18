use crate::colors::colors as col;

#[derive(Copy, Clone)]
pub enum CheckersColor {
    White,
    Black
}

impl CheckersColor {
    pub fn opposite_color(&self) -> Self {
        match self {
            CheckersColor::White=> CheckersColor::Black,
            CheckersColor::Black => CheckersColor::White,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Piece {
    Pawn(CheckersColor),
    Queen(CheckersColor),
}

impl Piece {
    pub fn marker(&self) -> String {
        match self {
            Piece::Pawn(CheckersColor::White) => col::colored_text("●", col::FG::WHITE, col::NONE),
            Piece::Pawn(CheckersColor::Black) => col::colored_text("●", col::FG::BLACK, col::NONE),
            Piece::Queen(CheckersColor::White) => col::colored_text("Q", col::FG::WHITE, col::NONE),
            Piece::Queen(CheckersColor::Black) => col::colored_text("Q", col::FG::BLACK, col::NONE),
        }
    }
}


