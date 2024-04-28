//!Implementation of [`TaskManager`]
use super::{current_task, current_user_token, TaskControlBlock, TaskStatus};
use crate::fs::{open_file, OpenFlags};
use crate::mm::translated_str;
use crate::sync::UPSafeCell;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use lazy_static::*;
///A array of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        let min_index = self
            .ready_queue
            .iter()
            .enumerate()
            .filter(|(_, task)| task.inner_exclusive_access().task_status == TaskStatus::Ready)
            .min_by_key(|(_, task)| task.inner_exclusive_access().stride)
            .unwrap();

        self.ready_queue.remove(min_index.0)
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
    //trace!("kernel: TaskManager::add_task");
    TASK_MANAGER.exclusive_access().add(task);
}

/// Take a process out of the ready queue
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    TASK_MANAGER.exclusive_access().fetch()
}

/// Spawn a new process
pub fn spawn_task(path: *const u8) -> isize {
    let token = current_user_token();
    let path = translated_str(token, path);
    if let Some(app_inode) = open_file(path.as_str(), OpenFlags::RDONLY) {
        let data = app_inode.read_all();
        let task = Arc::new(TaskControlBlock::new(&data));
        if let Some(parent) = current_task() {
            parent.inner_exclusive_access().children.push(task.clone());
            task.inner_exclusive_access().parent = Some(Arc::downgrade(&parent));
        }

        add_task(task.clone());
        task.pid.0 as isize
    } else {
        -1
    }
}
