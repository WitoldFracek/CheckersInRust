use std::cmp::max;
use std::future::pending;
use crate::{Board, CheckersColor, Piece};
use crate::moves::{Jump, Move, SimpleMove};

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

    pub fn get_pieces(board: &Board, color: CheckersColor) -> Vec<(usize, usize)> {
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

    pub fn get_capturing_pieces(board: &Board, pieces: Vec<(usize, usize)>, color: CheckersColor) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
        let mut cap_pawns = Vec::new();
        let mut cap_queens = Vec::new();
        for (x, y) in pieces {
            match board.get_at(x, y).unwrap() {
                None => {}
                Some(Piece::Pawn(_)) => {
                    if Self::can_pawn_capture(board, (x, y), color) {
                        cap_pawns.push((x, y));
                    }
                },
                Some(Piece::Queen(_)) => {
                    if Self::can_queen_capture(board, (x, y), color) {
                        cap_queens.push((x, y));
                    }
                }
            }
        }
        (cap_pawns, cap_queens)
    }

    fn get_moving_pieces(board: &Board, pieces: Vec<(usize, usize)>, color: CheckersColor) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
        let mut mov_pawns = Vec::new();
        let mut mov_queens = Vec::new();
        for (x, y) in pieces {
            match board.get_at(x, y).unwrap() {
                None => {}
                Some(Piece::Pawn(_)) => {
                    if Self::can_pawn_move(board, (x, y), color) {
                        mov_pawns.push((x, y));
                    }
                }
                Some(Piece::Queen(_)) => {
                    if Self::can_queen_move(board, (x, y), color) {
                        mov_queens.push((x, y));
                    }
                }
            }
        }
        (mov_pawns, mov_queens)
    }

    fn get_all_captures(board: &Board, color: CheckersColor) -> Vec<Vec<Jump>> {
        let pieces = Self::get_pieces(board, color);
        let (capturing_pawns, capturing_queens) = Self::get_capturing_pieces(board, pieces, color);
        let mut pawn_captures = Self::get_possible_pawn_captures(board, capturing_pawns, color);
        let mut queen_captures = Self::get_possible_queen_captures(board, capturing_queens, color);
        let mut all_captures = Vec::new();
        all_captures.append(&mut pawn_captures);
        all_captures.append(&mut queen_captures);
        all_captures
    }

    pub fn get_possible_pawn_captures(board: &Board, capturing_pawns: Vec<(usize, usize)>, color: CheckersColor) -> Vec<Vec<Jump>> {
        let mut paths = Vec::new();
        for (x, y) in capturing_pawns {
            let mut board_copy = board.clone();
            board_copy.set_at(x, y, Board::EMPTY).unwrap();
            let mut pawn_path = Vec::new();
            Self::get_pawn_capture_path(board, (x, y), color, &mut Vec::new(), &mut pawn_path);
            paths.append(&mut pawn_path);
        }
        if paths.is_empty() {
            return paths;
        }
        let max_len = paths.iter().map(|v| v.len()).max().unwrap();
        paths.retain(|v| v.len() == max_len);
        paths
    }

    fn get_pawn_capture_path(board: &Board, pawn: (usize, usize), color: CheckersColor, acc: &mut Vec<Jump>, solutions: &mut Vec<Vec<Jump>>) {
        if Self::can_pawn_capture(board, pawn, color) {
            let directions = Self::get_pawn_capture_directions(board, pawn, color);
            for (dx, dy) in directions {
                let (x_start, y_start) = pawn;
                let jump = Jump {
                    x_start,
                    y_start,
                    x_end: (x_start as i32 + 2 * dx) as usize,
                    y_end: (y_start as i32 + 2 * dx) as usize,
                    x_capture: (x_start as i32 + dx) as usize,
                    y_capture: (y_start as i32 + dx) as usize,
                };
                acc.push(jump);
                let mut board_copy = board.clone();
                let _ = board_copy.set_field_excluded(jump.x_capture, jump.y_capture);
                Self::get_pawn_capture_path(&board_copy, jump.end_pair(), color, &mut acc.to_vec(), solutions)
            }
        } else {
            solutions.push(acc.to_vec());
        }
    }

    fn get_pawn_capture_directions(board: &Board, pawn: (usize, usize), color: CheckersColor) -> Vec<(i32, i32)> {
        let mut ret = Vec::new();
        for direction in Self::DIRECTIONS {
            if Self::is_pawn_jump_possible(board, pawn, direction, color) {
                ret.push(direction);
            }
        }
        ret
    }

    pub fn get_possible_queen_captures(board: &Board, capturing_queens: Vec<(usize, usize)>, color: CheckersColor) -> Vec<Vec<Jump>> {
        let mut paths = Vec::new();
        for (x, y) in capturing_queens {
            let mut board_copy = board.clone();
            let _ = board_copy.set_at(x, y, Board::EMPTY);
            let mut queen_path = Vec::new();
            Self::get_queen_capture_path(board, (x, y), color, &mut Vec::new(), &mut queen_path);
            // println!("Q path: {:?}", queen_path);
            paths.append(&mut queen_path);
        }
        if paths.is_empty() {
            return paths;
        }
        let max_len = paths.iter().map(|v| v.len()).max().unwrap();
        paths.retain(|v| v.len() == max_len);
        paths
    }

    fn get_queen_capture_path(board: &Board, queen: (usize, usize), color: CheckersColor, acc: &mut Vec<Jump>, solutions: &mut Vec<Vec<Jump>>) {
        if Self::can_queen_capture(board, queen, color) {
            let landing_spots = Self::get_queen_landing_spots(board, queen, color);
            return
            // for jump in landing_spots {
            //     let x_enemy = jump.x_capture;
            //     let y_enemy = jump.y_capture;
            //     let mut board_copy = board.clone();
            //     let _ = board_copy.set_field_excluded(x_enemy, y_enemy);
            //     let mut acc_copy = acc.to_vec();
            //     println!("{:?}", acc_copy);
            //     acc_copy.push(jump);
            //     Self::get_queen_capture_path(&board_copy, jump.end_pair(), color, &mut acc_copy, solutions);
            // }
        } else {
            solutions.push(acc.to_vec());
        }
    }

    fn get_queen_landing_spots(board: &Board, queen: (usize, usize), color: CheckersColor) -> Vec<Jump> {
        let mut landing_spots = Vec::new();
        let (x, y) = queen;
        for direction in Self::DIRECTIONS {
            let diagonal = Self::diagonal(board, queen, direction);
            let mut enemy_index = -1_i32;
            let mut obstacle_index = -1_i32;
            for (i, &(x_pos, y_pos)) in diagonal.iter().enumerate() {
                if obstacle_index != -1 {
                    break;
                }
                if board.is_field_excluded(x_pos, y_pos).unwrap() {
                    continue;
                }
                if !board.is_empty_at(x_pos, y_pos).unwrap() {
                    let piece = board.get_at(x_pos, y_pos).unwrap().unwrap();
                    if piece.color() != color {
                        if enemy_index == -1 {
                            enemy_index = i as i32;
                        } else {
                            obstacle_index = i as i32;
                        }
                    } else {
                        obstacle_index = i as i32;
                    }
                } else if enemy_index != -1 {
                    let (ex, ey) = diagonal[enemy_index as usize];
                    landing_spots.push(Jump::new(x, y, x_pos, y_pos, ex, ey));
                }
            }
        }
        landing_spots
    }

    fn get_all_moves(board: &Board, color: CheckersColor) -> Vec<SimpleMove> {
        let pieces = Self::get_pieces(board, color);
        let (moving_pawns, moving_queens) = Self::get_moving_pieces(board, pieces, color);
        Vec::new()
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
        x >= 0 && x < 8 && y >= 0 && y < 8
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