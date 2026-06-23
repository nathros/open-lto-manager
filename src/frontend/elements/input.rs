use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum InputType {
    // Uncomment when needed
    //button,
    //checkbox,
    Colour,
    //date,
    //datetimelocal,
    //email,
    //file,
    //Hidden,
    //image,
    //month,
    //number,
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
            //InputType::checkbox => "checkbox",
            InputType::Colour => "color",
            //InputType::date => "date",
            //InputType::datetimelocal => "datetime-local",
            //InputType::email => "email",
            //InputType::file => "file",
            //InputType::Hidden => "hidden",
            //InputType::image => "image",
            //InputType::month => "month",
            //InputType::number => "number",
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
    oninput: EventHandler<FormEvent>,

    #[props(extends = input, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    rsx! {
        input {
            r#type: props.type_.to_string(),
            style: props.style,
            oninput: props.oninput,
            ..props.attributes,
        }
    }
}
