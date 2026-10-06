use alloc::vec::Vec;

use super::{Deduction, Technique};
use crate::bits::Digit;
use crate::cell::Cell;
use crate::grid::Grid;

pub(super) fn find_xy_wing(grid: &Grid) -> Option<Deduction> {
    let bivalue_cells: Vec<Cell> = Cell::all()
        .into_iter()
        .filter(|&cell| grid.is_empty_at(cell) && grid.candidates(cell).len() == 2)
        .collect();

    for &pivot in &bivalue_cells {
        let pivot_candidates = grid.candidates(pivot);

        for &wing_a in &pivot.peers() {
            if !bivalue_cells.contains(&wing_a) {
                continue;
            }
            let a_candidates = grid.candidates(wing_a);
            let a_overlap = a_candidates.intersect(pivot_candidates);
            if a_overlap.len() != 1 {
                continue;
            }

            for &wing_b in &pivot.peers() {
                if wing_b == wing_a || !bivalue_cells.contains(&wing_b) {
                    continue;
                }
                let b_candidates = grid.candidates(wing_b);
                let b_overlap = b_candidates.intersect(pivot_candidates);
                if b_overlap.len() != 1 || b_overlap == a_overlap {
                    continue;
                }

                let z = a_candidates
                    .difference(pivot_candidates)
                    .intersect(b_candidates.difference(pivot_candidates));

                let z_digits: Vec<Digit> = z.iter().collect();
                if z_digits.len() != 1 {
                    continue;
                }
                let z_digit = z_digits[0];

                let mut eliminations = Vec::new();
                for &other in &wing_a.peers() {
                    if other == pivot || other == wing_b {
                        continue;
                    }
                    if grid.is_empty_at(other) && grid.candidates(other).contains(z_digit) {
                        eliminations.push((other, z_digit));
                    }
                }

                if !eliminations.is_empty() {
                    return Some(Deduction::eliminations(Technique::XYWing, eliminations));
                }
            }
        }
    }

    None
}