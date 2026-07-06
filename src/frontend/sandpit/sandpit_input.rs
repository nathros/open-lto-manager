use dioxus::prelude::*;

use crate::{
    frontend::{
        css::Css,
        elements::input::{Input, InputType},
        forms::validator::{TValidator, Validator},
    },
    static_concat,
};

#[component]
pub fn SandpitInput() -> Element {
    let validator_1 = Validator::<String>::create().with_max_length(4);
    let mut value_1 = use_signal(|| "".to_string());
    let mut err_msg_1 = use_signal(move || None);

    let validator_2 = Validator::<String>::create().with_expected_length(4);
    let mut value_2 = use_signal(|| "too long".to_string());
    let mut err_msg_2 = use_signal(move || None);

    use_memo(move || {
        err_msg_1.set(validator_1.validate(&value_1()));
        err_msg_2.set(validator_2.validate(&value_2()));
    });

    rsx! {
        div { class: static_concat!(Css::FLEX_COL, Css::FLEX_ALIGN_LEFT),
            Input {
                type_: InputType::Text,
                label: "Max len(4)".to_string(),
                oninput: move |evt: Event<FormData>| {
                    value_1.set(evt.value());
                },
                validation: err_msg_1,
                value: value_1,
            }
            hr {}
            Input {
                type_: InputType::Text,
                label: "Expected len(4)".to_string(),
                oninput: move |evt: Event<FormData>| {
                    value_2.set(evt.value());
                },
                validation: err_msg_2,
                value: value_2,
            }
            hr {}
            Input {
                type_: InputType::Text,
                label: "Disabled".to_string(),
                value: "test data",
                disabled: true,
            }
        }
    }
}
