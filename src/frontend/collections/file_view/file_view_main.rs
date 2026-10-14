use dioxus::{fullstack::Loader, prelude::*};

use std::path;

use crate::{
    backend::api::{
        api_file_view::{fv_physical_files_in_dir, fv_virtual_files_in_dir, fv_working_dir},
        api_user::update_user,
    },
    either,
    frontend::{
        collections::file_view::file_view_breadcrumb::Breadcrumb,
        components::dropdown::component::DropDown,
        css::Css,
        elements::{
            button::{Button, FakeButton},
            input::{Input, InputType},
            loading::Loading,
        },
        icons::Icons,
        id::Id,
    },
    shared::models::{
        database::user::model_user::{FileTheme, RecordUserConfig},
        file_view::FileView,
    },
    static_concat,
};

#[derive(Props, PartialEq, Clone)]
pub struct FileViewerProps {
    #[props]
    is_virtual: bool,

    #[props(extends = button, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}

#[component]
pub fn FileViewer(props: FileViewerProps) -> Element {
    let mut current_path = use_signal(|| {
        either!(
            props.is_virtual,
            path::MAIN_SEPARATOR.to_string(),
            "".to_string()
        )
    });

    let mut current_user = use_context::<Loader<Option<RecordUserConfig>>>();
    let mut show_edit = use_signal(|| false);
    let mut edit_path = use_signal(|| current_path());

    let mut breadcrumbs = use_signal(move || Breadcrumb {
        current_index: (current_path()
            .chars()
            .filter(|c| *c == path::MAIN_SEPARATOR)
            .count()) as isize
            - 1,
        items_path: current_path(),
        items: Breadcrumb::create_tail(current_path(), path::MAIN_SEPARATOR),
    });

    let change_theme = move |theme: FileTheme| async move {
        if let Some(mut user) = current_user() {
            user.file_theme = theme;
            if update_user(user.clone()).await.is_ok() {
                current_user.set(Some(user));
            }
        }
    };

    let mut update_current_path = move |mut new_path: String| {
        if !new_path.ends_with(path::MAIN_SEPARATOR) {
            new_path.push(path::MAIN_SEPARATOR);
        }
        current_path.set(new_path.clone());
        breadcrumbs.write().update(new_path, path::MAIN_SEPARATOR);
    };

    let mut fetch_dir: Resource<FileView> = use_resource(move || async move {
        if props.is_virtual {
            fv_virtual_files_in_dir(current_path())
                .await
                .unwrap_or_default()
        } else {
            if current_path().is_empty() {
                update_current_path(fv_working_dir().await.unwrap_or_default());
            }
            fv_physical_files_in_dir(current_path())
                .await
                .unwrap_or_default()
        }
    });

    let str_cat = |path: &mut String, add: &String| -> String {
        path.push_str(add);
        if *add != path::MAIN_SEPARATOR.to_string() {
            path.push(path::MAIN_SEPARATOR);
        }
        path.to_string()
    };

    rsx! {
        if let file_theme = current_user().unwrap_or_default().file_theme
            && let theme_svg = file_theme.as_file_theme()
        {
            div { class: Css::FILE_VIEWER,
                if let index = breadcrumbs().current_index && let mut trail_paths = "".to_string() {
                    div { class: [Css::FILE_VIEWER_BREADCRUMB, Css::GAP_S].concat(),
                        Button {
                            icon: Icons::DIR_UP,
                            onclick: move |_| {
                                update_current_path(breadcrumbs().get_parent_dir(path::MAIN_SEPARATOR));
                            },
                        }
                        div {
                            if show_edit() {
                                Input {
                                    type_: InputType::Text,
                                    oninput: move |evt: Event<FormData>| {
                                        edit_path.set(evt.value());
                                    },
                                    onkeydown: move |evt: Event<KeyboardData>| {
                                        if evt.code() == Code::Enter {
                                            update_current_path(edit_path());
                                            show_edit.set(false);
                                        } else if evt.code() == Code::Escape {
                                            show_edit.set(false);
                                        }
                                    },
                                    onmounted: move |element: Event<MountedData>| async move {
                                        let _ = element.set_focus(true).await;
                                    },
                                    value: edit_path(),
                                }
                            } else {
                                for (i , item) in breadcrumbs().items.iter().enumerate() {
                                    if let p = str_cat(&mut trail_paths, item) {
                                        Button {
                                            text: item,
                                            primary: i as isize == index,
                                            onclick: move |_evt: MouseEvent| {
                                                update_current_path(p.clone());
                                            },
                                        }
                                    }
                                }
                            }
                        }
                        Button {
                            icon: either!(show_edit(), Icons::CHECK, Icons::EDIT),
                            primary: show_edit(),
                            onclick: move |_| async move {
                                if show_edit() {
                                    show_edit.set(false);
                                    update_current_path(edit_path()); // When check fetch dir
                                } else {
                                    show_edit.set(true);
                                    edit_path.set(current_path());
                                }
                            },
                        }
                        span {
                            DropDown {
                                anchor_id: either!(
                                    props.is_virtual, Id::FileViewerSettingsAnchorVirtual.as_str(),
                                    Id::FileViewerSettingsAnchorPhysical.as_str()
                                ),
                                anchor: rsx! {
                                    FakeButton { icon: Icons::COG }
                                },
                                h4 { style: "padding-left:var(--sm)", "Theme" }
                                hr {}
                                div {
                                    class: [
                                        Css::ICON_LIST_ITEM,
                                        Css::FLEX_ROW,
                                        Css::FLEX_ALIGN_CENTRE,
                                        either!(file_theme == FileTheme::Breeze, Css::SELECTED, ""),
                                    ]
                                        .concat(),
                                    onclick: move |_evt: Event<MouseData>| async move {
                                        change_theme(FileTheme::Breeze).await;
                                    },
                                    span { class: static_concat!(Css::ICON, Css::SM, Icons::PALETTE) }
                                    span { "Breeze" }
                                }
                                div {
                                    class: [
                                        Css::ICON_LIST_ITEM,
                                        Css::FLEX_ROW,
                                        Css::FLEX_ALIGN_CENTRE,
                                        either!(file_theme == FileTheme::Papirus, Css::SELECTED, ""),
                                    ]
                                        .concat(),
                                    onclick: move |_evt: Event<MouseData>| async move {
                                        change_theme(FileTheme::Papirus).await;
                                    },
                                    span { class: static_concat!(Css::ICON, Css::SM, Icons::PALETTE) }
                                    span { "Papirus" }
                                }
                                div {
                                    class: [
                                        Css::ICON_LIST_ITEM,
                                        Css::FLEX_ROW,
                                        Css::FLEX_ALIGN_CENTRE,
                                        either!(file_theme == FileTheme::Kora, Css::SELECTED, ""),
                                    ]
                                        .concat(),
                                    onclick: move |_evt: Event<MouseData>| async move {
                                        change_theme(FileTheme::Kora).await;
                                    },
                                    span { class: static_concat!(Css::ICON, Css::SM, Icons::PALETTE) }
                                    span { "Kora" }
                                }
                            }
                        }
                        Button {
                            icon: Icons::REFRESH,
                            onclick: move |_| {
                                fetch_dir.clear();
                                gloo_timers::callback::Timeout::new(
                                        333, // Prevent too fast refresh
                                        move || {
                                            fetch_dir.restart();
                                        },
                                    )
                                    .forget();
                            },
                        }
                    }
                }
                match &*fetch_dir.read_unchecked() {
                    Some(dir) => {
                        rsx! {
                            if let Some(_found_dir) = &dir.current_dir
                            {
                                table {
                                    tr {
                                        th { "File" }
                                        th { "Size" }
                                        th { "Modified" }
                                    }
                                    for (index , child) in dir.children.clone().iter().enumerate() {
                                        if let is_dir = child.is_dir && let move_path = child.file_name.clone() {
                                            tr {
                                                class: either!(child.is_selected, Css::SELECTED, ""),
                                                ondoubleclick: move |_| {
                                                    if is_dir {
                                                        update_current_path(format!("{}{}", current_path(), move_path));
                                                    }
                                                },
                                                onclick: move |_| {
                                                    fetch_dir
                                                        .with_mut(|opt_f| {
                                                            if let Some(fv) = opt_f {
                                                                fv.children
                                                                    .iter_mut()
                                                                    .enumerate()
                                                                    .for_each(|(i, f)| {
                                                                        f.is_selected = i == index;
                                                                    })
                                                            }
                                                        });
                                                },
                                                td {
                                                    div { class: [Css::FLEX_ROW, Css::FLEX_ALIGN_CENTRE, Css::GAP_S].concat(),
                                                        img {
                                                            class: Css::MD,
                                                            src: format!("{}{}", theme_svg, child.icon),
                                                        }
                                                        span { "{child.file_name}" }
                                                    }
                                                }
                                                td { "{child.pretty_size()}" }
                                                td { "{child.modified}" }
                                            }
                                        }
                                    }
                                }
                                if dir.children.is_empty() {
                                    h2 { "Dir empty" }
                                }
                            } else {
                                h2 { "Failed find dir" }
                            }
                        }
                    }
                    None => rsx! {
                        Loading {}
                    },
                }
            }
        }
        Debug { current_path: current_path(), breadcrumb: breadcrumbs() }
    }
}

#[cfg(debug_assertions)]
#[component]
fn Debug(current_path: ReadSignal<String>, breadcrumb: ReadSignal<Breadcrumb>) -> Element {
    use crate::frontend::components::header::{
        header_extra::HeaderExtraIcons, header_icon::HeaderIcon,
    };

    rsx! {
        HeaderExtraIcons {
            HeaderIcon { button_id: "dbg_btn", menu_id: "dbg_menu", icon: Icons::BUG,
                div { class: Css::DEBUG_MENU, style: "width:20rem",
                    h2 { margin: false, "Debug" }
                    p { "current_path: {current_path}" }
                    hr {}
                    p { "breadcrumb: {breadcrumb:?}" }
                }
            }
        }
    }
}

#[cfg(not(debug_assertions))]
#[component]
fn Debug(current_path: ReadSignal<String>, breadcrumb: ReadSignal<Breadcrumb>) -> Element {
    rsx! {} // Nothing for release build
}
