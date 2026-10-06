use alloc::format;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LoaderProps {
    #[prop_or(10)]
    pub size: u32,
    #[prop_or(1.5)]
    pub scale: f32,
}

#[function_component(Loader)]
pub fn loader(props: &LoaderProps) -> Html {
    let cell = (props.size as f32 * props.scale) as u32;
    let gap = cell;

    let container_style = format!(
        "grid-template-columns: repeat(3, {cell}px); \
         grid-template-rows: repeat(3, {cell}px); \
         gap: {gap}px;"
    );

    let square_style = format!("width: {cell}px; height: {cell}px;");

    let squares = (0..9).map(|index| {
        html! {
            <div
                key={index}
                class="rounded-sm bg-zinc-300 dark:bg-zinc-700 animate-pulse"
                style={square_style.clone()}
            />
        }
    });

    html! {
        <div data-component="loader" class="flex items-center justify-center">
            <div class="grid" style={container_style}>
                { for squares }
            </div>
        </div>
    }
}