use dioxus::prelude::*;

fn show_tab(show: bool) -> String {
    if show {
        "display:block".to_string()
    } else {
        "display:none".to_string()
    }
}

fn highlight_btn(show: bool) -> String {
    if show {
        "background-color:cyan".to_string()
    } else {
        "background-color:initial".to_string()
    }
}

#[component]
pub fn Tab(
    labels: Vec<String>,
    contents: Vec<Element>,
    #[props(extends = div, extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let mut selected_tab = use_signal(|| 0);

    rsx! {
        div {..attributes,
            if let tab_index = selected_tab() {
                for (index , name) in labels.iter().enumerate() {
                    button {
                        style: highlight_btn(tab_index == index),
                        onclick: move |_| {
                            selected_tab.set(index);
                        },
                        "{name}"
                    }
                }
                br {}
                br {}
                for (index , child) in contents.iter().enumerate() {
                    div { style: show_tab(tab_index == index), {child} }
                }
            }

        }
    }
}
