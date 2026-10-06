use yew::prelude::*;

use crate::components::Loader;
use crate::state::GameStateContext;

#[function_component(LoadingOverlay)]
pub fn loading_overlay() -> Html {
    let state = use_context::<GameStateContext>().expect("GameStateContext missing");

    if !state.generating {
        return Html::default();
    }

    html! {
        <div data-component="loading-overlay" class="fixed inset-0 bg-black/50 dark:bg-black/70 flex items-center justify-center z-50 p-6">
            <div class="bg-white dark:bg-zinc-900 rounded-lg shadow-xl border border-zinc-200 dark:border-zinc-800 px-10 py-8 text-center">
                <Loader />
                <p class="mt-6 text-sm text-zinc-600 dark:text-zinc-400">
                    { "Generating puzzle…" }
                </p>
            </div>
        </div>
    }
}