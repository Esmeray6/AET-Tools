use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::invoke;
use crate::app::Route;

#[derive(Serialize, Deserialize, Debug)]
struct ORBATData {
    orbat: String,
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

    let onclick = {
        let rolelist = rolelist.clone();
        let role_input_ref = role_input_ref.clone();
        dbg!(&rolelist, &role_input_ref);
        Callback::from(move |_e: MouseEvent| {
            // e.prevent_default();
            rolelist.set(dbg!(role_input_ref
                .cast::<web_sys::HtmlInputElement>()
                .unwrap()
                .value()));
        })
    };

    html! {
        <main class="container column">
            <div class="container column">
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
