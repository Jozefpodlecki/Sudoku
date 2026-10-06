use alloc::vec::Vec;

use crate::bits::Digit;
use crate::cell::Cell;
use crate::difficulty::GameDifficulty;
use crate::grid::Grid;
use crate::solver::Solver;
use crate::techniques::{Deduction, LogicalSolver, Technique};

pub const THRESHOLDS: [(u32, GameDifficulty); 5] = [
    (0, GameDifficulty::Easy),
    (60, GameDifficulty::Medium),
    (120, GameDifficulty::Hard),
    (200, GameDifficulty::Expert),
    (300, GameDifficulty::Extreme),
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rating {
    pub score: u32,
    pub max_technique: Technique,
    pub technique_counts: Vec<(Technique, u32)>,
    pub steps: u32,
    pub guesses: u32,
    pub solved_logically: bool,
}

impl Rating {
    pub fn bucket(&self) -> GameDifficulty {
        THRESHOLDS
            .iter()
            .rev()
            .find(|(threshold, _)| self.score >= *threshold)
            .map(|(_, tier)| *tier)
            .unwrap_or(GameDifficulty::Easy)
    }

    pub fn has_technique(&self, technique: Technique) -> bool {
        self.technique_counts
            .iter()
            .any(|(candidate, _)| *candidate == technique)
    }

    pub fn count_of(&self, technique: Technique) -> u32 {
        self.technique_counts
            .iter()
            .find(|(candidate, _)| *candidate == technique)
            .map(|(_, count)| *count)
            .unwrap_or(0)
    }
}

pub struct Rater;

impl Rater {
    pub fn rate(grid: &Grid) -> Rating {
        let mut working = *grid;
        let mut technique_counts: Vec<(Technique, u32)> = Vec::new();
        let mut score = 0u32;
        let mut steps = 0u32;
        let mut max_technique = Technique::NakedSingle;
        let mut solved_logically = false;

        loop {
            if working.is_solved() {
                solved_logically = true;
                break;
            }

            let Some(deduction) = LogicalSolver::step(&working) else {
                break;
            };

            let technique = deduction.technique;
            score += technique.weight();
            steps += 1;

            if technique.weight() > max_technique.weight() {
                max_technique = technique;
            }

            if let Some(entry) = technique_counts
                .iter_mut()
                .find(|(candidate, _)| *candidate == technique)
            {
                entry.1 += 1;
            } else {
                technique_counts.push((technique, 1));
            }

            apply_deduction(&mut working, &deduction);
        }

        let guesses = if solved_logically {
            0
        } else {
            count_backtracks(grid)
        };

        if guesses > 0 {
            score += guesses * 50;
        }

        Rating {
            score,
            max_technique,
            technique_counts,
            steps,
            guesses,
            solved_logically,
        }
    }
}

fn apply_deduction(grid: &mut Grid, deduction: &Deduction) {
    for (cell, digit) in &deduction.eliminations {
        grid.eliminate(*cell, *digit);
    }

    for (cell, digit) in &deduction.placements {
        let _ = grid.place(*cell, *digit);
    }
}

fn count_backtracks(grid: &Grid) -> u32 {
    let mut working = *grid;
    let mut guesses = 0u32;

    loop {
        if working.is_solved() {
            break;
        }

        if let Some(deduction) = LogicalSolver::step(&working) {
            apply_deduction(&mut working, &deduction);
            continue;
        }

        let Some((cell, digit)) = first_candidate(&working) else {
            break;
        };

        let _ = working.place(cell, digit);
        guesses += 1;
    }

    if working.is_solved() {
        guesses
    } else if Solver::solve(grid).is_some() {
        guesses + 1
    } else {
        guesses
    }
}

fn first_candidate(grid: &Grid) -> Option<(Cell, Digit)> {
    let mut best: Option<(Cell, Digit)> = None;
    let mut best_count = u32::MAX;

    for cell in Cell::all() {
        if !grid.is_empty_at(cell) {
            continue;
        }
        let candidates = grid.candidates(cell);
        let count = candidates.len();
        if count == 0 {
            return None;
        }
        if count < best_count {
            if let Some(digit) = candidates.lowest() {
                best_count = count;
                best = Some((cell, digit));
            }
        }
    }

    best
}