use alloc::string::String;
use sudoku_core::GameDifficulty;
use yew::prelude::*;
use yew_router::Routable;

use crate::pages::*;

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/game/:difficulty")]
    Game { difficulty: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

impl Route {
    pub fn default_game() -> Self {
        Self::Game {
            difficulty: GameDifficulty::default().slug().into(),
        }
    }
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Game { difficulty } => {
            let parsed = GameDifficulty::from_slug(&difficulty).unwrap_or_default();
            html! { <Game difficulty={parsed} /> }
        }
        Route::NotFound => html! { <NotFound /> },
    }
}