use std::collections::HashMap;

use log::info;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::invoke;
use crate::app::Route;

#[derive(Serialize, Deserialize, Debug)]
struct ORBATGenerationData {
    orbat: HashMap<String, u64>,
}

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
        // info!("New rolelist: {:?}", new_rolelist);
        let args = to_value(&ORBATGenerationData {
            orbat: new_rolelist.clone(),
        })
        .expect("Failed to serialize ORBATGenerationData");
        rolelist.set(new_rolelist);
        // let rolelist = rolelist.clone();
        // _e.prevent_default();
        // info!("Generating ORBAT with roles: {:?}", *rolelist);

        // info!("Role list: {:?}", *rolelist);
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
                { ROLES.into_iter().map(|role| html_nested! {
                    <div class="role" key={role}>
                        <p class="role-name">{role}</p>
                        <input type="number" id={role}/>
                    </div>
                }).collect::<Html>() }
            </div>
            <div class="container column">
                <button id="create-orbat-button" {onclick}>{ "Create ORBAT" }</button>
                <textarea
                    name="generated-orbat"
                    id="generated-orbat"
                    placeholder="Final ORBAT here..."
                    readonly=true
                    value={role_msg.to_string()}
                />
                <button id="go-home-button" onclick={go_home}>{ "Go Home" }</button>
            </div>
        </div>
    }
}
