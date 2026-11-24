use dioxus::prelude::*;
use serde_json::json;

use crate::{
    backend::api::{
        api_generate_lto_label::{GENERATE_PDF_LABEL_DOWNLOAD, generate_label_preview},
        api_settings_user::{del_label_preset, get_user_preset, save_label_preset},
        api_tape_type::list_type_type_labels,
    },
    frontend::{
        components::{card::Card, dropdown::component::DropDown},
        css::Css,
        elements::{
            button::{Button, FakeButton},
            input::{Input, InputType},
            select::Select,
        },
        forms::validator::{TValidator, Validator},
        icons::Icons,
        id::Id,
        js::{js_close, js_download_file, js_show_modal},
        modules::{
            dialog::Dialog,
            tab::{Tab, TabLabel},
        },
    },
    shared::{
        r#const::Const,
        models::{
            database::label_preset::model_label_preset::{
                LabelCheckDigit, LabelFont, LabelOptions, LabelTextDirection, LabelTextOrientation,
                LabelTheme, PDFPageType,
            },
            select_option::{vec_into, vec_into_enum, vec_into_enum_default},
        },
    },
    static_concat,
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
    let mut presets = use_loader(move || get_user_preset())?;

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
            .with_only_allowed_chars(Const::BARCODE_ALLOWED_CHARS),
    );

    let (mut preset_name, mut preset_name_error, preset_name_validator) = (
        use_signal(|| "".to_string()),
        use_signal(|| None),
        Validator::<String>::create()
            .with_not_empty()
            .with_max_length(24)
            .no_duplicates(presets().iter().map(|f| f.name.clone()).collect()),
    );
    use_memo(move || {
        preset_name_error.set(preset_name_validator.validate(&preset_name()));
    });

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

    let save_preset = move || async move {
        match save_label_preset(preset_name(), options()).await {
            Ok(added) => {
                if !added {
                    preset_name_error.set(Some("Failed to add".to_string()));
                } else {
                    presets.restart(); // Refetch
                    preset_name.set("".to_string());
                }
            }
            Err(e) => {
                preset_name_error.set(Some(format!("Failed to add {}", e)));
            }
        }
    };

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
            Select {
                label: "Check Digit".to_string(),
                options: vec_into_enum::<LabelCheckDigit>(),
                selected: options().check_digit as i64,
                onchange: move |evt: Event<FormData>| {
                    options.write().check_digit = LabelCheckDigit::from(
                        evt.value().parse::<i64>().unwrap_or_default(),
                    );
                },
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
                label: "Border Stroke".to_string(),
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
                label: "Border Radius".to_string(),
                oninput: move |evt: Event<FormData>| {
                    options.write().radius_outer = evt.value().parse().unwrap_or_default();
                },
                min: 0,
                max: 8,
                step: 0.5,
                value: options().radius_outer,
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
                label: "Barcode Height".to_string(),
                oninput: move |evt: Event<FormData>| {
                    options.write().barcode_height = evt.value().parse().unwrap_or_default();
                },
                min: 0.0,
                max: 15,
                step: 0.1,
                value: options().barcode_height,
            }
        }
    };

    let text_tab = rsx! {
        form { class: Css::FORM_GRID,
            Input {
                type_: InputType::Number,
                label: "Width (mm)".to_string(),
                oninput: move |evt: Event<FormData>| {
                    options.write().text_box_width = evt.value().parse().unwrap_or_default();
                },
                min: 3,
                max: 11.2,
                step: 0.1,
                value: options().text_box_width,
            }
            Input {
                type_: InputType::Number,
                label: "Height (mm)".to_string(),
                oninput: move |evt: Event<FormData>| {
                    options.write().text_box_height = evt.value().parse().unwrap_or_default();
                },
                min: 3,
                max: 16.5,
                step: 0.1,
                value: options().text_box_height,
            }
            Input {
                type_: InputType::Number,
                label: "Border Radius".to_string(),
                oninput: move |evt: Event<FormData>| {
                    options.write().text_box_radius = evt.value().parse().unwrap_or_default();
                },
                min: 0,
                max: 8,
                step: 0.5,
                value: options().text_box_radius,
            }
            Input {
                type_: InputType::Number,
                label: "Border Stroke".to_string(),
                oninput: move |evt: Event<FormData>| {
                    options.write().text_box_stroke = evt.value().parse().unwrap_or_default();
                },
                min: 0,
                max: 2,
                step: 0.1,
                value: options().text_box_stroke,
            }

            Input {
                type_: InputType::Number,
                label: "X offset (mm)".to_string(),
                oninput: move |evt: Event<FormData>| {
                    options.write().text_box_x_offset = evt.value().parse().unwrap_or_default();
                },
                min: -10,
                max: 10,
                step: 0.1,
                value: format!("{:.1}", options().text_box_x_offset),
            }
            Input {
                type_: InputType::Number,
                label: "Y Offset (mm)".to_string(),
                oninput: move |evt: Event<FormData>| {
                    options.write().text_box_y_offset = evt.value().parse().unwrap_or_default();
                },
                min: -10,
                max: 10,
                step: 0.1,
                value: format!("{:.1}", options().text_box_y_offset),
            }
            Input {
                type_: InputType::Number,
                label: "Font Size".to_string(),
                oninput: move |evt: Event<FormData>| {
                    options.write().text_box_font_size = evt.value().parse().unwrap_or_default();
                },
                min: 1,
                max: 10,
                step: 0.5,
                value: format!("{:.1}", options().text_box_font_size),
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
                label: "Left Margin (mm)".to_string(),
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
                label: "Top Margin (mm)".to_string(),
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
                label: "Column Spacing (mm)".to_string(),
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
                label: "Row Spacing (mm)".to_string(),
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
                style: "min-width: 16rem",
                class: static_concat!(Css::FLEX_COL),
                Tab {
                    labels: vec![
                        TabLabel::new("Label", None),
                        TabLabel::new("Style", None),
                        TabLabel::new("Text", None),
                        TabLabel::new("Page", None),
                    ],
                    grow_tabs: true,
                    contents: vec![label_tab, style_tab, text_tab, page_tab],
                }
                div {
                    class: static_concat!(Css::FLEX_ROW, Css::FLEX_CENTRE),
                    style: "margin-top: auto",
                    Button {
                        text: "Download".to_string(),
                        icon: Icons::PDF,
                        primary: true,
                        onclick: move |_| async move {
                            js_download_file(GENERATE_PDF_LABEL_DOWNLOAD, json!({ "options" : options() }));
                        },
                    }
                    Button {
                        text: "Default".to_string(),
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
                    DropDown {
                        anchor_id: Id::GenerateLabelPresetMenu.as_str(),
                        anchor: rsx! {
                            FakeButton { icon: Icons::STAR }
                        },
                        top: true,
                        for (index , preset) in presets().iter().enumerate() {
                            div {
                                class: [Css::ICON_LIST_ITEM, Css::FLEX_ROW, Css::FLEX_ALIGN_CENTRE].concat(),
                                onclick: move |_evt: Event<MouseData>| async move {
                                    if let Some(found) = presets().get(index) {
                                        options.set(found.options.clone());
                                        for (index, ty) in types().iter().enumerate() {
                                            if ty.designation == found.options.designation {
                                                selected_type.set(index as i64);
                                                break;
                                            }
                                        }
                                    }

                                },
                                span { "{preset.name}" }
                                span {
                                    class: static_concat!(
                                        Css::ICON, Css::SM, Icons::X_MARK, Css::FLOAT_RIGHT, Css::ICON_LIST_BTN
                                    ),
                                    onclick: move |evt: Event<MouseData>| async move {
                                        evt.stop_propagation(); // Stop call parent onclick above
                                        if let Some(found) = presets().get(index)
                                            && let Ok(_del) = del_label_preset(found.id).await // Refetch
                                        {
                                            presets.restart();
                                        }
                                    },
                                }
                            }
                        }
                        hr {}
                        div {
                            class: [Css::ICON_LIST_ITEM, Css::FLEX_ROW, Css::FLEX_ALIGN_CENTRE].concat(),
                            onclick: move |_evt: Event<MouseData>| async move {
                                js_show_modal(Id::GenerateLabelPresetDialog.as_str());
                            },
                            span { class: static_concat!(Css::ICON, Css::SM, Icons::SAVE) }
                            span { "Save preset" }
                        }
                    }
                }
            }
            Dialog {
                id: Id::GenerateLabelPresetDialog.as_str(),
                button_bar: true,
                button_bar_save: true,
                button_bar_close: true,
                button_bar_save_disabled: preset_name_error().is_some(),
                button_bar_save_click: move |_evt: Event<MouseData>| async move { save_preset().await },
                form {
                    class: Css::FORM_GRID,
                    onsubmit: move |evt| {
                        evt.prevent_default(); // Refresh on Enter
                    },
                    Input {
                        type_: InputType::Text,
                        label: "Preset Name".to_string(),
                        oninput: move |evt: Event<FormData>| {
                            preset_name.set(evt.value());
                        },
                        onkeydown: move |evt: Event<KeyboardData>| async move {
                            if evt.code() == Code::Enter && preset_name_error().is_none() {
                                save_preset().await;
                                js_close(Id::GenerateLabelPresetDialog.as_str());
                            }
                        },
                        value: preset_name(),
                        validation: preset_name_error,
                    }
                }
            }
            Card {
                div { class: [Css::PDF_PREVIEW, options().font.as_str()].concat(),
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
