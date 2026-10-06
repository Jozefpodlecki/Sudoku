use alloc::vec::Vec;
use sudoku_core::bits::Digit;
use sudoku_core::cell::Cell;
use sudoku_core::techniques::{Deduction, Technique};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HintState {
    pub technique: Technique,
    pub placements: Vec<(Cell, Digit)>,
    pub eliminations: Vec<(Cell, Digit)>,
}

impl HintState {
    pub fn from_deduction(deduction: &Deduction) -> Self {
        Self {
            technique: deduction.technique,
            placements: deduction.placements.clone(),
            eliminations: deduction.eliminations.clone(),
        }
    }

    pub fn is_placement(&self, cell: Cell) -> bool {
        self.placements.iter().any(|(hint_cell, _)| *hint_cell == cell)
    }

    pub fn is_elimination(&self, cell: Cell) -> bool {
        self.eliminations.iter().any(|(hint_cell, _)| *hint_cell == cell)
    }

    pub fn placement_digit(&self, cell: Cell) -> Option<Digit> {
        self.placements
            .iter()
            .find(|(hint_cell, _)| *hint_cell == cell)
            .map(|(_, digit)| *digit)
    }

    pub fn elimination_digits(&self, cell: Cell) -> Vec<Digit> {
        self.eliminations
            .iter()
            .filter(|(hint_cell, _)| *hint_cell == cell)
            .map(|(_, digit)| *digit)
            .collect()
    }
}