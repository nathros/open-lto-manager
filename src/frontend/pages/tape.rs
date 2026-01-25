use dioxus::{fullstack::Loader, prelude::*};

use crate::backend::{
    api::{
        api_manufacturer::list_manu,
        api_tape::{api_add_tape, api_get_tape},
        api_tape_type::list_type_type,
    },
    database::models::{
        model_manufacturer::RecordManufacturer,
        model_tape::{RecordTape, TapeFormat},
        model_tape_type::RecordTapeType,
    },
};

#[component]
pub fn Tape(id: i64) -> Element {
    let tape_loader: Loader<RecordTape> = use_loader(|| api_get_tape(0))?;
    let mut tape: Signal<RecordTape> = use_signal(|| tape_loader());

    let manufactures: Loader<Vec<RecordManufacturer>> = use_loader(list_manu)?;
    let types: Loader<Vec<RecordTapeType>> = use_loader(list_type_type)?;

    let current_designation = types()
        .iter()
        .find(|p| p.id == tape().tape_type_id)
        .unwrap_or(&RecordTapeType::blank())
        .clone();

    let barcode_designation = if tape().worm {
        current_designation.id_reg.clone()
    } else {
        current_designation.id_worm.clone()
    };

    let input_evt = move |evt: Event<FormData>| {
        tape.write().barcode = evt.value().to_uppercase();
    };

    let input_serial = move |evt: Event<FormData>| {
        tape.write().serial = evt.value();
    };

    let input_worm = move |evt: Event<FormData>| {
        tape.write().worm = evt.value() == "true";
    };

    let input_enc = move |evt: Event<FormData>| {
        tape.write().encrypted = evt.value() == "true";
    };

    let input_compression = move |evt: Event<FormData>| {
        tape.write().compressed = evt.value() == "true";
    };

    let input_format = move |evt: Event<FormData>| {
        let format_index = evt.value().parse::<i64>().unwrap_or(0);
        tape.write().format = TapeFormat::from(format_index);
    };

    let type_change = move |evt: Event<FormData>| {
        let selected_index = evt.value().parse::<i64>().unwrap_or(0);
        tape.write().tape_type_id = selected_index;
    };

    let manu_change = move |evt: Event<FormData>| {
        let selected_index = evt.value().parse::<i64>().unwrap_or(0);
        tape.write().manufacturer_id = selected_index;
    };

    let mut error_msg: Signal<String> = use_signal(|| "".to_string());
    let mut ok_msg: Signal<String> = use_signal(|| "".to_string());

    let submit = move |_| async move {
        match api_add_tape(tape()).await {
            Ok(_) => {
                ok_msg.set("Added new record".to_string());
                error_msg.set("".to_string());
            }
            Err(e) => {
                error_msg.set(format!("Insert error: {}", e));
                ok_msg.set("".to_string());
            }
        }
        tracing::info!("Hello");
    };

    rsx! {
        if tape().id == 0 {
            b { "New Tape" }
        } else {
            b { "Edit Tape" }
        }
        div {
            label { "Type:" }
            br {  }
            select {
                onchange: type_change,
                if tape().tape_type_id == 0 {
                    option {
                        disabled: true,
                        selected: true,
                        "Select"
                    }
                }
                for ty in types.cloned() {
                    option {
                        value : "{ty.id}",
                        "{ty.generation}"
                    }
                }
             }
            br {  }
            br {  }

            label { "Manufacturer:" }
            br {  }
            select {
                onchange: manu_change,
                if tape().manufacturer_id == 0 {
                    option {
                        disabled: true,
                        selected: true,
                        "Select"
                    }
                }
                for man in manufactures.cloned() {
                    option {
                        value : "{man.id}",
                        "{man.name}"
                    }
                }
             }
            br {  }
            br {  }

            label { "Format:" }
            br {  }
            input {
                id: "format1",
                name: "format",
                r#type: "radio",
                oninput: input_format,
                checked: tape().format == TapeFormat::Tar,
                value: "{<TapeFormat as Into<i64>>::into(TapeFormat::Tar)}"
            }
            label {
                r#for: "format1",
                "{<TapeFormat as Into<&str>>::into(TapeFormat::Tar)}"
            }
            br {  }
            input {
                id: "format2",
                name: "format",
                r#type: "radio",
                oninput: input_format,
                checked: tape().format == TapeFormat::LTFS,
                value: "{<TapeFormat as Into<i64>>::into(TapeFormat::LTFS)}"
            }
            label {
                r#for: "format2",
                "{<TapeFormat as Into<&str>>::into(TapeFormat::LTFS)}"
            }
            br {  }
            br {  }

            label { "Serial:" }
            br {}
            input {
                id: "serial",
                r#type: "search",
                maxlength: 32,
                oninput: input_serial,
                value: "{tape().serial}"
            }
            br {  }
            br {  }

            label { "Barcode:" }
            br {}
            input {
                id: "barcode",
                r#type: "search",
                maxlength: 6,
                oninput: input_evt,
                value: "{tape().barcode}"
            }
            input {
                disabled: true,
                value: "{barcode_designation}"
            }
            br {  }
            br {  }

            label {
                r#for: "worm",
                "WORM:"
            }
            br {}
            input {
                id: "worm",
                r#type: "checkbox",
                oninput: input_worm,
                checked: tape().worm,
            }
            br {  }
            br {  }

            label {
                r#for: "enc",
                "Encrypted:"
            }
            br {}
            input {
                id: "enc",
                r#type: "checkbox",
                oninput: input_enc,
                checked: tape().encrypted,
            }
            br {  }
            br {  }

            label {
                r#for: "compressed",
                "Compression Enabled:"
            }
            br {}
            input {
                id: "compressed",
                r#type: "checkbox",
                oninput: input_compression,
                checked: tape().compressed,
            }
            br {  }
            br {  }

            if !error_msg().is_empty() {
                p {
                    style: "background-color: red; font:weight:bold;",
                    "{error_msg()}"
                }
            }

            if !ok_msg().is_empty() {
                p {
                    style: "background-color: green; font:weight:bold;",
                    "{ok_msg()}"
                }
            }

            p { "Debug: {tape():?}]" }

        }
        button {
            r#type: "button",
            onclick: submit,
            "Submit"
        }
    }
}
