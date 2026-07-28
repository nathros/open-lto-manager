use dioxus::{fullstack::FullstackContext, prelude::*};

#[cfg(all(feature = "auto_login", debug_assertions))]
use crate::backend::api::api_login::api_login_bypass;

use crate::{
    Route,
    backend::api::{api_login::api_current_user, api_logout::api_logout, api_user::update_user},
    either,
    frontend::{
        assets::{
            APP_NAME, ICONS_ASSET_ICONOIR, ICONS_ASSET_REMIX, ICONS_ASSET_SARGAM_LINE,
            ICONS_ASSET_TABLER, LOGO_ASSET,
        },
        collections::message::{Message, MessageDetails},
        components::{
            main_body::MainBody,
            menu::{component::Menu, menu_config::MenuConfig},
        },
        css::Css,
        elements::input::InputType,
        icons::Icons,
        id::Id,
        js::js_hide_popover,
        level::Level,
        pages::login::login_user::LoginUser,
    },
    shared::models::database::user::model_user::{
        ACCENT_BLUE, ACCENT_GREEN, ACCENT_RED, ACCENT_STANDARD, ColourMode, IconTheme,
    },
    static_concat,
};

fn error_handler(err: ErrorContext) -> Element {
    let mut msg = MessageDetails::default();
    if let Some(e) = err.error() {
        let http_error = FullstackContext::commit_error_status(e);
        match http_error.status {
            StatusCode::NOT_FOUND => msg.text = "404 - Page not found".to_string(),
            StatusCode::UNAUTHORIZED => {
                msg.text = "401 - Unauthorized".to_string();
                msg.level = Level::Warning;
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                msg.text = "500 - Internal Server Error".to_string();
            }
            _ => msg.text = "An unknown error occurred".to_string(),
        }
    }
    //let cloned_errors = err.clone(); // Clear on load
    //use_effect(move || {
    //    cloned_errors.clear_errors();
    //    error!("clear");
    //});
    rsx! {
        Message { details: msg }
        p { "-- Refresh needed --" }
        button {
            onclick: move |_| {
                err.clear_errors();
            },
            "Retry"
        }
    }
}

#[component]
pub fn Header() -> Element {
    let mut current_user = use_loader(api_current_user)?;

    #[cfg(all(feature = "auto_login", debug_assertions))]
    {
        let mut skip = use_signal(|| false);
        if !skip() && current_user().is_none() {
            spawn(async move {
                // TODO fix called twice
                if api_login_bypass().await.is_ok() {
                    skip.set(true);
                    current_user.restart();
                }
            });
        }
    }

    let change_theme = move |theme: ColourMode| async move {
        if let Some(mut user) = current_user() {
            user.system_theme = theme;
            if update_user(user.clone()).await.is_ok() {
                current_user.set(Some(user));
            }
        }
    };
    let change_accent = move |colour: String| async move {
        if let Some(mut user) = current_user() {
            user.accent_colour = colour.clone().to_string();
            if update_user(user.clone()).await.is_ok() {
                current_user.set(Some(user));
            }
        }
    };
    let change_icon = move |icon: IconTheme| async move {
        if let Some(mut user) = current_user() {
            user.icon_theme = icon;
            if update_user(user.clone()).await.is_ok() {
                current_user.set(Some(user));
            }
        }
    };

    let route = use_route::<Route>();
    let route_str = route.to_string();
    let mut last_route_str = use_signal(|| route.to_string());
    let mut aside_config = use_signal(|| MenuConfig::default_aside(&route_str));
    if last_route_str() != route_str {
        last_route_str.set(route_str.clone());
        aside_config.set(MenuConfig::default_aside(&route_str)); // TODO fix Menu is rendered twice on route change
    }

    rsx! {
        if let user = current_user().unwrap_or_default() {
            MainBody { user: user.clone(),
                if current_user().is_some() {
                    aside { class: Css::MAIN_ASIDE,
                        Menu { config: aside_config, current_route: route_str }
                    }
                    header { class: Css::MAIN_HEADER,
                        div { class: Css::MAIN_HEADER_LOGO, "{APP_NAME}" }

                        button { id: Id::HeaderBaseAnchor.as_str() }

                        button {
                            class: Css::HEADER_DROPDOWN,
                            id: Id::HeaderNotificationIcon.as_str(),
                            "popovertarget": Id::HeaderNotificationMenu.as_str(),
                            span { class: static_concat!(Css::ICON, Icons::NOTIFICATION) }
                        }
                        div {
                            id: Id::HeaderNotificationMenu.as_str(),
                            "anchor": Id::HeaderNotificationIcon.as_str(),
                            class: Css::HEADER_DROPDOWN_CONTENT,
                            popover: "auto",
                            div { class: static_concat!(Css::ICON_LIST_ITEM, Css::FLEX_ROW, Css::FLEX_ALIGN_CENTRE),
                                "..."
                            }
                        }

                        button {
                            class: Css::HEADER_DROPDOWN,
                            id: Id::HeaderInfoIcon.as_str(),
                            "popovertarget": Id::HeaderInfoMenu.as_str(),
                            span { class: static_concat!(Css::ICON, Icons::INFO) }
                        }
                        div {
                            id: Id::HeaderInfoMenu.as_str(),
                            "anchor": Id::HeaderInfoIcon.as_str(),
                            class: Css::HEADER_DROPDOWN_CONTENT,
                            popover: "auto",
                            div {
                                class: static_concat!(Css::ICON_LIST_ITEM, Css::FLEX_ROW, Css::FLEX_ALIGN_CENTRE),
                                onclick: move |_evt: Event<MouseData>| async move {
                                    js_hide_popover(Id::HeaderInfoMenu.as_str());
                                    let _ = document::eval("alert('Not ready yet');").await;
                                },
                                span { class: static_concat!(Css::ICON, Css::SM, Icons::BOOK) }
                                span { "Docs" }
                            }
                            Link {
                                "target": "_blank",
                                class: static_concat!(Css::ICON_LIST_ITEM, Css::FLEX_ROW, Css::FLEX_ALIGN_CENTRE),
                                to: "https://github.com/nathros/open-lto-manager",
                                onclick: move |_evt: Event<MouseData>| {
                                    js_hide_popover(Id::HeaderInfoMenu.as_str());
                                },
                                span {
                                    style: format!("mask-image:url({}#github)", LOGO_ASSET),
                                    class: static_concat!(Css::ICON, Css::SM),
                                }
                                span { "GitHub" }
                            }
                            Link {
                                "target": "_blank",
                                class: static_concat!(Css::ICON_LIST_ITEM, Css::FLEX_ROW, Css::FLEX_ALIGN_CENTRE),
                                to: "https://github.com/nathros/open-lto-manager/issues",
                                onclick: move |_evt: Event<MouseData>| {
                                    js_hide_popover(Id::HeaderInfoMenu.as_str());
                                },
                                span { class: static_concat!(Css::ICON, Css::SM, Icons::BUG) }
                                span { "Bug report" }
                            }
                        }

                        button {
                            class: Css::HEADER_DROPDOWN,
                            id: Id::HeaderUserIcon.as_str(),
                            "popovertarget": Id::HeaderUserMenu.as_str(),
                            span { class: static_concat!(Css::ICON, Icons::USER) }
                        }
                        div {
                            id: Id::HeaderUserMenu.as_str(),
                            "anchor": Id::HeaderUserIcon.as_str(),
                            class: Css::HEADER_DROPDOWN_CONTENT,
                            popover: "auto",
                            div { class: static_concat!(Css::ICON_LIST_ITEM, Css::HEADER_USER),
                                "{user.username}"
                            }
                            Link {
                                class: static_concat!(Css::ICON_LIST_ITEM, Css::FLEX_ROW, Css::FLEX_ALIGN_CENTRE),
                                to: Route::DBUser {},
                                onclick: move |_evt: Event<MouseData>| {
                                    js_hide_popover(Id::HeaderUserMenu.as_str());
                                },
                                span { class: static_concat!(Css::ICON, Css::SM, Icons::USER) }
                                span { "Account" }
                            }

                            button {
                                class: static_concat!(Css::ICON_LIST_ITEM, Css::FLEX_ROW, Css::FLEX_ALIGN_CENTRE),
                                id: Id::HeaderAccentIcon.as_str(),
                                "popovertarget": Id::HeaderAccentMenu.as_str(),
                                span { class: static_concat!(Css::ICON, Css::SM, Icons::PALETTE) }
                                span { "Accent" }
                                span { class: static_concat!(Css::ICON, Css::SM, Icons::CHEVRON_RIGHT, Css::FLOAT_RIGHT) }
                            }
                            div {
                                id: Id::HeaderAccentMenu.as_str(),
                                "anchor": Id::HeaderAccentIcon.as_str(),
                                class: [Css::HEADER_DROPDOWN_CONTENT, Css::HEADER_DROPDOWN_NESTED].concat(),
                                popover: "auto",
                                div {
                                    class: [
                                        Css::ICON_LIST_ITEM,
                                        Css::FLEX_ROW,
                                        Css::FLEX_ALIGN_CENTRE,
                                        either!(user.accent_colour == ACCENT_STANDARD, Css::SELECTED, ""),
                                    ]
                                        .concat(),
                                    onclick: move |_evt: Event<MouseData>| async move {
                                        change_accent(ACCENT_STANDARD.into()).await;
                                    },
                                    span {
                                        style: static_concat!("background-color:", ACCENT_STANDARD),
                                        class: static_concat!(Css::ICON, Css::SM, Css::HEADER_COL),
                                    }
                                    span { "Standard" }
                                }
                                div {
                                    class: [
                                        Css::ICON_LIST_ITEM,
                                        Css::FLEX_ROW,
                                        Css::FLEX_ALIGN_CENTRE,
                                        either!(user.accent_colour == ACCENT_RED, Css::SELECTED, ""),
                                    ]
                                        .concat(),
                                    onclick: move |_evt: Event<MouseData>| async move {
                                        change_accent(ACCENT_RED.into()).await;
                                    },
                                    span {
                                        style: static_concat!("background-color:", ACCENT_RED),
                                        class: static_concat!(Css::ICON, Css::SM, Css::HEADER_COL),
                                    }
                                    span { "Red" }
                                }
                                div {
                                    class: [
                                        Css::ICON_LIST_ITEM,
                                        Css::FLEX_ROW,
                                        Css::FLEX_ALIGN_CENTRE,
                                        either!(user.accent_colour == ACCENT_GREEN, Css::SELECTED, ""),
                                    ]
                                        .concat(),
                                    onclick: move |_evt: Event<MouseData>| async move {
                                        change_accent(ACCENT_GREEN.into()).await;
                                    },
                                    span {
                                        style: static_concat!("background-color:", ACCENT_GREEN),
                                        class: static_concat!(Css::ICON, Css::SM, Css::HEADER_COL),
                                    }
                                    span { "Green" }
                                }
                                div {
                                    class: [
                                        Css::ICON_LIST_ITEM,
                                        Css::FLEX_ROW,
                                        Css::FLEX_ALIGN_CENTRE,
                                        either!(user.accent_colour == ACCENT_BLUE, Css::SELECTED, ""),
                                    ]
                                        .concat(),
                                    onclick: move |_evt: Event<MouseData>| async move {
                                        change_accent(ACCENT_BLUE.into()).await;
                                    },
                                    span {
                                        style: static_concat!("background-color:", ACCENT_BLUE),
                                        class: static_concat!(Css::ICON, Css::SM, Css::HEADER_COL),
                                    }
                                    span { "Blue" }
                                }
                                label {
                                    class: [
                                        Css::ICON_LIST_ITEM,
                                        Css::FLEX_ROW,
                                        Css::FLEX_ALIGN_CENTRE,
                                        either!(
                                            user.accent_colour == ACCENT_STANDARD || user.accent_colour == ACCENT_RED ||
                                            user.accent_colour == ACCENT_GREEN || user.accent_colour == ACCENT_BLUE, "",
                                            Css::SELECTED
                                        ),
                                    ]
                                        .concat(),
                                    r#for: Css::ID_ACCENT_PICKER,
                                    span { class: static_concat!(Css::ICON, Css::SM, Css::HEADER_COL, Css::RAINBOW) }
                                    span { "Custom" }
                                    input {
                                        id: Css::ID_ACCENT_PICKER,
                                        r#type: InputType::Colour.to_string(),
                                        oninput: move |evt| async move {
                                            evt.stop_propagation();
                                            change_accent(evt.value()).await;
                                        },
                                        value: user.accent_colour.clone(),
                                    }
                                }
                            }

                            button {
                                class: static_concat!(Css::ICON_LIST_ITEM, Css::FLEX_ROW, Css::FLEX_ALIGN_CENTRE),
                                id: Id::HeaderIconIcon.as_str(),
                                "popovertarget": Id::HeaderIconMenu.as_str(),
                                span { class: static_concat!(Css::ICON, Css::SM, Icons::SANDPIT) }
                                span { "Icons" }
                                span { class: static_concat!(Css::ICON, Css::SM, Icons::CHEVRON_RIGHT, Css::FLOAT_RIGHT) }
                            }
                            div {
                                id: Id::HeaderIconMenu.as_str(),
                                "anchor": Id::HeaderIconIcon.as_str(),
                                class: [Css::HEADER_DROPDOWN_CONTENT, Css::HEADER_DROPDOWN_NESTED].concat(),
                                popover: "auto",
                                div {
                                    class: [
                                        Css::ICON_LIST_ITEM,
                                        Css::FLEX_ROW,
                                        Css::FLEX_ALIGN_CENTRE,
                                        either!(user.icon_theme == IconTheme::Tabler, Css::SELECTED, ""),
                                    ]
                                        .concat(),
                                    onclick: move |_evt: Event<MouseData>| async move {
                                        change_icon(IconTheme::Tabler).await;
                                    },
                                    span {
                                        style: format!("mask-image:url({}#sandpit)", ICONS_ASSET_TABLER),
                                        class: static_concat!(Css::ICON, Css::SM, Css::HEADER_COL),
                                    }
                                    span { "Tabler" }
                                }
                                div {
                                    class: [
                                        Css::ICON_LIST_ITEM,
                                        Css::FLEX_ROW,
                                        Css::FLEX_ALIGN_CENTRE,
                                        either!(user.icon_theme == IconTheme::Remix, Css::SELECTED, ""),
                                    ]
                                        .concat(),
                                    onclick: move |_evt: Event<MouseData>| async move {
                                        change_icon(IconTheme::Remix).await;
                                    },
                                    span {
                                        style: format!("mask-image:url({}#sandpit)", ICONS_ASSET_REMIX),
                                        class: static_concat!(Css::ICON, Css::SM, Css::HEADER_COL),
                                    }
                                    span { "Remix" }
                                }
                                div {
                                    class: [
                                        Css::ICON_LIST_ITEM,
                                        Css::FLEX_ROW,
                                        Css::FLEX_ALIGN_CENTRE,
                                        either!(user.icon_theme == IconTheme::Iconoir, Css::SELECTED, ""),
                                    ]
                                        .concat(),
                                    onclick: move |_evt: Event<MouseData>| async move {
                                        change_icon(IconTheme::Iconoir).await;
                                    },
                                    span {
                                        style: format!("mask-image:url({}#sandpit)", ICONS_ASSET_ICONOIR),
                                        class: static_concat!(Css::ICON, Css::SM, Css::HEADER_COL),
                                    }
                                    span { "Iconoir" }
                                }
                                div {
                                    class: [
                                        Css::ICON_LIST_ITEM,
                                        Css::FLEX_ROW,
                                        Css::FLEX_ALIGN_CENTRE,
                                        either!(user.icon_theme == IconTheme::Sargam, Css::SELECTED, ""),
                                    ]
                                        .concat(),
                                    onclick: move |_evt: Event<MouseData>| async move {
                                        change_icon(IconTheme::Sargam).await;
                                    },
                                    span {
                                        style: format!("mask-image:url({}#sandpit)", ICONS_ASSET_SARGAM_LINE),
                                        class: static_concat!(Css::ICON, Css::SM, Css::HEADER_COL),
                                    }
                                    span { "Sargam" }
                                }
                            }

                            button {
                                class: static_concat!(Css::ICON_LIST_ITEM, Css::FLEX_ROW, Css::FLEX_ALIGN_CENTRE),
                                id: Id::HeaderThemeIcon.as_str(),
                                "popovertarget": Id::HeaderThemeMenu.as_str(),
                                span { class: static_concat!(Css::ICON, Css::SM, Icons::CONTRAST) }
                                span { "Theme" }
                                span { class: static_concat!(Css::ICON, Css::SM, Icons::CHEVRON_RIGHT, Css::FLOAT_RIGHT) }
                            }
                            div {
                                id: Id::HeaderThemeMenu.as_str(),
                                "anchor": Id::HeaderThemeIcon.as_str(),
                                class: [Css::HEADER_DROPDOWN_CONTENT, Css::HEADER_DROPDOWN_NESTED].concat(),
                                popover: "auto",
                                div {
                                    class: [
                                        Css::ICON_LIST_ITEM,
                                        Css::FLEX_ROW,
                                        Css::FLEX_ALIGN_CENTRE,
                                        either!(user.system_theme == ColourMode::System, Css::SELECTED, ""),
                                    ]
                                        .concat(),
                                    onclick: move |_evt: Event<MouseData>| async move {
                                        change_theme(ColourMode::System).await;
                                    },
                                    span { class: static_concat!(Css::ICON, Css::SM, Icons::SYSTEM) }
                                    span { "System" }
                                }
                                div {
                                    class: [
                                        Css::ICON_LIST_ITEM,
                                        Css::FLEX_ROW,
                                        Css::FLEX_ALIGN_CENTRE,
                                        either!(user.system_theme == ColourMode::Light, Css::SELECTED, ""),
                                    ]
                                        .concat(),
                                    onclick: move |_evt: Event<MouseData>| async move {
                                        change_theme(ColourMode::Light).await;
                                    },
                                    span { class: static_concat!(Css::ICON, Css::SM, Icons::LIGHT) }
                                    span { "Light" }
                                }
                                div {
                                    class: [
                                        Css::ICON_LIST_ITEM,
                                        Css::FLEX_ROW,
                                        Css::FLEX_ALIGN_CENTRE,
                                        either!(user.system_theme == ColourMode::Dark, Css::SELECTED, ""),
                                    ]
                                        .concat(),
                                    onclick: move |_evt: Event<MouseData>| async move {
                                        change_theme(ColourMode::Dark).await;
                                    },
                                    span { class: static_concat!(Css::ICON, Css::SM, Icons::DARK) }
                                    span { "Dark" }
                                }
                            }

                            hr {}

                            Link {
                                class: static_concat!(Css::ICON_LIST_ITEM, Css::FLEX_ROW, Css::FLEX_ALIGN_CENTRE),
                                onclick: |_| async {
                                    let _ = api_logout().await;
                                },
                                to: Route::LoginUser {},
                                span { class: static_concat!(Css::ICON, Css::SM, Icons::LOGOUT) }
                                span { "Log out" }
                            }
                        }
                    }

                    ErrorBoundary { handle_error: error_handler, Outlet::<Route> {} }
                } else {
                    ErrorBoundary { handle_error: error_handler,
                        LoginUser {
                            success_signal: move |_| async move {
                                current_user.restart();
                            },
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn HeaderExtraIcons(children: Element) -> Element {
    rsx! {
        div { class: Css::HEADER_ANCHOR_POSITION, {children} }
    }
}
