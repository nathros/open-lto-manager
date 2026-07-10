use dioxus::prelude::*;

use crate::{
    either,
    frontend::{
        collections::radio_pill::RadioPill,
        css::Css,
        elements::{
            button::Button,
            input::{Input, InputType},
            select::Select,
            vertical_divider::VerticalDivider,
        },
        forms::validator::{TValidator, Validator},
    },
    shared::models::{
        database::{
            manufacturer::model_manufacturer::RecordManufacturer,
            tape::model_tape::{BARCODE_LEN, BARCODE_VALID_CHARS, RecordTape, TapeFormat},
            tape_type::model_tape_type::RecordTapeType,
        },
        select_option::{SelectOption, vec_into},
    },
};

#[component]
pub fn TapeForm(
    id: i64,
    types: Vec<RecordTapeType>,
    manufacturers: Vec<RecordManufacturer>,
) -> Element {
    let mut tape: Signal<RecordTape> = use_signal(|| RecordTape::default());

    let barcode_validator = Validator::<String>::create()
        .with_only_allowed_chars(BARCODE_VALID_CHARS)
        .with_expected_length(BARCODE_LEN);
    let mut barcode_err = use_signal(move || None);

    let form_invalid = use_memo(move || {
        // Validate form
        barcode_err.set(barcode_validator.validate(&tape().barcode));

        barcode_err().is_some() || tape().tape_type_id == 0 || tape().manufacturer_id == 0
    });

    rsx! {
        div { class: Css::FLEX_ROW,
            form { class: Css::FORM_GRID,
                Select {
                    label: "Tape Type".to_string(),
                    required: true,
                    options: vec_into(types),
                    selected: tape().tape_type_id,
                    onchange: move |evt: Event<FormData>| {
                        tape.write().tape_type_id = evt.value().parse::<i64>().unwrap_or_default();
                    },
                }
                Select {
                    label: "Manufacturers".to_string(),
                    required: true,
                    options: vec_into(manufacturers),
                    selected: tape().manufacturer_id,
                    onchange: move |evt: Event<FormData>| {
                        tape.write().manufacturer_id = evt.value().parse::<i64>().unwrap_or_default();
                    },
                }
                RadioPill {
                    label: "Tape Format".to_string(),
                    options: [TapeFormat::Tar, TapeFormat::LTFS]
                        .map(|t| SelectOption {
                            id: t.into(),
                            label: format!("{:?}", t),
                        })
                        .to_vec(),
                    callback: use_callback(move |id: i64| {
                        tape.write().format = id.into();
                    }),
                    selected: tape().format.into(),
                    name: "test",
                }
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
                Input {
                    type_: InputType::Text,
                    label: "Serial Number".to_string(),
                    oninput: move |evt: Event<FormData>| {
                        tape.write().serial = evt.value();
                    },
                    value: tape().serial,
                    maxlength: 24,
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
