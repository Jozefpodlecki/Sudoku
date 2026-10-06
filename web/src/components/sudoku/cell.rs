use alloc::format;
use alloc::string::ToString;
use alloc::vec::Vec;

use sudoku_core::bits::{Digit, Mask};
use sudoku_core::cell::Cell;
use web_sys::MouseEvent;
use yew::prelude::*;

use crate::state::{Action, GameStateContext};
use crate::theme::{cell_background, cell_border, cell_text, CellText, CellVisual};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SudokuCellProps {
    pub cell: Cell,
    pub value: u8,
    pub candidates: Mask,
    pub is_given: bool,
    pub selected: bool,
    pub peer: bool,
    pub same_value: bool,
    pub conflict: bool,
    pub hint_placement: Option<Digit>,
    pub hint_elimination: Vec<Digit>,
}

#[function_component(SudokuCell)]
pub fn sudoku_cell(props: &SudokuCellProps) -> Html {
    let state = use_context::<GameStateContext>().expect("GameStateContext missing");
    let cell = props.cell;

    let onclick = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(Action::Select(cell));
        })
    };

    let visual = if props.selected {
        CellVisual::Selected
    } else if props.conflict {
        CellVisual::Conflict
    } else if props.hint_placement.is_some() {
        CellVisual::HintPlacement
    } else if props.same_value {
        CellVisual::SameValue
    } else if props.peer {
        CellVisual::Peer
    } else {
        CellVisual::Default
    };

    let text_kind = if props.value == 0 {
        CellText::Empty
    } else if props.is_given {
        CellText::Given
    } else {
        CellText::User
    };

    let border = cell_border(cell.row(), cell.col());
    let background = cell_background(visual);
    let text = cell_text(text_kind);

    let classes = format!(
        "relative aspect-square flex items-center justify-center text-2xl sm:text-3xl \
         tabular-nums transition-colors duration-100 cursor-pointer select-none \
         {border} {background} {text}"
    );

    let body = if props.value != 0 {
        html! { props.value.to_string() }
    } else {
        html! { <CandidateGrid candidates={props.candidates} /> }
    };

    let hint_overlay = if let Some(digit) = props.hint_placement {
        html! {
            <span class="absolute inset-0 flex items-center justify-center text-emerald-600 dark:text-emerald-400 pointer-events-none">
                { digit.get().to_string() }
            </span>
        }
    } else {
        Html::default()
    };

    html! {
        <div
            data-component="cell"
            {onclick}
            data-index={cell.index().to_string()}
            class={classes}
        >
            { body }
            { hint_overlay }
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct CandidateGridProps {
    pub candidates: Mask,
}

#[function_component(CandidateGrid)]
fn candidate_grid(props: &CandidateGridProps) -> Html {
    let digits = (1..=9u8).map(|value| {
        let digit = Digit::new(value).unwrap();
        let visible = props.candidates.contains(digit);
        let classes = if visible {
            "text-[0.625rem] text-zinc-500 dark:text-zinc-400"
        } else {
            "text-[0.625rem] text-transparent"
        };
        html! { <span key={value} class={classes}>{ value.to_string() }</span> }
    });

    html! {
        <div data-component="candidate-grid" class="grid grid-cols-3 grid-rows-3 w-full h-full p-[2px] pointer-events-none">
            { for digits }
        </div>
    }
}