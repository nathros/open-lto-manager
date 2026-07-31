use dioxus::{
    html::{geometry::ClientPoint, input_data::MouseButton},
    prelude::*,
};

use crate::frontend::css::Css;

#[component]
pub fn FloatingDebug(children: Element) -> Element {
    let mut start_x = use_signal(|| 255_f64);
    let mut start_y = use_signal(|| 55_f64);

    let mut current_x = use_signal(|| 255_f64);
    let mut current_y = use_signal(|| 55_f64);

    rsx! {
        div {
            class: format!("{}{}", Css::DEBUG_FLOAT, Css::CARD),
            style: "top:{current_y}px;left:{current_x}px",
            onmousemove: move |event| {
                let held_btn = event.held_buttons();
                let pos: ClientPoint = event.client_coordinates();
                if held_btn.len() == 1 && held_btn.contains(MouseButton::Primary) {
                    current_x.set(pos.x - start_x());
                    current_y.set(pos.y - start_y());
                } else {
                    let element_pos = event.element_coordinates();
                    start_x.set(element_pos.x);
                    start_y.set(element_pos.y);
                }
            },
            {children}
        }
    }
}
