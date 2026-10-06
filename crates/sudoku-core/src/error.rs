use core::fmt;

use crate::cell::Cell;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SudokuError {
    CellOccupied(Cell),
    PlacementConflict(Cell),
    UnknownDifficulty,
    InvalidPuzzleLength(usize),
    InvalidPuzzleCharacter(char),
    NoSolution,
    MultipleSolutions,
}

impl fmt::Display for SudokuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CellOccupied(cell) => write!(f, "cell {cell} is already filled"),
            Self::PlacementConflict(cell) => {
                write!(f, "digit conflicts with row, column, or box at {cell}")
            }
            Self::UnknownDifficulty => f.write_str("unknown difficulty"),
            Self::InvalidPuzzleLength(len) => {
                write!(f, "puzzle must be 81 characters, got {len}")
            }
            Self::InvalidPuzzleCharacter(ch) => {
                write!(f, "invalid character in puzzle: {ch:?}")
            }
            Self::NoSolution => f.write_str("puzzle has no solution"),
            Self::MultipleSolutions => f.write_str("puzzle has more than one solution"),
        }
    }
}

impl core::error::Error for SudokuError {}