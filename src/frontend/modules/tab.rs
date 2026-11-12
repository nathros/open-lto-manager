use dioxus::prelude::*;

use crate::{either, frontend::css::Css};

#[derive(PartialEq, Clone)]
pub struct TabLabel {
    text: &'static str,
    icon: Option<&'static str>,
}

impl TabLabel {
    pub fn new(text: &'static str, icon: Option<&'static str>) -> TabLabel {
        TabLabel { text, icon }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct TabProps {
    #[props(optional)]
    labels: Vec<TabLabel>,

    #[props(optional)]
    contents: Vec<Element>,

    #[props(extends = button, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn Tab(props: TabProps) -> Element {
    let mut selected_tab = use_signal(|| 0);

    rsx! {
        div {..props.attributes,
            div { class: Css::TAB_HEADER,
                for (index , label) in props.labels.iter().enumerate() {
                    button {
                        class: either!(selected_tab() == index, Css::SELECTED, ""),
                        onclick: move |_| {
                            selected_tab.set(index);
                        },
                        if let Some(icon) = label.icon {
                            span { class: [Css::ICON, Css::SM, icon].concat() }
                        }
                        "{label.text}"
                    }
                }
            }
            hr {}
            for (index , child) in props.contents.iter().enumerate() {
                div { style: either!(selected_tab() == index, "display:block", "display:none"),
                    {child}
                }
            }
        }
    }
}
