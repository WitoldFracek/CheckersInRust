use std::cmp::Ordering;
use std::convert::Infallible;
use std::fmt;
use std::fmt::Formatter;
use std::ops::{Index};
use std::vec::IntoIter;
use crate::{Piece, White, Black};
use crate::col;
use crate::col::colored_text;

pub struct Board {
    _mask: u8,
    _board: u128,
}

impl Board {
    pub fn repr(&self) -> String {
        let mut ret = String::from("   A  B  C  D  E  F  G  H \n");
        for i in 0..8 {
            ret = format!("{}{} ", ret, 8 - i);
            for j in 0..8 {
                if (i + j) % 2 == 0 {
                    let colored = colored_text("   ", col::NONE, col::BG::WHITE);
                    ret = format!("{}{}", ret, colored);
                }
                else {
                    match self[(i as usize, j as usize)] {
                        Some(piece) => ret = format!("{} {} ",ret, piece.marker()),
                        None => ret = format!("{}   ", ret)
                    }
                }
            }
            ret = format!("{} {}\n", ret, 8 - i);
        }
        ret = format!("{}   A  B  C  D  E  F  G  H \n", ret);
        return ret;
    }
}

impl Board {
    fn decode_piece(&self, value: u128) -> &Option<Piece> {
        if (value & 0b1) == 0 {
            return &None
        }

        if (value & 0b10) == 0 {
            if (value & 0b100) == 0 {
                return &Some(Piece::Pawn(White))
            }
            return &Some(Piece::Pawn(Black))
        }

        if (value & 0b100) == 0 {
            return &Some(Piece::Queen(White))
        }

        return &Some(Piece::Queen(Black));
    }

    pub fn test() -> Self{
        Board {
            _mask: 0b1111,
            _board: 0b00010111,
        }
    }

}

impl Index<(usize, usize)> for Board {
    type Output = Option<Piece>;
    fn index(&self, index: (usize, usize)) -> &Self::Output {

        if (index.0 + index.1) % 2 != 0 {
            return &None;
        }

        let shift = 4 * (4 * index.0 + index.1 / 2);
        let value = (self._board >> shift) & self._mask as u128;
        return self.decode_piece(value);
    }
}

impl Default for Board {
    fn default() -> Self {
        Board {
            _mask: 0b1111,
            _board: 0,
        }
    }
}

impl<'a> IntoIterator for &'a Board {
    type Item = Cell;
    type IntoIter = BoardIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BoardIterator {
            x: 0,
            y: 0,
            board: self,
        }
    }
}

pub struct BoardIterator<'a> {
    x: usize,
    y: usize,
    board: &'a Board
}

impl<'a> Iterator for BoardIterator<'_> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x == 8 {
            return None;
        }
        let ret = self.board[(self.x, self.y)];
        let cell = Cell {
            piece: ret
        };
        self.y += 1;
        if self.y == 8 {
            self.x += 1;
            self.y = 0;
        }
        return Some(cell);
    }
}

pub struct Cell {
    piece: Option<Piece>
}

impl Cell {
    pub fn new(piece: Option<Piece>) -> Cell {
        Cell {
            piece,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.piece {
            None => true,
            _ => false,
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        return match self.piece {
            Some(piece) => write!(f, "{}", piece.marker()),
            None => write!(f, " ")
        }
    }
}
