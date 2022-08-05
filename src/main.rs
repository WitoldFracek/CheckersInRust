mod colors;
mod pieces;
mod board;
mod checkers_utils;
mod moves;
mod tests;
mod players;
mod checkers_game;

use colors::colors as col;
use pieces::{Piece, CheckersColor};
use crate::board::Board;
use crate::CheckersColor::{White, Black};
use crate::Piece::{Pawn, Queen};
use crate::checkers_utils::{alias_from_coordinates, MoveExecutor};
use crate::moves::SimpleMove;
use crate::players::{Human, Player};

fn main() {

}
