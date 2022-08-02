mod colors;
mod pieces;
mod board;
mod checkers_utils;
mod moves;
mod tests;

use colors::colors as col;
use pieces::{Piece, CheckersColor};
use crate::board::Board;
use crate::CheckersColor::{White, Black};
use crate::Piece::{Pawn, Queen};
use crate::checkers_utils::{alias_from_coordinates, MoveExecutor};

fn main() {

    let mut board = Board::from_mockup([
            ["  ", "  ", "  ", "WQ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "BP", "  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  ", "BP", "  ", "  "],
            ["  ", "  ", "BP", "  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  ", "  ", "BP", "  "],
            ["  ", "  ", "  ", "BP", "  ", "  ", "  ", "  "],
            ["  ", "  ", "  ", "  ", "  ", "  ", "  ", "  "]]);
    println!("{:?}", board.is_field_excluded(3, 0));
    println!("{:?}", board.bit_repr(3, 0));
}
