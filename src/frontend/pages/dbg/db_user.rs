use dioxus::{fullstack::Loader, prelude::*};

use crate::{
    backend::api::api_user::list_users,
    shared::models::database::user::model_user::RecordUserConfig,
};

#[component]
pub fn DBUser() -> Element {
    info!("Render");
    rsx! {
        SuspenseBoundary {
            fallback: |_suspense_context: SuspenseContext| {
                rsx! {
                    Table {}
                }
            },
            Table { Inner {} }
        }
    }
}

#[component]
fn Table(children: Element) -> Element {
    rsx! {
        table {
            tr {
                th { "id" }
                th { "username" }
                th { "description" }
                th { "enabled" }
                th { "language" }
                th { "avatar" }
                th { "system_theme" }
                th { "icon_theme" }
                th { "fm_theme" }
                th { "accent_colour" }
            }
            {children}
        }
    }
}

#[component]
fn Inner() -> Element {
    let users_list: Loader<Vec<RecordUserConfig>> = use_loader(list_users)?;

    rsx! {
        for rec in users_list.cloned() {
            tr {
                td { "{rec.id}" }
                td { "{rec.username}" }
                td { "{rec.description}" }
                td { "{rec.enabled}" }
                td { "{rec.language}" }
                td { "{rec.avatar}" }
                td { "{rec.system_theme:?}" }
                td { "{rec.icon_theme:?}" }
                td { "{rec.file_theme:?}" }
                td { "{rec.accent_colour}" }
            }
        }
    }
}
