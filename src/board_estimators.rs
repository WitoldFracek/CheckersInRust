use itertools::max;
use crate::{Board, CheckersColor, MoveExecutor, Piece};
use crate::board::Cell;

pub trait Estimator {
    fn estimate(&self, board: Board, maximising_color: CheckersColor, check_for_endgame: bool) -> i32;
}

pub struct CountEstimator {
    pub pawn_weight: usize,
    pub queen_weight: usize,
}

impl CountEstimator {
    pub fn new(pawn_weight: usize, queen_weight: usize) -> Self {
        Self {
            pawn_weight,
            queen_weight,
        }
    }
}

impl Estimator for CountEstimator {
    fn estimate(&self, board: Board, maximising_color: CheckersColor, check_for_endgame: bool) -> i32 {
        if check_for_endgame {
            if MoveExecutor::has_game_ended(board, CheckersColor::White) {
                return match maximising_color {
                    CheckersColor::White => i32::MIN,
                    CheckersColor::Black => i32::MAX,
                }
            } else if MoveExecutor::has_game_ended(board, CheckersColor::Black) {
                return match maximising_color {
                    CheckersColor::White => i32::MAX,
                    CheckersColor::Black => i32::MIN,
                }
            }
        }
        let mut score = 0;
        for cell in &board {
            if cell.is_empty() {
                continue;
            }
            match cell.piece.unwrap() {
                Piece::Pawn(CheckersColor::White) => score += self.pawn_weight as i32,
                Piece::Pawn(CheckersColor::Black) => score -= self.pawn_weight as i32,
                Piece::Queen(CheckersColor::White) => score += self.queen_weight as i32,
                Piece::Queen(CheckersColor::Black) => score -= self.queen_weight as i32,
            }
        }
        score
    }
}