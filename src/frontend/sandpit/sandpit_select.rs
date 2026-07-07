use dioxus::prelude::*;

use crate::{
    frontend::{css::Css, elements::select::Select},
    shared::models::select_option::SelectOption,
    static_concat,
};

#[component]
pub fn SandpitSelect() -> Element {
    let all_opt = (1..6_i64)
        .map(|f| SelectOption {
            id: f,
            label: format!("Available Option {}", f),
        })
        .collect::<Vec<SelectOption>>();

    let mut current_opt_1 = use_signal(move || 0_i64);
    let mut current_opt_2 = use_signal(move || 1_i64);

    rsx! {
        div { class: static_concat!(Css::FLEX_COL, Css::FLEX_ALIGN_LEFT),
            Select {
                label: "With Required".to_string(),
                required: true,
                options: all_opt.clone(),
                selected: current_opt_1(),
                onchange: move |evt: Event<FormData>| {
                    current_opt_1.set(evt.value().parse::<i64>().unwrap_or(0));
                },
            }
            hr {}
            Select {
                label: "Not Required".to_string(),
                options: all_opt,
                selected: current_opt_2(),
                onchange: move |evt: Event<FormData>| {
                    current_opt_2.set(evt.value().parse::<i64>().unwrap_or(0));
                },
            }
            hr {}
            Select {
                label: "Disabled".to_string(),
                disabled: true,
                options: vec![
                    SelectOption {
                        id: 1,
                        label: "None".to_string(),
                    },
                ],
                selected: current_opt_2(),
            }
        }
    }
}
