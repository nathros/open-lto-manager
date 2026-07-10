use dioxus::prelude::*;

use crate::{
    frontend::components::colour_mode::ColourModeHidden,
    shared::models::database::user::model_user::RecordUserConfig,
};

#[component]
pub fn MainBody(user: RecordUserConfig, #[props] children: Element) -> Element {
    rsx! {
        div { style: "--icon-theme:{user.icon_theme.as_str()}; --colour-accent:{user.accent_colour}",
            ColourModeHidden { theme: user.system_theme }
            {children}
        }
    }
}
