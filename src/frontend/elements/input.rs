use dioxus::prelude::*;

use crate::{
    either,
    frontend::{css::Css, icons::Icons},
    static_concat,
};

#[derive(PartialEq, Clone)]
pub enum InputType {
    // Uncomment when needed
    //button,
    Checkbox,
    Colour,
    //date,
    //datetimelocal,
    //email,
    //file,
    //Hidden,
    //image,
    //month,
    Number,
    Password,
    Radio,
    //range,
    //reset,
    Search,
    //submit,
    //tel,
    Text,
    //time,
    //url,
    //week,
}

impl InputType {
    pub const fn to_string(&self) -> &str {
        match self {
            //InputType::button => "button",
            InputType::Checkbox => "checkbox",
            InputType::Colour => "color",
            //InputType::date => "date",
            //InputType::datetimelocal => "datetime-local",
            //InputType::email => "email",
            //InputType::file => "file",
            //InputType::Hidden => "hidden",
            //InputType::image => "image",
            //InputType::month => "month",
            InputType::Number => "number",
            InputType::Password => "password",
            InputType::Radio => "radio",
            //InputType::range => "range",
            //InputType::reset => "reset",
            InputType::Search => "search",
            //InputType::submit => "submit",
            //InputType::tel => "tel",
            InputType::Text => "text",
            //InputType::time => "time",
            //InputType::url => "url",
            //InputType::week => "week",
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct InputProps {
    #[props]
    type_: InputType,

    #[props(default = "".into())]
    style: String,

    #[props(optional)]
    label: String,

    #[props(optional)]
    oninput: EventHandler<FormEvent>,

    #[props(optional)]
    validation: ReadSignal<Option<String>>,

    #[props(extends = input, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional)]
    meta: String,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    rsx! {
        if let err = props.validation.as_ref().is_some() {
            div { class: Css::INPUT_CONTAINER,
                if !props.label.is_empty() {
                    label { "{props.label}:" }
                }
                div {
                    input {
                        class: [Css::INPUT, either!(err, Icons::ERROR, "")].concat(),
                        r#type: props.type_.to_string(),
                        style: props.style,
                        oninput: props.oninput,
                        ..props.attributes,
                    }
                    div { class: Css::INPUT_MESSAGE,
                        "{props.validation.read().clone().unwrap_or_default()}"
                    }
                    div { class: static_concat!(Css::INPUT_ERROR_ICON, Css::ICON, Icons::WARNING) }
                }
            }
        }
    }
}

#[component]
pub fn InputBarcode(props: InputProps) -> Element {
    rsx! {
        if let err = props.validation.as_ref().is_some() {
            div { class: Css::INPUT_CONTAINER,
                if !props.label.is_empty() {
                    label { "{props.label}:" }
                }
                div { class: Css::GAP_S, style: "display:flex",
                    div { style: "position:relative;flex:1",
                        input {
                            class: [Css::INPUT, either!(err, Icons::ERROR, "")].concat(),
                            r#type: props.type_.to_string(),
                            style: props.style,
                            oninput: props.oninput,
                            ..props.attributes,
                        }
                        div { class: Css::INPUT_MESSAGE,
                            "{props.validation.read().clone().unwrap_or_default()}"
                        }
                        div { class: static_concat!(Css::INPUT_ERROR_ICON, Css::ICON, Icons::WARNING) }
                    }
                    div {
                        input {
                            class: Css::INPUT,
                            style: "width:1rem",
                            r#type: props.type_.to_string(),
                            readonly: true,
                            value: props.meta,
                        }
                    }
                }
            }
        }
    }
}
