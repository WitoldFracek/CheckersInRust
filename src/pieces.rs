use std::fmt::{Display, Formatter};
use crate::Board;
use crate::col::RGBColor;
use crate::colors::colors as col;

#[derive(Copy, Clone, PartialEq)]
pub enum CheckersColor {
    White,
    Black,
}

impl CheckersColor {
    // pub fn opposite_color(&self) -> Self {
    //     match self {
    //         CheckersColor::White(color) => CheckersColor::Black(255 - r, 255 - g, 255 - b),
    //         CheckersColor::Black(r, g, b) => CheckersColor::White(255 - r, 255 - g, 255 - b),
    //     }
    // }
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
            Piece::Pawn(CheckersColor::White) => CheckersColor::White,
            Piece::Queen(CheckersColor::White) => CheckersColor::White,
            Piece::Pawn(CheckersColor::Black) => CheckersColor::Black,
            Piece::Queen(CheckersColor::Black) => CheckersColor::Black,

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


