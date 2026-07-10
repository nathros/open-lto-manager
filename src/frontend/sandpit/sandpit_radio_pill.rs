use dioxus::prelude::*;

use crate::{
    frontend::{
        collections::radio_pill::RadioPill,
        elements::input::{Input, InputType},
    },
    shared::models::select_option::SelectOption,
};

const MIN: i64 = 1;
const MAX: i64 = 10;

fn get_options(size: i64) -> Vec<SelectOption> {
    (MIN..size)
        .map(|id| SelectOption {
            id,
            label: id.to_string(),
        })
        .collect()
}

#[component]
pub fn SandpitRadioPill() -> Element {
    let mut selected_id = use_signal(|| MIN);
    let mut options = use_signal(|| get_options(MIN + MAX / 3));

    rsx! {
        Input {
            type_: InputType::Number,
            label: "Number of options",
            min: MIN,
            max: MAX,
            oninput: move |evt: Event<FormData>| {
                let new_size = evt.value().parse::<i64>().unwrap_or_default();
                if selected_id() >= new_size {
                    selected_id.set(new_size);
                }
                options.set(get_options(new_size + 1));
            },
            value: options().len(),
        }

        br {}

        RadioPill {
            name: "test_pill",
            label: "Pill Label".to_string(),
            options: options(),
            callback: use_callback(move |id: i64| {
                selected_id.set(id);
            }),
            selected: selected_id(),
        }
    }
}
