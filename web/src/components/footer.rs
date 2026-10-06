use yew::prelude::*;
use yew_icons::{Icon, IconData};

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="border-t border-zinc-200 dark:border-zinc-800 mt-auto">
            <div class="max-w-6xl mx-auto px-6 py-6 flex items-center justify-between text-sm text-zinc-500 dark:text-zinc-500">
                <div class="flex items-center gap-2 text-xs text-zinc-500 dark:text-zinc-500">
                    <span>{ "\u{00A9} 2026 Jozef Podlecki" }</span>
                    <span class="text-zinc-300 dark:text-zinc-700">{ "·" }</span>
                    <span>{ "Built with Rust, Yew, WebAssembly" }</span>
                </div>
                <a
                    href="https://github.com/Jozefpodlecki/Sudoku"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="inline-flex items-center gap-2 hover:text-zinc-900 dark:hover:text-zinc-100 transition-colors"
                >
                    <Icon data={IconData::LUCIDE_GITHUB} width="16px" height="16px" />
                    { "Source" }
                </a>
            </div>
        </footer>
    }
}