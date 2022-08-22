use std::cmp::min;
use itertools::all;
use crate::{Board, CheckersColor, Human, MoveExecutor, Piece, Player, SimpleMove};
use crate::checkers_utils::CheckersError;
use crate::moves::{Jump, Move};

pub struct Game<'a> {
    pub player1: &'a mut dyn Player,
    pub player2: &'a mut dyn Player,
    pub pawn_rows: usize,
    pub allow_first_random: bool,
    pub board: Board,
    current_color: CheckersColor,
    bot_count: u8,
    random_used: u8,
    idle_white: u8,
    idle_black: u8,
}

impl <'a> Game<'a> {
    pub fn new(p1: &'a mut dyn Player, p2: &'a mut dyn Player, rows: usize) -> Self {
        assert!(rows > 0 && rows < 4, "Invalid row number. Should be between 1 nad 3. Your input {}", rows);
        let board = Board::new(rows);
        p1.set_color(CheckersColor::White);
        p2.set_color(CheckersColor::Black);
        let game = Self {
            player1: p1,
            player2: p2,
            pawn_rows: rows,
            allow_first_random: false,
            board,
            current_color: CheckersColor::White,
            bot_count: 0,
            random_used: 0,
            idle_black: 0,
            idle_white: 0,
        };
        game
    }

    pub fn new_with_bots(p1: &'a mut dyn Player, p2: &'a mut dyn Player, bot_count: u8, rows: usize) -> Self {
        assert!(bot_count < 3 && bot_count > 0, "Bot count should be between 1 and 2. Your input: {}", bot_count);
        let mut game = Self::new(p1, p2, rows);
        game.bot_count = bot_count;
        game
    }

    pub fn play(&mut self) {
        while !MoveExecutor::has_game_ended(self.board, self.current_color) &&
            self.idle_black < 8 && self.idle_white < 8 {
            println!("{}", self.board.repr());
            self.one_move()
        }
        self.switch_player();
        println!("{}", self.board.repr());
        println!("Player {} win!", self.current_player().get_name());
    }

    fn switch_player(&mut self) {
        match self.current_color {
            CheckersColor::White => {
                self.current_color = CheckersColor::Black;
            }
            CheckersColor::Black => {
                self.current_color = CheckersColor::White;
            }
        }
    }

    fn current_player(&mut self) -> &mut dyn Player  {
        match self.current_color {
            CheckersColor::White => self.player1,
            CheckersColor::Black => self.player2,
        }
    }

    fn one_move(&mut self) {
        let pieces = MoveExecutor::get_pieces(&self.board, self.current_color);
        let (cap_pawns, cap_queens) = MoveExecutor::get_capturing_pieces(&self.board, &pieces, self.current_color);
        let (mov_pawns, mov_queens) = MoveExecutor::get_moving_pieces(&self.board, &pieces, self.current_color);
        if !cap_pawns.is_empty() || !cap_queens.is_empty() {
            self.do_capture(&cap_pawns, &cap_queens);
        } else {
            self.do_move(&mov_pawns, &mov_queens)
        }
        self.board = MoveExecutor::promote_to_queen(&self.board);
        self.switch_player();
    }

    fn do_capture(&mut self, cap_pawns: &[(usize, usize)], cap_queens: &[(usize, usize)]) {
        let pawn_captures = MoveExecutor::get_possible_pawn_captures(&self.board, cap_pawns, self.current_color);
        let queen_captures = MoveExecutor::get_possible_queen_captures(&self.board, cap_queens, self.current_color);
        let longest_captures = MoveExecutor::get_longest_captures(
            &mut pawn_captures.iter().map(|v| v).collect(),
            &mut queen_captures.iter().map(|v| v).collect());
        let pos = if self.allow_first_random && (self.random_used < self.bot_count) {
            self.random_used = min(self.random_used + 1, self.bot_count);
            self.current_player().capture(&longest_captures, &new_board, true)
        } else {
            self.current_player().capture(&longest_captures, &self.board, false)
        };
        let player_choice = longest_captures[pos];
        self.board = MoveExecutor::execute_capture(&self.board, &player_choice);
    }

    fn do_move(&mut self, mov_paws: &[(usize, usize)], mov_queens: &[(usize, usize)]) {
        let mut pawn_moves = MoveExecutor::get_possible_pawn_moves(&self.board, mov_paws, self.current_color);
        let mut queen_moves = MoveExecutor::get_possible_queen_moves(&self.board, mov_queens);
        pawn_moves.append(&mut queen_moves);
        let all_moves = pawn_moves;
        let pos = if self.allow_first_random && self.random_used < self.bot_count {
            self.random_used = min(self.random_used + 1, self.bot_count);
            self.current_player().move_piece(&all_moves, &self.board, true)
        } else {
            self.current_player().move_piece(&all_moves, &self.board, false)
        };
        let player_choice = all_moves[pos];
        self.check_for_idle(player_choice);
        self.board = MoveExecutor::execute_move(self.board, player_choice)
    }

    fn check_for_idle(&mut self, mov: SimpleMove) {
        match self.board.get_at(mov.x_start, mov.y_start).unwrap().unwrap() {
            Piece::Queen(CheckersColor::Black) => self.idle_black += 1,
            Piece::Queen(CheckersColor::White) => self.idle_white += 1,
            Piece::Pawn(CheckersColor::Black) => self.idle_black = 0,
            Piece::Pawn(CheckersColor::White) => self.idle_white = 0,
        };
    }
}