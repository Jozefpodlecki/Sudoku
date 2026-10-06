use alloc::vec::Vec;

use super::cover::ExactCover;
use super::placement::Placement;

pub(super) fn solve(exact: &mut ExactCover) -> Option<Vec<Placement>> {
    let mut solution = Vec::with_capacity(81);
    let mut placements = Vec::with_capacity(81);

    if search_first(exact, &mut solution, &mut placements) {
        Some(placements)
    } else {
        None
    }
}

pub(super) fn count(exact: &mut ExactCover, limit: usize) -> usize {
    if limit == 0 {
        return 0;
    }

    let mut solution = Vec::with_capacity(81);
    let mut placements = Vec::with_capacity(81);
    let mut count = 0;

    enumerate(
        exact,
        &mut solution,
        &mut placements,
        &mut count,
        limit,
    );

    count
}

fn search_first(
    exact: &mut ExactCover,
    solution: &mut Vec<u32>,
    placements: &mut Vec<Placement>,
) -> bool {
    let Some(column) = exact.choose_column() else {
        return true;
    };

    exact.cover(column);

    let mut row = exact.nodes[column as usize].down;

    while row != column {
        // IMPORTANT:
        // Save the next candidate before covering this row's
        // other columns. Those cover operations modify the
        // vertical links in the matrix.
        let next_row = exact.nodes[row as usize].down;

        solution.push(row);

        let placement = Placement::from_row(exact.nodes[row as usize].row);
        placements.push(placement);

        // Cover all other constraints satisfied by this row.
        let mut node = exact.nodes[row as usize].right;

        while node != row {
            let next_node = exact.nodes[node as usize].right;
            let node_column = exact.nodes[node as usize].column;

            exact.cover(node_column);

            node = next_node;
        }

        if search_first(exact, solution, placements) {
            return true;
        }

        // Undo the row selection.
        let mut node = exact.nodes[row as usize].left;

        while node != row {
            let previous_node = exact.nodes[node as usize].left;
            let node_column = exact.nodes[node as usize].column;

            exact.uncover(node_column);

            node = previous_node;
        }

        solution.pop();
        placements.pop();

        // Continue with the candidate that existed before
        // the row's columns were covered.
        row = next_row;
    }

    exact.uncover(column);

    false
}

fn enumerate(
    exact: &mut ExactCover,
    solution: &mut Vec<u32>,
    placements: &mut Vec<Placement>,
    count: &mut usize,
    limit: usize,
) {
    if *count >= limit {
        return;
    }

    let Some(column) = exact.choose_column() else {
        *count += 1;
        return;
    };

    exact.cover(column);

    let mut row = exact.nodes[column as usize].down;

    while row != column {
        // Save this before modifying the matrix.
        let next_row = exact.nodes[row as usize].down;

        solution.push(row);

        let placement = Placement::from_row(exact.nodes[row as usize].row);
        placements.push(placement);

        // Cover all other columns represented by this row.
        let mut node = exact.nodes[row as usize].right;

        while node != row {
            let next_node = exact.nodes[node as usize].right;
            let node_column = exact.nodes[node as usize].column;

            exact.cover(node_column);

            node = next_node;
        }

        enumerate(
            exact,
            solution,
            placements,
            count,
            limit,
        );

        // Uncover in reverse order.
        let mut node = exact.nodes[row as usize].left;

        while node != row {
            let previous_node = exact.nodes[node as usize].left;
            let node_column = exact.nodes[node as usize].column;

            exact.uncover(node_column);

            node = previous_node;
        }

        solution.pop();
        placements.pop();

        // Continue using the saved candidate.
        row = next_row;

        if *count >= limit {
            break;
        }
    }

    exact.uncover(column);
}