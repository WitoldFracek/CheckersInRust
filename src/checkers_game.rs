use std::cmp::min;
use crate::{Board, CheckersColor, MoveExecutor, Player};
use crate::checkers_utils::CheckersError;
use crate::moves::{Jump, Move};

pub struct Game<'a, P: Player> {
    pub player1: &'a P,
    pub player2: &'a P,
    pub pawn_rows: usize,
    pub allow_first_random: bool,
    pub board: Board,
    current_color: CheckersColor,
    current_player: &'a P,
    bot_count: usize,
    random_used: usize,
}

impl <'a, P: Player> Game<'a, P> {
    pub fn new(p1: &'a P, p2: &'a P, rows: usize) -> Result<Self, CheckersError> {
        let board_res = Board::new(rows);
        if let Err(_) = board_res {
            return Err(CheckersError::RuleError)
        }
        let board = board_res.unwrap();
        let game = Self {
            player1: p1,
            player2: p2,
            pawn_rows: rows,
            allow_first_random: false,
            board,
            current_color: CheckersColor::White,
            current_player: p1,
            bot_count: 0,
            random_used: 0,
        };
        Ok(game)
    }

    pub fn play(&self) {
        while !MoveExecutor::has_game_ended(&self.board, self.current_color) {

        }
    }

    fn switch_player(&mut self) {
        match self.current_color {
            CheckersColor::White => {
                self.current_player = self.player2;
                self.current_color = CheckersColor::Black;
            }
            CheckersColor::Black => {
                self.current_player = self.player1;
                self.current_color = CheckersColor::White;
            }
        }
    }

    fn one_move(&mut self) {
        let pieces = MoveExecutor::get_pieces(&self.board, self.current_color);
        let (cap_pawns, cap_queens) = MoveExecutor::get_capturing_pieces(&self.board, &pieces, self.current_color);
        if !cap_pawns.is_empty() || !cap_queens.is_empty() {
            self.do_capture(&cap_pawns, &cap_queens);
        }
    }

    fn do_capture(&mut self, cap_pawns: &Vec<(usize, usize)>, cap_queens: &Vec<(usize, usize)>) {
        let pawn_captures = MoveExecutor::get_possible_pawn_captures(&self.board, cap_pawns, self.current_color);
        let queen_captures = MoveExecutor::get_possible_queen_captures(&self.board, cap_queens, self.current_color);
        let longest_captures = MoveExecutor::get_longest_captures(
            &mut pawn_captures.iter().map(|v| v).collect(),
            &mut queen_captures.iter().map(|v| v).collect());
        let pos = if self.allow_first_random && (self.random_used < self.bot_count) {
            self.random_used = min(self.random_used + 1, self.bot_count);
            self.current_player.capture(&longest_captures, &self.board, true)
        } else {
            self.current_player.capture(&longest_captures, &self.board, false)
        };
        let player_choice = longest_captures[pos];
        // let mut last_move: Vec<(usize, usize)> = player_choice.iter().map(|j| j.start_pair()).collect();
        // last_move.push(player_choice[player_choice.len() - 1].end_pair());
        self.board = MoveExecutor::execute_capture(&self.board, &player_choice);
    }

    fn do_move(&self) {

    }
}