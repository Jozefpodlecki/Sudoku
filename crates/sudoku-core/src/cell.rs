use core::fmt;
use core::ops::Index;

use crate::peers::PEERS;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cell(pub(crate) u8);

impl Cell {
    pub const COUNT: usize = 81;

    pub const fn new(index: u8) -> Option<Self> {
        if (index as usize) < Self::COUNT {
            Some(Self(index))
        } else {
            None
        }
    }

    pub const fn from_index(index: usize) -> Option<Self> {
        if index < Self::COUNT {
            Some(Self(index as u8))
        } else {
            None
        }
    }

    pub const fn index(self) -> usize {
        self.0 as usize
    }

    pub const fn get(self) -> u8 {
        self.0
    }

    pub const fn row(self) -> u8 {
        self.0 / 9
    }

    pub const fn col(self) -> u8 {
        self.0 % 9
    }

    pub const fn box_index(self) -> u8 {
        (self.row() / 3) * 3 + self.col() / 3
    }

    pub const fn is_peer(self, other: Self) -> bool {
        if self.0 == other.0 {
            return false;
        }
        self.row() == other.row()
            || self.col() == other.col()
            || self.box_index() == other.box_index()
    }

    pub const fn peers(self) -> [Self; 20] {
        let raw = PEERS[self.index()];
        let mut result = [Self(0); 20];
        let mut index = 0;
        while index < 20 {
            result[index] = Self(raw[index]);
            index += 1;
        }
        result
    }

    pub const fn all() -> [Self; 81] {
        let mut result = [Self(0); 81];
        let mut index = 0;
        while index < 81 {
            result[index] = Self(index as u8);
            index += 1;
        }
        result
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "r{}c{}", self.row() + 1, self.col() + 1)
    }
}

pub struct CellIter {
    next: u8,
}

impl Iterator for CellIter {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.next as usize) >= Cell::COUNT {
            return None;
        }
        let cell = Cell(self.next);
        self.next += 1;
        Some(cell)
    }
}

pub const fn cells() -> CellIter {
    CellIter { next: 0 }
}

impl Index<Cell> for [u8; 81] {
    type Output = u8;

    fn index(&self, cell: Cell) -> &Self::Output {
        &self[cell.index()]
    }
}