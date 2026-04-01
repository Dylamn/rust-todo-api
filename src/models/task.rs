use super::Status;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(id: u32, description: String) -> Self {
        Self {
            id,
            description,
            status: Status::Pending,
            created_at: Utc::now(),
            completed_at: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_create_new_task() {
        let task_id = 1;
        let task_description = "Test task";

        let task = Task::new(task_id, task_description.to_string());

        assert_eq!(task.id, task_id);
        assert_eq!(task.description, task_description);
        assert_eq!(task.status, Status::Pending);
        // Check that the created_at is within 5 seconds before the assertion
        assert!(task.created_at < Utc::now());
        assert!(task.completed_at.is_none());
    }
}
