use super::cover::{ExactCover, ROWS};
use crate::cell::Cell;
use crate::grid::Grid;

pub(super) fn from_grid(grid: &Grid) -> ExactCover {
    let mut exact = ExactCover::new();
    let mut removed = [false; ROWS];

    for cell in Cell::all() {
        let value = grid.value(cell);
        if value == 0 {
            continue;
        }
        exact.givens[cell.index()] = value;
    }

    for cell in Cell::all() {
        let value = grid.value(cell);
        if value == 0 {
            continue;
        }

        let given_cell = cell.index();
        let digit = (value - 1) as usize;
        let given_row = given_cell * 9 + digit;
        let given_r = given_cell / 9;
        let given_c = given_cell % 9;
        let given_b = (given_r / 3) * 3 + given_c / 3;

        for other_row in 0..ROWS {
            if other_row == given_row || removed[other_row] {
                continue;
            }

            let other_cell = other_row / 9;
            let other_digit = other_row % 9;
            let other_r = other_cell / 9;
            let other_c = other_cell % 9;
            let other_b = (other_r / 3) * 3 + other_c / 3;

            let conflict = other_cell == given_cell
                || (other_r == given_r && other_digit == digit)
                || (other_c == given_c && other_digit == digit)
                || (other_b == given_b && other_digit == digit);

            if conflict {
                exact.unlink_row(other_row as u32);
                removed[other_row] = true;
            }
        }
    }

    exact
}