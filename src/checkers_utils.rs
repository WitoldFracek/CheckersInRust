use crate::{Board, CheckersColor, Piece};
use crate::moves::{Jump, SimpleMove};

pub fn alias_from_coordinates(x: usize, y: usize) -> String {
    let letters = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
    format!("{}{}", letters[y], 8 - x)
}


pub struct MoveExecutor {

}

impl MoveExecutor {
    
    const DIRECTIONS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

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
    fn can_pawn_capture(board: &Board, pawn: (usize, usize), current_color: CheckersColor) -> bool {
        for direction in Self::DIRECTIONS {
            if Self::is_pawn_jump_possible(board, pawn, direction, current_color) {
                return true;
            }
        }
        false
    }

    fn is_pawn_jump_possible(board: &Board, pawn: (usize, usize), direction: (i32, i32), current_color: CheckersColor) -> bool {
        let ((x, y), (dx, dy)) = (pawn, direction);
        if !Self::is_in_bounds(x as i32 + dx, y as i32 + dy) {
            return false;
        }
        if !Self::is_in_bounds(x as i32 + 2 * dx, y as i32 + 2 * dy) {
            return false;
        }
        let x_capture = (x as i32 + dx) as usize;
        let y_capture = (y as i32 + dy) as usize;
        if board.is_field_excluded(x_capture, y_capture).unwrap() {
            return false;
        }
        if board.is_empty_at(x_capture, y_capture).unwrap() {
            return false;
        }
        if board.get_at(x_capture, y_capture).unwrap().unwrap().color() == current_color {
            return false;
        }
        match board.is_empty_at((x as i32 + 2 * dx) as usize, (y as i32 + 2 * dy) as usize) {
            Ok(b) => b,
            _ => false
        }
    }

    fn can_queen_capture(board: &Board, queen: (usize, usize), current_color: CheckersColor) -> bool {
        for direction in Self::DIRECTIONS {
            if Self::is_queen_jump_possible(board, queen, direction, current_color) {
                return true;
            }
        }
        false
    }

    fn is_queen_jump_possible(board: &Board, queen: (usize, usize), direction: (i32, i32), current_color: CheckersColor) -> bool {
        let (dx, dy) = direction;
        let diagonal = Self::diagonal(board, queen, direction);
        if diagonal.len() < 2 {
            return false;
        }
        for &(x_pos, y_pos) in &diagonal[..diagonal.len() - 1] {
            if board.is_field_excluded(x_pos, y_pos).unwrap() {
                return false;
            }
            if board.is_empty_at(x_pos, y_pos).unwrap() {
                continue;
            }
            if board.is_empty_at((x_pos as i32 + dx) as usize, (y_pos as i32 + dy) as usize).unwrap() {
                if board.get_at(x_pos, y_pos).unwrap().unwrap().color() != current_color {
                    return true;
                }
            }
            return false;
        }
        false
    }

    fn can_pawn_move(board: &Board, pawn: (usize, usize), current_color: CheckersColor) -> bool {
        match current_color {
            CheckersColor::White => Self::is_move_possible(board, pawn, (-1, -1)) || Self::is_move_possible(board, pawn, (-1, 1)),
            CheckersColor::Black => Self::is_move_possible(board, pawn, (1, -1)) || Self::is_move_possible(board, pawn, (1, 1)),
        }
    }

    fn can_queen_move(board: &Board, queen: (usize, usize), current_color: CheckersColor) -> bool {
        for direction in Self::DIRECTIONS {
            if Self::is_move_possible(board, queen, direction) {
                return true;
            }
        }
        false
    }

    fn is_move_possible(board: &Board, piece: (usize, usize), direction: (i32, i32)) -> bool {
        let (dx, dy) = direction;
        let (x, y) = piece;
        let (x_to, y_to) = (x as i32 + dx, y as i32 + dy);
        if Self::is_in_bounds(x_to, y_to) {
            return board.is_empty_at(x_to as usize, y_to as usize).unwrap();
        }
        false
    }

    fn is_in_bounds(x: i32, y: i32) -> bool {
        x > 0 && x < 8 && y > 0 && y < 8
    }

    fn diagonal(board: &Board, queen: (usize, usize), direction: (i32, i32)) -> Vec<(usize, usize)> {
        let mut ret: Vec<(usize, usize)> = Vec::new();
        let (dx, dy) = direction;
        let (x, y) = queen;
        for i in 1..board.size() as i32 {
            let (x_cal, y_cal) = (x as i32 + i * dx, y as i32 + i * dy);
            if Self::is_in_bounds(x_cal, y_cal) {
                ret.push((x_cal as usize, y_cal as usize));
            }
        }
        ret
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CheckersError {
    IndexOutOfBounds,
    RuleError,
    PawnBinaryValueError
}