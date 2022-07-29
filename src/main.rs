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

    let v = vec![1, 2, 3, 4, 5, 6];
    for elem in &v[..v.len() - 1] {
        println!("{}", elem);
        if elem % 3 == 0 {
            continue;
        }
        println!("{}", elem);
    }
}
