use log::info;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_wasm_bindgen::{from_value, to_value};
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

#[derive(Serialize, Deserialize, Debug)]
struct ModArgs<'a> {
    modpreset: &'a str,
    backticks: bool,
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/command_line_generator")]
    CommandLineGenerator,
    #[at("/orbat_sorter")]
    ORBATSorter,
    #[at("/inventory_viewer")]
    InventoryViewer,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModData {
    mods: String,
    missing_mods: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ORBATData {
    orbat: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct MissionData {
    sqm: String,
    players: Vec<Value>,
}

// impl Display for ModData {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{}",
//             format!(
//                 "{{ ModData mods: {}, missing_mods: {} }}",
//                 self.mods, self.missing_mods
//             )
//         )
//     }
// }

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();

    let navig = navigator.clone();
    let command_line_redirect = Callback::from(move |_| navig.push(&Route::CommandLineGenerator));

    let navig = navigator.clone();
    let orbat_sorter_redirect = Callback::from(move |_| navig.push(&Route::ORBATSorter));

    let navig = navigator.clone();
    let inventory_viewer_redirect = Callback::from(move |_| navig.push(&Route::InventoryViewer));

    html! {
        <div class="container">
            <h1>{ "Antistasi Event Team Tools" }</h1>
            <button onclick={command_line_redirect}>{ "Command Line Generator" }</button>
            <button onclick={orbat_sorter_redirect}>{ "ORBAT Sorter" }</button>
            <button onclick={inventory_viewer_redirect}>{ "Inventory Viewer" }</button>
        </div>
    }
}

#[function_component(InventoryViewer)]
pub fn inventory_viewer() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    let mission_data = use_state(|| MissionData {
        sqm: String::new(),
        players: Vec::new(),
    });

    let onchange = {
        let mission_data = mission_data.clone();
        Callback::from(move |event: Event| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                if let Some(files) = input.files() {
                    if let Some(file) = files.get(0) {
                        let reader = FileReader::new().unwrap();
                        let onloadend = {
                            let mission_data = mission_data.clone();
                            Closure::wrap(Box::new(move |event: Event| {
                                let reader =
                                    event.target().unwrap().dyn_into::<FileReader>().unwrap();
                                let text = reader.result().unwrap().as_string().unwrap();

                                let mission_data = mission_data.clone();
                                // Invoke the Tauri command with the file content
                                spawn_local(async move {
                                    //let mission_data = mission_data.clone();
                                    let file_data = MissionData {
                                        sqm: text,
                                        players: Vec::new(),
                                    };
                                    let val = to_value(&file_data).unwrap();
                                    let x = invoke("inventory_view", val).await;
                                    info!(
                                        "{:#?}",
                                        serde_wasm_bindgen::from_value::<Value>(x.clone()).unwrap()
                                    );
                                    mission_data.set(from_value(x).unwrap());
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
            <h1>{ "Inventory Viewer" }</h1>
            <div class="container">
                <textarea
                    name="inventory-viewer"
                    id="inventory-viewer"
                    placeholder="Enter mods here..."
                    value={mission_data.sqm.to_string()}
                />
                <input {onchange} type="file" name="mod-preset" id="mod-preset" />
            </div>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

#[function_component(CommandLineGenerator)]
pub fn command_line_generator() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    let modlist = use_state(|| ModData {
        mods: String::new(),
        missing_mods: String::new(),
    });

    let onchange = {
        let mod_data = modlist.clone();
        Callback::from(move |event: Event| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                if let Some(files) = input.files() {
                    if let Some(file) = files.get(0) {
                        let reader = FileReader::new().unwrap();
                        let onloadend = {
                            let mod_data = mod_data.clone();
                            Closure::wrap(Box::new(move |event: Event| {
                                let reader =
                                    event.target().unwrap().dyn_into::<FileReader>().unwrap();
                                let text = reader.result().unwrap().as_string().unwrap();

                                let mod_data = mod_data.clone();
                                // Invoke the Tauri command with the file content
                                spawn_local(async move {
                                    //let mod_data = mod_data.clone();
                                    let file_data = ModArgs {
                                        modpreset: &text,
                                        backticks: false,
                                    };
                                    let val = to_value(&file_data).unwrap();
                                    let x = invoke("command_line_convert", val).await;
                                    mod_data.set(from_value(x).unwrap());
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
                    value={modlist.mods.to_string()}
                />
                <input {onchange} type="file" name="mod-preset" id="mod-preset" />
            </div>
            <p id="missing-mods">{ modlist.missing_mods.to_string() }</p>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

#[function_component(ORBATSorter)]
pub fn orbat_sorter() -> Html {
    let role_input_ref = use_node_ref();

    let rolelist = use_state(|| String::new());

    let role_msg = use_state(|| String::new());
    {
        let role_msg = role_msg.clone();
        let rolelist = rolelist.clone();
        let name2 = rolelist.clone();
        use_effect_with(name2, move |_| {
            spawn_local(async move {
                if rolelist.is_empty() {
                    return;
                }

                let args = to_value(&ORBATData {
                    orbat: rolelist.to_string(),
                })
                .unwrap();
                // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                let new_msg = invoke("orbat_convert", args).await.as_string().unwrap();
                role_msg.set(new_msg);
            });

            || {}
        });
    }

    let convert = {
        let rolelist = rolelist.clone();
        let role_input_ref = role_input_ref.clone();
        dbg!(&rolelist, &role_input_ref);
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            rolelist.set(dbg!(role_input_ref
                .cast::<web_sys::HtmlInputElement>()
                .unwrap()
                .value()));
        })
    };

    html! {
        <main class="container">
            <form class="row" onsubmit={convert}>
                <textarea
                    type="text"
                    id="convert-input"
                    ref={role_input_ref}
                    placeholder="Enter the list of roles..."
                />
                <button class="row" id="submit-button" type="submit">{ "Convert" }</button>
            </form>
            <textarea class="row" id="role-msg" value={role_msg.to_string()} />
        </main>
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
        Route::InventoryViewer => html! { <InventoryViewer /> },
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
