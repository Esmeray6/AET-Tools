use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{EventTarget, FileReader, HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct ModData {
    mod_preset: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModArgs {
    firstmodpreset: String,
    secondmodpreset: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct MergeResult {
    text: String,
}

use crate::app::Route;
use crate::app::invoke;

#[function_component(ModMerger)]
pub fn mod_merger() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick_home = Callback::from(move |_| navigator.push(&Route::Home));

    let modlist1 = use_state(|| ModData {
        mod_preset: String::new(),
    });

    let modlist2 = use_state(|| ModData {
        mod_preset: String::new(),
    });

    let merge_result = use_state(|| MergeResult {
        text: String::new(),
    });

    let onclick_merge = {
        let mod_list1 = modlist1.clone();
        let mod_list2 = modlist2.clone();
        let merge_result = merge_result.clone();

        Callback::from(move |_| {
            let mod_list1 = mod_list1.clone();
            let mod_list2 = mod_list2.clone();
            let merge_result = merge_result.clone();

            spawn_local(async move {
                let args = ModArgs {
                    firstmodpreset: mod_list1.mod_preset.clone(),
                    secondmodpreset: mod_list2.mod_preset.clone(),
                };
                let js_value = to_value(&args).unwrap();
                let res = invoke("merge_modsets", js_value).await;
                merge_result.set(MergeResult {
                    text: format!("Saved at {}", from_value::<String>(res).unwrap()),
                });
            });
        })
    };

    let onchange_first = {
        let mod_data = modlist1.clone();
        Callback::from(move |event: Event| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input
                && let Some(files) = input.files()
                && let Some(file) = files.get(0)
            {
                let reader = FileReader::new().unwrap();
                let onloadend = {
                    let mod_data = mod_data.clone();
                    Closure::wrap(Box::new(move |event: Event| {
                        let reader = event.target().unwrap().dyn_into::<FileReader>().unwrap();
                        let file_data = reader.result().unwrap().as_string().unwrap();

                        let mod_data = mod_data.clone();
                        // Invoke the Tauri command with the file content
                        mod_data.set(ModData {
                            mod_preset: file_data,
                        });
                    }) as Box<dyn FnMut(_)>)
                };
                reader.set_onloadend(Some(onloadend.as_ref().unchecked_ref()));
                reader.read_as_text(&file).unwrap();
                onloadend.forget();
            }
        })
    };

    let onchange_second = {
        let mod_data = modlist2.clone();
        Callback::from(move |event: Event| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input
                && let Some(files) = input.files()
                && let Some(file) = files.get(0)
            {
                let reader = FileReader::new().unwrap();
                let onloadend = {
                    let mod_data = mod_data.clone();
                    Closure::wrap(Box::new(move |event: Event| {
                        let reader = event.target().unwrap().dyn_into::<FileReader>().unwrap();
                        let file_data = reader.result().unwrap().as_string().unwrap();

                        let mod_data = mod_data.clone();
                        // Invoke the Tauri command with the file content
                        mod_data.set(ModData {
                            mod_preset: file_data,
                        });
                    }) as Box<dyn FnMut(_)>)
                };
                reader.set_onloadend(Some(onloadend.as_ref().unchecked_ref()));
                reader.read_as_text(&file).unwrap();
                onloadend.forget();
            }
        })
    };

    html! {
        <div class="container column">
            <h1>{ "Mod Merger" }</h1>
            <div class="container column">
                <p class="role">{ "Modset 1" }</p>
                <input
                    accept=".html"
                    onchange={onchange_first}
                    type="file"
                    name="mod-preset"
                    id="mod-preset"
                />
            </div>
            <div class="container column">
                <p class="role">{ "Modset 1" }</p>
                <input
                    accept=".html"
                    onchange={onchange_second}
                    type="file"
                    name="mod-preset"
                    id="mod-preset"
                />
            </div>
            <p id="merge-result">{ merge_result.text.to_string() }</p>
            <button id="merge-modsets" onclick={onclick_merge}>{ "Merge and Download" }</button>
            <button id="go-home-button" onclick={onclick_home}>{ "Go Home" }</button>
        </div>
    }
}
