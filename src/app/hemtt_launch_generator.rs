use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{EventTarget, FileReader, HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct ModData {
    mods: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModArgs<'a> {
    modpreset: &'a str,
}

use crate::app::invoke;
use crate::app::Route;

#[function_component(HEMTTLaunchGenerator)]
pub fn hemtt_launch_generator() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    let modlist = use_state(|| ModData {
        mods: String::new(),
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
                                    let file_data = ModArgs { modpreset: &text };
                                    let val = to_value(&file_data).unwrap();
                                    let x = invoke("hemtt_launch_convert", val).await;
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
        <div class="container column">
            <h1>{ "HEMTT Launch Generator" }</h1>
            <div class="container column">
                <textarea
                    name="result-config"
                    id="result-config"
                    placeholder="HEMTT config goes here..."
                    value={modlist.mods.to_string()}
                />
                <input accept=".html" {onchange} type="file" name="mod-list" id="mod-list" />
            </div>
            <button id="go-home-button" {onclick}>{ "Go Home" }</button>
        </div>
    }
}
