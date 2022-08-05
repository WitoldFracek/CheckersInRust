use std::fmt::format;
use rand::Rng;
use crate::{alias_from_coordinates, Board};
use crate::col;
use crate::col::colored_text;
use crate::moves::{Jump, Move, SimpleMove};

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
    fn move_piece(&self, possible_moves: &Vec<SimpleMove>, board: &Board, allow_first_random: bool) -> usize ;
    fn capture(&self, possible_captures: &Vec<&Vec<Jump>>, board: &Board, allow_first_random: bool) -> usize;
    fn get_name(&self) -> &String;
}

pub trait Bot { }

pub struct Human {
    name: String,
}

impl Human {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}

impl Player for Human {
    fn move_piece(&self, possible_moves: &Vec<SimpleMove>, board: &Board, allow_first_random: bool) -> usize {
        println!("{}", colored_text(format!("\nPlayer {} moves", self.name).as_str(), col::fg::color(153, 255, 51).as_str(), col::NONE, true));
        for (i, mov) in possible_moves.iter().enumerate() {
            let start = alias_from_coordinates(mov.x_start, mov.y_start).unwrap();
            let end = alias_from_coordinates(mov.x_end, mov.y_end).unwrap();
            println!("{}. From {} to {}", i, start, end);
        }
        get_correct_input(possible_moves)
    }

    fn capture(&self, possible_captures: &Vec<&Vec<Jump>>, board: &Board, allow_first_random: bool) -> usize {
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
}

pub struct DummyBot {
    name: String,
}

impl DummyBot {

}

impl Bot for DummyBot { }
impl Player for DummyBot {

    #[allow(dead_code)]
    fn move_piece(&self, possible_moves: &Vec<SimpleMove>, board: &Board, allow_first_random: bool) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..possible_moves.len())
    }

    #[allow(dead_code)]
    fn capture(&self, possible_captures: &Vec<&Vec<Jump>>, board: &Board, allow_first_random: bool) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..possible_captures.len())
    }

    fn get_name(&self) -> &String {
        &self.name
    }
}
