pub enum InputType {
    // Uncomment when needed
    //button,
    //checkbox,
    //color,
    //date,
    //datetimelocal,
    //email,
    //file,
    //hidden,
    //image,
    //month,
    //number,
    //password,
    //radio,
    //range,
    //reset,
    //search,
    //submit,
    //tel,
    Text,
    //time,
    //url,
    //week,
}

impl InputType {
    pub fn to_string(&self) -> &str {
        match self {
            //InputType::button => "button",
            //InputType::checkbox => "checkbox",
            //InputType::color => "color",
            //InputType::date => "date",
            //InputType::datetimelocal => "datetime-local",
            //InputType::email => "email",
            //InputType::file => "file",
            //InputType::hidden => "hidden",
            //InputType::image => "image",
            //InputType::month => "month",
            //InputType::number => "number",
            //InputType::password => "password",
            //InputType::radio => "radio",
            //InputType::range => "range",
            //InputType::reset => "reset",
            //InputType::search => "search",
            //InputType::submit => "submit",
            //InputType::tel => "tel",
            InputType::Text => "text",
            //InputType::time => "time",
            //InputType::url => "url",
            //InputType::week => "week",
        }
    }
}
