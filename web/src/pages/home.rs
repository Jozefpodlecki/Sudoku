// web/src/pages/home.rs
use yew::prelude::*;
use yew_icons::{Icon, IconData};
use yew_router::prelude::*;

use crate::components::{Layout, PreviewBoard, SavedGameBanner};
use crate::routes::Route;
use crate::state::{clear_state, saved_game_summary, saved_game_values};

#[function_component(Home)]
pub fn home() -> Html {
    let saved = use_state(saved_game_summary);
    let preview_values = use_state(|| None::<[u8; 81]>);
    let navigator = use_navigator().expect("navigator");

    let on_hover_start = {
        let preview_values = preview_values.clone();
        Callback::from(move |_: web_sys::MouseEvent| {
            preview_values.set(saved_game_values());
        })
    };

    let on_hover_end = {
        let preview_values = preview_values.clone();
        Callback::from(move |_: web_sys::MouseEvent| {
            preview_values.set(None);
        })
    };

    let on_new_game = {
        let navigator = navigator.clone();
        let saved = saved.clone();
        Callback::from(move |event: web_sys::MouseEvent| {
            event.prevent_default();
            clear_state();
            saved.set(None);
            navigator.push(&Route::default_game());
        })
    };

    let on_continue = {
        let navigator = navigator.clone();
        Callback::from(move |event: web_sys::MouseEvent| {
            event.prevent_default();
            navigator.push(&Route::default_game());
        })
    };

    let board = match *preview_values {
        Some(values) => html! {
            <PreviewBoard values={values} highlighted={true} />
        },
        None => html! { <PreviewBoard /> },
    };

    html! {
        <Layout>
            <section class="max-w-6xl mx-auto px-6 py-16 lg:py-24">
                <div class="grid lg:grid-cols-2 gap-16 items-center">
                    <div>
                    
                        <h1 class="mt-6 text-4xl sm:text-5xl lg:text-6xl font-semibold tracking-tight text-zinc-900 dark:text-zinc-100">
                            { "Sudoku" }
                        </h1>

                        <p class="mt-6 text-lg text-zinc-600 dark:text-zinc-400 max-w-xl">
                            { "Unique-solution puzzles, hints that name the technique, \
                               and a difficulty rating based on what it actually takes to solve." }
                        </p>

                        if let Some(summary) = &*saved {
                            <SavedGameBanner
                                summary={summary.clone()}
                                {on_hover_start}
                                {on_hover_end}
                                on_click={on_continue.clone()}
                            />
                        }

                        <div class="mt-10 flex flex-wrap items-center gap-3">
                            if saved.is_some() {
                                <a
                                    href={Route::default_game().to_path()}
                                    onclick={on_continue}
                                    class="inline-flex items-center gap-2 rounded-md bg-zinc-900 dark:bg-zinc-100 px-6 py-3 text-sm font-medium text-white dark:text-zinc-900 hover:bg-zinc-700 dark:hover:bg-zinc-300 transition-colors"
                                >
                                    <Icon data={IconData::LUCIDE_PLAY} width="16px" height="16px" />
                                    { "Continue" }
                                </a>
                                <a
                                    href={Route::default_game().to_path()}
                                    onclick={on_new_game}
                                    class="inline-flex items-center gap-2 rounded-md border border-zinc-200 dark:border-zinc-800 px-6 py-3 text-sm font-medium text-zinc-700 dark:text-zinc-300 hover:bg-zinc-100 dark:hover:bg-zinc-800 transition-colors"
                                >
                                    { "New Game" }
                                </a>
                            } else {
                                <Link<Route>
                                    to={Route::default_game()}
                                    classes="inline-flex items-center gap-2 rounded-md bg-zinc-900 dark:bg-zinc-100 px-6 py-3 text-sm font-medium text-white dark:text-zinc-900 hover:bg-zinc-700 dark:hover:bg-zinc-300 transition-colors"
                                >
                                    <Icon data={IconData::LUCIDE_PLAY} width="16px" height="16px" />
                                    { "Play now" }
                                </Link<Route>>
                            }
                            <a
                                href="https://github.com/Jozefpodlecki/Sudoku"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="inline-flex items-center gap-2 rounded-md border border-zinc-200 dark:border-zinc-800 px-6 py-3 text-sm font-medium text-zinc-700 dark:text-zinc-300 hover:bg-zinc-100 dark:hover:bg-zinc-800 transition-colors"
                            >
                                <Icon data={IconData::LUCIDE_GITHUB} width="16px" height="16px" />
                                { "Source" }
                            </a>
                        </div>
                    </div>

                    <div class="hidden lg:block">
                        { board }
                    </div>
                </div>
            </section>
        </Layout>
    }
}