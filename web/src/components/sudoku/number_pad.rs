use alloc::string::ToString;

use sudoku_core::Digit;
use web_sys::MouseEvent;
use yew::prelude::*;

use crate::state::{Action, GameStateContext};
use crate::utils::element_from_event;

#[function_component(NumberPad)]
pub fn number_pad() -> Html {
    let state = use_context::<GameStateContext>().expect("GameStateContext missing");

    let on_click = {
        let state = state.clone();
        Callback::from(move |event: MouseEvent| {
            let element = element_from_event(&event);
            let value: u8 = element
                .get_attribute("data-digit")
                .expect("number pad button missing data-digit")
                .parse()
                .expect("number pad data-digit is not a number");
            let digit = Digit::new(value).expect("number pad data-digit out of range");
            state.dispatch(Action::Input(digit));
        })
    };

    let allowed = state.allowed_digits();
    let has_selection = state.selection.anchor.is_some();
    let selected_is_given = state
        .selection
        .anchor
        .map(|cell| state.givens[cell.index()])
        .unwrap_or(false);
    let selected_has_value = state
        .selection
        .anchor
        .map(|cell| state.grid.value(cell) != 0)
        .unwrap_or(false);

    let can_clear = has_selection && !selected_is_given && selected_has_value;

    let buttons = (1..=9u8).map(|value| {
        let digit = sudoku_core::bits::Digit::new(value).unwrap();
        let enabled = allowed.contains(digit);
        html! {
            <button
                key={value}
                type="button"
                data-digit={value.to_string()}
                onclick={on_click.clone()}
                disabled={!enabled}
                class="aspect-square flex items-center justify-center text-md font-medium rounded border border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-900 text-zinc-900 dark:text-zinc-100 hover:bg-zinc-100 dark:hover:bg-zinc-800 transition-colors disabled:opacity-30 disabled:cursor-not-allowed disabled:hover:bg-white dark:disabled:hover:bg-zinc-900"
            >
                { value }
            </button>
        }
    });

    html! {
        <div data-component="number-pad" class="grid grid-cols-9 gap-2 mt-6">
            { for buttons }
        </div>
    }
}