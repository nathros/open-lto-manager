use crate::{
    frontend::{css::Css, icons::Icons},
    route::Route,
};

use super::menu_config::{MenuConfig, MenuGroup, MenuItemConfig};

impl MenuConfig {
    pub fn default_aside(current_route: &String) -> MenuConfig {
        // FIXME swap between library and jobs, icon wobble when only 1 child item in both
        MenuConfig {
            enable_search: true,
            groups: vec![
                MenuGroup::new(
                    current_route,
                    Icons::HOME.into(),
                    "Home".to_string(),
                    vec![MenuItemConfig {
                        icon: "".to_string(),
                        label: "Home".to_string(),
                        link: Route::Home {}.to_string(),
                    }],
                ),
                MenuGroup::new(
                    current_route,
                    Css::ICON_TAPE.into(),
                    "Library".to_string(),
                    vec![
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "View".to_string(),
                            link: Route::ViewLibrary {}.to_string(),
                        },
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "Add Tape".to_string(),
                            link: Route::Tape { id: (0) }.to_string(),
                        },
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "Generate LTO Label".to_string(),
                            link: Route::GenLabel {}.to_string(),
                        },
                    ],
                ),
                MenuGroup::new(
                    current_route,
                    Icons::LIST.into(),
                    "Jobs".to_string(),
                    vec![MenuItemConfig {
                        icon: "".to_string(),
                        label: "Add Job".to_string(),
                        link: Route::AddJob {}.to_string(),
                    }],
                ),
                MenuGroup::new(
                    current_route,
                    Icons::WARNING.into(),
                    "System".to_string(),
                    vec![
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "Show devices".to_string(),
                            link: Route::ShowDevices {}.to_string(),
                        },
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "Login sessions".to_string(),
                            link: Route::Sessions {}.to_string(),
                        },
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "Diagnostics".to_string(),
                            link: Route::Diagnostics {}.to_string(),
                        },
                    ],
                ),
                #[cfg(debug_assertions)]
                MenuGroup::new(
                    current_route,
                    Icons::BUG.into(),
                    "Debug".to_string(),
                    vec![
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "Test".to_string(),
                            link: Route::Test {}.to_string(),
                        },
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "Show".to_string(),
                            link: Route::Show {}.to_string(),
                        },
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "Dev".to_string(),
                            link: Route::ShowDev {}.to_string(),
                        },
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "User".to_string(),
                            link: Route::DBUser {}.to_string(),
                        },
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "Type".to_string(),
                            link: Route::DBType {}.to_string(),
                        },
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "Tape".to_string(),
                            link: Route::DBTape {}.to_string(),
                        },
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "File".to_string(),
                            link: Route::DBFile {}.to_string(),
                        },
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "Job".to_string(),
                            link: Route::DBJob {}.to_string(),
                        },
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "Job Metadata".to_string(),
                            link: Route::DBJobMetaData {}.to_string(),
                        },
                    ],
                ),
                #[cfg(debug_assertions)]
                MenuGroup::new(
                    current_route,
                    Icons::SANDPIT.into(),
                    "Sandpit".to_string(),
                    vec![
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "Index".to_string(),
                            link: Route::Sandpit {
                                name: "".to_string(),
                            }
                            .to_string(),
                        },
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "Showcase".to_string(),
                            link: Route::Sandpit {
                                name: "showcase".to_string(),
                            }
                            .to_string(),
                        },
                    ],
                ),
            ],
        }
    }
}
