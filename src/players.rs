use std::ascii::escape_default;
use std::fmt::{format, Pointer};
use itertools::max;
use rand::Rng;
use crate::{alias_from_coordinates, Board, CheckersColor, MoveExecutor};
use crate::board_estimators::Estimator;
use crate::col;
use crate::col::colored_text;
use crate::moves::{Jump, Move, SimpleMove};
use std::time::{Instant};
use crate::statistics::NodeCounter;

fn get_correct_input<T>(list: &Vec<T>) -> usize {
    use std::io::{stdin, stdout, Write};
    let mut is_correct = false;
    let mut pos = 0;
    while !is_correct {
        let mut s = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect(format!("{} Valid options are between 0 and {}", colored_text("Incorrect input.", col::fg::RED, col::NONE, true),list.len() - 1).as_str());
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }

        match s.parse::<usize>() {
            Ok(num) if num < list.len() => {
                pos = num;
                is_correct = true;
            }
            _ => {
                println!("{} Valid options are between 0 and {}", colored_text("Incorrect input.", col::fg::RED, col::NONE, true),list.len() - 1);
                continue;
            }
        }
    }
    pos
}


pub trait Player {
    fn move_piece(&self, possible_moves: &Vec<SimpleMove>, board: Board, allow_first_random: bool) -> usize ;
    fn capture(&self, possible_captures: &Vec<&Vec<Jump>>, board: Board, allow_first_random: bool) -> usize;
    fn get_name(&self) -> &String;
    fn set_color(&mut self, color: CheckersColor);
    fn get_color(&self) -> CheckersColor;
}

pub struct Human {
    name: String,
    color: CheckersColor,
}

impl Human {
    pub fn new(name: &str, color: CheckersColor) -> Self {
        Self {
            name: String::from(name),
            color,
        }
    }
}

impl Player for Human {
    fn move_piece(&self, possible_moves: &Vec<SimpleMove>, board: Board, allow_first_random: bool) -> usize {
        println!("{}", colored_text(format!("\nPlayer {} moves", self.name).as_str(), col::fg::color(153, 255, 51).as_str(), col::NONE, true));
        for (i, mov) in possible_moves.iter().enumerate() {
            let start = alias_from_coordinates(mov.x_start, mov.y_start).unwrap();
            let end = alias_from_coordinates(mov.x_end, mov.y_end).unwrap();
            println!("{}. From {} to {}", i, start, end);
        }
        get_correct_input(possible_moves)
    }

    fn capture(&self, possible_captures: &Vec<&Vec<Jump>>, board: Board, allow_first_random: bool) -> usize {
        println!("{}", colored_text(format!("\nPlayer {} moves", self.name).as_str(), col::fg::color(153, 255, 51).as_str(), col::NONE, true));
        for (i, jumps) in possible_captures.iter().enumerate() {
            let start_jump = jumps[0];
            let start = alias_from_coordinates(start_jump.x_start, start_jump.y_start).unwrap();
            let mut path = String::new();
            for jump in *jumps {
                path = format!("{} -> ", path);
                path = format!("{}{}", path, alias_from_coordinates(jump.x_end, jump.y_end).unwrap());
            }
            println!("{}. From {} {}", i, start, path);
        }
        get_correct_input(possible_captures)
    }

    fn get_name(&self) -> &String {
        &self.name
    }
    
    fn set_color(&mut self, color: CheckersColor) {
        self.color = color;
    }

    fn get_color(&self) -> CheckersColor {
        self.color
    }
}

pub struct DummyBot {
    name: String,
    color: CheckersColor,
}

impl DummyBot {
    pub fn new(name: &str, color: CheckersColor) -> Self {
        Self {
            name: name.to_string(),
            color,
        }
    }
}

impl Player for DummyBot {

    #[allow(dead_code)]
    fn move_piece(&self, possible_moves: &Vec<SimpleMove>, board: Board, allow_first_random: bool) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..possible_moves.len())
    }

    #[allow(dead_code)]
    fn capture(&self, possible_captures: &Vec<&Vec<Jump>>, board: Board, allow_first_random: bool) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..possible_captures.len())
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn set_color(&mut self, color: CheckersColor) {
        self.color = color;
    }

    fn get_color(&self) -> CheckersColor {
        self.color
    }
}

pub struct MinMaxBot<'a> {
    name: String,
    depth: usize,
    estimator: &'a dyn Estimator,
    color: CheckersColor,
    node_counter: Option<NodeCounter>,
}

impl <'a> MinMaxBot<'a> {
    pub fn new(name: &str, color: CheckersColor, depth: usize, estimator: &'a dyn Estimator) -> Self {
        Self {
            name: name.to_string(),
            depth,
            estimator,
            color,
            node_counter: None,
        }
    }

    pub fn set_node_counter(&mut self, node_counter: NodeCounter) {
        self.node_counter = Some(node_counter);
    }

    fn minmax(&self, board: Board, depth: usize, current_color: CheckersColor, maximising: bool) -> i32 {
        // self.node_count += 1;
        if let Some(_) = &self.node_counter {
            self.node_counter.unwrap().up();
        }
        if depth == 0 {
            return self.estimator.estimate(board, self.color, false);
        }

        let all_captures = MoveExecutor::get_all_captures(&board, current_color);
        if !all_captures.is_empty() {
            let ref_vec: Vec<&Vec<Jump>> = all_captures.iter().map(|v| v).collect();
            return self.minmax_jumps(board, &ref_vec, depth, current_color, maximising);
        }
        let all_moves = MoveExecutor::get_all_moves(&board, current_color);
        if !all_moves.is_empty() {
            return self.minmax_moves(board, all_moves.as_ref(), depth, current_color, maximising);
        }
        let current_estimation = self.estimator.estimate(board, self.color, true);
        if maximising {
            current_estimation + self.depth as i32 - depth as i32
        } else {
            current_estimation - self.depth as i32  + depth as i32
        }
    }

    fn minmax_moves(&self, board: Board, moves: &[SimpleMove], depth: usize, current_color: CheckersColor, maximising: bool) -> i32 {
        if maximising {
            let mut current_best = i32::MIN;
            for &m in moves {
                let mut new_board = board.clone();
                new_board = MoveExecutor::execute_move(new_board, m);
                let est = self.minmax(new_board, depth - 1, current_color.opposite_color(), false);
                if est > current_best {
                    current_best = est;
                }
            }
            return current_best;
        }
        let mut current_worst = i32::MAX;
        for &m in moves {
            let mut new_board = board.clone();
            new_board = MoveExecutor::execute_move(new_board, m);
            let est = self.minmax(new_board, depth - 1, current_color.opposite_color(), true);
            if est < current_worst {
                current_worst = est;
            }
        }
        current_worst
    }

    fn minmax_jumps(&self, board: Board, jumps: &Vec<&Vec<Jump>>, depth: usize, current_color: CheckersColor, maximising: bool) -> i32 {
        if maximising {
            let mut current_best = i32::MIN;
            for &jump in jumps {
                let mut new_board = board.clone();
                new_board = MoveExecutor::execute_capture(&new_board, jump);
                let est = self.minmax(new_board, depth - 1, current_color.opposite_color(), false);
                if est > current_best {
                    current_best = est;
                }
            }
            return current_best;
        }
        let mut current_worst = i32::MAX;
        for &jump in jumps {
            let mut new_board = board.clone();
            new_board = MoveExecutor::execute_capture(&new_board, jump);
            let est = self.minmax(new_board, depth - 1, current_color.opposite_color(), true);
            if est < current_worst {
                current_worst = est;
            }
        }
        current_worst
    }
}

impl Player for MinMaxBot<'_> {
    fn move_piece(&self, possible_moves: &Vec<SimpleMove>, board: Board, allow_first_random: bool) -> usize {
        let mut best_moves = Vec::new();
        // self.node_count = 0;
        if let Some(counter) = &self.node_counter {
            self.node_counter.unwrap().zero();
        }
        let mut best_eval = i32::MIN;
        if allow_first_random {
            let mut rng = rand::thread_rng();
            return rng.gen_range(0..possible_moves.len());
        }
        let start = Instant::now();
        if possible_moves.len() == 1 {
            let elapsed = start.elapsed();
            println!("{:?}", self.color);
            println!("Computed in:    {:?} s", elapsed.as_millis() / 1000);
            if let Some(counter) = &self.node_counter {
                println!("Visited nodes:  {:?}\n", counter.nodes);
            }
            return 0;
        }
        for (i, &mov) in possible_moves.iter().enumerate() {
            let mut new_board = board.clone();
            new_board = MoveExecutor::execute_move(new_board, mov);
            let eval = self.minmax(new_board, self.depth - 1, self.color.opposite_color(), false);
            if let Some(_) = &self.node_counter {
                self.node_counter.unwrap().up();
            }
            if eval > best_eval {
                best_eval = eval;
                best_moves.clear();
                best_moves.push(i);
            } else if eval == best_eval {
                best_moves.push(i);
            }
        }
        let elapsed = start.elapsed();
        println!("{:?}", self.color);
        println!("Computed in:    {:?} s", elapsed.as_millis() / 1000);
        if let Some(counter) = &self.node_counter {
                println!("Visited nodes:  {:?}\n", counter.nodes);
        }
        let mut rng = rand::thread_rng();
        best_moves[rng.gen_range(0..best_moves.len())]
    }

    fn capture(&self, possible_captures: &Vec<&Vec<Jump>>, board: Board, allow_first_random: bool) -> usize {
        let mut best_captures = Vec::new();
        let mut best_eval = i32::MIN;
        if allow_first_random {
            let mut rng = rand::thread_rng();
            return rng.gen_range(0..possible_captures.len());
        }
        let start = Instant::now();
        if possible_captures.len() == 1 {
            let elapsed = start.elapsed();
            println!("{:?}", self.color);
            println!("Computed in:    {:?} s", elapsed.as_millis() / 1000);
            if let Some(counter) = &self.node_counter {
                println!("Visited nodes:  {:?}\n", counter.nodes);
            }
            return 0;
        }
        for (i, &capture_path) in possible_captures.iter().enumerate() {
            let mut new_board = board.clone();
            new_board = MoveExecutor::execute_capture(&new_board, capture_path);
            let eval = self.minmax(new_board, self.depth - 1, self.color.opposite_color(), false);
            if eval > best_eval {
                best_eval = eval;
                best_captures.clear();
                best_captures.push(i);
            } else if eval == best_eval {
                best_captures.push(i);
            }
        }
        let elapsed = start.elapsed();
        println!("{:?}", self.color);
        println!("Computed in:    {:?} s", elapsed.as_millis() / 1000);
        if let Some(counter) = &self.node_counter {
            println!("Visited nodes:  {:?}\n", counter.nodes);
        }
        let mut rng = rand::thread_rng();
        best_captures[rng.gen_range(0..best_captures.len())]
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn set_color(&mut self, color: CheckersColor) {
        self.color = color;
    }

    fn get_color(&self) -> CheckersColor {
        self.color
    }
}
