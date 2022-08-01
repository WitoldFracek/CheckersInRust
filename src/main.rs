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

    let b = Board::from_mockup([["", "WP", "", "BQ", "", "", "BQ", "WQ"],
        ["WQ", "", "", "BP", "", "BQ", "WQ", ""],
        ["", "", "", "", "", "", "", ""],
        ["", "", "", "", "", "", "", ""],
        ["", "", "", "", "", "", "", ""],
        ["", "", "", "", "", "", "", ""],
        ["", "", "", "", "", "", "", ""],
        ["", "", "", "", "", "", "", ""],
    ]);

    let a = 3_usize;
    let d = 1;
    let p = a - 2 * d;
    println!("{}", p);

    println!("{}", b.repr());
}
