use rand::SeedableRng;
use rand_pcg::Pcg32;

use crate::difficulty::GameDifficulty;
use crate::grid::Grid;
use crate::rating::{Rater, Rating};

use super::carve::carve;
use super::fill::random_full_grid;

pub const DEFAULT_MAX_ATTEMPTS: u32 = 64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Attempt {
    pub index: u32,
    pub grid: Grid,
    pub rating: Rating,
    pub matches_target: bool,
}

impl Attempt {
    pub fn bucket(&self) -> GameDifficulty {
        self.rating.bucket()
    }

    pub fn score(&self) -> u32 {
        self.rating.score
    }

    pub fn distance_to(&self, target: GameDifficulty) -> u32 {
        bucket_distance(&self.rating, target)
    }

    pub fn is_better_than(&self, other: &Attempt, target: GameDifficulty) -> bool {
        self.distance_to(target) < other.distance_to(target)
    }

    pub fn clue_count(&self) -> usize {
        self.grid
            .values()
            .iter()
            .filter(|value| **value != 0)
            .count()
    }
}

pub struct Generation {
    rng: Pcg32,
    difficulty: GameDifficulty,
    target_clues: usize,
    attempts: u32,
    max_attempts: u32,
    best: Option<Grid>,
    best_distance: u32,
    finished: bool,
}

impl Generation {
    pub fn new(difficulty: GameDifficulty, seed: u64) -> Self {
        Self {
            rng: Pcg32::seed_from_u64(seed),
            difficulty,
            target_clues: difficulty.target_clues(),
            attempts: 0,
            max_attempts: DEFAULT_MAX_ATTEMPTS,
            best: None,
            best_distance: u32::MAX,
            finished: false,
        }
    }

    pub fn with_max_attempts(mut self, max_attempts: u32) -> Self {
        self.max_attempts = max_attempts;
        self
    }

    pub fn difficulty(&self) -> GameDifficulty {
        self.difficulty
    }

    pub fn attempts_so_far(&self) -> u32 {
        self.attempts
    }

    pub fn max_attempts(&self) -> u32 {
        self.max_attempts
    }

    pub fn best(&self) -> Option<&Grid> {
        self.best.as_ref()
    }

    pub fn best_distance(&self) -> u32 {
        self.best_distance
    }

    pub fn finish(self) -> Grid {
        self.best.unwrap_or_else(|| {
            let mut rng = Pcg32::seed_from_u64(0);
            random_full_grid(&mut rng)
        })
    }
}

impl Iterator for Generation {
    type Item = Attempt;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished || self.attempts >= self.max_attempts {
            self.finished = true;
            return None;
        }

        self.attempts += 1;

        let full = random_full_grid(&mut self.rng);
        let carved = carve(full, self.target_clues, &mut self.rng);
        let rating = Rater::rate(&carved);
        let matches_target = rating.bucket() == self.difficulty;

        let distance = bucket_distance(&rating, self.difficulty);
        if distance < self.best_distance {
            self.best_distance = distance;
            self.best = Some(carved);
        }

        if matches_target {
            self.finished = true;
        }

        Some(Attempt {
            index: self.attempts,
            grid: carved,
            rating,
            matches_target,
        })
    }
}

pub(crate) fn bucket_distance(rating: &Rating, target: GameDifficulty) -> u32 {
    let actual = rating.bucket() as u32;
    let target = target as u32;
    actual.abs_diff(target)
}