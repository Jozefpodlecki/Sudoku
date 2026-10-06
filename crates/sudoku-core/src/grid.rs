use crate::bits::{Digit, Mask};
use crate::cell::Cell;
use crate::peers::{BOX_CELLS, COL_CELLS, HOUSE_CELLS, ROW_CELLS};
use crate::SudokuError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Grid {
    values: [u8; 81],
    candidates: [Mask; 81],
    row_masks: [Mask; 9],
    col_masks: [Mask; 9],
    box_masks: [Mask; 9],
    solved: u8,
}

impl Grid {
    pub const fn empty() -> Self {
        Self {
            values: [0; 81],
            candidates: [Mask::FULL; 81],
            row_masks: [Mask::EMPTY; 9],
            col_masks: [Mask::EMPTY; 9],
            box_masks: [Mask::EMPTY; 9],
            solved: 0,
        }
    }

    pub const fn values(&self) -> &[u8; 81] {
        &self.values
    }

    pub const fn value(&self, cell: Cell) -> u8 {
        self.values[cell.index()]
    }

    pub const fn is_empty_at(&self, cell: Cell) -> bool {
        self.values[cell.index()] == 0
    }

    pub const fn solved_count(&self) -> u8 {
        self.solved
    }

    pub const fn is_solved(&self) -> bool {
        self.solved == 81
    }

    pub const fn candidates(&self, cell: Cell) -> Mask {
        self.candidates[cell.index()]
    }

    pub const fn row_mask(&self, row: usize) -> Mask {
        self.row_masks[row]
    }

    pub const fn col_mask(&self, col: usize) -> Mask {
        self.col_masks[col]
    }

    pub const fn box_mask(&self, box_index: usize) -> Mask {
        self.box_masks[box_index]
    }

    pub const fn house_mask(&self, house: usize) -> Mask {
        if house < 9 {
            self.row_masks[house]
        } else if house < 18 {
            self.col_masks[house - 9]
        } else {
            self.box_masks[house - 18]
        }
    }

    pub fn force_place(&mut self, cell: Cell, digit: Digit) {
        let index = cell.index();
        if self.values[index] != 0 {
            self.remove(cell);
        }

        self.values[index] = digit.get();
        self.candidates[index] = Mask::EMPTY;
        let bit = digit.mask();
        self.row_masks[cell.row() as usize] |= bit;
        self.col_masks[cell.col() as usize] |= bit;
        self.box_masks[cell.box_index() as usize] |= bit;
        self.solved += 1;

        for peer in cell.peers() {
            self.candidates[peer.index()] = self.candidates[peer.index()].remove(digit);
        }
    }

    pub fn place(&mut self, cell: Cell, digit: Digit) -> Result<(), SudokuError> {
        let index = cell.index();
        if self.values[index] != 0 {
            return Err(SudokuError::CellOccupied(cell));
        }
        if self.row_masks[cell.row() as usize].contains(digit)
            || self.col_masks[cell.col() as usize].contains(digit)
            || self.box_masks[cell.box_index() as usize].contains(digit)
        {
            return Err(SudokuError::PlacementConflict(cell));
        }

        self.values[index] = digit.get();
        self.candidates[index] = Mask::EMPTY;
        let bit = digit.mask();
        self.row_masks[cell.row() as usize] |= bit;
        self.col_masks[cell.col() as usize] |= bit;
        self.box_masks[cell.box_index() as usize] |= bit;
        self.solved += 1;

        for peer in cell.peers() {
            self.candidates[peer.index()] = self.candidates[peer.index()].remove(digit);
        }

        Ok(())
    }

    pub fn remove(&mut self, cell: Cell) -> Option<Digit> {
        let index = cell.index();
        let value = self.values[index];
        if value == 0 {
            return None;
        }

        let digit = Digit::new(value).unwrap();
        self.values[index] = 0;
        let bit = digit.mask();
        self.row_masks[cell.row() as usize] -= bit;
        self.col_masks[cell.col() as usize] -= bit;
        self.box_masks[cell.box_index() as usize] -= bit;
        self.solved -= 1;

        self.recompute_candidates(cell);

        Some(digit)
    }

    fn recompute_candidates(&mut self, cell: Cell) {
        for peer in cell.peers() {
            if self.values[peer.index()] != 0 {
                continue;
            }
            self.candidates[peer.index()] = Mask::FULL
                .difference(self.row_masks[peer.row() as usize])
                .difference(self.col_masks[peer.col() as usize])
                .difference(self.box_masks[peer.box_index() as usize]);
        }
        if self.values[cell.index()] == 0 {
            self.candidates[cell.index()] = Mask::FULL
                .difference(self.row_masks[cell.row() as usize])
                .difference(self.col_masks[cell.col() as usize])
                .difference(self.box_masks[cell.box_index() as usize]);
        }
    }

    pub fn eliminate(&mut self, cell: Cell, digit: Digit) {
        if self.values[cell.index()] != 0 {
            return;
        }
        self.candidates[cell.index()] = self.candidates[cell.index()].remove(digit);
    }

    pub fn clear(&mut self) {
        *self = Self::empty();
    }

    pub fn candidates_of_house(&self, house: usize) -> [Mask; 9] {
        let cells = HOUSE_CELLS[house];
        let mut result = [Mask::EMPTY; 9];
        let mut index = 0;
        while index < 9 {
            result[index] = self.candidates(Cell(cells[index]));
            index += 1;
        }
        result
    }

    pub fn duplicates_in_row(&self, row: usize) -> Mask {
        let mut seen = Mask::EMPTY;
        let mut duplicates = Mask::EMPTY;
        for cell_index in ROW_CELLS[row] {
            let value = self.values[cell_index as usize];
            if value == 0 {
                continue;
            }
            let digit = Digit::new(value).unwrap();
            if seen.contains(digit) {
                duplicates = duplicates.insert(digit);
            } else {
                seen = seen.insert(digit);
            }
        }
        duplicates
    }

    pub fn duplicates_in_col(&self, col: usize) -> Mask {
        let mut seen = Mask::EMPTY;
        let mut duplicates = Mask::EMPTY;
        for cell_index in COL_CELLS[col] {
            let value = self.values[cell_index as usize];
            if value == 0 {
                continue;
            }
            let digit = Digit::new(value).unwrap();
            if seen.contains(digit) {
                duplicates = duplicates.insert(digit);
            } else {
                seen = seen.insert(digit);
            }
        }
        duplicates
    }

    pub fn duplicates_in_box(&self, box_index: usize) -> Mask {
        let mut seen = Mask::EMPTY;
        let mut duplicates = Mask::EMPTY;
        for cell_index in BOX_CELLS[box_index] {
            let value = self.values[cell_index as usize];
            if value == 0 {
                continue;
            }
            let digit = Digit::new(value).unwrap();
            if seen.contains(digit) {
                duplicates = duplicates.insert(digit);
            } else {
                seen = seen.insert(digit);
            }
        }
        duplicates
    }

    pub fn has_conflicts(&self) -> bool {
        for index in 0..9 {
            if !self.duplicates_in_row(index).is_empty()
                || !self.duplicates_in_col(index).is_empty()
                || !self.duplicates_in_box(index).is_empty()
            {
                return true;
            }
        }
        false
    }
}