use alloc::format;
use yew::prelude::*;

use crate::state::{Action, GameStateContext};

#[function_component(StatusBar)]
pub fn status_bar() -> Html {
    let state = use_context::<GameStateContext>().expect("GameStateContext missing");
    let elapsed = state.elapsed;
    let minutes = elapsed.as_secs() / 60;
    let seconds = elapsed.as_secs() % 60;

    let on_pause = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(Action::TogglePause);
        })
    };

    html! {
        <div data-component="status-bar" class="flex items-center justify-between mb-4 text-sm text-zinc-600 dark:text-zinc-400">
            <div class="flex items-center gap-4">
                <span data-component="timer" class="font-mono tabular-nums">
                    { format!("{minutes:02}:{seconds:02}") }
                </span>
                <span class="text-zinc-400 dark:text-zinc-600">
                    { state.difficulty.label() }
                </span>
            </div>

            <div class="flex items-center gap-4">
                <button
                    type="button"
                    onclick={on_pause}
                    class="px-2 py-1 rounded text-xs border border-zinc-200 dark:border-zinc-800 hover:bg-zinc-100 dark:hover:bg-zinc-800 transition-colors"
                >
                    { if state.paused { "Resume" } else { "Pause" } }
                </button>
            </div>
        </div>
    }
}