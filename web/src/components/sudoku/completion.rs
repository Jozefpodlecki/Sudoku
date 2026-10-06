use alloc::format;

use web_sys::MouseEvent;
use yew::prelude::*;

use crate::state::{Action, GameStateContext};

#[function_component(CompletionOverlay)]
pub fn completion_overlay() -> Html {
    let state = use_context::<GameStateContext>().expect("GameStateContext missing");

    if !state.solved {
        return Html::default();
    }

    let on_new_game = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(Action::NewGameRequested(state.difficulty));
        })
    };

    let elapsed = state.elapsed;
    let minutes = elapsed.as_secs() / 60;
    let seconds = elapsed.as_secs() % 60;

    html! {
        <div data-component="completion-overlay" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-6">
            <div class="bg-white dark:bg-zinc-900 rounded-lg shadow-xl border border-zinc-200 dark:border-zinc-800 p-8 max-w-sm w-full text-center">
                <h2 class="text-xl font-semibold text-zinc-900 dark:text-zinc-100">
                    { "Solved" }
                </h2>

                <div class="mt-6 space-y-2 text-sm text-zinc-600 dark:text-zinc-400">
                    <div class="flex justify-between">
                        <span>{ "Time" }</span>
                        <span class="font-mono tabular-nums">{ format!("{minutes:02}:{seconds:02}") }</span>
                    </div>
                    <div class="flex justify-between">
                        <span>{ "Difficulty" }</span>
                        <span>{ state.difficulty.label() }</span>
                    </div>
                    <div class="flex justify-between">
                        <span>{ "Mistakes" }</span>
                        <span>{ state.mistakes }</span>
                    </div>
                </div>

                <button
                    type="button"
                    onclick={on_new_game}
                    class="mt-8 w-full py-2.5 rounded-md bg-zinc-900 dark:bg-zinc-100 text-white dark:text-zinc-900 text-sm font-medium hover:bg-zinc-700 dark:hover:bg-zinc-300 transition-colors"
                >
                    { "New Game" }
                </button>
            </div>
        </div>
    }
}