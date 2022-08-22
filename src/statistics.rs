use std::cmp::max;

#[derive(Copy, Clone)]
pub struct NodeCounter {
    pub nodes: usize
}

impl NodeCounter {
    pub fn new() -> Self {
        Self {
            nodes: 0,
        }
    }

    pub fn up(&mut self) {
        self.nodes += 1;
    }

    pub fn down(&mut self) {
        self.nodes -= max(0, self.nodes - 1);
    }

    pub fn zero(&mut self) {
        self.nodes = 0;
    }
}