mod colors;
mod pieces;
mod board;
mod checkers_utils;

use colors::colors as col;
use pieces::{Piece, CheckersColor};
use crate::board::Board;
use crate::CheckersColor::{White, Black};
use crate::Piece::{Pawn, Queen};
use crate::checkers_utils::alias_from_coordinates;

fn main() {
    // for cell in Board::test().into_iter() {
    //     println!("{}", cell);
    // }
    let board = Board::new(2).unwrap();
    println!("{}", board.repr());
    match board.get_at(0, 0).unwrap() {
        Some(piece) => println!("{}", piece),
        None => {}
    };

    println!("{}", alias_from_coordinates(0, 0));

    // println!("{}", Board::new(1).unwrap().repr());

    // println!("{}Witek{}", col::FG::color(147, 1, 211), col::END);
    // println!("{}Witek{}", col::BG::color(147, 1, 211), col::END);
}
