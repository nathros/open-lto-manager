use std::mem::discriminant;

use dioxus::prelude::*;

use crate::{
    backend::api::api_login::api_login,
    frontend::{
        assets::{APP_NAME, FAVICON},
        collections::message::{Message, MessageDetails},
        css::Css,
        elements::{
            button::Button,
            input::{Input, InputType},
        },
        level::Level,
    },
    route::Route,
    static_concat,
};

#[component]
pub fn LoginUser(#[props(default)] success_signal: Callback) -> Element {
    let mut username = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut result = use_signal(|| MessageDetails::default());

    rsx! {
        div { class: static_concat!(Css::LOGIN_CONTAINER, Css::FLEX_CENTRE),
            section {
                div {
                    div { class: static_concat!(Css::FLEX_COL, Css::FLEX_ALIGN_CENTRE),
                        img { src: FAVICON }
                        h3 { "{APP_NAME}" }
                    }
                }
                hr {}
                div {
                    div { class: Css::FLEX_COL,
                        label { "Username: " }
                        Input {
                            type_: InputType::Text,
                            value: username(),
                            oninput: move |evt: Event<FormData>| {
                                username.set(evt.value());
                            },
                        }
                        label { "Password: " }
                        Input {
                            type_: InputType::Password,
                            value: password(),
                            oninput: move |evt: Event<FormData>| {
                                password.set(evt.value());
                            },
                        }
                        Button {
                            primary: true,
                            text: "Login",
                            onclick: move |_evt: MouseEvent| async move {
                                match api_login(username(), password()).await {
                                    Ok(_o) => {
                                        success_signal(());
                                        if discriminant(&use_route::<Route>())
                                            == discriminant(&Route::LoginUser {})
                                        {
                                            use_navigator().push(Route::Home {});
                                        }
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
                        Message { details: result() }
                    }
                }
            }
        }
    }
}
