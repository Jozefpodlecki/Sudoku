use alloc::vec::Vec;

use super::{Deduction, Technique};
use crate::bits::Digit;
use crate::cell::Cell;
use crate::grid::Grid;
use crate::peers::{BOX_CELLS, COL_CELLS, ROW_CELLS};

pub(super) fn find_pointing(grid: &Grid) -> Option<Deduction> {
    for box_index in 0..9 {
        for digit in Digit::all() {
            let positions: Vec<Cell> = BOX_CELLS[box_index]
                .iter()
                .map(|&index| Cell(index))
                .filter(|&cell| grid.is_empty_at(cell) && grid.candidates(cell).contains(digit))
                .collect();

            if positions.len() < 2 {
                continue;
            }

            let same_row = positions.iter().all(|cell| cell.row() == positions[0].row());
            let same_col = positions.iter().all(|cell| cell.col() == positions[0].col());

            if same_row {
                let row = positions[0].row();
                let eliminations: Vec<(Cell, Digit)> = ROW_CELLS[row as usize]
                    .iter()
                    .map(|&index| Cell(index))
                    .filter(|&cell| {
                        cell.box_index() != box_index as u8
                            && grid.is_empty_at(cell)
                            && grid.candidates(cell).contains(digit)
                    })
                    .map(|cell| (cell, digit))
                    .collect();

                if !eliminations.is_empty() {
                    return Some(Deduction::eliminations(
                        Technique::LockedCandidatePointing,
                        eliminations,
                    ));
                }
            }

            if same_col {
                let col = positions[0].col();
                let eliminations: Vec<(Cell, Digit)> = COL_CELLS[col as usize]
                    .iter()
                    .map(|&index| Cell(index))
                    .filter(|&cell| {
                        cell.box_index() != box_index as u8
                            && grid.is_empty_at(cell)
                            && grid.candidates(cell).contains(digit)
                    })
                    .map(|cell| (cell, digit))
                    .collect();

                if !eliminations.is_empty() {
                    return Some(Deduction::eliminations(
                        Technique::LockedCandidatePointing,
                        eliminations,
                    ));
                }
            }
        }
    }
    None
}

pub(super) fn find_claiming(grid: &Grid) -> Option<Deduction> {
    for row in 0..9u8 {
        for digit in Digit::all() {
            let positions: Vec<Cell> = ROW_CELLS[row as usize]
                .iter()
                .map(|&index| Cell(index))
                .filter(|&cell| grid.is_empty_at(cell) && grid.candidates(cell).contains(digit))
                .collect();

            if positions.len() < 2 {
                continue;
            }

            if positions.iter().all(|cell| cell.box_index() == positions[0].box_index()) {
                let box_index = positions[0].box_index();
                let eliminations: Vec<(Cell, Digit)> = BOX_CELLS[box_index as usize]
                    .iter()
                    .map(|&index| Cell(index))
                    .filter(|&cell| {
                        cell.row() != row
                            && grid.is_empty_at(cell)
                            && grid.candidates(cell).contains(digit)
                    })
                    .map(|cell| (cell, digit))
                    .collect();

                if !eliminations.is_empty() {
                    return Some(Deduction::eliminations(
                        Technique::LockedCandidateClaiming,
                        eliminations,
                    ));
                }
            }
        }
    }

    for col in 0..9u8 {
        for digit in Digit::all() {
            let positions: Vec<Cell> = COL_CELLS[col as usize]
                .iter()
                .map(|&index| Cell(index))
                .filter(|&cell| grid.is_empty_at(cell) && grid.candidates(cell).contains(digit))
                .collect();

            if positions.len() < 2 {
                continue;
            }

            if positions.iter().all(|cell| cell.box_index() == positions[0].box_index()) {
                let box_index = positions[0].box_index();
                let eliminations: Vec<(Cell, Digit)> = BOX_CELLS[box_index as usize]
                    .iter()
                    .map(|&index| Cell(index))
                    .filter(|&cell| {
                        cell.col() != col
                            && grid.is_empty_at(cell)
                            && grid.candidates(cell).contains(digit)
                    })
                    .map(|cell| (cell, digit))
                    .collect();

                if !eliminations.is_empty() {
                    return Some(Deduction::eliminations(
                        Technique::LockedCandidateClaiming,
                        eliminations,
                    ));
                }
            }
        }
    }

    None
}