use crate::{Board, CheckersColor, Piece};
use crate::moves::{Jump, SimpleMove};

pub fn alias_from_coordinates(x: usize, y: usize) -> String {
    let letters = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
    format!("{}{}", letters[y], 8 - x)
}


pub struct MoveExecutor {

}

impl MoveExecutor {

    pub fn execute_capture(board: &Board, capture: &Vec<Jump>) -> Board {
        let mut ret = board.clone();
        let first_jump = capture.first().unwrap();
        let last_jump = capture.last().unwrap();
        let moving_piece = Self::get_moving_piece(&ret, first_jump.x_start, first_jump.y_start);
        let _ = ret.set_at(first_jump.x_start, first_jump.y_start, Board::EMPTY);
        for jump in capture {
            let x_enemy = jump.x_capture;
            let y_enemy = jump.y_capture;
            let _ = ret.set_at(x_enemy, y_enemy, Board::EMPTY);
        }
        let _ = ret.set_at(last_jump.x_end, last_jump.y_end, moving_piece);
        ret
    }

    pub fn execute_move(board: Board, one_move: SimpleMove) -> Board {
        let mut ret = board.clone();
        let moving_piece = Self::get_moving_piece(&ret, one_move.x_start, one_move.y_start);
        let _ = ret.set_at(one_move.x_start, one_move.y_start, Board::EMPTY);
        let _ = ret.set_at(one_move.x_end, one_move.y_end, moving_piece);
        ret
    }

    fn get_moving_piece(board: &Board, x: usize, y: usize) -> u8 {
        match board.get_at(x, y).unwrap().unwrap() {
            Piece::Pawn(CheckersColor::White) => Board::WHITE_PAWN,
            Piece::Queen(CheckersColor::White) => Board::WHITE_QUEEN,
            Piece::Pawn(CheckersColor::Black) => Board::BLACK_PAWN,
            Piece::Queen(CheckersColor::Black) => Board::BLACK_QUEEN,
        }
    }

    fn get_pieces(board: &Board, color: CheckersColor) -> Vec<(usize, usize)> {
        let mut coordinates: Vec<(usize, usize)> = Vec::new();
        let mut counter = 0;
        for cell in board {
            match cell.piece {
                Some(Piece::Pawn(piece_color)) | Some(Piece::Queen(piece_color)) => {
                    if piece_color == color {
                        coordinates.push((counter / 8, counter % 8));
                    }
                },
                None => {}
            }
            counter += 1;
        }

        coordinates
    }

    // === checks ===
    // fn can_pawn_capture(board: Board, pawn: (usize, usize), current_color: CheckersColor, excluded_cells)
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CheckersError {
    IndexOutOfBounds,
    RuleError,
    PawnBinaryValueError
}