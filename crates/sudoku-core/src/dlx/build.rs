use alloc::vec::Vec;

use super::cover::{
    ExactCover, Node, CELLS, COLS, COL_OFFSET_BOX, COL_OFFSET_CELL, COL_OFFSET_COL,
    COL_OFFSET_ROW, DIGITS, ROWS,
};

pub(super) fn from_scratch() -> ExactCover {
    let column_count = COLS + 1;
    let mut nodes = Vec::with_capacity(column_count + ROWS * 4);

    for index in 0..column_count {
        nodes.push(Node {
            left: ((index + column_count - 1) % column_count) as u32,
            right: ((index + 1) % column_count) as u32,
            up: index as u32,
            down: index as u32,
            column: index as u32,
            row: u32::MAX,
        });
    }

    let mut exact = ExactCover {
        nodes,
        givens: [0; CELLS],
    };
    append_all_rows(&mut exact);
    exact
}

fn append_all_rows(exact: &mut ExactCover) {
    for cell in 0..CELLS {
        for digit_index in 0..DIGITS {
            append_row(exact, cell, digit_index);
        }
    }
}

fn append_row(exact: &mut ExactCover, cell: usize, digit_index: usize) {
    let row = (cell * DIGITS + digit_index) as u32;
    let row_index = cell / 9;
    let col_index = cell % 9;
    let box_index = (row_index / 3) * 3 + col_index / 3;

    let columns = [
        (COL_OFFSET_CELL + cell) as u32,
        (COL_OFFSET_ROW + row_index * 9 + digit_index) as u32,
        (COL_OFFSET_COL + col_index * 9 + digit_index) as u32,
        (COL_OFFSET_BOX + box_index * 9 + digit_index) as u32,
    ];

    let first = append_node(exact, columns[0], row);
    let mut prev = first;

    for &column in &columns[1..] {
        let node = append_node(exact, column, row);
        exact.nodes[prev as usize].right = node;
        exact.nodes[node as usize].left = prev;
        prev = node;
    }

    exact.nodes[prev as usize].right = first;
    exact.nodes[first as usize].left = prev;
}

fn append_node(exact: &mut ExactCover, column: u32, row: u32) -> u32 {
    let node_index = exact.nodes.len() as u32;
    let up = exact.nodes[column as usize].up;

    exact.nodes.push(Node {
        left: node_index,
        right: node_index,
        up,
        down: column,
        column,
        row,
    });

    exact.nodes[up as usize].down = node_index;
    exact.nodes[column as usize].up = node_index;

    node_index
}