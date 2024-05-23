use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{EventTarget, FileReader, HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct ModArgs<'a> {
    modpreset: &'a str,
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/command_line_generator")]
    CommandLineGenerator,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::CommandLineGenerator));
    html! {
        <div>
            <button onclick={onclick}>{ "Command Line Generator" }</button>
        </div>
    }
}

#[function_component(CommandLineGenerator)]
pub fn command_line_generator() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    let modlist = use_state(|| String::new());

    let onchange = {
        let file_content = modlist.clone();
        Callback::from(move |event: Event| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                if let Some(files) = input.files() {
                    if let Some(file) = files.get(0) {
                        let reader = FileReader::new().unwrap();
                        let onloadend = {
                            let file_content = file_content.clone();
                            Closure::wrap(Box::new(move |event: Event| {
                                let reader =
                                    event.target().unwrap().dyn_into::<FileReader>().unwrap();
                                let text = reader.result().unwrap().as_string().unwrap();

                                let file_content = file_content.clone();
                                // Invoke the Tauri command with the file content
                                spawn_local(async move {
                                    //let file_content = file_content.clone();
                                    let file_data = ModArgs { modpreset: &text };
                                    let val = to_value(&file_data).unwrap();
                                    let x = invoke("convert", val).await.as_string().unwrap();
                                    file_content.set(x);
                                });
                            }) as Box<dyn FnMut(_)>)
                        };
                        reader.set_onloadend(Some(onloadend.as_ref().unchecked_ref()));
                        reader.read_as_text(&file).unwrap();
                        onloadend.forget();
                    }
                }
            }
        })
    };

    html! {
        <div>
            <h1>{ "Command Line Generator" }</h1>
            <div class="container">
                <textarea
                    name="command-line"
                    id="command-line"
                    placeholder="Enter mods here..."
                    value={modlist.to_string()}
                />
                <input {onchange} type="file" name="mod-preset" id="mod-preset" />
            </div>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::CommandLineGenerator => html! { <CommandLineGenerator /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
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
