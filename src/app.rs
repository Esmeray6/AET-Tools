use std::collections::HashMap;

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
    // invoke without arguments
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;

    // invoke with arguments (default)
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
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
    #[at("/orbat_generator")]
    ORBATGenerator,
    // #[at("/inventory_viewer")]
    // InventoryViewer,
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
struct ORBATConvertData {
    orbat: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ORBATGenerationData {
    orbat: HashMap<String, u64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MissionData {
    sqm: String,
    players: Vec<Value>,
}

// Probably going to be useful in ORBATGenerator.
pub const ROLES: [&str; 41] = [
    "Zeus",
    "ZH",
    "COY",
    "PL",
    "PSgt",
    "SL",
    "TL",
    "RTO",
    "JTAC",
    "Medic",
    "ENG",
    "EOD",
    "DEMO",
    "MG",
    "AMG",
    "AR",
    "AAR",
    "AT",
    "AAT",
    "AA",
    "AAA",
    "Pointman",
    "DMR",
    "GL",
    "AMMO",
    "Rifleman",
    "Sniper",
    "Spotter",
    "MG_Team",
    "ARTY",
    "LOGI",
    "MBT",
    "IFV",
    "APC",
    "MRAP",
    "CAS",
    "CAP",
    "VTOL",
    "CAS_Heli",
    "Transport",
    "UAV",
];

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

#[function_component(ORBATGenerator)]
pub fn orbat_generator() -> Html {
    let navigator = use_navigator().unwrap();
    let go_home = Callback::from(move |_| navigator.push(&Route::Home));

    let rolelist: UseStateHandle<HashMap<String, u64>> = use_state(HashMap::new);

    let role_msg: UseStateHandle<String> = use_state(String::new);
    let role_msg2 = role_msg.clone();

    let onclick = Callback::from(move |_e: MouseEvent| {
        let rolelist = rolelist.clone();
        let role_msg = role_msg2.clone();
        let document = web_sys::window()
            .expect("No window found")
            .document()
            .expect("No document found");

        let mut new_rolelist = (*rolelist).clone();
        for role in ROLES {
            // info!("Role: {}", &role);
            let input = document.get_element_by_id(role);
            if let Some(number) = input {
                let value = number
                    .dyn_ref::<web_sys::HtmlInputElement>()
                    .and_then(|input| input.value().parse::<u64>().ok())
                    .unwrap_or(0);
                new_rolelist.insert(role.to_string(), value);
            }
        }
        info!("New rolelist: {:?}", new_rolelist);
        let args = to_value(&ORBATGenerationData {
            orbat: new_rolelist.clone(),
        })
        .expect("Failed to serialize ORBATGenerationData");
        rolelist.set(new_rolelist);
        let rolelist = rolelist.clone();
        // _e.prevent_default();
        info!("Generating ORBAT with roles: {:?}", *rolelist);

        info!("Role list: {:?}", *rolelist);
        spawn_local(async move {
            // let rolelist = rolelist.clone();
            let role_msg = role_msg.clone();
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            let result = invoke("orbat_generate", args).await;
            let generated = from_value::<String>(result).unwrap_or_default();
            // You may want to update a state here with `generated`
            info!("Generated ORBAT: {}", generated);
            role_msg.set(generated);
        });
    });

    html! {
        <div>
            <h1>{ "ORBAT Generator" }</h1>
            <div class="container row">
                { ROLES.into_iter().map(|role| html!{<div key={role}>
                <p class="role-name">{role}</p>
                <input type="number" id={role}/>
                </div>}).collect::<Html>() }
            </div>
            <div class="container column">
                <button {onclick}>{ "Create ORBAT" }</button>
                <textarea
                    name="generated-orbat"
                    id="generated-orbat"
                    placeholder="Enter mods here..."
                    value={role_msg.to_string()}
                />
                <button onclick={go_home}>{ "Go Home" }</button>
            </div>
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
        <div class="container">
            <h1>{ "Command Line Generator" }</h1>
            <div class="container">
                <textarea
                    name="command-line"
                    id="command-line"
                    placeholder="Enter mods here..."
                    value={modlist.mods.to_string()}
                />
                <input accept=".html" {onchange} type="file" name="mod-preset" id="mod-preset" />
            </div>
            <p id="missing-mods">{ modlist.missing_mods.to_string() }</p>
            <button id="go-home-button" {onclick}>{ "Go Home" }</button>
        </div>
    }
}

#[function_component(ORBATSorter)]
pub fn orbat_sorter() -> Html {
    let navigator = use_navigator().unwrap();
    let gohome = Callback::from(move |_| navigator.push(&Route::Home));

    let role_input_ref = use_node_ref();

    let rolelist = use_state(String::new);

    let role_msg = use_state(String::new);

    {
        let role_msg = role_msg.clone();
        let rolelist = rolelist.clone();
        let name2 = rolelist.clone();
        use_effect_with(name2, move |_| {
            spawn_local(async move {
                if rolelist.is_empty() {
                    return;
                }

                let args = to_value(&ORBATConvertData {
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

    let onclick = {
        let rolelist = rolelist.clone();
        let role_input_ref = role_input_ref.clone();
        info!("{:?} {:?}", *rolelist, &role_input_ref);
        Callback::from(move |_e: MouseEvent| {
            // _e.prevent_default();
            rolelist.set(
                role_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    html! {
        <main class="container">
            <div class="container">
                <textarea
                    type="text"
                    id="convert-input"
                    ref={role_input_ref}
                    placeholder="Enter the list of roles..."
                />
                <button class="row" id="submit-button" type="submit" {onclick}>
                    { "Convert" }
                </button>
                <textarea class="row" id="role-msg" value={role_msg.to_string()} />
            </div>
            <button id="go-home-button" onclick={gohome} type="button">{ "Go Home" }</button>
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
