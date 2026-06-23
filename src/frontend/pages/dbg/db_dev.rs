use dioxus::prelude::*;

use crate::{
    backend::api::api_dev::{dev_db_backup, dev_db_list, dev_db_restore},
    frontend::elements::button::Button,
};

#[component]
pub fn ShowDev() -> Element {
    let found_dir = use_loader(dev_db_list)?;
    let mut dir = use_signal(|| "1".to_string());
    let mut message_ok = use_signal(|| "".to_string());
    let mut message_err = use_signal(|| "".to_string());

    rsx! {
        span { "Found: " }
        for dir in found_dir.cloned() {
            span { "[{dir}] " }
        }
        hr {}

        label { "Index: " }
        input {
            oninput: move |evt: Event<FormData>| {
                dir.set(evt.value());
            },
            value: dir(),
        }
        br {}
        br {}

        Button {
            onclick: move |_| async move {
                match dev_db_backup(dir()).await {
                    Ok(result) => {
                        if result {
                            message_ok.set("BACKUP OK".to_string());
                            message_err.set("".to_string());
                        } else {
                            message_err.set("BACKUP Error".to_string());
                            message_ok.set("".to_string());
                        }
                    }
                    Err(e) => {
                        message_err.set(format!("BACKUP Error: {}", e));
                        message_ok.set("".to_string());
                    }
                }
            },
            text: "Backup",
        }
        Button {
            style: "float:right",
            onclick: move |_evt: MouseEvent| async move {
                match dev_db_restore(dir()).await {
                    Ok(result) => {
                        if result {
                            message_ok.set("RESTORE OK".to_string());
                            message_err.set("".to_string());
                        } else {
                            message_err.set("RESTORE Error".to_string());
                            message_ok.set("".to_string());
                        }
                    }
                    Err(e) => {
                        message_err.set(format!("RESTORE Error: {}", e));
                        message_ok.set("".to_string());
                    }
                }
            },
            text: "Restore",
        }
        if !message_ok().is_empty() {
            p { style: "color:green", "{message_ok}" }
        }
        if !message_err().is_empty() {
            p { style: "color:red", "{message_err}" }
        }
    }
}
