
use alloc::vec::Vec;

use rand::seq::SliceRandom;
use rand_pcg::Pcg32;

use crate::cell::Cell;
use crate::grid::Grid;
use crate::solver::Solver;

pub(crate) fn carve(mut grid: Grid, target_clues: usize, rng: &mut Pcg32) -> Grid {
    let mut order: Vec<Cell> = Cell::all().to_vec();
    order.shuffle(rng);

    let mut clues = 81usize;

    for cell in order {
        if clues <= target_clues {
            break;
        }

        let Some(digit) = grid.remove(cell) else {
            continue;
        };

        if Solver::is_unique(&grid) {
            clues -= 1;
        } else {
            let _ = grid.place(cell, digit);
        }
    }

    grid
}