use dioxus::prelude::*;

use crate::{
    frontend::{
        assets::IMG_SANDPIT,
        components::card::Card,
        css::Css,
        elements::{
            button::Button,
            heading::{H2, H3},
        },
        sandpit::{
            sandpit_accordion::SandpitAccordion, sandpit_button::SandpitButton,
            sandpit_floating_message::SandpitFloating, sandpit_input::SandpitInput,
            sandpit_menu::SandpitMenu, sandpit_menu_item::SandpitMenuItem,
            sandpit_message::SandpitMessage, sandpit_modal::SandpitModal,
            sandpit_radio_pill::SandpitRadioPill, sandpit_select::SandpitSelect,
            sandpit_tab::SandpitTab,
        },
    },
    route::Route,
    static_concat,
};

#[component]
pub fn Sandpit(name: String) -> Element {
    let all_items = [
        (
            "UI Elements",
            vec![
                ("Button", SandpitButton()),
                ("Menu Item", SandpitMenuItem()),
                ("Input", SandpitInput()),
                ("Select", SandpitSelect()),
            ],
        ),
        (
            "UI Modules",
            vec![
                ("Accordion", SandpitAccordion()),
                ("Modal", SandpitModal()),
                ("Tab", SandpitTab()),
            ],
        ),
        (
            "UI Collections",
            vec![
                ("Message", SandpitMessage()),
                ("Radio Pill", SandpitRadioPill()),
                ("Floating", SandpitFloating()),
            ],
        ),
        ("UI Components", vec![("Menu", SandpitMenu())]),
    ];

    let style = "width:100%;align-items:unset";
    let background = || {
        rsx! {
            div {
                id: Css::ID_SAND,
                style: format!("background-image:url({})", IMG_SANDPIT),
            }
        }
    };
    let showDemo = |name_group: &str, component: Result<VNode, RenderError>| {
        rsx! {
            Card { top_padding: false,
                H3 { margin: true, "{name_group}" }
                div { {component} }
            }
        }
    };

    rsx! {
        div { class: static_concat!(Css::FLEX_COL, Css::FLEX_ALIGN_LEFT),
            match name.as_str() {
                "" => rsx! {
                    {background()}
                    H2 { margin: true, "Sandpit: Dev Testing Area" }
                    div { class: Css::CARD,
                        b { "Showcase" }
                        br {}
                        br {}
                        div { class: Css::FLEX_ROW,
                            Button {
                                primary: true,
                                onclick: move |_| {
                                    use_navigator()
                                        .push(Route::Sandpit {
                                            name: "showcase".to_string(),
                                        });
                                },
                                text: "UI Showcase",
                            }
                        }
                    }

                    div { class: Css::FLEX_ROW,
                        for (set_name , groups) in all_items {
                            div { class: Css::CARD,
                                b { "{set_name}" }
                                br {}
                                br {}
                                div { class: Css::FLEX_ROW,
                                    for (name_group , _) in groups {
                                        Button {
                                            onclick: move |_| {
                                                use_navigator()
                                                    .push(Route::Sandpit {
                                                        name: name_group.to_string(),
                                                    });
                                            },
                                            text: name_group,
                                        }
                                    }
                                }
                            }
                        }
                    }

                },

                "showcase" => rsx! {
                    for (_ , groups) in all_items {
                        div { class: Css::FLEX_COL, style,
                            for (name_group , component) in groups {
                                {showDemo(name_group, component)}
                            }
                        }
                    }
                },

                _ => rsx! {
                    {background()}
                    for (_ , groups) in all_items {
                        for (name_group , component) in groups {
                            if name_group == name {
                                div { class: Css::FLEX_COL, style, {showDemo(name_group, component)} }
                            }
                        }
                    }
                },
            }
        }
    }
}
