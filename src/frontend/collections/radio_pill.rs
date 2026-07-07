use dioxus::prelude::*;

use crate::{
    frontend::{css::Css, elements::input::InputType},
    shared::models::select_option::SelectOption,
};

#[derive(Props, PartialEq, Clone)]
pub struct RadioPillProps {
    #[props(optional)]
    label: String,

    #[props]
    options: Vec<SelectOption>,

    #[props]
    selected: i64,

    #[props]
    callback: Callback<i64>,

    #[props]
    name: &'static str,
}

#[component]
pub fn RadioPill(props: RadioPillProps) -> Element {
    rsx! {
        div { class: Css::INPUT_CONTAINER,
            if !props.label.is_empty() {
                label { "{props.label}:" }
            }
            div { class: Css::RADIO_PILL_CONTAINER,
                for opt in props.options {
                    label { class: Css::HOVER,
                        input {
                            r#type: InputType::Radio.to_string(),
                            name: props.name,
                            oninput: move |_| {
                                props.callback.call(opt.id);
                            },
                            checked: props.selected == opt.id,
                        }
                        "{opt.label}"
                    }
                }
            }
        }
    }
}
