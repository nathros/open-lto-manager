use dioxus::{fullstack::TextStream, prelude::*};

use crate::shared::models::tape_drive::TapeDrive;

#[get("/api/devices")]
pub async fn list_tape_devices() -> Result<Result<Vec<TapeDrive>, String>> {
    use crate::backend::system::devices::get_current_tape_devices;

    #[cfg(feature = "slow_server")]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    Ok(get_current_tape_devices())
}

#[get("/api/test_stream3/{id}")]
pub async fn text_stream3(id: i64) -> Result<TextStream> {
    Ok(TextStream::spawn(move |tx| async move {
        use crate::backend::system::devices::check_devices;

        match check_devices(id) {
            Ok(sender) => {
                let mut rx = sender.subscribe();
                loop {
                    match rx.recv().await {
                        Ok(msg) => {
                            if let Err(e) = tx.unbounded_send(msg) {
                                trace!("Failed to stream msg {}", e); // Either buffer is full or disconnected
                                return;
                            }
                        }
                        Err(e) => {
                            error!("Failed to get msg from Receiver<>: {}", e);
                            return;
                        }
                    }
                }
            }
            Err(e) => {
                error!("Failed to create service {}", id);
                if let Err(e) = tx.unbounded_send(e) {
                    error!("Failed to propagate create service error to UI {}", e);
                }
            }
        }
    }))
}
