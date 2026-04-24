use dioxus::prelude::*;

pub const FAVICON: Asset = asset!("/assets/logo.svg");

#[cfg(not(debug_assertions))] // Release build combined CSS files
pub const CSS_ASSETS: [Asset; 1] = [asset!("/assets/bundle.css")];

// Release (build.rs) will combine all these files into single bundle.css
#[cfg(debug_assertions)] // Debug build individual CSS files
pub const CSS_ASSETS: [Asset; 3] = [
    asset!("/assets/common.css"),
    asset!("/assets/main.css"),
    asset!("/assets/button.css"),
];
