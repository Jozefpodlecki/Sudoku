use yew::prelude::*;
use yew_router::prelude::*;
use yew_icons::{Icon, IconData};

use crate::{components::Layout, routes::Route};

#[function_component(NotFound)]
pub fn not_found() -> Html {
    let navigator = use_navigator().unwrap();

    let go_home = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Home);
        })
    };

    html! {
        <Layout>
            <div class="flex flex-col items-center justify-center min-h-[60vh] text-center">
                <Icon 
                    data={IconData::LUCIDE_FILE_QUESTION} 
                    class="text-gray-600 mb-6" 
                    width="80px"
                    height="80px"
                />
                <h1 class="text-7xl font-bold text-white mb-4">{"404"}</h1>
                <h2 class="text-2xl text-gray-300 mb-2">{"Page Not Found"}</h2>
                <p class="text-gray-500 mb-8 max-w-md">
                    {"The page you're looking for doesn't exist or has been moved."}
                </p>
                <button 
                    onclick={go_home}
                    class="flex items-center gap-2 px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg transition-colors"
                >
                    <Icon data={IconData::LUCIDE_HOME} width="18px" />
                    {"Go Home"}
                </button>
            </div>
        </Layout>
    }
}