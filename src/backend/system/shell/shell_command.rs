use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex, RwLock},
};
use tokio::sync::broadcast::{self, Sender};
use tracing::{error, info, trace};

type ShellMap = RwLock<HashMap<i64, Mutex<Sender<String>>>>;
pub static SHELL_SERVICES: LazyLock<ShellMap> = LazyLock::new(|| RwLock::new(HashMap::new()));

pub type TaskCompleteFn = Box<dyn Fn() + Send>;
type TaskFn = Box<dyn Fn(Sender<String>, TaskCompleteFn) + Send>;

pub struct TaskGuard {
    pub complete_fn: TaskCompleteFn,
}

impl Drop for TaskGuard {
    fn drop(&mut self) {
        trace!("Drop TaskGuard");
        self.complete_fn.as_ref()(); // Cleanup by calling complete function
    }
}

pub fn create_shell_service(
    services: &'static ShellMap,
    id: i64,
    task_fn: TaskFn,
    complete_fn: TaskCompleteFn,
) -> Result<Sender<String>, String> {
    match services.try_write() {
        Ok(mut write_guard) => match write_guard.get(&id) {
            Some(send) => match send.lock() {
                Ok(s2) => Ok(s2.clone()),
                Err(e) => Err(format!("Failed to get lock: {}, {}", id, e)),
            },
            None => {
                let (tx, _rx) = broadcast::channel::<String>(100);
                let _ = write_guard.insert(id, Mutex::new(tx.clone())); // Should not exist as checked above
                let tx_ret = tx.clone();
                let _handle = tokio::spawn(async move {
                    task_fn(tx, complete_fn);
                });
                Ok(tx_ret)
            }
        },
        Err(e) => {
            error!("Failed to get SERVICES write {} {}", id, e);
            Err(format!("Failed to get SERVICES write {} {}", id, e))
        }
    }
}

pub fn close_shell_service(services: &'static ShellMap, id: i64) {
    match services.try_write() {
        Ok(mut write_guard) => {
            let remove = write_guard.remove(&id);
            info!("Remove service id {} {}", id, remove.is_some());
        }
        Err(e) => error!("failed to close SERVICE {}, {}", id, e),
    }
}
