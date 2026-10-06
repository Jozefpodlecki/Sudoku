use alloc::vec::Vec;
use core::ops::{Index, IndexMut};

use super::bridge::from_grid;
use super::build::from_scratch;
use super::placement::Placement;
use super::query::{count, solve};

pub(crate) const CELLS: usize = 81;
pub(crate) const DIGITS: usize = 9;
pub(crate) const ROWS: usize = CELLS * DIGITS;
pub(crate) const COLS: usize = CELLS * 4;

pub(crate) const COL_OFFSET_CELL: usize = 1;
pub(crate) const COL_OFFSET_ROW: usize = 1 + CELLS;
pub(crate) const COL_OFFSET_COL: usize = 1 + CELLS * 2;
pub(crate) const COL_OFFSET_BOX: usize = 1 + CELLS * 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Node {
    pub left: u32,
    pub right: u32,
    pub up: u32,
    pub down: u32,
    pub column: u32,
    pub row: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExactCover {
    pub(crate) nodes: Vec<Node>,
    pub(crate) givens: [u8; CELLS],
}

impl ExactCover {
    pub fn new() -> Self {
        from_scratch()
    }

    pub fn from_grid(grid: &crate::grid::Grid) -> Self {
        from_grid(grid)
    }

    pub fn solve(&mut self) -> Option<Vec<Placement>> {
        solve(self)
    }

    pub fn count_solutions(&mut self, limit: usize) -> usize {
        count(self, limit)
    }

    pub(crate) fn cover(&mut self, column: u32) {
        let right = self.nodes[column as usize].right;
        let left = self.nodes[column as usize].left;

        self.nodes[right as usize].left = left;
        self.nodes[left as usize].right = right;

        let mut row = self.nodes[column as usize].down;

        while row != column {
            let mut node = self.nodes[row as usize].right;

            while node != row {
                let down = self.nodes[node as usize].down;
                let up = self.nodes[node as usize].up;

                self.nodes[down as usize].up = up;
                self.nodes[up as usize].down = down;

                node = self.nodes[node as usize].right;
            }

            row = self.nodes[row as usize].down;
        }
    }

    pub(crate) fn uncover(&mut self, column: u32) {
        let mut row = self.nodes[column as usize].up;

        while row != column {
            let mut node = self.nodes[row as usize].left;

            while node != row {
                let down = self.nodes[node as usize].down;
                let up = self.nodes[node as usize].up;

                self.nodes[down as usize].up = node;
                self.nodes[up as usize].down = node;

                node = self.nodes[node as usize].left;
            }

            row = self.nodes[row as usize].up;
        }

        let right = self.nodes[column as usize].right;
        let left = self.nodes[column as usize].left;

        self.nodes[right as usize].left = column;
        self.nodes[left as usize].right = column;
    }

    pub(crate) fn choose_column(&self) -> Option<u32> {
        let mut best = None;
        let mut best_size = u32::MAX;

        let mut column = self.nodes[0].right;

        while column != 0 {
            let mut size = 0u32;
            let mut node = self.nodes[column as usize].down;

            while node != column {
                size += 1;
                node = self.nodes[node as usize].down;
            }

            if size < best_size {
                best_size = size;
                best = Some(column);

                if size <= 1 {
                    break;
                }
            }

            column = self.nodes[column as usize].right;
        }

        best
    }

    pub(crate) fn unlink_row(&mut self, row: u32) {
        let first = (COLS + 1) as u32 + row * 4;
        let mut node = first;

        for _ in 0..4 {
            let up = self.nodes[node as usize].up;
            let down = self.nodes[node as usize].down;

            self.nodes[up as usize].down = down;
            self.nodes[down as usize].up = up;

            node = self.nodes[node as usize].right;
        }

        debug_assert_eq!(node, first);
    }
}

impl Default for ExactCover {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<u32> for ExactCover {
    type Output = Node;

    fn index(&self, index: u32) -> &Self::Output {
        &self.nodes[index as usize]
    }
}

impl IndexMut<u32> for ExactCover {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        &mut self.nodes[index as usize]
    }
}