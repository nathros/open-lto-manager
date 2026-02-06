use crate::backend::api::api_file_view::{fv_files_in_dir, fv_working_dir};
use crate::shared::models::file_view::FileView;
use dioxus::{fullstack::Loader, prelude::*};
use std::path::PathBuf;

#[component]
pub fn FileViewer(mut selected_files: Signal<Vec<PathBuf>>) -> Element {
    let mut current_path: Loader<String> = use_loader(fv_working_dir)?;

    let mut current_path_input: Signal<String> = use_signal(|| current_path());

    let current_path_update = move |evt: Event<FormData>| {
        info!("{}", evt.value());
        current_path_input.set(evt.value());
    };

    let apply = move |_| {
        info!("apply {}", current_path.is_error());
        current_path.set(current_path_input());
    };

    rsx! {
        div {
            input {
                style: "width:calc(100% - 4rem)",
                onchange: current_path_update,
                value: current_path(),
            }
            button { onclick: apply, "apply" }
            br {}
            br {}
        }

        FileViewerBody { current_path }

    }
}

#[component]
pub fn FileViewerBody(mut current_path: Loader<String>) -> Element {
    info!("Render FileViewerBody");
    let files_loader: Loader<Result<Vec<FileView>, String>> =
        use_loader(move || fv_files_in_dir(current_path()))?;

    rsx! {
        div { style: "width: 100%",
            if let Ok(files) = files_loader() {
                for file in files {
                    if file.is_dir {
                        if let file_name_clone = file.file_name.clone() {
                            div {
                                style: "cursor: pointer;",
                                onclick: move |_| {
                                    current_path.set(format!("{}/{}", current_path(), file_name_clone));
                                },
                                span { dangerous_inner_html: "&#128193; {file.file_name}" }
                                span { style: "", "" }
                            }
                        }
                    } else {
                        div {
                            span { dangerous_inner_html: " &#128240; {file.file_name}" }
                            span { style: "float:right", "size: {file.size}" }
                        }
                    }
                }
            } else if let Err(f) = files_loader() {
                p { style: "color:blue", "vvv {f}" }
            }
        }
    }
}
