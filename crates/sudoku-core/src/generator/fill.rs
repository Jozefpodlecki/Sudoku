
use alloc::vec::Vec;

use rand::seq::SliceRandom;
use rand_pcg::Pcg32;

use crate::bits::Digit;
use crate::cell::Cell;
use crate::grid::Grid;

pub(crate) fn random_full_grid(rng: &mut Pcg32) -> Grid {
    let mut grid = Grid::empty();
    fill(&mut grid, rng);
    grid
}

pub(crate) fn fill(grid: &mut Grid, rng: &mut Pcg32) -> bool {
    let Some(cell) = select_mrv_cell(grid) else {
        return grid.is_solved();
    };

    let mut digits: Vec<Digit> = grid.candidates(cell).iter().collect();
    digits.shuffle(rng);

    for digit in digits {
        if grid.place(cell, digit).is_err() {
            continue;
        }

        if fill(grid, rng) {
            return true;
        }

        grid.remove(cell);
    }

    false
}

fn select_mrv_cell(grid: &Grid) -> Option<Cell> {
    let mut best: Option<Cell> = None;
    let mut best_count = u32::MAX;

    for cell in Cell::all() {
        if !grid.is_empty_at(cell) {
            continue;
        }

        let count = grid.candidates(cell).len();

        if count == 0 {
            return None;
        }

        if count < best_count {
            best_count = count;
            best = Some(cell);

            if count == 1 {
                break;
            }
        }
    }

    best
}