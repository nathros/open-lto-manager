use dioxus::prelude::*;

use crate::{
    frontend::{css::Css, elements::input::InputType},
    shared::models::database::user::model_user::ColourMode,
};

#[component]
pub fn ColourModeHidden(theme: ColourMode) -> Element {
    rsx! {
        form { style: "display:none",
            if let type_ = InputType::Radio.to_string() {
                input {
                    r#type: type_,
                    id: Css::COLOUR_SCHEME_LIGHT,
                    name: Css::COLOUR_SCHEME,
                    value: "0",
                    checked: theme == ColourMode::Light,
                }
                input {
                    r#type: type_,
                    id: Css::COLOUR_SCHEME_DARK,
                    name: Css::COLOUR_SCHEME,
                    value: "1",
                    checked: theme == ColourMode::Dark,
                }
                input {
                    r#type: type_,
                    name: Css::COLOUR_SCHEME,
                    value: "auto",
                    checked: theme == ColourMode::System,
                }
            }

        }
    }
}
