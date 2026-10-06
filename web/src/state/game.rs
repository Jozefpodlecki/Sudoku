use std::rc::Rc;

use gloo::console::info;
use sudoku_core::bits::Digit;
use sudoku_core::cell::Cell;
use sudoku_core::difficulty::GameDifficulty;
use sudoku_core::generator::Generator;
use sudoku_core::grid::Grid;
use sudoku_core::solver::Solver;
use sudoku_core::techniques::LogicalSolver;
use sudoku_core::Mask;
use yew::Reducible;

use crate::utils::now;

use super::history::{History, Snapshot};
use super::hint::HintState;
use super::selection::Selection;

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    Select(Cell),
    Input(Digit),
    Clear,
    ToggleCandidateMode,
    RequestHint,
    ApplyHint,
    DismissHint,
    SolveStep,
    Undo,
    Redo,
    NewGameRequested(GameDifficulty),
    GenerationProgressed(GenerationProgress),
    NewGameReady(Grid),
    Tick,
    TogglePause,
}

impl Action {
    pub fn from_data_attribute(value: &str) -> Option<Self> {
        match value {
            "hint" => Some(Action::RequestHint),
            "apply-hint" => Some(Action::ApplyHint),
            "dismiss-hint" => Some(Action::DismissHint),
            "undo" => Some(Action::Undo),
            "redo" => Some(Action::Redo),
            "candidates" => Some(Action::ToggleCandidateMode),
            "solve-step" => Some(Action::SolveStep),
            _ => None,
        }
    }

    pub fn data_attribute(&self) -> Option<&'static str> {
        match self {
            Action::RequestHint => Some("hint"),
            Action::ApplyHint => Some("apply-hint"),
            Action::DismissHint => Some("dismiss-hint"),
            Action::SolveStep => Some("solve-step"),
            Action::Undo => Some("undo"),
            Action::Redo => Some("redo"),
            Action::ToggleCandidateMode => Some("candidates"),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GameState {
    pub grid: Grid,
    pub solution: Grid,
    pub givens: [bool; 81],
    pub user_eliminated: [Mask; 81],
    pub selection: Selection,
    pub hint: Option<HintState>,
    pub candidate_mode: bool,
    pub history: History,
    pub mistakes: u32,
    pub started_at: f64,
    pub elapsed: core::time::Duration,
    pub paused: bool,
    pub difficulty: GameDifficulty,
    pub generating: bool,
    pub pending_difficulty: Option<GameDifficulty>,
    pub generation_progress: Option<GenerationProgress>,
    pub solved: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GenerationProgress {
    pub attempts: u32,
    pub max_attempts: u32,
    pub best_bucket: Option<GameDifficulty>,
    pub best_distance: u32,
}

impl GameState {
    pub fn new(difficulty: GameDifficulty, seed: u64) -> Self {
        let grid = Generator::generate(difficulty, seed);
        let solution = Solver::solve(&grid).unwrap_or(grid);

        let mut givens = [false; 81];
        for (index, cell) in Cell::all().iter().enumerate() {
            givens[index] = !grid.is_empty_at(*cell);
        }

        Self {
            grid,
            solution,
            givens,
            user_eliminated: [Mask::EMPTY; 81],
            selection: Selection::new(),
            hint: None,
            candidate_mode: false,
            history: History::new(),
            mistakes: 0,
            started_at: now(),
            elapsed: core::time::Duration::ZERO,
            paused: false,
            difficulty,
            generating: false,
            generation_progress: None,
            pending_difficulty: None,
            solved: false,
        }
    }

    pub fn effective_candidates(&self, cell: Cell) -> Mask {
        self.grid
            .candidates(cell)
            .difference(self.user_eliminated[cell.index()])
    }

    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            grid: self.grid,
            mistakes: self.mistakes,
        }
    }

    pub fn restore(&mut self, snapshot: Snapshot) {
        self.grid = snapshot.grid;
        self.mistakes = snapshot.mistakes;
        self.hint = None;
        self.solved = self.grid.is_solved();
    }

    pub const fn values_match_solution(&self) -> bool {
        self.grid.values() == self.solution.values()
    }

    pub fn allowed_digits(&self) -> Mask {
        let Some(cell) = self.selection.anchor else {
            return Mask::EMPTY;
        };
        
        if self.givens[cell.index()] {
            return Mask::EMPTY;
        }
        
        if self.grid.value(cell) != 0 {
            return Mask::FULL;
        }

        self.effective_candidates(cell)
    }

    pub fn apply_new_game(&mut self, difficulty: GameDifficulty, grid: Grid) {
        let solution = Solver::solve(&grid).unwrap_or(grid);

        let mut givens = [false; 81];
        for (index, cell) in Cell::all().iter().enumerate() {
            givens[index] = !grid.is_empty_at(*cell);
        }

        self.grid = grid;
        self.solution = solution;
        self.givens = givens;
        self.user_eliminated = [Mask::EMPTY; 81];
        self.selection = Selection::new();
        self.hint = None;
        self.candidate_mode = false;
        self.history = History::new();
        self.mistakes = 0;
        self.started_at = now();
        self.elapsed = core::time::Duration::ZERO;
        self.paused = false;
        self.difficulty = difficulty;
        self.generating = false;
        self.pending_difficulty = None;
        self.solved = false;
    }
}

impl Reducible for GameState {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut next = (*self).clone();

        match action {
            Action::Select(cell) => {
                if next.selection.anchor == Some(cell) {
                    next.selection.clear();
                } else {
                    next.selection.select(cell);
                }
                return Rc::new(next);
            }

            Action::Input(digit) => {
                
                let Some(cell) = next.selection.anchor else {
                    return self;
                };

                if next.givens[cell.index()] {
                    return self;
                }

                let snapshot = next.snapshot();
                let mut grid = next.grid;

                if next.candidate_mode {
                    if grid.value(cell) != 0 {
                        return self;
                    }
                    next.user_eliminated[cell.index()] =
                        next.user_eliminated[cell.index()].toggle(digit);
                    next.hint = None;
                    return Rc::new(next);
                }

                if grid.value(cell) != 0 {
                    grid.remove(cell);
                }

                if grid.place(cell, digit).is_err() {
                    return self;
                }

                if next.solution.value(cell) != digit.get() {
                    next.mistakes += 1;
                }

                next.history.push(snapshot);
                next.grid = grid;
                next.hint = None;
                next.solved = next.grid.is_solved();
            }

            Action::Clear => {
                
                let Some(cell) = next.selection.anchor else {
                    return self;
                };

                if next.givens[cell.index()] {
                    return self;
                }

                let snapshot = next.snapshot();
                let mut grid = next.grid;
                if grid.remove(cell).is_none() {
                    next.user_eliminated[cell.index()] = Mask::EMPTY;
                    return Rc::new(next);
                }

                next.user_eliminated[cell.index()] = Mask::EMPTY;
                next.history.push(snapshot);
                next.grid = grid;
                next.hint = None;
                next.solved = false;
            }

            Action::ToggleCandidateMode => {
                next.candidate_mode = !next.candidate_mode;
            }

            Action::RequestHint => {
                if next.solved || next.paused {
                    return self;
                }
                let deduction = LogicalSolver::step(&next.grid);
                next.hint = deduction.as_ref().map(HintState::from_deduction);
            }
            Action::SolveStep => {
                if next.solved || next.paused {
                    return self;
                }

                let Some(deduction) = LogicalSolver::step(&next.grid) else {
                    return self;
                };

                if deduction.placements.is_empty() {
                    return self;
                }

                let snapshot = next.snapshot();
                let mut grid = next.grid;

                for (cell, digit) in &deduction.placements {
                    grid.force_place(*cell, *digit);
                }

                next.history.push(snapshot);
                next.grid = grid;
                next.hint = None;
                next.solved = next.values_match_solution();
            }
            Action::ApplyHint => {
                let Some(hint) = next.hint.clone() else {
                    return self;
                };
                if hint.placements.is_empty() {
                    next.hint = None;
                    return Rc::new(next);
                }

                let snapshot = next.snapshot();
                let mut grid = next.grid;
                for (cell, digit) in &hint.placements {
                    let _ = grid.place(*cell, *digit);
                }

                next.history.push(snapshot);
                next.grid = grid;
                next.hint = None;
                next.solved = next.grid.is_solved();
            }

            Action::DismissHint => {
                next.hint = None;
            }

            Action::Undo => {
                let current = next.snapshot();
                match next.history.undo(current) {
                    Some(snapshot) => next.restore(snapshot),
                    None => return self,
                }
            }

            Action::Redo => {
                let current = next.snapshot();
                match next.history.redo(current) {
                    Some(snapshot) => next.restore(snapshot),
                    None => return self,
                }
            }

            Action::NewGameRequested(difficulty) => {
                next.generating = true;
                next.pending_difficulty = Some(difficulty);
            }

            Action::NewGameReady(grid) => {
                let difficulty = next.pending_difficulty.unwrap_or(next.difficulty);
                next.apply_new_game(difficulty, grid);
            }

            Action::Tick => {
                if next.paused || next.solved {
                    return self;
                }
                next.elapsed += core::time::Duration::from_millis(1000);
            }

            Action::TogglePause => {
                next.paused = !next.paused;
            }
            Action::GenerationProgressed(progress) => {
                next.generation_progress = Some(progress);
            }
        }

        Rc::new(next)
    }
}