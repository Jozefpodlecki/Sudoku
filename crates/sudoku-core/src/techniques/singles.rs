use super::{Deduction, Technique};
use crate::bits::Digit;
use crate::cell::Cell;
use crate::grid::Grid;
use crate::peers::HOUSE_CELLS;

pub(super) fn find_naked_single(grid: &Grid) -> Option<Deduction> {
    for cell in Cell::all() {
        if !grid.is_empty_at(cell) {
            continue;
        }
        let candidates = grid.candidates(cell);
        if candidates.len() == 1 {
            let digit = candidates.lowest()?;
            return Some(Deduction::placement(Technique::NakedSingle, cell, digit));
        }
    }
    None
}

pub(super) fn find_hidden_single(grid: &Grid) -> Option<Deduction> {
    for house in 0..27 {
        let cells = HOUSE_CELLS[house];

        for digit in Digit::all() {
            let mut occurrences = 0u32;
            let mut found_cell = None;

            for &cell_index in &cells {
                let cell = Cell(cell_index);
                if !grid.is_empty_at(cell) {
                    continue;
                }
                if grid.candidates(cell).contains(digit) {
                    occurrences += 1;
                    found_cell = Some(cell);
                }
            }

            if occurrences == 1 {
                let cell = found_cell?;
                return Some(Deduction::placement(Technique::HiddenSingle, cell, digit));
            }
        }
    }
    None
}