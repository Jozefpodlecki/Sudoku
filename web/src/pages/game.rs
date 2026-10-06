use sudoku_core::difficulty::GameDifficulty;
use yew::prelude::*;

use crate::components::*;
use crate::state::GameStateProvider;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct GameProps {
    pub difficulty: GameDifficulty,
}

#[function_component(Game)]
pub fn game(props: &GameProps) -> Html {
    html! {
        <Layout>
            <GameStateProvider difficulty={props.difficulty}>
                <KeyboardHandler />
                <section class="max-w-4xl w-auto mx-auto px-6 py-8">
                    <StatusBar />
                    <SudokuGrid />
                    <Controls />
                </section>
                <LoadingOverlay />
                <CompletionOverlay />
            </GameStateProvider>
        </Layout>
    }
}