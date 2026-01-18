use dioxus::{fullstack::Loader, prelude::*};

use crate::backend::api::api_manufacturer::list_manu;
use crate::backend::database::models::model_manufacturer::RecordManufacturer;

/// Home page
#[component]
pub fn Show() -> Element {
    //let mut tapes_list = use_loader(list_tapes)?;
    let mut bar = use_signal(|| "".to_string());

    let manu_list: Loader<Vec<RecordManufacturer>> = use_loader(list_manu)?;

    rsx! {
        p { "list" }
        input { oninput: move |e| bar.set(e.value()) }
        /*button {
            id: "save",
            onclick: move |_| async move {
                _ = save_tape(Tape {
                        id: 0,
                        barcode: bar.to_string(),
                        worm: false,
                    })
                    .await;
                tapes_list.restart();
            },
            "save!"
        }*/
        br {}
        br {}
        table {
            tr {
                th { "id" }
                th { "barcode" }
                th { "worm" }
            }
           /* for (id , t) in tapes_list.cloned() {
                tr {
                    th { "{id}" }
                    th { "{t.barcode}" }
                    th { "{t.worm}" }
                }
            }*/
        }
        hr {}
        table {
            tr {
                th { "id" }
                th { "name" }
            }
            for t in manu_list.cloned() {
                tr {
                    th { "{t.id}" }
                    th { "{t.name}" }
                }
            }
        }
    }
}
