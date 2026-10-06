use log::Level::Info;
use sudoku_core::{difficulty::GameDifficulty, generator::Generation};
use sudoku_core::generator::Generator;
use yew::prelude::*;

use crate::state::game::GenerationProgress;
use crate::utils::now;

use super::game::{Action, GameState};

pub type GameStateContext = UseReducerHandle<GameState>;

#[derive(Clone, PartialEq, Properties)]
pub struct GameStateProviderProps {
    pub difficulty: GameDifficulty,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(GameStateProvider)]
pub fn game_state_provider(props: &GameStateProviderProps) -> Html {
    let state = use_reducer(|| {
        super::persistence::load_state()
            .unwrap_or_else(|| GameState::new(props.difficulty, now() as u64))
    });

    {
        let state = state.clone();
        use_effect_with((*state).clone(), move |state| {
            super::persistence::save_state(state);
            || ()
        });
    }

    {
        let state = state.clone();
        use_effect_with(state.generating, move |generating| {
            if *generating {
                let state = state.clone();
                let difficulty = state.pending_difficulty.unwrap_or(state.difficulty);
                let mut generation = Generator::generation(difficulty, now() as u64);
                drive_generation(state, generation);
            }
            || ()
        });
    }

    {
        let state = state.clone();
        let difficulty = props.difficulty;
        use_effect_with(difficulty, move |difficulty| {
            if state.difficulty != *difficulty && !state.generating {
                state.dispatch(Action::NewGameRequested(*difficulty));
            }
            || ()
        });
    }

    {
        let state = state.clone();
        use_effect_with((), move |_| {
            let handle = gloo_timers::callback::Interval::new(1000, move || {
                state.dispatch(Action::Tick);
            });
            move || drop(handle)
        });
    }

    html! {
        <ContextProvider<GameStateContext> context={state}>
            { for props.children.iter() }
        </ContextProvider<GameStateContext>>
    }
}

fn drive_generation(state: GameStateContext, mut generation: Generation) {
    let state_inner = state.clone();
    let handle = gloo_timers::callback::Timeout::new(0, move || {
        let Some(attempt) = generation.next() else {
            state.dispatch(Action::NewGameReady(generation.finish()));
            return;
        };

        let progress = GenerationProgress {
            attempts: attempt.index,
            max_attempts: generation.max_attempts(),
            best_bucket: Some(attempt.rating.bucket()),
            best_distance: attempt.distance_to(state_inner.difficulty),
        };
        state_inner.dispatch(Action::GenerationProgressed(progress));

        if attempt.matches_target {
            state.dispatch(Action::NewGameReady(attempt.grid));
            return;
        }

        drive_generation(state, generation);
    });
    handle.forget();
}