use super::{Status, TaskDescription, TaskDescriptionError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Task {
    pub id: Uuid,
    pub description: TaskDescription,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(id: Uuid, description: String) -> Result<Self, TaskDescriptionError> {
        Ok(Self {
            id,
            description: TaskDescription::try_from(description)?,
            status: Status::Pending,
            created_at: Utc::now(),
            completed_at: None,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_create_new_task() {
        let task_id = Uuid::new_v4();
        let task_description = "Test task";

        let task = Task::new(task_id, task_description.to_string())
            .expect("Task creation should succeed");

        assert_eq!(task.id, task_id);
        assert_eq!(task.description.as_ref(), task_description);
        assert_eq!(task.status, Status::Pending);
        assert!(task.created_at < Utc::now());
        assert!(task.completed_at.is_none());
    }

    #[test]
    fn should_fail_with_empty_description() {
        let task_id = Uuid::new_v4();

        let result = Task::new(task_id, "".to_string());

        assert!(result.is_err());
    }
}
