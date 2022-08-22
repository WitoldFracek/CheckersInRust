use std::fmt::{Display, Formatter};
use crate::Board;
use crate::colors::colors as col;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CheckersColor {
    White,
    Black,
}

impl CheckersColor {
    pub fn opposite_color(&self) -> Self {
        match self {
            CheckersColor::White => CheckersColor::Black,
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
            Piece::Pawn(_) => "●".to_string(),
            Piece::Queen(_) => "Q".to_string(),
        }
    }

    pub fn colored_marker(&self, has_end: bool) -> String {
        match self {
            Piece::Pawn(CheckersColor::White) => col::colored_text("●", col::fg::WHITE, col::NONE, has_end),
            Piece::Pawn(CheckersColor::Black) => col::colored_text("●", col::fg::BLACK, col::NONE, has_end),
            Piece::Queen(CheckersColor::White) => col::colored_text("Q", col::fg::WHITE, col::NONE, has_end),
            Piece::Queen(CheckersColor::Black) => col::colored_text("Q", col::fg::BLACK, col::NONE, has_end),
        }
    }

    pub fn color(&self) -> CheckersColor {
        match self {
            Piece::Pawn(color) => *color,
            Piece::Queen(color) => *color,

        }
    }

    pub fn board_u8_representation(&self) -> u8 {
        match self {
            Piece::Pawn(CheckersColor::White) => Board::WHITE_PAWN,
            Piece::Queen(CheckersColor::White) => Board::WHITE_QUEEN,
            Piece::Pawn(CheckersColor::Black) => Board::BLACK_PAWN,
            Piece::Queen(CheckersColor::Black) => Board::BLACK_QUEEN,
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.colored_marker(true))
    }
}


