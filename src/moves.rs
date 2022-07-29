

pub trait Move {
    fn start_pair(&self) -> (usize, usize);
    fn end_pair(&self) -> (usize, usize);
}

#[derive(Copy, Clone, Debug)]
pub struct SimpleMove {
    x_start: usize,
    y_start: usize,
    x_end: usize,
    y_end: usize,
}

impl SimpleMove {
    pub fn new(x_start: usize, y_start: usize, x_end: usize, y_end: usize) -> Self {
        SimpleMove {
            x_start,
            y_start,
            x_end,
            y_end,
        }
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

#[derive(Copy, Clone, Debug)]
pub struct Jump {
    x_start: usize,
    y_start: usize,
    x_end: usize,
    y_end: usize,
    x_capture: usize,
    y_capture: usize,
}

impl Jump {
    pub fn new(x_start: usize, y_start: usize, x_end: usize, y_end: usize, x_capture: usize, y_capture: usize) -> Self {
        Jump {
            x_start,
            y_start,
            x_end,
            y_end,
            x_capture,
            y_capture,
        }
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