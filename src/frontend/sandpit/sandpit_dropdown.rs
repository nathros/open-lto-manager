use dioxus::prelude::*;

use crate::frontend::{
    components::dropdown::component::DropDown, elements::button::Button, id::Id,
};

#[component]
pub fn SandpitDropDown() -> Element {
    rsx! {
        DropDown {
            anchor_id: Id::DropDownSandpitSingleAnchor.as_str(),
            anchor: rsx! {
                Button { primary: true, text: "Click Single".to_string() }
            },
            p { "Content" }
        }
    }
}
