use dioxus::prelude::*;

pub const FAVICON: Asset = asset!("/assets/logo.svg");

// CSS
#[cfg(not(debug_assertions))] // Release build combined CSS files
pub const CSS_ASSETS: [Asset; 1] = [asset!("/assets/bundle.css")];

// Release (build.rs) will combine all these files into single bundle.css
#[cfg(debug_assertions)] // Debug build individual CSS files
pub const CSS_ASSETS: [Asset; 4] = [
    asset!("/assets/common.css"),
    asset!("/assets/font/font.css"),
    asset!("/assets/main.css"),
    asset!("/assets/button.css"),
];

// Fonts
const _FONT_LATO_N_400: Asset = asset!(
    "/assets/font/lato-v25-normal-400.woff2",
    AssetOptions::builder().with_hash_suffix(false) // Disable hash as referenced in CSS
);
const _FONT_LATO_I_400: Asset = asset!(
    "/assets/font/lato-v25-italic-400.woff2",
    AssetOptions::builder().with_hash_suffix(false) // Disable hash as referenced in CSS
);
const _FONT_LATO_N_700: Asset = asset!(
    "/assets/font/lato-v25-normal-700.woff2",
    AssetOptions::builder().with_hash_suffix(false) // Disable hash as referenced in CSS
);
const _FONT_LATO_I_700: Asset = asset!(
    "/assets/font/lato-v25-italic-700.woff2",
    AssetOptions::builder().with_hash_suffix(false) // Disable hash as referenced in CSS
);
const _FONT_JETBRAINS_N: Asset = asset!(
    "/assets/font/jetbrains-mono-v24-normal-100-800.woff2",
    AssetOptions::builder().with_hash_suffix(false) // Disable hash as referenced in CSS
);
const _FONT_JETBRAINS_I: Asset = asset!(
    "/assets/font/jetbrains-mono-v24-italic-100-800.woff2",
    AssetOptions::builder().with_hash_suffix(false) // Disable hash as referenced in CSS
);
