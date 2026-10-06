use alloc::format;

use wasm_bindgen::JsCast;
use web_sys::MouseEvent;
use yew::prelude::*;

use crate::state::{Action, GameStateContext};

const BUTTON_BASE: &str = "px-3 py-1.5 text-sm rounded border transition-colors";
const BUTTON_DEFAULT: &str =
    "border-zinc-200 dark:border-zinc-800 hover:bg-zinc-100 dark:hover:bg-zinc-800";
const BUTTON_ACTIVE: &str = "border-blue-500 text-blue-600 dark:text-blue-400";
const BUTTON_SUCCESS: &str =
    "border-emerald-400 dark:border-emerald-700 text-emerald-700 dark:text-emerald-400 \
     hover:bg-emerald-50 dark:hover:bg-emerald-950";
const BUTTON_DISABLED: &str = "disabled:opacity-40 disabled:cursor-not-allowed";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControlVariant {
    Default,
    Active,
    Success,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ControlButtonProps {
    pub action: Action,
    #[prop_or_default]
    pub variant: Option<ControlVariant>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub children: Children,
    pub on_click: Callback<MouseEvent>,
}

#[function_component(ControlButton)]
pub fn control_button(props: &ControlButtonProps) -> Html {
    let variant_classes = match props.variant {
        Some(ControlVariant::Active) => BUTTON_ACTIVE,
        Some(ControlVariant::Success) => BUTTON_SUCCESS,
        _ => BUTTON_DEFAULT,
    };

    let classes = format!("{BUTTON_BASE} {variant_classes} {BUTTON_DISABLED}");
    let data_action = props.action.data_attribute();

    html! {
        <button
            type="button"
            data-action={data_action}
            onclick={props.on_click.clone()}
            disabled={props.disabled}
            class={classes}
        >
            { for props.children.iter() }
        </button>
    }
}

#[function_component(Controls)]
pub fn controls() -> Html {
    let state = use_context::<GameStateContext>().expect("GameStateContext missing");

    let on_click = {
        let state = state.clone();
        Callback::from(move |event: MouseEvent| {
            let target = event.target().unwrap();
            let element: web_sys::Element = target.unchecked_into();

            let Some(action_name) = element.get_attribute("data-action") else {
                return;
            };

            if action_name == "new-game" {
                state.dispatch(Action::NewGameRequested(state.difficulty));
                return;
            }

            if let Some(action) = Action::from_data_attribute(&action_name) {
                state.dispatch(action);
            }
        })
    };

    // let candidates_variant = if state.candidate_mode {
    //     Some(ControlVariant::Active)
    // } else {
    //     None
    // };

    html! {
        <div data-component="controls" class="mt-6">
            <div class="flex flex-wrap items-center gap-2">
                <button
                    type="button"
                    data-action="new-game"
                    onclick={on_click.clone()}
                    class={format!("{BUTTON_BASE} {BUTTON_DEFAULT} {BUTTON_DISABLED}")}
                >
                    { "New Game" }
                </button>

                <ControlButton action={Action::SolveStep} on_click={on_click.clone()}>
                    { "Solve Step" }
                </ControlButton>

                // <ControlButton
                //     action={Action::ToggleCandidateMode}
                //     variant={candidates_variant}
                //     on_click={on_click.clone()}
                // >
                //     { "Candidates" }
                // </ControlButton>

                <div class="flex-1" />

                <ControlButton action={Action::RequestHint} on_click={on_click.clone()}>
                    { "Hint" }
                </ControlButton>
            </div>

            if let Some(hint) = &state.hint {
                <div class="mt-2 flex items-center gap-2 text-xs text-zinc-500 dark:text-zinc-500">
                    <span>{ hint.technique.label() }</span>

                    if !hint.placements.is_empty() {
                        <ControlButton
                            action={Action::ApplyHint}
                            variant={Some(ControlVariant::Success)}
                            on_click={on_click.clone()}
                        >
                            { "Apply" }
                        </ControlButton>
                    }

                    <ControlButton action={Action::DismissHint} on_click={on_click.clone()}>
                        { "Dismiss" }
                    </ControlButton>
                </div>
            }
        </div>
    }
}