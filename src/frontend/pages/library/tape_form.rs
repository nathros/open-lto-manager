use dioxus::prelude::*;

use crate::{
    either,
    frontend::{
        css::Css,
        elements::{
            button::Button,
            input::{Input, InputType},
            vertical_divider::VerticalDivider,
        },
        forms::validator::{TValidator, Validator},
    },
    shared::models::database::tape::model_tape::{BARCODE_LEN, BARCODE_VALID_CHARS, RecordTape},
};

#[component]
pub fn TapeForm(id: i64) -> Element {
    let mut tape: Signal<RecordTape> = use_signal(|| RecordTape::default());

    let barcode_validator = Validator::<String>::create()
        .with_only_allowed_chars(BARCODE_VALID_CHARS)
        .with_expected_length(BARCODE_LEN);
    let mut barcode_err = use_signal(move || None);

    let form_invalid = use_memo(move || {
        // Validate form
        barcode_err.set(barcode_validator.validate(&tape().barcode));

        barcode_err().is_some()
    });

    rsx! {
        div { class: Css::FLEX_ROW,
            form { class: Css::FORM_GRID,
                Input {
                    type_: InputType::Text,
                    label: "Barcode".to_string(),
                    oninput: move |evt: Event<FormData>| {
                        tape.write().barcode = evt.value().to_uppercase();
                    },
                    validation: barcode_err,
                    value: tape().barcode,
                    maxlength: BARCODE_LEN,
                }
                p { "form_invalid: {form_invalid}" }
                p { "len: {tape().barcode.len()}" }

                hr {}
                Button {
                    primary: true,
                    disabled: form_invalid(),
                    text: either!(id == 0, "Add", "Update"),
                }
            }
            VerticalDivider {}
            div {
                // Preview
            }
        }
    }
}
