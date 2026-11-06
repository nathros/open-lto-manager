use dioxus::prelude::*;

use crate::frontend::{
    components::context_menu::component::ContextMenu, id::Id, js::js_show_context_menu,
};

#[component]
pub fn SandpitContextMenu() -> Element {
    let text: &str = "Item";
    rsx! {
        div {
            style: "width:10rem;height:10rem;background-color:grey",
            oncontextmenu: move |evt: MouseEvent| {
                evt.prevent_default();
                js_show_context_menu(Id::ContextMenuSandpit.as_str(), &evt);
            },
            p { "Right click in here" }

            ContextMenu { anchor_id: Id::ContextMenuSandpit.as_str(),
                p { "{text}" }
                p { "{text}" }
                p { "{text}" }
                p { "{text}" }
                p { "{text}" }
                p { "{text}" }
                p { "{text}" }
            }
        }
    }
}
