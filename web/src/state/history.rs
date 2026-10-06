use alloc::{collections::VecDeque, vec::Vec};
use sudoku_core::grid::Grid;

const MAX_DEPTH: usize = 128;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct History {
    undo: VecDeque<Snapshot>,
    redo: Vec<Snapshot>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Snapshot {
    pub grid: Grid,
    pub mistakes: u32,
}

impl History {
    pub fn new() -> Self {
        Self {
            undo: VecDeque::with_capacity(MAX_DEPTH),
            redo: Vec::new(),
        }
    }

    pub fn push(&mut self, snapshot: Snapshot) {
        if self.undo.len() == MAX_DEPTH {
            self.undo.pop_front();
        }
        self.undo.push_back(snapshot);
        self.redo.clear();
    }

    pub fn undo(&mut self, current: Snapshot) -> Option<Snapshot> {
        let previous = self.undo.pop_back()?;
        if self.redo.len() == MAX_DEPTH {
            self.redo.remove(0);
        }
        self.redo.push(current);
        Some(previous)
    }

    pub fn redo(&mut self, current: Snapshot) -> Option<Snapshot> {
        let next = self.redo.pop()?;
        if self.undo.len() == MAX_DEPTH {
            self.undo.pop_front();
        }
        self.undo.push_back(current);
        Some(next)
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    pub fn clear(&mut self) {
        self.undo.clear();
        self.redo.clear();
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}