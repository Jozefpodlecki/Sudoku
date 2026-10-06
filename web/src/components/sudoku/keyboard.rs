use alloc::string::ToString;

use sudoku_core::bits::Digit;
use sudoku_core::cell::Cell;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{AddEventListenerOptions, KeyboardEvent, window};
use yew::prelude::*;

use crate::state::{Action, GameStateContext};

#[function_component(KeyboardHandler)]
pub fn keyboard_handler() -> Html {
    let state = use_context::<GameStateContext>().expect("GameStateContext missing");

    {
        let state = state.clone();
        use_effect_with((), move |_| {
            let window = window().expect("window");

            let closure = Closure::<dyn FnMut(KeyboardEvent)>::new(move |event: KeyboardEvent| {
                handle_key(&state, &event);
            });

            let options = AddEventListenerOptions::new();
            options.set_passive(false);

            window
                .add_event_listener_with_callback_and_add_event_listener_options(
                    "keydown",
                    closure.as_ref().unchecked_ref(),
                    &options,
                )
                .expect("add keydown listener");

            move || {
                window
                    .remove_event_listener_with_callback(
                        "keydown",
                        closure.as_ref().unchecked_ref(),
                    )
                    .ok();
                drop(closure);
            }
        });
    }

    Html::default()
}

fn handle_key(state: &GameStateContext, event: &KeyboardEvent) {
    if event.ctrl_key() || event.meta_key() {
        match event.key().as_str() {
            "z" | "Z" => {
                event.prevent_default();
                if event.shift_key() {
                    state.dispatch(Action::Redo);
                } else {
                    state.dispatch(Action::Undo);
                }
            }
            "y" | "Y" => {
                event.prevent_default();
                state.dispatch(Action::Redo);
            }
            _ => {}
        }
        return;
    }

    match event.key().as_str() {
        "ArrowUp" => {
            event.prevent_default();
            move_selection(state, 0, -1);
        }
        "ArrowDown" => {
            event.prevent_default();
            move_selection(state, 0, 1);
        }
        "ArrowLeft" => {
            event.prevent_default();
            move_selection(state, -1, 0);
        }
        "ArrowRight" => {
            event.prevent_default();
            move_selection(state, 1, 0);
        }
        "Backspace" | "Delete" => {
            event.prevent_default();
            state.dispatch(Action::Clear);
        }
        " " => {
            event.prevent_default();
            state.dispatch(Action::TogglePause);
        }
        "c" | "C" => {
            state.dispatch(Action::ToggleCandidateMode);
        }
        "h" | "H" => {
            state.dispatch(Action::RequestHint);
        }
        "s" | "S" => {
            state.dispatch(Action::SolveStep);
        }
        "u" | "U" => {
            state.dispatch(Action::Undo);
        }
        "r" | "R" => {
            state.dispatch(Action::Redo);
        }
        "Escape" => {
            state.dispatch(Action::DismissHint);
        }
        key => {
            if let Some(digit) = digit_from_key(key) {
                state.dispatch(Action::Input(digit));
            }
        }
    }
}

fn digit_from_key(key: &str) -> Option<Digit> {
    let value: u8 = key.parse().ok()?;
    Digit::new(value)
}

fn move_selection(state: &GameStateContext, dx: i8, dy: i8) {
    let Some(cell) = state.selection.anchor else {
        state.dispatch(Action::Select(Cell::from_index(40).unwrap()));
        return;
    };

    let row = cell.row() as i8 + dy;
    let col = cell.col() as i8 + dx;

    if !(0..9).contains(&row) || !(0..9).contains(&col) {
        return;
    }

    let index = (row as usize) * 9 + (col as usize);
    if let Some(next) = Cell::from_index(index) {
        state.dispatch(Action::Select(next));
    }
}