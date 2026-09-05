use yew::{Callback, Html, html};
use yewlish_checkbox::{CheckboxRenderAsProps, CheckedState};

pub fn render_as_input(props: CheckboxRenderAsProps) -> Html {
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
}