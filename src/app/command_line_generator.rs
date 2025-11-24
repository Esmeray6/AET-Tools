use log::info;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{EventTarget, FileReader, HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::*;
use yewlish_checkbox::*;

#[derive(Serialize, Deserialize, Debug)]
struct ModData {
    mods: String,
    missing_mods: String,
    optional_mods: String,
    dlcs_list: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModArgs<'a> {
    modpreset: &'a str,
    backticks: bool,
}

use crate::app::invoke;
use crate::app::Route;

#[function_component(CommandLineGenerator)]
pub fn command_line_generator() -> Html {
    let navigator = use_navigator().unwrap();

    let checked_state = use_state(|| CheckedState::Checked);
    let onclick = Callback::from(move |_| navigator.push(&Route::Home));

    let modlist = use_state(|| ModData {
        mods: String::new(),
        missing_mods: String::new(),
        optional_mods: String::new(),
        dlcs_list: String::new(),
    });

    let onchange = {
        let checked_state = checked_state.clone();
        let mod_data = modlist.clone();
        Callback::from(move |event: Event| {
            let checked_state = checked_state.clone();
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                if let Some(files) = input.files() {
                    if let Some(file) = files.get(0) {
                        let reader = FileReader::new().unwrap();
                        let onloadend = {
                            let mod_data = mod_data.clone();
                            Closure::wrap(Box::new(move |event: Event| {
                                let checked_state = checked_state.clone();
                                let reader =
                                    event.target().unwrap().dyn_into::<FileReader>().unwrap();
                                let text = reader.result().unwrap().as_string().unwrap();

                                let mod_data = mod_data.clone();
                                // Invoke the Tauri command with the file content
                                spawn_local(async move {
                                    //let mod_data = mod_data.clone();
                                    info!("{}", *checked_state);
                                    let file_data = ModArgs {
                                        modpreset: &text,
                                        backticks: matches!(*checked_state, CheckedState::Checked),
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

    let on_checked_change = {
        let modlist = modlist.clone();
        let checked_state = checked_state.clone();
        Callback::from(move |new_state: CheckedState| {
            if !modlist.mods.is_empty() {
                let backticks = matches!(new_state, CheckedState::Checked);
                let mut new_mods = modlist
                    .mods
                    .strip_prefix("```\n")
                    .unwrap_or(&modlist.mods)
                    .strip_suffix("\n```")
                    .unwrap_or(&modlist.mods)
                    .to_string();
                new_mods = if backticks {
                    format!("```\n{new_mods}\n```")
                } else {
                    new_mods
                };
                modlist.set(ModData {
                    mods: new_mods,
                    missing_mods: modlist.missing_mods.clone(),
                    optional_mods: modlist.optional_mods.clone(),
                    dlcs_list: modlist.dlcs_list.clone(),
                });
            }
            checked_state.set(new_state);
        })
    };

    let render_as = Callback::from(|props: CheckboxRenderAsProps| {
        let is_checked = props.checked == CheckedState::Checked;

        html! {
            <label
                id={props.id.clone().map(|checkbox_id| format!("{checkbox_id}-label"))}
                class={props.class.clone()}
            >
                <input
                    id={props.id}
                    type="checkbox"
                    checked={is_checked}
                    onclick={Callback::from(move |_| props.toggle.emit(()))}
                    disabled={props.disabled}
                    required={props.required}
                    name={props.name.clone()}
                    value={props.value.clone()}
                />
                { for props.children.iter() }
            </label>
        }
    });

    html! {
        <div class="container column">
            <h1>{ "Command Line Generator" }</h1>
            <div class="container column">
                <textarea
                    name="command-line"
                    id="command-line"
                    placeholder="Enter mods here..."
                    value={modlist.mods.to_string()}
                />
                <Checkbox
                    id="backticks-toggle"
                    render_as={render_as}
                    default_checked={CheckedState::Checked}
                    checked={(*checked_state).clone()}
                    on_checked_change={on_checked_change}
                >
                    { "Add backticks?" }
                </Checkbox>
                <input accept=".html" {onchange} type="file" name="mod-preset" id="mod-preset" />
            </div>
            <p id="missing-mods">{ modlist.missing_mods.to_string() }</p>
            <p id="optional-mods">{ modlist.optional_mods.to_string() }</p>
            <button id="go-home-button" {onclick}>{ "Go Home" }</button>
        </div>
    }
}
