use alloc::vec::Vec;

use super::combinations::Combinations;
use super::{Deduction, Technique};
use crate::bits::Digit;
use crate::cell::Cell;
use crate::grid::Grid;
use crate::peers::{COL_CELLS, ROW_CELLS};

pub(super) fn find_x_wing(grid: &Grid) -> Option<Deduction> {
    for digit in Digit::all() {
        let rows: Vec<(u8, u8, u8)> = (0..9u8)
            .filter_map(|row| {
                let positions: Vec<u8> = ROW_CELLS[row as usize]
                    .iter()
                    .map(|&index| Cell(index))
                    .filter(|&cell| grid.is_empty_at(cell) && grid.candidates(cell).contains(digit))
                    .map(|cell| cell.col())
                    .collect();

                if positions.len() == 2 {
                    Some((row, positions[0], positions[1]))
                } else {
                    None
                }
            })
            .collect();

        for outer in 0..rows.len() {
            for inner in (outer + 1)..rows.len() {
                let (row_a, a_col_a, a_col_b) = rows[outer];
                let (row_b, b_col_a, b_col_b) = rows[inner];

                if a_col_a != b_col_a || a_col_b != b_col_b {
                    continue;
                }

                let mut eliminations = Vec::new();
                for col in [a_col_a, a_col_b] {
                    for &cell_index in &COL_CELLS[col as usize] {
                        let cell = Cell(cell_index);
                        if cell.row() == row_a || cell.row() == row_b {
                            continue;
                        }
                        if grid.is_empty_at(cell) && grid.candidates(cell).contains(digit) {
                            eliminations.push((cell, digit));
                        }
                    }
                }

                if !eliminations.is_empty() {
                    return Some(Deduction::eliminations(Technique::XWing, eliminations));
                }
            }
        }
    }
    None
}

pub(super) fn find_swordfish(grid: &Grid) -> Option<Deduction> {
    for digit in Digit::all() {
        let rows: Vec<(u8, Vec<u8>)> = (0..9u8)
            .filter_map(|row| {
                let positions: Vec<u8> = ROW_CELLS[row as usize]
                    .iter()
                    .map(|&index| Cell(index))
                    .filter(|&cell| grid.is_empty_at(cell) && grid.candidates(cell).contains(digit))
                    .map(|cell| cell.col())
                    .collect();

                if (2..=3).contains(&positions.len()) {
                    Some((row, positions))
                } else {
                    None
                }
            })
            .collect();

        if rows.len() < 3 {
            continue;
        }

        for combination in Combinations::new(rows.len(), 3) {
            let selected: Vec<&(u8, Vec<u8>)> = combination.iter().map(|&i| &rows[i]).collect();

            let mut all_columns: Vec<u8> = Vec::new();
            for (_, cols) in &selected {
                for &col in cols {
                    if !all_columns.contains(&col) {
                        all_columns.push(col);
                    }
                }
            }

            if all_columns.len() != 3 {
                continue;
            }

            let selected_rows: Vec<u8> = selected.iter().map(|(row, _)| *row).collect();
            let mut eliminations = Vec::new();

            for &col in &all_columns {
                for &cell_index in &COL_CELLS[col as usize] {
                    let cell = Cell(cell_index);
                    if selected_rows.contains(&cell.row()) {
                        continue;
                    }
                    if grid.is_empty_at(cell) && grid.candidates(cell).contains(digit) {
                        eliminations.push((cell, digit));
                    }
                }
            }

            if !eliminations.is_empty() {
                return Some(Deduction::eliminations(Technique::Swordfish, eliminations));
            }
        }
    }
    None
}