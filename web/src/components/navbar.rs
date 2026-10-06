use alloc::format;
use sudoku_core::GameDifficulty;
use yew::prelude::*;
use yew_icons::{Icon, IconData};
use yew_router::prelude::*;

use crate::app::AppContext;
use crate::components::DifficultyPicker;
use crate::routes::Route;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let context = use_context::<AppContext>().expect("AppContext not provided");
    let current_route = use_route::<Route>();
    let difficulty = current_route
        .as_ref()
        .and_then(difficulty_from_route)
        .unwrap_or_default();

    html! {
        <nav data-component="navbar" class="h-16 border-b border-zinc-200 dark:border-zinc-800 bg-white/80 dark:bg-zinc-950/80 backdrop-blur sticky top-0 z-50">
            <div class="max-w-6xl mx-auto h-full px-6 flex items-center justify-between">
                <Link<Route> to={Route::Home} classes="flex items-center gap-2 text-sm font-medium text-zinc-900 dark:text-zinc-100 hover:text-zinc-600 dark:hover:text-zinc-300 transition-colors">
                    <Icon
                        data={IconData::LUCIDE_GRID}
                        class="text-zinc-500 dark:text-zinc-400"
                        width="18px"
                        height="18px"
                    />
                    { "Sudoku" }
                </Link<Route>>

                <div class="flex items-center gap-4">
                    <DifficultyPicker current={difficulty} />
                    <span class="hidden sm:inline text-xs font-mono text-zinc-400 dark:text-zinc-600">
                        { format!("v{}", context.version) }
                    </span>
                </div>
            </div>
        </nav>
    }
}

fn difficulty_from_route(route: &Route) -> Option<GameDifficulty> {
    match route {
        Route::Game { difficulty } => GameDifficulty::from_slug(difficulty),
        _ => None,
    }
}