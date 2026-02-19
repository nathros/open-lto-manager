use std::collections::HashSet;

use crate::backend::api::api_file_view::{fv_files_in_dir, fv_working_dir};
use crate::shared::models::file_view::FileView;
use dioxus::fullstack::Loader;
use dioxus::prelude::*;

#[component]
pub fn FileViewer(mut selected_files: WriteSignal<HashSet<String>>) -> Element {
    let mut current_path: Signal<String> = use_signal(|| "".to_string());

    let mut current_path_input: Signal<String> = use_signal(|| current_path());

    let current_path_update = move |evt: Event<FormData>| {
        info!("{}", evt.value());
        current_path_input.set(evt.value());
    };

    let apply = move |_| {
        info!("apply {} = {}", current_path, current_path_input);
        current_path.set(current_path_input());
    };

    let res = use_resource(move || async move {
        let from_server = use_loader(fv_working_dir);
        match from_server {
            Ok(o) => {
                current_path.set(o());
                true
            }
            Err(_loading) => {
                current_path.set("".to_string());
                false
            }
        }
    });
    let pending = res.peek().unwrap_or(false);

    rsx! {
        div {
            input {
                style: "width:calc(100% - 4rem)",
                onchange: current_path_update,
                value: "{current_path}",
                disabled: !pending,
            }
            button { onclick: apply, "apply" }
            br {}
            br {}
        }

        SuspenseBoundary {
            fallback: |_| rsx! {
                p { "fetching" }
            },
            FileViewerBody2 { current_path, selected_files }
        }
    }
}

#[component]
fn FileViewerBody2(
    mut current_path: Signal<String>,
    mut selected_files: WriteSignal<HashSet<String>>,
) -> Element {
    info!("render");
    let mut loader: Loader<Result<Vec<FileView>, String>> =
        use_loader(move || fv_files_in_dir(current_path(), false, 0))?;

    rsx! {
        div { style: "width: 100%",
            if let Ok(dir) = loader() {
                for (index , file) in dir.into_iter().enumerate() {
                    if !file.hidden && let file_path = file.path.clone() {
                        if file.is_dir {
                            div { style: "cursor: pointer;padding-left:{file.nest * 20}px",
                                button {
                                    r#type: "button",
                                    onclick: move |_| {
                                        info!("click: {}", file_path);
                                        spawn(async move {
                                            if let Ok(mut dir) = loader() {
                                                FileView::toggle_dir(&mut dir, index).await;
                                                loader.set(Ok(dir));
                                            }
                                        });
                                    },
                                    if file.expanded {
                                        "–"
                                    } else {
                                        "+"
                                    }
                                }
                                input {
                                    r#type: "checkbox",
                                    oninput: move |event: Event<FormData>| {
                                        if let Ok(mut dir) = loader() {
                                            for f in dir.iter_mut().skip(index + 1) {
                                                if f.nest > file.nest {
                                                    if !f.is_dir {
                                                        f.selected = event.checked();
                                                    }
                                                } else {
                                                    break;
                                                }
                                            }
                                            if let Some(m) = dir.get_mut(index) {
                                                m.selected = event.checked();
                                            }
                                            selected_files
                                                .set(
                                                    HashSet::from_iter(
                                                        dir.iter().filter(|d| d.selected).map(|f| f.path.clone()),
                                                    ),
                                                );
                                            loader.set(Ok(dir));
                                        }
                                        info!("Selected {}", event.checked());
                                    },
                                }
                                span {
                                    dangerous_inner_html: "&#128193; {index} {file.nest} {file.name}",
                                    onclick: move |_| {
                                        current_path.set(file.path.clone());
                                    },
                                }
                                span { style: "float:right", "count: {file.size}" }

                            }
                        } else {
                            div { style: "padding-left:{file.nest * 20}px",
                                button { style: "opacity:0;", r#type: "button", "+" }

                                input {
                                    r#type: "checkbox",
                                    oninput: move |event: Event<FormData>| {
                                        if let Ok(mut dir) = loader() && let Some(f) = dir.get_mut(index) {
                                            f.selected = event.checked();
                                            selected_files
                                                .set(
                                                    HashSet::from_iter(
                                                        dir.iter().filter(|d| d.selected).map(|f| f.path.clone()),
                                                    ),
                                                );
                                            loader.set(Ok(dir));
                                        }
                                    },
                                    checked: file.selected,
                                }

                                span { dangerous_inner_html: " &#128240; {index} {file.nest} {file.name}" }
                                span { style: "float:right", "size: {file.size}" }
                            }
                        }
                    }
                }
            } else if let Err(f) = loader() {
                p { style: "color:blue", "Error: {f}" }
            }
        }
    }
}
