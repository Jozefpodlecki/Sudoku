use crate::Cell;

pub const CELL_COUNT: usize = 81;
pub const HOUSE_COUNT: usize = 27;
pub const HOUSE_SIZE: usize = 9;

pub const fn row_of(cell: usize) -> usize {
    cell / 9
}

pub const fn col_of(cell: usize) -> usize {
    cell % 9
}

pub const fn box_of(cell: usize) -> usize {
    (row_of(cell) / 3) * 3 + col_of(cell) / 3
}

pub const fn cell_at(row: usize, col: usize) -> usize {
    row * 9 + col
}

pub const fn is_peer(a: usize, b: usize) -> bool {
    if a == b {
        return false;
    }
    row_of(a) == row_of(b) || col_of(a) == col_of(b) || box_of(a) == box_of(b)
}

pub const PEERS: [[u8; 20]; CELL_COUNT] = {
    let mut table = [[0u8; 20]; CELL_COUNT];
    let mut cell_index = 0;
    while cell_index < CELL_COUNT {
        let mut count = 0;
        let mut other_index = 0;
        while other_index < CELL_COUNT {
            let cell = Cell(cell_index as u8);
            let other = Cell(other_index as u8);
            if cell.is_peer(other) {
                table[cell_index][count] = other_index as u8;
                count += 1;
            }
            other_index += 1;
        }
        cell_index += 1;
    }
    table
};

pub const ROW_CELLS: [[u8; 9]; 9] = {
    let mut table = [[0u8; 9]; 9];
    let mut row = 0;
    while row < 9 {
        let mut col = 0;
        while col < 9 {
            table[row][col] = cell_at(row, col) as u8;
            col += 1;
        }
        row += 1;
    }
    table
};

pub const COL_CELLS: [[u8; 9]; 9] = {
    let mut table = [[0u8; 9]; 9];
    let mut col = 0;
    while col < 9 {
        let mut row = 0;
        while row < 9 {
            table[col][row] = cell_at(row, col) as u8;
            row += 1;
        }
        col += 1;
    }
    table
};

pub const BOX_CELLS: [[u8; 9]; 9] = {
    let mut table = [[0u8; 9]; 9];
    let mut box_index = 0;
    while box_index < 9 {
        let base_row = (box_index / 3) * 3;
        let base_col = (box_index % 3) * 3;
        let mut offset = 0;
        while offset < 9 {
            let row = base_row + offset / 3;
            let col = base_col + offset % 3;
            table[box_index][offset] = cell_at(row, col) as u8;
            offset += 1;
        }
        box_index += 1;
    }
    table
};

pub const HOUSE_CELLS: [[u8; 9]; HOUSE_COUNT] = {
    let mut table = [[0u8; 9]; HOUSE_COUNT];
    let mut house = 0;
    while house < 9 {
        table[house] = ROW_CELLS[house];
        table[house + 9] = COL_CELLS[house];
        table[house + 18] = BOX_CELLS[house];
        house += 1;
    }
    table
};