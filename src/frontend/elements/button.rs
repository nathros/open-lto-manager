use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
    #[props(default = "".into())]
    style: String,

    #[props(default = "".into())]
    text: String,

    #[props(optional)]
    onclick: EventHandler<MouseEvent>,

    #[props(extends = button, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        button {
            style: format!("padding:0.5rem;{}", props.style),
            onclick: props.onclick,
            ..props.attributes,
            "{props.text}"
        }
    }
}
