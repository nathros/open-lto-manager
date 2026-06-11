use dioxus::prelude::*;

pub const APP_NAME: &str = "Open LTO Manager";

pub const FAVICON: Asset = asset!(
    "/assets/logo.svg",
    AssetOptions::builder().with_hash_suffix(false) // Disable hash as referenced in CSS
);
pub const LOGO_ASSET: Asset = asset!("/assets/logos-all.svg");

pub const _ICONS_ASSET_REMIX: Asset = asset!(
    "/assets/icons-remix.svg",
    AssetOptions::builder().with_hash_suffix(false) // Disable hash as referenced in CSS
);
pub const _ICONS_ASSET_TABLER: Asset = asset!(
    "/assets/icons-tabler.svg",
    AssetOptions::builder().with_hash_suffix(false) // Disable hash as referenced in CSS
);
pub const _ICONS_ASSET_ICONOIR: Asset = asset!(
    "/assets/icons-iconoir.svg",
    AssetOptions::builder().with_hash_suffix(false) // Disable hash as referenced in CSS
);
pub const _ICONS_ASSET_SARGAM_LINE: Asset = asset!(
    "/assets/icons-sargamline.svg",
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
pub const CSS_ASSETS: [Asset; 11] = [
    asset!("/assets/css/:root.css"),
    asset!("/assets/font/font.css"),
    asset!("/assets/css/common.css"),
    asset!("/assets/css/header.css"),
    asset!("/assets/css/icon.css"),
    asset!("/assets/css/icons.css"),
    asset!("/assets/css/message.css"),
    asset!("/assets/css/modal.css"),
    asset!("/assets/css/table.css"),
    asset!("/assets/css/button.css"),
    asset!("/assets/css/tape-preview.css"),
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
