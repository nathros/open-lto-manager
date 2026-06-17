use dioxus::prelude::*;

use crate::{
    backend::api::api_login::api_login,
    frontend::{
        collections::message::{Message, MessageDetails},
        elements::{
            button::Button,
            input::{Input, InputType},
        },
        level::Level,
    },
};

#[component]
pub fn LoginUser(#[props(default)] success_signal: Callback) -> Element {
    let mut username = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut result = use_signal(|| MessageDetails::default());

    rsx! {
        Message { details: result() }
        div {
            label { "Username: " }
            Input {
                type_: InputType::Text,
                value: username(),
                oninput: move |evt: Event<FormData>| {
                    username.set(evt.value());
                },
            }
            br {}
            label { "Password: " }
            Input {
                type_: InputType::Password,
                value: password(),
                oninput: move |evt: Event<FormData>| {
                    password.set(evt.value());
                },
            }
            br {}
            Button {
                primary: true,
                text: "Login",
                onclick: move |_| async move {
                    match api_login(username(), password()).await {
                        Ok(_o) => {
                            result
                                .set(MessageDetails {
                                    level: Level::Success,
                                    text: "ok".to_string(),
                                });
                            success_signal(());
                        }
                        Err(e) => {
                            result
                                .set(MessageDetails {
                                    level: Level::Error,
                                    text: format!("{}", e),
                                });
                        }
                    }
                },
            }
        }
    }
}
