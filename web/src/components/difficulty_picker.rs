use alloc::format;
use sudoku_core::GameDifficulty;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct DifficultyPickerProps {
    pub current: GameDifficulty,
}

#[function_component(DifficultyPicker)]
pub fn difficulty_picker(props: &DifficultyPickerProps) -> Html {
    html! {
        <div
            role="radiogroup"
            aria-label="Difficulty"
            class="inline-flex rounded-md border border-zinc-200 dark:border-zinc-800 bg-zinc-50 dark:bg-zinc-900 p-0.5"
        >
            { for GameDifficulty::ALL.iter().copied().map(|d| html! {
                <DifficultyOption
                    key={d.slug()}
                    difficulty={d}
                    active={d == props.current}
                />
            }) }
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct DifficultyOptionProps {
    difficulty: GameDifficulty,
    active: bool,
}

#[function_component(DifficultyOption)]
fn difficulty_option(props: &DifficultyOptionProps) -> Html {
    let base = "px-3 py-1.5 text-xs font-medium rounded transition-colors";

    let state = if props.active {
        "bg-white dark:bg-zinc-800 text-zinc-900 dark:text-zinc-100 shadow-sm"
    } else {
        "text-zinc-500 dark:text-zinc-500 hover:text-zinc-900 dark:hover:text-zinc-100"
    };

    let classes = format!("{base} {state}");

    html! {
        <Link<Route>
            to={Route::Game { difficulty: props.difficulty.slug().into() }}
            classes={Classes::from(classes)}
        >
            { props.difficulty.label() }
        </Link<Route>>
    }
}