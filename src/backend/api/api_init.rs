use dioxus::prelude::*;

use crate::shared::models::{about::About, app_state::AppState};

#[get("/api/app-state")]
pub async fn app_state() -> Result<AppState> {
    use crate::backend::init::APP_STATE;
    Ok(APP_STATE.clone())
}

#[get("/api/about")]
pub async fn app_about() -> Result<About> {
    Ok(About::get())
}
