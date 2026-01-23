use dioxus::{fullstack::Loader, prelude::*};

use crate::backend::{api::api_tape::api_get_tape, database::models::model_tape::RecordTape};

#[component]
pub fn Tape(id: i64) -> Element {
    let tape_loader: Loader<RecordTape> = use_loader(|| api_get_tape(0))?;
    let mut tape = use_signal(|| tape_loader());

    let input_evt = move |evt: Event<FormData>| {
        tape.write().barcode = evt.value().to_uppercase();
    };

    rsx! {
        if tape().id == 0 {
        p { "New Tape" }
        } else {
        p { "Edit Tape" }
        }
        form {
            p { "{tape().barcode}" }
            input {
                id: "barcode",
                r#type: "search",
                maxlength: 6,
                oninput: input_evt,
                value: "{tape().barcode}"
            }

        }
        button {
            r#type: "button",
            onclick: move |_e: Event<MouseData>| {
                tracing::info!("Hello");
            },
            "test"
        }
    }
}
