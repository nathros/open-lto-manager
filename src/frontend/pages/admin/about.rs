use dioxus::prelude::*;

use crate::{
    backend::api::api_init::app_about, frontend::components::card::Card, shared::r#const::Const,
};

#[component]
pub fn About() -> Element {
    let about = use_loader(app_about)?;

    rsx! {
        Card {
            h2 { "{Const::APP_NAME}" }
            p { "Branch: {about().branch}" }
            p { "Commit Hash: {about().commit_hash_short}" }
            p { "Dirty: {about().dirty}" }
            p { "Build Date: {about().build_date}" }
            p { "Tag: {about().tag}" }
            for dep in about().dependencies {
                hr {}
                b { "{dep.name}" }
                p { "Version: {dep.version}" }
                a { href: "{dep.url}", target: "_blank", "{dep.url}" }
            }
        }
    }
}
