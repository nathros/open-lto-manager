use dioxus::prelude::*;

use crate::{
    either,
    frontend::{
        collections::{radio_pill::RadioPill, tape_preview::TapePreview},
        css::Css,
        elements::{
            button::Button,
            input::{Input, InputBarcode, InputType},
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

    let check_types = types.clone();
    let get_selected_type = move |id: i64| {
        check_types
            .iter()
            .find(|t| t.id == id)
            .cloned()
            .unwrap_or_default()
    };
    let selected_type = get_selected_type(tape().tape_type_id);

    let current_designation = types
        .iter()
        .find(|p| p.id == tape().tape_type_id)
        .unwrap_or(&RecordTapeType::default())
        .clone();
    let barcode_designation = if tape().worm {
        current_designation.id_worm.clone()
    } else {
        current_designation.id_reg.clone()
    };

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
                    options: vec_into(types.clone()),
                    selected: tape().tape_type_id,
                    onchange: move |evt: Event<FormData>| {
                        let selected_index = evt.value().parse::<i64>().unwrap_or_default();
                        tape.write().tape_type_id = selected_index;
                        let selected_type = get_selected_type(selected_index);
                        if !selected_type.supports_worm {
                            tape.write().worm = false;
                        }
                        if !selected_type.supports_ltfs {
                            tape.write().format = TapeFormat::Tar;
                        }
                    },
                }
                Select {
                    label: "Manufacturers".to_string(),
                    required: true,
                    options: vec_into(manufacturers.clone()),
                    selected: tape().manufacturer_id,
                    onchange: move |evt: Event<FormData>| {
                        tape.write().manufacturer_id = evt.value().parse::<i64>().unwrap_or_default();
                    },
                }
                RadioPill {
                    label: "Tape Format".to_string(),
                    options: selected_type
                        .get_supported_format()
                        .into_iter()
                        .map(|t| SelectOption {
                            id: t.into(),
                            label: format!("{:?}", t),
                        })
                        .collect(),
                    callback: use_callback(move |id: i64| {
                        tape.write().format = id.into();
                    }),
                    selected: tape().format.into(),
                    name: "test",
                }
                InputBarcode {
                    type_: InputType::Text,
                    label: "Barcode".to_string(),
                    oninput: move |evt: Event<FormData>| {
                        tape.write().barcode = evt.value().to_uppercase();
                    },
                    validation: barcode_err,
                    value: tape().barcode,
                    maxlength: BARCODE_LEN,
                    meta: barcode_designation.clone(),
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
                Input {
                    type_: InputType::Checkbox,
                    label: "WORM".to_string(),
                    oninput: move |evt: Event<FormData>| {
                        tape.write().worm = evt.value() == "true";
                    },
                    checked: tape().worm,
                    disabled: !selected_type.supports_worm,
                }
                Input {
                    type_: InputType::Checkbox,
                    label: "Compression Enabled".to_string(),
                    oninput: move |evt: Event<FormData>| {
                        tape.write().compressed = evt.value() == "true";
                    },
                    checked: tape().compressed,
                }
                p { "form_invalid: {form_invalid}" }
                p { "len: {tape().barcode.len()}" }
                p { "Debug: {tape():?}" }

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
                TapePreview {
                    preview: tape(),
                    manufacturers,
                    tapes_list: types,
                    designation: barcode_designation,
                    size: "30",
                }
            }
        }
    }
}
