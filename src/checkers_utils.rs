use crate::{Board, CheckersColor, Piece};
use crate::moves::Jump;

pub fn alias_from_coordinates(x: usize, y: usize) -> String {
    let letters = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
    format!("{}{}", letters[y], 8 - x)
}


pub struct MoveExecutor {

}

impl MoveExecutor {

    pub fn execute_capture<'a>(board: &'a Board, capture: &Vec<Jump>) -> &'a Board {
        let first_jump = capture.first().unwrap();
        let last_jump = capture.last().unwrap();
        let moving_piece = board.get_at(first_jump.x_start, first_jump.y_start).unwrap().unwrap();
        for jump in capture {
            let x_enemy = jump.x_capture;
            let y_enemy = jump.y_capture;
            let enemy_cell = board.get_at(x_enemy, y_enemy).unwrap().unwrap();
        }

        board
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
}

pub enum CheckersError {
    IndexOutOfBounds,
    RuleError
}