use dioxus::{fullstack::FullstackContext, prelude::*};
use std::mem::discriminant;

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
            colour_mode::ColourModeHidden,
            menu::{Menu, MenuConfig, MenuGroup, MenuItemConfig},
        },
        css::Css,
        elements::input::InputType,
        icons::Icons,
        level::Level,
        pages::login::login_user::LoginUser,
    },
    shared::models::database::user::model_user::{
        ACCENT_BLUE, ACCENT_GREEN, ACCENT_RED, ACCENT_STANDARD, ColourMode, IconTheme,
    },
    static_concat,
};

fn route_eq<T>(a: &T, b: &T) -> bool {
    discriminant(a) == discriminant(b)
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

    let route: Route = use_route();

    let error_handler = move |err: ErrorContext| {
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
    };

    let mut show_notifications = use_signal(|| false);

    let mut show_info = use_signal(|| false);

    let mut show_menu = use_signal(|| false);
    let mut show_theme = use_signal(|| false);
    let mut show_accent = use_signal(|| false);
    let mut show_icon = use_signal(|| false);
    let mut close = move |evt: Event<MouseData>| {
        evt.stop_propagation();
        show_menu.set(false);
        show_theme.set(false);
        show_accent.set(false);
        show_icon.set(false);
        show_info.set(false);
        show_notifications.set(false); // FIXME not always close when switch
    };
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

    let aside_config = use_signal(|| MenuConfig {
        enable_search: true,
        groups: vec![
            MenuGroup {
                icon: Icons::HOME.into(),
                label: "Home".to_string(),
                open: route_eq(&route, &Route::Home {}),
                items: vec![MenuItemConfig {
                    icon: "".to_string(),
                    label: "Home".to_string(),
                    link: Route::Home {}.to_string(),
                    selected: route_eq(&route, &Route::Home {}),
                }],
            },
            MenuGroup {
                icon: Css::ICON_TAPE.into(),
                label: "Library".to_string(),
                open: route_eq(&route, &Route::Tape { id: (0) }),
                items: vec![MenuItemConfig {
                    icon: "".to_string(),
                    label: "Add Tape".to_string(),
                    link: Route::Tape { id: (0) }.to_string(),
                    selected: route_eq(&route, &Route::Tape { id: (0) }),
                }],
            },
            MenuGroup {
                icon: Icons::LIST.into(),
                label: "Jobs".to_string(),
                open: route_eq(&route, &Route::AddJob {}),
                items: vec![MenuItemConfig {
                    icon: "".to_string(),
                    label: "Add Job".to_string(),
                    link: Route::AddJob {}.to_string(),
                    selected: route_eq(&route, &Route::AddJob {}),
                }],
            },
            MenuGroup {
                icon: Icons::WARNING.into(),
                label: "System".to_string(),
                open: route_eq(&route, &Route::Sessions {})
                    || route_eq(&route, &Route::ShowDevices {}),
                items: vec![
                    MenuItemConfig {
                        icon: "".to_string(),
                        label: "Show devices".to_string(),
                        link: Route::ShowDevices {}.to_string(),
                        selected: route_eq(&route, &Route::ShowDevices {}),
                    },
                    MenuItemConfig {
                        icon: "".to_string(),
                        label: "Login sessions".to_string(),
                        link: Route::Sessions {}.to_string(),
                        selected: route_eq(&route, &Route::Sessions {}),
                    },
                ],
            },
            #[cfg(debug_assertions)]
            MenuGroup {
                icon: Icons::BUG.into(),
                label: "Debug".to_string(),
                open: route_eq(&route, &Route::Test {})
                    || route_eq(&route, &Route::Show {})
                    || route_eq(&route, &Route::ShowDev {})
                    || route_eq(&route, &Route::DBUser {})
                    || route_eq(&route, &Route::DBType {})
                    || route_eq(&route, &Route::DBFile {})
                    || route_eq(&route, &Route::DBTape {})
                    || route_eq(&route, &Route::DBJob {})
                    || route_eq(&route, &Route::DBJobMetaData {})
                    || route_eq(&route, &Route::ShowAppState {}),
                items: vec![
                    MenuItemConfig {
                        icon: "".to_string(),
                        label: "Test".to_string(),
                        link: Route::Test {}.to_string(),
                        selected: route_eq(&route, &Route::Test {}),
                    },
                    MenuItemConfig {
                        icon: "".to_string(),
                        label: "Show".to_string(),
                        link: Route::Show {}.to_string(),
                        selected: route_eq(&route, &Route::Show {}),
                    },
                    MenuItemConfig {
                        icon: "".to_string(),
                        label: "Dev".to_string(),
                        link: Route::ShowDev {}.to_string(),
                        selected: route_eq(&route, &Route::ShowDev {}),
                    },
                    MenuItemConfig {
                        icon: "".to_string(),
                        label: "User".to_string(),
                        link: Route::DBUser {}.to_string(),
                        selected: route_eq(&route, &Route::DBUser {}),
                    },
                    MenuItemConfig {
                        icon: "".to_string(),
                        label: "Type".to_string(),
                        link: Route::DBType {}.to_string(),
                        selected: route_eq(&route, &Route::DBType {}),
                    },
                    MenuItemConfig {
                        icon: "".to_string(),
                        label: "Tape".to_string(),
                        link: Route::DBTape {}.to_string(),
                        selected: route_eq(&route, &Route::DBTape {}),
                    },
                    MenuItemConfig {
                        icon: "".to_string(),
                        label: "File".to_string(),
                        link: Route::DBFile {}.to_string(),
                        selected: route_eq(&route, &Route::DBFile {}),
                    },
                    MenuItemConfig {
                        icon: "".to_string(),
                        label: "Job".to_string(),
                        link: Route::DBJob {}.to_string(),
                        selected: route_eq(&route, &Route::DBJob {}),
                    },
                    MenuItemConfig {
                        icon: "".to_string(),
                        label: "Job Metadata".to_string(),
                        link: Route::DBJobMetaData {}.to_string(),
                        selected: route_eq(&route, &Route::DBJobMetaData {}),
                    },
                    MenuItemConfig {
                        icon: "".to_string(),
                        label: "App State".to_string(),
                        link: Route::ShowAppState {}.to_string(),
                        selected: route_eq(&route, &Route::ShowAppState {}),
                    },
                ],
            },
            #[cfg(debug_assertions)]
            MenuGroup {
                icon: Icons::SANDPIT.into(),
                label: "Sandpit".to_string(),
                open: route_eq(&route, &Route::Sandpit {})
                    || route_eq(&route, &Route::SandpitShowcase {}),
                items: vec![
                    MenuItemConfig {
                        icon: "".to_string(),
                        label: "Index".to_string(),
                        link: Route::Sandpit {}.to_string(),
                        selected: route_eq(&route, &Route::Sandpit {}),
                    },
                    MenuItemConfig {
                        icon: "".to_string(),
                        label: "Showcase".to_string(),
                        link: Route::SandpitShowcase {}.to_string(),
                        selected: route_eq(&route, &Route::SandpitShowcase {}),
                    },
                ],
            },
        ],
    });

    rsx! {
        if let user = current_user().unwrap_or_default()
            && let icon_theme = format!("{:?}", user.icon_theme).to_lowercase()
        {
            div { style: "--icon-theme:{icon_theme}; --colour-accent:{user.accent_colour}",
                ColourModeHidden { theme: user.system_theme }
                if current_user().is_some() {
                    aside { class: Css::MAIN_ASIDE,
                        Menu { config: aside_config }
                    }
                    header { class: Css::MAIN_HEADER,
                        div { class: Css::MAIN_HEADER_LOGO, "{APP_NAME}" }
                        div {
                            class: [
                                Css::SCREEN_FILL,
                                either!(show_notifications() || show_info() || show_menu(), Css::SHOW, ""),
                            ]
                                .concat(),
                            onclick: close,
                        }
                        div {
                            class: [Css::HEADER_DROPDOWN, either!(show_notifications(), Css::SHOW, "")].concat(),
                            onclick: move |evt: Event<MouseData>| {
                                evt.stop_propagation();
                                show_theme.set(false);
                                show_accent.set(false);
                                show_menu.set(false);
                                show_info.set(false);
                                show_notifications.set(!show_notifications())
                            },
                            span { class: static_concat!(Css::ICON, Icons::NOTIFICATION) }
                            div { class: Css::HEADER_DROPDOWN_CONTENT,
                                div { class: static_concat!(Css::ICON_LIST_ITEM, Css::FLEX_ROW),
                                    "..."
                                }
                            }
                        }
                        div {
                            class: [Css::HEADER_DROPDOWN, either!(show_info(), Css::SHOW, "")].concat(),
                            onclick: move |evt: Event<MouseData>| {
                                evt.stop_propagation();
                                show_theme.set(false);
                                show_accent.set(false);
                                show_menu.set(false);
                                show_notifications.set(false);
                                show_info.set(!show_info());
                            },
                            span { class: static_concat!(Css::ICON, Icons::INFO) }
                            div { class: Css::HEADER_DROPDOWN_CONTENT,
                                div {
                                    class: static_concat!(Css::ICON_LIST_ITEM, Css::FLEX_ROW),
                                    onclick: move |_evt: Event<MouseData>| async move {
                                        let _ = document::eval("alert('Not ready yet');").await;
                                    },
                                    span { class: static_concat!(Css::ICON, Css::SM, Icons::BOOK) }
                                    span { "Docs" }
                                }
                                Link {
                                    "target": "_blank",
                                    class: static_concat!(Css::ICON_LIST_ITEM, Css::FLEX_ROW),
                                    to: "https://github.com/nathros/open-lto-manager",
                                    span {
                                        style: format!("mask-image:url({}#github)", LOGO_ASSET),
                                        class: static_concat!(Css::ICON, Css::SM),
                                    }
                                    span { "GitHub" }
                                }
                                Link {
                                    "target": "_blank",
                                    class: static_concat!(Css::ICON_LIST_ITEM, Css::FLEX_ROW),
                                    to: "https://github.com/nathros/open-lto-manager/issues",
                                    span { class: static_concat!(Css::ICON, Css::SM, Icons::BUG) }
                                    span { "Bug report" }
                                }
                            }
                        }
                        div {
                            class: [Css::HEADER_DROPDOWN, either!(show_menu(), Css::SHOW, "")].concat(),
                            onclick: move |evt: Event<MouseData>| {
                                evt.stop_propagation();
                                show_theme.set(false);
                                show_accent.set(false);
                                show_info.set(false);
                                show_notifications.set(false);
                                show_menu.set(!show_menu());
                            },
                            span { class: static_concat!(Css::ICON, Icons::USER) }
                            div {
                                class: Css::HEADER_DROPDOWN_CONTENT,
                                onclick: move |evt: Event<MouseData>| {
                                    evt.stop_propagation();
                                },
                                div { class: static_concat!(Css::ICON_LIST_ITEM, Css::HEADER_USER),
                                    "{user.username}"
                                }
                                Link {
                                    class: static_concat!(Css::ICON_LIST_ITEM, Css::FLEX_ROW),
                                    onclick: close,
                                    to: Route::DBUser {},
                                    span { class: static_concat!(Css::ICON, Css::SM, Icons::USER) }
                                    span { "Account" }
                                }
                                div {
                                    class: [Css::ICON_LIST_ITEM, Css::FLEX_ROW, either!(show_accent(), Css::SELECTED, "")]
                                        .concat(),
                                    onclick: move |evt: Event<MouseData>| {
                                        evt.stop_propagation();
                                        show_accent.set(!show_accent());
                                        show_theme.set(false);
                                        show_icon.set(false);
                                    },
                                    span { class: static_concat!(Css::ICON, Css::SM, Icons::PALETTE) }
                                    span { "Accent" }
                                    span { class: static_concat!(Css::ICON, Css::SM, Icons::CHEVRON_RIGHT, Css::FLOAT_RIGHT) }
                                    div {
                                        class: [
                                            Css::HEADER_DROPDOWN_CONTENT,
                                            Css::HEADER_DROPDOWN_NESTED,
                                            either!(show_accent(), Css::SHOW, ""),
                                        ]
                                            .concat(),
                                        div {
                                            class: [
                                                Css::ICON_LIST_ITEM,
                                                Css::FLEX_ROW,
                                                either!(user.accent_colour == ACCENT_STANDARD, Css::SELECTED, ""),
                                            ]
                                                .concat(),
                                            onclick: move |evt: Event<MouseData>| async move {
                                                change_accent(ACCENT_STANDARD.into()).await;
                                                close(evt);
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
                                                either!(user.accent_colour == ACCENT_RED, Css::SELECTED, ""),
                                            ]
                                                .concat(),
                                            onclick: move |evt: Event<MouseData>| async move {
                                                change_accent(ACCENT_RED.into()).await;
                                                close(evt);
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
                                                either!(user.accent_colour == ACCENT_GREEN, Css::SELECTED, ""),
                                            ]
                                                .concat(),
                                            onclick: move |evt: Event<MouseData>| async move {
                                                change_accent(ACCENT_GREEN.into()).await;
                                                close(evt);
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
                                                either!(user.accent_colour == ACCENT_BLUE, Css::SELECTED, ""),
                                            ]
                                                .concat(),
                                            onclick: move |evt: Event<MouseData>| async move {
                                                change_accent(ACCENT_BLUE.into()).await;
                                                close(evt);
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
                                                either!(
                                                    user.accent_colour == ACCENT_STANDARD ||
                                                    user.accent_colour == ACCENT_RED ||
                                                    user.accent_colour == ACCENT_GREEN ||
                                                    user.accent_colour == ACCENT_BLUE,
                                                    "",
                                                    Css::SELECTED
                                                ),
                                            ]
                                                .concat(),
                                            r#for: Css::ID_ACCENT_PICKER,
                                            onclick: move |evt: Event<MouseData>| {
                                                evt.stop_propagation();
                                            },
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
                                }
                                div {
                                    class: [Css::ICON_LIST_ITEM, Css::FLEX_ROW, either!(show_icon(), Css::SELECTED, "")]
                                        .concat(),
                                    onclick: move |evt: Event<MouseData>| {
                                        evt.stop_propagation();
                                        show_icon.set(!show_icon());
                                        show_theme.set(false);
                                        show_accent.set(false);
                                    },
                                    span { class: static_concat!(Css::ICON, Css::SM, Icons::SANDPIT) }
                                    span { "Icons" }
                                    span { class: static_concat!(Css::ICON, Css::SM, Icons::CHEVRON_RIGHT, Css::FLOAT_RIGHT) }
                                    div {
                                        class: [
                                            Css::HEADER_DROPDOWN_CONTENT,
                                            Css::HEADER_DROPDOWN_NESTED,
                                            either!(show_icon(), Css::SHOW, ""),
                                        ]
                                            .concat(),
                                        div {
                                            class: [
                                                Css::ICON_LIST_ITEM,
                                                Css::FLEX_ROW,
                                                either!(user.icon_theme == IconTheme::Tabler, Css::SELECTED, ""),
                                            ]
                                                .concat(),
                                            onclick: move |evt: Event<MouseData>| async move {
                                                change_icon(IconTheme::Tabler).await;
                                                close(evt);
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
                                                either!(user.icon_theme == IconTheme::Remix, Css::SELECTED, ""),
                                            ]
                                                .concat(),
                                            onclick: move |evt: Event<MouseData>| async move {
                                                change_icon(IconTheme::Remix).await;
                                                close(evt);
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
                                                either!(user.icon_theme == IconTheme::Iconoir, Css::SELECTED, ""),
                                            ]
                                                .concat(),
                                            onclick: move |evt: Event<MouseData>| async move {
                                                change_icon(IconTheme::Iconoir).await;
                                                close(evt);
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
                                                either!(user.icon_theme == IconTheme::Sargam, Css::SELECTED, ""),
                                            ]
                                                .concat(),
                                            onclick: move |evt: Event<MouseData>| async move {
                                                change_icon(IconTheme::Sargam).await;
                                                close(evt);
                                            },
                                            span {
                                                style: format!("mask-image:url({}#sandpit)", ICONS_ASSET_SARGAM_LINE),
                                                class: static_concat!(Css::ICON, Css::SM, Css::HEADER_COL),
                                            }
                                            span { "Sargam" }
                                        }
                                    }
                                }
                                div {
                                    class: [Css::ICON_LIST_ITEM, Css::FLEX_ROW, either!(show_theme(), Css::SELECTED, "")]
                                        .concat(),
                                    onclick: move |evt: Event<MouseData>| {
                                        evt.stop_propagation();
                                        show_theme.set(!show_theme());
                                        show_accent.set(false);
                                        show_icon.set(false);
                                    },
                                    span { class: static_concat!(Css::ICON, Css::SM, Icons::CONTRAST) }
                                    span { "Theme" }
                                    span { class: static_concat!(Css::ICON, Css::SM, Icons::CHEVRON_RIGHT, Css::FLOAT_RIGHT) }
                                    div {
                                        class: [
                                            Css::HEADER_DROPDOWN_CONTENT,
                                            Css::HEADER_DROPDOWN_NESTED,
                                            either!(show_theme(), Css::SHOW, ""),
                                        ]
                                            .concat(),
                                        div {
                                            class: [
                                                Css::ICON_LIST_ITEM,
                                                Css::FLEX_ROW,
                                                either!(user.system_theme == ColourMode::System, Css::SELECTED, ""),
                                            ]
                                                .concat(),
                                            onclick: move |evt: Event<MouseData>| async move {
                                                change_theme(ColourMode::System).await;
                                                close(evt);
                                            },
                                            span { class: static_concat!(Css::ICON, Css::SM, Icons::SYSTEM) }
                                            span { "System" }
                                        }
                                        div {
                                            class: [
                                                Css::ICON_LIST_ITEM,
                                                Css::FLEX_ROW,
                                                either!(user.system_theme == ColourMode::Light, Css::SELECTED, ""),
                                            ]
                                                .concat(),
                                            onclick: move |evt: Event<MouseData>| async move {
                                                change_theme(ColourMode::Light).await;
                                                close(evt);
                                            },
                                            span { class: static_concat!(Css::ICON, Css::SM, Icons::LIGHT) }
                                            span { "Light" }
                                        }
                                        div {
                                            class: [
                                                Css::ICON_LIST_ITEM,
                                                Css::FLEX_ROW,
                                                either!(user.system_theme == ColourMode::Dark, Css::SELECTED, ""),
                                            ]
                                                .concat(),
                                            onclick: move |evt: Event<MouseData>| async move {
                                                change_theme(ColourMode::Dark).await;
                                                close(evt);
                                            },
                                            span { class: static_concat!(Css::ICON, Css::SM, Icons::DARK) }
                                            span { "Dark" }
                                        }
                                    }
                                }
                                hr {}
                                Link {
                                    class: static_concat!(Css::ICON_LIST_ITEM, Css::FLEX_ROW),
                                    onclick: |_| async {
                                        let _ = api_logout().await;
                                    },
                                    to: Route::LoginUser {},
                                    span { class: static_concat!(Css::ICON, Css::SM, Icons::LOGOUT) }
                                    span { "Log out" }
                                }
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
