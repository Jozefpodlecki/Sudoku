use sudoku_core::cell::Cell;
use sudoku_core::grid::Grid;
use sudoku_core::peers::PEERS;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Selection {
    pub anchor: Option<Cell>,
}

impl Selection {
    pub fn new() -> Self {
        Self { anchor: None }
    }

    pub fn select(&mut self, cell: Cell) {
        self.anchor = Some(cell);
    }

    pub fn clear(&mut self) {
        self.anchor = None;
    }

    pub fn is_selected(&self, cell: Cell) -> bool {
        self.anchor == Some(cell)
    }

    pub fn is_peer(&self, cell: Cell) -> bool {
        let Some(anchor) = self.anchor else {
            return false;
        };
        if anchor == cell {
            return false;
        }
        PEERS[anchor.index()].contains(&(cell.index() as u8))
    }

    pub fn same_value(&self, cell: Cell, grid: &Grid) -> bool {
        let Some(anchor) = self.anchor else {
            return false;
        };
        let anchor_value = grid.value(anchor);
        if anchor_value == 0 {
            return false;
        }
        if anchor == cell {
            return false;
        }
        grid.value(cell) == anchor_value
    }

    pub fn conflicting_value(&self, cell: Cell, grid: &Grid) -> bool {
        let value = grid.value(cell);
        if value == 0 {
            return false;
        }
        for peer_index in PEERS[cell.index()] {
            let peer = Cell::from_index(peer_index as usize).unwrap();
            if grid.value(peer) == value {
                return true;
            }
        }
        false
    }
}