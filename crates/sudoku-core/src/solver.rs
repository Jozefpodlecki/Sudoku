
use crate::dlx::ExactCover;
use crate::error::SudokuError;
use crate::grid::Grid;

pub struct Solver;

impl Solver {
    pub fn solve(grid: &Grid) -> Option<Grid> {
        let placements = ExactCover::from_grid(grid).solve()?;
        let mut solved = *grid;
        for placement in placements {
            if solved.is_empty_at(placement.cell) {
                let _ = solved.place(placement.cell, placement.digit);
            }
        }
        Some(solved)
    }

    pub fn solve_into(grid: &Grid, target: &mut Grid) -> Result<(), SudokuError> {
        let placements = ExactCover::from_grid(grid)
            .solve()
            .ok_or(SudokuError::NoSolution)?;
        for placement in placements {
            if target.is_empty_at(placement.cell) {
                target.place(placement.cell, placement.digit)?;
            }
        }
        Ok(())
    }

    pub fn count_solutions(grid: &Grid, limit: usize) -> usize {
        ExactCover::from_grid(grid).count_solutions(limit)
    }

    pub fn is_unique(grid: &Grid) -> bool {
        Self::count_solutions(grid, 2) == 1
    }
}