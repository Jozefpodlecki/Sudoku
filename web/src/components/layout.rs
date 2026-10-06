use yew::prelude::*;

use crate::components::{Footer, Navbar};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LayoutProps {
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
        <div class="flex flex-col min-h-screen bg-white dark:bg-zinc-950 text-zinc-900 dark:text-zinc-100">
            <Navbar />
            <main class="flex-1 w-full min-w-0 flex justify-center">
                { for props.children.iter() }
            </main>
            <Footer />
        </div>
    }
}