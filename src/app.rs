use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::{command_line_generator::CommandLineGenerator, orbat_generator::ORBATGenerator, orbat_sorter::ORBATSorter};

mod command_line_generator;
mod orbat_sorter;
mod orbat_generator;

#[wasm_bindgen]
extern "C" {
    // invoke without arguments
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;

    // invoke with arguments (default)
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/command_line_generator")]
    CommandLineGenerator,
    #[at("/orbat_sorter")]
    ORBATSorter,
    #[at("/orbat_generator")]
    ORBATGenerator,
    // #[at("/inventory_viewer")]
    // InventoryViewer,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();

    let navig = navigator.clone();
    let command_line_redirect = Callback::from(move |_| navig.push(&Route::CommandLineGenerator));

    let navig = navigator.clone();
    let orbat_sorter_redirect = Callback::from(move |_| navig.push(&Route::ORBATSorter));

    let navig = navigator.clone();
    let orbat_generator_redirect = Callback::from(move |_| navig.push(&Route::ORBATGenerator));

    println!("Home");

    html! {
        <div class="container column">
            <h1>{ "Antistasi Event Team Tools" }</h1>
            <button onclick={command_line_redirect}>{ "Command Line Generator" }</button>
            <button onclick={orbat_sorter_redirect}>{ "ORBAT Sorter" }</button>
            <button onclick={orbat_generator_redirect}>{ "ORBAT Generator" }</button>
        </div>
    }
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::NotFound => html! {
            <div class="container">
                <h1>{ "404 Not Found" }</h1>
                <p>{ "I don't know how you ended up here." }</p>
            </div>
        },
        Route::Home => html! { <Home /> },
        Route::CommandLineGenerator => html! { <CommandLineGenerator /> },
        Route::ORBATSorter => html! { <ORBATSorter /> },
        Route::ORBATGenerator => html! { <ORBATGenerator /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
