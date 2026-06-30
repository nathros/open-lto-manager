use std::mem::discriminant;

use crate::{
    frontend::{css::Css, icons::Icons},
    route::Route,
};

use super::menu_config::{MenuConfig, MenuGroup, MenuItemConfig};

fn route_eq<T>(a: &T, b: &T) -> bool {
    discriminant(a) == discriminant(b)
}

impl MenuConfig {
    pub fn default_aside(route: &Route) -> MenuConfig {
        // FIXME swap between library and jobs, icon wobble
        MenuConfig {
            enable_search: true,
            groups: vec![
                MenuGroup {
                    icon: Icons::HOME.into(),
                    label: "Home".to_string(),
                    open: route_eq(route, &Route::Home {}),
                    items: vec![MenuItemConfig {
                        icon: "".to_string(),
                        label: "Home".to_string(),
                        link: Route::Home {}.to_string(),
                        selected: route_eq(route, &Route::Home {}),
                    }],
                },
                MenuGroup {
                    icon: Css::ICON_TAPE.into(),
                    label: "Library".to_string(),
                    open: route_eq(route, &Route::Tape { id: (0) }),
                    items: vec![MenuItemConfig {
                        icon: "".to_string(),
                        label: "Add Tape".to_string(),
                        link: Route::Tape { id: (0) }.to_string(),
                        selected: route_eq(route, &Route::Tape { id: (0) }),
                    }],
                },
                MenuGroup {
                    icon: Icons::LIST.into(),
                    label: "Jobs".to_string(),
                    open: route_eq(route, &Route::AddJob {}),
                    items: vec![MenuItemConfig {
                        icon: "".to_string(),
                        label: "Add Job".to_string(),
                        link: Route::AddJob {}.to_string(),
                        selected: route_eq(route, &Route::AddJob {}),
                    }],
                },
                MenuGroup {
                    icon: Icons::WARNING.into(),
                    label: "System".to_string(),
                    open: route_eq(route, &Route::Sessions {})
                        || route_eq(route, &Route::ShowDevices {}),
                    items: vec![
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "Show devices".to_string(),
                            link: Route::ShowDevices {}.to_string(),
                            selected: route_eq(route, &Route::ShowDevices {}),
                        },
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "Login sessions".to_string(),
                            link: Route::Sessions {}.to_string(),
                            selected: route_eq(route, &Route::Sessions {}),
                        },
                    ],
                },
                #[cfg(debug_assertions)]
                MenuGroup {
                    icon: Icons::BUG.into(),
                    label: "Debug".to_string(),
                    open: route_eq(route, &Route::Test {})
                        || route_eq(route, &Route::Show {})
                        || route_eq(route, &Route::ShowDev {})
                        || route_eq(route, &Route::DBUser {})
                        || route_eq(route, &Route::DBType {})
                        || route_eq(route, &Route::DBFile {})
                        || route_eq(route, &Route::DBTape {})
                        || route_eq(route, &Route::DBJob {})
                        || route_eq(route, &Route::DBJobMetaData {})
                        || route_eq(route, &Route::ShowAppState {}),
                    items: vec![
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "Test".to_string(),
                            link: Route::Test {}.to_string(),
                            selected: route_eq(route, &Route::Test {}),
                        },
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "Show".to_string(),
                            link: Route::Show {}.to_string(),
                            selected: route_eq(route, &Route::Show {}),
                        },
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "Dev".to_string(),
                            link: Route::ShowDev {}.to_string(),
                            selected: route_eq(route, &Route::ShowDev {}),
                        },
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "User".to_string(),
                            link: Route::DBUser {}.to_string(),
                            selected: route_eq(route, &Route::DBUser {}),
                        },
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "Type".to_string(),
                            link: Route::DBType {}.to_string(),
                            selected: route_eq(route, &Route::DBType {}),
                        },
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "Tape".to_string(),
                            link: Route::DBTape {}.to_string(),
                            selected: route_eq(route, &Route::DBTape {}),
                        },
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "File".to_string(),
                            link: Route::DBFile {}.to_string(),
                            selected: route_eq(route, &Route::DBFile {}),
                        },
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "Job".to_string(),
                            link: Route::DBJob {}.to_string(),
                            selected: route_eq(route, &Route::DBJob {}),
                        },
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "Job Metadata".to_string(),
                            link: Route::DBJobMetaData {}.to_string(),
                            selected: route_eq(route, &Route::DBJobMetaData {}),
                        },
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "App State".to_string(),
                            link: Route::ShowAppState {}.to_string(),
                            selected: route_eq(route, &Route::ShowAppState {}),
                        },
                    ],
                },
                #[cfg(debug_assertions)]
                MenuGroup {
                    icon: Icons::SANDPIT.into(),
                    label: "Sandpit".to_string(),
                    open: route_eq(route, &Route::Sandpit {})
                        || route_eq(route, &Route::SandpitShowcase {}),
                    items: vec![
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "Index".to_string(),
                            link: Route::Sandpit {}.to_string(),
                            selected: route_eq(route, &Route::Sandpit {}),
                        },
                        MenuItemConfig {
                            icon: "".to_string(),
                            label: "Showcase".to_string(),
                            link: Route::SandpitShowcase {}.to_string(),
                            selected: route_eq(route, &Route::SandpitShowcase {}),
                        },
                    ],
                },
            ],
        }
    }
}
