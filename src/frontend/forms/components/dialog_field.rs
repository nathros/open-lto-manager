use dioxus::prelude::*;

use crate::{
    frontend::{
        css::Css,
        elements::input::{Input, InputType},
        js::js_close,
        modules::dialog::Dialog,
    },
    shared::validator::validator_base::{TValidator, Validator},
    static_concat,
};

#[derive(Props, PartialEq, Clone)]
pub struct DialogFieldProps {
    #[props]
    id: &'static str,

    #[props]
    label: &'static str,

    #[props]
    validator: Validator<String>,

    #[props]
    save: Callback<String>,
    // For async Callback with same call signature
    // https://github.com/DioxusLabs/dioxus/discussions/5207
    // Just use spawn() for now
    // save: Callback<String, Pin<Box<dyn Future<Output = ()>>>>,
}

#[component]
pub fn DialogField(props: DialogFieldProps) -> Element {
    let mut input_value = use_signal(|| "".to_string());
    let mut input_error = use_signal(|| None);
    use_memo(move || {
        input_error.set(props.validator.validate(&input_value()));
    });
    let mut close_dialog = move || {
        js_close(props.id);
        props.save.call(input_value());
        input_value.write().clear();
    };

    rsx! {
        Dialog {
            id: props.id,
            button_bar: true,
            button_bar_save: true,
            button_bar_close: true,
            button_bar_save_disabled: input_error().is_some(),
            button_bar_save_click: move |_evt: Event<MouseData>| {
                close_dialog();
            },
            form {
                class: static_concat!(Css::FORM_GRID, Css::LG),
                onsubmit: move |evt| {
                    evt.prevent_default(); // Stops refresh on press Enter
                },
                Input {
                    type_: InputType::Text,
                    label: props.label,
                    oninput: move |evt: Event<FormData>| {
                        input_value.set(evt.value());
                    },
                    onkeydown: move |evt: Event<KeyboardData>| {
                        if evt.code() == Code::Enter && input_error().is_none() {
                            close_dialog();
                        }
                    },
                    clear_button: input_value,
                    value: input_value(),
                    validation: input_error,
                }
            }
        }
    }
}
