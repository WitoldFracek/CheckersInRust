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
mod statistics;

use colors::colors as col;
use pieces::{Piece, CheckersColor};
use crate::board::Board;
use crate::board_estimators::CountEstimator;
use crate::checkers_game::Game;
use crate::CheckersColor::{White, Black};
use crate::Piece::{Pawn, Queen};
use crate::checkers_utils::{alias_from_coordinates, MoveExecutor};
use crate::moves::SimpleMove;
use crate::players::{DummyBot, Human, MinMaxBot, Player};

fn main() {
    let mut bot = DummyBot::new("Richard", CheckersColor::White);
    let count_estimator = CountEstimator::new(1, 3);
    let mut minmax = MinMaxBot::new("MinMax", CheckersColor::White, 7, &count_estimator);
    let mut human = Human::new("Witek", CheckersColor::Black);
    let mut game = Game::new_with_bots(&mut human, &mut minmax, 1, 2);
    game.play();
}
