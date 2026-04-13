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

/*pub enum ButtonType {
    button, // HTML default
    submit,
    reset,
}

impl ButtonType {
    pub fn to_string(&self) -> &str {
        match self {
            ButtonType::button => "button",
            ButtonType::submit => "submit",
            ButtonType::reset => "reset",
        }
    }
}*/

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        button {
            class: "btn",
            style: props.style,
            onclick: props.onclick,
            ..props.attributes,
            "{props.text}"
        }
    }
}
