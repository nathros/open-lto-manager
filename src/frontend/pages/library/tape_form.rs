use dioxus::prelude::*;

use crate::{
    backend::api::api_tape::{api_add_tape, api_tape_barcode_exists, api_tape_serial_exists},
    either,
    frontend::{
        collections::{
            message::{Message, MessageDetails},
            radio_pill::RadioPill,
            tape_preview::TapePreview,
        },
        css::Css,
        elements::{
            button::Button,
            input::{Input, InputBarcode, InputType},
            select::Select,
            vertical_divider::VerticalDivider,
        },
        forms::validator::{TValidator, Validator},
        level::Level,
    },
    shared::{
        r#const::Const,
        models::{
            database::{
                manufacturer::model_manufacturer::RecordManufacturer,
                tape::model_tape::{RecordTape, TapeFormat},
                tape_type::model_tape_type::RecordTapeType,
            },
            select_option::{SelectOption, vec_into},
        },
    },
};

#[component]
pub fn TapeForm(
    id: i64,
    types: Vec<RecordTapeType>,
    manufacturers: Vec<RecordManufacturer>,
) -> Element {
    let mut message = use_signal(|| MessageDetails::default());
    let mut tape: Signal<RecordTape> = use_signal(|| RecordTape::default());

    let mut barcode_err = use_signal(move || None);
    let mut serial_err = use_signal(move || None);

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

    let form_invalid = use_resource(move || async move {
        let barcode_validator = Validator::<String>::create()
            .with_only_allowed_chars(Const::BARCODE_VALID_CHARS)
            .with_expected_length(Const::CODE_39_LTO_USABLE_LEN);

        // Validate form
        barcode_err.set(barcode_validator.validate(&tape().barcode));
        if tape().barcode.len() == Const::CODE_39_LTO_USABLE_LEN {
            match api_tape_barcode_exists(tape().barcode).await {
                Ok(exists) => {
                    if exists {
                        barcode_err.set(Some("Barcode already exists".to_string()));
                    }
                }
                Err(e) => message.set(MessageDetails {
                    level: Level::Error,
                    text: format!("Barcode exists check error: {}", e),
                }),
            }
        }
        if let Some(serial) = tape().serial {
            match api_tape_serial_exists(serial).await {
                Ok(exists) => {
                    if exists {
                        serial_err.set(Some("Serial number already exists".to_string()));
                    } else {
                        serial_err.set(None);
                    }
                }
                Err(e) => message.set(MessageDetails {
                    level: Level::Error,
                    text: format!("Serial number exists check error: {}", e),
                }),
            }
        } else {
            serial_err.set(None);
        }

        barcode_err().is_some()
            || serial_err().is_some()
            || tape().tape_type_id == 0
            || tape().manufacturer_id == 0
    });

    let submit = move |_event: Event<MouseData>| async move {
        match api_add_tape(tape()).await {
            Ok(_) => {
                message.set(MessageDetails {
                    level: Level::Success,
                    text: "Added new record".to_string(),
                });
            }
            Err(e) => {
                message.set(MessageDetails {
                    level: Level::Error,
                    text: format!("Insert error: {}", e),
                });
            }
        }
    };

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
                    label: "Manufacturer".to_string(),
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
                    oninput: move |evt: Event<FormData>| async move {
                        tape.write().barcode = evt.value().to_uppercase();
                    },
                    validation: barcode_err,
                    value: tape().barcode,
                    maxlength: Const::CODE_39_LTO_USABLE_LEN,
                    meta: barcode_designation.clone(),
                }
                Input {
                    type_: InputType::Text,
                    label: "Serial Number".to_string(),
                    oninput: move |evt: Event<FormData>| {
                        tape.write().serial = either!(evt.value().is_empty(), None, Some(evt.value()));
                    },
                    validation: serial_err,
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
                hr {}
                Button {
                    r#type: "button",
                    primary: true,
                    disabled: form_invalid.read().unwrap_or(true),
                    onclick: submit,
                    text: either!(id == 0, "Add", "Update"),
                }
                Message { details: message() }
            }
            VerticalDivider {}
            // Preview, div is needed
            div {
                TapePreview {
                    preview: tape(),
                    manufacturers,
                    tapes_list: types,
                    designation: barcode_designation,
                    size: "30",
                }
            }
        }
        Debug { tape, form_invalid: form_invalid.read().unwrap_or(true) }
    }
}

#[cfg(debug_assertions)]
#[component]
fn Debug(tape: Signal<RecordTape>, form_invalid: bool) -> Element {
    use crate::frontend::{
        components::header::{header_extra::HeaderExtraIcons, header_icon::HeaderIcon},
        icons::Icons,
    };
    rsx! {
        HeaderExtraIcons {
            HeaderIcon { button_id: "dbg_btn", menu_id: "dbg_menu", icon: Icons::BUG,
                div { class: Css::DEBUG_MENU, style: "width:20rem",
                    h2 { "Debug" }
                    p { "{tape():?}" }
                    hr {}
                    p { "form_invalid: {form_invalid}" }
                    hr {}
                    p { "barcode_len: {tape().barcode.len()}" }
                }
            }
        }
    }
}

#[cfg(not(debug_assertions))]
#[component]
fn Debug(tape: Signal<RecordTape>, form_invalid: bool) -> Element {
    rsx! {} // Nothing for release build
}
