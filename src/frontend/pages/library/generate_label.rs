use dioxus::prelude::*;
use serde_json::json;

use crate::{
    backend::api::{
        api_generate_lto_label::{GENERATE_PDF_LABEL_DOWNLOAD, generate_label_preview},
        api_tape_type::list_type_type_labels,
    },
    frontend::{
        components::card::Card,
        css::Css,
        elements::{
            button::Button,
            input::{Input, InputType},
            select::Select,
        },
        forms::validator::{TValidator, Validator},
        icons::Icons,
        js::js_download_file,
        modules::tab::Tab,
    },
    shared::{
        r#const::Const,
        models::{
            database::label_preset::model_label_preset::{
                LabelFont, LabelOptions, LabelTextDirection, LabelTextOrientation, LabelTheme,
                PDFPageType,
            },
            select_option::{vec_into, vec_into_enum, vec_into_enum_default},
        },
    },
};

const QUANTITY_MIN: usize = 0;
const QUANTITY_MAX: usize = 500;
const START_INDEX_MIN: usize = 0;
const START_INDEX_MAX: usize = 999999;

#[component]
pub fn GenLabel() -> Element {
    rsx! {
        SuspenseBoundary {
            fallback: |_sc: SuspenseContext| {
                rsx! {}
            },
            GenLabelInner {}
        }
    }
}

#[component]
fn GenLabelInner() -> Element {
    let mut options = use_signal(|| LabelOptions::default());
    let previews = use_loader(move || generate_label_preview(options()))?;

    let types = use_loader(list_type_type_labels)?;

    let mut selected_type = use_signal(move || {
        if let Some(find) = types().first() {
            options.write().designation = find.designation.clone();
            find.id
        } else {
            0
        }
    });

    let mut form_valid: Signal<bool> = use_signal(|| true);

    let (mut postfix_error, mut prefix_error, p_validator) = (
        use_signal(|| None),
        use_signal(|| None),
        Validator::<String>::create()
            .with_max_length(Const::CODE_39_LTO_USABLE_LEN)
            .with_only_allowed_chars(Const::BARCODE_VALID_CHARS),
    );
    let (mut quantity_error, n_validator) = (
        use_signal(|| None),
        Validator::<usize>::create()
            .with_min_number(QUANTITY_MIN)
            .with_max_number(QUANTITY_MAX),
    );
    let (mut start_index_error, i_validator) = (
        use_signal(|| None),
        Validator::<usize>::create()
            .with_min_number(START_INDEX_MIN)
            .with_max_number(START_INDEX_MAX),
    );

    use_memo(move || {
        let tmp = options();
        prefix_error.set(p_validator.validate(&tmp.prefix));
        postfix_error.set(p_validator.validate(&tmp.postfix));
        quantity_error.set(n_validator.validate(&tmp.quantity));
        start_index_error.set(i_validator.validate(&tmp.start_index));

        if tmp.postfix.len() + tmp.postfix.len() > Const::CODE_39_LTO_USABLE_LEN {
            let msg = format!(
                "Prefix and postfix length must be less than {}",
                Const::CODE_39_LTO_USABLE_LEN
            );
            prefix_error.set(Some(msg.clone()));
            postfix_error.set(Some(msg));
        }

        let valid: bool = !(prefix_error().is_some()
            || postfix_error().is_some()
            || quantity_error().is_some()
            || start_index_error().is_some());
        form_valid.set(valid);
    });

    let label_tab = rsx! {
        form { class: Css::FORM_GRID,
            Select {
                label: "Tape Type".to_string(),
                options: vec_into(types()),
                selected: selected_type(),
                onchange: move |evt: Event<FormData>| {
                    let index = evt.value().parse::<i64>().unwrap_or_default();
                    selected_type.set(index);
                    if let Some(t) = types().get(index as usize) {
                        options.write().designation = t.designation.clone();
                    }
                },
            }
            Input {
                type_: InputType::Text,
                label: "Prefix".to_string(),
                oninput: move |evt: Event<FormData>| {
                    options.write().prefix = evt.value().to_uppercase();
                },
                value: options().prefix,
                validation: prefix_error,
            }
            Input {
                type_: InputType::Text,
                label: "Postfix".to_string(),
                oninput: move |evt: Event<FormData>| {
                    options.write().postfix = evt.value().to_uppercase();
                },
                value: options().postfix,
                validation: postfix_error,
            }
            Input {
                type_: InputType::Number,
                label: "Start Index".to_string(),
                oninput: move |evt: Event<FormData>| {
                    options.write().start_index = evt.value().parse().unwrap_or_default();
                },
                min: START_INDEX_MIN,
                max: START_INDEX_MAX,
                value: options().start_index,
                validation: start_index_error,
            }
            Input {
                type_: InputType::Number,
                label: "Quantity".to_string(),
                oninput: move |evt: Event<FormData>| {
                    options.write().quantity = evt.value().parse().unwrap_or_default();
                },
                min: QUANTITY_MIN,
                max: QUANTITY_MAX,
                value: options().quantity,
                validation: quantity_error,
            }
        }
    };

    let style_tab = rsx! {
        form { class: Css::FORM_GRID,
            Select {
                label: "Theme".to_string(),
                options: vec_into_enum_default::<LabelTheme>(),
                selected: options().theme as i64,
                onchange: move |evt: Event<FormData>| {
                    options.write().theme = LabelTheme::from(
                        evt.value().parse::<i64>().unwrap_or_default(),
                    );
                },
            }
            Select {
                label: "Font".to_string(),
                options: vec_into_enum_default::<LabelFont>(),
                selected: options().font as i64,
                onchange: move |evt: Event<FormData>| {
                    options.write().font = LabelFont::from(
                        evt.value().parse::<i64>().unwrap_or_default(),
                    );
                },
            }
            Select {
                label: "Text Direction".to_string(),
                options: vec_into_enum_default::<LabelTextDirection>(),
                selected: options().text_direction as i64,
                onchange: move |evt: Event<FormData>| {
                    options.write().text_direction = LabelTextDirection::from(
                        evt.value().parse::<i64>().unwrap_or_default(),
                    );
                },
            }
            Select {
                label: "Text Orientation".to_string(),
                options: vec_into_enum_default::<LabelTextOrientation>(),
                selected: options().text_orientation as i64,
                onchange: move |evt: Event<FormData>| {
                    options.write().text_orientation = LabelTextOrientation::from(
                        evt.value().parse::<i64>().unwrap_or_default(),
                    );
                },
            }
            Input {
                type_: InputType::Number,
                label: "Stroke Outer".to_string(),
                oninput: move |evt: Event<FormData>| {
                    options.write().stroke_outer = evt.value().parse().unwrap_or_default();
                },
                min: 0,
                max: 2,
                step: 0.1,
                value: options().stroke_outer,
            }
            Input {
                type_: InputType::Number,
                label: "Stroke Inner".to_string(),
                oninput: move |evt: Event<FormData>| {
                    options.write().stroke_inner = evt.value().parse().unwrap_or_default();
                },
                min: 0,
                max: 2,
                step: 0.1,
                value: options().stroke_inner,
            }
            Input {
                type_: InputType::Number,
                label: "Radius Outer".to_string(),
                oninput: move |evt: Event<FormData>| {
                    options.write().radius_outer = evt.value().parse().unwrap_or_default();
                },
                min: 0,
                max: 8,
                step: 0.1,
                value: options().radius_outer,
            }
            Input {
                type_: InputType::Number,
                label: "Radius Inner".to_string(),
                oninput: move |evt: Event<FormData>| {
                    options.write().radius_inner = evt.value().parse().unwrap_or_default();
                },
                min: 0,
                max: 8,
                step: 0.1,
                value: options().radius_inner,
            }
            Input {
                type_: InputType::Number,
                label: "Barcode Scale".to_string(),
                oninput: move |evt: Event<FormData>| {
                    options.write().barcode_scale = evt.value().parse().unwrap_or_default();
                },
                min: 0.1,
                max: 1.15,
                step: 0.05,
                value: options().barcode_scale,
            }
            Input {
                type_: InputType::Number,
                label: "Text Box Width".to_string(),
                oninput: move |evt: Event<FormData>| {
                    options.write().text_box_width = evt.value().parse().unwrap_or_default();
                },
                min: 3,
                max: 11.2,
                step: 0.1,
                value: options().text_box_width,
            }
        }
    };

    let page_tab = rsx! {
        form { class: Css::FORM_GRID,
            Select {
                label: "Page Type".to_string(),
                options: vec_into_enum::<PDFPageType>(),
                selected: options().page as i64,
                onchange: move |evt: Event<FormData>| {
                    let p_type = PDFPageType::from(evt.value().parse::<i64>().unwrap_or_default());
                    options.write().switch_page(p_type);
                },
            }
            Input {
                type_: InputType::Number,
                label: "X Offset (mm)".to_string(),
                oninput: move |evt: Event<FormData>| {
                    options.write().page_x_offset = evt.value().parse().unwrap_or_default();
                },
                min: -10,
                max: 10,
                step: 0.1,
                value: format!("{:.1}", options().page_x_offset),
            }
            Input {
                type_: InputType::Number,
                label: "Y Offset (mm)".to_string(),
                oninput: move |evt: Event<FormData>| {
                    options.write().page_y_offset = evt.value().parse().unwrap_or_default();
                },
                min: -10,
                max: 10,
                step: 0.1,
                value: format!("{:.1}", options().page_y_offset),
            }
            Input {
                type_: InputType::Number,
                label: "Inner X Gap (mm)".to_string(),
                oninput: move |evt: Event<FormData>| {
                    options.write().page_inner_x_gap = evt.value().parse().unwrap_or_default();
                },
                min: -4,
                max: 4,
                step: 0.05,
                value: format!("{:.2}", options().page_inner_x_gap),
            }
            Input {
                type_: InputType::Number,
                label: "Inner Y Gap (mm)".to_string(),
                oninput: move |evt: Event<FormData>| {
                    options.write().page_inner_y_gap = evt.value().parse().unwrap_or_default();
                },
                min: -4,
                max: 4,
                step: 0.05,
                value: format!("{:.2}", options().page_inner_y_gap),
            }
        }
    };

    rsx! {
        div { class: Css::FLEX_ROW,
            Card {
                Tab {
                    labels: vec!["Label".to_string(), "Style".to_string(), "Page".to_string()],
                    contents: vec![label_tab, style_tab, page_tab],
                }
                hr {}
                Button {
                    text: "Download".to_string(),
                    icon: Icons::PDF,
                    primary: true,
                    onclick: move |_| async move {
                        js_download_file( // Should match endpoint _ep
                            GENERATE_PDF_LABEL_DOWNLOAD,
                            "pdf",
                            json!({ "options" : options() }),
                        );
                    },
                }
                Button {
                    text: "Reset".to_string(),
                    icon: Icons::RESTORE,
                    primary: true,
                    onclick: move |_| {
                        let page = options().page;
                        let des = options().designation;
                        let mut replace = LabelOptions::default();
                        replace.switch_page(page);
                        replace.designation = des;
                        options.set(replace);
                    },
                }
            }
            Card {
                div { class: Css::PDF_PREVIEW,
                    for p in previews() {
                        div { dangerous_inner_html: p }
                    }
                }
            }
        }
        Debug { options, form_valid: form_valid() }
    }
}

#[cfg(debug_assertions)]
#[component]
fn Debug(options: Signal<LabelOptions>, form_valid: bool) -> Element {
    use crate::frontend::{
        components::header::{header_extra::HeaderExtraIcons, header_icon::HeaderIcon},
        icons::Icons,
    };
    rsx! {
        HeaderExtraIcons {
            HeaderIcon { button_id: "dbg_btn", menu_id: "dbg_menu", icon: Icons::BUG,
                div { class: Css::DEBUG_MENU, style: "width:20rem",
                    h2 { "Debug" }
                    p { "{options():?}" }
                    hr {}
                    p { "form_valid: {form_valid}" }
                }
            }
        }
    }
}

#[cfg(not(debug_assertions))]
#[component]
fn Debug(options: Signal<LabelOptions>, form_valid: bool) -> Element {
    rsx! {}
}
