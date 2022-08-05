use std::fmt::{Display, Formatter};
use crate::{alias_from_coordinates};
use crate::checkers_utils::{CheckersError, is_in_bounds};

pub trait Move {
    fn start_pair(&self) -> (usize, usize);
    fn end_pair(&self) -> (usize, usize);
}

#[derive(Copy, Clone, Debug)]
pub struct SimpleMove {
    pub x_start: usize,
    pub y_start: usize,
    pub x_end: usize,
    pub y_end: usize,
}

impl SimpleMove {
    pub fn new(x_start: usize, y_start: usize, x_end: usize, y_end: usize) -> Result<Self, CheckersError> {
        if !is_in_bounds(x_start as i32, y_start as i32) ||
            !is_in_bounds(x_end as i32, y_end as i32) {
            return Err(CheckersError::IndexOutOfBounds);
        }
        Ok(SimpleMove {
            x_start,
            y_start,
            x_end,
            y_end,
        })
    }
}

impl Move for SimpleMove {
    fn start_pair(&self) -> (usize, usize) {
        (self.x_start, self.y_start)
    }

    fn end_pair(&self) -> (usize, usize) {
        (self.x_end, self.y_end)
    }
}

impl Display for SimpleMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} to {}",
               alias_from_coordinates(self.x_start, self.y_start).unwrap(),
               alias_from_coordinates(self.x_end, self.y_end).unwrap())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Jump {
    pub x_start: usize,
    pub y_start: usize,
    pub x_end: usize,
    pub y_end: usize,
    pub x_capture: usize,
    pub y_capture: usize,
}

impl Jump {
    pub fn new(x_start: usize, y_start: usize, x_end: usize, y_end: usize, x_capture: usize, y_capture: usize) -> Result<Self, CheckersError> {
        if !is_in_bounds(x_start as i32, y_start as i32) ||
            !is_in_bounds(x_end as i32, y_end as i32) ||
            !is_in_bounds(x_capture as i32, y_capture as i32) {
            return Err(CheckersError::IndexOutOfBounds);
        }
        Ok(Jump {
            x_start,
            y_start,
            x_end,
            y_end,
            x_capture,
            y_capture,
        })
    }
}

impl Move for Jump {
    fn start_pair(&self) -> (usize, usize) {
        (self.x_start, self.y_start)
    }

    fn end_pair(&self) -> (usize, usize) {
        (self.x_end, self.y_end)
    }
}

impl Display for Jump {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} to {} over {}",
               alias_from_coordinates(self.x_start, self.y_start).unwrap(),
               alias_from_coordinates(self.x_end, self.y_end).unwrap(),
               alias_from_coordinates(self.x_capture, self.y_capture).unwrap())
    }
}