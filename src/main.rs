mod colors;
mod pieces;
mod board;

use colors::colors as col;
use pieces::{Piece, CheckersColor};
use crate::board::Board;
use crate::CheckersColor::{White, Black};
use crate::Piece::{Pawn, Queen};

fn main() {
    // for cell in Board::test().into_iter() {
    //     println!("{}", cell);
    // }
    println!("{}", Board::test().repr());

    println!("{}Witek{}", col::FG::color(147, 1, 211), col::END);
    println!("{}Witek{}", col::BG::color(147, 1, 211), col::END);
}
