use alloc::format;

use sudoku_core::cell::Cell;
use yew::prelude::*;

use crate::state::GameStateContext;

use super::SudokuCell;

const CELL_SIZE: u32 = 56;

#[function_component(SudokuGrid)]
pub fn sudoku_grid() -> Html {
    let state = use_context::<GameStateContext>().expect("GameStateContext missing");
    let grid = state.grid;
    let selection = state.selection;
    let hint = state.hint.clone();

    let cells = (0..81u8).map(|index| {
        let cell = Cell::from_index(index as usize).unwrap();
        let value = grid.value(cell);
        let is_given = state.givens[cell.index()];
        let selected = selection.is_selected(cell);
        let peer = selection.is_peer(cell);
        let same_value = selection.same_value(cell, &grid);
        let conflict = if value != 0 {
            cell.peers()
                .iter()
                .any(|peer_cell| grid.value(*peer_cell) == value)
        } else {
            false
        };

        let hint_placement = hint.as_ref().and_then(|hint| hint.placement_digit(cell));
        let hint_elimination = hint
            .as_ref()
            .map(|hint| hint.elimination_digits(cell))
            .unwrap_or_default();

        html! {
            <SudokuCell
                key={index}
                cell={cell}
                value={value}
                candidates={state.effective_candidates(cell)}
                is_given={is_given}
                selected={selected}
                peer={peer}
                same_value={same_value}
                conflict={conflict}
                hint_placement={hint_placement}
                hint_elimination={hint_elimination}
            />
        }
    });

    let classes = format!(
        "grid grid-cols-9 border-2 border-zinc-400 dark:border-zinc-600 rounded-md overflow-hidden select-none mx-auto"
    );

    let style = format!(
        "grid-template-columns: repeat(9, {size}px); grid-template-rows: repeat(9, {size}px);",
        size = CELL_SIZE
    );

    html! {
        <div data-component="sudoku-grid" class={classes} {style}>
            { for cells }
        </div>
    }
}