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

    // let mut b = Board::new(3).unwrap();
    // let p = MoveExecutor::get_pieces(&b, CheckersColor::Black);
    // println!("{}", b.repr());
    // let _ = b.set_at(7, 0, Board::BLACK_QUEEN);
    // let _ = b.set_at(0, 1, Board::WHITE_PAWN);
    // let _ = b.set_at(0, 3, Board::WHITE_QUEEN);
    // let _ = b.set_at(4, 3, Board::BLACK_PAWN);
    // let _ = b.set_at(1, 2, Board::EMPTY);
    // println!("{}", b.repr());
    // println!("{}", b.get_at(4, 3).unwrap().unwrap());
}
