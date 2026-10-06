use alloc::format;

use sudoku_core::difficulty::GameDifficulty;
use web_sys::MouseEvent;
use yew::prelude::*;

use crate::state::SavedGameSummary;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SavedGameBannerProps {
    pub summary: SavedGameSummary,
    pub on_hover_start: Callback<MouseEvent>,
    pub on_hover_end: Callback<MouseEvent>,
    pub on_click: Callback<MouseEvent>,
}

#[function_component(SavedGameBanner)]
pub fn saved_game_banner(props: &SavedGameBannerProps) -> Html {
    let seconds = props.summary.elapsed_seconds;
    let minutes = seconds / 60;
    let remaining = seconds % 60;

    let accent = DifficultyAccent::for_difficulty(props.summary.difficulty);

    let container_classes = format!(
        "mt-8 inline-flex items-center gap-2 rounded-md border px-4 py-2 text-xs \
         cursor-pointer transition-all duration-200 \
         {} {} {} {} {}",
        accent.border,
        accent.background,
        accent.text,
        accent.hover_border,
        accent.hover_ring,
    );

    html! {
        <div
            onmouseenter={props.on_hover_start.clone()}
            onmouseleave={props.on_hover_end.clone()}
            onclick={props.on_click.clone()}
            role="button"
            tabindex="0"
            class={container_classes}
        >
            <span class={format!("w-1.5 h-1.5 rounded-full {}", accent.dot)} />
            { format!(
                "Game in progress - {} · {:02}:{:02}",
                props.summary.difficulty.label(),
                minutes,
                remaining
            ) }
        </div>
    }
}

struct DifficultyAccent {
    dot: &'static str,
    border: &'static str,
    background: &'static str,
    text: &'static str,
    hover_border: &'static str,
    hover_ring: &'static str,
}

impl DifficultyAccent {
    fn for_difficulty(difficulty: GameDifficulty) -> Self {
        match difficulty {
            GameDifficulty::Easy => Self {
                dot: "bg-emerald-500",
                border: "border-emerald-200 dark:border-emerald-900",
                background: "bg-emerald-50 dark:bg-emerald-950/40",
                text: "text-emerald-700 dark:text-emerald-300",
                hover_border: "hover:border-emerald-400 dark:hover:border-emerald-600",
                hover_ring: "hover:ring-2 hover:ring-emerald-400/30 dark:hover:ring-emerald-500/30",
            },
            GameDifficulty::Medium => Self {
                dot: "bg-blue-500",
                border: "border-blue-200 dark:border-blue-900",
                background: "bg-blue-50 dark:bg-blue-950/40",
                text: "text-blue-700 dark:text-blue-300",
                hover_border: "hover:border-blue-400 dark:hover:border-blue-600",
                hover_ring: "hover:ring-2 hover:ring-blue-400/30 dark:hover:ring-blue-500/30",
            },
            GameDifficulty::Hard => Self {
                dot: "bg-amber-500",
                border: "border-amber-200 dark:border-amber-900",
                background: "bg-amber-50 dark:bg-amber-950/40",
                text: "text-amber-700 dark:text-amber-300",
                hover_border: "hover:border-amber-400 dark:hover:border-amber-600",
                hover_ring: "hover:ring-2 hover:ring-amber-400/30 dark:hover:ring-amber-500/30",
            },
            GameDifficulty::Expert => Self {
                dot: "bg-rose-500",
                border: "border-rose-200 dark:border-rose-900",
                background: "bg-rose-50 dark:bg-rose-950/40",
                text: "text-rose-700 dark:text-rose-300",
                hover_border: "hover:border-rose-400 dark:hover:border-rose-600",
                hover_ring: "hover:ring-2 hover:ring-rose-400/30 dark:hover:ring-rose-500/30",
            },
            GameDifficulty::Extreme => Self {
                dot: "bg-purple-500",
                border: "border-purple-200 dark:border-purple-900",
                background: "bg-purple-50 dark:bg-purple-950/40",
                text: "text-purple-700 dark:text-purple-300",
                hover_border: "hover:border-purple-400 dark:hover:border-purple-600",
                hover_ring: "hover:ring-2 hover:ring-purple-400/30 dark:hover:ring-purple-500/30",
            },
        }
    }
}