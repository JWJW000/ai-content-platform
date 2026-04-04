use std::sync::Arc;
use tokio::sync::Mutex;

use crate::scheduler::TaskScheduler;

pub static SCHEDULER: std::sync::OnceLock<Arc<Mutex<Option<TaskScheduler>>>> = std::sync::OnceLock::new();

pub fn get_scheduler() -> &'static Arc<Mutex<Option<TaskScheduler>>> {
    SCHEDULER.get_or_init(|| Arc::new(Mutex::new(None)))
}
