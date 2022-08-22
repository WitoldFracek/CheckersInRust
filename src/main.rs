extern crate core;

mod colors;
mod pieces;
mod board;
mod checkers_utils;
mod moves;
mod tests;
mod players;
mod checkers_game;
mod board_estimators;

use colors::colors as col;
use pieces::{Piece, CheckersColor};
use crate::board::Board;
use crate::checkers_game::Game;
use crate::CheckersColor::{White, Black};
use crate::Piece::{Pawn, Queen};
use crate::checkers_utils::{alias_from_coordinates, MoveExecutor};
use crate::moves::SimpleMove;
use crate::players::{DummyBot, Human, Player};

fn main() {
    let mut bot = DummyBot::new("Richard");
    let mut human = Human::new("Witek");
    let mut game = Game::new_with_bots(&mut human, &mut bot, 1, 2);
    //game.play();
}
