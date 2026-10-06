mod combinations;
mod fish;
mod locked;
mod singles;
mod subsets;
mod wings;

use alloc::vec::Vec;

use crate::bits::Digit;
use crate::cell::Cell;
use crate::grid::Grid;

pub use combinations::Combinations;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Technique {
    NakedSingle,
    HiddenSingle,
    LockedCandidatePointing,
    LockedCandidateClaiming,
    NakedPair,
    NakedTriple,
    HiddenPair,
    HiddenTriple,
    XWing,
    Swordfish,
    XYWing,
}

impl Technique {
    pub const fn label(self) -> &'static str {
        match self {
            Self::NakedSingle => "Naked Single",
            Self::HiddenSingle => "Hidden Single",
            Self::LockedCandidatePointing => "Locked Candidate (Pointing)",
            Self::LockedCandidateClaiming => "Locked Candidate (Claiming)",
            Self::NakedPair => "Naked Pair",
            Self::NakedTriple => "Naked Triple",
            Self::HiddenPair => "Hidden Pair",
            Self::HiddenTriple => "Hidden Triple",
            Self::XWing => "X-Wing",
            Self::Swordfish => "Swordfish",
            Self::XYWing => "XY-Wing",
        }
    }

    pub const fn weight(self) -> u32 {
        match self {
            Self::NakedSingle => 1,
            Self::HiddenSingle => 2,
            Self::LockedCandidatePointing => 5,
            Self::LockedCandidateClaiming => 5,
            Self::NakedPair => 10,
            Self::NakedTriple => 15,
            Self::HiddenPair => 12,
            Self::HiddenTriple => 18,
            Self::XWing => 25,
            Self::Swordfish => 35,
            Self::XYWing => 40,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Deduction {
    pub technique: Technique,
    pub placements: Vec<(Cell, Digit)>,
    pub eliminations: Vec<(Cell, Digit)>,
}

impl Deduction {
    pub(crate) fn placement(technique: Technique, cell: Cell, digit: Digit) -> Self {
        Self {
            technique,
            placements: alloc::vec![(cell, digit)],
            eliminations: Vec::new(),
        }
    }

    pub(crate) fn eliminations(
        technique: Technique,
        entries: Vec<(Cell, Digit)>,
    ) -> Self {
        Self {
            technique,
            placements: Vec::new(),
            eliminations: entries,
        }
    }
}

pub struct LogicalSolver;

impl LogicalSolver {
    pub fn step(grid: &Grid) -> Option<Deduction> {
        if let Some(deduction) = singles::find_naked_single(grid) {
            return Some(deduction);
        }
        if let Some(deduction) = singles::find_hidden_single(grid) {
            return Some(deduction);
        }
        if let Some(deduction) = locked::find_pointing(grid) {
            return Some(deduction);
        }
        if let Some(deduction) = locked::find_claiming(grid) {
            return Some(deduction);
        }
        if let Some(deduction) = subsets::find_naked_pair(grid) {
            return Some(deduction);
        }
        if let Some(deduction) = subsets::find_hidden_pair(grid) {
            return Some(deduction);
        }
        if let Some(deduction) = subsets::find_naked_triple(grid) {
            return Some(deduction);
        }
        if let Some(deduction) = subsets::find_hidden_triple(grid) {
            return Some(deduction);
        }
        if let Some(deduction) = fish::find_x_wing(grid) {
            return Some(deduction);
        }
        if let Some(deduction) = fish::find_swordfish(grid) {
            return Some(deduction);
        }
        if let Some(deduction) = wings::find_xy_wing(grid) {
            return Some(deduction);
        }
        None
    }

    pub fn apply(grid: &mut Grid, deduction: &Deduction) {
        for (cell, digit) in &deduction.placements {
            let _ = grid.place(*cell, *digit);
        }
    }
}