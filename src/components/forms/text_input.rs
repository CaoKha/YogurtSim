// use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    pub name: String,
    pub on_username_change: Callback<String>,
}

#[function_component(CustomTextInput)]
pub fn custom_text_input(
    TextInputProps {
        name,
        on_username_change,
    }: &TextInputProps,
) -> Html {
    let onchange = {
        let handle_change = on_username_change.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            handle_change.emit(value);
        })
    };
    html! {
        <>
        <input type="text" name={name.clone()} onchange={onchange}/>
        </>
    }
}