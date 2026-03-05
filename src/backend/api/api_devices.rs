use dioxus::prelude::*;

use crate::shared::models::tape_drive::TapeDrive;

#[get("/api/devices")]
pub async fn list_tape_devices() -> Result<Result<Vec<TapeDrive>, String>> {
    use crate::backend::system::devices::get_current_tape_devices;

    #[cfg(feature = "slow_server")]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(get_current_tape_devices())
}
