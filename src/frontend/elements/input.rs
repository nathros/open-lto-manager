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
    value: String,

    #[props(default = "".into())]
    style: String, // For container

    #[props(default = "")]
    icon: &'static str,

    #[props(optional)]
    clear_button: Option<Signal<String>>,

    #[props(optional)]
    label: String,

    #[props(optional)]
    oninput: EventHandler<FormEvent>,

    #[props(optional)]
    onkeydown: EventHandler<KeyboardEvent>,

    #[props(optional)]
    onmounted: EventHandler<MountedEvent>,

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
            div { class: Css::INPUT_CONTAINER, style: props.style,
                if !props.label.is_empty() {
                    label { "{props.label}:" }
                }
                label { class: either!(props.type_ == InputType::Checkbox, "", Css::INPUT_WRAPPER),
                    if !props.icon.is_empty() {
                        span { class: [Css::ICON, props.icon, Css::SM].concat() }
                    }
                    input {
                        class: [Css::INPUT, either!(err, Icons::ERROR, "")].concat(),
                        r#type: props.type_.to_string(),
                        oninput: props.oninput,
                        onkeydown: props.onkeydown,
                        onmounted: props.onmounted,
                        value: props.value,
                        ..props.attributes,
                    }
                    div { class: Css::INPUT_MESSAGE,
                        "{props.validation.read().clone().unwrap_or_default()}"
                    }
                    div { class: static_concat!(Css::INPUT_ERROR_ICON, Css::ICON, Icons::WARNING) }
                    if let Some(mut clear) = props.clear_button && !clear().is_empty() {
                        button {
                            class: static_concat!(Css::ICON, Icons::X_MARK, Css::SM),
                            onclick: move |_| {
                                clear.write().clear();
                            },
                        }
                    }
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
                    div {
                        class: Css::INPUT_WRAPPER,
                        style: "position:relative;flex:1",
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
                    div { class: Css::INPUT_WRAPPER,
                        input {
                            class: Css::INPUT,
                            style: "text-align:center;width:1.25rem", // field-sizing:content
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
