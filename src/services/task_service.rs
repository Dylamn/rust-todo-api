use crate::models::{Status, Task};
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn list_tasks(db: &SqlitePool) -> anyhow::Result<Vec<Task>> {
    let tasks: Vec<Task> = sqlx::query_as(
        // language=sqlite
        r#"SELECT id, description, status, created_at, completed_at FROM tasks"#,
    )
    .fetch_all(db)
    .await?;

    Ok(tasks)
}

pub async fn find_task_by_id(db: &SqlitePool, id: Uuid) -> anyhow::Result<Option<Task>> {
    Ok(sqlx::query_as::<_, Task>(
        // language=sqlite
        r#"SELECT id, description, status, created_at, completed_at FROM tasks WHERE id = $1"#,
    )
    .bind(id.to_string())
    .fetch_optional(db)
    .await?)
}

pub async fn create_task(db: &SqlitePool, description: String) -> anyhow::Result<Task> {
    let id = Uuid::new_v4();

    let new_task = Task::new(id, description)?;

    sqlx::query::<sqlx::Sqlite>(
        // language=sqlite
        r#"INSERT INTO tasks (id, description, status) VALUES ($1, $2, $3)"#,
    )
    .bind(new_task.id.to_string())
    .bind(new_task.description.as_ref())
    .bind(&new_task.status)
    .execute(db)
    .await?;

    Ok(new_task)
}

pub fn mark_task_done(task: &mut Task) {
    task.completed_at = Some(Utc::now());
    task.status = Status::Completed;
}

pub fn mark_task_pending(task: &mut Task) {
    task.completed_at = None;
    task.status = Status::Pending;
}
