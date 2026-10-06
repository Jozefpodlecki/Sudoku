use alloc::vec::Vec;

use super::combinations::Combinations;
use super::{Deduction, Technique};
use crate::bits::{Digit, Mask};
use crate::cell::Cell;
use crate::grid::Grid;
use crate::peers::HOUSE_CELLS;

pub(super) fn find_naked_pair(grid: &Grid) -> Option<Deduction> {
    find_naked_subset::<2>(grid, Technique::NakedPair)
}

pub(super) fn find_naked_triple(grid: &Grid) -> Option<Deduction> {
    find_naked_subset::<3>(grid, Technique::NakedTriple)
}

pub(super) fn find_hidden_pair(grid: &Grid) -> Option<Deduction> {
    find_hidden_subset::<2>(grid, Technique::HiddenPair)
}

pub(super) fn find_hidden_triple(grid: &Grid) -> Option<Deduction> {
    find_hidden_subset::<3>(grid, Technique::HiddenTriple)
}

fn find_naked_subset<const SIZE: usize>(grid: &Grid, technique: Technique) -> Option<Deduction> {
    for house in 0..27 {
        let cells: Vec<Cell> = HOUSE_CELLS[house]
            .iter()
            .map(|&index| Cell(index))
            .filter(|&cell| grid.is_empty_at(cell))
            .collect();

        if cells.len() < SIZE {
            continue;
        }

        for combination in Combinations::new(cells.len(), SIZE) {
            let selected: Vec<Cell> = combination.iter().map(|&i| cells[i]).collect();

            let mut union = Mask::EMPTY;
            let mut valid = true;
            for &cell in &selected {
                let candidates = grid.candidates(cell);
                if candidates.len() > SIZE as u32 {
                    valid = false;
                    break;
                }
                union = union.union(candidates);
            }

            if !valid || union.len() as usize != SIZE {
                continue;
            }

            let mut eliminations = Vec::new();
            for &cell in &cells {
                if selected.contains(&cell) {
                    continue;
                }
                for digit in union.iter() {
                    if grid.candidates(cell).contains(digit) {
                        eliminations.push((cell, digit));
                    }
                }
            }

            if !eliminations.is_empty() {
                return Some(Deduction::eliminations(technique, eliminations));
            }
        }
    }

    None
}

fn find_hidden_subset<const SIZE: usize>(grid: &Grid, technique: Technique) -> Option<Deduction> {
    for house in 0..27 {
        let cells: Vec<Cell> = HOUSE_CELLS[house]
            .iter()
            .map(|&index| Cell(index))
            .filter(|&cell| grid.is_empty_at(cell))
            .collect();

        if cells.len() < SIZE {
            continue;
        }

        let digits: Vec<Digit> = Digit::all().to_vec();

        for combination in Combinations::new(digits.len(), SIZE) {
            let selected_digits: Vec<Digit> = combination.iter().map(|&i| digits[i]).collect();
            let digit_mask = selected_digits
                .iter()
                .fold(Mask::EMPTY, |accumulator, &digit| accumulator.union(digit.mask()));

            let mut matching_cells: Vec<Cell> = Vec::new();
            let mut valid = true;

            for &cell in &cells {
                let candidates = grid.candidates(cell);
                let overlap = candidates.intersect(digit_mask);
                if !overlap.is_empty() {
                    matching_cells.push(cell);
                }
            }

            if matching_cells.len() != SIZE {
                continue;
            }

            for &cell in &matching_cells {
                let candidates = grid.candidates(cell);
                let overlap = candidates.intersect(digit_mask);
                if !candidates.difference(overlap).is_empty() {
                    valid = false;
                }
            }

            if !valid {
                continue;
            }

            let mut eliminations = Vec::new();
            for &cell in &matching_cells {
                let candidates = grid.candidates(cell);
                for digit in candidates.iter() {
                    if !digit_mask.contains(digit) {
                        eliminations.push((cell, digit));
                    }
                }
            }

            if !eliminations.is_empty() {
                return Some(Deduction::eliminations(technique, eliminations));
            }
        }
    }

    None
}