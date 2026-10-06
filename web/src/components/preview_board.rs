use alloc::format;
use alloc::string::ToString;

use yew::prelude::*;

const PLACEHOLDER_VALUES: [u8; 81] = [
    5,3,0, 0,7,0, 0,0,0,
    6,0,0, 1,9,5, 0,0,0,
    0,9,8, 0,0,0, 0,6,0,
    8,0,0, 0,6,0, 0,0,3,
    4,0,0, 8,0,3, 0,0,1,
    7,0,0, 0,2,0, 0,0,6,
    0,6,0, 0,0,0, 2,8,0,
    0,0,0, 4,1,9, 0,0,5,
    0,0,0, 0,8,0, 0,7,9,
];

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PreviewBoardProps {
    #[prop_or(PLACEHOLDER_VALUES)]
    pub values: [u8; 81],
    #[prop_or_default]
    pub highlighted: bool,
}

#[function_component(PreviewBoard)]
pub fn preview_board(props: &PreviewBoardProps) -> Html {
    let values = props.values;

    let container_classes = if props.highlighted {
        "grid grid-cols-9 border-2 border-blue-400 dark:border-blue-500 rounded-lg overflow-hidden select-none \
         opacity-100 ring-2 ring-blue-400/30 dark:ring-blue-500/30 \
         transition-all duration-200"
    } else {
        "grid grid-cols-9 border-2 border-zinc-400 dark:border-zinc-700 rounded-lg overflow-hidden select-none \
         opacity-70 dark:opacity-50 transition-all duration-200"
    };

    let digit_classes = if props.highlighted {
        "text-zinc-600 dark:text-zinc-300"
    } else {
        "text-zinc-400 dark:text-zinc-600"
    };

    let cells = (0..81u8).map(|index| {
        let value = values[index as usize];
        let col = index % 9;
        let row = index / 9;

        let thick_right = col == 2 || col == 5;
        let thick_bottom = row == 2 || row == 5;

        let border = match (thick_right, thick_bottom) {
            (true, true) => "border-r-2 border-b-2 border-zinc-400 dark:border-zinc-700",
            (true, false) => "border-r-2 border-b border-zinc-200 dark:border-zinc-800",
            (false, true) => "border-r border-b-2 border-zinc-200 dark:border-zinc-800",
            (false, false) => "border-r border-b border-zinc-200 dark:border-zinc-800",
        };

        let content = if value == 0 {
            Html::default()
        } else {
            html! { value.to_string() }
        };

        html! {
            <div class={format!("aspect-square flex items-center justify-center text-lg font-semibold {digit_classes} {border}")}>
                { content }
            </div>
        }
    });

    html! {
        <div class={container_classes}>
            { for cells }
        </div>
    }
}