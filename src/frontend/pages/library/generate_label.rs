use dioxus::prelude::*;

use crate::{
    frontend::{
        components::{card::Card, header::HeaderExtraIcons},
        css::Css,
        elements::input::{Input, InputType},
        forms::validator::{TValidator, Validator},
        modules::tab::Tab,
    },
    shared::models::database::{
        label_preset::model_label_preset::LabelOptions,
        tape::model_tape::{BARCODE_LEN, BARCODE_VALID_CHARS},
    },
};

#[component]
pub fn GenLabel() -> Element {
    let mut options = use_signal(|| LabelOptions::default());

    let mut form_valid = use_signal(|| true);

    let (mut postfix_error, mut prefix_error, p_validator) = (
        use_signal(|| None),
        use_signal(|| None),
        Validator::<String>::create()
            .with_max_length(BARCODE_LEN)
            .with_only_allowed_chars(BARCODE_VALID_CHARS),
    );

    let (mut number_error, mut number_val, n_validator) = (
        use_signal(|| None),
        use_signal(|| 555_i32),
        Validator::<i32>::create().with_min_number(50),
    );

    use_memo(move || {
        let tmp = options();
        prefix_error.set(p_validator.validate(&tmp.prefix));
        postfix_error.set(p_validator.validate(&tmp.postfix));
        number_error.set(n_validator.validate(&number_val()));

        if tmp.postfix.len() + tmp.postfix.len() > BARCODE_LEN {
            let msg = "Prefix and postfix length must be less than 6".to_string();
            prefix_error.set(Some(msg.clone()));
            postfix_error.set(Some(msg));
        }

        form_valid.set(
            !(prefix_error().is_some() || postfix_error().is_some() || number_error().is_some()),
        );
    });

    let label_tab = rsx! {
        span { "prefix " }
        Input {
            type_: InputType::Text,
            oninput: move |evt: Event<FormData>| {
                options.write().prefix = evt.value().to_uppercase();
            },
            value: options().prefix,
            validation: prefix_error,
        }
        hr {}
        span { "postfix " }
        Input {
            type_: InputType::Text,
            oninput: move |evt: Event<FormData>| {
                options.write().postfix = evt.value().to_uppercase();
            },
            value: options().postfix,
            validation: postfix_error,
        }
        hr {}
        span { "number " }
        Input {
            type_: InputType::Text,
            oninput: move |evt: Event<FormData>| {
                number_val.set(evt.value().parse::<i32>().unwrap_or(0));
            },
            value: number_val,
            validation: number_error,
        }
        p { "form valid {form_valid}" }

    };

    let page_tab = rsx! {
        p { "aaa" }
    };

    rsx! {
        HeaderExtraIcons {
            button { "added" }
        }
        div { class: Css::FLEX_ROW,
            Card {
                Tab {
                    labels: vec!["Label".to_string(), "Page".to_string()],
                    contents: vec![label_tab, page_tab],
                }
                hr {}
            }
            Card { top_padding: false,
                h3 { "Preview" }
            }
        }
    }
}
