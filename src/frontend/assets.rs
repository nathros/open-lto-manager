use dioxus::prelude::*;

pub const APP_NAME: &str = "Open LTO Manager";

pub const FAVICON: Asset = asset!(
    "/assets/logo.svg",
    AssetOptions::builder().with_hash_suffix(false) // Disable hash as referenced in CSS
);

pub const LOGO_ASSET: Asset = asset!(
    "/assets/logos-all.svg",
    AssetOptions::builder().with_hash_suffix(false) // Disable hash as referenced in CSS
);

pub const ICONS_ASSET_REMIX: Asset = asset!(
    "/assets/icons-remix.svg",
    AssetOptions::builder().with_hash_suffix(false) // Disable hash as referenced in CSS
);
pub const ICONS_ASSET_TABLER: Asset = asset!(
    "/assets/icons-tabler.svg",
    AssetOptions::builder().with_hash_suffix(false) // Disable hash as referenced in CSS
);
pub const ICONS_ASSET_ICONOIR: Asset = asset!(
    "/assets/icons-iconoir.svg",
    AssetOptions::builder().with_hash_suffix(false) // Disable hash as referenced in CSS
);
pub const ICONS_ASSET_SARGAM_LINE: Asset = asset!(
    "/assets/icons-sargam.svg",
    AssetOptions::builder().with_hash_suffix(false) // Disable hash as referenced in CSS
);

// CSS
#[cfg(not(debug_assertions))] // Release build combined CSS files
pub const CSS_ASSETS: [Asset; 1] = [asset!(
    "/assets/bundle.css", // Created via build.rs (release only) by combining other CSS_ASSETS array bellow into single file
    AssetOptions::css().with_minify(true)
)];

// Release (build.rs) will combine all these files into single bundle.css
#[cfg(debug_assertions)] // Debug build individual CSS files
pub const CSS_ASSETS: [Asset; 19] = [
    asset!("/assets/css/:root.css"),
    asset!("/assets/font/font.css"),
    asset!("/assets/css/header.css"),
    asset!("/assets/css/accordion.css"),
    asset!("/assets/css/icon-list.css"),
    asset!("/assets/css/icon.css"),
    asset!("/assets/css/icons.css"),
    asset!("/assets/css/forms.css"),
    asset!("/assets/css/input.css"),
    asset!("/assets/css/message.css"),
    asset!("/assets/css/modal.css"),
    asset!("/assets/css/table.css"),
    asset!("/assets/css/card.css"),
    asset!("/assets/css/button.css"),
    asset!("/assets/css/section.css"),
    asset!("/assets/css/login.css"),
    asset!("/assets/css/menu.css"),
    asset!("/assets/css/tape-preview.css"),
    asset!("/assets/css/common.css"),
];

// JS
#[cfg(not(debug_assertions))] // Release build combined JS files
pub const JS_ASSETS: [Asset; 1] = [asset!(
    "/assets/bundle.js", // Created via build.rs (release only) by combining other JS_ASSETS array bellow into single file
    AssetOptions::js().with_minify(true)
)];

// Release (build.rs) will combine all these files into single bundle.js
#[cfg(debug_assertions)] // Debug build individual JS files
pub const JS_ASSETS: [Asset; 2] = [
    asset!("/assets/js/common.js"),
    asset!("/assets/js/header.js"),
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

// Images
pub const IMG_TAPE_PREVIEW: Asset = asset!("/assets/img/tape-preview.jxl");
pub const IMG_TAPE_PREVIEW_TAB: Asset = asset!("/assets/img/tape-preview-tab.jxl");
pub const _IMG_TAPE_PREVIEW_MASK: Asset = asset!(
    "/assets/img/tape-preview-mask.jxl",
    AssetOptions::builder().with_hash_suffix(false) // Disable hash as referenced in CSS
);
pub const IMG_SANDPIT: Asset = asset!("/assets/img/sandpit.svg");
