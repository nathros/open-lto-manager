use dioxus::prelude::*;

use crate::{
    either,
    frontend::{css::Css, icons::Icons},
    shared::models::select_option::SelectOption,
    static_concat,
};

#[derive(Props, PartialEq, Clone)]
pub struct SelectProps {
    #[props(default = "".into())]
    style: String,

    #[props(optional)]
    label: String,

    #[props]
    options: Vec<SelectOption>,

    #[props]
    selected: i64,

    #[props(default = false)]
    required: bool,

    #[props(optional)]
    onchange: EventHandler<FormEvent>,

    #[props(optional)]
    validation: ReadSignal<Option<String>>,

    #[props(extends = select, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn Select(props: SelectProps) -> Element {
    let not_found = props
        .options
        .iter()
        .find(|p| p.id == props.selected)
        .is_none();

    rsx! {
        div { class: Css::INPUT_CONTAINER,
            if !props.label.is_empty() {
                label { "{props.label}:" }
            }
            div {
                select {
                    class: [Css::SELECT, either!(not_found && props.required, Icons::ERROR, "")].concat(),
                    onchange: props.onchange,
                    if not_found {
                        option { disabled: true, selected: true, "Select" }
                    }
                    for opt in props.options {
                        option { value: "{opt.id}", "{opt.label}" }
                    }
                }
                div { class: static_concat!(Css::INPUT_ERROR_ICON, Css::ICON, Icons::WARNING) }
            }
        }
    }
}
