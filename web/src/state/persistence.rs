use alloc::string::ToString;
use core::time::Duration;

use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use sudoku_core::bits::Digit;
use sudoku_core::cell::Cell;
use sudoku_core::difficulty::GameDifficulty;
use sudoku_core::grid::Grid;
use sudoku_core::solver::Solver;
use sudoku_core::Mask;

use super::game::GameState;
use super::history::History;
use super::selection::Selection;
use crate::storage::StorageService;

const STORAGE_KEY: &str = "sudoku.game";

#[derive(Clone, Serialize, Deserialize)]
struct PersistedState {
    #[serde(with = "BigArray")]
    values: [u8; 81],
    #[serde(with = "BigArray")]
    givens: [bool; 81],
    selection: Option<u8>,
    mistakes: u32,
    elapsed_seconds: u64,
    difficulty: alloc::string::String,
    paused: bool,
}

fn storage() -> StorageService {
    StorageService::new(STORAGE_KEY)
}

pub fn save_state(state: &GameState) {
    let persisted = PersistedState {
        values: *state.grid.values(),
        givens: state.givens,
        selection: state.selection.anchor.map(|cell| cell.index() as u8),
        mistakes: state.mistakes,
        elapsed_seconds: state.elapsed.as_secs(),
        difficulty: state.difficulty.slug().to_string(),
        paused: state.paused,
    };

    let _ = storage().save(&persisted);
}

pub fn load_state() -> Option<GameState> {
    let persisted: PersistedState = storage().load()?;

    let mut grid = Grid::empty();
    for (index, &value) in persisted.values.iter().enumerate() {
        if value == 0 {
            continue;
        }
        let cell = Cell::from_index(index)?;
        let digit = Digit::new(value)?;
        if grid.place(cell, digit).is_err() {
            return None;
        }
    }

    let mut selection = Selection::new();
    if let Some(index) = persisted.selection {
        if let Some(cell) = Cell::from_index(index as usize) {
            selection.select(cell);
        }
    }

    let difficulty = GameDifficulty::from_slug(&persisted.difficulty).unwrap_or_default();
    let solution = Solver::solve(&grid).unwrap_or(grid);

    Some(GameState {
        grid,
        solution,
        givens: persisted.givens,
        user_eliminated: [Mask::EMPTY; 81],
        selection,
        hint: None,
        candidate_mode: false,
        history: History::new(),
        mistakes: persisted.mistakes,
        started_at: 0.0,
        elapsed: Duration::from_secs(persisted.elapsed_seconds),
        paused: persisted.paused,
        difficulty,
        generating: false,
        pending_difficulty: None,
        generation_progress: None,
        solved: grid.is_solved(),
    })
}

pub fn clear_state() {
    storage().remove();
}

pub fn has_saved_game() -> bool {
    storage().exists()
}

pub fn saved_game_values() -> Option<[u8; 81]> {
    let persisted: PersistedState = storage().load()?;
    Some(persisted.values)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SavedGameSummary {
    pub difficulty: GameDifficulty,
    pub elapsed_seconds: u64,
    pub mistakes: u32,
}

pub fn saved_game_summary() -> Option<SavedGameSummary> {
    let persisted: PersistedState = storage().load()?;
    let difficulty = GameDifficulty::from_slug(&persisted.difficulty).unwrap_or_default();

    Some(SavedGameSummary {
        difficulty,
        elapsed_seconds: persisted.elapsed_seconds,
        mistakes: persisted.mistakes,
    })
}