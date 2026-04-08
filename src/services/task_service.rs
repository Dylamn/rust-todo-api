use crate::models::{Status, Task, TaskDescriptionError};
use chrono::Utc;
use uuid::Uuid;

pub fn create_task(description: String) -> Result<Task, TaskDescriptionError> {
    let id = Uuid::new_v4();

    Task::new(id, description)
}

pub fn mark_task_done(task: &mut Task) {
    task.completed_at = Some(Utc::now());
    task.status = Status::Completed;
}

pub fn mark_task_pending(task: &mut Task) {
    task.completed_at = None;
    task.status = Status::Pending;
}