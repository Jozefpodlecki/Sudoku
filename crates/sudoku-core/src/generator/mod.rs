mod carve;
mod fill;
mod generation;

pub use generation::{Generation, DEFAULT_MAX_ATTEMPTS};

use crate::difficulty::GameDifficulty;
use crate::grid::Grid;

pub struct Generator;

impl Generator {
    pub fn generation(difficulty: GameDifficulty, seed: u64) -> Generation {
        Generation::new(difficulty, seed)
    }

    pub fn generate(difficulty: GameDifficulty, seed: u64) -> Grid {
        let mut generation = Generation::new(difficulty, seed);
        
        for attempt in generation.by_ref() {
            if attempt.matches_target {
                return attempt.grid;
            }
        }

        generation.finish()
    }
}