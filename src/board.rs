use std::borrow::Borrow;
use std::cmp::Ordering;
use std::convert::Infallible;
use std::fmt;
use std::fmt::Formatter;
use std::ops::{Index};
use std::vec::IntoIter;
use crate::{Piece, White, Black, CheckersColor};
use crate::checkers_utils::CheckersError;
use crate::col;
use crate::col::{colored_text, RGBColor};

#[derive(Copy, Clone)]
pub struct Board {
    _mask: u8,
    _board: u128,
}

impl Board {

    //binary = reserved : pawn(0) : white(0) : empty(0)

    pub const EMPTY: u8 = 0b0000;
    pub const WHITE_PAWN: u8 =  0b0001;
    pub const WHITE_QUEEN: u8 = 0b0101;
    pub const BLACK_PAWN: u8 =  0b0011;
    pub const BLACK_QUEEN: u8 = 0b0111;
}

impl Board {

    pub fn pieces_count(&self, color: CheckersColor) -> usize {
        let mut counter = 0_usize;
        for cell in self {
            match cell.piece {
                Some(Piece::Queen(col)) | Some(Piece::Pawn(col)) => {
                    if col == color {
                        counter += 1;
                    }
                }
                None => {}
            }
        }
        counter
    }

    // temp function
    pub fn repr(&self) -> String {
        let mut ret = String::from("   A  B  C  D  E  F  G  H \n");
        let mut row_counter = 0_i32;
        let mut column_counter = 0_i32;
        for cell in self {
            row_counter = column_counter / 8;
            if column_counter % 8 == 0 {
                ret = format!("{}{} ", ret, 8 - row_counter);
            }
            ret = format!("{}{}", ret, self.get_cell_repr(&cell, column_counter, row_counter));
            if column_counter % 8 == 7 {
                ret = format!("{} {}\n", ret, 8 - row_counter);
            }
            column_counter += 1;
        }
        ret = format!("{}   A  B  C  D  E  F  G  H \n", ret);
        return ret;
    }

    fn get_cell_repr(&self, cell: &Cell, column_counter: i32, row_counter: i32) -> String {
        let mut ret = "".to_string();
        if (column_counter - row_counter).abs() % 2 == 0 {
            let colored = colored_text("   ", col::NONE, col::BG::WHITE, true);
            ret = format!("{}{}", ret, colored);
        } else {
            ret = match cell.piece {
                Some(piece) => {
                    colored_text(format!("{} {} ",ret, piece.colored_marker(false)).as_str(),
                                 col::NONE,
                                 col::BG::color(70, 70, 70).as_str(),
                                 true)
                },
                None => colored_text(format!("{}   ", ret).as_str(),
                                     col::NONE,
                                     col::BG::color(70, 70, 70).as_str(),
                                     true)
            }
        }
        return ret
    }

    pub fn get_at(&self, x: usize, y: usize) -> Result<Option<Piece>, CheckersError> {
        if  x > 7 || y > 7 {
            return Err(CheckersError::IndexOutOfBounds);
        }
        if (x + y) % 2 == 0 {
            return Err(CheckersError::RuleError);
        }
        let shift = 4 * (4 * x + y / 2);
        let value = (self._board >> shift) & self._mask as u128;
        let ret = self.decode_piece(value);
        Ok(ret)
    }

    pub fn set_at(&mut self, x: usize, y: usize, value: u8) -> Result<(), CheckersError> {
        if  x > 7 || y > 7 {
            return Err(CheckersError::IndexOutOfBounds);
        }
        if (x + y) % 2 == 0 {
            return Err(CheckersError::RuleError);
        }
        match value {
            Self::EMPTY | Self::WHITE_PAWN | Self::WHITE_QUEEN | Self::BLACK_PAWN | Self::BLACK_QUEEN => {},
            _ => return Err(CheckersError::PawnBinaryValueError),
        };
        let shift = x as u128 * 16_u128 + (y / 2) as u128 * 4_u128;
        let remove_mask = 0b1111_u128 << shift;
        self._board = self._board & !remove_mask;
        let temp = value as u128;
        let mask = temp << shift;
        self._board = self._board | mask;
        Ok(())
    }

    pub fn is_empty_at(&self, x: usize, y: usize) -> Result<bool, CheckersError> {
        match self.get_at(x, y) {
            Ok(None) => Ok(true),
            Err(err) => Err(err),
            _ => Ok(false),
        }
    }

    pub fn is_field_excluded(&self, x: usize, y: usize) -> Result<bool, CheckersError> {
        if  x > 7 || y > 7 {
            return Err(CheckersError::IndexOutOfBounds);
        }
        if (x + y) % 2 == 0 {
            return Err(CheckersError::RuleError);
        }
        let shift = x as u128 * 16_u128 + (y / 2) as u128 * 4_u128;
        Ok((self._board >> shift) & 0b10000 == 0b10000)
    }

    pub fn set_field_excluded(&mut self, x: usize, y: usize) -> Result<(), CheckersError> {
        if  x > 7 || y > 7 {
            return Err(CheckersError::IndexOutOfBounds);
        }
        if (x + y) % 2 == 0 {
            return Err(CheckersError::RuleError);
        }

        let shift = x as u128 * 16_u128 + (y / 2) as u128 * 4_u128;
        self._board = self._board | (0b1000_u128 << shift);

        Ok(())
    }

    pub fn reset_excluded_fields(&mut self) {
        for shift in 0..32_u128 {
            let mask = 0b1000 << shift * 4;
            self._board = self._board & !mask;
        }
    }

    // pub fn repr(&self) -> String {
    //     let mut ret = String::from("   A  B  C  D  E  F  G  H \n");
    //     for i in 0..8 {
    //         ret = format!("{}{} ", ret, 8 - i);
    //         for j in 0..8 {
    //             if (i + j) % 2 != 0 {
    //                 let colored = colored_text("   ", col::NONE, col::BG::WHITE);
    //                 ret = format!("{}{}", ret, colored);
    //             }
    //             else {
    //                 match self[(i as usize, j as usize)] {
    //                     Some(piece) => ret = format!("{} {} ",ret, piece.marker()),
    //                     None => ret = format!("{}   ", ret)
    //                 }
    //             }
    //         }
    //         ret = format!("{} {}\n", ret, 8 - i);
    //     }
    //     ret = format!("{}   A  B  C  D  E  F  G  H \n", ret);
    //     return ret;
    // }
}

impl Board {

    pub fn get_board(&self) -> u128 {
        self._board
    }

    pub fn size(&self) -> usize {
        8
    }

    fn decode_piece(&self, value: u128) -> Option<Piece> {
        if (value & 0b1) == 0 {
            return None;
        }

        if (value & 0b10) == 0 {
            if (value & 0b100) == 0 {
                return Some(Piece::Pawn(White));
            }
            return Some(Piece::Queen(White));
        }

        if (value & 0b100) == 0 {
            return Some(Piece::Pawn(Black));
        }

        return Some(Piece::Queen(Black));
    }

    pub fn test() -> Self{
        Board {
            _mask: 0b1111,
            _board: 0b00110001,
        }
    }

    pub fn empty() -> Board {
        Board {
            _mask: 0b1111,
            _board: 0,
        }
    }

    pub fn new(pawn_rows: usize) -> Result<Board, String> {
        match pawn_rows {
            0 => return  Err("Cannot have 0 rows".to_string()),
            4..=usize::MAX => return Err(format!("Too many rows. 3 is the maximum of rows per player. Got {}", pawn_rows)),
            _ => {}
        };

        let empty_rows = 8 - 2 * pawn_rows;
        let mut board = 0_u128;
        for _ in 0..pawn_rows {
            for _ in 0..4 {
                board = board | Self::WHITE_PAWN as u128;
                board = board << 4;
            }
        }

        for _ in 0..empty_rows {
            for _ in 0..4 {
                board = board | Self::EMPTY as u128;
                board = board << 4;
            }
        }

        for i in 0..pawn_rows {
            for j in 0..4 {
                board = board | Self::BLACK_PAWN as u128;
                if i != pawn_rows - 1 || j != 3 {
                    board = board << 4;
                }
            }
        }

        let ret = Board {
            _mask: 0b1111,
            _board: board
        };

        return Ok(ret)
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

impl <'a> Iterator for BoardIterator<'_> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x > 7 {
            return None;
        }
        let ret = match self.board.get_at(self.x, self.y) {
            Ok(Some(piece)) => Some(piece),
            _ => None,
        };
        let cell = Cell {
            piece: ret
        };
        self.y += 1;
        if self.y == 8 {
            self.x += 1;
            self.y = 0;
        }
        Some(cell)
    }
}

pub struct Cell {
    pub piece: Option<Piece>
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
        match self.piece {
            Some(piece) => write!(f, "{}", piece.colored_marker(true)),
            None => write!(f, " ")
        }
    }
}
